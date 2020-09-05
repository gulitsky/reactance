#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32g4::stm32g474 as pac;

mod board;

#[rtic::app(device = stm32g4::stm32g474, monotonic = rtic::cyccnt::CYCCNT, peripherals = true)]
const APP: () = {
    struct Resources {}

    #[init]
    fn init(cx: init::Context) {
        rtt_init_print!();

        let mut core = cx.core;
        // Enable the monotonic timer
        core.DCB.enable_trace();
        core.DWT.enable_cycle_counter();

        board::init();
        rprintln!("Reactance");
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }
};
