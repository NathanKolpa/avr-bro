//! Special instructions for the AVR architecture

#[cfg(target_arch = "avr")]
use core::arch::asm;

#[inline]
pub fn no_op() {
    #[cfg(target_arch = "avr")]
    unsafe {
        asm!("nop")
    }

    #[cfg(not(target_arch = "avr"))]
    unimplemented!()
}
