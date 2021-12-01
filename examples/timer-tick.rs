// TODO: monotonic, and dispatchers.

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [USART1])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};

    use stm32f4xx_hal as hal;

    use hal::gpio::{gpioc::PC13, Output, PushPull};
    use hal::pac;
    //use hal::prelude::*;
    use hal::gpio::GpioExt as _stm32f4xx_hal_gpio_GpioExt;
    use hal::rcc::RccExt as _stm32f4xx_hal_rcc_RccExt;
    use hal::time::U32Ext as _stm32f4xx_hal_time_U32Ext;
    use hal::timer::{monotonic::MonoTimer, Timer}; // for the split method.

    use fugit::ExtU32 as _ExtU32;

    const SYSTEM_CLOCK: u32 = 84;
    const FREQ: u32 = 1_000_000; // 1 [MHz] = 1[usec]

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PC13<Output<PushPull>>,
    }

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = MonoTimer<pac::TIM2, FREQ>;

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(SYSTEM_CLOCK.mhz()).freeze();

        let gpioc = ctx.device.GPIOC.split();
        let led = gpioc.pc13.into_push_pull_output();

        let mono = Timer::new(ctx.device.TIM2, &clocks).monotonic();
        tick::spawn().ok();
        hprintln!("init").unwrap();

        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[idle(local = [x: u32 = 0])]
    fn idle(cx: idle::Context) -> ! {
        // Locals in idle have lifetime 'static
        let _x: &'static mut u32 = cx.local.x;

        hprintln!("idle").unwrap();

        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator

        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(local = [led])]
    fn tick(ctx: tick::Context) {
        tick::spawn_after(1.secs()).ok();
        ctx.local.led.toggle();
    }
}
