
use core::ops::Neg;

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

/// Constructors
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
	
	/// Creates a 2D vector from a single angle (heading)
	/// - **angle**: The angle in radians to create the 2D vector from
	/// 
	/// **Returns**: Returns a 2D vector from the single angle
	/// #### Examples
	/// ```
	/// # use mathx::{Vector2,Math,assert_range};
	/// let vector = Vector2::from_heading(Math::PI_OVER_4);
	/// assert_range!(0.7071068, vector.x());
	/// assert_range!(0.7071068, vector.y());
	/// let vector = Vector2::from_heading(4.0);
	/// assert_range!(-0.653643620864, vector.x());
	/// assert_range!(-0.756802495308, vector.y());
	/// ```
	pub fn from_heading(angle: f32) -> Self {
		let (sin, cos) = Math::sin_cos(angle);
		
		Vector2::new(cos, sin)
	}
	
	/// Creates a 2D vector from a single angle (heading)
	/// - **angle**: The angle in degrees to create the 2D vector from
	/// 
	/// **Returns**: Returns a 2D vector from the single angle
	/// #### Examples
	/// ```
	/// # use mathx::{Vector2,Math,assert_range};
	/// let vector = Vector2::from_heading_deg(45.0);
	/// assert_range!(0.7071068, vector.x());
	/// assert_range!(0.7071068, vector.y());
	/// let vector = Vector2::from_heading_deg(229.183118052);
	/// assert_range!(-0.653643620864, vector.x());
	/// assert_range!(-0.756802495308, vector.y());
	/// ```
	pub fn from_heading_deg(angle: f32) -> Self {
		let (sin, cos) = Math::sin_cos_deg(angle);
		
		Vector2::new(cos, sin)
	}
}

/// Properties
impl Vector2 {
	/// Gets the x coordinate of the vector
	/// 
	/// **Returns**: Returns the x coordinate of the vector
	pub fn x(&self) -> f32 { self.x }
	
	/// Sets the x coordinate of the vector
	/// - **value**: The value to set the x coordinate of the vector
	pub fn set_x(&mut self, value: f32) { self.x = value; }
	
	/// Gets the y coordinate of the vector
	/// 
	/// **Returns**: Returns the y coordinate of the vector
	pub fn y(&self) -> f32 { self.y }
	
	/// Sets the y coordinate of the vector
	/// - **value**: The value to set the y coordinate of the vector
	pub fn set_y(&mut self, value: f32) { self.y = value; }
	
	/// Get the heading from the vector in radians
	/// 
	/// **Returns**: Returns the heading from the vector in radians
	/// #### Examples
	/// ```
	/// # use mathx::{Math,Vector2,assert_range};
	/// let heading = Vector2::one().heading();
	/// assert_range!(Math::PI_OVER_4, heading);
	/// ```
	pub fn heading(&self) -> f32 { Math::atan2(self.y, self.x) }
	
	/// Sets the heading for the vector in radians
	/// - **angle**: The angle to set the heading of the vector for in radians
	/// #### Examples
	/// ```
	/// # use mathx::{Math,Vector2,assert_range};
	/// let mut vector = Vector2::zero();
	/// vector.set_heading(Math::PI_OVER_4);
	/// assert_range!(0.70710678118, vector.x());
	/// assert_range!(0.70710678118, vector.y());
	/// ```
	pub fn set_heading(&mut self, angle: f32) {
		let vector = Vector2::from_heading(angle);
		
		self.x = vector.x;
		self.y = vector.y;
	}
	
	/// Get the heading from the vector in degrees
	/// 
	/// **Returns**: Returns the heading from the vector in degrees
	/// #### Examples
	/// ```
	/// # use mathx::{Math,Vector2,assert_range};
	/// let heading = Vector2::one().heading_deg();
	/// assert_range!(45.0, heading, 0.001);
	/// ```
	pub fn heading_deg(&self) -> f32 { Math::rad2deg(self.heading()) }
	
