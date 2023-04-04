use core::time::Duration;

#[inline]
fn no_op() {
    #[cfg(target_arch = "avr")]
    unsafe {
        use core::arch::asm;
        asm!("nop")
    }

    #[cfg(not(target_arch = "avr"))]
    unimplemented!()
}


/// Block the running thread for the specified duration in a busy-wait loop.
///
/// # Internals
///
/// This function calculates how many loop iterations are needed and then runs said amount.
pub fn blocking_delay(cpu_cycles_per_ms: u32, duration: Duration) {
    const CYCLES_PER_LOOP: u32 = 15;
    let cycles_needed = cpu_cycles_per_ms * duration.as_millis() as u32 / CYCLES_PER_LOOP;

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