// Credits: https://www.youtube.com/watch?v=qSUbe_a54gY

use core::ptr;

pub fn setup()
{
    // Enable USART2 clock

    // Define the RCC_APB1ENR register address
    // For base address of RCC: Table 1. STM32F75xxx and STM32F74xxx register boundary addresses
    // For offset for APB1ENR: Table 21. RCC register map and reset values
	const RCC_APB1ENR: *mut u32 = (0x4002_3800 + 0x40) as *mut u32;

    // For USART2 base address: Table 1. STM32F75xxx and STM32F74xxx register boundary addresses
    // For offsets: Table 201. USART register map and reset values
    const USART2_CR1: *mut u32 = (0x4000_4400 + 0x00)  as *mut u32; // Control Register of USART2
    
	unsafe {
    	*RCC_APB1ENR |= 1 << 17;     // Enable USART2 clock by setting the corresponding bit to high
        *USART2_CR1 = 0x9; // Enable UE & TE bits, 1001, 31.8.1
    }

}

pub fn write(data: &[u8])
{
    const USART2_TDR: *mut u32 = (0x4000_4400 + 0x28) as *mut u32; // Transmit data register of USART2
	
    // Wait until the transmit data register is empty
    // while (unsafe { ptr::read_volatile(usart2::CR1) } & 0x0080) == 0 {}

	for &byte in data 
	{
    	unsafe {
        	ptr::write_volatile(USART2_TDR, byte as u32);
    	}
    
    }
    
}
