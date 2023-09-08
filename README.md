# Bare Metal Rust

## Motivation
The motivation for doing this is the fundamental principle that true understanding comes from building things from the ground up. 

Here I plan on constructing a software stack for embedded and IoT applications entirely from scratch, using Rust programming language and Renode simulation.

The primary goal is to create a self-contained ecosystem, starting with a bare metal boot loader and extending to include components such as a memory manager, task scheduler, logger, and exception handler. By embarking on this journey, I aim to (re)gain a deep(er) understanding of the inner workings of ARM Cortex M7® based embedded systems  while harnessing the power of Rust's safety and expressiveness.



## Datasheets
To better understand and work with the STM32F746NG microcontroller, will need to refer to the following resources:

1. [STM32F746NG Development Board](https://www.st.com/en/microcontrollers-microprocessors/stm32f746ng.html)
2. [PM0253 STM32F7 Series and STM32H7 Series Cortex®-M7 processor programming manual](https://www.st.com/resource/en/reference_manual/rm0385-stm32f75xxx-and-stm32f74xxx-advanced-armbased-32bit-mcus-stmicroelectronics.pdf)

These documents provide in-depth information about the microcontroller's features, registers, and functionalities.

## Installation
Getting started with this project is straightforward; you only need to install two essential components:

1. Rust: You can install it by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
2. Renode: Renode is an open-source hardware simulation framework that will help you test and debug your code in a controlled environment. You can find installation instructions for Renode on the [Renode GitHub repository](https://github.com/renode/renode)
## Tags