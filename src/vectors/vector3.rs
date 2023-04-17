
use crate::Math;
use crate::Vector2;
use crate::{AddSubArithmetic, MulDivScalar, use_impl_ops, impl_add, impl_sub, impl_mul, impl_div};

/// A 2D vector that holds an x-coordinate, y-coordinate, and z-coordinate
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
	/// - **x**: The x coordinate of the vector
	/// - **y**: The y coordinate of the vector
	/// - **z**: The z coordinate of the vector
	/// 
	/// **Returns**: Returns a new 3D vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let vector = Vector3::new(1.2, 3.45, 6.789);
	/// assert_eq!(1.2, vector.x());
	/// assert_eq!(3.45, vector.y());
	/// assert_eq!(6.789, vector.z());
	/// ```
	pub fn new(x: f32, y: f32, z: f32) -> Self { Vector3 { x, y, z } }
	
	/// Creates a new 3D vector from a 2D vector
	/// - **vector**: The 2D vector to convert from
	/// 
	/// **Returns**: Returns a converted 3D vector
	/// #### Examples
	/// ```
	/// # use mathx::{Vector2,Vector3};
	/// let vector2 = Vector2::new(1.2, 3.45);
	/// let vector3 = Vector3::from_vector2(vector2);
	/// assert_eq!(1.2, vector3.x());
	/// assert_eq!(3.45, vector3.y());
	/// assert_eq!(0.0, vector3.z());
	/// ```
	pub fn from_vector2(vector: Vector2) -> Self { Vector3::new(vector.x(), vector.y(), 0.0) }
	
	/// Creates an empty 3D vector
	/// 
	/// **Returns**: Returns an empty 3D vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let vector = Vector3::zero();
	/// assert_eq!(0.0, vector.x());
	/// assert_eq!(0.0, vector.y());
	/// assert_eq!(0.0, vector.z());
	/// ```
	pub fn zero() -> Self { Vector3 { x: 0.0, y: 0.0, z: 0.0 } }
	
	/// Creates a 3D unit vector that's pointing to the lefT: (-1, 0, 0)
	/// 
	/// **Returns**: Returns a 3D unit vector that's pointing to the left
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let vector = Vector3::left();
	/// assert_eq!(-1.0, vector.x());
	/// assert_eq!(0.0, vector.y());
	/// assert_eq!(0.0, vector.z());
	/// ```
	pub fn left() -> Self { Vector3 { x: -1.0, y: 0.0, z: 0.0 } }
	
	/// Creates a 3D unit vector that's pointing to the right: (1, 0, 0)
	/// 
	/// **Returns**: Returns a 3D unit vector that's pointing to the left
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let vector = Vector3::right();
	/// assert_eq!(1.0, vector.x());
	/// assert_eq!(0.0, vector.y());
	/// assert_eq!(0.0, vector.z());
	/// ```
	pub fn right() -> Self { Vector3 { x: 1.0, y: 0.0, z: 0.0 } }
	
	/// Creates a 3D unit vector that's pointing up: (0, 1, 0)
	/// 
	/// **Returns**: Returns a 3D unit vector that's pointing up
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let vector = Vector3::up();
	/// assert_eq!(0.0, vector.x());
	/// assert_eq!(1.0, vector.y());
	/// assert_eq!(0.0, vector.z());
	/// ```
	pub fn up() -> Self { Vector3 { x: 0.0, y: 1.0, z: 0.0 } }
	
	/// Creates a 3D unit vector that's pointing down: (0, -1, 0)
	/// 
	/// **Returns**: Returns a 3D unit vector that's pointing down
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let vector = Vector3::down();
	/// assert_eq!(0.0, vector.x());
	/// assert_eq!(-1.0, vector.y());
	/// assert_eq!(0.0, vector.z());
	/// ```
	pub fn down() -> Self { Vector3 { x: 0.0, y: -1.0, z: 0.0 } }
	
	/// Creates a 3D unit vector that's pointing forward: (0, 0, 1)
	/// 
	/// **Returns**: Returns a 3D unit vector that's pointing forward
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let vector = Vector3::forward();
	/// assert_eq!(0.0, vector.x());
	/// assert_eq!(0.0, vector.y());
	/// assert_eq!(1.0, vector.z());
	/// ```
	pub fn forward() -> Self { Vector3 { x: 0.0, y: 0.0, z: 1.0 } }
	
	/// Creates a 3D unit vector that's pointing backwards: (0, 0, -1)
	/// 
	/// **Returns**: Returns a 3D unit vector that's pointing backwards
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let vector = Vector3::back();
	/// assert_eq!(0.0, vector.x());
	/// assert_eq!(0.0, vector.y());
	/// assert_eq!(-1.0, vector.z());
	/// ```
	pub fn back() -> Self { Vector3 { x: 0.0, y: 0.0, z: -1.0 } }
	
	/// Creates a 3D vector that contains 1 in all it's components: (1, 1, 1)
	/// 
	/// **Returns**: Returns a 3D vector that contains 1 in all it's components
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let vector = Vector3::one();
	/// assert_eq!(1.0, vector.x());
	/// assert_eq!(1.0, vector.y());
	/// assert_eq!(1.0, vector.z());
	/// ```
	pub fn one() -> Self { Vector3 { x: 1.0, y: 1.0, z: 1.0 } }
}

