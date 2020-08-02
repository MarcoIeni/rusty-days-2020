use crate::cell::{Cell, CellState};
use crate::point::Point;
use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2};

pub const TICK: f32 = 0.001;

pub struct Config<'a> {
	/// Speed of each cell in units/s
	pub cell_speed: CellSpeed,
	/// maximum rotation angle in radians (measured in deg/s)
	pub cell_rotation_speed: CellSpeed,
	pub cells: &'a [Cell], // Maybe use a Vec?
	pub sights: CellSightLength,
	pub hunter_kill_range: f32,
	// hunter field of view angle
	pub hunter_fov: f32,
	pub wall_detect_range: f32,
	pub world_size: f32,
	pub cell_size: f32,
}

pub struct CellSpeed {
	pub male: f32,
	pub female: f32,
	pub tired_female: f32,
	pub child: f32,
	pub hunter: f32,
}

pub struct CellSightLength {
	pub male: f32,
	pub female: f32,
	pub child: f32,
	pub hunter: f32,
}

static CONFIGURATION: &Config = &Config {
	cell_speed: CellSpeed {
		male: 1.0,
		female: 1.0,
		tired_female: 0.7,
		child: 2.0,
		hunter: 1.7,
	},
	cell_rotation_speed: CellSpeed {
		male: 0.2,
		female: 0.2,
		tired_female: 0.2,
		child: 0.2,
		hunter: 0.2,
	},
	sights: CellSightLength {
		male: 20.0,
		female: 5.0,
		child: 10.0,
		hunter: 20.0,
	},
	hunter_fov: FRAC_PI_2,
	hunter_kill_range: 2.0,
	wall_detect_range: 10.0,
	world_size: 100.0,
	cell_size: 2.0,
	cells: &[
		Cell::new(
			CellState::Male,
			Point::new(12.0, 10.0),
			Point::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2), // 45 deg
		),
		Cell::new(
			CellState::Female,
			Point::new(-12.0, -10.0),
			Point::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2), // 45 deg
		),
		Cell::new(
			CellState::Female,
			Point::new(-10.0, -1.0),
			Point::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2), // 45 deg
		),
		Cell::new(
			CellState::Female,
			Point::new(-1.0, 10.0),
			Point::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2), // 45 deg
		),
	],
};

pub fn get() -> &'static Config<'static> {
	CONFIGURATION
}
