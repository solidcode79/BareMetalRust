/* Credits: https://docs.rust-embedded.org/embedonomicon/memory-layout.html 
 * In cargo.toml opt-level = 0 to make sure nothing is optimized out!
 */

/* Address & Size of all section headers
$ cargo objdump --bin bare-metal-rust -- --section-headers
Idx Name                    Size     VMA      LMA      Type
  0                         00000000 00000000 00000000 
  1 .reset_vector_table     00000008 08000000 08000000 DATA
  2 .exception_vector_table 00000038 08000008 08000008 DATA
  3 .text                   000003b4 08000040 08000040 TEXT
  4 .rodata                 00000084 08000400 08000400 DATA
  5 .data                   00000004 20010000 08000484 DATA
  6 .bss                    00000004 20010004 20010004 BSS

*/

/* Address of the reset handler
$ cargo objdump --bin bare-metal-rust -- -t | grep 'ARMResetHandler'
0800009e g     F .text  000000b0 ARMResetHandler
*/



/* Lets verify the full_vector_table:
$ cargo objdump --bin bare-metal-rust -- -s --section .reset_vector_table
 8000000 00000520 9f000008  // 8000000, first four bytes are 00000520 (Stack Pointer) and next four are 41000008 (Program Counter)
 00000520 -> Should be 0x20050000 -> SRAM2(rw): ORIGIN = 0x2004C000, LENGTH = 16K(0x4000) ORIGIN + LENGTH = 0x200500000
          20_05_00_00 == 00_00_05_20 (endianess)
 9f000008 -> Should be address of the reset handler as 0800009e g     F .text 000000b0 ARMResetHandler
           08_00_00_9f == 9f_00_00_08 (endianess) (0800009f) 
           WTF: 0800009f != 0800009e off by one
*/

/* The .text section
cargo objdump --bin bare-metal-rust -- -d --no-show-raw-insn
* First function that is in the flash after the end of full_vector_table is 
* BTW 0x40 is 64 which .reset_vector_table 8 plus .full_vector_table 56 
 0800009e <ARMResetHandler>:
 800009e:       push  {r7, lr}
 80000a0:       mov r7, sp
 80000a2:       sub sp, #0x20

 */


/* The update from Renode, matches the cargo objdump --bin bare-metal-rust -- --section-headers

//  1 .reset_vector_table     00000008 08000000 08000000 DATA
//  2 .exception_vector_table 00000038 08000008 08000008 DATA
16:23:58.3959 [INFO] sysbus: Loading segment of 64 bytes length at 0x8000000.

//   3 .text                   000003b4 08000040 08000040 TEXT
16:23:58.4270 [INFO] sysbus: Loading segment of 948 bytes length at 0x8000040.

//  4 .rodata                 00000084 08000400 08000400 DATA
16:23:58.4282 [INFO] sysbus: Loading segment of 132 bytes length at 0x8000400.

//  5 .data                   00000004 20010000 08000484 DATA
16:23:58.4287 [INFO] sysbus: Loading segment of 4 bytes length at 0x8000484.

//   6 .bss                    00000004 20010004 20010004 BSS
16:23:58.4291 [INFO] sysbus: Loading segment of 4 bytes length at 0x20010004.
16:23:58.6252 [INFO] cpu: Guessing VectorTableOffset value to be 0x8000000.

// SP: SRAM2(rw): ORIGIN = 0x2004C000, LENGTH = 16K(0x4000) ORIGIN + LENGTH = 0x200500000
// PC: 0800009e g     F .text  000000b0 ARMResetHandler
16:23:58.6400 [INFO] cpu: Setting initial values: PC = 0x800009F, SP = 0x20050000.
16:23:58.6470 [INFO] machine-0: Machine started.

// 0800009e g     F .text  000000b0 ARMResetHandler
16:23:58.7800 [INFO] cpu: Entering function ARMResetHandler (entry) at 0x800009E 

*/


/* 
 $ cargo size --bin bare-metal-rust  -- -A 
 section                       size        addr
.reset_vector_table              8   0x8000000
.exception_vector_table         56   0x8000008
.text                          948   0x8000040
.rodata                        132   0x8000400
.data                            4  0x20010000
.bss                             4  0x20010004
*/

MEMORY
{
  FLASH(rx): ORIGIN = 0x08000000, LENGTH = 1024K
  SRAM1(rw): ORIGIN = 0x20010000, LENGTH = 128K /* Use for .data .bss and heap */ 
  SRAM2(rw): ORIGIN = 0x2004C000, LENGTH = 16K  /* Use for stack fully , 0x4000 is the LENGTH */
}

ENTRY(ResetFunction);

SECTIONS
{
  .reset_vector_table ORIGIN(FLASH) :
  {
    /* First entry: initial Stack Pointer value */
    LONG(ORIGIN(SRAM2) + LENGTH(SRAM2)); 

    /* Second entry: reset vector */
    KEEP(*(.vector_table.reset_vector));
  }


  .exception_vector_table :
  {
    /* the next 14 entries are exception vectors */
    KEEP(*(.vector_table.exceptions));
  } > FLASH

  .text :
  {
    *(.text .text.*);
  } > FLASH
  
  .rodata :
  {
    *(.rodata .rodata.*);
  }

  .data :
  {
    linker_start_data = .;
    *(.data .data.*);
    . = ALIGN(4);
    linker_end_data = .;
  } > SRAM1 AT > FLASH
 

  linker_load_memory_address_data_section = LOADADDR(.data); 
  /*
   linker_load_memory_address_data_section =-> load from this address; address of flash
   linker_start_data -> load to this address; address of RAM
   linker_end_data -> load till this address; address of RAM
  */
   
  .bss :
  {
      /* .bss section for uninitialized data */
	  linker_start_bss = .;
      KEEP(*(.bss .bss.*));
	  . = ALIGN(4);
	  linker_end_bss = .;
  } > SRAM1
  
  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}