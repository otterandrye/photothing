#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

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

    pub fn read(&mut self, tiff: &[u8], length: u32) {
    	log(&format!("{:#x?}", tiff[0]));

    	for p in &mut self.pixels {
    		p.red(tiff[0]);
    		p.blue(tiff[0]);
		}

    	log(&format!("{:#x?}", self.pixels[0].r));
    }

    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }
}