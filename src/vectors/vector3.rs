
use core::ops::Neg;

use crate::Math;
use crate::Vector2;
use crate::{AddSubArithmetic, MulDivScalar, use_impl_ops, impl_add, impl_sub, impl_mul, impl_div};

/// A 3D vector that holds an x-coordinate, y-coordinate, and z-coordinate
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
/// The following are constructors for creating a new 3D vector
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
	
	/// Creates a 3D vector from two given angles
	/// - **theta**: The first angle to create the vector from
	/// - **phi**: The second angle to create the vector from
	/// 
	/// **Returns**: Returns a 3D vector from the two angles
	/// #### Examples
	/// ```
	/// # use mathx::{Math,Vector3,assert_range};
	/// let vector = Vector3::from_angles(Math::PI_OVER_4, Math::PI_OVER_4);
	/// let expected = Vector3::new(0.5, 0.5, 0.707106781187);
	/// assert_range!(expected.x(), vector.x());
	/// assert_range!(expected.y(), vector.y());
	/// assert_range!(expected.z(), vector.z());
	/// let vector = Vector3::from_angles(-2.21656815003, 2.21656815003);
	/// let expected = Vector3::new(0.3621814, 0.4806309, 0.7986355);
	/// assert_range!(expected.x(), vector.x());
	/// assert_range!(expected.y(), vector.y());
	/// assert_range!(expected.z(), vector.z());
	/// ```
	pub fn from_angles(theta: f32, phi: f32) -> Self {
		let (sin_theta, cos_theta) = Math::sin_cos(theta);
		let (sin_phi, cos_phi) = Math::sin_cos(phi);
		
		Vector3::new(
			cos_phi * cos_theta,
			cos_phi * sin_theta,
			sin_phi
		)
	}
	
	/// Creates a 3D vector from two given angles
	/// - **theta**: The first angle to create the vector from
	/// - **phi**: The second angle to create the vector from
	/// 
	/// **Returns**: Returns a 3D vector from the two angles
	/// #### Examples
	/// ```
	/// # use mathx::{Math,Vector3,assert_range};
	/// let vector = Vector3::from_angles_deg(45.0, 45.0);
	/// let expected = Vector3::new(0.5, 0.5, 0.707106781187);
	/// assert_range!(expected.x(), vector.x());
	/// assert_range!(expected.y(), vector.y());
	/// assert_range!(expected.z(), vector.z());
	/// let vector = Vector3::from_angles_deg(-127.0, 127.0);
	/// let expected = Vector3::new(0.3621814, 0.4806309, 0.7986355);
	/// assert_range!(expected.x(), vector.x());
	/// assert_range!(expected.y(), vector.y());
	/// assert_range!(expected.z(), vector.z());
	/// ```
	pub fn from_angles_deg(theta: f32, phi: f32) -> Self {
		Vector3::from_angles(Math::deg2rad(theta), Math::deg2rad(phi))
	}
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

