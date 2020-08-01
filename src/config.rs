pub struct Config {
	/// Speed of each cell in units/ticks
	pub cell_speed: CellSpeed,
	/// maximum rotation angle in radians (per tick)
	pub cell_rotation_speed: CellSpeed,
}

pub struct CellSpeed {
	pub male: f32,
	pub female: f32,
	pub tired_female: f32,
	pub child: f32,
	pub hunter: f32,
}

static CONFIGURATION: Config = Config {
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
};

pub fn get() -> &'static Config {
	&CONFIGURATION
}
