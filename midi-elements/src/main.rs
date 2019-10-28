#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use hal::delay::Delay;
use hal::gpio::*;
use hal::spi::*;
use hal::stm32;
use stm32f4::stm32f407::{interrupt, SPI2, TIM1};
use stm32f4xx_hal as hal;
use stm32f4xx_hal::rcc::RccExt;

extern crate cty;

mod driver;
use driver::encoder::RotaryEncoder;

use st7920::ST7920;

use embedded_graphics::fonts::Font6x12;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;

use embedded_hal::digital::v2::InputPin;

use numtoa::NumToA;

mod ui;
use ui::button::Button;
use ui::framework::Widget;

include!("elements.rs");

struct App<'a> {
    button_pin: gpiob::PB11<Input<PullUp>>,
    display: st7920::ST7920<
        Spi<
            SPI2,
            (
                gpiob::PB13<Alternate<AF5>>,
                NoMiso,
                gpiob::PB15<Alternate<AF5>>,
            ),
        >,
        gpioe::PE13<Output<PushPull>>,
        gpioe::PE13<Output<PushPull>>,
    >,
    enc1: TIM1,
    delay: Delay,
    button: Button<'a>,
}

impl<'a> App<'a> {
    fn new() -> Self {
        let p = stm32::Peripherals::take().unwrap();
        let cp = Peripherals::take().unwrap();
        let rcc = p.RCC.constrain();

        let clocks = rcc
            .cfgr
            .sysclk(stm32f4xx_hal::time::MegaHertz(168))
            .freeze();
        let mut delay = Delay::new(cp.SYST, clocks);

        let gpioa = p.GPIOA.split();
        let gpiob = p.GPIOB.split();
        let gpioe = p.GPIOE.split();

        p.TIM1.setup_enc(
            gpioa.pa8.into_alternate_af1(),
            gpioe.pe11.into_alternate_af1(),
        );

        let lcd_sck = gpiob.pb13.into_alternate_af5();
        let lcd_mosi = gpiob.pb15.into_alternate_af5();
        let lcd_reset = gpioe.pe13.into_push_pull_output();
        let spi = Spi::spi2(
            p.SPI2,
            (lcd_sck, NoMiso, lcd_mosi),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            stm32f4xx_hal::time::KiloHertz(1200).into(),
            clocks,
        );
        let button_pin = gpiob.pb11.into_pull_up_input();

        let mut display = ST7920::new(
            spi,
            lcd_reset,
            None as Option<stm32f4xx_hal::gpio::gpioe::PE13<Output<PushPull>>>,
            true,
        );

        display.init(&mut delay).expect("could not init display");
        display.clear(&mut delay).expect("could not clear display");

        unsafe {
            Init(false);
        }

        let button = Button::new(Point::new(0, 0), "test");

        App {
            button_pin,
            display,
            enc1: p.TIM1,
            delay,
            button,
        }
    }

    fn update(&mut self) {
        let button = !self.button_pin.is_high().unwrap();
        let value = self.enc1.read_enc();
        let mut buffer = [0u8; 10];

        unsafe {
            SetGate(button);
            (*GetPatch()).exciter_strike_level = (value as f32) / 20f32;
        }

        self.display.draw(
            Font6x12::render_str(value.numtoa_str(10, &mut buffer))
                .fill(Some(if button {
                    BinaryColor::On
                } else {
                    BinaryColor::Off
                }))
                .stroke(Some(BinaryColor::On))
                .translate(Point::new(30, 30)),
        );
        self.display
            .flush_region(30, 30, 16, 16, &mut self.delay)
            .expect("could not flush display");

        let invalidate = self.button.render(&mut self.display);
        self.display
            .flush_region_graphics(invalidate, &mut self.delay)
            .expect("could not flush display");
    }
}

#[entry]
fn main() -> ! {
    let mut app = App::new();
    loop {
        app.update();
    }
}

#[interrupt]
fn DMA1_STREAM5() {
    unsafe {
        Elements_DMA1_Stream5_IRQHandler();
    }
}
