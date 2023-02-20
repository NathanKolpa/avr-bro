//! The module containing all code specific to each microcontroller.

#[cfg(any(feature = "atmega328p", doc))]
mod atmega328p;

#[cfg(any(feature = "atmega328p", doc))]
#[doc(cfg(feature = "atmega328p"))]
pub use atmega328p::*;
