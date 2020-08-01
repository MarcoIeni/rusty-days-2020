use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
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

impl Mul<f32> for Point {
	type Output = Self;
	
	fn mul(self, other: f32) -> Self::Output {
		Self {
			x: self.x * other,
			y: self.y * other,
		}
	}
}