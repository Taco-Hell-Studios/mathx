
use core::ops::Neg;

use crate::Math;
#[cfg(not(feature = "no_vectors"))]
use crate::{Vector2,Vector3};
use crate::{AddSubArithmetic, MulDivScalar, use_impl_ops, impl_add, impl_sub, impl_mul, impl_div};

/// A 4D quaternion that holds 3 complex numbers and 1 real number
/// structured as such: (a + b *i* + c *j* + d *k*)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
	/// The real component of the quaternion
	a: f32,
	/// The complex *i* component of the quaternion
	b: f32,
	/// The complex *j* component of the quaternion
	c: f32,
	/// The complex *k* component of the quaternion
	d: f32,
}

/// Constructors
impl Quaternion {
	/// Creates a new quaternion from the given values
	/// - **a**: The real component of the quaternion
	/// - **b**: The complex *i* component of the quaternion
	/// - **c**: The complex *j* component of the quaternion
	/// - **d**: The complex *k* component of the quaternion
	/// 
	/// **Returns**: Returns a new quaternion
	/// #### Examples
	/// ```
	/// # use mathx::Quaternion;
	/// let quat = Quaternion::new(1.0, 0.2, 0.4, 0.6);
	/// assert_eq!(1.0, quat.a());
	/// assert_eq!(0.2, quat.b());
	/// assert_eq!(0.4, quat.c());
	/// assert_eq!(0.6, quat.d());
	/// ```
	pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self { Quaternion { a, b, c, d } }
	
	/// Gets the identity quaternion that represents no rotation
	/// 
	/// **Returns**: Returns the identity quaternion that represents no rotation
	/// #### Examples
	/// ```
	/// # use mathx::Quaternion;
	/// let quat = Quaternion::identity();
	/// assert_eq!(1.0, quat.a());
	/// assert_eq!(0.0, quat.b());
	/// assert_eq!(0.0, quat.c());
	/// assert_eq!(0.0, quat.d());
	/// ```
	pub fn identity() -> Self { Quaternion::new(1.0, 0.0, 0.0, 0.0) }
	
