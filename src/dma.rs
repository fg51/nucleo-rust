#![allow(non_camel_case_types)]

extern crate stm32f4xx_hal as hal;

use hal::adc::{config, Adc};
use hal::rcc::Clocks;
// use hal::time::{Hertz, U32Ext as __hal_time_u32ext};
use hal::time::Hertz;

// const ADC_CR2_ADON_Pos: u16 = 0u16;
// const ADC_CR2_ADON_Msk: u32 = (0x1u32 << ADC_CR2_ADON_Pos); // 0x00000001
// const ADC_CR2_ADON: u32 = ADC_CR2_ADON_Msk; // A/D Converter ON / OFF

// const ADC_CR2_SWSTART: u32 = 3;
const ADC_STAB_DELAY_US: u32 = 3;

const fn ADC_CR1_JAUTO() -> u32 {
    const ADC_CR1_JAUTO_Pos: u32 = 10;
    const ADC_CR1_JAUTO_Msk: u32 = 0x1 << ADC_CR1_JAUTO_Pos;
    return ADC_CR1_JAUTO_Msk;
}

const RESET: u32 = 0;

static mut pData: [u32; 2] = [0; 2];

// pub fn adc_start_dma(ADC_HandleTypeDef* hadc,  pData: usize, uint32_t Length) {
pub fn adc_start_dma(adc: &mut Adc<hal::stm32::ADC1>, clocks: &Clocks, length: u32) {
    let mut adc_state = 0;
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
            adc_state,
            ADCState::Ready as u32 | ADCState::Reg_EOC as u32 | ADCState::Reg_OVR as u32,
            ADCState::Reg_Busy as u32,
        );

        /* If conversions on group regular are also triggering group injected,
         * update ADC state.                                                         */
        //if READ_BIT(adc.Instance.CR1, ADC_CR1_JAUTO()) != RESET {
        // if (adc.adc_reg.cr1.read().bits() == ADC_CR1_JAUTO()) != RESET {
        // ADC_STATE_CLR_SET(adc.State, ADCState::INJ_EOC, ADCState::INJ_BUSY);
        adc_state = ADC_STATE_CLR_SET(
            adc_state,
            ADCState::INJ_EOC as u32,
            ADCState::INJ_Busy as u32,
        );
        //}

        /* State machine update: Check if an injected conversion is ongoing */
        // if (HAL_IS_BIT_SET(adc.State, HAL_ADC_STATE_INJ_BUSY)) {
        //     /* Reset ADC error code fields related to conversions on group regular */
        //     CLEAR_BIT(adc.ErrorCode, (HAL_ADC_ERROR_OVR | HAL_ADC_ERROR_DMA));
        // } else {
        //     /* Reset ADC all error code fields */
        //     ADC_CLEAR_ERRORCODE(adc);
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
        __HAL_ADC_CLEAR_FLAG(adc, ADC_FLAG_EOC | ADC_FLAG_OVR);

        // Enable ADC overrun interrupt
        __HAL_ADC_ENABLE_IT(adc, ADC_IT_OVR);

        // // Enable ADC DMA mode
        // adc.Instance.CR2 |= ADC_CR2_DMA;
        adc.set_dma(config::Dma::Continuous);

        // // Start the DMA channel
        HAL_DMA_Start_IT(
            adc.DMA_Handle,
            adc.data_register_address(),
            &mut pData,
            length,
        );

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

fn ADC_STATE_CLR_SET(reg: u32, clear_mask: u32, set_mask: u32) -> u32 {
    // modify_reg(reg: &mut u32, clear_mask, set_mask);
    return modify_reg(reg, clear_mask, set_mask);
}

fn modify_reg(reg: u32, clearmask: u32, setmask: u32) -> u32 {
    // write_reg(reg, (((read_reg(reg)) & (~clearmask)) | (setmask)))
    return reg & ((!clearmask) | setmask);
}

