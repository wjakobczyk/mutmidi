use hal::gpio::gpioa::PA8;
use hal::gpio::gpioe::PE11;
use hal::gpio::*;
use hal::stm32;
use stm32::RCC;
use stm32f4xx_hal as hal;

pub trait RotaryEncoder {
    type PIN1;
    type PIN2;

    fn read_enc(&self) -> u32;
    fn setup_enc(&self, pin1: Self::PIN1, pin2: Self::PIN2);
}

#[rustfmt::skip]
macro_rules! define_rotary_encoder {
    ($TIMX:ident, $PIN1X:ty, $PIN2X:ty, $PERIPH_EN_REG:ident, $PERIPH_EN_FIELD:ident) => {
        impl RotaryEncoder for stm32::$TIMX {
            type PIN1 = $PIN1X;
            type PIN2 = $PIN2X;

            fn read_enc(&self) -> u32 {
                self.cnt.read().bits()
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
                self.ccer.write(|w| w
                    .cc1e().clear_bit()
                    .cc1p().clear_bit()
                    .cc1ne().clear_bit()
                    .cc1np().clear_bit()
                    .cc2e().clear_bit()
                    .cc2p().clear_bit()
                    .cc2ne().clear_bit()
                    .cc2np().clear_bit()
                    .cc3e().clear_bit()
                    .cc3p().clear_bit()
                    .cc3ne().clear_bit()
                    .cc3np().clear_bit()
                    .cc4e().clear_bit()
                    .cc4p().clear_bit()
                );
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
                self.egr.write(|w| w
                    .ug().clear_bit()
                    .cc1g().clear_bit()
                    .cc2g().clear_bit()
                    .cc3g().clear_bit()
                    .cc4g().clear_bit()
                    .comg().clear_bit()
                    .tg().clear_bit()
                    .bg().clear_bit()
                );
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
