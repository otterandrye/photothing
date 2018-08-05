#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	pub fn error(s: String);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn time(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn timeEnd(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);
}