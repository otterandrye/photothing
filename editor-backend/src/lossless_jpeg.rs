use ::console::log;
use ::byte_order::{ByteOrder::BigEndian, BufferReader};

const SOI: u16 = 0xFFD8;
const SOF_3: u16 = 0xFFC3;
const DHT: u16 = 0xFFC4;
const EOI: u16 = 0xFFD9;
const SOS: u16 = 0xFFDA;
const RST: u16 = 0xFFDD;
const COM: u16 = 0xFFFE;
const DNL: u16 = 0xFFDC;
const APP1: u16 = 0xFFE0;
const APP16: u16 = 0xFFEF;

#[derive(Debug)]
struct FrameHeader {
	y: u16, 
	x: u16, 
	components: Vec<Component>,
}

#[derive(Debug)]
pub struct Frame {
	precision: u8,
	pub y: u16,
	pub x: u16,
	pub scans: Vec<Scan>,
}

#[derive(Debug)]
struct Component {
	id: u8,
	horizonal_sampling_factor: u8,
	vertical_sampling_factor: u8,
	q_table: u8,
}

#[derive(Debug)]
struct HuffmanTable {
	number_of_codes: [u8; 16],
	values: Vec<u8>,
}

impl HuffmanTable {
	fn generate_huffman_codes(&self) -> Vec<HuffmanCode> {
		let huffsize = self.generate_size_table(&self.number_of_codes);
		let huffcode = self.generate_code_table(&huffsize);
		let mut codelist = Vec::with_capacity(16);
		for i in 0..huffcode.len() {
			codelist.push(HuffmanCode{
				code: huffcode[i],
				size: huffsize[i],
				symbol: self.values[i]
			});
		}
		codelist
	}

	fn generate_size_table(&self, n_codes_per_size: &[u8]) -> Vec<u8> {
		let mut j = 1;
		let mut huffsize = Vec::with_capacity(16);
		for i in 0..16 {
			while j <= n_codes_per_size[i] {
				huffsize.push((i as u8) + 1);
				j += 1;
			}
			j = 1;
		}
		huffsize
	}

	fn generate_code_table(&self, huffsize: &Vec<u8>) -> Vec<u16> {
		let mut huffcode = Vec::with_capacity(16);
		let mut k = 0;
		let mut code = 0;
		let mut size = huffsize[0];

		while k < huffsize.len() && huffsize[k] != 0 {
			while k < huffsize.len() && huffsize[k] == size {
				huffcode.push(code);
				code += 1;
				k += 1;
			}
			code = code << 1;
			size += 1;
		}

		huffcode
	}
}

#[derive(Debug)]
struct HuffmanCode {
	code: u16,
	size: u8,
	symbol: u8,
}

// Computes the ith triangular number
fn triangular(ith: i32) -> i32 {
	if ith > 0 {
		ith * (ith + 1) / 2
	} else {
		0
	}
}

// Given a width and height of the image, plus x and y coordinates,
// find the index in JPEG buffer. This takes on the shape of a zig-zag
// that ensures that the above and left neighbors of any given pixel
// preceed that given pixel in the ordering.
// This was a pain in the ass.
fn index(width: u16, height: u16, x: u16, y: u16) -> usize {
	let z: i32 = x as i32 + y as i32;
	let right_offset = z - ((width as i32) - 1);
	let left_offset = z - ((height as i32) - 1);
	(triangular(z) + if z % 2 == 1 {
		(y as i32) - triangular(right_offset) - triangular(left_offset - 1)
	} else {
		(x as i32) - triangular(right_offset - 1) - triangular(left_offset)
	}) as usize
}

#[derive(Debug)]
pub struct Scan {
	pub components: Vec<ComponentInfo>,
}

#[derive(Debug)]
pub struct Image {
	x: u16,
	y: u16,
	precision: u8,
	components: Vec<ComponentData>,
}

#[derive(Debug)]
pub struct ComponentData {
	data: Vec<u16>,
	x_sample: u8,
	y_sample: u8,
}

