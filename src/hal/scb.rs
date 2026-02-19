use crate::hal::unsafe_io;
use crate::boards::board::scb_map;


pub fn pendsv_init() {

    let val = unsafe_io::mmio_read_inline(scb_map::SCB_SHPR3); 
    unsafe_io::mmio_write_inline(scb_map::SCB_SHPR3, val | scb_map::SCB_SHPR3_PRIORITY_LOWEST);

}
