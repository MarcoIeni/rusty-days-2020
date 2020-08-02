use crate::cell::{Cell, InteractionResult};
use crate::config;
use crate::renderer::Sync;
use std::sync::{Arc, RwLock};
use std::time::Instant;

pub struct Game {
	current: Vec<Cell>,
	next: Vec<Cell>,
	shared: Arc<RwLock<Vec<Cell>>>,
	sync: Sync,
	// tick_count: f32,
	last_update: Instant,
}

impl Game {
	pub fn new(shared: Arc<RwLock<Vec<Cell>>>, sync: Sync) -> Self {
		let current = shared.read().unwrap().clone();
		Game {
			next: current.clone(),
			current,
			shared,
			sync,
			// tick_count: 0.0,
			last_update: Instant::now(),
		}
	}

	fn interation_result(
		result: InteractionResult,
		id: usize,
		cells: &mut Vec<Cell>,
		deads: &mut Vec<usize>,
	) {
		match result {
			InteractionResult::Lives(cell) => {
				cells[id] = cell;
			}
			InteractionResult::Procreates(cell, child) => {
				cells[id] = cell;
				cells.push(child);
			}
			InteractionResult::Dies => deads.push(id),
		}
	}

	fn tick(&mut self) {
		let mut dead = Vec::new();
		for (i, cell) in self.current.iter().enumerate() {
			for other_cell in self.current[..i].iter() {
				Self::interation_result(cell.interact(other_cell), i, &mut self.next, &mut dead);
			}
			for other_cell in self.current[i + 1..].iter() {
				Self::interation_result(cell.interact(other_cell), i, &mut self.next, &mut dead);
			}
		}
		// If you remove an item at the start of the vector, all the other items indices will be
		// decreased by one, and keeping track of this reordering of the idices is impossible
		// without sorting the items first.
		dead.sort_unstable();
		for (i, cell_idx) in dead.into_iter().enumerate() {
			self.next.remove(cell_idx - i);
		}
		self.next.iter_mut().for_each(Cell::advance);
		self.current = self.next.clone();
	}

	pub fn try_send(&self) {
		if let Ok(()) = self.sync.try_recv() {
			// ? When a message is received from the channel, the lock has already been dropped
			if let Ok(mut vec) = self.shared.write() {
				if vec.len() != self.current.len() {
					vec.resize(self.current.len(), Cell::default());
				}
				vec.copy_from_slice(&self.current);
				std::mem::drop(vec); // Free the lock
				self.sync.send().ok(); // TODO
			} else {
				// TODO
			}
		} else {
			// TODO
		}
	}

	pub fn update(&mut self) {
		// Execute ticks until the relative amout of real time passes (?)
		if self.last_update.elapsed().as_secs_f32() > config::TICK {
			self.try_send();
			self.last_update = Instant::now();
		}
		self.tick();
	}
}
