
use core::ops::Range;

/// A "static" structure used to compute math functions. Since `f32` gets a lot of it's
/// functions stripped away when using `no_std`, you can use this structure to regain
/// those functions. It will also work the same even if you don't use it for `no_std`.
pub struct Math;

impl Math {
	pub const PI: f32 = 3.14159265359;
	pub const PI_OVER_2: f32 = 1.570796326;
	pub const TWO_PI: f32 = 6.28318530718;
	pub const E: f32 = 2.71828182845;
	pub const DEG_TO_RAD: f32 = Math::PI / 180.0;
	pub const RAD_TO_DEG: f32 = 180.0 / Math::PI;
}

#[cfg(feature = "no_std")]
impl Math {
	/// Gets the power of the given number by the other given number, with the power being an `i32`
	/// - **a**: The base number to power
	/// - **b**: The number to power with
	/// 
	/// **Returns**: Returns the powered number
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::pow_i32(3.0, 5);
	/// assert_eq!(243.0, value);
	/// let value = Math::pow_i32(10.45, 3);
	/// assert_eq!(1141.166125, value);
	/// let value = Math::pow_i32(0.0, 0);
	/// assert_eq!(1.0, value);
	/// let value = Math::pow_i32(10.0, 0);
	/// assert_eq!(1.0, value);
	/// let value = Math::pow_i32(0.0, 2);
	/// assert_eq!(0.0, value);
	/// let value = Math::pow_i32(2.0, -3);
	/// assert_eq!(0.125, value);
	/// ```
	pub fn pow_i32(a: f32, b: i32) -> f32 {
		if b == 0 { return 1.0 }
		
		let mut result = a;
		
		for _ in 1..Math::abs_i32(b) {
			result *= a;
		}
		
		if b < 0 { 1.0 / result }
		else { result }
	}
	
