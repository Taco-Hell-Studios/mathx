
use crate::Math;
use crate::Vector3;
use crate::{AddSubArithmetic, MulDivScalar, use_impl_ops, impl_add, impl_sub, impl_mul, impl_div};

/// A 2D vector that holds an x-coordinate and y-coordinate
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
	/// The x coordinate of the vector
	x: f32,
	/// The y coordinate of the vector
	y: f32,
}

// Constructors
impl Vector2 {
	/// Creates a new 2D vector
	/// - **x**: The x coordinate of the vector
	/// - **y**: The y coordinate of the vector
	/// 
	/// **Returns**: Returns a new 2D vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let vector = Vector2::new(1.2, 3.45);
	/// assert_eq!(1.2, vector.x());
	/// assert_eq!(3.45, vector.y());
	/// ```
	pub fn new(x: f32, y: f32) -> Self { Vector2 { x, y } }
	
	/// Creates a new 2D vector from a 3D vector
	/// - **vector**: The 3D vector to convert from
	/// 
	/// **Returns**: Returns a converted 2D vector
	/// #### Examples
	/// ```
	/// # use mathx::{Vector2,Vector3};
	/// let vector3 = Vector3::new(1.2, 3.45, 6.789);
	/// let vector2 = Vector2::from_vector3(vector3);
	/// assert_eq!(1.2, vector2.x());
	/// assert_eq!(3.45, vector2.y());
	/// ```
	pub fn from_vector3(vector: Vector3) -> Self { Vector2::new(vector.x(), vector.y()) }
	
	/// Creates an empty 2D vector: (0, 0)
	/// 
	/// **Returns**: Returns an empty 2D vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let vector = Vector2::zero();
	/// assert_eq!(0.0, vector.x());
	/// assert_eq!(0.0, vector.y());
	/// ```
	pub fn zero() -> Self { Vector2 { x: 0.0, y: 0.0 } }
	
	/// Creates a 2D unit vector that's pointing to the left: (-1, 0)
	/// 
	/// **Returns**: Returns a 2D unit vector that's pointing to the left
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let vector = Vector2::left();
	/// assert_eq!(-1.0, vector.x());
	/// assert_eq!(0.0, vector.y());
	/// ```
	pub fn left() -> Self { Vector2 { x: -1.0, y: 0.0 } }
	
	/// Creates a 2D unit vector that's pointing to the right: (1, 0)
	/// 
	/// **Returns**: Returns a 2D unit vector that's pointing to the right
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let vector = Vector2::right();
	/// assert_eq!(1.0, vector.x());
	/// assert_eq!(0.0, vector.y());
	/// ```
	pub fn right() -> Self { Vector2 { x: 1.0, y: 0.0 } }
	
	/// Creates a 2D unit vector that's pointing up: (0, 1)
	/// 
	/// **Returns**: Returns a 2D unit vector that's pointing up
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let vector = Vector2::up();
	/// assert_eq!(0.0, vector.x());
	/// assert_eq!(1.0, vector.y());
	/// ```
	pub fn up() -> Self { Vector2 { x: 0.0, y: 1.0 } }
	
	/// Creates a 2D unit vector that's pointing down: (0, -1)
	/// 
	/// **Returns**: Returns a 2D unit vector that's pointing down
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let vector = Vector2::down();
	/// assert_eq!(0.0, vector.x());
	/// assert_eq!(-1.0, vector.y());
	/// ```
	pub fn down() -> Self { Vector2 { x: 0.0, y: -1.0 } }
	
	/// Creates a 2D vector that contains 1 in all it's components: (1, 1)
	/// 
	/// **Returns**: Returns a 2D vector that contains 1 in all it's components
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let vector = Vector2::one();
	/// assert_eq!(1.0, vector.x());
	/// assert_eq!(1.0, vector.y());
	/// ```
	pub fn one() -> Self { Vector2 { x: 1.0, y: 1.0 } }
}

// Properties
impl Vector2 {
	// Getters
	/// Gets the x coordinate of the vector
	/// 
	/// **Returns**: Returns the x coordinate of the vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let a = Vector2::new(10.0, 20.0);
	/// assert_eq!(10.0, a.x());
	/// ```
	pub fn x(&self) -> f32 { self.x }
	
	/// Gets the y coordinate of the vector
	/// 
	/// **Returns**: Returns the y coordinate of the vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let a = Vector2::new(10.0, 20.0);
	/// assert_eq!(20.0, a.y());
	/// ```
	pub fn y(&self) -> f32 { self.y }
	
	// Setters
	/// Sets the x coordinate of the vector
	/// - **value**: The value to set the x coordinate of the vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let mut a = Vector2::left();
	/// a.set_x(-100.0);
	/// assert_eq!(-100.0, a.x());
	/// ```
	pub fn set_x(&mut self, value: f32) { self.x = value; }
	
	/// Sets the y coordinate of the vector
	/// - **value**: The value to set the y coordinate of the vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let mut a = Vector2::up();
	/// a.set_y(6.0);
	/// assert_eq!(6.0, a.y());
	/// ```
	pub fn set_y(&mut self, value: f32) { self.y = value; }
	
