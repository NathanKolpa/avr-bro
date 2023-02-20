//! The ATmega328P microcontroller.
//!
//! ## Additional resources:
//! - [Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf)

pub use registers::*;

use crate::controller_traits::*;

mod registers;

pub struct Atmega328P;

impl CpuFrequency for Atmega328P {
    fn cycles_per_millisecond() -> u32 {
        16000
    }
}