	/// Computes the cos and sin of the angle
	/// - **angle**: The angle to compute the sine and cosine with
	/// 
	/// **Returns**: Returns the sine and cosine (respectively) as a tuple
	/// #### Remarks
	/// If you need to compute both `cos` and `sin` of the same angle, this function is more
	/// performant to produce both values than calling `cos` and `sin` separately
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range_tuple2};
	/// let value = Math::sin_cos(0.0);
	/// assert_range_tuple2!((0.0, 1.0), value);
	/// let value = Math::sin_cos(Math::PI_OVER_2);
	/// assert_range_tuple2!((1.0, 0.0), value);
	/// let value = Math::sin_cos(Math::PI);
	/// assert_range_tuple2!((0.0, -1.0), value);
	/// let value = Math::sin_cos(Math::PI + Math::PI_OVER_2);
	/// assert_range_tuple2!((-1.0, 0.0), value);
	/// let value = Math::sin_cos(Math::TWO_PI);
	/// assert_range_tuple2!((0.0, 1.0), value);
	/// let value = Math::sin_cos(Math::PI_OVER_2 * 0.5);
	/// assert_range_tuple2!((0.707106781, 0.707106781), value);
	/// let value = Math::sin_cos(1.0);
	/// assert_range_tuple2!((0.841470985, 0.540302306), value);
	/// let value = Math::sin_cos(-100.0);
	/// assert_range_tuple2!((0.506365641, 0.862318872), value);
	/// ```
	pub fn sin_cos(angle: f32) -> (f32, f32) { Math::cordic(angle) }
	
	/// Computes the sine of the given angle
	/// - **angle**: The angle to compute sine with in radians
	/// 
	/// **Returns**: Returns a value from the computed sine
	/// #### Remarks
	/// If you need to compute both `cos` and `sin` of the same angle, use `sin_cos` instead as it's more
	/// performant to produce both values than calling `cos` and `sin` separately
	/// ##### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::sin(0.0);
	/// assert_range!(0.0, value);
	/// let value = Math::sin(Math::PI_OVER_2);
	/// assert_range!(1.0, value);
	/// let value = Math::sin(Math::PI);
	/// assert_range!(0.0, value);
	/// let value = Math::sin(Math::PI + Math::PI_OVER_2);
	/// assert_range!(-1.0, value);
	/// let value = Math::sin(Math::TWO_PI);
	/// assert_range!(0.0, value);
	/// let value = Math::sin(Math::PI_OVER_2 * 0.5);
	/// assert_range!(0.707106781, value);
	/// let value = Math::sin(1.0);
	/// assert_range!(0.841470985, value);
	/// let value = Math::sin(-100.0);
	/// assert_range!(0.506365641, value);
	/// ```
	pub fn sin(angle: f32) -> f32 { Math::cordic(angle).0 }
	
	/// Computes the cos of the given angle
	/// - **angle**: The angle to compute cosine with in radians
	/// 
	/// **Returns**: Returns a value from the computed cosine
	/// #### Remarks
	/// If you need to compute both `cos` and `sin` of the same angle, use `sin_cos` instead as it's more
	/// performant to produce both values than calling `cos` and `sin` separately
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::cos(0.0);
	/// assert_range!(1.0, value);
	/// let value = Math::cos(Math::PI_OVER_2);
	/// assert_range!(0.0, value);
	/// let value = Math::cos(Math::PI);
	/// assert_range!(-1.0, value);
	/// let value = Math::cos(Math::PI + Math::PI_OVER_2);
	/// assert_range!(0.0, value);
	/// let value = Math::cos(Math::TWO_PI);
	/// assert_range!(1.0, value);
	/// let value = Math::cos(Math::PI_OVER_2 * 0.5);
	/// assert_range!(0.707106781, value);
	/// let value = Math::cos(1.0);
	/// assert_range!(0.540302306, value);
	/// let value = Math::cos(-100.0);
	/// assert_range!(0.862318872, value);
	/// ```
	pub fn cos(angle: f32) -> f32 { Math::cordic(angle).1 }
	
	/// Gets the pre-calculated arc tangent values for use in the cordic algorithm
	/// - **index**: The index to get the pre-calculated value from
	/// 
	/// **Returns**: Returns the pre-calculated value for the arc tangent
	pub(self) fn get_atan_for_cordic(index: i32) -> f32 {
		match index {
			0 => 0.7853982,
			1 => 0.4636476,
			2 => 0.24497867,
			3 => 0.124354996,
			4 => 0.06241881,
			5 => 0.031239834,
			6 => 0.015623729,
			7 => 0.007812341,
			8 => 0.0039062302,
			9 => 0.0019531226,
			10 => 0.0009765622,
			11 => 0.00048828122,
			12 => 0.00024414063,
			13 => 0.00012207031,
			14 => 0.000061035156,
			15 => 0.000030517578,
			_ => 0.0,
		}
	}
	
	/// Negates the tuple, multiplying both components by -1
	/// - **tuple**: The tuple to negate
	/// 
	/// **Returns**: Returns the negated tuple
	pub(self) fn negate_tuple(tuple: (f32, f32)) -> (f32, f32) { (-tuple.0, -tuple.1) }
	
	/// Performs the CORDIC algorithm used to retrieve the sine and cosine values
	/// - **angle**: The angle to find the value for
	/// 
	/// **Returns**: Returns the results of sine and cosine (respectively) in tuple form
	pub(self) fn cordic(angle: f32) -> (f32, f32) {
		const ITERATIONS: i32 = 16;
		
		if angle < -Math::PI_OVER_2 || angle > Math::PI_OVER_2 {
			return if angle < 0.0 { Math::negate_tuple(Math::cordic(angle + Math::PI)) }
				else { Math::negate_tuple(Math::cordic(angle - Math::PI)) };
		}
		
		let mut cos = 0.6072529_f32;
		let mut sin = 0.0_f32;
		let mut z = angle;
		
		for i in 0..ITERATIONS {
			let di = if z <= 0.0 { -1.0 } else { 1.0 };
			let new_cos = cos - (sin * di * Math::pow_i32(2.0, -i));
			let new_sin = sin + (cos * di * Math::pow_i32(2.0, -i));
			
			cos = new_cos;
			sin = new_sin;
			z -= di * Math::get_atan_for_cordic(i);
		}
		
		return (sin, cos);
	}
	
	/// Finds if the two floating point numbers are approximately close to each other
	/// - **a**: The first number to check with
	/// - **b**: The second number to check with
	/// 
	/// **Returns**: Returns true if the two values are approximately close to each other
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// assert!(Math::approx(1.20000001, 1.2));
	/// ```
	pub fn approx(a: f32, b: f32) -> bool {
		Math::abs(a - b) < 0.000001
	}
	
	/// Finds if the two floating point numbers are approximately close to each other, provided the epsilon
	/// - **a**: The first number to check with
	/// - **b**: The second number to check with
	/// - **epsilon**: The epsilon (smallest possible difference between numbers) to check with
	/// 
	/// **Returns**: Returns true if the two values are approximately close to each other
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// assert!(Math::approx_epsilon(1.2001, 1.2, 0.001));
	/// ```
	pub fn approx_epsilon(a: f32, b: f32, epsilon: f32) -> bool {
		Math::abs(a - b) < epsilon
	}
	
	/// Gets the fractional part of the value, getting only a value between 0 and 1
	/// - **value**: The value to get the fraction from
	/// 
	/// **Returns**: Returns the fraction of the given number
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::frac(3.0);
	/// assert_eq!(0.0, value);
	/// let value = Math::frac(-3.0);
	/// assert_eq!(0.0, value);
	/// let value = Math::frac(4.9);
	/// assert!((0.9..0.90001).contains(&value));
	/// let value = Math::frac(-4.9);
	/// assert!((0.0999999..0.1).contains(&value));
	/// let value = Math::frac(12.34);
	/// assert!((0.34..0.340001).contains(&value));
	/// ```
	pub fn frac(value: f32) -> f32 { value - Math::floor(value) }
	
	/// Gets the smallest integer number that is greater than or equal to the given number
	/// - **value**: The value to get the ceiling with
	/// 
	/// **Returns**: Returns the ceiling number
	/// #### Example
	/// ```
	/// # use mathx::Math;
	/// let value = Math::ceil(-3.0);
	/// assert_eq!(-3.0, value);
	/// let value = Math::ceil(1.4);
	/// assert_eq!(2.0, value);
	/// let value = Math::ceil(2.9);
	/// assert_eq!(3.0, value);
	/// let value = Math::ceil(-4.9);
	/// assert_eq!(-4.0, value);
	/// let value = Math::ceil(-5.3);
	/// assert_eq!(-5.0, value);
	/// ```
	pub fn ceil(value: f32) -> f32 {
		let trunc = Math::trunc(value);
		
		if trunc == value { return trunc; }
		
		return trunc + if value < 0.0 { 0.0 } else { 1.0 };
	}
	
	/// Gets the largest integer number that is less than or equal to the given number
	/// - **value**: The value to get the floor with
	/// 
	/// **Returns**: Returns the floored number
	/// #### Example
	/// ```
	/// # use mathx::Math;
	/// let value = Math::floor(-3.0);
	/// assert_eq!(-3.0, value);
	/// let value = Math::floor(1.4);
	/// assert_eq!(1.0, value);
	/// let value = Math::floor(2.9);
	/// assert_eq!(2.0, value);
	/// let value = Math::floor(-4.9);
	/// assert_eq!(-5.0, value);
	/// let value = Math::floor(-5.3);
	/// assert_eq!(-6.0, value);
	/// ```
	pub fn floor(value: f32) -> f32 {
		let trunc = Math::trunc(value);
		
		if trunc == value { return trunc; }
		
		return trunc - if value < 0.0 { 1.0 } else { 0.0 };
	}
	
	/// Gets the sign (positive or negative) of the given value
	/// - **value**: The value to check the sign with
	/// 
	/// **Returns**: Returns 1.0 if the value is positive, and -1.0 if the value is negative
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::sign(10.0);
	/// assert_eq!(1.0, value);
	/// let value = Math::sign(-10.0);
	/// assert_eq!(-1.0, value);
	/// let value = Math::sign(-0.0);
	/// assert_eq!(1.0, value);
	/// ```
	pub fn sign(value: f32) -> f32 { if value < 0.0 { -1.0 } else { 1.0 } }
	
	/// Maps the value from one range into another range
	/// - **value**: The value to map
	/// - **in_range**: The starting input range to map from
	/// - **out_range**: The ending output range to map to
	/// 
	/// **Returns**: Returns the mapped value
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::map(1.5, 1.0..2.0, 1.0..2.0);
	/// assert_eq!(1.5, value);
	/// let value = Math::map(1.0, 0.0..10.0, 0.0..1.0);
	/// assert_eq!(0.1, value);
	/// let value = Math::map(11.0, 0.0..10.0, 0.0..1.0);
	/// assert_eq!(1.1, value);
	/// let value = Math::map(1.0, -10.0..10.0, 0.0..1.0);
	/// assert_eq!(0.55, value);
	/// let value = Math::map(-10.0, -100.0..-10.0, 10.0..100.0);
	/// assert_eq!(100.0, value);
	/// ```
	pub fn map(value: f32, in_range: Range<f32>, out_range: Range<f32>) -> f32 {
		return
			(value - in_range.start)
			* (out_range.end - out_range.start)
			/ (in_range.end - in_range.start)
			+ out_range.start;
	}
	
	/// Computes a smooth Hermite interpolation that returns a number between 0.0 and 1.0
	/// - **value**: The value for the interpolation, where `left_edge` &lt; `value` &lt; `right_edge`
	/// - **left_edge**: The leftmost edge to where 0.0 would start at
	/// - **right_edge**: The rightmost edge where 1.0 would start at
	/// 
	/// **Returns**: Returns a smooth Hermite interpolation that returns a number between 0.0 and 1.0
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::smoothstep(-1.0, 0.0, 1.5);
	/// assert_eq!(0.0, value);
	/// let value = Math::smoothstep(1.0, 0.0, 1.5);
	/// assert_eq!(0.7407408, value);
	/// let value = Math::smoothstep(2.0, 0.0, 1.5);
	/// assert_eq!(1.0, value);
	/// let value = Math::smoothstep(0.5, -1.0, 3.0);
	/// assert_eq!(0.31640625, value);
	/// ```
	pub fn smoothstep(value: f32, left_edge: f32, right_edge: f32) -> f32 {
		let y = Math::clamp((value - left_edge) / (right_edge - left_edge), 0.0, 1.0);
		
		return y * y * (3.0 - 2.0 * y);
	}
	
	/// Gets the minimum value between the two values
	/// - **a**: The first value to get the minimum value from
	/// - **b**: The second value to get the minimum value from
	/// 
	/// **Returns**: Returns the minimum number between the two values
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::min(-1.0, 1.0);
	/// assert_eq!(-1.0, value);
	/// let value = Math::min(-19.0, -19.1);
	/// assert_eq!(-19.1, value);
	/// ```
	pub fn min(a: f32, b: f32) -> f32 { a.min(b) }
	
	/// Gets the maximum value between the two values
	/// - **a**: The first value to get the maximum value from
	/// - **b**: The second value to get the maximum value from
	/// 
	/// **Returns**: Returns the maximum number between the two values
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::max(-1.0, 1.0);
	/// assert_eq!(1.0, value);
	/// let value = Math::max(-19.0, -19.1);
	/// assert_eq!(-19.0, value);
	/// ```
	pub fn max(a: f32, b: f32) -> f32 { a.max(b) }
	
	/// Clamps the value between the min and max values
	/// - **value**: The value to clamp with
	/// - **min**: The lower-bound minimum value to clamp to
	/// - **max**: The upper-bound maximum value to clamp to
	/// 
	/// **Returns**: Returns the clamped value
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::clamp(20.0, 0.0, 10.0);
	/// assert_eq!(10.0, value);
	/// let value = Math::clamp(20.0, 0.0, 100.0);
	/// assert_eq!(20.0, value);
	/// let value = Math::clamp(-0.001, 0.0, 10.0);
	/// assert_eq!(0.0, value);
	/// let value = Math::clamp(0.18, -0.1, 0.1);
	/// assert_eq!(0.1, value);
	/// ```
	pub fn clamp(value: f32, min: f32, max: f32) -> f32 { value.clamp(min, max) }
	
	/// Linearly interpolates between the first and second values
	/// - **a**: The first value to start from
	/// - **b**: The second value to end from
	/// - **t**: The ratio value to interpolate between both values. Clamped between 0.0 and 1.0
	/// 
	/// **Returns**: Returns the interpolated value
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::lerp(0.0, 1.0, 0.5);
	/// assert_eq!(0.5, value);
	/// let value = Math::lerp(0.0, 0.1, 0.9);
	/// assert_eq!(0.089999996, value);
	/// let value = Math::lerp(-10.0, 10.0, 0.6);
	/// assert_eq!(2.0, value);
	/// let value = Math::lerp(-10.0, -4.0, 0.7);
	/// assert_eq!(-5.8, value);
	/// ```
	pub fn lerp(a: f32, b: f32, t: f32) -> f32 { Math::lerp_unclamped(a, b, Math::clamp(t, 0.0, 1.0)) }
	
	/// Linearly interpolates between the first and second values (not clamped)
	/// - **a**: The first value to start from
	/// - **b**: The second value to end from
	/// - **t**: The ratio value to interpolate between both values
	/// 
	/// **Returns**: Returns the interpolated value
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::lerp(0.0, 1.0, 0.5);
	/// assert_eq!(0.5, value);
	/// let value = Math::lerp(0.0, 0.1, 0.9);
	/// assert_eq!(0.089999996, value);
	/// let value = Math::lerp(-10.0, 10.0, 0.6);
	/// assert_eq!(2.0, value);
	/// let value = Math::lerp(-10.0, -4.0, 0.7);
	/// assert_eq!(-5.8, value);
	/// ```
	pub fn lerp_unclamped(a: f32, b: f32, t: f32) -> f32 { a + t * (b - a) }
	
	/// Gets the absolute value of the number
	/// - **value**: The number to get the absolute value from
	/// 
	/// **Returns**: Returns the absolute value of the number
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::abs(10.0);
	/// assert_eq!(10.0, value);
	/// let value = Math::abs(-10.0);
	/// assert_eq!(10.0, value);
	/// let value = Math::abs(-0.0);
	/// assert_eq!(0.0, value);
	/// ```
	pub fn abs(value: f32) -> f32 { if value < 0.0 { -value } else { value } }
	
	/// Gets the absolute value of the number
	/// - **value**: The number to get the absolute value from
	/// 
	/// **Returns**: Returns the absolute value of the number
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::abs_i32(10);
	/// assert_eq!(10, value);
	/// let value = Math::abs_i32(-10);
	/// assert_eq!(10, value);
	/// let value = Math::abs_i32(-0);
	/// assert_eq!(0, value);
	/// ```
	pub fn abs_i32(value: i32) -> i32 { if value < 0 { -value } else { value } }
	
	/// Truncates the value of the floating point number
	/// - **value**: The number to truncate
	/// 
	/// **Returns**: Returns the truncated number
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::trunc(123.456);
	/// assert_eq!(123.0, value);
	/// let value = Math::trunc(-5.4);
	/// assert_eq!(-5.0, value);
	/// let value = Math::trunc(6.0);
	/// assert_eq!(6.0, value);
	/// let value = Math::trunc(-0.0);
	/// assert_eq!(0.0, value);
	/// ```
	pub fn trunc(value: f32) -> f32 { (value as i32) as f32 }
	
	/// Gets the square root of the given number
	/// - **value**: The number to square root
	/// 
	/// **Returns**: Returns the square root of the number, returns NaN if `value` is negative
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::sqrt(16.0);
	/// assert_eq!(4.0, value);
	/// let value = Math::sqrt(1023.835);
	/// assert_eq!(31.9974217711, value);
	/// let value = Math::sqrt(-102.0);
	/// assert_eq!(true, f32::is_nan(value));
	/// let value = Math::sqrt(-0.0);
	/// assert_eq!(0.0, value);
	/// ```
	pub fn sqrt(value: f32) -> f32 {
		if value < -0.0 { return f32::NAN; }
		if value == 0.0 { return 0.0; }
		
		let mut max = 50;
		let mut x = value;
		
		while max > 0 && (value - x * x) <= 0.000001 {
			x = (x + value / x) / 2.0;
			if value - x * x == 0.0 { break; }
			max -= 1;
		}
		
		return x;
	}
}

