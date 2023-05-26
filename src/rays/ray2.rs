
use core::ops::{Neg, Mul, MulAssign, Div, DivAssign};

use crate::Ray3;
use crate::Vector2;
use crate::{MulDivScalar, impl_mul, impl_div};

/// A 2D ray that holds an origin and direction both as 2D vectors
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct Ray2 {
	/// The origin of the ray
	origin: Vector2,
	/// The direction the ray is pointing towards
	direction: Vector2,
}

/// Constructors
impl Ray2 {
	/// Creates a new 2D ray
	/// - **origin**: The origin of the ray
	/// - **direction**: The direction the ray is pointing at
	/// 
	/// **Returns**: Returns a new 2D ray
	/// #### Examples
	/// ```
	/// # use mathx::{Ray2,Vector2};
	/// let ray = Ray2::new(Vector2::one(), Vector2::up());
	/// assert_eq!(Vector2::one(), ray.origin());
	/// assert_eq!(Vector2::up(), ray.direction());
	/// ```
	pub fn new(origin: Vector2, direction: Vector2) -> Self { Ray2 { origin, direction } }
}

/// Properties
impl Ray2 {
	/// Gets the origin of the ray as a 2D vector
	/// 
	/// **Returns**: Returns the origin of the ray
	/// #### Examples
	/// ```
	/// # use mathx::{Ray2,Vector2};
	/// let ray = Ray2::new(Vector2::one(), Vector2::up());
	/// assert_eq!(Vector2::one(), ray.origin());
	/// ```
	pub fn origin(&self) -> Vector2 { self.origin }
	
	/// Sets the origin of the ray
	/// - **value**: The value to set the origin to
	/// #### Examples
	/// ```
	/// # use mathx::{Ray2,Vector2};
	/// let mut ray = Ray2::new(Vector2::one(), Vector2::up());
	/// ray.set_origin(Vector2::up());
	/// assert_eq!(Vector2::up(), ray.origin());
	/// ```
	pub fn set_origin(&mut self, value: Vector2) { self.origin = value; }
	
	/// Gets the direction of the ray as a 2D vector
	/// 
	/// **Returns**: Returns the direction of the ray
	/// #### Examples
	/// ```
	/// # use mathx::{Ray2,Vector2};
	/// let ray = Ray2::new(Vector2::one(), Vector2::up());
	/// assert_eq!(Vector2::up(), ray.direction());
	/// ```
	pub fn direction(&self) -> Vector2 { self.direction }
	
	/// Sets the direction of the ray
	/// - **value**: The value to set the direction to
	/// #### Examples
	/// ```
	/// # use mathx::{Ray2,Vector2};
	/// let mut ray = Ray2::new(Vector2::one(), Vector2::up());
	/// 
	/// ray.set_direction(Vector2::one());
	/// assert_eq!(Vector2::one(), ray.direction());
	/// ```
	pub fn set_direction(&mut self, value: Vector2) { self.direction = value; }
}

/// Public Methods
impl Ray2 {
	/// Gets the point on the ray from the given distance
	/// - **distance**: The distance from the ray to get the point from
	/// 
	/// **Returns**: Returns a 2D point from the given distance from the ray
	/// #### Examples
	/// ```
	/// # use mathx::{Ray2, Vector2};
	/// let ray = Ray2::new(Vector2::one(), Vector2::up());
	/// let point = ray.get_point(4.3);
	/// assert_eq!(Vector2::new(1.0, 5.3), point);
	/// ```
	pub fn get_point(self, distance: f32) -> Vector2 {
		let dir = self.direction * distance;
		
		return self.origin + dir;
	}
	
	/// Gets the closest point on the ray from the given point
	/// - **point**: The point to get the closest point from
	/// 
	/// **Returns**: Returns the closest point from the given point
	/// #### Examples
	/// ```
	/// # use mathx::{Ray2, Vector2};
	/// let ray = Ray2::new(Vector2::one(), Vector2::up());
	/// let point = ray.closest_point(Vector2::down());
	/// assert_eq!(Vector2::new(1.0, -1.0), point);
	/// ```
	pub fn closest_point(self, point: Vector2) -> Vector2 {
		let diff = point - self.origin;
		let projected = diff.project(self.direction);
		
		return projected + self.origin;
	}
	
	/// Gets the distance between the point and the ray's line
	/// - **point**: The point to check the distance from
	/// 
	/// **Returns**: Returns the distance between the point and the ray's line
	/// #### Examples
	/// ```
	/// # use mathx::{Ray2, Vector2};
	/// let ray = Ray2::new(Vector2::zero(), Vector2::left());
	/// let distance = ray.distance(Vector2::down());
	/// assert_eq!(1.0, distance);
	/// let ray = Ray2::new(Vector2::one(), Vector2::left());
	/// let distance = ray.distance(Vector2::down());
	/// assert_eq!(2.0, distance);
	/// ```
	pub fn distance(self, point: Vector2) -> f32 { point.distance(self.closest_point(point)) }
}

impl From<Ray3> for Ray2 {
	fn from(value: Ray3) -> Self {
		Ray2::new(value.origin().to_vector2(), value.direction().to_vector2())
	}
}

unsafe impl Send for Ray2 {}
unsafe impl Sync for Ray2 {}

impl Eq for Ray2 {}
impl PartialEq for Ray2 {
	fn eq(&self, other: &Self) -> bool {
		self.origin == other.origin
		&& self.direction == other.direction
	}
}

// Display
#[cfg(not(feature = "no_std"))]
impl std::fmt::Display for Ray2 {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.write_str(&format!("{{ origin: {}, direction: {} }}", self.origin, self.direction))
	}
}

impl MulDivScalar for Ray2 {
	type Output = Ray2;
	fn multiply_scalar(self, rhs: f32) -> Self::Output {
		Ray2::new(self.origin, rhs * self.direction)
	}
	fn multiply_assign_scalar(&mut self, rhs: f32) {
		self.direction *= rhs;
	}
	fn divide_scalar(self, rhs: f32) -> Self::Output {
		Ray2::new(self.origin, self.direction / rhs)
	}
	fn divide_assign_scalar(&mut self, rhs: f32) {
		self.direction /= rhs;
	}
	fn reciprocal_scalar(self, rhs: f32) -> Self::Output {
		Ray2::new(self.origin, rhs / self.direction)
	}
}

impl Neg for Ray2 {
	type Output = Ray2;
	fn neg(self) -> Self::Output { Ray2::new(self.origin, -self.direction) }
}

impl_mul!(Ray2);
impl_div!(Ray2);
