use grid::Grid;

pub struct WhiteScale<'a> {
	leveled_white: u16,
	grid: &'a Grid<u16>,
}

impl <'a> WhiteScale<'a> {
	pub fn new(white: u16, black: u16, grid: &'a Grid<u16>) -> WhiteScale {
		WhiteScale {
			leveled_white: white - black,
			grid,
		}
	}
}

impl <'a> Grid<f32> for WhiteScale<'a> {
	fn get(&self, x: u16, y: u16) -> f32 {
		let val = self.grid.get(x, y);
		if val > self.leveled_white {
			1.0
		} else {
			val as f32 / self.leveled_white as f32
		}
	}

	fn width(&self) -> u16 {
		self.grid.width()
	}

	fn height(&self) -> u16 {
		self.grid.height()
	}
}
