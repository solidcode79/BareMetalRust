/* 
View Section Headers: (Check VMA/LMA and sizes)
cargo objdump --bin <name> -- --section-headers 

Locate Symbol Address: 
cargo objdump --bin <name> -- -t | grep '<symbol_name>'.

Verify Vector Table Content: (Useful for checking endianness of SP/PC).
cargo objdump --bin <name> -- -s --section .reset_vector_table 

Disassemble Functions: (Inspect entry point assembly).
cargo objdump --bin <name> -- -d --no-show-raw-insn 

Summarize Memory Usage: (Berkeley output format for section sizes).
cargo size --bin <name> -- -A 

arm-none-eabi-objdump -d target/thumbv7m-none-eabi/debug/rtos-hello > dump.asm
nm ./target/thumbv7m-none-eabi/debug/rtos-hello
*/

MEMORY
{
    /* 256 KB on-chip flash */
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K 
    /* 64 KB SRAM  */
    RAM   (rw): ORIGIN = 0x20000000, LENGTH = 64K 
}

stacks_end = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
    .vectors ORIGIN(FLASH) :
    {
        LONG(ORIGIN(RAM) + LENGTH(RAM)); 
        KEEP(*(.vectors))
    } > FLASH

    .text :
    {
        *(.text*)
        *(.rodata*)
    } > FLASH
    
    .kernel_data : {
        . = ALIGN(32); /* MPU regions often need 32-byte alignment */
        _skernel_data = .;
        *(.kernel_data)
        _ekernel_data = .;
        . = ALIGN(32);
    } > RAM    
    
    .data : {
    . = ALIGN(4);
    _start_of_data = .;
    *(.data*)
    _end_of_data = .;
    . = ALIGN(4);
    } > RAM AT > FLASH
    
    _start_data_in_flash = LOADADDR(.data);
    
    .bss : {
    . = ALIGN(4);
    _start_of_bss = .;
    *(.bss*)
    _end_of_bss = .;
    . = ALIGN(4);
    } > RAM
    
    __msp_stack_size = 2048;
    
    .stack_buffer ORIGIN(RAM) + LENGTH(RAM) - __msp_stack_size - SIZEOF(.stack_buffer) (NOLOAD) :
    {
        . = ALIGN(8);
        *(.stack_buffer)
        . = ALIGN(8);
    } > RAM    
    

}