#[cfg(not(feature = "no_std"))]
impl Math {
	/// Gets the power of the given number by the other given number, with the power being an `i32`
	/// - **a**: The base number to power
	/// - **b**: The number to power with
	/// 
	/// **Returns**: Returns the powered number
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::pow_i32(3.0, 5);
	/// assert_eq!(243.0, value);
	/// let value = Math::pow_i32(10.45, 3);
	/// assert_eq!(1141.166125, value);
	/// let value = Math::pow_i32(0.0, 0);
	/// assert_eq!(1.0, value);
	/// let value = Math::pow_i32(10.0, 0);
	/// assert_eq!(1.0, value);
	/// let value = Math::pow_i32(0.0, 2);
	/// assert_eq!(0.0, value);
	/// let value = Math::pow_i32(2.0, -3);
	/// assert_eq!(0.125, value);
	/// ```
	pub fn pow_i32(a: f32, b: i32) -> f32 {
		if b == 0 { return 1.0 }
		
		let mut result = a;
		
		for _ in 1..Math::abs_i32(b) {
			result *= a;
		}
		
		if b < 0 { 1.0 / result }
		else { result }
	}
	
	/// Computes the cos and sin of the angle
	/// - **angle**: The angle to compute the sine and cosine with
	/// 
	/// **Returns**: Returns the sine and cosine (respectively) as a tuple
	/// #### Remarks
	/// If you need to compute both `cos` and `sin` of the same angle, this function is more
	/// performant to produce both values than calling `cos` and `sin` separately
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range_tuple2};
	/// let value = Math::sin_cos(0.0);
	/// assert_range_tuple2!((0.0, 1.0), value);
	/// let value = Math::sin_cos(Math::PI_OVER_2);
	/// assert_range_tuple2!((1.0, 0.0), value);
	/// let value = Math::sin_cos(Math::PI);
	/// assert_range_tuple2!((0.0, -1.0), value);
	/// let value = Math::sin_cos(Math::PI + Math::PI_OVER_2);
	/// assert_range_tuple2!((-1.0, 0.0), value);
	/// let value = Math::sin_cos(Math::TWO_PI);
	/// assert_range_tuple2!((0.0, 1.0), value);
	/// let value = Math::sin_cos(Math::PI_OVER_2 * 0.5);
	/// assert_range_tuple2!((0.707106781, 0.707106781), value);
	/// let value = Math::sin_cos(1.0);
	/// assert_range_tuple2!((0.841470985, 0.540302306), value);
	/// let value = Math::sin_cos(-100.0);
	/// assert_range_tuple2!((0.506365641, 0.862318872), value);
	/// ```
	pub fn sin_cos(angle: f32) -> (f32, f32) { angle.sin_cos() }
	
	/// Computes the sine of the given angle
	/// - **angle**: The angle to compute sine with in radians
	/// 
	/// **Returns**: Returns a value from the computed sine
	/// #### Remarks
	/// If you need to compute both `cos` and `sin` of the same angle, use `sin_cos` instead as it's more
	/// performant to produce both values than calling `cos` and `sin` separately
	/// ##### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::sin(0.0);
	/// assert_range!(0.0, value);
	/// let value = Math::sin(Math::PI_OVER_2);
	/// assert_range!(1.0, value);
	/// let value = Math::sin(Math::PI);
	/// assert_range!(0.0, value);
	/// let value = Math::sin(Math::PI + Math::PI_OVER_2);
	/// assert_range!(-1.0, value);
	/// let value = Math::sin(Math::TWO_PI);
	/// assert_range!(0.0, value);
	/// let value = Math::sin(Math::PI_OVER_2 * 0.5);
	/// assert_range!(0.707106781, value);
	/// let value = Math::sin(1.0);
	/// assert_range!(0.841470985, value);
	/// let value = Math::sin(-100.0);
	/// assert_range!(0.506365641, value);
	/// ```
	pub fn sin(angle: f32) -> f32 { angle.sin() }
	
	/// Computes the cos of the given angle
	/// - **angle**: The angle to compute cosine with in radians
	/// 
	/// **Returns**: Returns a value from the computed cosine
	/// #### Remarks
	/// If you need to compute both `cos` and `sin` of the same angle, use `sin_cos` instead as it's more
	/// performant to produce both values than calling `cos` and `sin` separately
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::cos(0.0);
	/// assert_range!(1.0, value);
	/// let value = Math::cos(Math::PI_OVER_2);
	/// assert_range!(0.0, value);
	/// let value = Math::cos(Math::PI);
	/// assert_range!(-1.0, value);
	/// let value = Math::cos(Math::PI + Math::PI_OVER_2);
	/// assert_range!(0.0, value);
	/// let value = Math::cos(Math::TWO_PI);
	/// assert_range!(1.0, value);
	/// let value = Math::cos(Math::PI_OVER_2 * 0.5);
	/// assert_range!(0.707106781, value);
	/// let value = Math::cos(1.0);
	/// assert_range!(0.540302306, value);
	/// let value = Math::cos(-100.0);
	/// assert_range!(0.862318872, value);
	/// ```
	pub fn cos(angle: f32) -> f32 { angle.cos() }
	
	/// Finds if the two floating point numbers are approximately close to each other
	/// - **a**: The first number to check with
	/// - **b**: The second number to check with
	/// 
	/// **Returns**: Returns true if the two values are approximately close to each other
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// assert!(Math::approx(1.20000001, 1.2));
	/// ```
	pub fn approx(a: f32, b: f32) -> bool {
		Math::approx_epsilon(a, b, 0.00001)
	}
	
	/// Finds if the two floating point numbers are approximately close to each other, provided the epsilon
	/// - **a**: The first number to check with
	/// - **b**: The second number to check with
	/// - **epsilon**: The epsilon (smallest possible difference between numbers) to check with
	/// 
	/// **Returns**: Returns true if the two values are approximately close to each other
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// assert!(Math::approx_epsilon(1.2001, 1.2, 0.001));
	/// ```
	pub fn approx_epsilon(a: f32, b: f32, epsilon: f32) -> bool {
		Math::abs(a - b) < epsilon
	}
	
	/// Gets the fractional part of the value, getting only a value between 0 and 1
	/// - **value**: The value to get the fraction from
	/// 
	/// **Returns**: Returns the fraction of the given number
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::frac(3.0);
	/// assert_eq!(0.0, value);
	/// let value = Math::frac(-3.0);
	/// assert_eq!(0.0, value);
	/// let value = Math::frac(4.9);
	/// assert!((0.9..0.90001).contains(&value));
	/// let value = Math::frac(-4.9);
	/// assert!((0.0999999..0.1).contains(&value));
	/// let value = Math::frac(12.34);
	/// assert!((0.34..0.340001).contains(&value));
	/// ```
	pub fn frac(value: f32) -> f32 { value - Math::floor(value) }
	
	/// Gets the smallest integer number that is greater than or equal to the given number
	/// - **value**: The value to get the ceiling with
	/// 
	/// **Returns**: Returns the ceiling number
	/// #### Example
	/// ```
	/// # use mathx::Math;
	/// let value = Math::ceil(-3.0);
	/// assert_eq!(-3.0, value);
	/// let value = Math::ceil(1.4);
	/// assert_eq!(2.0, value);
	/// let value = Math::ceil(2.9);
	/// assert_eq!(3.0, value);
	/// let value = Math::ceil(-4.9);
	/// assert_eq!(-4.0, value);
	/// let value = Math::ceil(-5.3);
	/// assert_eq!(-5.0, value);
	/// ```
	pub fn ceil(value: f32) -> f32 { value.ceil() }
	
	/// Gets the largest integer number that is less than or equal to the given number
	/// - **value**: The value to get the floor with
	/// 
	/// **Returns**: Returns the floored number
	/// #### Example
	/// ```
	/// # use mathx::Math;
	/// let value = Math::floor(-3.0);
	/// assert_eq!(-3.0, value);
	/// let value = Math::floor(1.4);
	/// assert_eq!(1.0, value);
	/// let value = Math::floor(2.9);
	/// assert_eq!(2.0, value);
	/// let value = Math::floor(-4.9);
	/// assert_eq!(-5.0, value);
	/// let value = Math::floor(-5.3);
	/// assert_eq!(-6.0, value);
	/// ```
	pub fn floor(value: f32) -> f32 { value.floor() }
	
	/// Gets the sign (positive or negative) of the given value
	/// - **value**: The value to check the sign with
	/// 
	/// **Returns**: Returns 1.0 if the value is positive, and -1.0 if the value is negative
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::sign(10.0);
	/// assert_eq!(1.0, value);
	/// let value = Math::sign(-10.0);
	/// assert_eq!(-1.0, value);
	/// let value = Math::sign(-0.0);
	/// assert_eq!(1.0, value);
	/// ```
	pub fn sign(value: f32) -> f32 { if value < 0.0 { -1.0 } else { 1.0 } }
	
	/// Maps the value from one range into another range
	/// - **value**: The value to map
	/// - **in_range**: The starting input range to map from
	/// - **out_range**: The ending output range to map to
	/// 
	/// **Returns**: Returns the mapped value
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::map(1.5, 1.0..2.0, 1.0..2.0);
	/// assert_eq!(1.5, value);
	/// let value = Math::map(1.0, 0.0..10.0, 0.0..1.0);
	/// assert_eq!(0.1, value);
	/// let value = Math::map(11.0, 0.0..10.0, 0.0..1.0);
	/// assert_eq!(1.1, value);
	/// let value = Math::map(1.0, -10.0..10.0, 0.0..1.0);
	/// assert_eq!(0.55, value);
	/// let value = Math::map(-10.0, -100.0..-10.0, 10.0..100.0);
	/// assert_eq!(100.0, value);
	/// ```
	pub fn map(value: f32, in_range: Range<f32>, out_range: Range<f32>) -> f32 {
		return
			(value - in_range.start)
			* (out_range.end - out_range.start)
			/ (in_range.end - in_range.start)
			+ out_range.start;
	}
	
	/// Computes a smooth Hermite interpolation that returns a number between 0.0 and 1.0
	/// - **value**: The value for the interpolation, where `left_edge` &lt; `value` &lt; `right_edge`
	/// - **left_edge**: The leftmost edge to where 0.0 would start at
	/// - **right_edge**: The rightmost edge where 1.0 would start at
	/// 
	/// **Returns**: Returns a smooth Hermite interpolation that returns a number between 0.0 and 1.0
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::smoothstep(-1.0, 0.0, 1.5);
	/// assert_eq!(0.0, value);
	/// let value = Math::smoothstep(1.0, 0.0, 1.5);
	/// assert_eq!(0.7407408, value);
	/// let value = Math::smoothstep(2.0, 0.0, 1.5);
	/// assert_eq!(1.0, value);
	/// let value = Math::smoothstep(0.5, -1.0, 3.0);
	/// assert_eq!(0.31640625, value);
	/// ```
	pub fn smoothstep(value: f32, left_edge: f32, right_edge: f32) -> f32 {
		let y = Math::clamp((value - left_edge) / (right_edge - left_edge), 0.0, 1.0);
		
		return y * y * (3.0 - 2.0 * y);
	}
	
	/// Gets the minimum value between the two values
	/// - **a**: The first value to get the minimum value from
	/// - **b**: The second value to get the minimum value from
	/// 
	/// **Returns**: Returns the minimum number between the two values
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::min(-1.0, 1.0);
	/// assert_eq!(-1.0, value);
	/// let value = Math::min(-19.0, -19.1);
	/// assert_eq!(-19.1, value);
	/// ```
	pub fn min(a: f32, b: f32) -> f32 { a.min(b) }
	
	/// Gets the maximum value between the two values
	/// - **a**: The first value to get the maximum value from
	/// - **b**: The second value to get the maximum value from
	/// 
	/// **Returns**: Returns the maximum number between the two values
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::max(-1.0, 1.0);
	/// assert_eq!(1.0, value);
	/// let value = Math::max(-19.0, -19.1);
	/// assert_eq!(-19.0, value);
	/// ```
	pub fn max(a: f32, b: f32) -> f32 { a.max(b) }
	
	/// Clamps the value between the min and max values
	/// - **value**: The value to clamp with
	/// - **min**: The lower-bound minimum value to clamp to
	/// - **max**: The upper-bound maximum value to clamp to
	/// 
	/// **Returns**: Returns the clamped value
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::clamp(20.0, 0.0, 10.0);
	/// assert_eq!(10.0, value);
	/// let value = Math::clamp(20.0, 0.0, 100.0);
	/// assert_eq!(20.0, value);
	/// let value = Math::clamp(-0.001, 0.0, 10.0);
	/// assert_eq!(0.0, value);
	/// let value = Math::clamp(0.18, -0.1, 0.1);
	/// assert_eq!(0.1, value);
	/// ```
	pub fn clamp(value: f32, min: f32, max: f32) -> f32 { value.clamp(min, max) }
	
	/// Linearly interpolates between the first and second values (not clamped)
	/// - **a**: The first value to start from
	/// - **b**: The second value to end from
	/// - **t**: The ratio value to interpolate between both values
	/// 
	/// **Returns**: Returns the interpolated value
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::lerp(0.0, 1.0, 0.5);
	/// assert_eq!(0.5, value);
	/// let value = Math::lerp(0.0, 0.1, 0.9);
	/// assert_eq!(0.089999996, value);
	/// let value = Math::lerp(-10.0, 10.0, 0.6);
	/// assert_eq!(2.0, value);
	/// let value = Math::lerp(-10.0, -4.0, 0.7);
	/// assert_eq!(-5.8, value);
	/// ```
	pub fn lerp(a: f32, b: f32, t: f32) -> f32 { Math::lerp_unclamped(a, b, Math::clamp(t, 0.0, 1.0)) }
	
	/// Linearly interpolates between the first and second values (not clamped)
	/// - **a**: The first value to start from
	/// - **b**: The second value to end from
	/// - **t**: The ratio value to interpolate between both values
	/// 
	/// **Returns**: Returns the interpolated value
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::lerp(0.0, 1.0, 0.5);
	/// assert_eq!(0.5, value);
	/// let value = Math::lerp(0.0, 0.1, 0.9);
	/// assert_eq!(0.089999996, value);
	/// let value = Math::lerp(-10.0, 10.0, 0.6);
	/// assert_eq!(2.0, value);
	/// let value = Math::lerp(-10.0, -4.0, 0.7);
	/// assert_eq!(-5.8, value);
	/// ```
	pub fn lerp_unclamped(a: f32, b: f32, t: f32) -> f32 { a + t * (b - a) }
	
	/// Gets the absolute value of the number
	/// - **value**: The number to get the absolute value from
	/// 
	/// **Returns**: Returns the absolute value of the number
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::abs(10.0);
	/// assert_eq!(10.0, value);
	/// let value = Math::abs(-10.0);
	/// assert_eq!(10.0, value);
	/// let value = Math::abs(-0.0);
	/// assert_eq!(0.0, value);
	/// ```
	pub fn abs(value: f32) -> f32 { value.abs() }
	
	/// Gets the absolute value of the number
	/// - **value**: The number to get the absolute value from
	/// 
	/// **Returns**: Returns the absolute value of the number
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::abs_i32(10);
	/// assert_eq!(10, value);
	/// let value = Math::abs_i32(-10);
	/// assert_eq!(10, value);
	/// let value = Math::abs_i32(-0);
	/// assert_eq!(0, value);
	/// ```
	pub fn abs_i32(value: i32) -> i32 { value.abs() }
	
	/// Truncates the value of the floating point number
	/// - **value**: The number to truncate
	/// 
	/// **Returns**: Returns the truncated number
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::trunc(123.456);
	/// assert_eq!(123.0, value);
	/// let value = Math::trunc(-5.4);
	/// assert_eq!(-5.0, value);
	/// let value = Math::trunc(6.0);
	/// assert_eq!(6.0, value);
	/// let value = Math::trunc(-0.0);
	/// assert_eq!(0.0, value);
	/// ```
	pub fn trunc(value: f32) -> f32 { value.trunc() }
	
	/// Gets the square root of the given number
	/// - **value**: The number to square root
	/// 
	/// **Returns**: Returns the square root of the number, returns NaN if `value` is negative
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::sqrt(16.0);
	/// assert_eq!(4.0, value);
	/// let value = Math::sqrt(1023.835);
	/// assert_eq!(31.9974217711, value);
	/// let value = Math::sqrt(-102.0);
	/// assert_eq!(true, f32::is_nan(value));
	/// let value = Math::sqrt(-0.0);
	/// assert_eq!(0.0, value);
	/// ```
	pub fn sqrt(value: f32) -> f32 { value.sqrt() }
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_range {
	($expected:expr, $value:expr) => {
		if !Math::approx_epsilon($expected, $value, 0.0001) { panic!("\n\nleft: {:?}\nright: {:?}\n\n", $expected, $value); }
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_range_tuple2 {
	($expected:expr, $value:expr) => {
		if !Math::approx_epsilon($expected.0, $value.0, 0.0001) || !Math::approx_epsilon($expected.1, $value.1, 0.0001) { panic!("\n\nleft: {:?}\nright: {:?}\n\n", $expected, $value); }
	};
}
