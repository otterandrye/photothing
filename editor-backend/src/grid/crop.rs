use grid::Grid;

pub struct Crop<'a> {
	top_x: u16,
	top_y: u16,
	bottom_x: u16,
	bottom_y: u16,
	grid: &'a Grid<u16>,
}

impl <'a> Crop<'a> {
	pub fn new(top_x: u16, top_y: u16, bottom_x: u16, bottom_y: u16, grid: &'a Grid<u16>) -> Crop {
		Crop {
			top_x,
			top_y,
			bottom_x,
			bottom_y,
			grid,
		}
	}
}

impl <'a> Grid<u16> for Crop<'a> {
	fn get(&self, x: u16, y: u16) -> u16 {
		self.grid.get(x + self.top_x, y + self.top_y)
	}

	fn width(&self) -> u16 {
		self.bottom_x - self.top_x
	}

	fn height(&self) -> u16 {
		self.bottom_y - self.top_y
	}
}
