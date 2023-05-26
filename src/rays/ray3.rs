
use core::ops::{Neg, Mul, MulAssign, Div, DivAssign};

use crate::Ray2;
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
	/// ```
	/// # use mathx::{Ray3,Vector3};
	/// let ray = Ray3::new(Vector3::one(), Vector3::forward());
	/// assert_eq!(Vector3::one(), ray.origin());
	/// ```
	pub fn origin(&self) -> Vector3 { self.origin }
	
	/// Sets the origin of the ray
	/// - **value**: The value to set the origin to
	/// #### Examples
	/// ```
	/// # use mathx::{Ray3,Vector3};
	/// let mut ray = Ray3::new(Vector3::one(), Vector3::forward());
	/// ray.set_origin(Vector3::forward());
	/// assert_eq!(Vector3::forward(), ray.origin());
	/// ```
	pub fn set_origin(&mut self, value: Vector3) { self.origin = value; }
	
	/// Gets the direction of the ray as a 3D vector
	/// 
	/// **Returns**: Returns the direction of the ray
	/// #### Examples
	/// ```
	/// # use mathx::{Ray3,Vector3};
	/// let ray = Ray3::new(Vector3::one(), Vector3::forward());
	/// assert_eq!(Vector3::forward(), ray.direction());
	/// ```
	pub fn direction(&self) -> Vector3 { self.direction }
	
	/// Sets the direction of the ray
	/// - **value**: The value to set the direction to
	/// #### Examples
	/// ```
	/// # use mathx::{Ray3,Vector3};
	/// let mut ray = Ray3::new(Vector3::one(), Vector3::forward());
	/// 
	/// ray.set_direction(Vector3::one());
	/// assert_eq!(Vector3::one(), ray.direction());
	/// ```
	pub fn set_direction(&mut self, value: Vector3) { self.direction = value; }
}

/// Public Methods
impl Ray3 {
	/// Gets the point on the ray from the given distance
	/// - **distance**: The distance from the ray to get the point from
	/// 
	/// **Returns**: Returns a 3D point from the given distance from the ray
	/// #### Examples
	/// ```
	/// # use mathx::{Ray3, Vector3};
	/// let ray = Ray3::new(Vector3::one(), Vector3::forward());
	/// let point = ray.get_point(4.3);
	/// assert_eq!(Vector3::new(1.0, 1.0, 5.3), point);
	/// ```
	pub fn get_point(self, distance: f32) -> Vector3 {
		let dir = self.direction * distance;
		
		return self.origin + dir;
	}
	
	/// Gets the closest point on the ray from the given point
	/// - **point**: The point to get the closest point from
	/// 
	/// **Returns**: Returns the closest point from the given point
	/// #### Examples
	/// ```
	/// # use mathx::{Ray3, Vector3};
	/// let ray = Ray3::new(Vector3::one(), Vector3::forward());
	/// let point = ray.closest_point(Vector3::down());
	/// assert_eq!(Vector3::new(1.0, 1.0, 0.0), point);
	/// ```
	pub fn closest_point(self, point: Vector3) -> Vector3 {
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
	/// # use mathx::{Ray3, Vector3};
	/// let ray = Ray3::new(Vector3::forward(), Vector3::forward());
	/// let distance = ray.distance(Vector3::down());
	/// assert_eq!(1.0, distance);
	/// let ray = Ray3::new(Vector3::one(), Vector3::forward());
	/// let distance = ray.distance(Vector3::down());
	/// assert_eq!(2.236068, distance);
	/// ```
	pub fn distance(self, point: Vector3) -> f32 { point.distance(self.closest_point(point)) }
}

impl From<Ray2> for Ray3 {
	fn from(value: Ray2) -> Self {
		Ray3::new(value.origin().to_vector3(), value.direction().to_vector3())
	}
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
