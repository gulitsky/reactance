#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;
use stm32g4::stm32g474 as pac;

mod board;

#[rtic::app(device = stm32g4::stm32g474, monotonic = rtic::cyccnt::CYCCNT, peripherals = true)]
const APP: () = {
    struct Resources {}

    #[init]
    fn init(cx: init::Context) {
        let mut core = cx.core;
        // Enable the monotonic timer
        core.DCB.enable_trace();
        core.DWT.enable_cycle_counter();

        board::init();
        defmt::info!("Reactance");
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }
};
