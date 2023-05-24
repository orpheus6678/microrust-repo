#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;

#[allow(unused_imports)]
use panic_rtt_target;

use rtt_target::{rtt_init_print, rprintln};
use microbit::Board;

// #[allow(unused_imports)]
// use panic_halt;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    loop {}
}
