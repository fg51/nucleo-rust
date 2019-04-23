//! direct memory access
#![deny(missing_docs)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

// use crate::{gpio::*, stm32};
use crate::stm32;
use core::{fmt, ptr};

// PROVIDE(DMA2_STREAM0 = DefaultHandler);
const DMA2_Stream0_IRQn: u32 = 56;
// macro_rules! DMA2_Stream0_IRQn1 {
//     () => {
//         56 as c_int
//     };
// }

// static void MX_DMA_Init(void)
fn dma_init() {
    /* DMA controller clock enable */
    __HAL_RCC_DMA2_CLK_ENABLE();

    /* DMA interrupt init */
    /* DMA2_Stream0_IRQn interrupt configuration */
    // HAL_NVIC_SetPriority(DMA2_Stream0_IRQn, 0, 0);
    HAL_NVIC_EnableIRQ(DMA2_Stream0_IRQn);
}

// void __HAL_RCC_DMA2_CLK_ENABLE(void) {
fn __HAL_RCC_DMA2_CLK_ENABLE() {
    // SET_BIT(RCC->AHB1ENR, RCC_AHB1ENR_DMA2EN());
    unsafe {
        let rcc = &(*stm32::RCC::ptr());
        rcc.ahb1enr.modify(|_, w| w.dma2en().enabled()); //.set_bit();
    }
    /* Delay after an RCC peripheral clock enabling */
    // let tmpreg: u32 = READ_BIT(RCC->AHB1ENR, RCC_AHB1ENR_DMA2EN);
    //UNUSED(tmpreg);
    unsafe {
        let rcc = &(*stm32::RCC::ptr());
        rcc.ahb1enr.read().dma2en().is_enabled();
    }
}

// fn SET_BIT(reg, bit) {
//     reg |= bit
// }
//
// fn READ_BIT(reg, bit) {
//     reg & bit
// }
//
// const fn RCC_AHB1ENR_DMA2EN(){
//     let RCC_AHB1ENR_DMA2EN_Pos = 22;
//     let RCC_AHB1ENR_DMA2EN_Msk = 0x1 << RCC_AHB1ENR_DMA2EN_Pos;
//     return RCC_AHB1ENR_DMA2EN_Msk;
// }

//__STATIC_INLINE void __NVIC_EnableIRQ(IRQn_Type IRQn)
// fn HAL_NVIC_EnableIRQ(IRQn_Type IRQn) {
//   if IRQn >= 0 {
//     NVIC->ISER[IRQn >> 5] = 1 << (IRQn & 0x1F);
//   }
// }