	/// Sets the heading for the vector in degrees
	/// - **angle**: The angle to set the heading of the vector for in degrees
	/// 
	/// #### Examples
	/// ```
	/// # use mathx::{Math,Vector2,assert_range};
	/// let mut vector = Vector2::zero();
	/// vector.set_heading_deg(45.0);
	/// assert_range!(0.70710678118, vector.x());
	/// assert_range!(0.70710678118, vector.y());
	/// ```
	pub fn set_heading_deg(&mut self, angle: f32) { self.set_heading(Math::deg2rad(angle)) }
	
	/// Gets the magnitude of the vector. This returns the length of the vector
	/// 
	/// **Returns**: Returns the magnitude of the vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let a = Vector2::new(-1.0, 2.0);
	/// assert_eq!(2.236068, a.magnitude());
	/// ```
	pub fn magnitude(&self) -> f32 {
		let magnitude = self.square_magnitude();
		
		if magnitude == 0.0 || magnitude == 1.0 {
			return magnitude;
		}
		
		return Math::sqrt(magnitude);
	}
	
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

/// Public Methods
impl Vector2 {
	/// Gets the angle between the two vectors in radians
	/// - **rhs**: The other vector to get the angle from
	/// 
	/// **Returns**: Returns the angle between the two vectors in radians
	/// #### Examples
	/// ```
	/// # use mathx::{Vector2,Math,assert_range};
	/// let a = Vector2::new(0.25, -0.5);
	/// let b = Vector2::new(2.0, 0.5);
	/// assert_range!(1.35212751547, a.angle_between(b));
	/// ```
	pub fn angle_between(self, rhs: Vector2) -> f32 {
		let value = Math::sqrt(self.square_magnitude() * rhs.square_magnitude());
		
		if value < 0.0000000001 { return 0.0; }
		else { return Math::acos(Math::clamp((self * rhs) / value, -1.0, 1.0)); }
	}
	
	/// Gets the angle between the two vectors in degrees
	/// - **rhs**: The other vector to get the angle from
	/// 
	/// **Returns**: Returns the angle between the two vectors in degrees
	/// #### Examples
	/// ```
	/// # use mathx::{Vector2,Math,assert_range};
	/// let a = Vector2::new(0.25, -0.5);
	/// let b = Vector2::new(2.0, 0.5);
	/// assert_range!(77.4712, a.angle_between_deg(b), 0.01);
	/// ```
	pub fn angle_between_deg(self, rhs: Vector2) -> f32 { return Math::rad2deg(self.angle_between(rhs)); }
	
	/// Gets the distance between the two vectors
	/// - **rhs**: The other vector to get the distance between
	/// 
	/// **Returns**: Returns the distance between the two vectors
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let a = Vector2::new(0.25, -0.5);
	/// let b = Vector2::new(2.0, 0.5);
	/// assert_eq!(2.0155644, a.distance(b));
	/// ```
	pub fn distance(self, rhs: Vector2) -> f32 { (rhs - self).magnitude() }
	
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
	
	/// Linearly interpolates between the this and the other vector
	/// - **rhs**: The other vector to end from
	/// - **t**: The ratio value to interpolate between both vectors. Clamped between 0.0 and 1.0
	/// 
	/// **Returns**: Returns the interpolated vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let a = Vector2::new(0.0, -10.0);
	/// let b = Vector2::new(1.0, -4.0);
	/// let expected = Vector2::new(0.7, -5.8);
	/// assert_eq!(expected, a.lerp_unclamped(b, 0.7));
	/// ```
	pub fn lerp(self, rhs: Vector2, t: f32) -> Self { self.lerp_unclamped(rhs, t.clamp(0.0, 1.0)) }
	
