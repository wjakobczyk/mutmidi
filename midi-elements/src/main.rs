#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate panic_halt;

use core::alloc::Layout;

use alloc_cortex_m::CortexMHeap;
use cortex_m::asm;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use hal::delay::Delay;
use hal::gpio::*;
use hal::serial::config::*;
use hal::serial::*;
use hal::spi::*;
use hal::stm32;
use hal::stm32::UART4;
use stm32f4::stm32f407::{interrupt, SPI2, TIM1, TIM2, TIM3, TIM5};
use stm32f4xx_hal as hal;
use stm32f4xx_hal::rcc::RccExt;

extern crate cty;

mod driver;
use driver::encoder::RotaryEncoder;

use st7920::ST7920;

use midi_port::*;

use embedded_hal::digital::v2::InputPin;

mod ui;
use ui::framework::*;

use ui::*;

mod elements_handlers;
use elements_handlers::*;

use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;

use alloc::boxed::Box;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 4 * 1024; // in bytes

#[derive(Clone, Copy)]
enum InputDeviceId {
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    Knob1,
    Knob2,
    Knob3,
    Knob4,
}

enum PanelId {
    PanelBow,
    PanelBlow,
    PanelStrike,
    PanelRes,
    PanelOutput,
}

type MidiUart = Serial<UART4, (NoTx, gpioc::PC11<Alternate<AF8>>)>;

struct App<'a> {
    button_pins: (
        gpioe::PE7<Input<PullUp>>,
        gpioe::PE15<Input<PullUp>>,
        gpiod::PD9<Input<PullUp>>,
        gpiod::PD11<Input<PullUp>>,
        gpiob::PB11<Input<PullUp>>,
    ),
    _trigger_pin: gpioe::PE9<Input<PullUp>>,
    button_states: [bool; 5],
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
    midi_in: MidiInPort<MidiUart>,
    encoders: (TIM2, TIM3, TIM5, TIM1),
    delay: Delay,
    panels: Option<[Panel<'a>; 5]>,
    current_panel: Option<&'a mut Panel<'a>>,
}

