#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]

pub extern crate embedded_hal as hal;
extern crate stm32f4;

pub use stm32f4::stm32f401;

pub mod gpio;
pub mod rcc;
