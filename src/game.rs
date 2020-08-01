use crate::cell::{Cell, CellState};
// use std::sync::{Arc, Mutex};

pub const TICK: f32 = 1.0 / 20.0;

pub struct GameState {
	cells: Vec<Cell>,
	// shared: Arc<Mutex<Vec<(CellType, Point, )>>>,
}

// struct Renderer {
// 	shared: Arc<Mutex<Vec<Cell>>>
// }