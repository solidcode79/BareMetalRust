use crate::hal::unsafe_io;
use crate::boards::lm3s6965::scb_map;
use crate::{info, error};

#[no_mangle]
pub unsafe extern "C" fn sysTick_handler() {
    crate::kernel::task::schedule();
    unsafe_io::mmio_write_inline(scb_map::SCB_ICSR, scb_map::SCB_ICSR_PENDSVSET);
    info!("SysTick")
}

#[no_mangle]
pub unsafe extern "C" fn default_handler() {
    error!("default_handler");
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn hard_fault() {
    error!("hard_fault");
    let store_base = core::ptr::addr_of!(crate::kernel::task::STACK_STORE.data) as *const u32;
    for i in 0..crate::kernel::task::TOTAL_STACK_SIZE + crate::kernel::task::GUARD_SIZE {
            let current_addr = store_base.add(i);
          
            error!("Addr: ", current_addr as u32, "  Content: ", *current_addr);
        }    
    error!("Done Printing Full Stack");

    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn bus_fault() {
    error!("bus_fault");
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn usage_fault() {
    error!("usage_fault");
    loop {}
}