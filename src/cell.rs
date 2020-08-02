use crate::config;
use crate::point::Point;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
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

	fn ignores(&self, other: Self) -> bool {
		match (self, other) {
			(Self::Male, Self::Female) => false,
			(Self::Female, Self::Male) => false,
			(Self::Hunter, Self::Hunter) => true,
			(Self::Hunter, _) => false,
			(_, Self::Hunter) => false,
			(Self::Child, Self::Male) | (Self::Child, Self::Female) => false,
			_ => true,
		}
	}

	fn sight_length(&self) -> f32 {
		let sights = &config::get().sights;
		match self {
			Self::Male => sights.male,
			Self::Female => sights.female,
			Self::TiredFemale => sights.female,
			Self::Child => sights.child,
			Self::Hunter => sights.hunter,
		}
	}
}

impl Default for CellState {
	fn default() -> Self {
		Self::Male
	}
}

pub struct InteractionState<'a> {
	/// nearest cell and distance to it
	nearest: Option<(&'a Cell, f32)>,
	alive: bool,
	counter: Option<isize>,
	cell: &'a Cell,
}

impl<'a> InteractionState<'a> {
	pub const fn new(cell: &'a Cell) -> Self {
		Self {
			nearest: None,
			alive: true,
			counter: None,
			cell,
		}
	}

	fn update_nearest(&mut self, distance: f32, other_cell: &'a Cell) {
		match self.nearest {
			None => self.nearest = Some((other_cell, distance)),
			Some((_, curr_distance)) => {
				if distance < curr_distance {
					self.nearest = Some((other_cell, distance));
				}
			}
		}
	}

	pub fn interact(&mut self, other: &'a Cell) {
		use CellState as CS;
		if self.cell.state.ignores(other.state) {
			return;
		}

		let growth_time = config::get().growth_time;

		if let Some(distance) = self.cell.can_see(other) {
			match (self.cell.state, other.state) {
				(CS::Hunter, CS::Hunter) => unreachable!(),
				(_, CS::Hunter) => {
					let hunter_kill_range = config::get().hunter_kill_range;
					if distance < hunter_kill_range {
						self.alive = false;
					}
				}
				(CS::Hunter, _) | (CS::Female, CS::Male) | (CS::Male, CS::Female) => {
					self.update_nearest(distance, other);
				}
				(CS::Child, CS::Female) => {
					if self.cell.age == growth_time {
						if let Some(ref mut val) = self.counter {
							*val += 1;
						} else {
							self.counter = Some(1);
						}
					}
				}
				(CS::Child, CS::Male) => {
					if self.cell.age == growth_time {
						if let Some(ref mut val) = self.counter {
							*val -= 1;
						} else {
							self.counter = Some(-1);
						}
					}
				}
				_ => unreachable!(),
			}
		}
	}

	/// if the cell is female and she had a child it returns the child itself, None otherwise.
	pub fn child(&self) -> Option<Cell> {
		if self.cell.state == CellState::Female {
			if let Some((father, _)) = self.nearest {
				let child_position = self.cell.position.between(father.position);
				let child_direction = -self.cell.direction.bisect(father.direction);
				Some(Cell::new(CellState::Child, child_position, child_direction))
			} else {
				None
			}
		} else {
			None
		}
	}

	/// Some if alive, None if it's dead
	pub fn cell(&self) -> Option<Cell> {
		let tired_time = config::get().tired_time;
		let growth_time = config::get().growth_time;

		if self.alive {
			let self_cell = match self.cell.state {
				CellState::Male | CellState::Hunter => {
					if let Some((cell, _)) = self.nearest {
						let direction = cell.position - self.cell.position;
						let mut self_cell = *self.cell;
						self_cell.steer(direction.normilized());
						self_cell
					} else {
						*self.cell
					}
				}
				CellState::Female => {
					if self.nearest.is_some() {
						Cell::new(
							CellState::TiredFemale,
							self.cell.position,
							self.cell.direction,
						)
					} else {
						*self.cell
					}
				}
				CellState::TiredFemale if self.cell.age == tired_time => {
					Cell::new(CellState::Female, self.cell.position, self.cell.direction)
				}
				CellState::Child if self.cell.age == growth_time => {
					let state = if let Some(val) = self.counter {
						if val < 0 {
							CellState::Female
						} else {
							CellState::Male
						}
					} else {
						CellState::Hunter
					};
					Cell::new(state, self.cell.position, self.cell.direction)
				}
				_ => *self.cell,
			};
			Some(self_cell)
		} else {
			None
		}
	}
}

#[derive(Clone, Copy, Default, Debug, Deserialize)]
#[repr(C, packed)]
pub struct Cell {
	pub state: CellState,
	pub position: Point,
	pub direction: Point,
	#[serde(skip)]
	pub age: usize,
}

impl Cell {
	pub const fn new(state: CellState, position: Point, direction: Point) -> Self {
		Self {
			state,
			position,
			direction,
			age: 0,
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
			if dot > 1.0 {
				0.0
			} else {
				dot.acos()
			}
		} else {
			rotation_speed
		};
		// Compute the dot product between the direction perpendicular to where the cell is
		// looking and the one that points towards the other cell.
		// That is used to know if the other cell is to the left or to the right of this cell
		let is_right = self.direction.rotate_90cw().dot(direction) > 0.0;
		let angle = if is_right { -angle } else { angle };
		self.direction = self.direction.rotate(angle);
	}

	pub fn advance(&mut self) {
		use std::f32::consts as c;

		let tired_time = config::get().tired_time;

		if self.state == CellState::Child {
			self.age += 1;
		} else if self.state == CellState::TiredFemale {
			if self.age == tired_time {
				self.state = CellState::Female;
			} else {
				self.age += 1;
			}
		}

		let sight = config::get().wall_detect_range;
		let wall = config::get().world_size;
		if self.position.x + sight > wall {
			if self.position.y + sight > wall {
				self.steer(Point::new(-c::FRAC_1_SQRT_2, -c::FRAC_1_SQRT_2));
			} else if self.position.y - sight < -wall {
				self.steer(Point::new(-c::FRAC_1_SQRT_2, c::FRAC_1_SQRT_2));
			} else {
				self.steer(Point::new(-1.0, 0.0));
			}
		} else if self.position.x - sight < -wall {
			if self.position.y + sight > wall {
				self.steer(Point::new(c::FRAC_1_SQRT_2, -c::FRAC_1_SQRT_2));
			} else if self.position.y - sight < -wall {
				self.steer(Point::new(c::FRAC_1_SQRT_2, c::FRAC_1_SQRT_2));
			} else {
				self.steer(Point::new(1.0, 0.0));
			}
		} else {
			if self.position.y + sight > wall {
				self.steer(Point::new(0.0, -1.0));
			} else if self.position.y - sight < -wall {
				self.steer(Point::new(0.0, 1.0));
			}
		}
		self.position += self.direction * self.state.speed() * config::TICK;
	}

	/// Some(distance) if can see, None otherwise
	pub fn can_see(&self, other: &Self) -> Option<f32> {
		let difference = other.position - self.position;
		let distance = difference.length();
		let sight_length = self.state.sight_length();

		if distance < sight_length {
			if self.state == CellState::Hunter {
				let hunter_fov = config::get().hunter_fov;
				if self.direction.dot(difference) > (hunter_fov * 0.5).cos() {
					Some(distance)
				} else {
					None
				}
			} else {
				Some(distance)
			}
		} else {
			None
		}
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
