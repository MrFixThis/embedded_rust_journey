//! This program controls a little circuit that toggles the state of two leds.
//!
//! This is the MrFixThis' style 'Hello Wold' to the low level world using Rust!

#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
	let dp = arduino_hal::Peripherals::take().unwrap();
	let pins = arduino_hal::pins!(dp);

	let mut led2 = pins.d12.into_output();
	let mut led3 = pins.d11.into_output();

	loop {
		led2.toggle();
		arduino_hal::delay_ms(150);

		led3.toggle();
		arduino_hal::delay_ms(150);
    }
}
