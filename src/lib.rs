#![doc = include_str!("../README.md")]
#![no_std]
#![feature(doc_cfg)]
#![cfg_attr(target_arch = "avr", feature(asm_experimental_arch))]

pub mod controllers;

pub mod register;

#[cfg(any(feature = "hal", doc))]
#[doc(cfg(feature = "hal"))]
pub mod hal;

pub mod avr;
pub mod controller_traits;

pub use avr_bro_macros::avr_main;
