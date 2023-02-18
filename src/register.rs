//! This module provides a low-level interface for working with 8-bit hardware registers.

use core::ptr::{read_volatile, write_volatile};

/// A pointer to a mutable 8-bit unsigned integer, representing the memory address of a hardware register.
pub type RegisterAddress = *mut u8;

/// A hardware register, representing a memory location with a specific address that can be read from and written to.
///
/// # Safety
///
/// The `unsafe` methods for this struct uses [`read_volatile`] and [`write_volatile`] functions to read from and write
/// to the memory location pointed to by the `address` field of the `Register` instance.
#[derive(Clone, Eq, Ord, PartialOrd, PartialEq)]
pub struct Register {
    address: RegisterAddress,
}

impl Register {
    /// Creates a new `Register` instance with the specified memory address.
    #[inline]
    pub const fn new(address: RegisterAddress) -> Self {
        Self { address }
    }

    /// Sets the value of a single bit in the hardware register to 1
    ///
    /// # Arguments
    ///
    /// * `bit` - The index of the bit to set, where 0 is the least significant bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use avr_bro::register::{Register, RegisterAddress};
    /// # use std::ptr;
    /// let mut  mock_register: u8 = 0;
    /// let register = Register::new(&mut mock_register as *mut u8);
    ///
    /// // Set bit 4 of the register to 1
    /// unsafe {
    ///     register.set_bit(3);
    ///     assert_eq!(8, register.read())
    /// };
    /// ```
    #[inline]
    pub unsafe fn set_bit(&self, bit: u8) {
        write_volatile(
            self.address,
            read_volatile(self.address) | Self::bit_value(bit),
        )
    }

    /// Sets the value of a single bit in the hardware register to 0
    ///
    /// # Arguments
    ///
    /// * `bit` - The index of the bit to set, where 0 is the least significant bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use avr_bro::register::{Register, RegisterAddress};
    /// # use std::ptr;
    /// let mut  mock_register: u8 = 3;
    /// let register = Register::new(&mut mock_register as *mut u8);
    ///
    /// // Set bit 0 of the register to 0
    /// unsafe {
    ///     register.clear_bit(0);
    ///     assert_eq!(2, register.read())
    /// };
    /// ```
    #[inline]
    pub unsafe fn clear_bit(&self, bit: u8) {
        write_volatile(
            self.address,
            read_volatile(self.address) & !Self::bit_value(bit),
        )
    }

    /// Toggle the value of a single bit in the hardware register
    ///
    /// # Arguments
    ///
    /// * `bit` - The index of the bit to set, where 0 is the least significant bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use avr_bro::register::{Register, RegisterAddress};
    /// # use std::ptr;
    /// let mut  mock_register: u8 = 0;
    /// let register = Register::new(&mut mock_register as *mut u8);
    ///
    /// // Toggle bit 2 of the register to 1 and vice versa
    /// unsafe {
    ///     register.toggle_bit(1);
    ///     assert_eq!(2, register.read());
    ///
    ///     register.toggle_bit(1);
    ///     assert_eq!(0, register.read())
    /// };
    /// ```
    #[inline]
    pub unsafe fn toggle_bit(&self, bit: u8) {
        write_volatile(
            self.address,
            read_volatile(self.address) ^ Self::bit_value(bit),
        )
    }

    /// Writes a byte value to the hardware register.
    ///
    /// # Arguments
    ///
    /// * `value` - The byte value to write to the register.
    ///
    /// # Example
    ///
    /// ```
    /// # use avr_bro::register::{Register, RegisterAddress};
    /// # use std::ptr;
    /// let mut  mock_register: u8 = 10;
    /// let register = Register::new(&mut mock_register as *mut u8);
    ///
    /// // Write the value 60 to the register
    /// unsafe {
    ///     register.write(60);
    ///     assert_eq!(60, register.read());
    /// };
    /// ```
    #[inline]
    pub unsafe fn write(&self, value: u8) {
        write_volatile(self.address, value)
    }

    /// Reads a byte value from the hardware register.
    ///
    /// # Returns
    ///
    /// The byte value read from the register
    ///
    /// ```
    /// # use avr_bro::register::{Register, RegisterAddress};
    /// # use std::ptr;
    /// let mut  mock_register: u8 = 15;
    /// let register = Register::new(&mut mock_register as *mut u8);
    ///
    /// // Read the value from the register
    /// unsafe {
    ///     assert_eq!(15, register.read());
    /// };
    /// ```
    #[inline]
    pub unsafe fn read(&self) -> u8 {
        read_volatile(self.address)
    }

    /// Returns the memory address of this `Register`.
    pub fn address(&self) -> RegisterAddress {
        self.address
    }

    #[inline]
    fn bit_value(bit: u8) -> u8 {
        1 << bit
    }
}