	/// Linearly interpolates between the this and the other vector (not clamped)
	/// - **rhs**: The other vector to end from
	/// - **t**: The ratio value to interpolate between both vectors
	/// 
	/// **Returns**: Returns the interpolated vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let a = Vector2::new(0.0, -10.0);
	/// let b = Vector2::new(1.0, -4.0);
	/// let expected = Vector2::new(0.7, -5.8);
	/// assert_eq!(expected, a.lerp_unclamped(b, 0.7));
	/// ```
	pub fn lerp_unclamped(self, rhs: Vector2, t: f32) -> Self {
		Vector2::new(
			Math::lerp_unclamped(self.x, rhs.x, t),
			Math::lerp_unclamped(self.y, rhs.y, t)
		)
	}
	
	/// Moves this vector towards the target vector, it will never move past the target
	/// - **target**: The target vector to move towards
	/// - **delta**: The delta distance to try and move with, defines the maximum distance moved
	/// 
	/// **Returns**: Returns the vector that is closer towards the target
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let a = Vector2::new(0.25, -0.5);
	/// let b = Vector2::new(2.0, 0.5);
	/// let expected = Vector2::new(0.42364863, -0.4007722);
	/// assert_eq!(expected, a.move_towards(b, 0.2));
	/// assert_eq!(b, a.move_towards(b, 20.0));
	/// ```
	pub fn move_towards(self, target: Vector2, delta: f32) -> Self {
		let dir = target - self;
		let sq_magnitude = dir.square_magnitude();
		if sq_magnitude == 0.0 || (delta >= 0.0 && sq_magnitude <= delta * delta) {
			return target;
		}
		
		let diff = delta / Math::sqrt(sq_magnitude);
		
		return diff * dir + self;
	}
	
	/// Normalizes the vector
	/// 
	/// **Returns**: Returns the unit vector version of this vector
	/// #### Examples
	/// ```
	/// # use mathx::{Vector2,Math,assert_range};
	/// let vector = Vector2::one().normalize();
	/// assert_range!(0.70710678118, vector.x());
	/// assert_range!(0.70710678118, vector.y());
	/// let vector = Vector2::new(-0.1, 1.0).normalize();
	/// assert_range!(-0.09950372, vector.x());
	/// assert_range!(0.99503714, vector.y());
	/// ```
	pub fn normalize(self) -> Self { self / self.magnitude() }
	
	/// Creates a perpendicular 2D vector
	/// 
	/// **Returns**: Returns a perpendicular 2D vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let vector = Vector2::new(1.0, 2.0);
	/// let perpendicular = vector.perpendicular();
	/// assert_eq!(0.0, vector * perpendicular);
	/// ```
	pub fn perpendicular(self) -> Self { Vector2::new(self.y, -self.x) }
	
	/// Projects this vector onto the given vector
	/// - **rhs**: The vector to project onto
	/// 
	/// **Returns**: Returns the projected vector
	/// #### Examples
	/// ```
	/// # use mathx::{Vector2,Math,assert_range};
	/// let a = Vector2::new(1.0, 2.0);
	/// let b = Vector2::new(3.0, 4.0);
	/// let expected = Vector2::new(1.32, 1.76);
	/// assert_range!(expected.x(), a.project(b).x());
	/// assert_range!(expected.y(), a.project(b).y());
	/// ```
	pub fn project(self, rhs: Vector2) -> Self {
		let top = self * rhs;
		let bottom = rhs.square_magnitude();
		
		return (top / bottom) * rhs;
	}
	
	/// Rejects this vector from the given vector
	/// - **rhs**: The vector to reject from
	/// 
	/// **Returns**: Returns the rejected vector
	/// #### Examples
	/// ```
	/// # use mathx::{Vector2,Math,assert_range};
	/// let a = Vector2::new(1.0, 2.0);
	/// let b = Vector2::new(3.0, 4.0);
	/// let expected = Vector2::new(-0.32, 0.24);
	/// assert_range!(expected.x(), a.reject(b).x());
	/// assert_range!(expected.y(), a.reject(b).y());
	/// ```
	pub fn reject(self, rhs: Vector2) -> Self {
		self - self.project(rhs)
	}
	
