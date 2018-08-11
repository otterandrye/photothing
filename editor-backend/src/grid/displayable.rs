use grid::Grid;

pub struct Displayable<'a> {
	grid: &'a Grid<f32>,
}

impl <'a> Displayable<'a> {
	pub fn new(grid: &'a Grid<f32>) -> Displayable {
		Displayable {
			grid,
		}
	}
}

impl <'a> Grid<u8> for Displayable<'a> {
	fn get(&self, x: u16, y: u16) -> u8 {
		let val = self.grid.get(x, y);
		(val * 255.0) as u8
	}

	fn width(&self) -> u16 {
		self.grid.width()
	}

	fn height(&self) -> u16 {
		self.grid.height()
	}
}
