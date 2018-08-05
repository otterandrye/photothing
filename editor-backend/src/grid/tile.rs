use grid::Grid;

pub struct Tile {
	width: u16,
	height: u16,
	data: Vec<u16>,
}

impl Tile {
	pub fn new(width: u16, height: u16, data: Vec<u16>) -> Tile {
		Tile {
			width: width,
			height: height,
			data: data,
		}
	}
}

impl Grid<u16> for Tile {
	fn get(&self, x: u16, y: u16) -> u16 {
		self.data[(y * self.width + x) as usize]
	}

	fn width(&self) -> u16 {
		self.width
	}

	fn height(&self) -> u16 {
		self.height
	}
}
