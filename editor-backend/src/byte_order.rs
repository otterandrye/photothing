#[derive(Copy, Clone)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

impl ByteOrder {
    pub fn read_u16(&self, buffer: &[u8], offset: usize) -> u16 {
        match self {
            ByteOrder::LittleEndian => (u16::from(buffer[offset + 1]) << 8) + u16::from(buffer[offset]),
            ByteOrder::BigEndian => (u16::from(buffer[offset]) << 8) + u16::from(buffer[offset + 1]),
        }
    }

    pub fn read_u32(&self, buffer: &[u8], offset: usize) -> u32 {
        match self {
            ByteOrder::LittleEndian => {
                (u32::from(buffer[offset + 3]) << 24) 
                    + (u32::from(buffer[offset + 2]) << 16)
                    + (u32::from(buffer[offset + 1]) << 8)
                    + u32::from(buffer[offset])
            },
            ByteOrder::BigEndian => {
                (u32::from(buffer[offset]) << 24) 
                    + (u32::from(buffer[offset + 1]) << 16) 
                    + (u32::from(buffer[offset + 2]) << 8)
                    + u32::from(buffer[offset + 3])
            },
        }
    }
}

pub struct BufferReader<'a> {
	byte_order: ByteOrder,
	buffer: &'a [u8],
	offset: usize,
	bit_offset: u8,
}

impl<'a> BufferReader<'a> {
	pub fn new(buffer: &[u8], byte_order: ByteOrder) -> BufferReader {
		BufferReader {
			byte_order: byte_order,
			buffer: buffer,
			offset: 0,
			bit_offset: 0,
		}
	}

	pub fn read_u1(&mut self) -> bool {
		let val = self.buffer[self.offset] & (0x01 << self.bit_offset) != 0;
		if self.bit_offset == 7 {
			self.bit_offset = 0;
			self.offset += 1;
		} else {
			self.bit_offset += 1;
		}
		val
	}

	pub fn reset_bit_offset(&mut self) {
		self.bit_offset = 0;
	}

	pub fn read_u8(&mut self) -> u8 {
		if self.bit_offset != 0 {
			panic!("Cannot read u8 from non-zero bit offset");
		}
		let val = self.buffer[self.offset];
		self.offset += 1;
		val
	}
	pub fn read_u16(&mut self) -> u16 {
		if self.bit_offset != 0 {
			panic!("Cannot read u16 from non-zero bit offset");
		}
		let val = self.byte_order.read_u16(self.buffer, self.offset);
		self.offset += 2;
		val
	}
	pub fn read_u32(&mut self) -> u32 {
		if self.bit_offset != 0 {
			panic!("Cannot read u32 from non-zero bit offset");
		}
		let val = self.byte_order.read_u32(self.buffer, self.offset);
		self.offset += 4;
		val
	}
}