	/// Creates a rotation quaternion over the given axis and angle in radians
	/// - **axis**: The axis that the quaternion will rotate around
	/// - **angle**: The angle in radians that the quaternion will rotate around
	/// 
	/// **Returns**: Returns a rotation quaternion
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3,Quaternion,Math,assert_range};
	/// let axis = Vector3::new(0.0, 1.0, 0.0);
	/// let quat = Quaternion::from_axis_angle(axis, Math::PI_OVER_2);
	/// assert_range!(0.70710678, quat.a());
	/// assert_range!(0.0, quat.b());
	/// assert_range!(0.70710678, quat.c());
	/// assert_range!(0.0, quat.d());
	/// let axis = Vector3::new(1.0, 2.0, 3.0);
	/// let quat = Quaternion::from_axis_angle(axis, Math::PI_OVER_2);
	/// assert_range!(0.70710678, quat.a());
	/// assert_range!(0.18898223, quat.b());
	/// assert_range!(0.37796447, quat.c());
	/// assert_range!(0.5669467, quat.d());
	/// ```
	#[cfg(not(feature = "no_vectors"))]
	pub fn from_axis_angle(axis: Vector3, angle: f32) -> Self {
		let (sin, cos) = Math::sin_cos(0.5 * angle);
		let norm = axis.normalize();
		
		return Quaternion::new(
			cos,
			sin * norm.x(),
			sin * norm.y(),
			sin * norm.z(),
		);
	}
	
	/// Creates a rotation quaternion over the given axis and angle in degrees
	/// - **axis**: The axis that the quaternion will rotate around
	/// - **angle**: The angle in degrees that the quaternion will rotate around
	/// 
	/// **Returns**: Returns a rotation quaternion
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3,Quaternion,Math,assert_range};
	/// let axis = Vector3::new(0.0, 1.0, 0.0);
	/// let quat = Quaternion::from_axis_angle_deg(axis, 90.0);
	/// assert_range!(0.70710678, quat.a());
	/// assert_range!(0.0, quat.b());
	/// assert_range!(0.70710678, quat.c());
	/// assert_range!(0.0, quat.d());
	/// let axis = Vector3::new(1.0, 2.0, 3.0);
	/// let quat = Quaternion::from_axis_angle_deg(axis, 90.0);
	/// assert_range!(0.70710678, quat.a());
	/// assert_range!(0.18898223, quat.b());
	/// assert_range!(0.37796447, quat.c());
	/// assert_range!(0.5669467, quat.d());
	/// ```
	#[cfg(not(feature = "no_vectors"))]
	pub fn from_axis_angle_deg(axis: Vector3, angle: f32) -> Self { Quaternion::from_axis_angle(axis, Math::deg2rad(angle)) }
	
	/// Creates a new rotation quaternion from the given euler angles (in radians) on each axis
	/// - **euler_angles**: The angles rotating around the relative axis used to create the quaternion
	/// 
	/// **Returns**: Returns the new rotation quaternion from the given euler angles (in radians)
	/// #### Examples
	/// ```
	/// # use mathx::{Quaternion,Vector3,Math};
	/// let actual = Quaternion::from_euler(Vector3::new(-0.209439510239, 0.698131700798, 1.34390352404));
	/// let expected = Quaternion::new(0.7091271, 0.1348748, 0.3273477, 0.6097468);
	/// assert_eq!(expected, actual);
	/// let actual = Quaternion::from_euler(Vector3::new(
	/// 	Math::PI_OVER_2, Math::PI_OVER_4, 0.0
	/// ));
	/// let expected = Quaternion::new(0.65328145, 0.6532815, 0.27059805, -0.27059805);
	/// assert_eq!(expected, actual);
	/// let actual = Quaternion::from_euler(Vector3::new(
	/// 	0.0, Math::PI_OVER_2, 0.0
	/// ));
	/// let expected = Quaternion::new(0.70710678, 0.0, 0.70710678, 0.0);
	/// assert_eq!(expected, actual);
	/// ```
	#[cfg(not(feature = "no_vectors"))]
	pub fn from_euler(euler_angles: Vector3) -> Self {
		let (sin_yaw, cos_yaw) = Math::sin_cos(-0.5 * euler_angles.x());
		let (sin_pitch, cos_pitch) = Math::sin_cos(-0.5 * euler_angles.y());
		let (sin_roll, cos_roll) = Math::sin_cos(-0.5 * euler_angles.z());
		
		return Quaternion::new(
			(cos_yaw * cos_pitch * cos_roll) - (sin_yaw * sin_pitch * sin_roll),
			(cos_yaw * sin_pitch * sin_roll) - (sin_yaw * cos_pitch * cos_roll),
			-(cos_yaw * sin_pitch * cos_roll) - (sin_yaw * cos_pitch * sin_roll),
			-(sin_yaw * sin_pitch * cos_roll) - (cos_yaw * cos_pitch * sin_roll)
		);
	}
	
	/// Creates a new rotation quaternion from the given euler angles (in degrees) on each axis
	/// - **euler_angles**: The angles rotating around the relative axis used to create the quaternion
	/// 
	/// **Returns**: Returns the new rotation quaternion from the given euler angles (in degrees)
	/// #### Examples
	/// ```
	/// # use mathx::{Quaternion,Vector3,Math};
	/// let actual = Quaternion::from_euler_deg(Vector3::new(
	/// 	-12.0, 40.0, 77.0
	/// ));
	/// let expected = Quaternion::new(0.7091271, 0.1348748, 0.3273477, 0.6097468);
	/// assert_eq!(expected, actual);
	/// let actual = Quaternion::from_euler_deg(Vector3::new(
	/// 	90.0, 45.0, 0.0
	/// ));
	/// let expected = Quaternion::new(0.65328145, 0.65328145, 0.27059805, -0.27059805);
	/// assert_eq!(expected, actual);
	/// let actual = Quaternion::from_euler_deg(Vector3::new(
	/// 	0.0, 90.0, 0.0
	/// ));
	/// let expected = Quaternion::new(0.70710678, 0.0, 0.70710678, 0.0);
	/// assert_eq!(expected, actual);
	/// let actual = Quaternion::from_euler_deg(Vector3::new(
	/// 	-23.0, 45.0, 67.0
	/// ));
	/// let expected = Quaternion::new(0.7128339, 0.05338183, 0.4143703, 0.5633075);
	/// assert_eq!(expected, actual);
	/// ```
	#[cfg(not(feature = "no_vectors"))]
	pub fn from_euler_deg(euler_angles: Vector3) -> Self {
		return Quaternion::from_euler(Vector3::new(
			Math::deg2rad(euler_angles.x()),
			Math::deg2rad(euler_angles.y()),
			Math::deg2rad(euler_angles.z())
		));
	}
	
	// TODO: Add a from_matrix function here
}

