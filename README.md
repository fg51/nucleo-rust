# nucleo-rust
NUCLEO-F401RE via rust

## run server

```sh
$ ./qemu-system-gnuarmeclipse.sh ./target/thumbv7em-none-eabi/debug/nucle-rust
```

## connect server

```sh
$ arm-none-eabi-gdb ./target/thumbv7em-none-eabi/debug/nucle-rust
```

## pin assign
| PIN NAME | NUMBER |
| -------- | ------ |
| LED1     | PA 5   |
| BUTTON   | PC 13  |
| UART2 TX | PA 2   |
| UART2 RX | PA 3   |
| I2C SCL  | PB 8   |
| I2C SDA  | PB 9   |
| PWM OUT  | PB 3   |

## internal ADC channels
* temperature 0xF0
* VREF 0xF1
* VBAT 0xF2


## output clock on MC01 pin(PA8) for debugging purpose
HAL_RCC_MCOConfig(RCC_MCO1, RCC_MCO1SOURCE_HSI, RCC_MCODIV_1); 16 MHz
