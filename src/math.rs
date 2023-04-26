
use core::ops::Range;

/// A "static" structure used to compute math functions. Since `f32` gets a lot of it's
/// functions stripped away when using `no_std`, you can use this structure to regain
/// those functions. It will also work the same even if you don't use it for `no_std`.
pub struct Math;

// Constants
impl Math {
	pub const PI: f32 = 3.14159265359;
	pub const PI_OVER_2: f32 = 1.570796326;
	pub const PI_OVER_4: f32 = 0.785398163;
	pub const TWO_PI: f32 = 6.28318530718;
	pub const E: f32 = 2.71828182845;
	pub const DEG_TO_RAD: f32 = 0.01745329251;
	pub const RAD_TO_DEG: f32 = 57.2957795131;
	pub const LN2: f32 = 0.69314718056;
	pub const LN10: f32 = 2.30258509299;
}

// Public Functions
impl Math {
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
	pub fn abs(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.abs() }
		#[cfg(feature = "no_std")] {
			if value < 0.0 { -value } else { value }
		}
	}
	
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
	pub fn abs_i32(value: i32) -> i32 {
		#[cfg(not(feature = "no_std"))] { value.abs() }
		#[cfg(feature = "no_std")] {
			if value < 0 { -value } else { value }
		}
	}
	
	/// Finds if the two floating point numbers are approximately close to each other. Checks with epsilon = 0.000001
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
	
	/// Computes the arc cosine (a.k.a. inverse cosine) with the provided value
	/// - **value**: The value to compute the arc cosine with, must be within -1 and 1
	/// 
	/// **Returns**: Returns the angle at which the value exists in radians,
	/// returns `NaN` if the value provided is less than -1 or greater than 1
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::acos(0.0);
	/// assert_range!(Math::PI_OVER_2, value);
	/// let value = Math::acos(1.0);
	/// assert_range!(0.0, value);
	/// let value = Math::acos(-1.0);
	/// assert_range!(Math::PI, value);
	/// let value = Math::acos(0.707106781);
	/// assert_range!(Math::PI_OVER_4, value);
	/// let value = Math::acos(0.540302306);
	/// assert_range!(1.0, value);
	/// let value = Math::acos(2.0);
	/// assert!(value.is_nan());
	/// let value = Math::acos(-1.001);
	/// assert!(value.is_nan());
	/// ```
	pub fn acos(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.acos() }
		#[cfg(feature = "no_std")] {
			if value < -1.0 || value > 1.0 { return f32::NAN; }
			
			let negate = if value <= -0.0 { 1.0 } else { 0.0 };
			let value = Math::abs(value);
			let mut angle = -0.0187293;
			
			angle *= value;
			angle += 0.0742610;
			angle *= value;
			angle -= 0.2121144;
			angle *= value;
			angle += Math::PI_OVER_2;
			angle *= Math::sqrt(1.0 - value);
			angle -= 2.0 * negate * angle;
			
			return negate * Math::PI + angle;
		}
	}
	
	/// Computes the arc cosine (a.k.a. inverse cosine) with the provided value
	/// - **value**: The value to compute the arc cosine with, must be within -1 and 1
	/// 
	/// **Returns**: Returns the angle at which the value exists in degrees,
	/// returns `NaN` if the value provided is less than -1 or greater than 1
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::acos_deg(0.0);
	/// assert_range!(90.0, value);
	/// let value = Math::acos_deg(1.0);
	/// assert_range!(0.0, value);
	/// let value = Math::acos_deg(-1.0);
	/// assert_range!(180.0, value);
	/// let value = Math::acos_deg(0.707106781);
	/// assert_range!(45.0, value, 0.005);
	/// let value = Math::acos_deg(0.540302306);
	/// assert_range!(57.29578, value, 0.001);
	/// let value = Math::acos_deg(2.0);
	/// assert!(value.is_nan());
	/// let value = Math::acos_deg(-1.001);
	/// assert!(value.is_nan());
	/// ```
	pub fn acos_deg(value: f32) -> f32 { Math::RAD_TO_DEG * Math::acos(value) }
	
	/// Computes the arc hyperbolic cosine (a.k.a. inverse hyperbolic cosine)
	/// - **value**: The value to compute with
	/// 
	/// **Returns**: Returns the computed inverse hyperbolic cosine
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::acosh(0.0);
	/// assert!(value.is_nan());
	/// let value = Math::acosh(1.0);
	/// assert_range!(0.0, value);
	/// let value = Math::acosh(1.54308063482);
	/// assert_range!(1.0, value);
	/// let value = Math::acosh(11.591954);
	/// assert_range!(Math::PI, value);
	/// let value = Math::acosh(7.6101246);
	/// assert_range!(Math::E, value);
	/// ```
	pub fn acosh(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.acosh() }
		#[cfg(feature = "no_std")] {
			if value < 1.0 { return f32::NAN; }
			Math::ln(value + Math::sqrt(value * value - 1.0))
		}
	}
	
	/// Computes the arc sine (a.k.a. inverse sine) with the provided value
	/// - **value**: The value to compute the arc sine with, must be within -1 and 1
	/// 
	/// **Returns**: Returns the angle at which the value exists in radians,
	/// returns `NaN` if the value provided is less than -1 or greater than 1
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::asin(0.0);
	/// assert_range!(0.0, value);
	/// let value = Math::asin(1.0);
	/// assert_range!(Math::PI_OVER_2, value);
	/// let value = Math::asin(-1.0);
	/// assert_range!(-Math::PI_OVER_2, value);
	/// let value = Math::asin(0.707106781);
	/// assert_range!(Math::PI_OVER_4, value);
	/// let value = Math::asin(-1.1);
	/// assert!(value.is_nan());
	/// let value = Math::asin(2.0);
	/// assert!(value.is_nan());
	/// let value = Math::asin(0.9999);
	/// assert_range!(1.5566529, value);
	/// let value = Math::asin(-0.25);
	/// assert_range!(-0.25268024, value);
	/// ```
	pub fn asin(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.asin() }
		#[cfg(feature = "no_std")] {
			if value < -1.0 || value > 1.0 { return f32::NAN; }
			
			let negate = if value < 0.0 { 1.0 } else { 0.0 };
			let value = Math::abs(value);
			let mut angle = -0.0187293;
			
			angle *= value;
			angle += 0.0742610;
			angle *= value;
			angle -= 0.2121144;
			angle *= value;
			angle += Math::PI_OVER_2;
			angle = Math::PI * 0.5 - Math::sqrt(1.0 - value) * angle;
			
			return angle - 2.0 * negate * angle;
		}
	}
	
	/// Computes the arc sine (a.k.a. inverse sine) with the provided value
	/// - **value**: The value to compute the arc sine with, must be within -1 and 1
	/// 
	/// **Returns**: Returns the angle at which the value exists in degrees,
	/// returns `NaN` if the value provided is less than -1 or greater than 1
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::asin_deg(0.0);
	/// assert_range!(0.0, value);
	/// let value = Math::asin_deg(1.0);
	/// assert_range!(90.0, value);
	/// let value = Math::asin_deg(-1.0);
	/// assert_range!(-90.0, value);
	/// let value = Math::asin_deg(0.707106781);
	/// assert_range!(45.0, value, 0.005);
	/// let value = Math::asin_deg(-1.1);
	/// assert!(value.is_nan());
	/// let value = Math::asin_deg(2.0);
	/// assert!(value.is_nan());
	/// let value = Math::asin_deg(0.9999);
	/// assert_range!(89.189644, value);
	/// let value = Math::asin_deg(-0.25);
	/// assert_range!(-14.477511, value, 0.005);
	/// ```
	pub fn asin_deg(value: f32) -> f32 { Math::RAD_TO_DEG * Math::asin(value) }
	
	/// Computes the arc hyperbolic sine (a.k.a. inverse hyperbolic sine)
	/// - **value**: The value to compute with
	/// 
	/// **Returns**: Returns the computed inverse hyperbolic sine
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::asinh(0.0);
	/// assert_range!(0.0, value);
	/// let value = Math::asinh(1.0);
	/// assert_range!(0.8813736, value);
	/// let value = Math::asinh(1.1752012);
	/// assert_range!(1.0, value);
	/// let value = Math::asinh(-1.1752012);
	/// assert_range!(-1.0, value);
	/// let value = Math::asinh(11.54874);
	/// assert_range!(Math::PI, value);
	/// let value = Math::asinh(7.5441365);
	/// assert_range!(Math::E, value);
	/// ```
	pub fn asinh(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.asinh() }
		#[cfg(feature = "no_std")] {
			Math::ln(value + Math::sqrt(value * value + 1.0))
		}
	}
	
	/// Computes the arc tangent (a.k.a. inverse tangent) with the provided value
	/// - **value**: The value to compute the arc tangent with
	/// 
	/// **Returns**: Returns the angle at which the value exists in radians
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::atan(0.0);
	/// assert_range!(0.0, value);
	/// let value = Math::atan(1.0);
	/// assert_range!(Math::PI_OVER_4, value);
	/// let value = Math::atan(-1.0);
	/// assert_range!(-Math::PI_OVER_4, value);
	/// let value = Math::atan(0.707106781);
	/// assert_range!(0.615479708546, value);
	/// let value = Math::atan(1.557407725);
	/// assert_range!(1.0, value);
	/// ```
	pub fn atan(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.atan() }
		#[cfg(feature = "no_std")] {
			Math::atan2(value, 1.0)
		}
	}
	
	/// Computes the arc tangent (a.k.a. inverse tangent) with the provided value
	/// - **value**: The value to compute the arc tangent with
	/// 
	/// **Returns**: Returns the angle at which the value exists in degrees
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::atan_deg(0.0);
	/// assert_range!(0.0, value);
	/// let value = Math::atan_deg(1.0);
	/// assert_range!(45.0, value, 0.001);
	/// let value = Math::atan_deg(-1.0);
	/// assert_range!(-45.0, value, 0.001);
	/// let value = Math::atan_deg(0.707106781);
	/// assert_range!(35.26439, value, 0.001);
	/// let value = Math::atan_deg(1.557407725);
	/// assert_range!(57.29578, value);
	/// ```
	pub fn atan_deg(value: f32) -> f32 { Math::RAD_TO_DEG * Math::atan(value) }
	
	/// Computes the arc hyperbolic tangent (a.k.a. inverse hyperbolic tangent)
	/// - **value**: The value to compute with
	/// 
	/// **Returns**: Returns the computed inverse hyperbolic tangent
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::atanh(0.0);
	/// assert_range!(0.0, value);
	/// let value = Math::atanh(1.0);
	/// assert!(value.is_infinite());
	/// let value = Math::atanh(0.7615942);
	/// assert_range!(1.0, value, 0.001);
	/// let value = Math::atanh(-0.7615942);
	/// assert_range!(-1.0, value, 0.001);
	/// let value = Math::atanh(0.9962721);
	/// assert_range!(Math::PI, value);
	/// let value = Math::atanh(0.9913289);
	/// assert_range!(Math::E, value);
	/// ```
	pub fn atanh(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.atanh() }
		#[cfg(feature = "no_std")] {
			if value >= 1.0 { return f32::INFINITY; }
			if value <= -1.0 { return f32::NEG_INFINITY; }
			0.5 * Math::ln((1.0 + value) * (1.0 - value).recip())
		}
	}
	
	/// Computes the arc tangent (a.k.a. inverse tangent) with the provided x and y values
	/// - **y**: The y value to compute the arc tangent with
	/// - **x**: The x value to compute the arc tangent with
	/// 
	/// **Returns**: Returns the angle at with the two values divided exists in radians
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::atan2(0.0, 1.0);
	/// assert_range!(0.0, value);
	/// let value = Math::atan2(1.0, 1.0);
	/// assert_range!(Math::PI_OVER_4, value);
	/// let value = Math::atan2(-1.0, 1.0);
	/// assert_range!(-Math::PI_OVER_4, value);
	/// let value = Math::atan2(5.0, 1.0);
	/// assert_range!(1.3734008, value);
	/// let value = Math::atan2(1.0, 5.0);
	/// assert_range!(0.19739556, value);
	/// let value = Math::atan2(-5.0, 1.0);
	/// assert_range!(-1.3734008, value);
	/// let value = Math::atan2(-1.0, 5.0);
	/// assert_range!(-0.19739556, value);
	/// ```
	pub fn atan2(y: f32, x: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { y.atan2(x) }
		#[cfg(feature = "no_std")] {
			let mut a = Math::abs(x);
			let mut b = Math::abs(y);
			let mut c = Math::max(a, b);
			b = Math::min(a, b);
			a = c.recip();
			a = b * a;
		  
			let d = a * a;
			c = -0.013480470;
			c = c * d + 0.057477314;
			c = c * d - 0.121239071;
			c = c * d + 0.195635925;
			c = c * d - 0.332994597;
			c = c * d + 0.999995630;
			a *= c;
			
			if Math::abs(y) > Math::abs(x) { a = Math::PI_OVER_2 - a; }
			if x < 0.0 { a = Math::PI - a; }
			if y < 0.0 { a *= -1.0; }
			
			return a;
		}
	}
	
	/// Computes the arc tangent (a.k.a. inverse tangent) with the provided x and y values
	/// - **y**: The y value to compute the arc tangent with
	/// - **x**: The x value to compute the arc tangent with
	/// 
	/// **Returns**: Returns the angle at with the two values divided exists in degrees
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::atan2_deg(0.0, 1.0);
	/// assert_range!(0.0, value);
	/// let value = Math::atan2_deg(1.0, 1.0);
	/// assert_range!(45.0, value, 0.005);
	/// let value = Math::atan2_deg(-1.0, 1.0);
	/// assert_range!(-45.0, value, 0.005);
	/// let value = Math::atan2_deg(5.0, 1.0);
	/// assert_range!(78.69007, value);
	/// let value = Math::atan2_deg(1.0, 5.0);
	/// assert_range!(11.309933, value);
	/// let value = Math::atan2_deg(-5.0, 1.0);
	/// assert_range!(-78.69007, value);
	/// let value = Math::atan2_deg(-1.0, 5.0);
	/// assert_range!(-11.309933, value);
	/// ```
	pub fn atan2_deg(y: f32, x: f32) -> f32 { Math::RAD_TO_DEG * Math::atan2(y, x) }
	
	/// Gets the smallest integer number that is greater than or equal to the given number
	/// - **value**: The value to get the ceiling with
	/// 
	/// **Returns**: Returns the ceiling number
	/// #### Examples
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
		#[cfg(not(feature = "no_std"))] { value.ceil() }
		#[cfg(feature = "no_std")] {
			let truncated = Math::trunc(value);
			
			if truncated == value { return truncated; }
			
			return truncated + if value < 0.0 { 0.0 } else { 1.0 };
		}
	}
	
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
	
	/// Computes the cosine of the given angle in radians
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
	/// let value = Math::cos(Math::PI_OVER_4);
	/// assert_range!(0.707106781, value);
	/// let value = Math::cos(1.0);
	/// assert_range!(0.540302306, value);
	/// let value = Math::cos(-100.0);
	/// assert_range!(0.862318872, value);
	/// ```
	pub fn cos(angle: f32) -> f32 { Math::sin_cos(angle).1 }
	
	/// Computes the cosine of the given angle in degrees
	/// - **angle**: The angle to compute cosine with in degrees
	/// 
	/// **Returns**: Returns a value from the computed cosine
	/// #### Remarks
	/// If you need to compute both `cos_deg` and `sin_deg` of the same angle, use `sin_cos_deg` instead as it's more
	/// performant to produce both values than calling `cos_deg` and `sin_deg` separately
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::cos_deg(0.0);
	/// assert_range!(1.0, value);
	/// let value = Math::cos_deg(90.0);
	/// assert_range!(0.0, value);
	/// let value = Math::cos_deg(180.0);
	/// assert_range!(-1.0, value);
	/// let value = Math::cos_deg(270.0);
	/// assert_range!(0.0, value);
	/// let value = Math::cos_deg(360.0);
	/// assert_range!(1.0, value);
	/// let value = Math::cos_deg(45.0);
	/// assert_range!(0.707106781, value);
	/// let value = Math::cos_deg(57.29577951);
	/// assert_range!(0.540302306, value);
	/// let value = Math::cos_deg(-5729.577951);
	/// assert_range!(0.862318872, value);
	/// ```
	pub fn cos_deg(angle: f32) -> f32 { Math::cos(Math::DEG_TO_RAD * angle) }
	
	/// Computes the hyperbolic cosine function
	/// - **value**: The value to compute the hyperbolic cosine function
	/// 
	/// **Returns**: Returns the computed hyperbolic cosine function
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::cosh(0.0);
	/// assert_range!(1.0, value);
	/// let value = Math::cosh(1.0);
	/// assert_range!(1.54308063482, value);
	/// let value = Math::cosh(-1.0);
	/// assert_range!(1.54308063482, value);
	/// let value = Math::cosh(Math::PI);
	/// assert_range!(11.591954, value);
	/// let value = Math::cosh(Math::E);
	/// assert_range!(7.6101246, value);
	/// ```
	pub fn cosh(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.cosh() }
		#[cfg(feature = "no_std")] {
			let exp = Math::exp(value);
			
			if exp.is_infinite() || exp.is_nan() {
				if value > 0.0 { return f32::INFINITY; }
				else { return f32::NEG_INFINITY; }
			}
			
			(exp + exp.recip()) * 0.5
		}
	}
	
	/// Computes the cotangent of the given angle in radians
	/// - **angle**: The angle to compute the cotangent with in radians
	/// 
	/// **Returns**: Returns the computed cotangent value
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::cot(Math::PI_OVER_2);
	/// assert_range!(0.0, value);
	/// let value = Math::cot(Math::PI + Math::PI_OVER_2);
	/// assert_range!(0.0, value);
	/// let value = Math::cot(Math::PI_OVER_4);
	/// assert_range!(1.0, value);
	/// let value = Math::cot(1.0);
	/// assert_range!(0.642092616, value);
	/// let value = Math::cot(-100.0);
	/// assert_range!(1.702956919, value);
	/// ```
	pub fn cot(angle: f32) -> f32 { Math::tan(angle).recip() }
	
	/// Computes the cotangent of the given angle in degrees
	/// - **angle**: The angle to compute the cotangent with in degrees
	/// 
	/// **Returns**: Returns the computed cotangent value
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::cot_deg(90.0);
	/// assert_range!(0.0, value);
	/// let value = Math::cot_deg(270.0);
	/// assert_range!(0.0, value);
	/// let value = Math::cot_deg(45.0);
	/// assert_range!(1.0, value);
	/// let value = Math::cot_deg(57.29577951);
	/// assert_range!(0.642092616, value);
	/// let value = Math::cot_deg(-5729.577951);
	/// assert_range!(1.702956919, value);
	/// ```
	pub fn cot_deg(angle: f32) -> f32 { Math::cot(Math::DEG_TO_RAD * angle) }
	
	/// Computes the cosecant of the given angle in radians
	/// - **angle**: The angle to compute the cosecant with in radians
	/// 
	/// **Returns**: Returns the computed cosecant value
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::csc(Math::PI_OVER_2);
	/// assert_range!(1.0, value);
	/// let value = Math::csc(Math::PI + Math::PI_OVER_2);
	/// assert_range!(-1.0, value);
	/// let value = Math::csc(Math::PI_OVER_4);
	/// assert_range!(1.414213562, value);
	/// let value = Math::csc(1.0);
	/// assert_range!(1.188395106, value);
	/// let value = Math::csc(-100.0);
	/// assert_range!(1.974857531, value);
	/// ```
	pub fn csc(angle: f32) -> f32 { Math::sin(angle).recip() }
	
	/// Computes the cosecant of the given angle in degrees
	/// - **angle**: The angle to compute the cosecant with in degrees
	/// 
	/// **Returns**: Returns the computed cosecant value
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::csc_deg(90.0);
	/// assert_range!(1.0, value);
	/// let value = Math::csc_deg(270.0);
	/// assert_range!(-1.0, value);
	/// let value = Math::csc_deg(45.0);
	/// assert_range!(1.414213562, value);
	/// let value = Math::csc_deg(57.29577951);
	/// assert_range!(1.188395106, value);
	/// let value = Math::csc_deg(-5729.577951);
	/// assert_range!(1.974857531, value);
	/// ```
	pub fn csc_deg(angle: f32) -> f32 { Math::csc(Math::DEG_TO_RAD * angle) }
	
	/// Computes e^x
	/// - **value**: The value to compute with
	/// 
	/// **Returns**: Returns the computed e^x
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::exp(0.0);
	/// assert_range!(1.0, value);
	/// let value = Math::exp(-10.0);
	/// assert_range!(0.000004539993, value);
	/// let value = Math::exp(10.0);
	/// assert_range!(22026.465, value);
	/// let value = Math::exp(12.34);
	/// assert_range!(228661.98, value, 0.05);
	/// let value = Math::exp(2.9);
	/// assert_range!(18.174147, value);
	/// ```
	pub fn exp(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.exp() }
		#[cfg(feature = "no_std")] {
			if value < 0.0 { return Math::exp(-value).recip(); }
			
			let mut result = 1.0;
			let mut term = 1.0;
			let mut n = 1;
			
			while n <= 100 {
				term *= value / n as f32;
				result += term;
				n += 1;
			}
			
			return result;
		}
	}
	
	/// Computes 2^x
	/// - **value**: The value to compute with
	/// 
	/// **Returns**: Returns the computed 2^x
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::exp2(0.0);
	/// assert_range!(1.0, value);
	/// let value = Math::exp2(-10.0);
	/// assert_range!(0.0009765625, value);
	/// let value = Math::exp2(10.0);
	/// assert_range!(1024.0, value, 0.0002);
	/// let value = Math::exp2(12.34);
	/// assert_range!(5184.5396, value, 0.05);
	/// let value = Math::exp2(2.9);
	/// assert_range!(7.464265, value);
	/// ```
	pub fn exp2(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.exp2() }
		#[cfg(feature = "no_std")] {
			Math::exp(value * Math::LN2)
		}
	}
	
	/// Gets the largest integer number that is less than or equal to the given number
	/// - **value**: The value to get the floor with
	/// 
	/// **Returns**: Returns the floored number
	/// #### Examples
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
		#[cfg(not(feature = "no_std"))] { value.floor() }
		#[cfg(feature = "no_std")] {
			let truncated = Math::trunc(value);
			
			if truncated == value { return truncated; }
			
			return truncated - if value < 0.0 { 1.0 } else { 0.0 };
		}
	}
	
	/// Gets the fractional part of the value, getting only a value between 0 and 1
	/// - **value**: The value to get the fraction from
	/// 
	/// **Returns**: Returns the fraction of the given number
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::fract(3.0);
	/// assert_range!(0.0, value);
	/// let value = Math::fract(-3.0);
	/// assert_range!(0.0, value);
	/// let value = Math::fract(4.9);
	/// assert_range!(0.9, value);
	/// let value = Math::fract(-4.9);
	/// assert_range!(0.1, value);
	/// let value = Math::fract(12.34);
	/// assert_range!(0.34, value);
	/// ```
	pub fn fract(value: f32) -> f32 { value - Math::floor(value) }
	
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
	
	/// Computes the natural log of the given number
	/// - **value**: The value to compute the natural log of
	/// 
	/// **Returns**: Returns the natural log of the given value. Returns `infinity` if the value infinity
	/// and `-infinity` if the value is 0.0. Returns `NaN` if the value is `NaN` or less than 0.0
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::ln(1.0);
	/// assert_range!(0.0, value);
	/// let value = Math::ln(100.0);
	/// assert_range!(4.60517018599, value);
	/// let value = Math::ln(0.01);
	/// assert_range!(-4.60517018599, value);
	/// let value = Math::ln(Math::E);
	/// assert_range!(1.0, value);
	/// let value = Math::ln(2.0);
	/// assert_range!(0.69314718056, value);
	/// let value = Math::ln(10.0);
	/// assert_range!(2.30258509299, value);
	/// let value = Math::ln(-10.0);
	/// assert!(value.is_nan());
	/// let value = Math::ln(0.0);
	/// assert!(value.is_infinite());
	/// ```
	pub fn ln(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.ln() }
		#[cfg(feature = "no_std")] {
			if value.is_nan() { return f32::NAN; }
			if value == 0.0 { return f32::NEG_INFINITY; }
			if value < 0.0 { return f32::NAN; }
			if value < 1.0 { return -Math::ln(value.recip()); }
			if value.is_infinite() { return f32::INFINITY; }
			if value == 1.0 { return 0.0; }
			
			let mut x = value;
			let mut ln10_count = 0;
			let mut ln2_count = 0;
			
			while x > 10.0 {
				x /= 10.0;
				ln10_count += 1;
			}
			while x >= 2.0 {
				x /= 2.0;
				ln2_count += 1;
			}
			
			if x == 1.0 { return ln2_count as f32 * Math::LN2 + ln10_count as f32 * Math::LN10; }
			
			let term = x - 1.0;
			let mut power = term;
			let mut series = power;
			
			for i in 2..17 {
				let negative = if i % 2 == 0 { -1.0 } else { 1.0 };
				
				power *= term;
				series += negative * power / i as f32;
			}
			
			return ln2_count as f32 * Math::LN2 + ln10_count as f32 * Math::LN10 + series;
		}
	}
	
	/// Computes the natural log of the given number plus one
	/// - **value**: The value to compute the natural log of
	/// 
	/// **Returns**: Returns the natural log of the given value. Returns `infinity` if the value infinity
	/// and `-infinity` if the value is -1.0. Returns `NaN` if the value is `NaN` or less than -1.0
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::ln_1p(1.0);
	/// assert_range!(0.6931472, value);
	/// let value = Math::ln_1p(100.0);
	/// assert_range!(4.6151204, value);
	/// let value = Math::ln_1p(0.01);
	/// assert_range!(0.0099503305, value);
	/// let value = Math::ln_1p(2.0);
	/// assert_range!(1.0986123, value);
	/// let value = Math::ln_1p(10.0);
	/// assert_range!(2.3978953, value);
	/// let value = Math::ln_1p(-10.0);
	/// assert!(value.is_nan());
	/// let value = Math::ln_1p(0.0);
	/// assert_range!(0.0, value);
	/// ```
	pub fn ln_1p(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.ln_1p() }
		#[cfg(feature = "no_std")] { Math::ln(value + 1.0) }
	}
	
	/// Computes the log of the given number with a given base
	/// - **value**: The value to compute the logarithm with
	/// - **base**: The base of the logarithm
	/// 
	/// **Returns**: Returns the computed logarithm
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::log(2.0, 2.0);
	/// assert_range!(1.0, value);
	/// let value = Math::log(1.0, 2.0);
	/// assert_range!(0.0, value);
	/// let value = Math::log(10.0, 2.0);
	/// assert_range!(3.32192809489, value);
	/// let value = Math::log(16.0, 4.0);
	/// assert_range!(2.0, value);
	/// let value = Math::log(2.0, 1.0);
	/// assert!(value.is_infinite());
	/// ```
	pub fn log(value: f32, base: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.log(base) }
		#[cfg(feature = "no_std")] { Math::ln(value) * Math::ln(base).recip() }
	}
	
	/// Computes the log of the given number with base 10
	/// - **value**: The value to compute the log with
	/// 
	/// **Returns**: Returns the computed log in base 10
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::log10(1.0);
	/// assert_range!(0.0, value);
	/// let value = Math::log10(2.0);
	/// assert_range!(0.301029995664, value);
	/// let value = Math::log10(10.0);
	/// assert_range!(1.0, value);
	/// let value = Math::log10(50.0);
	/// assert_range!(1.69897000434, value);
	/// let value = Math::log10(100.0);
	/// assert_range!(2.0, value);
	/// ```
	pub fn log10(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.log10() }
		#[cfg(feature = "no_std")] { Math::ln(value) * Math::LN10.recip() }
	}
	
	/// Computes the log of the given number with base 2
	/// - **value**: The value to compute the log with
	/// 
	/// **Returns**: Returns the computed log in base 2
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::log2(1.0);
	/// assert_range!(0.0, value);
	/// let value = Math::log2(2.0);
	/// assert_range!(1.0, value);
	/// let value = Math::log2(10.0);
	/// assert_range!(3.32192809489, value);
	/// let value = Math::log2(16.0);
	/// assert_range!(4.0, value);
	/// ```
	pub fn log2(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.log2() }
		#[cfg(feature = "no_std")] { Math::ln(value) * Math::LN2.recip() }
	}
	
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
	
	/// Gets the minimum and maximum value returned as a tuple correctly sorted
	/// - **a**: The first value to get the minimum and maximum value from
	/// - **b**: The second value to get the minimum and maximum value from
	/// 
	/// **Returns**: Returns a tuple that holds the minimum and maximum values respectively
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::min_max(-1.0, 1.0);
	/// assert_eq!((-1.0, 1.0), value);
	/// let value = Math::min_max(-19.0, -19.1);
	/// assert_eq!((-19.1, -19.0), value);
	/// ```
	pub fn min_max(a: f32, b: f32) -> (f32, f32) { (Math::min(a, b), Math::max(a, b)) }
	
	/// Raised the value by the power (as a floating point number)
	/// - **value**: The value to raise with
	/// - **power**: The power to raise by
	/// 
	/// **Returns**: Returns the value raised by the power
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::pow(1.0, 0.0);
	/// assert_range!(1.0, value);
	/// let value = Math::pow(1.0, 10.0);
	/// assert_range!(1.0, value);
	/// let value = Math::pow(2.0, 10.0);
	/// assert_range!(1024.0, value, 0.0002);
	/// let value = Math::pow(40.0, 1.2);
	/// assert_range!(83.65118, value);
	/// let value = Math::pow(3.0, -2.3);
	/// assert_range!(0.07991368, value);
	/// ```
	pub fn pow(value: f32, power: f32) -> f32 {
		if power == 0.0 { return 1.0; }
		if power == 1.0 { return value; }
		if value == 1.0 { return 1.0; }
		if value == 2.0 { return Math::exp2(power); }
		
		let fract = Math::fract(power);
		
		if fract == 0.0 { return Math::pow_i32(value, Math::floor(power) as i32); }
		
		#[cfg(not(feature = "no_std"))] { value.powf(power) }
		#[cfg(feature = "no_std")] {
			Math::exp(power * Math::ln(value))
		}
	}
	
	/// Gets the power of the given number by the other given number, with the power being an `i32`
	/// - **a**: The base number to power
	/// - **b**: The number to power with
	/// 
	/// **Returns**: Returns the powered number
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::pow_i32(3.0, 5);
	/// assert_range!(243.0, value);
	/// let value = Math::pow_i32(10.45, 3);
	/// assert_range!(1141.166, value, 0.001);
	/// let value = Math::pow_i32(0.0, 0);
	/// assert_range!(1.0, value);
	/// let value = Math::pow_i32(10.0, 0);
	/// assert_range!(1.0, value);
	/// let value = Math::pow_i32(0.0, 2);
	/// assert_range!(0.0, value);
	/// let value = Math::pow_i32(2.0, -3);
	/// assert_range!(0.125, value);
	/// ```
	pub fn pow_i32(a: f32, b: i32) -> f32 {
		#[cfg(not(feature = "no_std"))] { a.powi(b) }
		#[cfg(feature = "no_std")] {
			if b == 0 { return 1.0 }
			
			let mut result = a;
			
			for _ in 1..Math::abs_i32(b) {
				result *= a;
			}
			
			if b < 0 { result.recip() }
			else { result }
		}
	}
	
	/// Rounds the given value to the nearest zero
	/// - **value**: The value to round with
	/// 
	/// **Returns**: Returns the rounded value
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::round(0.0);
	/// assert_eq!(0.0, value);
	/// let value = Math::round(1.1);
	/// assert_eq!(1.0, value);
	/// let value = Math::round(2.9);
	/// assert_eq!(3.0, value);
	/// let value = Math::round(3.5);
	/// assert_eq!(4.0, value);
	/// let value = Math::round(-4.5);
	/// assert_eq!(-5.0, value);
	/// let value = Math::round(-5.45);
	/// assert_eq!(-5.0, value);
	/// ```
	pub fn round(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.round() }
		#[cfg(feature = "no_std")] {
			let mut fraction = Math::fract(value);
			let truncated = Math::trunc(value);
			
			if value < 0.0 && fraction > 0.0 { fraction = 1.0 - fraction; }
			
			if fraction >= 0.5 {
				return truncated + Math::sign(value);
			}
			
			return truncated;
		}
	}
	
	/// Rounds the value up to the given amount of digits past the decimal
	/// - **value**: The value to round with
	/// - **digits**: The digit past the decimal to round to, must be between -15 and 15
	/// 
	/// #### Examples
	/// ```
	/// # use mathx::Math;
	/// let value = Math::round_to_digit(1.0, 0);
	/// assert_eq!(1.0, value);
	/// let value = Math::round_to_digit(-1.0, 0);
	/// assert_eq!(-1.0, value);
	/// let value = Math::round_to_digit(1.525, 0);
	/// assert_eq!(2.0, value);
	/// let value = Math::round_to_digit(1.525, 1);
	/// assert_eq!(1.5, value);
	/// let value = Math::round_to_digit(1.525, 2);
	/// assert_eq!(1.53, value);
	/// let value = Math::round_to_digit(-1.525, 0);
	/// assert_eq!(-2.0, value);
	/// let value = Math::round_to_digit(-1.525, 2);
	/// assert_eq!(-1.53, value);
	/// let value = Math::round_to_digit(-2.4, 0);
	/// assert_eq!(-2.0, value);
	/// let value = Math::round_to_digit(-2.6, 0);
	/// assert_eq!(-3.0, value);
	/// ```
	pub fn round_to_digit(value: f32, digits: i32) -> f32 {
		let digits = digits.clamp(-15, 15);
		let pow10 = Math::pow_i32(10.0, digits);
		let powered = value * pow10;
		let mut fraction = Math::fract(powered);
		let truncated = Math::trunc(powered);
		
		if fraction == 0.0 { return value; }
		if value < 0.0 { fraction = 1.0 - fraction; }
		
		if fraction >= 0.5 {
			return (truncated + Math::sign(value)) / pow10;
		}
		
		return truncated / pow10;
	}
	
	/// Computes the secant of the given angle in radians
	/// - **angle**: The given angle to compute the secant with in radians
	/// 
	/// **Returns**: Returns the computed secant value
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::sec(0.0);
	/// assert_range!(1.0, value);
	/// let value = Math::sec(Math::PI);
	/// assert_range!(-1.0, value);
	/// let value = Math::sec(Math::TWO_PI);
	/// assert_range!(1.0, value);
	/// let value = Math::sec(Math::PI_OVER_4);
	/// assert_range!(1.414213562, value);
	/// let value = Math::sec(1.0);
	/// assert_range!(1.850815718, value);
	/// let value = Math::sec(-100.0);
	/// assert_range!(1.159663823, value);
	/// ```
	pub fn sec(angle: f32) -> f32 { Math::cos(angle).recip() }
	
	/// Computes the secant of the given angle in degrees
	/// - **angle**: The given angle to compute the secant with in degrees
	/// 
	/// **Returns**: Returns the computed secant value
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::sec_deg(0.0);
	/// assert_range!(1.0, value);
	/// let value = Math::sec_deg(180.0);
	/// assert_range!(-1.0, value);
	/// let value = Math::sec_deg(360.0);
	/// assert_range!(1.0, value);
	/// let value = Math::sec_deg(45.0);
	/// assert_range!(1.414213562, value);
	/// let value = Math::sec_deg(57.29577951);
	/// assert_range!(1.850815718, value);
	/// let value = Math::sec_deg(-5729.577951);
	/// assert_range!(1.159663823, value);
	/// ```
	pub fn sec_deg(angle: f32) -> f32 { Math::sec(Math::DEG_TO_RAD * angle) }
	
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
	/// assert_eq!(-1.0, value);
	/// ```
	pub fn sign(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.signum() }
		#[cfg(feature = "no_std")] {
			if value.is_nan() { return value; }
			if value <= -0.0 { -1.0 } else { 1.0 }
		}
	}
	
	/// Computes the sine of the given angle in radians
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
	/// let value = Math::sin(Math::PI_OVER_4);
	/// assert_range!(0.707106781, value);
	/// let value = Math::sin(1.0);
	/// assert_range!(0.841470985, value);
	/// let value = Math::sin(-100.0);
	/// assert_range!(0.506365641, value);
	/// ```
	pub fn sin(angle: f32) -> f32 { Math::sin_cos(angle).0 }
	
	/// Computes the sine of the given angle in degrees
	/// - **angle**: The angle to compute sine with in degrees
	/// 
	/// **Returns**: Returns a value from the computed sine
	/// #### Remarks
	/// If you need to compute both `cos_deg` and `sin_deg` of the same angle, use `sin_cos_deg` instead as it's more
	/// performant to produce both values than calling `cos_deg` and `sin_deg` separately
	/// ##### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::sin_deg(0.0);
	/// assert_range!(0.0, value);
	/// let value = Math::sin_deg(90.0);
	/// assert_range!(1.0, value);
	/// let value = Math::sin_deg(180.0);
	/// assert_range!(0.0, value);
	/// let value = Math::sin_deg(270.0);
	/// assert_range!(-1.0, value);
	/// let value = Math::sin_deg(360.0);
	/// assert_range!(0.0, value);
	/// let value = Math::sin_deg(45.0);
	/// assert_range!(0.707106781, value);
	/// let value = Math::sin_deg(57.29577951);
	/// assert_range!(0.841470985, value);
	/// let value = Math::sin_deg(-5729.577951);
	/// assert_range!(0.506365641, value);
	/// ```
	pub fn sin_deg(angle: f32) -> f32 { Math::sin(Math::DEG_TO_RAD * angle) }
	
	/// Computes the sine and cosine of the angle in radians
	/// - **angle**: The angle to compute the sine and cosine with in radians
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
	/// let value = Math::sin_cos(Math::PI_OVER_4);
	/// assert_range_tuple2!((0.707106781, 0.707106781), value);
	/// let value = Math::sin_cos(1.0);
	/// assert_range_tuple2!((0.841470985, 0.540302306), value);
	/// let value = Math::sin_cos(-100.0);
	/// assert_range_tuple2!((0.506365641, 0.862318872), value);
	/// ```
	pub fn sin_cos(angle: f32) -> (f32, f32) {
		#[cfg(not(feature = "no_std"))] { angle.sin_cos() }
		#[cfg(feature = "no_std")] {
			const ITERATIONS: i32 = 28;
			
			if angle < -Math::PI_OVER_2 || angle > Math::PI_OVER_2 {
				return if angle < 0.0 { Math::negate_tuple(Math::sin_cos(angle + Math::PI)) }
					else { Math::negate_tuple(Math::sin_cos(angle - Math::PI)) };
			}
			
			let mut cos = 0.60725293500888;
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
	}
	
	/// Computes the sine and cosine of the angle in degrees
	/// - **angle**: The angle to compute the sine and cosine with in degrees
	/// 
	/// **Returns**: Returns the sine and cosine (respectively) as a tuple
	/// #### Remarks
	/// If you need to compute both `cos_deg` and `sin_deg` of the same angle, this function is more
	/// performant to produce both values than calling `cos_deg` and `sin_deg` separately
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range_tuple2};
	/// let value = Math::sin_cos_deg(0.0);
	/// assert_range_tuple2!((0.0, 1.0), value);
	/// let value = Math::sin_cos_deg(90.0);
	/// assert_range_tuple2!((1.0, 0.0), value);
	/// let value = Math::sin_cos_deg(180.0);
	/// assert_range_tuple2!((0.0, -1.0), value);
	/// let value = Math::sin_cos_deg(270.0);
	/// assert_range_tuple2!((-1.0, 0.0), value);
	/// let value = Math::sin_cos_deg(360.0);
	/// assert_range_tuple2!((0.0, 1.0), value);
	/// let value = Math::sin_cos_deg(45.0);
	/// assert_range_tuple2!((0.707106781, 0.707106781), value);
	/// let value = Math::sin_cos_deg(57.29577951);
	/// assert_range_tuple2!((0.841470985, 0.540302306), value);
	/// let value = Math::sin_cos_deg(-5729.577951);
	/// assert_range_tuple2!((0.506365641, 0.862318872), value);
	/// ```
	pub fn sin_cos_deg(angle: f32) -> (f32, f32) { Math::sin_cos(Math::DEG_TO_RAD * angle) }
	
	/// Computes the hyperbolic sine function
	/// - **value**: The value to compute the hyperbolic sine function with
	/// 
	/// **Returns**: Returns the computed hyperbolic sine function
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::sinh(0.0);
	/// assert_range!(0.0, value);
	/// let value = Math::sinh(1.0);
	/// assert_range!(1.1752012, value);
	/// let value = Math::sinh(-1.0);
	/// assert_range!(-1.1752012, value);
	/// let value = Math::sinh(Math::PI);
	/// assert_range!(11.54874, value);
	/// let value = Math::sinh(Math::E);
	/// assert_range!(7.5441365, value);
	/// ```
	pub fn sinh(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.sinh() }
		#[cfg(feature = "no_std")] {
			let exp = Math::exp(value);
			
			if exp.is_infinite() || exp.is_nan() {
				if value > 0.0 { return f32::INFINITY; }
				else { return f32::NEG_INFINITY; }
			}
			
			(exp - exp.recip()) * 0.5
		}
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
	
	/// Gets the square root of the given number
	/// - **value**: The number to square root
	/// 
	/// **Returns**: Returns the square root of the number, returns NaN if `value` is negative
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::sqrt(16.0);
	/// assert_range!(4.0, value);
	/// let value = Math::sqrt(1023.835);
	/// assert_range!(31.9974217711, value);
	/// let value = Math::sqrt(-102.0);
	/// assert_eq!(true, f32::is_nan(value));
	/// let value = Math::sqrt(-0.0);
	/// assert_range!(0.0, value);
	/// let value = Math::sqrt(0.2146018);
	/// assert_range!(0.46325132, value);
	/// ```
	pub fn sqrt(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.sqrt() }
		#[cfg(feature = "no_std")] {
			if value < -0.0 { return f32::NAN; }
			if value == 0.0 { return 0.0; }
			
			let mut max = 50;
			let mut x = value;
			
			while max > 0 && Math::abs(x) > 0.000000001 {
				x = (x * x * x + 3.0 * value * x) / (3.0 * x * x + value);
				max -= 1;
			}
			
			return x;
		}
	}
	
	/// Gets the tangent  of the angle in radians
	/// - **angle**: The angle to compute the tangent with in radians
	/// 
	/// **Returns**: Returns the value from the computed tangent
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::tan(0.0);
	/// assert_range!(0.0, value);
	/// let value = Math::tan(Math::PI);
	/// assert_range!(0.0, value);
	/// let value = Math::tan(Math::TWO_PI);
	/// assert_range!(0.0, value);
	/// let value = Math::tan(Math::PI_OVER_4);
	/// assert_range!(1.0, value);
	/// let value = Math::tan(1.0);
	/// assert_range!(1.557407725, value);
	/// let value = Math::tan(-100.0);
	/// assert_range!(0.587213915, value);
	/// ```
	pub fn tan(angle: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { angle.tan() }
		#[cfg(feature = "no_std")] {
			let (sin, cos) = Math::sin_cos(angle);
			
			sin / cos
		}
	}
	
	/// Gets the tangent  of the angle in degrees
	/// - **angle**: The angle to compute the tangent with in degrees
	/// 
	/// **Returns**: Returns the value from the computed tangent
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::tan_deg(0.0);
	/// assert_range!(0.0, value);
	/// let value = Math::tan_deg(180.0);
	/// assert_range!(0.0, value);
	/// let value = Math::tan_deg(360.0);
	/// assert_range!(0.0, value);
	/// let value = Math::tan_deg(45.0);
	/// assert_range!(1.0, value);
	/// let value = Math::tan_deg(57.29577951);
	/// assert_range!(1.557407725, value);
	/// let value = Math::tan_deg(-5729.577951);
	/// assert_range!(0.587213915, value);
	/// ```
	pub fn tan_deg(angle: f32) -> f32 { Math::tan(Math::DEG_TO_RAD * angle) }
	
	
	/// Computes the hyperbolic tangent function
	/// - **value**: The value to compute the hyperbolic tangent function with
	/// 
	/// **Returns**: Returns the computed hyperbolic tangent function
	/// #### Examples
	/// ```
	/// # use mathx::{Math,assert_range};
	/// let value = Math::tanh(0.0);
	/// assert_range!(0.0, value);
	/// let value = Math::tanh(1.0);
	/// assert_range!(0.7615942, value);
	/// let value = Math::tanh(-1.0);
	/// assert_range!(-0.7615942, value);
	/// let value = Math::tanh(Math::PI);
	/// assert_range!(0.9962721, value);
	/// let value = Math::tanh(Math::E);
	/// assert_range!(0.9913289, value);
	/// ```
	pub fn tanh(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.tanh() }
		#[cfg(feature = "no_std")] {
			let exp = Math::exp(2.0 * value);
			
			if exp.is_infinite() || exp.is_nan() {
				if value > 0.0 { return 1.0; }
				else { return -1.0; }
			}
			
			(exp - 1.0) * (exp + 1.0).recip()
		}
	}
	
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
	pub fn trunc(value: f32) -> f32 {
		#[cfg(not(feature = "no_std"))] { value.trunc() }
		#[cfg(feature = "no_std")] {
			(value as i32) as f32
		}
	}
}

