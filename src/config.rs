use crate::cell::Cell;

pub struct Config<'a> {
	/// Speed of each cell in units/ticks
	pub cell_speed: CellSpeed,
	/// maximum rotation angle in radians (per tick)
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
	cells: &[],
};

pub fn get() -> &'static Config<'static> {
	CONFIGURATION
}
