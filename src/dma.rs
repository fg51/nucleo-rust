#![allow(non_camel_case_types)]

extern crate stm32f4xx_hal as hal;

use hal::adc::Adc;
use hal::rcc::Clocks;
// use hal::time::{Hertz, U32Ext as __hal_time_u32ext};
use hal::time::Hertz;

// const ADC_CR2_ADON_Pos: u16 = 0u16;
// const ADC_CR2_ADON_Msk: u32 = (0x1u32 << ADC_CR2_ADON_Pos); // 0x00000001
// const ADC_CR2_ADON: u32 = ADC_CR2_ADON_Msk; // A/D Converter ON / OFF

// const ADC_CR2_SWSTART: u32 = 3;
const ADC_STAB_DELAY_US: u32 = 3;

const ADC_CR1_JAUTO_Pos: u32 = 10;
const ADC_CR1_JAUTO_Msk: u32 = 0x1 << ADC_CR1_JAUTO_Pos;
const ADC_CR1_JAUTO: u32 = ADC_CR1_JAUTO_Msk;

const RESET: u32 = 0;

// pub fn adc_start_dma(ADC_HandleTypeDef* hadc,  pData: usize, uint32_t Length) {
pub fn adc_start_dma(adc: &mut Adc<hal::stm32::ADC1>, clocks: &Clocks) {
    // Check the parameters
    // assert_param(IS_FUNCTIONAL_STATE(hadc.Init.ContinuousConvMode));
    // assert_param(IS_ADC_EXT_TRIG_EDGE(hadc.Init.ExternalTrigConvEdge));

    // Process locked
    // __HAL_LOCK(hadc);

    /* Enable the ADC peripheral
     * Check if ADC peripheral is disabled in order to enable it and wait during
     * Tstab time the ADC's stabilization
     */
    //if (adc.Instance.CR2 & ADC_CR2_ADON) != ADC_CR2_ADON {
    if adc.is_enabled() == false {
        // Enable the Peripheral
        adc.enable(); // __HAL_ADC_ENABLE(adc);

        // Delay for ADC stabilization time
        // Compute number of CPU cycles to wait for

        // counter = ADC_STAB_DELAY_US * (SystemCoreClock / 1000000);
        let Hertz(clk) = clocks.sysclk();
        let mut counter = ADC_STAB_DELAY_US * (clk / 1000000);
        while counter != 0 {
            counter -= 1;
        }
    }

    /* Start conversion if ADC is effectively enabled */
    // if(HAL_IS_BIT_SET(hadc.Instance.CR2, ADC_CR2_ADON)) {
    if adc.is_enabled() == true {
        /* Set ADC state
         * - Clear state bitfield related to regular group conversion results
         * - Set state bitfield related to regular group operation
         */

        // ADC_STATE_CLR_SET(
        //     hadc->State,
        //     HAL_ADC_STATE_READY | HAL_ADC_STATE_REG_EOC | HAL_ADC_STATE_REG_OVR,
        //     HAL_ADC_STATE_REG_BUSY);
        ADC_STATE_CLR_SET(
            adc.State,
            ADCState::Ready as u32 | ADCState::Reg_EOC as u32 | ADCState::Reg_OVR as u32,
            ADCState::Reg_Busy as u32,
        );

        /* If conversions on group regular are also triggering group injected,
         * update ADC state.                                                         */
        if READ_BIT(adc.Instance.CR1, ADC_CR1_JAUTO) != RESET {
          ADC_STATE_CLR_SET(
              adc.State, ADCState::INJ_EOC, ADCState::INJ_BUSY);
        }

        /* State machine update: Check if an injected conversion is ongoing */
        // if (HAL_IS_BIT_SET(adc.State, HAL_ADC_STATE_INJ_BUSY)) {
        //   /* Reset ADC error code fields related to conversions on group regular */
        //   CLEAR_BIT(adc.ErrorCode, (HAL_ADC_ERROR_OVR | HAL_ADC_ERROR_DMA));
        // } else {
        //   /* Reset ADC all error code fields */
        //   ADC_CLEAR_ERRORCODE(adc);
        // }

        // /* Process unlocked */
        // // Unlock before starting ADC conversions: in case of potential
        // // interruption, to let the process to ADC IRQ Handler.
        // __HAL_UNLOCK(adc);

        // /* Pointer to the common control register to which is belonging hadc
        //  * (Depending on STM32F4 product, there may be up to 3 ADCs and 1 common
        //  * control register)
        //  */
        // let mut tmpADC_Common: ADC_Common_TypeDef = ADC_COMMON_REGISTER(adc);

        // /* Set the DMA transfer complete callback */
        // adc.DMA_Handle.XferCpltCallback = ADC_DMAConvCplt;

        // /* Set the DMA half transfer complete callback */
        // adc.DMA_Handle.XferHalfCpltCallback = ADC_DMAHalfConvCplt;

        // /* Set the DMA error callback */
        // adc.DMA_Handle.XferErrorCallback = ADC_DMAError;

        // /* Manage ADC and DMA start: ADC overrun interruption, DMA start, ADC
        //  * start (in case of SW start):
        //  * Clear regular group conversion flag and overrun flag
        //  * (To ensure of no unknown state from
        //  * potential previous ADC operations)
        //  */
        // __HAL_ADC_CLEAR_FLAG(adc, ADC_FLAG_EOC | ADC_FLAG_OVR);

        // // Enable ADC overrun interrupt
        // __HAL_ADC_ENABLE_IT(adc, ADC_IT_OVR);

        // // Enable ADC DMA mode
        // adc.Instance.CR2 |= ADC_CR2_DMA;

        // // Start the DMA channel
        // HAL_DMA_Start_IT(adc.DMA_Handle, (uint32_t)&adc.Instance.DR, (uint32_t)pData, Length);

        // // Check if Multimode enabled
        // if(HAL_IS_BIT_CLR(tmpADC_Common.CCR, ADC_CCR_MULTI)) {

        //   // if((hadc.Instance == ADC1) || ((hadc.Instance == ADC2) && ((ADC.CCR & ADC_CCR_MULTI_Msk) < ADC_CCR_MULTI_0)) \
        //                               //|| ((hadc.Instance == ADC3) && ((ADC.CCR & ADC_CCR_MULTI_Msk) < ADC_CCR_MULTI_4)))
        //   if(
        //       (adc.Instance == ADC1)
        //       || ((adc.Instance == ADC2) && ((ADC.CCR & ADC_CCR_MULTI_Msk) < ADC_CCR_MULTI_0))
        //       || ((adc.Instance == ADC3) && ((ADC.CCR & ADC_CCR_MULTI_Msk) < ADC_CCR_MULTI_4)))
        //   {
        //     /* if no external trigger present
        //      * enable software conversion of regular channels
        //      */
        //     if((adc.Instance.CR2 & ADC_CR2_EXTEN) == RESET) {
        //       // Enable the selected ADC software conversion for regular group
        //       adc.Instance.CR2 |= (uint32_t)ADC_CR2_SWSTART;
        //     }
        //   }
        // } else {
        //   /* if instance of handle correspond to ADC1
        //    * and  no external trigger present
        //    * enable software conversion of regular channels
        //    */
        //   if((adc.Instance == ADC1) && ((adc.Instance.CR2 & ADC_CR2_EXTEN) == RESET))
        //   {
        //     /* Enable the selected ADC software conversion for regular group */
        //       adc.Instance.CR2 |= (uint32_t)ADC_CR2_SWSTART;
        //   }
        // }
    }
}

