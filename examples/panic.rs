#![no_main]
#![no_std]

use panic_halt as _;

use stm32f407g_disc as board;

// use crate::board::{
//     hal::stm32,
//     hal::{delay::Delay, prelude::*},
//     led::{LedColor, Leds},
// };

use cortex_m::{iprintln, peripheral::ITM};
use cortex_m_rt::entry;
// use core::panic::PanicInfo;

// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     iprintln!(info.message());
// }

#[entry]
fn main() -> ! {
    // panic!("Hello, world!");
    let mut itm = cortex_m::Peripherals::take().unwrap().ITM;
    iprintln!(&mut itm.stim[0], "Hello, world!");

    loop {}
}
