pub const SCB_SHPR3: *mut u32 = 0xE000_ED20 as *mut u32;
pub const SCB_SHPR3_PRIORITY_LOWEST : u32 = 0xFF << 16;

pub const SCB_ICSR: *mut u32 = 0xE000_ED04 as *mut u32;
pub const SCB_ICSR_PENDSVSET : u32 =  1 << 28;
