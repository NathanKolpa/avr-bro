//! The ATmega328P microcontroller.
//!
//! ## Additional resources:
//! - [Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf)

pub mod registers;
pub const CYCLES_PER_MILLISECOND: u32 = 16000;

pub struct Atmega328P;

impl CpuFrequency for Atmega328P {
    fn cycles_per_millisecond() -> u32 {
        16000
    }
}