fn read_frame(reader: &mut BufferReader) -> Frame {
	// First, we can just read the length. We don't need it though.
	reader.read_u16();
	let precision = reader.read_u8();
	let y = reader.read_u16();
	let x = reader.read_u16();
	let component_count = reader.read_u8();

	let mut components = Vec::new();
	for _i in 0..component_count {
		let id = reader.read_u8();
		let sampling_factors = reader.read_u8();
		let q_table = reader.read_u8();

		components.push(Component {
			id: id,
			horizonal_sampling_factor: sampling_factors >> 4,
			vertical_sampling_factor: sampling_factors & 0x0F,
			q_table: q_table,
		});
	}

	let frame_header = FrameHeader {
		y: y,
		x: x,
		components: components,
	};

	let mut tables: [Option<HuffmanTable>; 4] = [None, None, None, None];
	let mut scans = Vec::new();
	let mut next_marker = reader.read_u16();
	let mut _restart_interval = 0;
	let mut num_lines = y;
	while next_marker != EOI {
		if next_marker == DHT {
			let (slot, table) = read_huffman_table(reader);
			tables[(slot % 4) as usize] = Some(table);
		} else if next_marker == SOS {
			let scan = read_scan(reader, &tables, &frame_header);
			scans.push(scan);
		} else if next_marker == RST {
			reader.read_u16(); // Length. Always 4.
			_restart_interval = reader.read_u16();
		} else if next_marker == COM || (next_marker >= APP1 && next_marker <= APP16) {
			// These sections are comments and proprietary app data. Skip them.
			let length = reader.read_u16();
			for _i in 0..length - 1 {
				reader.read_u8();
			}
		} else if next_marker == DNL {
			reader.read_u16(); // Length. Always 4.
			num_lines = reader.read_u16();
		} else {
			if (next_marker >> 8) == 0xFF && next_marker != 0xFFFF && next_marker != 0xFF00 {
				log(&format!("Unknown marker: {:#x?}", next_marker));
			}
		}
		next_marker = reader.read_u16();
	}

	Frame {
		precision: precision,
		y: num_lines,
		x: x,
		scans: scans,
	}
}

fn read_huffman_table(reader: &mut BufferReader) -> (u8, HuffmanTable) {
	let length = reader.read_u16();

	(reader.read_u8(), HuffmanTable {
		number_of_codes: [
			reader.read_u8(), reader.read_u8(), reader.read_u8(), reader.read_u8(),
			reader.read_u8(), reader.read_u8(), reader.read_u8(), reader.read_u8(),
			reader.read_u8(), reader.read_u8(), reader.read_u8(), reader.read_u8(),
			reader.read_u8(), reader.read_u8(), reader.read_u8(), reader.read_u8(),
		],
		values: {
			let mut values = Vec::with_capacity(16);
			for _i in 0..length - 19 {
				values.push(reader.read_u8());
			}
			values
		}
	})
}

#[derive(Debug)]
pub struct ComponentInfo {
	id: u8,
	codes: Vec<HuffmanCode>,
	pub samples: Vec<u16>,
}

fn coord_index(width: u16, x: u16, y: u16) -> usize {
	((width * y) + x) as usize
}

fn predict_data_unit(data: &Vec<u16>, width: u16, x: u16, y: u16, predictor: u8) -> u16 {
	if x == 0 && y == 0 {
		return 1 << (16 - 1); // Technically, this should be the precision, but I'm not wiring that.
	} else if y == 0 {
		return data[coord_index(width, x - 1, y)];
	} else if x == 0 {
		return data[coord_index(width, x, y - 1)];
	}

	let a = data[coord_index(width, x - 1, y)];
	let b = data[coord_index(width, x, y - 1)];
	let c = data[coord_index(width, x - 1, y - 1)];

	match predictor {
		1 => a,
		2 => b,
		3 => c,
		4 => a + b - c,
		5 => a + ((b - c) >> 1),
		6 => b + ((a - c) >> 1),
		7 => (a + b) >> 1,
		_ => panic!("Unsupported"),
	}
} 

fn read_data_unit(reader: &mut JpegBitReader, component: &mut ComponentInfo, width: u16, x: u16, y: u16, predictor: u8, point_transform: u8) {
	let count = decode(&component.codes, reader);
	let diff = extend(receive(reader, count), count);
	let prediction = predict_data_unit(&component.samples, width, x, y, predictor);
	component.samples.push(((i32::from(diff) + i32::from(prediction)) << point_transform) as u16);
}

