use hal::gpio::gpioe::{PE11, PE9};
use hal::gpio::*;
use hal::stm32;
use stm32f4xx_hal as hal;

pub trait RotaryEncoder {
    type PIN1;
    type PIN2;

    fn read_enc(&self) -> u32;
    fn setup_enc(&self, pin1: Self::PIN1, pin2: Self::PIN2);
}

macro_rules! define_rotary_encoder {
    ($TIMX:ident, $PIN1X:ty, $PIN2X:ty) => {
        impl RotaryEncoder for stm32::$TIMX {
            type PIN1 = $PIN1X;
            type PIN2 = $PIN2X;

            fn read_enc(&self) -> u32 {
                self.cnt.read().bits()
            }

            fn setup_enc(&self, pin1: $PIN1X, pin2: $PIN2X) {
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
    };
}

define_rotary_encoder!(TIM1, PE9<Alternate<AF1>>, PE11<Alternate<AF1>>);
