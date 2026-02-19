#![no_std]
#![no_main]

mod boards;
mod hal;
mod kernel;

use core::panic::PanicInfo;
use hal::systick;
use hal::scb;

#[no_mangle]
pub unsafe extern "C" fn reset_handler() {
        
    extern "C" {
        static mut _start_of_bss: u32;
        static mut _end_of_bss: u32;
        static mut _start_of_data: u32;
        static mut _end_of_data: u32;
        static _start_data_in_flash: u32;
    }

    // zero init the .bss from _start_of_bss to _end_of_bss (from linker.ld)
    let mut bss = &raw mut _start_of_bss as *mut u32;
    while bss < &raw mut _end_of_bss {
        bss.write_volatile(0);
        bss = bss.add(1);
    }

    // copy the data from _start_data_in_flash (flash) to _start_of_data (RAM) (starting point) 
    let mut src = &_start_data_in_flash as *const u32;
    let mut dest = &raw mut _start_of_data as *mut u32;
    while dest < &raw mut _end_of_data {
        dest.write_volatile(src.read_volatile());
        dest = dest.add(1);
        src = src.add(1);
    }

    crate::main();
}


#[no_mangle]
pub extern "C" fn main() {
    unsafe { 
        core::arch::asm!("cpsid i"); // Disbale interrupts! 
        info!("Reached Main!");
        let stack_store_end = (core::ptr::addr_of!(kernel::task::STACK_STORE.data) as *mut u32)
                .add(kernel::task::TOTAL_STACK_SIZE);
        for i in 0..8 {
            stack_store_end.add(i).write(0xBAAAAAAD);
        }

        info!("MSP Guard [0xBAAAAAAD] placed at: ", stack_store_end as u32 - 32);  
    }
    scb::pendsv_init();   
    systick::systick_init(12800);

    unsafe {
        kernel::task::init();
        kernel::task::start_kernel();
    }
    // Start kernel has a forever loop
}

/* Panic handler */
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    error!("Panic :: RUST trigger panic triggered SVC");
    unsafe {
            core::arch::asm!("svc #1");
    }   
    loop {}
}

// --- APP TASKS For Now! ---
pub fn task1_entry() -> ! {
    info!("T1");
    loop {
        info!("Running T1");
        unsafe { core::arch::asm!("wfi"); }


        // Application Logic
    }
}

pub fn task2_entry() -> ! {
    info!("T2");    
    loop {
        info!(" Running T2");  
        unsafe { core::arch::asm!("wfi"); }
        // panic!();


         // Application Logic
    }
}

pub fn idle_entry() -> ! {
    info!("Idle"); 
    unsafe { 
        // Must be in Thread Mode (IPSR == 0)
        let ipsr: u32;
        core::arch::asm!("mrs {}, ipsr", out(reg) ipsr);
        if ipsr != 0 {
            crate::error!("KERNEL PANIC: idle_task is still in Handler Mode!");
            panic!();
        }

        // Must be using the Process Stack Pointer (PSP)
        let control: u32;
        core::arch::asm!("mrs {}, control", out(reg) control);
        if (control & 0x02) == 0 {
            crate::error!("KERNEL PANIC: idle_task is using MSP instead of PSP!");
            panic!();
        }
        
        // Enable Interrupts finally!! 
        core::arch::asm!("cpsie i"); 
        
        // Drop privilges
        let new_control = control | 0x01;
                core::arch::asm!(
                    "msr control, {}",
                    "isb", // CRITICAL: Instruction Sync Barrier required after CONTROL update
                    in(reg) new_control
                );
                
    }    

    loop {
        info!("Running Idle");
        unsafe { core::arch::asm!("wfi"); }
    }
}