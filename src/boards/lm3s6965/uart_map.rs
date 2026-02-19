pub const UART0_DR: *mut u32 = 0x4000_C000 as *mut u32;
pub const UART0_FR: *mut u32 = 0x4000_C018 as *mut u32;
pub const UART_FR_TXFF: u32 = 1 << 5;