/// Properties
impl Quaternion {
	/// Gets the real component of the quaternion
	/// 
	/// **Returns**: Returns the real component of the quaternion
	pub fn a(&self) -> f32 { self.a }
	
	/// Sets the real component of the quaternion
	/// - **value**: The value to set the real component to
	pub fn set_a(&mut self, value: f32) { self.a = value; }
	
	/// Gets the complex *i* component of the quaternion
	/// 
	/// **Returns**: Returns the complex *i* component of the quaternion
	pub fn b(&self) -> f32 { self.b }
	
	/// Sets the complex *i* component of the quaternion
	/// - **value**: The value to set the complex *i* component of the quaternion
	pub fn set_b(&mut self, value: f32) { self.b = value; }
	
	/// Gets the complex *j* component of the quaternion
	/// 
	/// **Returns**: Returns the complex *j* component of the quaternion
	pub fn c(&self) -> f32 { self.c }
	
	/// Sets the complex *j* component of the quaternion
	/// - **value**: The value to set the complex *j* component of the quaternion
	pub fn set_c(&mut self, value: f32) { self.c = value; }
	
	/// Gets the complex *k* component of the quaternion
	/// 
	/// **Returns**: Returns the complex *k* component of the quaternion
	pub fn d(&self) -> f32 { self.d }
	
	/// Sets the complex *k* component of the quaternion
	/// - **value**: The value to set the complex *k* component of the quaternion
	pub fn set_d(&mut self, value: f32) { self.d = value; }
	