// Private Functions
impl Math {
	/// Gets the pre-calculated arc tangent values for use in the cordic algorithm
	/// - **index**: The index to get the pre-calculated value from
	/// 
	/// **Returns**: Returns the pre-calculated value for the arc tangent
	#[cfg(feature = "no_std")]
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
			16 => 0.00001525878906,
			17 => 0.00000762939453,
			18 => 0.00000381469727,
			19 => 0.00000190734863,
			20 => 0.00000095367432,
			21 => 0.00000047683716,
			22 => 0.00000023841858,
			23 => 0.00000011920929,
			24 => 0.00000005960464,
			25 => 0.00000002980232,
			26 => 0.00000001490116,
			27 => 0.00000000745058,
			_ => 0.0,
		}
	}
	
	/// Negates the tuple, multiplying both components by -1
	/// - **tuple**: The tuple to negate
	/// 
	/// **Returns**: Returns the negated tuple
	#[cfg(feature = "no_std")]
	pub(self) fn negate_tuple(tuple: (f32, f32)) -> (f32, f32) { (-tuple.0, -tuple.1) }
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_range {
	($expected:expr, $value:expr) => {
		assert_range!($expected, $value, 0.0001);
	};
	($expected:expr, $value:expr, $epsilon:expr) => {
		if !Math::approx_epsilon($expected, $value, $epsilon) { panic!("\n\nleft: {:?}\nright: {:?}\n\n", $expected, $value); }
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_range_tuple2 {
	($expected:expr, $value:expr, $epsilon:expr) => {
		if !Math::approx_epsilon($expected.0, $value.0, $epsilon) || !Math::approx_epsilon($expected.1, $value.1,  $epsilon) { panic!("\n\nleft: {:?}\nright: {:?}\n\n", $expected, $value); }
	};
	($expected:expr, $value:expr) => {
		assert_range_tuple2!($expected, $value, 0.0001);
	};
}