// fn write_reg(reg: &mut u32, val: u32) {
//     reg = val;
// }
//
// fn read_reg(reg){
//     return reg;
// }

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
    Instance: ADC_TypeDef,           // Register base address
    Init: ADC_InitTypeDef,           // ADC required parameters
    NbrOfCurrentConversionRank: u32, // ADC number of current conversion rank
    // DMA_Handle: DMA_HandleTypeDef,   // Pointer DMA Handler
    Lock: HAL_LockTypeDef, // ADC locking object
    State: u32,            // ADC communication state
    ErrorCode: u32,        // ADC Error code

                           // #if (USE_HAL_ADC_REGISTER_CALLBACKS == 1)
                           //  void (* ConvCpltCallback)(struct __ADC_HandleTypeDef *hadc);  // ADC conversion complete callback
                           //  void (* ConvHalfCpltCallback)(struct __ADC_HandleTypeDef *hadc);  // ADC conversion DMA half-transfer callback
                           //  void (* LevelOutOfWindowCallback)(struct __ADC_HandleTypeDef *hadc);  // ADC analog watchdog 1 callback
                           //  void (* ErrorCallback)(struct __ADC_HandleTypeDef *hadc);                 // ADC error callback
                           //  void (* InjectedConvCpltCallback)(struct __ADC_HandleTypeDef *hadc);      // ADC group injected conversion complete callback */
                           //  void (* MspInitCallback)(struct __ADC_HandleTypeDef *hadc);               // ADC Msp Init callback
                           //  void (* MspDeInitCallback)(struct __ADC_HandleTypeDef *hadc);             // ADC Msp DeInit callback
                           // #endif /* USE_HAL_ADC_REGISTER_CALLBACKS */
}

//typedef struct
//{
//  __IO uint32_t SR;     /*!< ADC status register,                         Address offset: 0x00 */
//  __IO uint32_t CR1;    /*!< ADC control register 1,                      Address offset: 0x04 */
//  __IO uint32_t CR2;    /*!< ADC control register 2,                      Address offset: 0x08 */
//  __IO uint32_t SMPR1;  /*!< ADC sample time register 1,                  Address offset: 0x0C */
//  __IO uint32_t SMPR2;  /*!< ADC sample time register 2,                  Address offset: 0x10 */
//  __IO uint32_t JOFR1;  /*!< ADC injected channel data offset register 1, Address offset: 0x14 */
//  __IO uint32_t JOFR2;  /*!< ADC injected channel data offset register 2, Address offset: 0x18 */
//  __IO uint32_t JOFR3;  /*!< ADC injected channel data offset register 3, Address offset: 0x1C */
//  __IO uint32_t JOFR4;  /*!< ADC injected channel data offset register 4, Address offset: 0x20 */
//  __IO uint32_t HTR;    /*!< ADC watchdog higher threshold register,      Address offset: 0x24 */
//  __IO uint32_t LTR;    /*!< ADC watchdog lower threshold register,       Address offset: 0x28 */
//  __IO uint32_t SQR1;   /*!< ADC regular sequence register 1,             Address offset: 0x2C */
//  __IO uint32_t SQR2;   /*!< ADC regular sequence register 2,             Address offset: 0x30 */
//  __IO uint32_t SQR3;   /*!< ADC regular sequence register 3,             Address offset: 0x34 */
//  __IO uint32_t JSQR;   /*!< ADC injected sequence register,              Address offset: 0x38*/
//  __IO uint32_t JDR1;   /*!< ADC injected data register 1,                Address offset: 0x3C */
//  __IO uint32_t JDR2;   /*!< ADC injected data register 2,                Address offset: 0x40 */
//  __IO uint32_t JDR3;   /*!< ADC injected data register 3,                Address offset: 0x44 */
//  __IO uint32_t JDR4;   /*!< ADC injected data register 4,                Address offset: 0x48 */
//  __IO uint32_t DR;     /*!< ADC regular data register,                   Address offset: 0x4C */
//} ADC_TypeDef;