	/// Gets the magnitude of the vector. This returns the length of the vector
	/// 
	/// **Returns**: Returns the magnitude of the vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let a = Vector2::new(-1.0, 2.0);
	/// assert_eq!(2.236068, a.magnitude());
	/// ```
	pub fn magnitude(&self) -> f32 { Math::sqrt(self.square_magnitude()) }
	
	/// Gets the magnitude squared, avoiding the use of a square root
	/// 
	/// **Returns**: Returns the magnitude of the vector squared
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let a = Vector2::new(-1.0, 2.0);
	/// assert_eq!(5.0, a.square_magnitude());
	/// ```
	pub fn square_magnitude(&self) -> f32 { self.x * self.x + self.y * self.y }
}

// Special Vector Functions
impl Vector2 {
	/// Gets the dot product of between the two vectors.
	/// It can be used to determine the angle between two vectors.
	/// - **rhs**: The other vector to dot product with
	/// 
	/// **Returns**: Returns the dot product
	/// #### Remarks
	/// Using two unit vectors, the maximum range of numbers go from -1 to 1. It scales with
	/// the magnitude of both vectors (multiplying them together `a.magnitude() * b.magnitude()`)
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let a = Vector2::one();
	/// let b = Vector2::new(0.25, 1.1);
	/// let dot = a.dot(b);
	/// assert_eq!(1.35, dot);
	/// ```
	/// Note that if the angle is 90 degrees (PI / 2) then it's going to return 0
	/// ```
	/// # use mathx::Vector2;
	/// let a = Vector2::right();
	/// let b = 2.0 * Vector2::up();
	/// let dot = a.dot(b);
	/// assert_eq!(0.0, dot);
	/// ```
	/// Where as, if the angle is 0 degrees or 180 degrees (PI) then it's going to return 1 and -1 respectively;
	/// given that the two vectors are unit vectors
	/// ```
	/// # use mathx::Vector2;
	/// let a = Vector2::right();
	/// let b = Vector2::left();
	/// let dot_one = a.dot(a);
	/// let dot_negative_one = a.dot(b);
	/// assert_eq!(1.0, dot_one);
	/// assert_eq!(-1.0, dot_negative_one);
	/// ```
	pub fn dot(self, rhs: Vector2) -> f32 {
		self.x * rhs.x + self.y * rhs.y
	}
}

// Conversions
impl Vector2 {
	pub fn to_vector3(self) -> Vector3 { Vector3::new(self.x, self.y, 0.0) }
}

impl From<Vector3> for Vector2 {
	fn from(value: Vector3) -> Self { Vector2::from_vector3(value) }
}

// Equates
impl Eq for Vector2 {}
impl PartialEq for Vector2 {
	fn eq(&self, other: &Self) -> bool {
		Math::approx(self.x, other.x)
		&& Math::approx(self.y, other.y)
	}
}

// Display
#[cfg(not(feature = "no_std"))]
impl std::fmt::Display for Vector2 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!("({}, {})", self.x, self.y))
	}
}

// Arithmetic
impl AddSubArithmetic<Vector2> for Vector2 {
	type Output = Vector2;
	fn add_other(self, rhs: Vector2) -> Self::Output {
		Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
	}
	fn add_assign_other(&mut self, rhs: Vector2) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
	fn subtract_other(self, rhs: Vector2) -> Self::Output {
		Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
	}
	fn subtract_assign_other(&mut self, rhs: Vector2) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl AddSubArithmetic<Vector3> for Vector2 {
	type Output = Vector3;
	
	fn add_other(self, rhs: Vector3) -> Self::Output {
		Vector3::new(self.x + rhs.x(), self.y + rhs.y(), rhs.z())
	}
	fn add_assign_other(&mut self, rhs: Vector3) {
		self.x += rhs.x();
		self.y += rhs.y();
	}
	fn subtract_other(self, rhs: Vector3) -> Self::Output {
		Vector3::new(self.x - rhs.x(), self.y - rhs.y(), -rhs.z())
	}
	fn subtract_assign_other(&mut self, rhs: Vector3) {
		self.x -= rhs.x();
		self.y -= rhs.y();
	}
}

impl MulDivScalar for Vector2 {
	type Output = Vector2;
	fn multiply_scalar(self, rhs: f32) -> Self::Output {
		Vector2 { x: rhs * self.x, y: rhs * self.y }
	}
	fn multiply_assign_scalar(&mut self, rhs: f32) {
		self.x *= rhs;
		self.y *= rhs;
	}
	fn divide_scalar(self, rhs: f32) -> Self::Output {
		if rhs == 0.0 { return Vector2::zero(); }
		Vector2 { x: self.x / rhs, y: self.y / rhs }
	}
	fn divide_assign_scalar(&mut self, rhs: f32) {
		if rhs == 0.0 {
			self.x = 0.0;
			self.y = 0.0;
		}
		else {
			self.x /= rhs;
			self.y /= rhs;
		}
	}
	fn reciprocal_scalar(self, rhs: f32) -> Self::Output {
		Vector2 {
			x: if self.x != 0.0 { rhs / self.x } else { 0.0 },
			y: if self.y != 0.0 { rhs / self.y } else { 0.0 },
		}
	}
}

use_impl_ops!();
impl_add!(Vector2);
impl_add!(Vector2 => Vector3: Vector3);
impl_sub!(Vector2);
impl_sub!(Vector2 => Vector3: Vector3);
impl_mul!(Vector2);
impl_div!(Vector2);