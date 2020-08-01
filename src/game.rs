use crate::cell::{Cell, CellState};
use crate::config;
use crate::renderer::Sync;
use std::sync::{Arc, RwLock};
use std::time::Instant;

pub const TICK: f32 = 1.0 / 100.0;

pub struct Game {
	current: Vec<Cell>,
	next: Vec<Cell>,
	shared: Arc<RwLock<Vec<Cell>>>,
	sync: Sync,
	tick_count: f32,
}

impl Game {
	pub fn new(shared: Arc<RwLock<Vec<Cell>>>, sync: Sync) -> Self {
		let current = shared.read().unwrap().clone();
		Game {
			next: Vec::with_capacity(current.len()),
			current,
			shared,
			sync,
			tick_count: 0.0,
		}
	}

	fn tick(&mut self) {
		self.tick_count += TICK;

		let mut dead = Vec::new();
		for (i, cell) in self.current.iter().enumerate() {
			for other_cell in self.current[..i].iter() {
				match cell.interact(other_cell) {
					Some(cell) => self.next[i] = cell,
					None => dead.push(i),
				}
			}
			for other_cell in self.current[i + 1..].iter() {
				match cell.interact(other_cell) {
					Some(cell) => self.next[i] = cell,
					None => dead.push(i),
				}
			}
		}
	}

	pub fn try_send(&self) {
		if let Ok(()) = self.sync.try_recv() {
			// ? When a message is received from the channel, the lock has already been dropped
			if let Ok(mut vec) = self.shared.write() {
				if vec.len() != self.current.len() {
					vec.resize(self.current.len(), Cell::default());
				}
				vec.copy_from_slice(&self.current)
			} else {
				// TODO
			}
		}
	}

	pub fn run(&mut self) {
		loop {
			let instant = Instant::now();
			self.try_send();
			self.tick_count = 0.0;
			// Execute ticks until the real time passes
			while instant.elapsed().as_secs_f32() < self.tick_count {
				self.tick();
			}
		}
	}
}