	/// Gets the euler angles (in radians) of the quaternion
	/// 
	/// **Returns**: Returns the euler angles (in radians) in a 3D vector
	/// #### Remarks
	/// This isn't very accurate, the x and y coordinates have an error-margin of 0.01
	/// while the z coordinate has an error-margin of 0.06
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3,Quaternion,Math,assert_range};
	/// let euler = Vector3::new(0.3, 0.2, 1.0);
	/// let quat = Quaternion::from_euler(euler);
	/// assert_range!(euler.x(), quat.euler().x(), 0.01);
	/// assert_range!(euler.y(), quat.euler().y(), 0.01);
	/// assert_range!(euler.z(), quat.euler().z(), 0.06);
	/// ```
	#[cfg(not(feature = "no_vectors"))]
	pub fn euler(&self) -> Vector3 {
		const SINGULARITY_TEST: f32 = 0.499999995;
		
		let sq_a = self.a * self.a;
		let sq_b = self.b * self.b;
		let sq_c = self.c * self.c;
		let sq_d = self.d * self.d;
		let unit = sq_a + sq_b + sq_c + sq_d;
		let singularity_test = (self.b * self.d) + (self.a * self.c);
		
		if singularity_test > SINGULARITY_TEST * unit {
			return Vector3::new(
				0.0,
				Math::PI_OVER_2,
				2.0 * Math::atan2(self.b, self.a)
			);
		}
		else if singularity_test < -SINGULARITY_TEST * unit {
			return Vector3::new(
				0.0,
				-Math::PI_OVER_2,
				-2.0 * Math::atan2(self.b, self.a)
			);
		}
		
		return Vector3::new(
			Math::atan2(
				2.0 * ((self.a * self.b) - (self.c * self.d)),
				sq_a - sq_b - sq_c + sq_d
			),
			Math::asin(2.0 * singularity_test / unit),
			Math::atan2(
				2.0 * ((self.a * self.d) - (self.b * self.c)),
				sq_a + sq_b - sq_c - sq_d
			)
		);
	}
	
	/// Sets the euler angles (in radians) of the quaternion
	/// - **value**: The euler angles (in radians) to update the quaternion with
	/// #### Examples
	/// ```
	/// # use mathx::{Quaternion,Vector3,Math,assert_range};
	/// let mut actual = Quaternion::identity();
	/// actual.set_euler(Vector3::new(
	/// 	Math::PI_OVER_2, Math::PI_OVER_4, 0.0
	/// ));
	/// let expected = Quaternion::new(0.65328145, 0.65328145, 0.27059805, -0.27059805);
	/// assert_eq!(expected, actual);
	/// actual.set_euler(Vector3::new(
	/// 	0.0, Math::PI_OVER_2, 0.0
	/// ));
	/// let expected = Quaternion::new(0.70710678, 0.0, 0.70710678, 0.0);
	/// assert_eq!(expected, actual);
	/// ```
	#[cfg(not(feature = "no_vectors"))]
	pub fn set_euler(&mut self, value: Vector3) {
		let quat = Quaternion::from_euler(value);
		self.a = quat.a;
		self.b = quat.b;
		self.c = quat.c;
		self.d = quat.d;
	}
	
	/// Gets the euler angles (in degrees) of the quaternion
	/// 
	/// **Returns**: Returns the euler angles (in degrees) in a 3D vector
	/// #### Remarks
	/// This isn't very accurate, the x and y coordinates have an error-margin of 4.0 while
	/// the z coordinate has an error margin of 14.0
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3,Quaternion,Math,assert_range};
	/// let euler = Vector3::new(-12.0, 40.0, 77.0);
	/// let quat = Quaternion::from_euler_deg(euler);
	/// assert_range!(euler.x(), quat.euler_deg().x(), 4.0);
	/// assert_range!(euler.y(), quat.euler_deg().y(), 4.0);
	/// assert_range!(euler.z(), quat.euler_deg().z(), 10.0);
	/// ```
	#[cfg(not(feature = "no_vectors"))]
	pub fn euler_deg(&self) -> Vector3 {
		let euler = self.euler();
		
		return Vector3::new(
			Math::rad2deg(euler.x()),
			Math::rad2deg(euler.y()),
			Math::rad2deg(euler.z())
		);
	}
	
	/// Sets the euler angles (in degrees) of the quaternion
	/// - **value**: The euler angles (in degrees) to update the quaternion with
	/// #### Examples
	/// ```
	/// # use mathx::{Quaternion,Vector3,Math,assert_range};
	/// let mut actual = Quaternion::identity();
	/// actual.set_euler_deg(Vector3::new(
	/// 	90.0, 45.0, 0.0
	/// ));
	/// let expected = Quaternion::new(0.65328145, 0.65328145, 0.27059805, -0.27059805);
	/// assert_eq!(expected, actual);
	/// actual.set_euler_deg(Vector3::new(
	/// 	0.0, 90.0, 0.0
	/// ));
	/// let expected = Quaternion::new(0.70710678, 0.0, 0.70710678, 0.0);
	/// assert_eq!(expected, actual);
	/// ```
	#[cfg(not(feature = "no_vectors"))]
	pub fn set_euler_deg(&mut self, value: Vector3) {
		let quat = Quaternion::from_euler_deg(value);
		self.a = quat.a;
		self.b = quat.b;
		self.c = quat.c;
		self.d = quat.d;
	}
	
	/// Gets the magnitude of the quaternion
	/// 
	/// **Returns**: Returns the magnitude of the quaternion
	/// #### Examples
	/// ```
	/// # use mathx::{Quaternion,Math,assert_range};
	/// let quat = Quaternion::identity();
	/// assert_eq!(1.0, quat.magnitude());
	/// let quat = Quaternion::new(0.0, 0.0, 0.0, 0.0);
	/// assert_eq!(0.0, quat.magnitude());
	/// let quat = Quaternion::new(1.0, 2.0, 3.0, 4.0);
	/// assert_range!(5.477225575051661, quat.magnitude());
	/// ```
	pub fn magnitude(&self) -> f32 {
		let magnitude = self.squared_magnitude();
		
		if magnitude == 0.0 || magnitude == 1.0 {
			return magnitude;
		}
		
		return Math::sqrt(magnitude);
	}
	
	/// Gets the squared magnitude of the quaternion
	/// 
	/// **Returns**: Returns the squared magnitude of the quaternion
	/// #### Examples
	/// ```
	/// # use mathx::Quaternion;
	/// let quat = Quaternion::identity();
	/// assert_eq!(1.0, quat.squared_magnitude());
	/// let quat = Quaternion::new(0.0, 0.0, 0.0, 0.0);
	/// assert_eq!(0.0, quat.squared_magnitude());
	/// let quat = Quaternion::new(1.0, 2.0, 3.0, 4.0);
	/// assert_eq!(30.0, quat.squared_magnitude());
	/// ```
	pub fn squared_magnitude(&self) -> f32 {
		self.a * self.a
		+ self.b * self.b
		+ self.c * self.c
		+ self.d * self.d
	}
}

