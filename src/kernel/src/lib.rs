//! This crate contains everything related to the kernel of the OS.
#![no_std]
#![no_main]

pub mod arch;
pub mod entry;
pub mod memlayout;
pub mod params;
pub mod spinlock;
pub mod start;
pub mod uart;
