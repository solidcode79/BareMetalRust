#![no_main]
#![no_std] // This already includes core lib https://doc.rust-lang.org/core/index.html even if no std!

use core::panic::PanicInfo;
mod startup;

pub static mut YOUR_RW_VARIABLE: u32 =  0x0000002A; // Should go into .data section
pub static mut YOUR_BSS_VARIABLE: u32 = 0; // Should go in the .bss section
pub const YOUR_RO_VARIABLE: &str  = "TestSomeROData";

#[no_mangle]
pub extern "C" fn main() -> ! {

    extern "C" {
        static mut linker_start_bss: u8; // Start of .bss section
        static linker_end_bss: u8; // End of .bss section
        static mut linker_start_data: u8; // Start of .data section
        static linker_end_data: u8; // End of .data section
        static linker_load_memory_address_data_section: u8; // Start of .rodata section
    }
    
    unsafe {        
        let _i = &YOUR_RW_VARIABLE;
        let _j = &YOUR_BSS_VARIABLE;
        let _i = &linker_start_bss;
        let _j = &linker_end_bss;
        let _k = &linker_start_data;
        let _l = &linker_end_data;
        let _m = &linker_load_memory_address_data_section;
        let _n = &YOUR_RO_VARIABLE;
    }

	loop {
	}	
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}