/// Public Methods
impl Quaternion {
	// TODO: to_matrix
	
	/// Conjugates the quaternion, so it turns it from (a + b *i* + c *j* + d *k*) to (a - b *i* - c *j* - d *k*)
	/// 
	/// **Returns**: Returns the conjugated quaternion
	/// #### Examples
	/// ```
	/// # use mathx::Quaternion;
	/// let quat = Quaternion::new(1.0, -2.0, 3.0, -4.0);
	/// let expected = Quaternion::new(1.0, 2.0, -3.0, 4.0);
	/// assert_eq!(expected, quat.conjugate());
	/// ```
	pub fn conjugate(self) -> Self { Quaternion::new(self.a, -self.b, -self.c, -self.d) }
	
	/// Divides the two quaternions together
	/// - **rhs**: The other quaternion to divide with
	/// 
	/// **Returns**: Returns a divided quaternion
	/// #### Examples
	/// ```
	/// # use mathx::Quaternion;
	/// let a = Quaternion::new(1.0, 2.0, 3.0, 4.0);
	/// let b = Quaternion::new(5.0, 6.0, 7.0, 8.0);
	/// let expected = Quaternion::new(0.013409962, 0.0015325671, 0.0, 0.0030651342);
	/// assert_eq!(expected, a / b);
	/// let expected = Quaternion::new(0.013409962, -0.0015325671, 0.0, -0.0030651342);
	/// assert_eq!(expected, b / a);
	/// let expected = Quaternion::new(-0.031111112, 0.0044444446, 0.006666667, 0.008888889);
	/// assert_eq!(expected, a / a.conjugate());
	/// ```
	pub fn divide(self, rhs: Quaternion) -> Self {
		let divided = self * rhs.conjugate();
		
		return divided / divided.squared_magnitude();
	}
	
	/// Dot products the two quaternions together
	/// - **rhs**: The other quaternion to get the dot product with
	/// 
	/// **Returns**: Returns the dot product
	/// #### Examples
	/// ```
	/// # use mathx::Quaternion;
	/// let a = Quaternion::new(1.0, 2.0, 3.0, 4.0);
	/// let b = Quaternion::new(5.0, 6.0, 7.0, 8.0);
	/// assert_eq!(70.0, a.dot(b));
	/// ```
	pub fn dot(self, rhs: Quaternion) -> f32 { self.a * rhs.a + self.b * rhs.b + self.c * rhs.c + self.d * rhs.d }
	