impl<'a> App<'a> {
    fn new() -> Self {
        let p = stm32::Peripherals::take().unwrap();
        let mut cp = Peripherals::take().unwrap();
        let rcc = p.RCC.constrain();

        let clocks = rcc
            .cfgr
            .sysclk(stm32f4xx_hal::time::MegaHertz(168))
            .freeze();
        let mut delay = Delay::new(cp.SYST, clocks);

        let gpioa = p.GPIOA.split();
        let gpiob = p.GPIOB.split();
        let gpioc = p.GPIOC.split();
        let gpiod = p.GPIOD.split();
        let gpioe = p.GPIOE.split();

        p.TIM1.setup_enc(
            gpioa.pa8.into_alternate_af1(),
            gpioe.pe11.into_alternate_af1(),
        );
        p.TIM2.setup_enc(
            gpioa.pa15.into_alternate_af1(),
            gpiob.pb3.into_alternate_af1(),
        );
        p.TIM3.setup_enc(
            gpiob.pb5.into_alternate_af2(),
            gpiob.pb4.into_alternate_af2(),
        );
        p.TIM5.setup_enc(
            gpioa.pa1.into_alternate_af2(),
            gpioa.pa0.into_alternate_af2(),
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
        let button_pins = (
            gpioe.pe7.into_pull_up_input(),
            gpioe.pe15.into_pull_up_input(),
            gpiod.pd9.into_pull_up_input(),
            gpiod.pd11.into_pull_up_input(),
            gpiob.pb11.into_pull_up_input(),
        );
        let _trigger_pin = gpioe.pe9.into_pull_up_input();

        let mut display = ST7920::new(
            spi,
            lcd_reset,
            None as Option<stm32f4xx_hal::gpio::gpioe::PE13<Output<PushPull>>>,
            true,
        );

        display.init(&mut delay).expect("could not  display");
        display.clear(&mut delay).expect("could not clear display");

        let mut midi_uart = Serial::uart4(
            p.UART4,
            (NoTx, gpioc.pc11.into_alternate_af8()),
            Config {
                baudrate: stm32f4xx_hal::time::Bps(31250),
                wordlength: WordLength::DataBits8,
                parity: Parity::ParityNone,
                stopbits: StopBits::STOP1,
            },
            clocks,
        )
        .unwrap();
        midi_uart.listen(hal::serial::Event::Rxne);
        unsafe {
            cortex_m::peripheral::NVIC::unmask(stm32f4::stm32f407::Interrupt::UART4);
            cp.NVIC
                .set_priority(stm32f4::stm32f407::Interrupt::UART4, 0);
        }
        let midi_in = MidiInPort::new(midi_uart);

        unsafe {
            Elements_Init(false);
            cp.NVIC
                .set_priority(stm32f4::stm32f407::Interrupt::DMA1_STREAM5, 16);
        }

        App {
            button_pins,
            _trigger_pin,
            button_states: [false; 5],
            display,
            midi_in,
            encoders: (p.TIM2, p.TIM3, p.TIM5, p.TIM1),
            delay,
            panels: None,
            current_panel: None,
        }
    }

    fn setup_ui(&mut self) {
        self.panels = Some([
            Panel::new(panel_bow::setup()),
            Panel::new(panel_blow::setup()),
            Panel::new(panel_strike::setup()),
            Panel::new(panel_res::setup()),
            Panel::new(panel_out::setup()),
        ])
    }

    fn pause_synth(pause: bool) {
        unsafe {
            Elements_Pause(pause);
        }
    }

    pub fn change_panel(&mut self, self2: &'a mut App<'a>, panel: PanelId) {
        if let Some(panels) = &mut self2.panels {
            self.current_panel = Some(&mut panels[panel as usize]);
        }

        if let Some(panel) = &mut self.current_panel {
            panel.input_reset();
        }

        self.update_knobs();
        App::pause_synth(true);

        self.display.draw(
            Rectangle::new(
                Point::new(0, 0),
                Point::new(st7920::WIDTH - 1, st7920::HEIGHT - 1),
            )
            .fill(Some(BinaryColor::Off)),
        );

        if let Some(panel) = &mut self.current_panel {
            panel.render(&mut self.display);
            self.display
                .flush(&mut self.delay)
                .expect("could not flush display");
        }

        App::pause_synth(false);
    }

    fn update_knobs(&mut self) {
        if let Some(panel) = &mut self.current_panel {
            panel.input_update(
                InputDeviceId::Knob1 as InputId,
                Value::Int(self.encoders.0.read_enc() as i32),
            );
            panel.input_update(
                InputDeviceId::Knob2 as InputId,
                Value::Int(self.encoders.1.read_enc() as i32),
            );
            panel.input_update(
                InputDeviceId::Knob3 as InputId,
                Value::Int(self.encoders.2.read_enc() as i32),
            );
            panel.input_update(
                InputDeviceId::Knob4 as InputId,
                Value::Int(self.encoders.3.read_enc() as i32),
            );
        };
    }

    fn update_button(&mut self, id: InputDeviceId, value: bool) {
        if value && value != self.button_states[id as usize] {
            if let Some(panel) = &mut self.current_panel {
                panel.input_update(id as InputId, Value::Bool(value));
            };
        }
        self.button_states[id as usize] = value;
    }

    fn update_buttons(&mut self) {
        self.update_button(
            InputDeviceId::Button1,
            !self.button_pins.0.is_high().unwrap(),
        );
        self.update_button(
            InputDeviceId::Button2,
            !self.button_pins.1.is_high().unwrap(),
        );
        self.update_button(
            InputDeviceId::Button3,
            !self.button_pins.2.is_high().unwrap(),
        );
        self.update_button(
            InputDeviceId::Button4,
            !self.button_pins.3.is_high().unwrap(),
        );
        self.update_button(
            InputDeviceId::Button5,
            !self.button_pins.4.is_high().unwrap(),
        );
    }

    fn update(&mut self) {
        self.update_knobs();
        self.update_buttons();

        if let Some(panel) = &mut self.current_panel {
            let invalidate = panel.render(&mut self.display);
            if invalidate.1.width != 0 && invalidate.1.height != 0 {
                self.display
                    .flush_region_graphics(invalidate, &mut self.delay)
                    .expect("could not flush display");
            }
        }
    }

    pub fn handle_note(&mut self, on: bool, note: NoteNumber, velocity: u8) {
        unsafe {
            Elements_SetGate(on);
            if on {
                Elements_SetNote(note as f32);
                Elements_SetStrength((velocity as f32) / 127.0);
                Elements_SetModulation(0.0);
            }
        }
    }

    pub fn set_modulation(&mut self, value: u8) {
        unsafe {
            Elements_SetModulation((value as f32) / 127.0);
        }
    }

    fn handle_midi_irq(&mut self) {
        self.midi_in.poll_uart();

        if let Some(message) = self.midi_in.get_message() {
            match message {
                MidiMessage::NoteOn {
                    channel: _,
                    note,
                    velocity,
                } => self.handle_note(true, note, velocity),
                MidiMessage::NoteOff {
                    channel: _,
                    note,
                    velocity,
                } => self.handle_note(false, note, velocity),
                MidiMessage::Aftertouch {
                    channel: _,
                    note: None,
                    value,
                } => self.set_modulation(value),
                _ => (),
            };
        }
    }
}

static mut APP: *mut App = 0 as *mut App;

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    let mut app = Box::new(App::new());
    unsafe {
        APP = &mut *app as *mut App;
    }

    app.setup_ui();
    unsafe {
        app.change_panel(&mut *APP, PanelId::PanelBow);
    }

    loop {
        app.update();
    }
}

#[interrupt]
fn UART4() {
    unsafe {
        (*APP).handle_midi_irq();
    }
}

#[interrupt]
fn DMA1_STREAM5() {
    unsafe {
        Elements_DMA1_Stream5_IRQHandler();
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}
