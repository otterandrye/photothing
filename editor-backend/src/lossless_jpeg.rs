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
struct Frame {
	tables: HashMap<u8,HuffmanTable>,
	precision: u8,
	y: u16,
	x: u16,
	components: Vec<Component>,
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
	destination: u8,
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
		while huffsize[k] != 0 {
			while huffsize[k] == size {
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

	let mut tables = HashMap::new();
	let mut scans = Vec::new();
	let mut next_marker = reader.read_u16();
	let mut _restart_interval = 0;
	let mut num_lines = y;
	while next_marker != EOI {
		if next_marker == DHT {
			let table = read_huffman_table(reader);
			tables.insert(table.destination, table);
		} else if next_marker == SOS {
			let scan = read_scan(reader);
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
		tables: tables,
		precision: precision,
		y: num_lines,
		x: x,
		components: components,
		scans: scans,
	}
}

fn read_huffman_table(reader: &mut BufferReader) -> HuffmanTable {
	let length = reader.read_u16();

	HuffmanTable {
		destination: reader.read_u8(),
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
	}	
}

fn read_scan(reader: &mut BufferReader) -> Scan {
	reader.read_u16(); // Eat the length of this section.
	let component_count = reader.read_u8();
	let mut table_mappings = HashMap::new();
	for _i in 0..component_count {
		table_mappings.insert(reader.read_u8(), reader.read_u8() >> 4);
	}

	// So now the bytes here are just... huffman coded components?
	// I think I only know the number of bytes I have post-decode here...
	// So I'll need to do huffman decode on-the-fly?
	// Based on what I've read so far, it seems like I'll end up with..
	//   X * Y + X * Y huffman coded bytes?

	Scan {
		table_mappings: table_mappings,
		predictor: reader.read_u8(),
		point_transform: {
			reader.read_u8(); // Eat "End of spectral selection" byte
			reader.read_u8()
		},
		data: HashMap::new(),
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