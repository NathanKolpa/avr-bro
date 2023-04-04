use crate::register::{Register, RegisterAddress};

/// The status register
///
/// # Internals
///
/// No address location is specified in the AVR Datasheet.
/// However we can copy [gcc's avr-lib implementation](https://github.com/avrdudes/avr-libc/blob/502f5091d2b49191a87eb4a3a926525a2a34926f/include/avr/common.h#L99)
pub const STATUS_REGISTER: Register = Register::new(0x3F as RegisterAddress);

#[cfg(target_arch = "avr")]
use core::arch::asm;

/// Clears the Global Interrupt Enable (I) bit in [`STATUS_REGISTER`].
/// The interrupts will be immediately disabled.
/// No interrupt will be executed after the [`disable_interrupts`] call, even if it occurs simultaneously.
///
/// # Internals
///
/// This function is equivalent to the `cli` instruction with zero overhead.
/// [AVR Datasheet, 6.37 CLI – Clear Global Interrupt Enable Bit](https://ww1.microchip.com/downloads/en/DeviceDoc/AVR-InstructionSet-Manual-DS40002198.pdf#_OPENTOPIC_TOC_PROCESSING_d1951e30539)
#[inline]
pub fn disable_interrupts() {
    #[cfg(target_arch = "avr")]
    unsafe {
        asm!("cli")
    }

    #[cfg(not(target_arch = "avr"))]
    unimplemented!()
}

/// Sets the Global Interrupt Enable (I) bit in [`STATUS_REGISTER`].
/// The instruction following the [`enable_interrupts`] call will be executed before any pending interrupts.
///
/// This function is equivalent to the `sei` instruction with zero overhead.
/// [AVR Datasheet, 6.104 SEI – Set Global Interrupt Enable Bit](https://ww1.microchip.com/downloads/en/DeviceDoc/AVR-InstructionSet-Manual-DS40002198.pdf#_OPENTOPIC_TOC_PROCESSING_d1951e63086)
#[inline]
pub fn enable_interrupts() {
    #[cfg(target_arch = "avr")]
    unsafe {
        asm!("sei")
    }

    #[cfg(not(target_arch = "avr"))]
    unimplemented!()
}