use hal::gpio::gpioa::*;
use hal::gpio::gpiob::*;
use hal::gpio::gpioe::*;
use hal::gpio::*;
use hal::stm32;
use stm32::RCC;
use stm32f4xx_hal as hal;

pub trait RotaryEncoder {
    type PIN1;
    type PIN2;

    fn read_enc(&self) -> i16;
    fn setup_enc(&self, pin1: Self::PIN1, pin2: Self::PIN2);
}

#[rustfmt::skip]
macro_rules! define_rotary_encoder {
    ($TIMX:ident, $PIN1X:ty, $PIN2X:ty, $PERIPH_EN_REG:ident, $PERIPH_EN_FIELD:ident) => {
        impl RotaryEncoder for stm32::$TIMX {
            type PIN1 = $PIN1X;
            type PIN2 = $PIN2X;

            fn read_enc(&self) -> i16 {
                (self.cnt.read().bits() as i16) / 2
            }

            fn setup_enc(&self, pin1: $PIN1X, pin2: $PIN2X) {
                let rcc = unsafe { &(*RCC::ptr()) };
                rcc.$PERIPH_EN_REG.write(|w| w.$PERIPH_EN_FIELD().set_bit());

                self.smcr.write(|w| w
                    .sms().encoder_mode_3()
                    .ts().itr0()
                    .msm().no_sync()
                    .etf().no_filter()
                    .etps().div1()
                    .ece().clear_bit()
                    .etp().not_inverted()

                );
                self.ccer.write(|w| unsafe { w.bits(0) });
                self.arr.write(|w| w.arr().bits(0xFFFF));
                self.ccmr1_input().write(|w| unsafe { w
                    .cc1s().ti1()
                    .ic1psc().bits(0)
                    .ic1f().fdts_div16_n8()
                    .cc2s().ti1()
                    .ic2psc().bits(0)
                    .ic2f().bits(0xc)
                });
                self.cnt.write(|w| w.cnt().bits(0));
                self.egr.write(|w| unsafe { w.bits(0) });
                self.cr1.write(|w| w
                    .cen().enabled()
                    .udis().clear_bit()
                    .urs().any_event()
                    .opm().disabled()
                    .dir().up()
                    .cms().edge_aligned()
                    .arpe().disabled()
                    .ckd().div1()
                );

                pin1.internal_pull_up(true);
                pin2.internal_pull_up(true);
            }
        }
    };
}

define_rotary_encoder!(
    TIM1,
    PA8<Alternate<AF1>>,
    PE11<Alternate<AF1>>,
    apb2enr,
    tim1en
);
define_rotary_encoder!(
    TIM2,
    PA15<Alternate<AF1>>,
    PB3<Alternate<AF1>>,
    apb1enr,
    tim2en
);
define_rotary_encoder!(
    TIM3,
    PB5<Alternate<AF2>>,
    PB4<Alternate<AF2>>,
    apb1enr,
    tim3en
);
define_rotary_encoder!(
    TIM5,
    PA1<Alternate<AF2>>,
    PA0<Alternate<AF2>>,
    apb1enr,
    tim5en
);
