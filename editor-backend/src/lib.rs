#![feature(use_extern_macros)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
mod dng;
use dng::parse_dng;
use std::panic;
mod console;
mod byte_order;
mod lossless_jpeg;

pub use console::*;
use dng::IfdEntryTag;
use lossless_jpeg::parse_lossless_jpeg;

pub struct Pixel {
	r: u8,
	g: u8,
	b: u8,
	a: u8,
}

impl Pixel {
    fn red(&mut self, val: u8) {
        self.r = val;
    }
    fn green(&mut self, val: u8) {
    	self.g = val;
    }
    fn blue(&mut self, val: u8) {
    	self.b = val;
    }
}

#[wasm_bindgen]
pub struct Preview {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

fn hook(info: &panic::PanicInfo) {
    error(info.to_string());
}

#[wasm_bindgen]
impl Preview {
    pub fn new(width: u32, height: u32) -> Preview {
        let pixels = (0..width * height)
            .map(|_i| {
                Pixel {
                	r: 100,
                	g: 100,
                	b: 255,
                	a: 255,
                }
            })
            .collect();

        Preview {
            width,
            height,
            pixels,
        }
    }

    pub fn read(&mut self, tiff: &[u8], length: u32) {
    	panic::set_hook(Box::new(hook));

    	let dng = parse_dng(tiff, length).unwrap();
    	//log(&format!("Data length: {}", dngData.len()));
    	//log(&format!("Px length: {}", self.pixels.len()));

    	log(&format!("{:#?}", dng.ifds));

    	for ifd in &dng.ifds {
    		log("NEW IFD");

    		match ifd.get(&IfdEntryTag::NewSubfileType) {
				Some(entry) => {
					log(&format!("Type: {:#?}", dng.read_u32(entry.offset)));
				},
				None => ()
			}

			match ifd.get(&IfdEntryTag::ImageWidth) {
				Some(entry) => {
					log(&format!("Width: {:#?}", dng.read_u32(entry.offset)));
				},
				None => ()
			}

			match ifd.get(&IfdEntryTag::ImageLength) {
				Some(entry) => {
					log(&format!("Length: {:#?}", dng.read_u32(entry.offset)));
				},
				None => ()
			}

			match ifd.get(&IfdEntryTag::Compression) {
				Some(entry) => {
					log(&format!("Compression: {:#?}", dng.read_u16(entry.offset)));
				},
				None => ()
			}

			match ifd.get(&IfdEntryTag::PhotometricInterpretation) {
				Some(entry) => {
					log(&format!("PhotometricInterpretation: {:#?}", dng.read_u16(entry.offset)));
				},
				None => ()
			}

			match ifd.get(&IfdEntryTag::BitsPerSample) {
				Some(entry) => {
					log(&format!("BitsPerSample first: {:#?} type: {:#?} count: {:#?}", dng.read_u16(entry.offset), entry.entry_type, entry.count));
				},
				None => ()
			}

			match ifd.get(&IfdEntryTag::TileLength) {
				Some(entry) => {
					log(&format!("TileLength: {:#?}", dng.read_u32(entry.offset)));
				},
				None => ()
			}

			match ifd.get(&IfdEntryTag::TileWidth) {
				Some(entry) => {
					log(&format!("TileWidth: {:#?}", dng.read_u32(entry.offset)));
				},
				None => ()
			}

			match ifd.get(&IfdEntryTag::PlanarConfiguration) {
				Some(entry) => {
					log(&format!("PlanarConfiguration: {:#?}", dng.read_u16(entry.offset)));
				},
				None => ()
			}

			match ifd.get(&IfdEntryTag::TileByteCounts) {
				Some(entry) => {
					log(&format!("TileByteCounts: first: {:#?}", dng.read_u32(entry.offset)));
				},
				None => ()
			}

			match ifd.get(&IfdEntryTag::TileOffsets) {
				Some(entry) => {
					log(&format!("TileOffsets type: {:#?} count: {:#?}", entry.entry_type, entry.count));
					
					let first_tile_offset = dng.read_u32(entry.offset) as usize;

					log(&format!("JPEG1 (SOI): {:#x?}", dng.read_u16_be(first_tile_offset)));
					let encoding_type = dng.read_u16_be(first_tile_offset + 2);
					if (encoding_type != 0xffc3) {
						return;
					}

					parse_lossless_jpeg(&tiff[first_tile_offset..]);

					log(&format!("JPEG2 (SOF_3): {:#x?}", encoding_type));
					log(&format!("JPEG3 (L_f: {:#?}", dng.read_u16_be(first_tile_offset + 4)));
					log(&format!("JPEG4 (P): {:#?}", dng.read_u8(first_tile_offset + 6)));
					log(&format!("JPEG5 (Y): {:#?}", dng.read_u16_be(first_tile_offset + 7)));
					log(&format!("JPEG6 (X): {:#?}", dng.read_u16_be(first_tile_offset + 9)));
					log(&format!("JPEG7 (N_f): {:#?}", dng.read_u8(first_tile_offset + 11)));
					log(&format!("JPEG8 (C_i): {:#?}", dng.read_u8(first_tile_offset + 12)));
					let sampling_factors = dng.read_u8(first_tile_offset + 13);
					log(&format!("JPEG9 (H_i, V_i): {:#?}, {:#?}", sampling_factors >> 4, sampling_factors & 0x0F));
					log(&format!("JPEG10 (T_qi): {:#?}", dng.read_u8(first_tile_offset + 14)));

					log(&format!("JPEG11 (C_i2): {:#?}", dng.read_u8(first_tile_offset + 15)));
					let sampling_factors = dng.read_u8(first_tile_offset + 16);
					log(&format!("JPEG12 (H_i2, V_i2): {:#?}, {:#?}", sampling_factors >> 4, sampling_factors & 0x0F));
					log(&format!("JPEG13 (T_qi2): {:#?}", dng.read_u8(first_tile_offset + 17)));
					log(&format!("Next Marker: {:#x?}", dng.read_u16_be(first_tile_offset + 18)));
					let L_h = dng.read_u16_be(first_tile_offset + 20) as usize;
					log(&format!("DHT (L_h): {:#?}", L_h));

					let dht_info = dng.read_u8(first_tile_offset + 22);
					log(&format!("DHT (T_c, T_h): {:#?}, {:#?}", dht_info >> 4, dht_info & 0x0F));
					// Skip the DHT Tc/Th byte
					for i in 1..17 {
						log(&format!("DHT (L_{:}):  {:#?}", i, dng.read_u8(first_tile_offset + 22 + i)));
					}

					for i in 0..(L_h - 19) {
						log(&format!("DHT (V_{:}):  {:#?}", i, dng.read_u8(first_tile_offset + 39 + i)));
					}

					log(&format!("Next Marker: {:#x?}", dng.read_u16_be(first_tile_offset + 20 + L_h)));
					let L_h2 = dng.read_u16_be(first_tile_offset + 22 + L_h) as usize;
					log(&format!("DHT2 (L_h): {:#?}", L_h2));

					let dht_info2 = dng.read_u8(first_tile_offset + 24 + L_h);
					log(&format!("DHT2 (T_c, T_h): {:#?}, {:#?}", dht_info2 >> 4, dht_info2 & 0x0F));

				},
				None => ()
			}
    	}



    	for _p in &mut self.pixels {
    		//p.red(dngData[px*3]);
    		//p.green(dngData[px*3 + 1]);
    		//p.blue(dngData[px*3 + 2]);
		}

		//log(&format!("First IFD entry count: {:#?}", parse_dng(tiff, length)));
    }

    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }
}
