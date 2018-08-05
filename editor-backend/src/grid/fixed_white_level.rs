use grid::Grid;

pub struct FixedWhiteLevel<'a> {
	white: u16,
	grid: &'a Grid<u16>,
}

impl <'a> FixedWhiteLevel<'a> {
	pub fn new(white: u16, grid: &'a Grid<u16>) -> FixedWhiteLevel {
		FixedWhiteLevel {
			white: white,
			grid: grid,
		}
	}
}

impl <'a> Grid<u8> for FixedWhiteLevel<'a> {
	fn get(&self, x: u16, y: u16) -> u8 {
		let val = self.grid.get(x, y);
		let capped = if val > self.white {
			self.white
		} else {
			val
		};
		let scaled = (capped as u32) * ((1 << 8) - 1);
		(scaled / self.white as u32) as u8
	}

	fn width(&self) -> u16 {
		self.grid.width()
	}

	fn height(&self) -> u16 {
		self.grid.height()
	}
}
