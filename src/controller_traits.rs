use crate::avr::no_op;

pub trait CpuFrequency {
    /// Returns the current number of CPU cycles executed per millisecond on the system.
    fn cycles_per_millisecond() -> u32;
}

pub trait Delay {
    /// Blocks the process for the specified number of milliseconds in a busy-wait loop.
    /// It's not guaranteed that this function will complete in the exact amount of time specified.
    /// This is due to interrupts possibly executing while this function is running.
    ///
    /// # Arguments
    ///
    /// * `millis` - The amount of time in milliseconds this function should take to execute.
    fn blocking_delay(milliseconds: u32);
}

impl<T: CpuFrequency> Delay for T {
    fn blocking_delay(milliseconds: u32) {
        const CYCLES_PER_LOOP: u32 = 15;
        let cycles_needed = T::cycles_per_millisecond() * milliseconds / CYCLES_PER_LOOP;

        // Generated assembly code for this loop:
        // cpi     r24, 0x00
        // cpc     r25, r1
        // breq    .+14
        // ldi     r30, 0x000
        // ldi     r31, 0x00
        // nop
        // adiw    r30, 0x01
        // cp      r30, r24
        // cpc     r31, r25
        // brcs    .-10
        for _ in 0..cycles_needed {
            no_op();
        }
    }
}
