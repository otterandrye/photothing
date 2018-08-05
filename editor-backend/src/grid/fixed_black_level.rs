use grid::Grid;

pub struct FixedBlackLevel<'a> {
	black: u16,
	grid: &'a Grid<u16>,
}

impl <'a> FixedBlackLevel<'a> {
	pub fn new(black: u16, grid: &'a Grid<u16>) -> FixedBlackLevel {
		FixedBlackLevel {
			black: black,
			grid: grid,
		}
	}
}

impl <'a> Grid<u16> for FixedBlackLevel<'a> {
	fn get(&self, x: u16, y: u16) -> u16 {
		let val = self.grid.get(x, y);
		if val < self.black {
			self.black
		} else {
			val - self.black
		}
	}

	fn width(&self) -> u16 {
		self.grid.width()
	}

	fn height(&self) -> u16 {
		self.grid.height()
	}
}
