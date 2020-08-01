use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Point {
	pub x: f32,
	pub y: f32,
}

impl Point {
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

	pub fn length(&self) -> f32 {
		(self.x * self.x + self.y * self.y).sqrt()
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
