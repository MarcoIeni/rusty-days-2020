use crate::config;
use crate::point::Point;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum CellState {
	Male = 1,
	Female = 2,
	SlowFemale = 3,
	Child = 4,
	Hunter = 5,
}

impl CellState {
	fn speed(&self) -> f32 {
		let cell_speed = &config::get().cell_speed;
		let slow_factor = &config::get().slow_factor;
		match self {
			Self::Male => cell_speed.male,
			Self::Female => cell_speed.female,
			Self::SlowFemale => cell_speed.female * slow_factor,
			Self::Child => cell_speed.child,
			Self::Hunter => cell_speed.hunter,
		}
	}
}

impl Default for CellState {
	fn default() -> Self {
		Self::Male
	}
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Cell {
	pub state: CellState,
	pub position: Point,
	pub direction: Point,
}

impl Cell {
	pub fn steer(&mut self, direction: Point) {
		let rotation_speed = config::get().rotation_speed;
		let dot = self.direction.dot(direction);
		let is_right = self.direction.rotate_90cw().dot(direction) > 0.0;
		let angle = if dot > rotation_speed.cos() {
			rotation_speed
		} else {
			dot.acos()
		};
		let angle = if is_right { -angle } else { angle };
		self.direction = self.direction.rotate(angle);
	}

	pub fn advance(&mut self) {
		self.position += self.direction * self.state.speed();
	}

	pub fn interact(&self, other: &Self) -> Self {
		if self.can_see(other) {}
		// TODO
	}

	pub fn can_see(&self, other: &Self) -> bool {
		let difference = other.position - self.position;
		let distance = difference.length();
		// TODO match self.state {}
		true
	}
}

// pub struct Male {
// 	position: Point,
// 	direction: Point,
// }
// pub struct Female {
// 	position: Point,
// 	direction: Point,
// }
// pub struct SlowFemale {
// 	position: Point,
// 	direction: Point,
// }
// pub struct Child {
// 	position: Point,
// 	direction: Point,
// }
// pub struct Hunter {
// 	position: Point,
// 	direction: Point,
// }
