pub trait Grid<T> {
	fn get(&self, x: u16, y: u16) -> T;
	fn width(&self) -> u16;
	fn height(&self) -> u16;
}

pub mod tile;
pub mod fixed_black_level;
pub mod fixed_white_level;
pub mod pixelate;
pub mod composite_tile_grid;
pub mod crop;

pub mod displayable;
pub mod white_balance;
pub mod white_scale;
pub mod dual_component_tile;