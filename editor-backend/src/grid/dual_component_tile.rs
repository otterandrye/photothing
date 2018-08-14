use grid::Grid;
use ::lossless_jpeg::Frame;

pub struct DualComponentTile {
	width: u16,
	height: u16,
	frame: Frame,
}

impl DualComponentTile {
	pub fn new(width: u16, height: u16, frame: Frame) -> DualComponentTile {
		DualComponentTile {
			width,
			height,
			frame,
		}
	}
}

impl Grid<u16> for DualComponentTile {
	fn get(&self, x: u16, y: u16) -> u16 {
		if x % 2 == 0 {
			self.frame.scans[0].components[0].samples[(y * (self.width >> 1) + (x >> 1)) as usize]
		} else {
			self.frame.scans[0].components[1].samples[(y * (self.width >> 1) + (x >> 1)) as usize]
		}
	}

	fn width(&self) -> u16 {
		self.width
	}

	fn height(&self) -> u16 {
		self.height
	}
}