struct ADC_Common_TypeDef {
    csr: u32, // ADC Common status register, Address offset: ADC1 base address + 0x300 */
    ccr: u32, // ADC common control register, Address offset: ADC1 base address + 0x304 */
    cdr: u32, // ADC common regular data register for dual AND triple modes,                            Address offset: ADC1 base address + 0x308 */
}

/// The number of cycles to sample a given channel for
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ADCState {
    Reset,
    Ready,
    Busy_Internal,
    Timeout,
    Reg_Busy,
    Reg_EOC,
    Reg_OVR,
    INJ_Busy,
    INJ_EOC,
}

impl From<u32> for ADCState {
    fn from(f: u32) -> ADCState {
        match f {
            0x0000_0000 => ADCState::Reset,
            0x0000_0001 => ADCState::Ready,
            0x0000_0002 => ADCState::Busy_Internal,
            0x0000_0004 => ADCState::Timeout,
            0x0000_0100 => ADCState::Reg_Busy,
            0x0000_0200 => ADCState::Reg_EOC,
            0x0000_0400 => ADCState::Reg_OVR,
            0x0000_1000 => ADCState::INJ_Busy,
            0x0000_2000 => ADCState::INJ_EOC,
            _ => unimplemented!(),
        }
    }
}

impl From<ADCState> for u32 {
    fn from(l: ADCState) -> u32 {
        match l {
            ADCState::Reset => 0x0000_0000,
            ADCState::Ready => 0x0000_0001,
            ADCState::Busy_Internal => 0x0000_0002,
            ADCState::Timeout => 0x0000_0004,
            ADCState::Reg_Busy => 0x0000_0100, // A conversion on group regular is ongoing or can occur (either by continuous mode,
            ADCState::Reg_EOC => 0x0000_0200,  // Conversion data available on group regular */
            ADCState::Reg_OVR => 0x0000_0400,  // Overrun occurrence */
            ADCState::INJ_Busy => 0x0000_1000, /* A conversion on group injected is ongoing or can occur (either by auto-injection mode,
            external t, low power auto power-on (if feature available), multimode ADC master control (if feature available)) */
            ADCState::INJ_EOC => 0x0000_2000, // Conversion data available on group injected */
        }
    }
}


