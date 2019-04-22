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
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use core::fmt::Write as _core_fmt_Write;

extern crate stm32f4xx_hal as hal;

use cortex_m_semihosting::hio;
use rt::{entry, exception, ExceptionFrame};

use hal::gpio::GpioExt as _gpio_GpioExt;

use hal::{
    block,
    rcc::RccExt,
    time::U32Ext, // enable: hz()
};

// timer
use hal::timer::Timer;

// ADC
use hal::adc::{
    config::{AdcConfig, Clock, Eoc, SampleTime, Scan, Sequence},
    Adc,
};

// usart
use hal::{
    hal::serial::Write as _hal_serial_Write,
    serial::{config::Config, Serial},
};

mod led;
use led::{LED, LED2};

use hal::timer;

use core::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

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
    let mut syst = Timer::syst(cp.SYST, timeout, clocks);
    syst.listen(timer::Event::TimeOut);

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

    let mut adc1: hal::adc::Adc<hal::stm32::ADC1> = Adc::adc1(
        p.ADC1,
        true,
        AdcConfig::default()
            .end_of_conversion_interrupt(Eoc::Conversion)
            .scan(Scan::Enabled)
            .clock(Clock::Pclk2_div_8),
    );
    let pa0 = gpioa.pa0.into_analog();
    adc1.configure_channel(&pa0, Sequence::One, SampleTime::Cycles_480);

    led2.turn_off();
    block!(tx.write(b'!')).ok();
    block!(tx.write(b'\r')).ok();
    adc1.start_conversion();

    let mut count = 0;
    loop {
        if COUNTER.load(Ordering::Relaxed) == 1 {
            COUNTER.store(0, Ordering::Relaxed);
            count += 2;
            if count == 2 {
                count = 0;
                let sample = adc1.convert(&pa0, SampleTime::Cycles_480);
                let millivolts: u16 = adc1.sample_to_millivolts(sample);
                write!(tx, "pa0: {0} [mV]\r", millivolts,).ok();
            }
        }
    }
}

#[exception]
fn SysTick() -> ! {
    // static mut TX_UART: Option<Tx<USART2>> = None;
    static mut EX_LED2: Option<LED2> = None;

    if EX_LED2.is_none() {
        let p = hal::stm32::Peripherals::take().unwrap();
        let gpioa = p.GPIOA.split();
        *EX_LED2 = Some(LED2::new(gpioa.pa5.into_push_pull_output()));
    }
    if let Some(led) = EX_LED2.as_mut() {
        COUNTER.store(1, Ordering::Relaxed);
        led.blinky();
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
