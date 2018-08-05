use grid::Grid;

use grid::tile::Tile;

pub struct CompositeTileGrid {
	tile_count_x: u16,
	image_width: u16,
	image_height: u16,
	tile_width: u16,
	tile_height: u16,
	tiles: Vec<Tile>,
}

impl CompositeTileGrid {
	pub fn new(image_width: u16, image_height: u16, tile_width: u16, tile_height: u16, tiles: Vec<Tile>) -> CompositeTileGrid {
		let mut tile_count_x = image_width / tile_width;
		if image_width % tile_width != 0 {
			tile_count_x += 1
		}

		CompositeTileGrid {
			tile_count_x: tile_count_x,
			image_width: image_width,
			image_height: image_height,
			tile_width: tile_width,
			tile_height: tile_height,
			tiles: tiles,
		}
	}
}

impl Grid<u16> for CompositeTileGrid {
	fn get(&self, x: u16, y: u16) -> u16 {
		let tile_x = x / self.tile_width;
		let in_tile_x = x % self.tile_width;
		let tile_y = y / self.tile_height;
		let in_tile_y = y % self.tile_height;
		self.tiles[(tile_y * self.tile_count_x + tile_x) as usize].get(in_tile_x, in_tile_y)
	}

	fn width(&self) -> u16 {
		self.image_width
	}

	fn height(&self) -> u16 {
		self.image_height
	}
}
