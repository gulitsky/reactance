#![no_main]
#![no_std]

use panic_persist as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32g4::stm32g474 as pac;

pub mod board;

#[entry]
fn main() -> ! {
    asm::nop();

    loop {}
}
