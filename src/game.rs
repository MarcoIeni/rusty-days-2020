use crate::cell::{Cell, InteractionState};
use crate::config;
use crate::renderer::Sync;
use std::sync::{Arc, RwLock};
use std::time::Instant;

pub struct Game {
	cells_a: Vec<Cell>,
	cells_b: Vec<Cell>,
	active: bool, // true a, false b,
	shared: Arc<RwLock<Vec<Cell>>>,
	sync: Sync,
	last_update: Instant,
}

impl Game {
	pub fn new(shared: Arc<RwLock<Vec<Cell>>>, sync: Sync) -> Self {
		let current = shared.read().unwrap().clone();
		Game {
			cells_a: current.clone(),
			cells_b: current,
			active: true,
			shared,
			sync,
			last_update: Instant::now(),
		}
	}

	fn tick(&mut self) {
		let (current, next) = if self.active {
			(&mut self.cells_a, &mut self.cells_b)
		} else {
			(&mut self.cells_b, &mut self.cells_a)
		};
		self.active = !self.active;

		let mut dead = Vec::new();
		for (i, cell) in current.iter().enumerate() {
			let mut inter_state = InteractionState::new(cell);
			for other_cell in current[..i].iter() {
				inter_state.interact(other_cell);
			}
			for other_cell in current[i + 1..].iter() {
				inter_state.interact(other_cell);
			}
			if let Some(child) = inter_state.child() {
				next.push(child);
			}
			if let Some(cell) = inter_state.cell() {
				next[i] = cell;
			} else {
				dead.push(i);
			}
		}
		// If you remove an item at the start of the vector, all the other items indices will be
		// decreased by one, and keeping track of this reordering of the idices is impossible
		// without sorting the items first.
		dead.sort_unstable();
		for (i, cell_idx) in dead.into_iter().enumerate() {
			next.remove(cell_idx - i);
		}
		next.iter_mut().for_each(Cell::advance);
		current.resize_with(next.len(), Default::default);
	}

	pub fn try_send(&self) {
		if let Ok(()) = self.sync.try_recv() {
			// ? When a message is received from the channel, the lock has already been dropped
			if let Ok(mut vec) = self.shared.write() {
				let current = if self.active {
					&self.cells_a
				} else {
					&self.cells_b
				};
				if vec.len() != current.len() {
					vec.resize_with(current.len(), Default::default);
				}
				vec.copy_from_slice(&current);
				std::mem::drop(vec); // Free the lock
				self.sync.send().ok(); // TODO
			} else {
				// TODO
				panic!()
			}
		} else {
			// TODO
		}
	}

	pub fn update(&mut self) {
		if self.last_update.elapsed().as_secs_f32() > config::TICK {
			self.try_send();
			self.last_update = Instant::now();
		}
		self.tick();
	}
}
