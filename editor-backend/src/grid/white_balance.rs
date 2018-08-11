use grid::Grid;

use ::console::log;

pub struct WhiteBalance<'a> {
	red_balance: f32,
	green_balance: f32,
	blue_balance: f32,
	grid: &'a Grid<f32>,
}

fn is_r(x: u16, y: u16) -> bool {
	y % 2 == 0 && x % 2 == 0
}

fn is_b(x: u16, y: u16) -> bool {
	y % 2 == 1 && x % 2 == 1
}

impl <'a> WhiteBalance<'a> {
	pub fn new(red_balance: f32, green_balance: f32, blue_balance: f32, grid: &'a Grid<f32>) -> WhiteBalance {
		WhiteBalance {
			red_balance,
			green_balance,
			blue_balance,
			grid,
		}
	}
}

impl <'a> Grid<f32> for WhiteBalance<'a> {
	fn get(&self, x: u16, y: u16) -> f32 {
		let val = self.grid.get(x, y);

		if x < 256 && y < 256 {
			if is_r(x, y) {
				log(&format!("({}, {}) is red {}", x, y, val));
			} else if is_b(x, y) {
				log(&format!("({}, {}) is blue {}", x, y, val));
			} else {
				log(&format!("({}, {}) is green {}", x, y, val));
			}
		}


		let scaled = if is_r(x, y) {
			val / self.red_balance
		} else if is_b(x, y) {
			val / self.blue_balance
		} else {
			val / self.green_balance
		};
		if scaled > 1.0 {
			1.0
		} else {
			scaled
		}
	}

	fn width(&self) -> u16 {
		self.grid.width()
	}

	fn height(&self) -> u16 {
		self.grid.height()
	}
}
