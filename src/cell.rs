use crate::config;
use crate::point::Point;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum CellState {
	Male = 0,
	Female = 1,
	TiredFemale = 2,
	Child = 3,
	Hunter = 4,
}

impl CellState {
	fn speed(&self) -> f32 {
		let cell_speed = &config::get().cell_speed;
		match self {
			Self::Male => cell_speed.male,
			Self::Female => cell_speed.female,
			Self::TiredFemale => cell_speed.tired_female,
			Self::Child => cell_speed.child,
			Self::Hunter => cell_speed.hunter,
		}
	}
	fn rotation_speed(&self) -> f32 {
		let cell_speed = &config::get().cell_rotation_speed;
		match self {
			Self::Male => cell_speed.male,
			Self::Female => cell_speed.female,
			Self::TiredFemale => cell_speed.tired_female,
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

pub enum InteractionResult {
	Procreates(Cell, Cell),
	Lives(Cell),
	Dies,
}

#[derive(Clone, Copy, Default, Debug)]
#[repr(C, packed)]
pub struct Cell {
	pub state: CellState,
	pub position: Point,
	pub direction: Point,
}

impl Cell {
	pub const fn new(state: CellState, position: Point, direction: Point) -> Self {
		Self {
			state,
			position,
			direction,
		}
	}

	pub fn steer(&mut self, direction: Point) {
		// Get the maximum angle this type of cell can rotate
		let rotation_speed = self.state.rotation_speed() * config::TICK;
		// Compute the dot product between the direction the cell is looking towards and the
		// direction that points towards the other cell.
		// This is used in order to know if the angle between those two directions is smaller
		// than rotation_speed.
		// That's because the dot product is equal to the cosine of that angle.
		let dot = self.direction.dot(direction);
		let angle = if dot > rotation_speed.cos() {
			rotation_speed
		} else {
			// If it's small enough than use that angle
			dot.acos()
		};
		// Compute the dot product between the direction perpendicular to where the cell is
		// looking and the one that points towards the other cell.
		// That is used to know if the other cell is to the left or to the right of this cell
		let is_right = self.direction.rotate_90cw().dot(direction) > 0.0;
		let angle = if is_right { -angle } else { angle };
		self.direction = self.direction.rotate(angle);
	}

	pub fn advance(&mut self) {
		self.position += self.direction * self.state.speed() * config::TICK;
	}

	pub fn interact(&self, other: &Self) -> InteractionResult {
		if self.can_see(other) {}
		// TODO
		InteractionResult::Lives(*self)
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
// pub struct TiredFemale {
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