struct ADC_TypeDef {
    SR: u32,    // ADC status register,                          Address offset: 0x00
    CR1: u32,   //  ADC control register 1,                      Address offset: 0x04
    CR2: u32,   //  ADC control register 2,                      Address offset: 0x08
    SMPR1: u32, //  ADC sample time register 1,                  Address offset: 0x0C
    SMPR2: u32, //  ADC sample time register 2,                  Address offset: 0x10
    JOFR1: u32, //  ADC injected channel data offset register 1, Address offset: 0x14
    JOFR2: u32, //  ADC injected channel data offset register 2, Address offset: 0x18
    JOFR3: u32, //  ADC injected channel data offset register 3, Address offset: 0x1C
    JOFR4: u32, //  ADC injected channel data offset register 4, Address offset: 0x20
    HTR: u32,   //  ADC watchdog higher threshold register,      Address offset: 0x24
    LTR: u32,   //  ADC watchdog lower threshold register,       Address offset: 0x28
    SQR1: u32,  //  ADC regular sequence register 1,             Address offset: 0x2C
    SQR2: u32,  //  ADC regular sequence register 2,             Address offset: 0x30
    SQR3: u32,  //  ADC regular sequence register 3,             Address offset: 0x34
    JSQR: u32,  //  ADC injected sequence register,              Address offset: 0x38
    JDR1: u32,  //  ADC injected data register 1,                Address offset: 0x3C
    JDR2: u32,  //  ADC injected data register 2,                Address offset: 0x40
    JDR3: u32,  //  ADC injected data register 3,                Address offset: 0x44
    JDR4: u32,  //  ADC injected data register 4,                Address offset: 0x48
    DR: u32,    //  ADC regular data register,                   Address offset: 0x4C
}

/**
 * @brief  Structure definition of ADC and regular group initialization
 * @note   Parameters of this structure are shared within 2 scopes:
 *          - Scope entire ADC (affects regular and injected groups): ClockPrescaler, Resolution, ScanConvMode, DataAlign, ScanConvMode, EOCSelection, LowPowerAutoWait, LowPowerAutoPowerOff, ChannelsBank.
 *          - Scope regular group: ContinuousConvMode, NbrOfConversion, DiscontinuousConvMode, NbrOfDiscConversion, ExternalTrigConvEdge, ExternalTrigConv.
 * @note   The setting of these parameters with function HAL_ADC_Init() is conditioned to ADC state.
 *         ADC state can be either:
 *          - For all parameters: ADC disabled
 *          - For all parameters except 'Resolution', 'ScanConvMode', 'DiscontinuousConvMode', 'NbrOfDiscConversion' : ADC enabled without conversion on going on regular group.
 *          - For parameters 'ExternalTrigConv' and 'ExternalTrigConvEdge': ADC enabled, even with conversion on going.
 *         If ADC is not in the appropriate state to modify some parameters, these parameters setting is bypassed
 *         without error reporting (as it can be the expected behaviour in case of intended action to update another parameter (which fulfills the ADC state condition) on the fly).
 */
