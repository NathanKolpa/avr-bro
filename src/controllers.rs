//! The module containing all code specific to each microcontroller.

#[cfg(any(feature = "atmega328p", doc))]
#[doc(cfg(feature = "atmega328p"))]
pub mod atmega328p;
