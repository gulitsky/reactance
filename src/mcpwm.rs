//! # Motor Control PWM
use super::device::{RCC, TIM1, TIM8};
use super::hal::{
    gpio::{
        gpioa::{PA10, PA5, PA6, PA7, PA8, PA9},
        gpiob::{PB0, PB1, PB12, PB13, PB14, PB15},
        gpioc::{PC6, PC7, PC8},
        gpioe::{PE10, PE11, PE12, PE13, PE15, PE8, PE9},
        gpioh::{PH13, PH14, PH15},
        gpioi::{PI4, PI5, PI6, PI7},
        Alternate, AF1, AF3,
    },
    time::Hertz,
};
use embedded_hal::Pwm;

pub trait PinUL<TIM> {}
pub trait PinUH<TIM> {}
pub trait PinVH<TIM> {}
pub trait PinVL<TIM> {}
pub trait PinWH<TIM> {}
pub trait PinWL<TIM> {}

impl PinUL<TIM1> for PA7<Alternate<AF1>> {}
impl PinUH<TIM1> for PA8<Alternate<AF1>> {}
impl PinUL<TIM1> for PB13<Alternate<AF1>> {}
impl PinUL<TIM1> for PE8<Alternate<AF1>> {}
impl PinUH<TIM1> for PE9<Alternate<AF1>> {}

impl PinUL<TIM8> for PA5<Alternate<AF3>> {}
impl PinUL<TIM8> for PA7<Alternate<AF3>> {}
impl PinUH<TIM8> for PC6<Alternate<AF3>> {}
impl PinUL<TIM8> for PH13<Alternate<AF3>> {}
impl PinUH<TIM8> for PI5<Alternate<AF3>> {}

impl PinVL<TIM1> for PB0<Alternate<AF1>> {}
impl PinVH<TIM1> for PA9<Alternate<AF1>> {}
impl PinVL<TIM1> for PB14<Alternate<AF1>> {}
impl PinVL<TIM1> for PE10<Alternate<AF1>> {}
impl PinVH<TIM1> for PE11<Alternate<AF1>> {}

impl PinVL<TIM8> for PB0<Alternate<AF3>> {}
impl PinVL<TIM8> for PB14<Alternate<AF3>> {}
impl PinVH<TIM8> for PC7<Alternate<AF3>> {}
impl PinVL<TIM8> for PH14<Alternate<AF3>> {}
impl PinVH<TIM8> for PI6<Alternate<AF3>> {}

impl PinWH<TIM1> for PA10<Alternate<AF1>> {}
impl PinWL<TIM1> for PB1<Alternate<AF1>> {}
impl PinWL<TIM1> for PB15<Alternate<AF1>> {}
impl PinWL<TIM1> for PE12<Alternate<AF1>> {}
impl PinWH<TIM1> for PE13<Alternate<AF1>> {}

impl PinWL<TIM8> for PB1<Alternate<AF3>> {}
impl PinWL<TIM8> for PB15<Alternate<AF3>> {}
impl PinWH<TIM8> for PC8<Alternate<AF3>> {}
impl PinWL<TIM8> for PH15<Alternate<AF3>> {}
impl PinWH<TIM8> for PI7<Alternate<AF3>> {}

pub trait Pins<TIM> {}
impl<TIM, UH, UL, VH, VL, WH, WL> Pins<TIM> for (UH, UL, VH, VL, WH, WL)
where
    UH: PinUH<TIM>,
    UL: PinUL<TIM>,
    VH: PinVH<TIM>,
    VL: PinVL<TIM>,
    WH: PinWH<TIM>,
    WL: PinWL<TIM>,
{
}

pub enum Phase {
    U,
    V,
    W,
}

/// Hardware motor control PWM peripheral
pub struct Mcpwm<TIM, PINS> {
    tim: TIM,
    pins: PINS,
}

impl<PINS> Mcpwm<TIM1, PINS> {
    /// Configures a TIM peripheral as a motor control PWM output
    pub fn new(tim: TIM1, pins: PINS) -> Self
    where
        PINS: Pins<TIM1>,
    {
        let rcc = unsafe { &(*RCC::ptr()) };
        // enable and reset peripheral to a clean slate state
        rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
        rcc.apb2rstr.modify(|_, w| w.tim1rst().reset());
        rcc.apb2rstr.modify(|_, w| w.tim1rst().clear_bit());

        tim.cr1
            .write(|w| w.cen().enabled().cms().center_aligned3().arpe().enabled());
        tim.cr2.write(|w| w.ccpc().set_bit());
        tim.bdtr
            .write(|w| w.ossi().idle_level().ossr().idle_level());

        Mcpwm { tim, pins }
    }
}

impl<PINS> Pwm for Mcpwm<TIM1, PINS> {
    type Channel = Phase;
    type Time = Hertz;
    type Duty = u16;

    fn enable(&mut self, phase: Self::Channel) {
        match phase {
            Self::Channel::U => {
                self.tim
                    .cr2
                    .modify(|_, w| w.ois1().clear_bit().ois1n().clear_bit());
                self.tim
                    .ccmr1_output()
                    .modify(|_, w| w.oc1fe().set_bit().oc1pe().enabled().oc1m().pwm_mode1());
                self.tim
                    .ccer
                    .modify(|_, w| w.cc1e().set_bit().cc1ne().set_bit());
            }
            Self::Channel::V => {
                self.tim
                    .cr2
                    .modify(|_, w| w.ois2().clear_bit().ois2n().clear_bit());
                self.tim
                    .ccmr1_output()
                    .modify(|_, w| w.oc2fe().set_bit().oc2pe().enabled().oc2m().pwm_mode1());
                self.tim
                    .ccer
                    .modify(|_, w| w.cc2e().set_bit().cc2ne().set_bit());
            }
            Self::Channel::W => {
                self.tim
                    .cr2
                    .modify(|_, w| w.ois3().clear_bit().ois3n().clear_bit());
                self.tim
                    .ccmr2_output()
                    .modify(|_, w| w.oc3fe().set_bit().oc3pe().enabled().oc3m().pwm_mode1());
                self.tim
                    .ccer
                    .modify(|_, w| w.cc3e().set_bit().cc3ne().set_bit());
            }
        }
        self.tim.egr.write(|w| w.comg().set_bit());
    }
}