//typedef struct
//{
//  uint32_t ClockPrescaler;               /*!< Select ADC clock prescaler. The clock is common for
//                                              all the ADCs.
//                                              This parameter can be a value of @ref ADC_ClockPrescaler */
//  uint32_t Resolution;                   /*!< Configures the ADC resolution.
//                                              This parameter can be a value of @ref ADC_Resolution */
//  uint32_t DataAlign;                    /*!< Specifies ADC data alignment to right (MSB on register bit 11 and LSB on register bit 0) (default setting)
//                                              or to left (if regular group: MSB on register bit 15 and LSB on register bit 4, if injected group (MSB kept as signed value due to potential negative value after offset application): MSB on register bit 14 and LSB on register bit 3).
//                                              This parameter can be a value of @ref ADC_Data_align */
//  uint32_t ScanConvMode;                 /*!< Configures the sequencer of regular and injected groups.
//                                              This parameter can be associated to parameter 'DiscontinuousConvMode' to have main sequence subdivided in successive parts.
//                                              If disabled: Conversion is performed in single mode (one channel converted, the one defined in rank 1).
//                                                           Parameters 'NbrOfConversion' and 'InjectedNbrOfConversion' are discarded (equivalent to set to 1).
//                                              If enabled:  Conversions are performed in sequence mode (multiple ranks defined by 'NbrOfConversion'/'InjectedNbrOfConversion' and each channel rank).
//                                                           Scan direction is upward: from rank1 to rank 'n'.
//                                              This parameter can be set to ENABLE or DISABLE */
//  uint32_t EOCSelection;                 /*!< Specifies what EOC (End Of Conversion) flag is used for conversion by polling and interruption: end of conversion of each rank or complete sequence.
//                                              This parameter can be a value of @ref ADC_EOCSelection.
//                                              Note: For injected group, end of conversion (flag&IT) is raised only at the end of the sequence.
//                                                    Therefore, if end of conversion is set to end of each conversion, injected group should not be used with interruption (HAL_ADCEx_InjectedStart_IT)
//                                                    or polling (HAL_ADCEx_InjectedStart and HAL_ADCEx_InjectedPollForConversion). By the way, polling is still possible since driver will use an estimated timing for end of injected conversion.
//                                              Note: If overrun feature is intended to be used, use ADC in mode 'interruption' (function HAL_ADC_Start_IT() ) with parameter EOCSelection set to end of each conversion or in mode 'transfer by DMA' (function HAL_ADC_Start_DMA()).
//                                                    If overrun feature is intended to be bypassed, use ADC in mode 'polling' or 'interruption' with parameter EOCSelection must be set to end of sequence */
//  FunctionalState ContinuousConvMode;    /*!< Specifies whether the conversion is performed in single mode (one conversion) or continuous mode for regular group,
//                                              after the selected trigger occurred (software start or external trigger).
//                                              This parameter can be set to ENABLE or DISABLE. */
//  uint32_t NbrOfConversion;              /*!< Specifies the number of ranks that will be converted within the regular group sequencer.
//                                              To use regular group sequencer and convert several ranks, parameter 'ScanConvMode' must be enabled.
//                                              This parameter must be a number between Min_Data = 1 and Max_Data = 16. */
//  FunctionalState DiscontinuousConvMode; /*!< Specifies whether the conversions sequence of regular group is performed in Complete-sequence/Discontinuous-sequence (main sequence subdivided in successive parts).
//                                              Discontinuous mode is used only if sequencer is enabled (parameter 'ScanConvMode'). If sequencer is disabled, this parameter is discarded.
//                                              Discontinuous mode can be enabled only if continuous mode is disabled. If continuous mode is enabled, this parameter setting is discarded.
//                                              This parameter can be set to ENABLE or DISABLE. */
//  uint32_t NbrOfDiscConversion;          /*!< Specifies the number of discontinuous conversions in which the  main sequence of regular group (parameter NbrOfConversion) will be subdivided.
//                                              If parameter 'DiscontinuousConvMode' is disabled, this parameter is discarded.
//                                              This parameter must be a number between Min_Data = 1 and Max_Data = 8. */
//  uint32_t ExternalTrigConv;             /*!< Selects the external event used to trigger the conversion start of regular group.
//                                              If set to ADC_SOFTWARE_START, external triggers are disabled.
//                                              If set to external trigger source, triggering is on event rising edge by default.
//                                              This parameter can be a value of @ref ADC_External_trigger_Source_Regular */
//  uint32_t ExternalTrigConvEdge;         /*!< Selects the external trigger edge of regular group.
//                                              If trigger is set to ADC_SOFTWARE_START, this parameter is discarded.
//                                              This parameter can be a value of @ref ADC_External_trigger_edge_Regular */
//  FunctionalState DMAContinuousRequests; /*!< Specifies whether the DMA requests are performed in one shot mode (DMA transfer stop when number of conversions is reached)
//											  or in Continuous mode (DMA transfer unlimited, whatever number of conversions).
//											  Note: In continuous mode, DMA must be configured in circular mode. Otherwise an overrun will be triggered when DMA buffer maximum pointer is reached.
//											  Note: This parameter must be modified when no conversion is on going on both regular and injected groups (ADC disabled, or ADC enabled without continuous mode or external trigger that could launch a conversion).
//											  This parameter can be set to ENABLE or DISABLE. */
//}ADC_InitTypeDef;