	/// Reflects this vector using a normal vector
	/// - **normal**: The normal vector to reflect off of
	/// 
	/// **Returns**: Returns the reflected vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let direction = Vector2::new(1.0, 0.0);
	/// let normal = Vector2::new(1.0, 1.0);
	/// let expected = Vector2::new(-1.0, -2.0);
	/// assert_eq!(expected, direction.reflect(normal));
	/// let direction = Vector2::new(0.25, -0.5);
	/// let normal = Vector2::new(1.0, 0.5);
	/// let expected = Vector2::new(0.25, -0.5);
	/// assert_eq!(expected, direction.reflect(normal));
	/// ```
	pub fn reflect(self, normal: Vector2) -> Self {
		let dot = -2.0 * (self * normal);
		
		return dot * normal + self;
	}
	
	/// Scales the vector using another vector, multiplying everything component-wise
	/// - **rhs**: The other vector to scale with
	/// 
	/// **Returns**: Returns the scaled vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector2;
	/// let a = Vector2::new(0.25, -0.5);
	/// let b = Vector2::new(2.0, 0.5);
	/// let expected = Vector2::new(0.5, -0.25);
	/// assert_eq!(expected, a.scale(b));
	/// ```
	pub fn scale(self, rhs: Vector2) -> Self {
		Vector2::new(
			self.x * rhs.x,
			self.y * rhs.y
		)
	}
	
	/// Gets the signed angle between the two vectors using an axis in radians
	/// - **rhs**: The other vector to get the angle from
	/// 
	/// **Returns**: Returns the signed angle between the two vectors using an axis in radians
	/// #### Examples
	/// ```
	/// # use mathx::{Vector2,Math,assert_range};
	/// let a = Vector2::new(0.25, -0.5);
	/// let b = Vector2::new(-2.0, 0.5);
	/// assert_range!(-2.27942269238, a.signed_angle_between(b));
	/// ```
	pub fn signed_angle_between(self, rhs: Vector2) -> f32 {
		let angle = self.angle_between(rhs);
		let sign = Math::sign(self * rhs.perpendicular());
		
		return sign * angle;
	}
	
	/// Gets the signed angle between the two vectors using an axis in degrees
	/// - **rhs**: The other vector to get the angle from
	/// 
	/// **Returns**: Returns the signed angle between the two vectors using an axis in degrees
	/// #### Examples
	/// ```
	/// # use mathx::{Vector2,Math,assert_range};
	/// let a = Vector2::new(0.25, -0.5);
	/// let b = Vector2::new(-2.0, 0.5);
	/// assert_range!(-130.6013, a.signed_angle_between_deg(b), 0.01);
	/// ```
	pub fn signed_angle_between_deg(self, rhs: Vector2) -> f32 { Math::rad2deg(self.signed_angle_between(rhs)) }
	
}

/// Conversions
impl Vector2 {
	pub fn to_vector3(self) -> Vector3 { Vector3::new(self.x, self.y, 0.0) }
}

impl From<Vector3> for Vector2 {
	fn from(value: Vector3) -> Self { Vector2::from_vector3(value) }
}

unsafe impl Send for Vector2 {}
unsafe impl Sync for Vector2 {}

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

impl Neg for Vector2 {
	type Output = Vector2;
	fn neg(self) -> Self::Output { Vector2::new(-self.x, -self.y) }
}

use_impl_ops!();
impl_add!(Vector2);
impl_add!(Vector2 => Vector3: Vector3);
impl_sub!(Vector2);
impl_sub!(Vector2 => Vector3: Vector3);
impl_mul!(Vector2);
impl_mul!(Vector2, Vector2 => f32: dot);
impl_div!(Vector2);