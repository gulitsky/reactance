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

#[cfg(all(debug_assertions, feature = "panic-ramdump"))]
use panic_ramdump as _;

#[cfg(all(not(debug_assertions), feature = "panic-reset"))]
use panic_reset as _;

#[cfg(all(debug_assertions, feature = "panic-semihosting"))]
use panic_semihosting as _;

use rtfm::app;

use nrf52840_hal as hal;
use hal::{nrf52840_pac as device};

#[app(device = nrf52840_hal::nrf52840_pac, peripherals = true)]
const APP: () = {
    #[init]
    fn init(c: init::Context) {
        let cp: cortex_m::Peripherals = c.core;
        let dp: device::Peripherals = c.device;
    }
};
