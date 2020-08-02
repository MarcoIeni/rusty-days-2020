use std::ops::{Add, AddAssign, Mul, Neg, Sub};

#[derive(Clone, Copy, Default, Debug)]
#[repr(C, packed)]
pub struct Point {
	pub x: f32,
	pub y: f32,
}

impl Point {
	pub const fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}

	pub fn dot(self, other: Self) -> f32 {
		self.x * other.x + self.y * other.y
	}

	/// rotate 90 degrees clockwise
	pub fn rotate_90cw(self) -> Self {
		Self {
			x: self.y,
			y: -self.x,
		}
	}

	pub fn rotate(self, alpha: f32) -> Self {
		let c = alpha.cos();
		let s = alpha.sin();
		Self {
			x: self.x * c - self.y * s,
			y: self.x * s + self.y * c,
		}
	}

	pub fn length(self) -> f32 {
		(self.x * self.x + self.y * self.y).sqrt()
	}

	pub fn normilized(self) -> Self {
		self * self.length().recip()
	}

	pub fn between(self, other: Self) -> Self {
		(self + other) * 0.5
	}

	pub fn bisect(self, other: Self) -> Self {
		let c = self.dot(other);
		let s = self.rotate_90cw().dot(other);
		Self {
			x: self.x * c - self.y * s,
			y: self.x * s + self.y * c,
		}
	}
}

impl Add for Point {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl Neg for Point {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
		}
	}
}

impl Sub for Point {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

impl Mul<f32> for Point {
	type Output = Self;

	fn mul(self, other: f32) -> Self::Output {
		Self {
			x: self.x * other,
			y: self.y * other,
		}
	}
}

impl AddAssign for Point {
	fn add_assign(&mut self, other: Self) {
		self.x += other.x;
		self.y += other.y;
	}
}
