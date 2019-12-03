use super::device::{TIM1, TIM8};
use super::hal::{
    gpio::{
        gpioa::{PA10, PA5, PA6, PA7, PA8, PA9},
        gpiob::{PB0, PB1, PB12, PB13, PB14, PB15},
        gpioc::{PC6, PC7, PC8, PC9},
        gpioe::{PE10, PE11, PE13, PE15, PE8, PE9},
        gpioh::{PH13, PH14, PH15},
        gpioi::{PI4, PI5, PI6, PI7},
        Alternate, AF1, AF3,
    },
    time::Hertz,
};
use embedded_hal::Pwm;

pub enum Phase {
    U,
    V,
    W,
}

pub struct Mcpwm<TIM> {
    tim: TIM,
}

impl Mcpwm<TIM1> {
    pub fn new<T>(tim: TIM1) -> Self {
        Mcpwm { tim }
    }
}

impl Mcpwm<TIM8> {
    pub fn new<T>(tim: TIM8) -> Self {
        Mcpwm { tim }
    }
}

impl<TIM> Pwm for Mcpwm<TIM> {
    type Channel = Phase;
    type Time = Hertz;
    type Duty = u16;

    fn enable(&mut self, phase: Self::Channel) {
        match phase {
            Self::Channel::U => {}
            Self::Channel::V => {}
            Self::Channel::W => {}
        }
    }
}
