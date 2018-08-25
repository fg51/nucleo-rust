#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;

use core::fmt::Write;
use rt::ExceptionFrame;
use sh::hio;

entry!(main);

fn main() -> ! {
    print().unwrap();
    loop {}
}

fn print() -> Result<(), core::fmt::Error> {
    let mut stdout = match hio::hstdout() {
        Ok(fd) => fd,
        Err(()) => return Err(core::fmt::Error),
    };
    let language = "Rust";

    write!(stdout, "{} on embedded !", language)?;
    Ok(())
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("unhandled exception (IRQn={})", irqn);
}
