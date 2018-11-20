#![feature(nll)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
mod dng;
use dng::parse_dng;
use std::panic;
mod console;
mod byte_order;
mod lossless_jpeg;
mod grid;

pub use console::*;
use dng::{IfdEntryTag, IfdEntryValue};
use lossless_jpeg::parse_lossless_jpeg;
use grid::Grid;
use grid::tile::Tile;
use grid::fixed_black_level::FixedBlackLevel;
use grid::fixed_white_level::FixedWhiteLevel;
use grid::pixelate::Pixelate;
use grid::composite_tile_grid::CompositeTileGrid;
use grid::crop::Crop;

use grid::white_balance::WhiteBalance;
use grid::white_scale::WhiteScale;
use grid::displayable::Displayable;
use grid::dual_component_tile::DualComponentTile;

#[derive(Debug)]
pub struct CanvasPixel {
	r: u8,
	g: u8,
	b: u8,
	a: u8,
}

#[wasm_bindgen]
pub struct Preview {
    width: u32,
    height: u32,
    pixels: Vec<CanvasPixel>,
}

fn hook(info: &panic::PanicInfo) {
    error(info.to_string());
}

#[wasm_bindgen]
impl Preview {
    pub fn new(width: u32, height: u32) -> Preview {
    	time("Blue render");
        let pixels = (0..width * height)
            .map(|_i| {
                CanvasPixel {
                	r: 100,
                	g: 100,
                	b: 255,
                	a: 255,
                }
            })
            .collect();
        timeEnd("Blue render");
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

    	time("DNG Parse");
    	let dng = parse_dng(tiff, length).unwrap();
    	log(&format!("{:#?}", dng.ifds));
    	timeEnd("DNG Parse");

    	for ifd in &dng.ifds {
			match ifd.get(&IfdEntryTag::TileOffsets) {
				Some(IfdEntryValue::Offset(_entry_type, count, offset)) => {
					if *count != 425 {
						// There are other IFDs here I don't care about.
						// In the future, I should check the Subtype.
						return;
					}

					// Some hard coded values I should probably read from the DNG:
					// The DNG file says 14558, sensor values max ~16384
					let white_level = 14558;
					// The DNG file says 2048, sensor values min ~1980
					let black_level = 2048;
					let image_width = 6384;
					let image_height = 4224;
					let tile_count = 425;
					let tile_width = 256;
					let tile_height = 256;

					time("Data Parse");
					let mut tiles = Vec::with_capacity(tile_count);
					for i in 0..tile_count {
						let tile_offset = dng.read_u32(*offset + 4 * i) as usize;

						time("Tile Parse");
						let mut frame = parse_lossless_jpeg(&tiff[tile_offset..]);
						timeEnd("Tile Parse");
						match frame {
							Ok(mut frame) => {
								let tile = DualComponentTile::new(tile_width, tile_height, frame);

								tiles.push(tile);
							},
							Err(msg) => {
								log(&format!("{:?}", msg));
							},
						}
					}
					timeEnd("Data Parse");

					let composite = CompositeTileGrid::new(image_width, image_height, tile_width, tile_height, tiles);
					let cropped = Crop::new(120, 44, composite.width(), composite.height(), &composite);
					let blackleveled = FixedBlackLevel::new(black_level, &cropped);
					//let whiteleveled = FixedWhiteLevel::new(white_level - black_level, &blackleveled);
					// So far I'm not scaling as a separate step
					//let scaled = SampleScale::new(&whiteleveled);
					// Pixelate also handles the CFA for now

					let whitescaled = WhiteScale::new(white_level, black_level, &blackleveled);
					// This varries by photo, but for now picking:
					// AsShotNeutral: RationalTriple(
            		// 	440430,
            		// 	1000000,
            		// 	1000000,
            		// 	1000000,
            		// 	620230,
            		// 	1000000
        			// ),
					let whitebalanced = WhiteBalance::new(440430.0 / 1000000.0, 1.0, 620230.0 / 1000000.0, &whitescaled);
					let displayable = Displayable::new(&whitebalanced);
					let pixelated = Pixelate::new(&displayable);

					time("Render");
					for y in 0..pixelated.height() {
						for x in 0..pixelated.width() {
							let internal_pixel = pixelated.get(x, y);
							let px_offset = (y as u32 * self.width + x as u32) as usize;
							self.pixels[px_offset] = CanvasPixel {
								r: internal_pixel.red,
								g: internal_pixel.green,
								b: internal_pixel.blue,
								a: 255,
							}
						}
					}
					timeEnd("Render");
				},
				_ => ()
			}
    	}
    }

    pub fn pixels(&self) -> *const CanvasPixel {
        self.pixels.as_ptr()
    }
}
