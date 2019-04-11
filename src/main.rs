// system clock source
// 1- use pll hse extc (external 8 MHz clock)
// 2- use pll hse xtal (external 8 MHz xtal) (X3 on board)
// 3- use pll hsi (internal 16 MHz clock)
//
// sysclk: 84MHz
// ahbclk: 84MHz
// apb1clk: 42MHz
// apb2clk: 84MHz

#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use core::fmt::Write as _core_fmt_Write;

extern crate stm32f4xx_hal as hal;

use cortex_m_semihosting::hio;
use rt::{entry, exception, ExceptionFrame};

use hal::gpio::GpioExt as _gpio_GpioExt;

// timer
use hal::{
    block,
    hal::timer::CountDown, // enable: wait()
    rcc::RccExt,
    time::U32Ext, // enable: hz()
    timer::Timer,
};

// usart
use hal::{
    hal::serial::Write as _hal_serial_Write,
    serial::{config::Config, Serial},
};

mod led;
use led::{LED, LED2};

#[entry]
fn main() -> ! {
    let cp = hal::stm32::CorePeripherals::take().unwrap();
    let p = hal::stm32::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let clocks: hal::rcc::Clocks = rcc.cfgr.freeze();
    // let clocks: hal::rcc::Clocks = rcc.cfgr.sysclk(84.mhz()).freeze();

    let gpioa = p.GPIOA.split();
    let mut led2 = LED2::new(gpioa.pa5.into_push_pull_output());

    let timeout = 1.hz();
    let mut timer = Timer::syst(cp.SYST, timeout, clocks);

    // usart2
    let pin_tx = gpioa.pa2.into_alternate_af7();
    let pin_rx = gpioa.pa3.into_alternate_af7();

    let usart2 = Serial::usart2(
        p.USART2,
        (pin_tx, pin_rx),
        Config::default().baudrate(115_200.bps()),
        clocks,
    )
    .unwrap();
    let (mut tx, mut _rx) = usart2.split();

    led2.turn_off();

    // let _rec = block!(rx.read()).unwrap();

    let mut count = 0;
    loop {
        block!(timer.wait()).unwrap();
        led2.blinky();

        count = (count + 1) & 0b0111;
        block!(tx.write(b'0' + count)).ok();
        write!(tx, "hello, world\r").unwrap();
        write!(tx, "good bye\r").unwrap();
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    if let Ok(mut hstdout) = hio::hstdout() {
        writeln!(hstdout, "{:#?}", ef).ok();
    }
    loop {}
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("unhandled exception (IRQn={})", irqn);
}