struct ADC_InitTypeDef {
    ClockPrescaler: u32, /* Select ADC clock prescaler. The clock is common for
                         all the ADCs.
                         This parameter can be a value of @ref ADC_ClockPrescaler */
    Resolution: u32, /* Configures the ADC resolution.
                     This parameter can be a value of @ref ADC_Resolution */
    DataAlign: u32, /* Specifies ADC data alignment to right (MSB on register bit 11 and LSB on register bit 0) (default setting)
                    or to left (if regular group: MSB on register bit 15 and LSB on register bit 4, if injected group (MSB kept as signed value due to potential negative value after offset application): MSB on register bit 14 and LSB on register bit 3).
                    This parameter can be a value of @ref ADC_Data_align */
    ScanConvMode: u32, /* Configures the sequencer of regular and injected groups.
                       This parameter can be associated to parameter 'DiscontinuousConvMode' to have main sequence subdivided in successive parts.
                       If disabled: Conversion is performed in single mode (one channel converted, the one defined in rank 1).
                                    Parameters 'NbrOfConversion' and 'InjectedNbrOfConversion' are discarded (equivalent to set to 1).
                       If enabled:  Conversions are performed in sequence mode (multiple ranks defined by 'NbrOfConversion'/'InjectedNbrOfConversion' and each channel rank).
                                    Scan direction is upward: from rank1 to rank 'n'.
                       This parameter can be set to ENABLE or DISABLE */
    EOCSelection: u32, /* Specifies what EOC (End Of Conversion) flag is used for conversion by polling and interruption: end of conversion of each rank or complete sequence.
                       This parameter can be a value of @ref ADC_EOCSelection.
                       Note: For injected group, end of conversion (flag&IT) is raised only at the end of the sequence.
                             Therefore, if end of conversion is set to end of each conversion, injected group should not be used with interruption (HAL_ADCEx_InjectedStart_IT)
                             or polling (HAL_ADCEx_InjectedStart and HAL_ADCEx_InjectedPollForConversion). By the way, polling is still possible since driver will use an estimated timing for end of injected conversion.
                       Note: If overrun feature is intended to be used, use ADC in mode 'interruption' (function HAL_ADC_Start_IT() ) with parameter EOCSelection set to end of each conversion or in mode 'transfer by DMA' (function HAL_ADC_Start_DMA()).
                             If overrun feature is intended to be bypassed, use ADC in mode 'polling' or 'interruption' with parameter EOCSelection must be set to end of sequence */
    ContinuousConvMode: FunctionalState, /* Specifies whether the conversion is performed in single mode (one conversion) or continuous mode for regular group,
                                         after the selected trigger occurred (software start or external trigger).
                                         This parameter can be set to ENABLE or DISABLE. */
    NbrOfConversion: u32, /* Specifies the number of ranks that will be converted within the regular group sequencer.
                          To use regular group sequencer and convert several ranks, parameter 'ScanConvMode' must be enabled.
                          This parameter must be a number between Min_Data = 1 and Max_Data = 16. */
    DiscontinuousConvMode: FunctionalState, /* Specifies whether the conversions sequence of regular group is performed in Complete-sequence/Discontinuous-sequence (main sequence subdivided in successive parts).
                                            Discontinuous mode is used only if sequencer is enabled (parameter 'ScanConvMode'). If sequencer is disabled, this parameter is discarded.
                                            Discontinuous mode can be enabled only if continuous mode is disabled. If continuous mode is enabled, this parameter setting is discarded.
                                            This parameter can be set to ENABLE or DISABLE. */
    NbrOfDiscConversion: u32, /* Specifies the number of discontinuous conversions in which the  main sequence of regular group (parameter NbrOfConversion) will be subdivided.
                              If parameter 'DiscontinuousConvMode' is disabled, this parameter is discarded.
                              This parameter must be a number between Min_Data = 1 and Max_Data = 8. */
    ExternalTrigConv: u32, /* Selects the external event used to trigger the conversion start of regular group.
                           If set to ADC_SOFTWARE_START, external triggers are disabled.
                           If set to external trigger source, triggering is on event rising edge by default.
                           This parameter can be a value of @ref ADC_External_trigger_Source_Regular */
    ExternalTrigConvEdge: u32, /* Selects the external trigger edge of regular group.
                               If trigger is set to ADC_SOFTWARE_START, this parameter is discarded.
                               This parameter can be a value of @ref ADC_External_trigger_edge_Regular */
    DMAContinuousRequests: FunctionalState, /* Specifies whether the DMA requests are performed in one shot mode (DMA transfer stop when number of conversions is reached)
                                            or in Continuous mode (DMA transfer unlimited, whatever number of conversions).
                                            Note: In continuous mode, DMA must be configured in circular mode. Otherwise an overrun will be triggered when DMA buffer maximum pointer is reached.
                                            Note: This parameter must be modified when no conversion is on going on both regular and injected groups (ADC disabled, or ADC enabled without continuous mode or external trigger that could launch a conversion).
                                            This parameter can be set to ENABLE or DISABLE. */
}

