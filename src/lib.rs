#![doc = include_str!("../README.md")]
#![no_std]
#![feature(doc_cfg)]
#![cfg_attr(target_arch = "avr", feature(asm_experimental_arch))]

pub mod controllers;
pub mod register;
pub mod delay;
pub mod atomic;
pub mod interrupts;

pub use avr_bro_macros::avr_main;
