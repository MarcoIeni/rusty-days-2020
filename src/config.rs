use crate::cell::Cell;
use serde::Deserialize;

pub const TICK: f32 = 0.001;

#[derive(Deserialize)]
pub struct Config {
	/// Speed of each cell in units/s
	pub cell_speed: CellSpeed,
	/// maximum rotation angle in radians (measured in deg/s)
	pub cell_rotation_speed: CellSpeed,
	pub cells: Vec<Cell>, // Maybe use a Vec?
	pub sights: CellSightLength,
	pub hunter_kill_range: f32,
	// hunter field of view angle
	pub hunter_fov: f32,
	pub wall_detect_range: f32,
	pub world_size: f32,
	pub cell_size: f32,
	pub tired_time: usize,
	pub growth_time: usize,
	pub hunter_lifetime: usize,
}

#[derive(Deserialize)]
pub struct CellSpeed {
	pub male: f32,
	pub female: f32,
	pub tired_female: f32,
	pub child: f32,
	pub hunter: f32,
}

#[derive(Deserialize)]
pub struct CellSightLength {
	pub male: f32,
	pub female: f32,
	pub child: f32,
	pub hunter: f32,
}

lazy_static::lazy_static! {
	static ref CONFIGURATION: Config =
		serde_json::from_reader(std::fs::File::open("config.json").unwrap()).unwrap();
}

pub fn get() -> &'static Config {
	&CONFIGURATION
}
