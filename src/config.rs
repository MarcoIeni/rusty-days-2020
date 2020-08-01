pub struct Config {
    pub cell_speed: CellSpeed,
    /// how much a female is slowed after making a child
    pub slow_factor: f32,
    /// maximum rotation angle in radians (per tick)
    pub rotation_speed: f32,
}

pub struct CellSpeed {
    pub male: f32,
    pub female: f32,
    pub child: f32,
    pub hunter: f32,
}

impl Config {
    const fn default() -> Self {
        Self {
            cell_speed: CellSpeed {
                male: 1.0,
                female: 1.0,
                child: 1.0,
                hunter: 1.0,
            },
            slow_factor: 1.0,
            rotation_speed: 1.0,
        }
    }
}

static CONFIGURATION: Config = Config::default();

pub fn get() -> &'static Config {
    &CONFIGURATION
}