// Public Methods
impl Vector3 {
	pub fn rotate_towards(self, target: Vector3, radians_delta: f32, magnitude_delta: f32) -> Self {
		todo!()
	}
	pub fn slerp(self, rhs: Vector3, t: f32) -> Self {
		todo!()
	}
	pub fn slerp_unclamped(self, rhs: Vector3, t: f32) -> Self {
		todo!()
	}
	
	
	/// Gets the angle between the two vectors in radians
	/// - **rhs**: The other vector to get the angle from
	/// 
	/// **Returns**: Returns the angle between the two vectors in radians
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3,Math,assert_range};
	/// let a = Vector3::new(0.25, -0.5, 1.25);
	/// let b = Vector3::new(2.0, 0.5, -1.0);
	/// assert_range!(1.89518322157, a.angle_between(b));
	/// ```
	pub fn angle_between(self, rhs: Vector3) -> f32 {
		let value = Math::sqrt(self.square_magnitude() * rhs.square_magnitude());;
		
		if value < 0.0000000001 { return 0.0; }
		else { return Math::acos(Math::clamp((self * rhs) / value, -1.0, 1.0)); }
	}
	
	/// Gets the angle between the two vectors in degrees
	/// - **rhs**: The other vector to get the angle from
	/// 
	/// **Returns**: Returns the angle between the two vectors in degrees
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3,Math,assert_range};
	/// let a = Vector3::new(0.25, -0.5, 1.25);
	/// let b = Vector3::new(2.0, 0.5, -1.0);
	/// assert_range!(108.586, a.angle_between_deg(b), 0.01);
	/// ```
	pub fn angle_between_deg(self, rhs: Vector3) -> f32 { return Math::rad2deg(self.angle_between(rhs)); }
	
	/// Performs a cross product and creates a 3D vector that is orthogonal to both vectors provided
	/// - **rhs**: The other vector to cross product
	/// 
	/// 
	/// **Returns**: Returns the vector that is orthogonal to both vectors
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(1.0, 2.0, 3.0);
	/// let b = Vector3::new(4.0, 5.0, 6.0);
	/// let expected = Vector3::new(-3.0, 6.0, -3.0);
	/// assert_eq!(expected, a.cross(b));
	/// assert_eq!(Vector3::zero(), a.cross(a));
	/// ```
	pub fn cross(self, rhs: Vector3) -> Self {
		Vector3::new(
			self.y * rhs.z - self.z * rhs.y,
			self.z * rhs.x - self.x * rhs.z,
			self.x * rhs.y - self.y * rhs.x
		)
	}
	
	/// Gets the distance between the two vectors
	/// - **rhs**: The other vector to get the distance between
	/// 
	/// **Returns**: Returns the distance between the two vectors
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(0.25, -0.5, 1.25);
	/// let b = Vector3::new(2.0, 0.5, -1.0);
	/// assert_eq!(3.0207615, a.distance(b));
	/// ```
	pub fn distance(self, rhs: Vector3) -> f32 { (rhs - self).magnitude() }
	
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
	/// # use mathx::Vector3;
	/// let a = Vector3::one();
	/// let b = Vector3::new(0.25, 1.1, -4.1);
	/// let dot = a.dot(b);
	/// assert_eq!(-2.75, dot);
	/// ```
	/// Note that if the angle is 90 degrees (PI / 2) then it's going to return 0
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::right();
	/// let b = 2.0 * Vector3::up();
	/// let dot = a.dot(b);
	/// assert_eq!(0.0, dot);
	/// ```
	/// Where as, if the angle is 0 degrees or 180 degrees (PI) then it's going to return 1 and -1 respectively;
	/// given that the two vectors are unit vectors
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::right();
	/// let b = Vector3::left();
	/// let dot_one = a.dot(a);
	/// let dot_negative_one = a.dot(b);
	/// assert_eq!(1.0, dot_one);
	/// assert_eq!(-1.0, dot_negative_one);
	/// ```
	pub fn dot(self, rhs: Vector3) -> f32 {
		self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}
	
	/// Linearly interpolates between the this and the other vector
	/// - **rhs**: The other vector to end from
	/// - **t**: The ratio value to interpolate between both vectors. Clamped between 0.0 and 1.0
	/// 
	/// **Returns**: Returns the interpolated vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(0.0, 4.0, -10.0);
	/// let b = Vector3::new(1.0, 10.0, -4.0);
	/// let expected = Vector3::new(0.7, 8.2, -5.8);
	/// assert_eq!(expected, a.lerp(b, 0.7));
	/// ```
	pub fn lerp(self, rhs: Vector3, t: f32) -> Self { self.lerp_unclamped(rhs, t.clamp(0.0, 1.0)) }
	
	/// Linearly interpolates between the this and the other vector (not clamped)
	/// - **rhs**: The other vector to end from
	/// - **t**: The ratio value to interpolate between both vectors
	/// 
	/// **Returns**: Returns the interpolated vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(0.0, 4.0, -10.0);
	/// let b = Vector3::new(1.0, 10.0, -4.0);
	/// let expected = Vector3::new(0.7, 8.2, -5.8);
	/// assert_eq!(expected, a.lerp_unclamped(b, 0.7));
	/// ```
	pub fn lerp_unclamped(self, rhs: Vector3, t: f32) -> Self {
		Vector3::new(
			Math::lerp_unclamped(self.x, rhs.x, t),
			Math::lerp_unclamped(self.y, rhs.y, t),
			Math::lerp_unclamped(self.z, rhs.z, t)
		)
	}
	
	/// Moves this vector towards the target vector, it will never move past the target
	/// - **target**: The target vector to move towards
	/// - **delta**: The delta distance to try and move with, defines the maximum distance moved
	/// 
	/// **Returns**: Returns the vector that is closer towards the target
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(0.25, -0.5, 1.25);
	/// let b = Vector3::new(2.0, 0.5, -1.0);
	/// let expected = Vector3::new(0.3658648, -0.4337915, 1.101031);
	/// assert_eq!(expected, a.move_towards(b, 0.2));
	/// assert_eq!(b, a.move_towards(b, 20.0));
	/// ```
	pub fn move_towards(self, target: Vector3, delta: f32) -> Self {
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
	/// # use mathx::{Vector3,Math,assert_range};
	/// let vector = Vector3::one().normalize();
	/// assert_range!(0.5773503, vector.x());
	/// assert_range!(0.5773503, vector.y());
	/// assert_range!(0.5773503, vector.z());
	/// let vector = Vector3::new(-0.1, 1.0, -2.4).normalize();
	/// assert_range!(-0.03843312, vector.x());
	/// assert_range!(0.3843312, vector.y());
	/// assert_range!(-0.9223949, vector.z());
	/// ```
	pub fn normalize(self) -> Self {
		let magnitude = self.magnitude();
		
		if magnitude == 0.0 { return Vector3::zero(); }
		if magnitude == 1.0 { return self; }
		
		let inverse_magnitude = magnitude.recip();
		
		return inverse_magnitude * self;
	}
	
	/// Projects this vector onto the given vector
	/// - **rhs**: The vector to project onto
	/// 
	/// **Returns**: Returns the projected vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(1.0, 2.0, 3.0);
	/// let b = Vector3::new(4.0, 5.0, 6.0);
	/// let expected = Vector3::new(1.662337662337662, 2.077922077922078, 2.493506493506494);
	/// assert_eq!(expected, a.project(b));
	/// ```
	pub fn project(self, rhs: Vector3) -> Self {
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
	/// # use mathx::Vector3;
	/// let a = Vector3::new(1.0, 2.0, 3.0);
	/// let b = Vector3::new(4.0, 5.0, 6.0);
	/// let expected = Vector3::new(-0.66233766, -0.077922106, 0.50649357);
	/// assert_eq!(expected, a.reject(b));
	/// ```
	pub fn reject(self, rhs: Vector3) -> Self {
		self - self.project(rhs)
	}
	
	/// Reflects this vector using a normal vector
	/// - **normal**: The normal vector to reflect off of
	/// 
	/// **Returns**: Returns the reflected vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let direction = Vector3::new(1.0, 0.0, 1.0);
	/// let normal = Vector3::new(0.0, 0.0, -1.0);
	/// let expected = Vector3::new(1.0, 0.0, -1.0);
	/// assert_eq!(expected, direction.reflect(normal));
	/// let direction = Vector3::new(0.25, -0.5, 1.25);
	/// let normal = Vector3::new(1.0, 0.5, -1.0);
	/// let expected = Vector3::new(2.75, 0.75, -1.25);
	/// assert_eq!(expected, direction.reflect(normal));
	/// ```
	pub fn reflect(self, normal: Vector3) -> Self {
		let dot = -2.0 * (self * normal);
		
		return dot * normal + self;
	}
	
	/// Scales the vector using another vector, multiplying everything component-wise
	/// - **rhs**: The other vector to scale with
	/// 
	/// **Returns**: Returns the scaled vector
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let a = Vector3::new(0.25, -0.5, 1.25);
	/// let b = Vector3::new(2.0, 0.5, -1.0);
	/// let expected = Vector3::new(0.5, -0.25, -1.25);
	/// assert_eq!(expected, a.scale(b));
	/// ```
	pub fn scale(self, rhs: Vector3) -> Self {
		Vector3::new(
			self.x * rhs.x,
			self.y * rhs.y,
			self.z * rhs.z
		)
	}
	
	/// Gets the signed angle between the two vectors using an axis in radians
	/// - **rhs**: The other vector to get the angle from
	/// - **axis**: The axis vector to determine what direction the angle is going
	/// 
	/// **Returns**: Returns the signed angle between the two vectors using an axis in radians
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3,Math,assert_range};
	/// let a = Vector3::new(0.25, -0.5, 1.25);
	/// let b = Vector3::new(2.0, 0.5, -1.0);
	/// let axis = Vector3::new(1.0, -1.0, 0.0);
	/// assert_range!(-1.89518322157, a.signed_angle_between(b, axis));
	/// ```
	pub fn signed_angle_between(self, rhs: Vector3, axis: Vector3) -> f32 {
		let angle = self.angle_between(rhs);
		let cross = self.cross(rhs);
		let sign = Math::sign(axis * cross);
		
		return sign * angle;
	}
	
	/// Gets the signed angle between the two vectors using an axis in degrees
	/// - **rhs**: The other vector to get the angle from
	/// - **axis**: The axis vector to determine what direction the angle is going
	/// 
	/// **Returns**: Returns the signed angle between the two vectors using an axis in degrees
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3,Math,assert_range};
	/// let a = Vector3::new(0.25, -0.5, 1.25);
	/// let b = Vector3::new(2.0, 0.5, -1.0);
	/// let axis = Vector3::new(1.0, -1.0, 0.0);
	/// assert_range!(-108.586, a.signed_angle_between_deg(b, axis), 0.01);
	/// ```
	pub fn signed_angle_between_deg(self, rhs: Vector3, axis: Vector3) -> f32 { Math::rad2deg(self.signed_angle_between(rhs, axis)) }
	
	/// Smooths a vector towards a desired goal over time
	/// - **target**: The position to try to reach
	/// - **velocity**: The current velocity
	/// - **smooth_time**: The time (in seconds) it will take to reach the target
	/// - **max_speed**: The maximum speed of the vector
	/// - **delta**: The time between frames
	/// 
	/// **Returns**: Returns a tuple of a vector that is closer towards the target and the new velocity
	/// #### Examples
	/// ```
	/// # use mathx::Vector3;
	/// let current = Vector3::new(1.0, 2.0, 3.0);
	/// let target = Vector3::new(14.0, 15.0, 16.0);
	/// let velocity = Vector3::new(4.0, 5.0, 6.0);
	/// let time = 8.0;
	/// let max_speed = 2.3;
	/// let delta = 0.2;
	/// let (position, velocity) = Vector3::smooth_damp(
	/// 	current,
	/// 	target,
	/// 	velocity,
	/// 	time,
	/// 	max_speed,
	/// 	delta
	/// );
	/// let expected_position = Vector3::new(1.7734365, 2.9636898, 4.156722);
	/// let expected_velocity = Vector3::new(3.7411351, 4.644839, 5.5768046);
	/// assert_eq!(expected_position, position);
	/// assert_eq!(expected_velocity, velocity);
	/// ```
	pub fn smooth_damp(self, target: Vector3, velocity: Vector3, smooth_time: f32, max_speed: f32, delta: f32) -> (Self, Self) {
		let smooth_time = Math::max(0.0001, smooth_time);
		let inv_smooth_time = 2.0 / smooth_time;
		let inv_smooth_delta = inv_smooth_time * delta;
		let cubic = 1.0 / (
			1.0
			+ inv_smooth_delta
			+ 0.47999998927116394 * inv_smooth_delta * inv_smooth_delta
			+ 0.23499999940395355 * inv_smooth_delta * inv_smooth_delta * inv_smooth_delta
		);
		let mut dir = self - target;
		let smooth_speed = max_speed * smooth_time;
		let sq_speed = smooth_speed * smooth_speed;
		let sq_magnitude = dir.square_magnitude();
		
		if sq_magnitude > sq_speed {
			dir *= smooth_speed / Math::sqrt(sq_magnitude);
		}
		
		let temp_target = target;
		let target = self - dir;
		let smooth_velocity = (velocity + inv_smooth_time * dir) * delta;
		let mut velocity = (velocity - inv_smooth_time * smooth_velocity) * cubic;
		let a = temp_target - self;
		let result = target + (dir + smooth_velocity) * cubic;
		let b = result - temp_target;
		
		if a * b > 0.0 {
			velocity = (result - temp_target) / delta;
		}
		
		return (result, velocity);
	}
	
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

impl Neg for Vector3 {
	type Output = Vector3;
	fn neg(self) -> Self::Output { Vector3::new(-self.x, -self.y, -self.z) }
}

use_impl_ops!();
impl_add!(Vector3);
impl_add!(Vector3 => Vector2: Vector3);
impl_sub!(Vector3);
impl_sub!(Vector3 => Vector2: Vector3);
impl_mul!(Vector3, Vector3 => f32: dot);
impl_mul!(Vector3);
impl_div!(Vector3);
