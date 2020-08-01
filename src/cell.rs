use crate::point::Point;

/// maximum rotation angle in radians (per tick)
const ROTATION_SPEED: f32 = 0.1;

pub enum CellState {
    Male,
    Female,
    SlowFemale,
    Child,
    Hunter,
}

pub struct Cell {
	pub state: CellState,
	pub position: Point,
	pub direction: Point,
}

impl Cell {
	pub fn steer(&mut self, direction: Point) {
		let dot = self.direction.dot(direction);
		let is_right = self.direction.rotate_90cw().dot(direction) > 0.0;
		let angle = if dot > ROTATION_SPEED.cos() {
			ROTATION_SPEED
		} else {
			dot.acos()
		};
		let angle = if is_right {
			-angle
		} else {
			angle
		};
		self.direction = self.direction.rotate(angle);
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