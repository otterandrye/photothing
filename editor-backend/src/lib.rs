#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

pub struct Pixel {
	r: u8,
	g: u8,
	b: u8,
	a: u8,
}

#[wasm_bindgen]
pub struct Preview {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

#[wasm_bindgen]
impl Preview {
    pub fn new(width: u32, height: u32) -> Preview {
        let pixels = (0..width * height)
            .map(|i| {
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

    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }
}