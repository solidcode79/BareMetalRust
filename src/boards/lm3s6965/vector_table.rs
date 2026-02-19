#[repr(C)]
pub struct VectorTable {
    pub reset: unsafe extern "C" fn(),
    pub nmi: unsafe extern "C" fn(),
    pub hard_fault: unsafe extern "C" fn(),
    pub mem_manage: unsafe extern "C" fn(),
    pub bus_fault: unsafe extern "C" fn(),
    pub usage_fault: unsafe extern "C" fn(),
    pub reserved_x001c: [unsafe extern "C" fn(); 4],
    pub svcall: unsafe extern "C" fn(),
    pub debug_monitor: unsafe extern "C" fn(),
    pub reserved_x0034: unsafe extern "C" fn(),
    pub pendsv: unsafe extern "C" fn(),
    pub systick: unsafe extern "C" fn(),
}

extern "C" {
    fn sysTick_handler();
    fn pendSV_handler();   
    fn reset_handler();
    fn default_handler();
    fn usage_fault();
    fn bus_fault();
    fn hard_fault();
}



#[link_section = ".vectors"]
#[no_mangle]
pub static VECTOR_TABLE: VectorTable = VectorTable {
    reset: reset_handler,
    nmi: default_handler,
    hard_fault: hard_fault,
    mem_manage: default_handler,
    bus_fault: bus_fault,
    usage_fault: usage_fault,
    reserved_x001c: [default_handler; 4],
    svcall: hard_fault,
    debug_monitor: default_handler,
    reserved_x0034: default_handler,
    pendsv: pendSV_handler,
    systick: sysTick_handler,
};

