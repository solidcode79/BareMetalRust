use crate::usart_driver;

pub fn initialize()
{
	usart_driver::setup();
}
pub fn write_str(msg: &str)
{
	let mut buffer = [0u8; 256]; // arbitrary
	let mut index = 0;

	for character in msg.chars() 
	{
		buffer[index] = character as u8;
		index = index + 1;		
	}

	buffer[index] = b'\n';
	usart_driver::write(&buffer);
}

pub fn write_adddress(value: u32)
{
	const HEX_DIGITS: &[u8] = b"0123456789ABCDEF";
	let mut buffer = [0u8; 11]; //0x AA BB CC DD is ten
	let mut index = 0;

	buffer[index] = b' ';
	index = index + 1;
	buffer[index] = b'0';
	index = index + 1;
	buffer[index] = b'x';
	index = index + 1;

	let mut remaining = value;

    for _ in 0..8 {
        let i = (remaining >> 28 & 0xF) as usize;
        buffer[index] = HEX_DIGITS[i];
        remaining <<= 4;
        index = index + 1;
    }

	buffer[index] = b'\n';
	usart_driver::write(&buffer);
	
}
