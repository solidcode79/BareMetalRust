#!/bin/bash
# see .cargo/config for some magic and wonder why you can't pass it from here
cargo clean
cargo build --features lm3s6965
# Two things ELF file and actual bin file
# cargo objcopy --release --features lm3s6965 -- -O binary rtos.bin
