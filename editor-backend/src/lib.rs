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
    	// This sets the panic handler to console.error.
    	// It should only be called once, but that's a bit tricky.
    	panic::set_hook(Box::new(hook));

    	let dng = parse_dng(tiff, length).unwrap();

    	log(&format!("{:#?}", dng.ifds));

    	for ifd in &dng.ifds {
    		log("NEW IFD");

			match ifd.get(&IfdEntryTag::TileOffsets) {
				Some(IfdEntryValue::Offset(entry_type, count, offset)) => {					
					//let first_tile_offset = dng.read_u32(*offset) as usize;

					let mut tile_offset = dng.read_u32(*offset) as usize;

					for _i in 0..225 {
						tile_offset = dng.read_u32(*offset) as usize;
					}

					let frame = parse_lossless_jpeg(&tiff[tile_offset..]);

					match frame {
						Ok(frame) => {
							for (component_id, component) in frame.scans[0].components.iter().enumerate() {
								for (idx, val) in component.samples.iter().enumerate() {
									let x: u32 = (idx % 256) as u32;
									let y: u32 = 2*((idx as u32) >> 8) + (component_id as u32);
									self.pixels[(y*self.width + x) as usize] = Pixel {
										r: ((*val as f32) / (<u16>::max_value() as f32) * (<u8>::max_value() as f32)) as u8,
										g: ((*val as f32) / (<u16>::max_value() as f32) * (<u8>::max_value() as f32)) as u8,
										b: ((*val as f32) / (<u16>::max_value() as f32) * (<u8>::max_value() as f32)) as u8,
										a: 255
									};
								}
							}
						}
						_ => (),
					}
				},
				_ => ()
			}
    	}

		//log(&format!("First IFD entry count: {:#?}", parse_dng(tiff, length)));
    }

    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }
}
