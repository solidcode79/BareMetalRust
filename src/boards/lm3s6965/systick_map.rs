pub const SYST_CSR: *mut u32 = 0xE000E010 as *mut u32;
pub const SYST_RVR:  *mut u32 = 0xE000E014 as *mut u32;
pub const SYST_CVR:  *mut u32 = 0xE000E018 as *mut u32;

pub const CSR_ENABLE: u32    = 1 << 0;
pub const CSR_TICKINT: u32   = 1 << 1;
pub const CSR_CLKSOURCE: u32 = 1 << 2;