fn ADC_STATE_CLR_SET(reg: &mut usize, clear_mask:u32, set_mask:u32){
    modify_reg(reg: &mut usize, clear_mask, set_mask);
}

fn modify_reg(reg &mut usize, clearmask: u32, setmask: u32) {
    write_reg(reg, (((read_reg(reg)) & (~clearmask)) | (setmask)))
}

fn write_reg(reg &mut usize, val: u32) {
    reg = val;
}

fn read_reg(reg){
    return reg;
}

/*
typedef struct __ADC_HandleTypeDef
{
  ADC_TypeDef                   *Instance;                   /*!< Register base address */

  ADC_InitTypeDef               Init;                        /*!< ADC required parameters */

  __IO uint32_t                 NbrOfCurrentConversionRank;  /*!< ADC number of current conversion rank */

  DMA_HandleTypeDef             *DMA_Handle;                 /*!< Pointer DMA Handler */

  HAL_LockTypeDef               Lock;                        /*!< ADC locking object */

  __IO uint32_t                 State;                       /*!< ADC communication state */

  __IO uint32_t                 ErrorCode;                   /*!< ADC Error code */
#if (USE_HAL_ADC_REGISTER_CALLBACKS == 1)
  void (* ConvCpltCallback)(struct __ADC_HandleTypeDef *hadc);              /*!< ADC conversion complete callback */
  void (* ConvHalfCpltCallback)(struct __ADC_HandleTypeDef *hadc);          /*!< ADC conversion DMA half-transfer callback */
  void (* LevelOutOfWindowCallback)(struct __ADC_HandleTypeDef *hadc);      /*!< ADC analog watchdog 1 callback */
  void (* ErrorCallback)(struct __ADC_HandleTypeDef *hadc);                 /*!< ADC error callback */
  void (* InjectedConvCpltCallback)(struct __ADC_HandleTypeDef *hadc);      /*!< ADC group injected conversion complete callback */
  void (* MspInitCallback)(struct __ADC_HandleTypeDef *hadc);               /*!< ADC Msp Init callback */
  void (* MspDeInitCallback)(struct __ADC_HandleTypeDef *hadc);             /*!< ADC Msp DeInit callback */
#endif /* USE_HAL_ADC_REGISTER_CALLBACKS */
}ADC_HandleTypeDef;
*/

struct ADC_HandleTypeDef {
  ADC_TypeDef  *Instance;                   /*!< Register base address */

  ADC_InitTypeDef               Init;                        /*!< ADC required parameters */

  __IO uint32_t                 NbrOfCurrentConversionRank;  /*!< ADC number of current conversion rank */

  DMA_HandleTypeDef             *DMA_Handle;                 /*!< Pointer DMA Handler */

  HAL_LockTypeDef               Lock;                        /*!< ADC locking object */

  __IO uint32_t                 State;                       /*!< ADC communication state */

  __IO uint32_t                 ErrorCode;                   /*!< ADC Error code */
// #if (USE_HAL_ADC_REGISTER_CALLBACKS == 1)
  void (* ConvCpltCallback)(struct __ADC_HandleTypeDef *hadc);              /*!< ADC conversion complete callback */
  void (* ConvHalfCpltCallback)(struct __ADC_HandleTypeDef *hadc);          /*!< ADC conversion DMA half-transfer callback */
  void (* LevelOutOfWindowCallback)(struct __ADC_HandleTypeDef *hadc);      /*!< ADC analog watchdog 1 callback */
  void (* ErrorCallback)(struct __ADC_HandleTypeDef *hadc);                 /*!< ADC error callback */
  void (* InjectedConvCpltCallback)(struct __ADC_HandleTypeDef *hadc);      /*!< ADC group injected conversion complete callback */
  void (* MspInitCallback)(struct __ADC_HandleTypeDef *hadc);               /*!< ADC Msp Init callback */
  void (* MspDeInitCallback)(struct __ADC_HandleTypeDef *hadc);             /*!< ADC Msp DeInit callback */
// #endif /* USE_HAL_ADC_REGISTER_CALLBACKS */