fn decode(codes: &Vec<HuffmanCode>, reader: &mut JpegBitReader) -> u8 {
	let mut code = 0;
	let mut size = 0;
	loop {
		code = code << 1;
		if reader.read_u1().unwrap() {
			code += 1
		}
		size += 1;

		for hcode in codes.iter() {
			if hcode.code == code && size == hcode.size {
				return hcode.symbol
			} else if hcode.size > size {
				break;
			}
		}
	}
}

fn receive(reader: &mut JpegBitReader, count: u8) -> u16 {
	let mut val: u16 = 0;
	for _i in 0..count {
		val = val << 1;
		if reader.read_u1().unwrap() {
			val += 1
		}
	}
	val
}

fn extend(number: u16, size: u8) -> i16 {
	if size > 0 && number < (1 << (i16::from(size) - 1)) {
		let negative_one: i16 = -1;
		let (shifted, _) = negative_one.overflowing_shl(u32::from(size));
		return (number as i16) + shifted + 1
	} 
	number as i16
}

fn read_scan(reader: &mut BufferReader, tables: &[Option<HuffmanTable>; 4], frame: &FrameHeader) -> Scan {
	reader.read_u16(); // Eat the length of this section.
	let component_count = reader.read_u8();

	let mut components = Vec::with_capacity(component_count as usize);


	for _i in 0..component_count {
		let component_id = reader.read_u8();
		let slot = reader.read_u8() >> 4;
		components.push(ComponentInfo {
			id: component_id,
			codes: (&tables[(slot % 4) as usize]).as_ref().unwrap().generate_huffman_codes(),
			samples: Vec::with_capacity((frame.x * frame.y) as usize),
		});
	}

	let predictor = reader.read_u8();
	reader.read_u8(); // Eat "End of spectral selection" byte
	let point_transform = reader.read_u8();

	let mut bitreader = JpegBitReader::new(reader);

	let mut x = 0;
	let mut y = 0;
	while (x < frame.x) && (y < frame.y) {
		// Read an MCU!
		//log(&format!("Reading ({}, {})", x, y));
		for mut component in components.iter_mut() {
			read_data_unit(&mut bitreader, &mut component, frame.x, x, y, predictor, point_transform);
		}
		x += 1;
		if x == frame.x {
			x = 0;
			y += 1;
		}
	}

	Scan {
		components: components,
	}
}

#[derive(Debug)]
enum JpegBitReaderError {
	DNLMarker,
	IllegalMarker,
}

struct JpegBitReader<'i, 'u: 'i> {
	byte_reader: &'i mut BufferReader<'u>,
	byte_buffer: u8,
	bit_offset: u8,
}

impl <'i, 'u> JpegBitReader<'i, 'u> {
	fn new(byte_reader: &'i mut BufferReader<'u>) -> JpegBitReader<'i, 'u> {
		JpegBitReader {
			byte_reader: byte_reader,
			bit_offset: 0,
			byte_buffer: 0,
		}
	}

	fn read_u1(&mut self) -> Result<bool, JpegBitReaderError> {
		if self.bit_offset == 0 {
			self.byte_buffer = self.byte_reader.read_u8();
			self.bit_offset = 8;
			if self.byte_buffer == 0xFF {
				let marker = self.byte_reader.read_u8();
				if marker != 0 {
					if marker == (DNL & 0x00FF) as u8 {
						return Err(JpegBitReaderError::DNLMarker);
					} else {
						return Err(JpegBitReaderError::IllegalMarker);
					}
				}
			}
		}
		let bit = self.byte_buffer >> 7;
		self.bit_offset -= 1;
		self.byte_buffer = self.byte_buffer << 1;
		Ok(bit == 1)
	}
}

#[derive(Debug)]
pub enum JpegParseError {
	UnsupportedJpeg,
	NotAJpeg,
}

pub fn parse_lossless_jpeg(buffer: &[u8]) -> Result<Frame, JpegParseError> {
	let mut reader = BufferReader::new(buffer, BigEndian);
	let soi = reader.read_u16();
	if soi != SOI {
		return Err(JpegParseError::NotAJpeg);
	}

	let sof = reader.read_u16();
	if sof != SOF_3 {
		return Err(JpegParseError::UnsupportedJpeg);
	}

	let frame = read_frame(&mut reader);
	return Ok(frame);
	/*Ok(Image {
		x: frame.x,
		y: frame.y,
		precision: frame.precision,
		components: frame.scan[0].components,
	})*/
}