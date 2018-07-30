use std::collections::HashMap;

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
struct Frame {
	precision: u8,
	y: u16,
	x: u16,
	scans: Vec<Scan>,
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
		let mut codelist = Vec::new();
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
		let mut huffsize = Vec::new();
		for i in 0..16 {
			while j <= n_codes_per_size[i]{
				huffsize.push((i as u8) + 1);
				j += 1;
			}
			j = 1;
		}
		huffsize
	}

	fn generate_code_table(&self, huffsize: &Vec<u8>) -> Vec<u16> {
		let mut huffcode = Vec::new();
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

#[derive(Debug)]
struct Scan {
	table_mappings: HashMap<u8, u8>,
	predictor: u8,
	point_transform: u8,
	data: HashMap<u8, Vec<u8>>,
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
			let mut values = Vec::new();
			for _i in 0..length - 19 {
				values.push(reader.read_u8());
			}
			values
		}
	})
}

fn read_scan(reader: &mut BufferReader, tables: &[Option<HuffmanTable>; 4], frame: &FrameHeader) -> Scan {
	reader.read_u16(); // Eat the length of this section.
	let component_count = reader.read_u8();
	let mut table_mappings = HashMap::new();
	for _i in 0..component_count {
		table_mappings.insert(reader.read_u8(), reader.read_u8() >> 4);
	}

	let predictor = reader.read_u8();
	reader.read_u8(); // Eat "End of spectral selection" byte
	let point_transform = reader.read_u8();

	let mut data = HashMap::new();

	for i in 0..component_count {
		let slot = table_mappings.get(&i).unwrap();
		let table = (&tables[(slot % 4) as usize]).as_ref().unwrap();
		let code_list = table.generate_huffman_codes();

		let mut component_data = Vec::new();

		let mut code: u16 = 0;
		let mut code_size: u8 = 0;


		// This condition isn't really right... Need to account for sampling and precision
		while (component_data.len() as u16) < frame.x * frame.y {
			if code_size > 16 {
				// This is a temporary hack.
				break;
			}
			code = code << 1;
			code_size += 1;
			if reader.read_u1() {
				code += 1
			}

			//log(&format!("code: {:#b} {:#?}", code, code_size));


			for hcode in code_list.iter() {
				//log(&format!("hcode: {:#b} {:#?}", hcode.code, hcode.size));

				if hcode.code == code && code_size == hcode.size {
					component_data.push(hcode.symbol);
					code_size = 0;
					code = 0;
				} else if hcode.size > code_size {
					break;
				}
			}
		}

		log(&format!("Found data with {} bytes", component_data.len()));

		data.insert(i, component_data);
	}

	reader.reset_bit_offset();

	// So now the bytes here are just... huffman coded components?
	// I think I only know the number of bytes I have post-decode here...
	// So I'll need to do huffman decode on-the-fly?
	// Based on what I've read so far, it seems like I'll end up with..
	//   X * Y + X * Y huffman coded bytes?

	Scan {
		table_mappings: table_mappings,
		predictor: predictor,
		point_transform: point_transform,
		data: data,
	}
}

pub enum JpegParseError {
	UnsupportedJpeg,
	NotAJpeg,
}

pub fn parse_lossless_jpeg(buffer: &[u8]) -> Result<Image, JpegParseError> {
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
	log(&format!("Found {:#?}", frame));

	Ok(Image {
		x: frame.x,
		y: frame.y,
		precision: frame.precision,
		components: Vec::new(),
	})
}