#![no_std]
#![no_main]

#[cfg(all(not(debug_assertions), feature = "panic-abort"))]
use panic_abort as _;

#[cfg(all(debug_assertions, feature = "panic-halt"))]
use panic_halt as _;

#[cfg(all(debug_assertions, feature = "panic-itm"))]
use panic_itm as _;

#[cfg(all(not(debug_assertions), feature = "panic-never"))]
use panic_never as _;

#[cfg(all(not(debug_assertions), feature = "panic-persist"))]
use panic_persist as _;

#[cfg(all(not(debug_assertions), feature = "panic-reset"))]
use panic_reset as _;

#[cfg(all(debug_assertions, feature = "panic-semihosting"))]
use panic_semihosting as _;

use stm32f4xx_hal::{prelude::*, stm32 as device};

use rtfm::app;

#[app(device = stm32f4xx_hal::stm32, peripherals = true)]
const APP: () = {
    #[init]
    fn init(c: init::Context) {
        let _cp: cortex_m::Peripherals = c.core;
        let _dp: device::Peripherals = c.device;

        let rcc: stm32f4xx_hal::rcc::Rcc = _dp.RCC.constrain();
        let _clocks: stm32f4xx_hal::rcc::Clocks = rcc.cfgr.use_hse(168.mhz()).freeze();
    }
};
