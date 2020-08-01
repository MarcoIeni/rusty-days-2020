use crate::cell::{Cell, CellState};
// use std::sync::{Arc, Mutex};

pub const TICK: f32 = 1.0 / 20.0;

struct GameState {
    current: Vec<Cell>,
    next: Vec<Cell>,
    // shared: Arc<Mutex<Vec<(CellType, Point, )>>>,
}

impl GameState {
    fn tick(&mut self) {
        for (i, cell) in self.current.iter().enumerate() {
            for (j, other_cell) in self.current.iter().enumerate().filter(|(j, _c)| *j != i) {
                let next_cell = cell.interact(other_cell);
                self.next[i] = next_cell;
            }
        }
    }
}

// struct Renderer {
// 	shared: Arc<Mutex<Vec<Cell>>>
// }
