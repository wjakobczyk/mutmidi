#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use embedded_hal::blocking::delay::DelayMs;
use hal::delay::Delay;
use hal::gpio::*;
use hal::stm32;
use hal::gpio::gpioe::{PE9, PE11};
use stm32f4::stm32f407::interrupt;
use stm32f4xx_hal as hal;
use stm32f4xx_hal::rcc::RccExt;

extern crate cty;

#[link(name = "elements")]
extern "C" {
    fn RunElements(application: bool);
    fn Elements_DMA1_Stream5_IRQHandler();
}

trait RotaryEncoder {
    type PIN1;
    type PIN2;

    fn read(&self) -> u32;
    fn setup(&self, pin1: Self::PIN1, pin2: Self::PIN2);
}

macro_rules! rotary_encoder {
    ($TIMX:ident, $PIN1X:ty, $PIN2X:ty) => {
        impl RotaryEncoder for stm32::$TIMX {
            type PIN1 = $PIN1X;
            type PIN2 = $PIN2X;

            fn read(&self) -> u32 {
                self.cnt.read().bits()
            }

            fn setup(&self, pin1: $PIN1X, pin2: $PIN2X) {
                self.smcr.write(|w| unsafe { w.bits(3) });
                self.ccer.write(|w| unsafe { w.bits(0) });
                self.arr.write(|w| unsafe { w.bits(0xFFFF) });
                self.ccmr1_input().write(|w| unsafe { w.bits(0xC1C1) });
                self.cnt.write(|w| unsafe { w.bits(0) });
                self.egr.write(|w| unsafe { w.bits(0) });
                self.cr1.write(|w| unsafe { w.bits(1) });
                pin1.internal_pull_up(true);
                pin2.internal_pull_up(true);
            }
        }
    }
}

rotary_encoder!(TIM1, PE9<Alternate<AF1>>, PE11<Alternate<AF1>>);

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        hprintln!("Hello, world!").unwrap();

        p.RCC.apb2enr.write(|w| w.tim1en().set_bit());

        let rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc
            .cfgr
            .sysclk(stm32f4xx_hal::time::MegaHertz(168))
            .freeze();
        let mut delay = Delay::new(cp.SYST, clocks);

        let gpioe = p.GPIOE.split();

        p.TIM1.setup(gpioe.pe9.into_alternate_af1(), gpioe.pe11.into_alternate_af1());

        loop {
            hprintln!("timer {}", p.TIM1.read()).unwrap();
            delay.delay_ms(100 as u32);
        }
    }

    unsafe {
        RunElements(false);
    }

    loop {}
}

#[interrupt]
fn DMA1_STREAM5() {
    unsafe {
        Elements_DMA1_Stream5_IRQHandler();
    }
}
