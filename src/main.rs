#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;

extern crate cortex_m;
extern crate stm32f401re_hal as hal;

// use core::fmt::Write;
use rt::ExceptionFrame;
// use sh::hio;

use hal::gpio::gpioa::PA5;
use hal::gpio::GpioExt; // GPIO.split
use hal::gpio::{Output, PushPull};
use hal::hal::digital::OutputPin;
use hal::rcc::RccExt; // RCC.constrain
use hal::stm32f401::Peripherals;

use cortex_m::asm::delay;

entry!(main);

fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let mut syst = cp.SYST;
    syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    syst.set_reload(8_000_000); // 1 [s] (core freq: 8 [MHz]
    syst.enable_counter();

    let mut rcc = p.RCC.constrain();
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb1);

    let mut pa5: PA5<Output<PushPull>> = gpioa
        .pa5
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    // print().unwrap();

    pa5.set_low(); // turn off. reboot: turn on
    delay(1_000_000);
    pa5.set_high(); // turn off. reboot: turn on

    loop {
        while !syst.has_wrapped() {}
        // delay(10_000_000);
        pa5.set_low(); // turn off. reboot: turn on

        while !syst.has_wrapped() {}
        // delay(10_000_000);
        pa5.set_high(); // turn off. reboot: turn on
    }
}

// fn print() -> Result<(), core::fmt::Error> {
//     let mut stdout = match hio::hstdout() {
//         Ok(fd) => fd,
//         Err(()) => return Err(core::fmt::Error),
//     };
//     let language = "Rust";
//
//     write!(stdout, "{} on embedded !", language)?;
//     Ok(())
// }

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("unhandled exception (IRQn={})", irqn);
}
