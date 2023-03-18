//! This program controls a seven-segments display by printing the values
//! from 0 to 9 on it.

#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use seven_segment::SevenSegmentPins;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
	let dp = arduino_hal::Peripherals::take().unwrap();
	let pins = arduino_hal::pins!(dp);

    // structuring the seven-segments' pins by mapping it with the Arduino
    // UNO's digital pins
    let mut seven_seg = SevenSegmentPins {
        a: pins.d6.into_output(),
        b: pins.d5.into_output(),
        c: pins.d2.into_output(),
        d: pins.d3.into_output(),
        e: pins.d4.into_output(),
        f: pins.d7.into_output(),
        g: pins.d8.into_output()
    }.with_common_anode(); // assembling a common-anode seven-segments display

	loop {
        // This for loop iterates over a range from 0 to 9
        // and prints those values on the seven-segments display
        for i in 0..=9 {
            seven_seg.set(i).unwrap();
            delay_ms(250);
        }
    }
}