// Properties
impl Vector3 {
	/// Gets the x coordinate of the vector
	/// 
	/// **Returns**: Returns the x coordinate of the vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(10.0, 20.0, 30.0);
	/// assert_eq!(10.0, a.x());
	/// ```
	pub fn x(&self) -> f32 { self.x }
	
	/// Gets the y coordinate of the vector
	/// 
	/// **Returns**: Returns the y coordinate of the vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(10.0, 20.0, 30.0);
	/// assert_eq!(20.0, a.y());
	/// ```
	pub fn y(&self) -> f32 { self.y }
	
	/// Gets the z coordinate of the vector
	/// 
	/// **Returns**: Returns the z coordinate of the vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(10.0, 20.0, 30.0);
	/// assert_eq!(30.0, a.z());
	/// ```
	pub fn z(&self) -> f32 { self.z }
	
	/// Sets the x coordinate of the vector
	/// - **value**: The value to set the x coordinate of the vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let mut a = Vector3::left();
	/// a.set_x(-100.0);
	/// assert_eq!(-100.0, a.x());
	/// ```
	pub fn set_x(&mut self, value: f32) { self.x = value; }
	
	/// Sets the y coordinate of the vector
	/// - **value**: The value to set the y coordinate of the vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let mut a = Vector3::up();
	/// a.set_y(6.0);
	/// assert_eq!(6.0, a.y());
	/// ```
	pub fn set_y(&mut self, value: f32) { self.y = value; }
	
	/// Sets the z coordinate of the vector
	/// - **value**: The value to set the z coordinate of the vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let mut a = Vector3::forward();
	/// a.set_z(10.0);
	/// assert_eq!(10.0, a.z());
	/// ```
	pub fn set_z(&mut self, value: f32) { self.z = value; }
	
	/// Gets the magnitude of the vector. This returns the length of the vector
	/// 
	/// **Returns**: Returns the magnitude of the vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(-1.0, 2.0, -2.0);
	/// assert_eq!(3.0, a.magnitude());
	/// ```
	pub fn magnitude(&self) -> f32 { Math::sqrt(self.square_magnitude()) }
	
	/// Gets the magnitude squared, avoiding the use of a square root
	/// 
	/// **Returns**: Returns the magnitude of the vector squared
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(-1.0, 2.0, 2.0);
	/// assert_eq!(9.0, a.square_magnitude());
	/// ```
	pub fn square_magnitude(&self) -> f32 { self.x * self.x + self.y * self.y + self.z * self.z }
}

// Conversions
impl Vector3 {
	pub fn to_vector2(self) -> Vector2 { Vector2::new(self.x, self.y) }
}

impl From<Vector2> for Vector3 {
	fn from(value: Vector2) -> Self { Vector3::from_vector2(value) }
}

unsafe impl Send for Vector3 {}
unsafe impl Sync for Vector3 {}

// Equates
impl Eq for Vector3 {}
impl PartialEq for Vector3 {
	fn eq(&self, other: &Self) -> bool {
		Math::approx(self.x, other.x)
		&& Math::approx(self.y, other.y)
		&& Math::approx(self.z, other.z)
	}
}

// Display
#[cfg(not(feature = "no_std"))]
impl std::fmt::Display for Vector3 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