	/// Inverts the quaternion
	/// 
	/// **Returns**: Returns the inverted quaternion
	/// #### Examples
	/// ```
	/// # use mathx::Quaternion;
	/// let actual = Quaternion::new(1.0, -2.0, 3.0, -4.0);
	/// let expected = Quaternion::new(0.033333333, 0.06666667, -0.1, 0.13333334);
	/// assert_eq!(expected, actual.invert());
	/// assert_eq!(Quaternion::identity(), actual * actual.invert());
	/// assert_eq!(Quaternion::identity(), Quaternion::identity().invert());
	/// ```
	pub fn invert(self) -> Self {
		let magnitude = self.squared_magnitude();
		
		if magnitude == 0.0 { return self; }
		
		return self.conjugate() / magnitude;
	}
	
	/// Multiplies the two quaternions together
	/// - **rhs**: The other quaternion to multiply with
	/// 
	/// **Returns**: Returns a multiplied quaternion
	/// #### Remarks
	/// Multiplying quaternions are not commutative, meaning that `a * b =/= b * a`
	/// #### Examples
	/// ```
	/// # use mathx::Quaternion;
	/// let a = Quaternion::new(1.0, 2.0, 3.0, 4.0);
	/// let b = Quaternion::new(5.0, 6.0, 7.0, 8.0);
	/// let expected = Quaternion::new(-60.0, 12.0, 30.0, 24.0);
	/// assert_eq!(expected, a * b);
	/// let expected = Quaternion::new(-60.0, 20.0, 14.0, 32.0);
	/// assert_eq!(expected, b * a);
	/// assert_eq!(30.0 * Quaternion::identity(), a * a.conjugate());
	/// ```
	pub fn multiply(self, rhs: Quaternion) -> Self {
		Quaternion::new(
			self.a * rhs.a - self.b * rhs.b - self.c * rhs.c - self.d * rhs.d,
			self.a * rhs.b + self.b * rhs.a + self.c * rhs.d - self.d * rhs.c,
			self.a * rhs.c - self.b * rhs.d + self.c * rhs.a + self.d * rhs.b,
			self.a * rhs.d + self.b * rhs.c - self.c * rhs.b + self.d * rhs.a
		)
	}
	
	/// Multiplies the quaternion with the vector to rotate the vector
	/// - **rhs**: The vector to multiply with
	/// 
	/// **Returns**: Returns the rotated vector
	/// #### Examples
	/// ```
	/// # use mathx::{Quaternion,Vector2,Vector3,Math,assert_range};
	/// let vector = Vector2::new(100.0, 200.0);
	/// let rotation = Quaternion::from_euler_deg(Vector3::new(-12.0, 40.0, 77.0));
	/// let expected = Vector2::new(-151.0844, 139.3148);
	/// assert_range!(expected.x(), (rotation * vector).x());
	/// assert_range!(expected.y(), (rotation * vector).y());
	/// ```
	#[cfg(not(feature = "no_vectors"))]
	pub fn multiply_vector2(self, rhs: Vector2) -> Vector2 { self.multiply_vector3(rhs.to_vector3()).to_vector2() }
	
	/// Multiplies the quaternion with the vector to rotate the vector
	/// - **rhs**: The vector to multiply with
	/// 
	/// **Returns**: Returns the rotated vector
	/// #### Examples
	/// ```
	/// # use mathx::{Quaternion,Vector3,Math,assert_range};
	/// let vector = Vector3::new(100.0, 200.0, 300.0);
	/// let rotation = Quaternion::from_euler_deg(Vector3::new(-12.0, 40.0, 77.0));
	/// let expected = Vector3::new(37.538, 201.6883, 312.9101);
	/// assert_range!(expected.x(), (rotation * vector).x());
	/// assert_range!(expected.y(), (rotation * vector).y());
	/// assert_range!(expected.z(), (rotation * vector).z());
	/// ```
	#[cfg(not(feature = "no_vectors"))]
	pub fn multiply_vector3(self, rhs: Vector3) -> Vector3 {
		let vector = Vector3::new(self.b, self.c, self.d);
		
		rhs + 2.0 * Vector3::cross(
			vector,
			Vector3::cross(
				vector,
				rhs
			) + self.a * rhs
		)
	}
	
