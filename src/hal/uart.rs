use crate::boards::board::uart_map;
use crate::hal::unsafe_io;
//use core::sync::atomic::{AtomicBool, Ordering};
//static UART_LOCKED: AtomicBool = AtomicBool::new(false);

fn putc(c: u8) {
        while (unsafe_io::mmio_read_inline(uart_map::UART0_FR) & uart_map::UART_FR_TXFF) != 0 {}
        unsafe_io::mmio_write_inline(uart_map::UART0_DR, c as u32);
}

pub fn put_str(s: &str) {
    //if UART_LOCKED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
        for b in s.bytes() {
            putc(b);
        }
    //     UART_LOCKED.store(false, Ordering::Release);
    // }
    
}

pub fn put_hex(val: u32) {
    putc(b'0');
    putc(b'x');
    for i in (0..8).rev() {
        let nibble = (val >> (i * 4)) & 0xF;
        let digit = if nibble < 10 {
            b'0' + nibble as u8
        } else {
            b'A' + (nibble - 10) as u8
        };
        putc(digit);
    }    
}

pub trait Loggable {
    fn log(self);
}

impl Loggable for &str {
    fn log(self) { put_str(self); }
}

impl Loggable for u32 {
    fn log(self) { put_hex(self); }
}

#[macro_export]
macro_rules! info {
    ($($arg:expr),*) => {
        {
            $crate::hal::uart::put_str("[I] ");
            use $crate::hal::uart::Loggable;
            $(
                ($arg).log(); 
            )*
            $crate::hal::uart::put_str("\r\n");
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:expr),*) => {
        {
            $crate::hal::uart::put_str("[E] ");
            use $crate::hal::uart::Loggable;
            $(
                ($arg).log(); 
            )*
            $crate::hal::uart::put_str("\r\n");
        }
    };
}