pub enum FunctionalState {
    DISABLE,
    ENABLE,
}

impl From<u32> for FunctionalState {
    fn from(f: u32) -> FunctionalState {
        match f {
            0x0000_0000 => FunctionalState::DISABLE,
            0x0000_0001 => FunctionalState::ENABLE,
            _ => unimplemented!(),
        }
    }
}

impl From<FunctionalState> for u32 {
    fn from(l: FunctionalState) -> u32 {
        match l {
            FunctionalState::DISABLE => 0x0000_0000,
            FunctionalState::ENABLE => 0x0000_0001,
        }
    }
}

fn READ_BIT(reg: u32, bit: u32) -> u32 {
    reg & bit
}

enum HAL_LockTypeDef {
    HAL_UNLOCKED,
    HAL_LOCKED,
}

// struct DMA_HandleTypeDef
// {
//   DMA_Stream_TypeDef         *Instance;                                                        /*!< Register base address                  */
//
//   DMA_InitTypeDef            Init;                                                             /*!< DMA communication parameters           */
//
//   HAL_LockTypeDef            Lock;                                                             /*!< DMA locking object                     */
//
//   __IO HAL_DMA_StateTypeDef  State;                                                            /*!< DMA transfer state                     */
//
//   void                       *Parent;                                                          /*!< Parent object state                    */
//
//   void                       (* XferCpltCallback)( struct __DMA_HandleTypeDef * hdma);         /*!< DMA transfer complete callback         */
//
//   void                       (* XferHalfCpltCallback)( struct __DMA_HandleTypeDef * hdma);     /*!< DMA Half transfer complete callback    */
//
//   void                       (* XferM1CpltCallback)( struct __DMA_HandleTypeDef * hdma);       /*!< DMA transfer complete Memory1 callback */
//
//   void                       (* XferM1HalfCpltCallback)( struct __DMA_HandleTypeDef * hdma);   /*!< DMA transfer Half complete Memory1 callback */
//
//   void                       (* XferErrorCallback)( struct __DMA_HandleTypeDef * hdma);        /*!< DMA transfer error callback            */
//
//   void                       (* XferAbortCallback)( struct __DMA_HandleTypeDef * hdma);        /*!< DMA transfer Abort callback            */
//
//   __IO uint32_t              ErrorCode;                                                        /*!< DMA Error code                          */
//
//   uint32_t                   StreamBaseAddress;                                                /*!< DMA Stream Base Address                */
//
//   uint32_t                   StreamIndex;                                                      /*!< DMA Stream Index                       */
//
// }
