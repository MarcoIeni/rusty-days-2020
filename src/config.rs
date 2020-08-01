struct Config {
    cell_speed: CellSpeed,
    /// how much a female is slowed after making a child
    slow_factor: f32,
}

struct CellSpeed {
    male: f32,
    female: f32,
    child: f32,
    hunter: f32,
}
