use crate::cell::{Cell, CellState};
use crate::point::Point;
use std::f32::consts::FRAC_1_SQRT_2;

pub const TICK: f32 = 1.0 / 100.0;

pub struct Config<'a> {
	/// Speed of each cell in units/s
	pub cell_speed: CellSpeed,
	/// maximum rotation angle in radians (measured in deg/s)
	pub cell_rotation_speed: CellSpeed,
	pub cells: &'a [Cell], // Maybe use a Vec?
}

pub struct CellSpeed {
	pub male: f32,
	pub female: f32,
	pub tired_female: f32,
	pub child: f32,
	pub hunter: f32,
}

static CONFIGURATION: &Config = &Config {
	cell_speed: CellSpeed {
		male: 1.0,
		female: 1.0,
		tired_female: 1.0,
		child: 1.0,
		hunter: 1.0,
	},
	cell_rotation_speed: CellSpeed {
		male: 1.0,
		female: 1.0,
		tired_female: 1.0,
		child: 1.0,
		hunter: 1.0,
	},
	cells: &[Cell::new(
		CellState::Female,
		Point::new(0.0, 0.0),
		Point::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2), // 45 deg
	)],
};

pub fn get() -> &'static Config<'static> {
	CONFIGURATION
}
