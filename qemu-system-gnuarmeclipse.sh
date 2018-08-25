#!/bin/sh

set -eux
: $1

qemu_command=$HOME/opt/xPacks/@gnu-mcu-eclipse/qemu/2.8.0-3.1/.content/bin/qemu-system-gnuarmeclipse
board_name=NUCLEO-F411RE
mcu_name=STM32F411RE

$qemu_command \
  --verbose --verbose \
  --board $board_name \
  --gdb tcp::3333 \
  --mcu $mcu_name \
  -d unimp,guest_errors \
  --image $1 \
  -nographic \
  --semihosting-config enable=on,target=native \
  --semihosting-cmdline $1
