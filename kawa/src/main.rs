// Copyright 2019 Wojciech Jakóbczyk
//
// Author: Wojciech Jakóbczyk (jakobczyk.woj@gmail.com)
//
// This file is part of Kawa Synth.
//
// Kawa Synth is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Kawa Synth is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Kawa Synth.  If not, see <https://www.gnu.org/licenses/>.

#![feature(alloc_error_handler)]
#![no_std]
#![no_main]
#![feature(drain_filter)]

extern crate alloc;
extern crate panic_halt;

use core::alloc::Layout;

use alloc::vec::Vec;
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

use stm32_flash::Flash;

use embedded_hal::digital::v2::InputPin;

mod ui;
use ui::framework::{BinaryColor, Value};
use ui::*;

mod elements_handlers;

mod midi_input;
use midi_input::MidiInput;

mod patch;
mod util;
mod voice;

mod synth;
use synth::*;

use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;

use alloc::boxed::Box;

include!("elements.rs");

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 4 * 1024; // in bytes

const FLASH_SECTOR_STORE: u8 = 11;

type MidiUart = Serial<UART4, (NoTx, gpioc::PC11<Alternate<AF8>>)>;

struct App<'a> {
    button_pins: (
        gpioe::PE7<Input<PullUp>>,
        gpioe::PE15<Input<PullUp>>,
        gpiod::PD9<Input<PullUp>>,
        gpiod::PD11<Input<PullUp>>,
        gpiob::PB11<Input<PullUp>>,
    ),
    trigger_pin: gpioe::PE9<Input<PullUp>>,
    trigger_state: bool,
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
    encoders: (TIM2, TIM3, TIM5, TIM1),
    delay: Delay,
    midi_input: MidiInput<MidiUart>,
    flash: Flash,
    ui: UI<'a>,
    synth: Synth,
}

impl<'a> App<'a> {
    fn new() -> Self {
        //TODO split this to smaller functions or if not possible (partial borrows), at least to macros
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
            stm32f4xx_hal::time::KiloHertz(400).into(),
            clocks,
        );
        let button_pins = (
            gpioe.pe7.into_pull_up_input(),
            gpioe.pe15.into_pull_up_input(),
            gpiod.pd9.into_pull_up_input(),
            gpiod.pd11.into_pull_up_input(),
            gpiob.pb11.into_pull_up_input(),
        );
        let trigger_pin = gpioe.pe9.into_pull_up_input();

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
        let midi_input = MidiInput::new(MidiInPort::new(midi_uart));

        unsafe {
            Elements_Init(false);
            cp.NVIC
                .set_priority(stm32f4::stm32f407::Interrupt::DMA1_STREAM5, 16);
        }

        let ui = UI::new();

        let flash = Flash::new(p.FLASH, FLASH_SECTOR_STORE);

        let mut synth = Synth::new();

        if button_pins.0.is_high().unwrap() {
            synth.load_patch(&flash, 0);
        }

        App {
            button_pins,
            trigger_pin,
            trigger_state: false,
            display,
            midi_input,
            encoders: (p.TIM2, p.TIM3, p.TIM5, p.TIM1),
            delay,
            flash,
            ui,
            synth,
        }
    }

    fn setup(&mut self) {
        self.ui.setup();
    }

    fn pause_synth(pause: bool) {
        unsafe {
            Elements_Pause(pause);
        }
    }

    fn save(&mut self) {
        self.flash.erase().unwrap();
        self.synth.save_patch(&mut self.flash, 0);
    }

    pub fn change_panel(&mut self, self2: &'a mut App<'a>, panel: PanelId) {
        self.ui.change_panel(&mut self2.ui, panel);

        self.update_knobs();
        App::pause_synth(true);

        self.display.draw(
            Rectangle::new(
                Point::new(0, 0),
                Point::new(st7920::WIDTH - 1, st7920::HEIGHT - 1),
            )
            .fill(Some(BinaryColor::Off)),
        );

        self.ui.render(&mut self.display);
        self.display
            .flush(&mut self.delay)
            .expect("could not flush display");

        self.save();

        App::pause_synth(false);
    }

    fn update_knobs(&mut self) {
        self.ui.update_knobs((
            Value::Int(self.encoders.0.read_enc() as i32),
            Value::Int(self.encoders.1.read_enc() as i32),
            Value::Int(self.encoders.2.read_enc() as i32),
            Value::Int(self.encoders.3.read_enc() as i32),
        ));
    }

    fn update_buttons(&mut self) {
        self.ui.update_buttons((
            !self.button_pins.0.is_high().unwrap(),
            !self.button_pins.1.is_high().unwrap(),
            !self.button_pins.2.is_high().unwrap(),
            !self.button_pins.3.is_high().unwrap(),
            !self.button_pins.4.is_high().unwrap(),
        ));
    }

    fn update(&mut self) {
        self.update_knobs();
        self.update_buttons();
        let invalidate = self.ui.render(&mut self.display);

        if let Some(invalidate) = invalidate {
            if invalidate.1.width != 0 && invalidate.1.height != 0 {
                self.display
                    .flush_region_graphics(invalidate, &mut self.delay)
                    .expect("could not flush display");
            }
        }

        if !self.trigger_state && self.trigger_pin.is_low().unwrap() {
            self.synth
                .shared_state
                .voice_events
                .enque(voice::VoiceEvent::NoteOn {
                    retrigger: false,
                    note: 40.0,
                    strength: 1.0,
                });
            self.trigger_state = true;
        } else if self.trigger_state && self.trigger_pin.is_high().unwrap() {
            self.synth
                .shared_state
                .voice_events
                .enque(voice::VoiceEvent::NoteOff);
            self.trigger_state = false;
        }
    }
}

static mut APP: *mut App = core::ptr::null_mut();

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    let mut app = Box::new(App::new());
    unsafe {
        APP = &mut *app as *mut App;
    }

    app.setup();
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
        if !APP.is_null() {
            (*APP)
                .midi_input
                .handle_midi_irq(&mut (*APP).synth.shared_state.voice_events);
        }
    }
}

#[interrupt]
fn DMA1_STREAM5() {
    //using APP only to get voice and voice_events references which are behind a Mutex
    //and we're in an interrupt, so voice is not being accessed (critical section blocks irqs)
    unsafe {
        if !APP.is_null() {
            let mut events = Vec::new();

            (*APP)
                .synth
                .shared_state
                .voice_events
                .deque_all(&mut events);

            cortex_m::interrupt::free(|cs| {
                let patch = &(*APP).synth.shared_state.patch.borrow(cs).borrow();

                (*APP)
                    .synth
                    .shared_state
                    .voice
                    .borrow(cs)
                    .update(&events, patch)
            });
        }
    }

    unsafe {
        Elements_DMA1_Stream5_IRQHandler();
    };
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}
