
use crate::Vector2;
use crate::{AddSubArithmetic, MulDivScalar, use_impl_ops, impl_add, impl_sub, impl_mul, impl_div};
use std::fmt::{Display, Formatter};

/// A 2D vector that holds an x-coordinate, y-coordinate, and z-coordinate
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
	/// The x coordinate of the vector
	x: f32,
	/// The y coordinate of the vector
	y: f32,
	/// The z coordinate of the vector
	z: f32,
}

// Constructors
impl Vector3 {
	/// Creates a new 3D vector
	pub fn new(x: f32, y: f32, z: f32) -> Self { Vector3 { x, y, z } }
	
	/// Creates an empty 3D vector
	pub fn zero() -> Self { Vector3 { x: 0.0, y: 0.0, z: 0.0 } }
	
	/// Creates a 3D unit vector that's pointing to the lefT: (-1, 0, 0)
	pub fn left() -> Self { Vector3 { x: -1.0, y: 0.0, z: 0.0 } }
	
	/// Creates a 3D unit vector that's pointing to the right: (1, 0, 0)
	pub fn right() -> Self { Vector3 { x: 1.0, y: 0.0, z: 0.0 } }
	
	/// Creates a 3D unit vector that's pointing up: (0, 1, 0)
	pub fn up() -> Self { Vector3 { x: 0.0, y: 1.0, z: 0.0 } }
	
	/// Creates a 3D unit vector that's pointing down: (0, -1, 0)
	pub fn down() -> Self { Vector3 { x: 0.0, y: -1.0, z: 0.0 } }
	
	/// Creates a 3D unit vector that's pointing forward: (0, 0, 1)
	pub fn forward() -> Self { Vector3 { x: 0.0, y: 0.0, z: 1.0 } }
	
	/// Creates a 3D unit vector that's pointing backwards: (0, 0, -1)
	pub fn back() -> Self { Vector3 { x: 0.0, y: 0.0, z: -1.0 } }
	
	/// Creates a 3D vector that contains 1 in all it's components: (1, 1, 1)
	pub fn one() -> Self { Vector3 { x: 1.0, y: 1.0, z: 1.0 } }
}

// Properties
impl Vector3 {
	/// Gets the x coordinate of the vector
	/// #### Example
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(10.0, 20.0, 30.0);
	/// assert_eq!(10.0, a.x());
	/// ```
	pub fn x(&self) -> f32 { self.x }
	
	/// Gets the y coordinate of the vector
	/// #### Example
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(10.0, 20.0, 30.0);
	/// assert_eq!(20.0, a.y());
	/// ```
	pub fn y(&self) -> f32 { self.y }
	
	/// Gets the z coordinate of the vector
	/// #### Example
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(10.0, 20.0, 30.0);
	/// assert_eq!(30.0, a.z());
	/// ```
	pub fn z(&self) -> f32 { self.z }
	
	/// Sets the x coordinate of the vector
	/// - **value**: The value to set the x coordinate of the vector
	/// #### Example
	/// ```
	/// # use mathx::Vector3;
	/// let mut a = Vector3::left();
	/// a.set_x(-100.0);
	/// assert_eq!(-100.0, a.x());
	/// ```
	pub fn set_x(&mut self, value: f32) { self.x = value; }
	
	/// Sets the y coordinate of the vector
	/// - **value**: The value to set the y coordinate of the vector
	/// #### Example
	/// ```
	/// # use mathx::Vector3;
	/// let mut a = Vector3::up();
	/// a.set_y(6.0);
	/// assert_eq!(6.0, a.y());
	/// ```
	pub fn set_y(&mut self, value: f32) { self.y = value; }
	
	/// Sets the z coordinate of the vector
	/// - **value**: The value to set the z coordinate of the vector
	/// #### Example
	/// ```
	/// # use mathx::Vector3;
	/// let mut a = Vector3::forward();
	/// a.set_z(10.0);
	/// assert_eq!(10.0, a.z());
	/// ```
	pub fn set_z(&mut self, value: f32) { self.z = value; }
	
	/// Gets the magnitude of the vector. This returns the length of the vector
	/// #### Example
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(-1.0, 2.0, -2.0);
	/// assert_eq!(3.0, a.magnitude());
	/// ```
	pub fn magnitude(&self) -> f32 { self.square_magnitude().sqrt() }
	
	/// Gets the magnitude squared, avoiding the use of a square root
	/// #### Example
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(-1.0, 2.0, 2.0);
	/// assert_eq!(9.0, a.square_magnitude());
	/// ```
	pub fn square_magnitude(&self) -> f32 { self.x * self.x + self.y * self.y + self.z * self.z }
}

// Display
impl Display for Vector3 {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!("({}, {}, {})", self.x, self.y, self.z))
	}
}

// Arithmetic
impl AddSubArithmetic<Vector3> for Vector3 {
	type Output = Vector3;
	fn add_other(self, rhs: Vector3) -> Self::Output {
		Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
	}
	fn add_assign_other(&mut self, rhs: Vector3) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
	fn subtract_other(self, rhs: Vector3) -> Self::Output {
		Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
	}
	fn subtract_assign_other(&mut self, rhs: Vector3) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
	}
}

impl AddSubArithmetic<Vector2> for Vector3 {
	type Output = Vector3;
	fn add_other(self, rhs: Vector2) -> Self::Output {
		Vector3 { x: self.x + rhs.x(), y: self.y + rhs.y(), z: self.z }
	}
	fn add_assign_other(&mut self, rhs: Vector2) {
		self.x += rhs.x();
		self.y += rhs.y();
	}
	fn subtract_other(self, rhs: Vector2) -> Self::Output {
		Vector3 { x: self.x - rhs.x(), y: self.y - rhs.y(), z: self.z }
	}
	fn subtract_assign_other(&mut self, rhs: Vector2) {
		self.x -= rhs.x();
		self.y -= rhs.y();
	}
}

impl MulDivScalar for Vector3 {
	type Output = Vector3;
	fn multiply_scalar(self, rhs: f32) -> Self::Output {
		Vector3 { x: rhs * self.x, y: rhs * self.y, z: rhs * self.z }
	}
	fn multiply_assign_scalar(&mut self, rhs: f32) {
		self.x *= rhs;
		self.y *= rhs;
	}
	fn divide_scalar(self, rhs: f32) -> Self::Output {
		if rhs == 0.0 { return Vector3::zero(); }
		Vector3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
	}
	fn divide_assign_scalar(&mut self, rhs: f32) {
		if rhs == 0.0 {
			self.x = 0.0;
			self.y = 0.0;
			self.z = 0.0;
		}
		else {
			self.x /= rhs;
			self.y /= rhs;
			self.z /= rhs;
		}
	}
	fn reciprocal_scalar(self, rhs: f32) -> Self::Output {
		Vector3 {
			x: if self.x != 0.0 { rhs / self.x } else { 0.0 },
			y: if self.y != 0.0 { rhs / self.y } else { 0.0 },
			z: if self.z != 0.0 { rhs / self.z } else { 0.0 },
		}
	}
}

use_impl_ops!();
impl_add!(Vector3);
impl_add!(Vector3 => Vector2: Vector3);
impl_sub!(Vector3);
impl_sub!(Vector3 => Vector2: Vector3);
impl_mul!(Vector3);
impl_div!(Vector3);