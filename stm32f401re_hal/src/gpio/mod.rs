//! General Purpose Input / Output
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(dead_code)]

use core::marker::PhantomData;

use rcc::AHB1;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self, ahb: &mut AHB1) -> Self::Parts;
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;
/// Pulled down input (type state)
pub struct PullDown;
/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;
/// Open drain output (type state)
pub struct OpenDrain;

/// Alternate function 0 (type state)
pub struct AF0;

/// Alternate function 1 (type state)
pub struct AF1;

/// Alternate function 2 (type state)
pub struct AF2;

/// Alternate function 3 (type state)
pub struct AF3;

/// Alternate function 4 (type state)
pub struct AF4;

/// Alternate function 5 (type state)
pub struct AF5;

/// Alternate function 6 (type state)
pub struct AF6;

/// Alternate function 7 (type state)
pub struct AF7;

/// Alternate function 8 (type state)
pub struct AF8;

/// Alternate function 9 (type state)
pub struct AF9;

/// Alternate function 10 (type state)
pub struct AF10;

/// Alternate function 11 (type state)
pub struct AF11;

/// Alternate function 12 (type state)
pub struct AF12;

/// Alternate function 13 (type state)
pub struct AF13;

/// Alternate function 14 (type state)
pub struct AF14;

/// Alternate function 15 (type state)
pub struct AF15;

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $gpioy:ident, $iopxenr:ident, $iopxrst:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty, $AFR:ident),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use core::marker::PhantomData;

            use hal::digital::OutputPin;
            use stm32f401::{$gpioy, $GPIOX};

            use rcc::AHB1;
            use super::{
                AF4, AF5, AF6, AF7, Floating, GpioExt, Input, OpenDrain, Output,
                PullDown, PullUp, PushPull,
            };

            /// GPIO parts
            pub struct Parts {
                /// Opaque AFRH register
                pub afrh: AFRH,
                /// Opaque AFRL register
                pub afrl: AFRL,
                /// Opaque MODER register
                pub moder: MODER,
                /// Opaque OTYPER register
                pub otyper: OTYPER,
                /// Opaque PUPDR register
                pub pupdr: PUPDR,
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self, ahb: &mut AHB1) -> Parts {
                    ahb.enr().modify(|_, w| w.$iopxenr().enabled());
                    ahb.rstr().modify(|_, w| w.$iopxrst().set_bit());
                    ahb.rstr().modify(|_, w| w.$iopxrst().clear_bit());

                    Parts {
                        afrh: AFRH { _0: () },
                        afrl: AFRL { _0: () },
                        moder: MODER { _0: () },
                        otyper: OTYPER { _0: () },
                        pupdr: PUPDR { _0: () },
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            /// Opaque AFRL register
            pub struct AFRL {
                _0: (),
            }

            impl AFRL {
                pub(crate) fn afr(&mut self) -> &$gpioy::AFRL {
                    unsafe { &(*$GPIOX::ptr()).afrl }
                }
            }

            /// Opaque AFRH register
            pub struct AFRH {
                _0: (),
            }

            impl AFRH {
                pub(crate) fn afr(&mut self) -> &$gpioy::AFRH {
                    unsafe { &(*$GPIOX::ptr()).afrh }
                }
            }

            /// Opaque MODER register
            pub struct MODER {
                _0: (),
            }

            impl MODER {
                pub(crate) fn moder(&mut self) -> &$gpioy::MODER {
                    unsafe { &(*$GPIOX::ptr()).moder }
                }
            }

            /// Opaque OTYPER register
            pub struct OTYPER {
                _0: (),
            }

            impl OTYPER {
                pub(crate) fn otyper(&mut self) -> &$gpioy::OTYPER {
                    unsafe { &(*$GPIOX::ptr()).otyper }
                }
            }

            /// Opaque PUPDR register
            pub struct PUPDR {
                _0: (),
            }

            impl PUPDR {
                pub(crate) fn pupdr(&mut self) -> &$gpioy::PUPDR {
                    unsafe { &(*$GPIOX::ptr()).pupdr }
                }
            }

            /// Partially erased pin
            pub struct $PXx<MODE> {
                i: u8,
                _mode: PhantomData<MODE>,
            }

            impl<MODE> OutputPin for $PXx<Output<MODE>> {
                fn set_high(&mut self) {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << self.i)) }
                }

                fn set_low(&mut self) {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (16 + self.i))) }
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    /// Configures the pin to serve as alternate function 4 (AF4)
                    pub fn into_af4(
                        self,
                        moder: &mut MODER,
                        afr: &mut $AFR,
                    ) -> $PXi<AF4> {
                        let offset = 2 * $i;

                        // alternate function mode
                        let mode = 0b10;
                        moder.moder().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                        });

                        let af = 4;
                        let offset = 4 * ($i % 8);
                        afr.afr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to serve as alternate function 5 (AF5)
                    pub fn into_af5(
                        self,
                        moder: &mut MODER,
                        afr: &mut $AFR,
                    ) -> $PXi<AF5> {
                        let offset = 2 * $i;

                        // alternate function mode
                        let mode = 0b10;
                        moder.moder().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                        });

                        let af = 5;
                        let offset = 4 * ($i % 8);
                        afr.afr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to serve as alternate function 6 (AF6)
                    pub fn into_af6(
                        self,
                        moder: &mut MODER,
                        afr: &mut $AFR,
                    ) -> $PXi<AF6> {
                        let offset = 2 * $i;

                        // alternate function mode
                        let mode = 0b10;
                        moder.moder().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                        });

                        let af = 6;
                        let offset = 4 * ($i % 8);
                        afr.afr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to serve as alternate function 7 (AF7)
                    pub fn into_af7(
                        self,
                        moder: &mut MODER,
                        afr: &mut $AFR,
                    ) -> $PXi<AF7> {
                        let offset = 2 * $i;

                        // alternate function mode
                        let mode = 0b10;
                        moder.moder().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                        });

                        let af = 7;
                        let offset = 4 * ($i % 8);

                        afr.afr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a floating input pin
                    pub fn into_floating_input(
                        self,
                        moder: &mut MODER,
                        pupdr: &mut PUPDR,
                    ) -> $PXi<Input<Floating>> {
                        let offset = 2 * $i;

                        // input mode
                        moder
                            .moder()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << offset)) });

                        // no pull-up or pull-down
                        pupdr
                            .pupdr()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << offset)) });

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled down input pin
                    pub fn into_pull_down_input(
                        self,
                        moder: &mut MODER,
                        pupdr: &mut PUPDR,
                    ) -> $PXi<Input<PullDown>> {
                        let offset = 2 * $i;

                        // input mode
                        moder
                            .moder()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << offset)) });

                        // pull-down
                        pupdr.pupdr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (0b10 << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled up input pin
                    pub fn into_pull_up_input(
                        self,
                        moder: &mut MODER,
                        pupdr: &mut PUPDR,
                    ) -> $PXi<Input<PullUp>> {
                        let offset = 2 * $i;

                        // input mode
                        moder
                            .moder()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << offset)) });

                        // pull-up
                        pupdr.pupdr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an open drain output pin
                    pub fn into_open_drain_output(
                        self,
                        moder: &mut MODER,
                        otyper: &mut OTYPER,
                    ) -> $PXi<Output<OpenDrain>> {
                        let offset = 2 * $i;

                        // general purpose output mode
                        let mode = 0b01;
                        moder.moder().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                        });

                        // open drain output
                        otyper
                            .otyper()
                            .modify(|r, w| unsafe { w.bits(r.bits() | (0b1 << $i)) });

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an push pull output pin
                    pub fn into_push_pull_output(
                        self,
                        moder: &mut MODER,
                        otyper: &mut OTYPER,
                    ) -> $PXi<Output<PushPull>> {
                        let offset = 2 * $i;

                        // general purpose output mode
                        let mode = 0b01;
                        moder.moder().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                        });

                        // push pull output
                        otyper
                            .otyper()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b1 << $i)) });

                        $PXi { _mode: PhantomData }
                    }
                }

                impl $PXi<Output<OpenDrain>> {
                    /// Enables / disables the internal pull up
                    pub fn internal_pull_up(&mut self, pupdr: &mut PUPDR, on: bool) {
                        let offset = 2 * $i;

                        pupdr.pupdr().modify(|r, w| unsafe {
                            w.bits(
                                (r.bits() & !(0b11 << offset)) | if on {
                                    0b01 << offset
                                } else {
                                    0
                                },
                            )
                        });
                    }
                }

                impl<MODE> $PXi<Output<MODE>> {
                    /// Erases the pin number from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> $PXx<Output<MODE>> {
                        $PXx {
                            i: $i,
                            _mode: self._mode,
                        }
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    fn set_high(&mut self) {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << $i)) }
                    }

                    fn set_low(&mut self) {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (16 + $i))) }
                    }
                }
            )+
        }
    }
}

