#![no_std]
#![feature(panic_info_message)]

#[cfg(feature = "x86")]
mod x86;

mod panic;
