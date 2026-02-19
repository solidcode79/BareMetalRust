use core::ptr;

#[inline(always)]
pub fn mmio_read_inline(addr: *const u32) -> u32 {
    unsafe { ptr::read_volatile(addr) }
}

#[inline(always)]
pub fn mmio_write_inline(addr: *mut u32, value: u32) {
    unsafe { ptr::write_volatile(addr, value) }
}
