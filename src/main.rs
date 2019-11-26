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

use nrf52840_hal::{clocks, prelude::*, target as device};

use rtfm::app;

#[app(device = nrf52840_hal::target, peripherals = true)]
const APP: () = {
    #[init]
    fn init(c: init::Context) {
        let _cp: cortex_m::Peripherals = c.core;
        let dp: device::Peripherals = c.device;

        let _clocks = dp
            .CLOCK
            .constrain()
            .enable_ext_hfosc()
            .set_lfclk_src_external(clocks::LfOscConfiguration::NoExternalNoBypass)
            .start_lfclk();
    }
};
