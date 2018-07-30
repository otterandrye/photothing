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
use dng::{IfdEntryTag, IfdEntryValue};
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

    	log(&format!("{:#?}", dng.ifds));

    	for ifd in &dng.ifds {
    		log("NEW IFD");

			match ifd.get(&IfdEntryTag::TileOffsets) {
				Some(IfdEntryValue::Offset(entry_type, count, offset)) => {					
					let first_tile_offset = dng.read_u32(*offset) as usize;
					parse_lossless_jpeg(&tiff[first_tile_offset..]);
				},
				_ => ()
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
