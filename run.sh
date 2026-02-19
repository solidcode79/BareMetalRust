#!/bin/bash
./build.sh

qemu-system-arm \
  -M lm3s6965evb \
  -kernel target/thumbv7m-none-eabi/debug/RustRTOS \
  -display none \
  -serial stdio