	/// Normalizes the quaternion
	/// 
	/// **Returns**: Returns the normalized quaternion
	/// #### Examples
	/// ```
	/// # use mathx::Quaternion;
	/// let actual = Quaternion::new(1.0, 2.0, 3.0, 4.0);
	/// let expected = Quaternion::new(0.18257418, 0.36514837, 0.5477225, 0.73029673);
	/// assert_eq!(expected, actual.normalize());
	/// ```
	pub fn normalize(self) -> Self { self / self.magnitude() }
	
	/// Spherically interpolates between the two quaternions
	/// - **rhs**: The other quaternion to interpolate towards
	/// - **t**: The clamped ratio (t) to interpolate with
	/// 
	/// **Returns**: Returns the spherically interpolated quaternion
	/// #### Examples
	/// ```
	/// # use mathx::{Quaternion,Math,assert_range};
	/// let a = Quaternion::new(0.8660254, 0.0, 0.5, 0.0);
	/// let b = Quaternion::new(0.4158418, 0.1114245, -0.2336062, 0.8718304);
	/// let expected = Quaternion::new(0.81289685, 0.07065991, 0.1689338, 0.55287176);
	/// assert_range!(expected.a(), a.slerp(b, 0.5).a(), 0.001);
	/// assert_range!(expected.b(), a.slerp(b, 0.5).b(), 0.001);
	/// assert_range!(expected.c(), a.slerp(b, 0.5).c(), 0.001);
	/// assert_range!(expected.d(), a.slerp(b, 0.5).d(), 0.001);
	/// ```
	pub fn slerp(self, rhs: Quaternion, t: f32) -> Self { self.slerp_unclamped(rhs, t.clamp(0.0, 1.0)) }
	
	/// Spherically interpolates between the two quaternions (not clamped)
	/// - **rhs**: The other quaternion to interpolate towards
	/// - **t**: The unclamped ratio (t) to interpolate with
	/// 
	/// **Returns**: Returns the spherically interpolated quaternion
	/// #### Examples
	/// ```
	/// # use mathx::{Quaternion,Math,assert_range};
	/// let a = Quaternion::new(0.8660254, 0.0, 0.5, 0.0);
	/// let b = Quaternion::new(0.4158418, 0.1114245, -0.2336062, 0.8718304);
	/// let expected = Quaternion::new(0.81289685, 0.07065991, 0.1689338, 0.55287176);
	/// assert_range!(expected.a(), a.slerp_unclamped(b, 0.5).a(), 0.001);
	/// assert_range!(expected.b(), a.slerp_unclamped(b, 0.5).b(), 0.001);
	/// assert_range!(expected.c(), a.slerp_unclamped(b, 0.5).c(), 0.001);
	/// assert_range!(expected.d(), a.slerp_unclamped(b, 0.5).d(), 0.001);
	/// ```
	pub fn slerp_unclamped(self, rhs: Quaternion, t: f32) -> Self {
		let unit_self = self.normalize();
		let mut unit_rhs = rhs.normalize();
		let mut dot = unit_self.dot(unit_rhs);
		
		if dot < 0.0 {
			unit_rhs = -unit_rhs;
			dot = -dot;
		}
		if dot > 0.9995 {
			return (unit_self + t * (unit_rhs - unit_self)).normalize();
		}
		
		let angle = t * Math::acos(dot);
		let unit = dot * unit_self;
		let unit_rhs = (unit_rhs - unit).normalize();
		let (sin, cos) = Math::sin_cos(angle);
		
		return cos * unit_self + sin * unit_rhs;
	}
	
}

