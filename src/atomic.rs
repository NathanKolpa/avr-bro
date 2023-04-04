use crate::interrupts::{disable_interrupts, STATUS_REGISTER};

/// Run the callback passed in the argument `critical_section` with the guarantee that no memory/register
/// is modified during it's execution.
///
/// # Internals
///
/// This function works by saving the [`STATUS_REGISTER`] to a temporary variable,
/// disabling interrupts and restoring the [`STATUS_REGISTER`] after execution.
///
/// This method of running critical sections is also used in:
/// - [avr-lib](https://github.com/avrdudes/avr-libc/blob/502f5091d2b49191a87eb4a3a926525a2a34926f/include/util/atomic.h)
/// - [portable-atomic](https://github.com/taiki-e/portable-atomic/blob/4f934499db33c4217141dae8ec1379b253c28f02/src/imp/interrupt/avr.rs)
#[inline]
pub fn run_atomic_block<F: Fn()>(critical_section: F) {
    let old_value = unsafe { STATUS_REGISTER.read() };
    disable_interrupts();

    critical_section();

    unsafe { STATUS_REGISTER.write(old_value); }
}