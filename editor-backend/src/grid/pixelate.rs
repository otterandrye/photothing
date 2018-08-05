use grid::Grid;

pub struct Pixel {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

pub struct Pixelate<'a> {
	grid: &'a Grid<u8>,
}

impl <'a> Pixelate<'a> {
	pub fn new(grid: &'a Grid<u8>) -> Pixelate {
		Pixelate {
			grid: grid,
		}
	}

	fn nearby(&self, neighborhood: Neighborhood, x: u16, y: u16) -> u8{
		match neighborhood {
			Neighborhood::X => {
				let mut sum: u32 = 0;
				let mut neighbors: u8 = 0;
				if x > 0 && y > 0 {
					sum += self.grid.get(x - 1, y - 1) as u32;
					neighbors += 1;
				}
				if x > 0 && y < self.height() - 1 {
					sum += self.grid.get(x - 1, y + 1) as u32;
					neighbors += 1;
				}
				if x < self.width() - 1 && y < self.height() - 1 {
					sum += self.grid.get(x + 1, y + 1) as u32;
					neighbors += 1;
				}
				if x < self.width() - 1 && y > 0 {
					sum += self.grid.get(x + 1, y - 1) as u32;
					neighbors += 1;
				}

				(sum / neighbors as u32) as u8
			},
			Neighborhood::Plus => {
				let mut sum: u32 = 0;
				let mut neighbors: u8 = 0;
				if x > 0 {
					sum += self.grid.get(x - 1, y) as u32;
					neighbors += 1;
				}
				if y > 0 {
					sum += self.grid.get(x, y - 1) as u32;
					neighbors += 1;
				}
				if y < self.height() - 1 {
					sum += self.grid.get(x, y + 1) as u32;
					neighbors += 1;
				}
				if x < self.width() - 1 {
					sum += self.grid.get(x + 1, y) as u32;
					neighbors += 1;
				}

				(sum / neighbors as u32) as u8
			},
			Neighborhood::Vertical => {
				let mut sum: u32 = 0;
				let mut neighbors: u8 = 0;

				if y > 0 {
					sum += self.grid.get(x, y - 1) as u32;
					neighbors += 1;
				}
				if y < self.height() - 1 {
					sum += self.grid.get(x, y + 1) as u32;
					neighbors += 1;
				}

				(sum / neighbors as u32) as u8
			},
			Neighborhood::Horizontal => {
				let mut sum: u32 = 0;
				let mut neighbors: u8 = 0;

				if x > 0 {
					sum += self.grid.get(x - 1, y) as u32;
					neighbors += 1;
				}
				if x < self.width() - 1 {
					sum += self.grid.get(x + 1, y) as u32;
					neighbors += 1;
				}

				(sum / neighbors as u32) as u8
			},
		}
	}
}

fn is_r(x: u16, y: u16) -> bool {
	y % 2 == 0 && x % 2 == 0
}

fn is_b(x: u16, y: u16) -> bool {
	y % 2 == 1 && x % 2 == 1
}

fn is_g(x: u16, y: u16) -> bool {
	!is_r(x, y) && !is_b(x, y)
}

enum Neighborhood {
	Vertical,
	Horizontal,
	X,
	Plus,
}

impl <'a> Grid<Pixel> for Pixelate<'a> {
	fn get(&self, x: u16, y: u16) -> Pixel {
		let val = self.grid.get(x, y);
		Pixel {
			red: if is_r(x, y) { val } else {
				if is_b(x, y) {
					self.nearby(Neighborhood::X, x, y)
				} else {
					if y % 2 == 0 {
						self.nearby(Neighborhood::Horizontal, x, y)
					} else {
						self.nearby(Neighborhood::Vertical, x, y)
					}
				}
			},
			green: if is_g(x, y) { val } else {
				self.nearby(Neighborhood::Plus, x, y)
			},
			blue: if is_b(x, y) { val } else {
				if is_r(x, y) {
					self.nearby(Neighborhood::X, x, y)
				} else {
					if y % 2 == 1 {
						self.nearby(Neighborhood::Horizontal, x, y)
					} else {
						self.nearby(Neighborhood::Vertical, x, y)
					}
				}
			},
		}
	}

	fn width(&self) -> u16 {
		self.grid.width()
	}

	fn height(&self) -> u16 {
		self.grid.height()
	}
}
