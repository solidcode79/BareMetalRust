use crate::hal::unsafe_io;
use crate::boards::board::systick_map;


pub fn systick_init(ticks: u32) {

    unsafe_io::mmio_write_inline(systick_map::SYST_RVR, ticks - 1); 
    unsafe_io::mmio_write_inline(systick_map::SYST_CVR, 0);         

    unsafe_io::mmio_write_inline(
        systick_map::SYST_CSR,
        systick_map::CSR_ENABLE | systick_map::CSR_TICKINT | systick_map::CSR_CLKSOURCE,
    );
}
