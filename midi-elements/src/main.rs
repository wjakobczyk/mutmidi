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
use stm32f4::stm32f407::interrupt;
use stm32f4xx_hal as hal;
use stm32f4xx_hal::rcc::RccExt;

extern crate cty;

#[link(name = "elements")]
extern "C" {
    fn RunElements(application: bool);
    fn Elements_DMA1_Stream5_IRQHandler();
}
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

        gpioe.pe9.into_alternate_af1().internal_pull_up(true);
        gpioe.pe11.into_alternate_af1().internal_pull_up(true);

        p.TIM1.smcr.write(|w| unsafe { w.bits(3) });
        p.TIM1.ccer.write(|w| unsafe { w.bits(0) });
        p.TIM1.arr.write(|w| unsafe { w.bits(0xFFFF) });
        p.TIM1.ccmr1_input().write(|w| unsafe { w.bits(0xC1C1) });
        p.TIM1.cnt.write(|w| unsafe { w.bits(0) });
        p.TIM1.egr.write(|w| unsafe { w.bits(0) });
        p.TIM1.cr1.write(|w| unsafe { w.bits(1) });

        loop {
            hprintln!("timer {}", p.TIM1.cnt.read().bits()).unwrap();
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
