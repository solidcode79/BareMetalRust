# Bare Metal Rust

## Motivation
The motivation for doing this is the fundamental principle that true understanding comes from building things from the ground up. 

Here I plan on constructing a software stack for embedded and IoT applications entirely from scratch, using Rust programming language and QEMU simulation (I tried Renode but found QEMU simpler/leaner for this case).

The primary goal is to create a self-contained ecosystem, starting with a bare metal boot loader and extending to include components such as a memory manager, task scheduler, logger, and exception handler. By embarking on this journey, I aim to (re)gain a deep(er) understanding of the inner workings of ARM Cortex M7® based embedded systems  while harnessing the power of Rust's safety and expressiveness.

The aim is to NOT include ANY RUST crate (no external crate) to keep the code understandable. 


## Datasheets
To better understand and work with the lm3s6965 microcontroller [datasheet here](https://www.ti.com/product/LM3S6965), will need to refer to the following resources:

## Installation
Getting started with this project is straightforward; you only need to install two essential components:

1. Rust: You can install it by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
2. QEMU: You can install it by following instructions on the [Official QEMU website] (https://www.qemu.org/download/)
## Tags