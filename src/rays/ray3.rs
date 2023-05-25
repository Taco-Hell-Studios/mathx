
use core::ops::{Neg, Mul, MulAssign, Div, DivAssign};

use crate::Vector3;
use crate::{MulDivScalar, impl_mul, impl_div};

/// A 3D ray that holds an origin and direction both as 3D vectors
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct Ray3 {
	/// The origin of the ray
	origin: Vector3,
	/// The direction the ray is pointing towards
	direction: Vector3,
}

/// Constructors
impl Ray3 {
	/// Creates a new 3D ray
	/// - **origin**: The origin of the ray
	/// - **direction**: The direction the ray is pointing at
	/// 
	/// **Returns**: Returns a new 3D ray
	/// #### Examples
	/// ```
	/// # use mathx::{Ray3,Vector3};
	/// let ray = Ray3::new(Vector3::one(), Vector3::forward());
	/// assert_eq!(Vector3::one(), ray.origin());
	/// assert_eq!(Vector3::forward(), ray.direction());
	/// ```
	pub fn new(origin: Vector3, direction: Vector3) -> Self { Ray3 { origin, direction } }
}

/// Properties
impl Ray3 {
	/// Gets the origin of the ray as a 3D vector
	/// 
	/// **Returns**: Returns the origin of the ray
	/// #### Examples
	/// 
	pub fn origin(&self) -> Vector3 { self.origin }
	pub fn set_origin(&mut self, value: Vector3) { self.origin = value; }
	pub fn direction(&self) -> Vector3 { self.direction }
	pub fn set_direction(&mut self, value: Vector3) { self.direction = value; }
}

/// Public Methods
impl Ray3 {
	pub fn get_point(self, distance: f32) -> Vector3 {
		let dir = self.direction * distance;
		
		return self.origin + dir;
	}
	pub fn closest_point(self, point: Vector3) -> Vector3 {
		let diff = point - self.origin;
		let projected = diff.project(self.direction);
		
		return projected + self.origin;
	}
	pub fn distance(self, point: Vector3) -> f32 { point.distance(self.closest_point(point)) }
}

unsafe impl Send for Ray3 {}
unsafe impl Sync for Ray3 {}

impl Eq for Ray3 {}
impl PartialEq for Ray3 {
	fn eq(&self, other: &Self) -> bool {
		self.origin == other.origin
		&& self.direction == other.direction
	}
}

// Display
#[cfg(not(feature = "no_std"))]
impl std::fmt::Display for Ray3 {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.write_str(&format!("{{ origin: {}, direction: {} }}", self.origin, self.direction))
	}
}

impl MulDivScalar for Ray3 {
	type Output = Ray3;
	fn multiply_scalar(self, rhs: f32) -> Self::Output {
		Ray3::new(self.origin, rhs * self.direction)
	}
	fn multiply_assign_scalar(&mut self, rhs: f32) {
		self.direction *= rhs;
	}
	fn divide_scalar(self, rhs: f32) -> Self::Output {
		Ray3::new(self.origin, self.direction / rhs)
	}
	fn divide_assign_scalar(&mut self, rhs: f32) {
		self.direction /= rhs;
	}
	fn reciprocal_scalar(self, rhs: f32) -> Self::Output {
		Ray3::new(self.origin, rhs / self.direction)
	}
}

impl Neg for Ray3 {
	type Output = Ray3;
	fn neg(self) -> Self::Output { Ray3::new(self.origin, -self.direction) }
}

impl_mul!(Ray3);
impl_div!(Ray3);