unsafe impl Send for Quaternion {}
unsafe impl Sync for Quaternion {}

// Equates
impl Eq for Quaternion {}
impl PartialEq for Quaternion {
	fn eq(&self, other: &Self) -> bool {
		Math::approx(self.a, other.a)
		&& Math::approx(self.b, other.b)
		&& Math::approx(self.c, other.c)
		&& Math::approx(self.d, other.d)
	}
}

// Display
#[cfg(not(feature = "no_std"))]
impl std::fmt::Display for Quaternion {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.write_str(&format!("({}, {}i, {}j, {}k)", self.a, self.b, self.c, self.d))
	}
}

impl AddSubArithmetic<Quaternion> for Quaternion {
	type Output = Quaternion;
	fn add_other(self, rhs: Quaternion) -> Self::Output {
		Quaternion::new(
			self.a + rhs.a,
			self.b + rhs.b,
			self.c + rhs.c,
			self.d + rhs.d
		)
	}
	fn add_assign_other(&mut self, rhs: Quaternion) {
		self.a += rhs.a;
		self.b += rhs.b;
		self.c += rhs.c;
		self.d += rhs.d;
	}
	fn subtract_other(self, rhs: Quaternion) -> Self::Output {
		Quaternion::new(
			self.a - rhs.a,
			self.b - rhs.b,
			self.c - rhs.c,
			self.d - rhs.d
		)
	}
	fn subtract_assign_other(&mut self, rhs: Quaternion) {
		self.a -= rhs.a;
		self.b -= rhs.b;
		self.c -= rhs.c;
		self.d -= rhs.d;
	}
}

impl MulDivScalar for Quaternion {
	type Output = Quaternion;
	fn multiply_scalar(self, rhs: f32) -> Self::Output {
		Quaternion::new(
			rhs * self.a,
			rhs * self.b,
			rhs * self.c,
			rhs * self.d
		)
	}
	fn multiply_assign_scalar(&mut self, rhs: f32) {
		self.a *= rhs;
		self.b *= rhs;
		self.c *= rhs;
		self.d *= rhs;
	}
	fn divide_scalar(self, rhs: f32) -> Self::Output {
		if rhs == 0.0 { return Quaternion::new(0.0, 0.0, 0.0, 0.0); }
		Quaternion::new(
			self.a / rhs,
			self.b / rhs,
			self.c / rhs,
			self.d / rhs
		)
	}
	fn divide_assign_scalar(&mut self, rhs: f32) {
		if rhs == 0.0 {
			self.a = 0.0;
			self.b = 0.0;
			self.c = 0.0;
			self.d = 0.0;
		}
		else {
			self.a /= rhs;
			self.b /= rhs;
			self.c /= rhs;
			self.d /= rhs;
		}
	}
	fn reciprocal_scalar(self, rhs: f32) -> Self::Output {
		Quaternion::new(
			if self.a != 0.0 { rhs / self.a } else { 0.0 },
			if self.b != 0.0 { rhs / self.b } else { 0.0 },
			if self.c != 0.0 { rhs / self.c } else { 0.0 },
			if self.d != 0.0 { rhs / self.d } else { 0.0 }
		)
	}
}

impl Neg for Quaternion {
	type Output = Quaternion;
	fn neg(self) -> Self::Output {
		Quaternion::new(
			-self.a,
			-self.b,
			-self.c,
			-self.d
		)
	}
}

use_impl_ops!();
impl_add!(Quaternion);
impl_sub!(Quaternion);
impl_mul!(Quaternion);
impl_mul!(Quaternion, Quaternion => Quaternion: multiply);
#[cfg(not(feature = "no_vectors"))]
impl_mul!(Quaternion, Vector2 => Vector2: multiply_vector2);
#[cfg(not(feature = "no_vectors"))]
impl_mul!(Quaternion, Vector3 => Vector3: multiply_vector3);
impl_div!(Quaternion);
impl_div!(Quaternion, Quaternion => Quaternion: divide);
