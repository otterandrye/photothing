pub struct Pixel {
    red: u32,
    green: u32,
    blue: u32,
    alpha: u32,
}

pub struct Image {
    pixels: [Pixel],
}

enum ByteOrder {
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

pub fn parse_dng(buffer: &[u8], length: u32) -> u32 {
    let byte_order: ByteOrder = if buffer[0] == 0x49 && buffer[1] == 0x49 { ByteOrder::LittleEndian }
        else if buffer[0] == 0x4D && buffer[1] == 0x4D { ByteOrder::BigEndian }
        else { panic!("File has invalid Byte Order") };

    let mut position: usize = 2;
    let confirm_dng = byte_order.read_u16(buffer, position);

    if confirm_dng != 42 {
        panic!("File is not a DNG {}", confirm_dng)
    }
    position += 2;

    // Get the position of the first IFD
    position = byte_order.read_u32(buffer, position) as usize;

    // Get the number of IFD entries...
    u32::from(byte_order.read_u16(buffer, position))
}