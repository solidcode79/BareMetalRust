#!/bin/bash
./build.sh

qemu-system-arm \
  -M lm3s6965evb \
  -kernel target/thumbv7m-none-eabi/debug/rtos-hello \
  -display none \
  -serial stdio
