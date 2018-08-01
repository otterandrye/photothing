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

    pub fn read_i32(&self, buffer: &[u8], offset: usize) -> i32 {
        self.read_u32(buffer, offset) as i32
    }
}

pub struct BufferReader<'a> {
	byte_order: ByteOrder,
	buffer: &'a [u8],
	offset: usize,
}

impl <'a> BufferReader<'a> {
	pub fn new(buffer: &'a [u8], byte_order: ByteOrder) -> BufferReader {
		BufferReader {
			byte_order: byte_order,
			buffer: buffer,
			offset: 0,
		}
	}

	pub fn skip_to(&mut self, offset: usize) {
		self.offset = offset;
	}

	pub fn offset(&self) -> usize {
		self.offset
	} 

	pub fn read_u8(&mut self) -> u8 {
		let val = self.buffer[self.offset];
		self.offset += 1;
		val
	}

	pub fn read_u16(&mut self) -> u16 {
		let val = self.byte_order.read_u16(self.buffer, self.offset);
		self.offset += 2;
		val
	}

	pub fn read_u32(&mut self) -> u32 {
		let val = self.byte_order.read_u32(self.buffer, self.offset);
		self.offset += 4;
		val
	}

	pub fn read_i32(&mut self) -> i32 {
		let val = self.byte_order.read_i32(self.buffer, self.offset);
		self.offset += 4;
		val
	}
}