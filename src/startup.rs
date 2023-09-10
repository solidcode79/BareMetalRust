extern "C" {fn main() -> !;}
use core::ptr;
use crate::logger;

// The first function that is called when ARM reset happens
#[no_mangle]
pub extern "C" fn ARMResetHandler() -> ! {
	
    extern "C" {
        static mut linker_start_bss: u8; // Start of .bss section
        static linker_end_bss: u8; // End of .bss section
        static mut linker_start_data: u8; // Start of .data section
        static linker_end_data: u8; // End of .data section
        static linker_load_memory_address_data_section: u8; // Start of .rodata section
    }

    // zero init the .bss from linker_start_bss to linker_end_bss
    unsafe {
        let count = &linker_end_bss as *const u8 as usize - &linker_start_bss as *const u8 as usize;
        ptr::write_bytes(&mut linker_start_bss as *mut u8, 0, count);
    }

    // copy the data from linker_load_memory_address_data_section to linker_start_data (starting point) 
    unsafe {
        let count = &linker_end_data as *const u8 as usize - &linker_start_data as *const u8 as usize;
        ptr::copy_nonoverlapping(&linker_load_memory_address_data_section as *const u8, &mut linker_start_data as *mut u8, count);
    }

    // Tun on SysTick
    unsafe {

        pub const SYST_CSR: *mut u32 = (0xE000E010)  as *mut u32; // SysTick Control Register
        pub const SYST_RVR: *mut u32 = (0xE000E014)  as *mut u32; // SysTick Reload Value Register
        pub const SYST_CVR: *mut u32 = (0xE000E018)  as *mut u32; // SysTick Current Value Register

        ptr::write_volatile(SYST_RVR, 0x00ffffff); 
        ptr::write_volatile(SYST_CVR, 0x0); 
        ptr::write_volatile(SYST_CSR, 0x7); // Enable SysTick
    }

    unsafe { main() }
}

#[no_mangle]
pub unsafe extern "C" fn ARMNmiHandler() -> ! {
    loop {

    }
 }

#[no_mangle]
pub unsafe extern "C" fn ARMHardFaultHandler() -> ! {
    loop {

    }
 }

#[no_mangle]
pub unsafe extern "C" fn ARMMemManageHandler() -> ! {
    loop {

    }
 }

#[no_mangle]
pub unsafe extern "C" fn ARMBusFaultHandler() -> ! {
    loop {

    }
 }

#[no_mangle]
pub unsafe extern "C" fn ARMUsageFaultHandler() -> ! {
    loop {

    }
 }

#[no_mangle]
pub unsafe extern "C" fn ARMSVCallHandler() -> ! {
    loop {

    }
 }

 #[no_mangle]
pub unsafe extern "C" fn ARMDebugMonitorHandler() -> ! {
    loop {

    }
 }

#[no_mangle]
pub unsafe extern "C" fn ARMPendSVHandler() -> ! {
    loop {

    }
 }

#[no_mangle]
pub unsafe extern "C" fn ARMSysTickHandler() -> ! {
    loop {
        logger::write_str("TickTock");
    }
 }

#[no_mangle]
pub unsafe extern "C" fn reserved_dummy() -> ! {
    loop {

    }
 }

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
 pub static ResetFunction: extern "C" fn() -> ! = ARMResetHandler;


// From Table 44. STM32F75xxx and STM32F74xxx vector table of RM0385
#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static RESET_HANDLERS: [unsafe extern "C" fn() -> !; 14] = [
    ARMNmiHandler, //0008
    ARMHardFaultHandler, //000C
    ARMMemManageHandler, //0010
    ARMBusFaultHandler, //0014
    ARMUsageFaultHandler, //0018
    // Don't know enough RUST, this should be pointing to a null but is it safe? 
    reserved_dummy, //001C
    reserved_dummy, //0020
    reserved_dummy, //0024
    reserved_dummy, //0028 
    ARMSVCallHandler, //002C
    ARMDebugMonitorHandler, //0030
    reserved_dummy, //0034
    ARMPendSVHandler, //0038
    ARMSysTickHandler, //003C
];