#![doc = include_str!("../README.md")]
#![no_std]
#![feature(doc_cfg)]

pub mod controllers;

pub mod register;

#[cfg(any(feature = "hal", doc))]
#[doc(cfg(feature = "hal"))]
pub mod hal;

pub use avr_bro_macros::avr_main;
