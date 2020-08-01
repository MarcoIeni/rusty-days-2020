use crate::cell::{Cell, CellState};
use crate::renderer::Sync;

pub const TICK: f32 = 1.0 / 20.0;

struct GameState {
	current: Vec<Cell>,
	next: Vec<Cell>,
	shared: Arc<Mutex<Vec<Cell>>>,
	sync: Sync,
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

	pub fn try_send(&self) {
		if let Ok(()) = self.sync.0.try_recv() {
			// When a message is received from the channel, the lock has already been dropped
			let vec = self.shared.lock().unwrap();
			if vec.len() != self.current.len() {
				vec.resize(self.current.len(), Cell::default());
			}
			vec.copy_from_slice(&self.current)
		}
	}
}