gpio!(GPIOA, gpioa, gpioa, gpioaen, gpioarst, PAx, [
    PA0: (pa0, 0, Input<Floating>, AFRL),
    PA1: (pa1, 1, Input<Floating>, AFRL),
    PA2: (pa2, 2, Input<Floating>, AFRL),
    PA3: (pa3, 3, Input<Floating>, AFRL),
    PA4: (pa4, 4, Input<Floating>, AFRL),
    PA5: (pa5, 5, Input<Floating>, AFRL),
    PA6: (pa6, 6, Input<Floating>, AFRL),
    PA7: (pa7, 7, Input<Floating>, AFRL),
    PA8: (pa8, 8, Input<Floating>, AFRH),
    PA9: (pa9, 9, Input<Floating>, AFRH),
    PA10: (pa10, 10, Input<Floating>, AFRH),
    PA11: (pa11, 11, Input<Floating>, AFRH),
    PA12: (pa12, 12, Input<Floating>, AFRH),
    PA13: (pa13, 13, Input<Floating>, AFRH),
    PA14: (pa14, 14, Input<Floating>, AFRH),
    PA15: (pa15, 15, Input<Floating>, AFRH),
]);

gpio!(GPIOB, gpiob, gpiob, gpioben, gpiobrst, PBx, [
    PB0: (pb0, 0, Input<Floating>, AFRL),
    PB1: (pb1, 1, Input<Floating>, AFRL),
    PB2: (pb2, 2, Input<Floating>, AFRL),
    PB3: (pb3, 3, Input<Floating>, AFRL),
    PB4: (pb4, 4, Input<Floating>, AFRL),
    PB5: (pb5, 5, Input<Floating>, AFRL),
    PB6: (pb6, 6, Input<Floating>, AFRL),
    PB7: (pb7, 7, Input<Floating>, AFRL),
    PB8: (pb8, 8, Input<Floating>, AFRH),
    PB9: (pb9, 9, Input<Floating>, AFRH),
    PB10: (pb10, 10, Input<Floating>, AFRH),
    PB11: (pb11, 11, Input<Floating>, AFRH),
    PB12: (pb12, 12, Input<Floating>, AFRH),
    PB13: (pb13, 13, Input<Floating>, AFRH),
    PB14: (pb14, 14, Input<Floating>, AFRH),
    PB15: (pb15, 15, Input<Floating>, AFRH),
]);

// gpio!( GPIOC, gpioc, gpioc, gpiocen, gpiocrst, PCx, [
//     PB0: (pb0, 0, Input<Floating>, AFRL),
//     PB1: (pb1, 1, Input<Floating>, AFRL),
// //     PB2: (pb2, 2, Input<Floating>, AFRL),
// //     PB3: (pb3, 3, Input<Floating>, AFRL),
// //     PB4: (pb4, 4, Input<Floating>, AFRL),
// //     PB5: (pb5, 5, Input<Floating>, AFRL),
// //     PB6: (pb6, 6, Input<Floating>, AFRL),
// //     PB7: (pb7, 7, Input<Floating>, AFRL),
// //     PB8: (pb8, 8, Input<Floating>, AFRH),
// //     PB9: (pb9, 9, Input<Floating>, AFRH),
// //     PB10: (pb10, 10, Input<Floating>, AFRH),
// //     PB11: (pb11, 11, Input<Floating>, AFRH),
// //     PB12: (pb12, 12, Input<Floating>, AFRH),
// //     PB13: (pb13, 13, Input<Floating>, AFRH),
// //     PB14: (pb14, 14, Input<Floating>, AFRH),
// //     PB15: (pb15, 15, Input<Floating>, AFRH),
// ]);
