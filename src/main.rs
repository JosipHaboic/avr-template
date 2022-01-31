#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;
use crate::time_it::millis;

mod time_it;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    loop {
    }
}
