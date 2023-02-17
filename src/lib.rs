#![doc = include_str!("../README.md")]
#![no_std]
#![feature(doc_cfg)]

pub mod controllers;

#[cfg(feature = "hal")]
#[doc(cfg(feature = "hal"))]
pub mod hal;
