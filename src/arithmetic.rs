
#[doc(hidden)]
pub trait AddSubArithmetic<T> {
	type Output;
	fn add_other(self, rhs: T) -> Self::Output;
	fn subtract_other(self, rhs: T) -> Self::Output;
	fn add_assign_other(&mut self, rhs: T);
	fn subtract_assign_other(&mut self, rhs: T);
}

#[doc(hidden)]
pub trait MulDivScalar {
	type Output;
	fn multiply_scalar(self, rhs: f32) -> Self::Output;
	fn divide_scalar(self, rhs: f32) -> Self::Output;
	fn reciprocal_scalar(self, rhs: f32) -> Self::Output;
	fn multiply_assign_scalar(&mut self, rhs: f32);
	fn divide_assign_scalar(&mut self, rhs: f32);
}

#[doc(hidden)]
#[macro_export]
macro_rules! use_impl_ops {
	() => {
		use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_add {
	($($t:ty)*) => {
		$(
			impl Add<$t> for $t {
				type Output = $t;
				
				fn add(self, rhs: $t) -> Self::Output {
					self.add_other(rhs)
				}
			}
			impl AddAssign<$t> for $t {
				fn add_assign(&mut self, rhs: $t) {
					self.add_assign_other(rhs);
				}
			}
		)*
	};
	($($t1:ty => $t2:ty: $tout:ty)*) => {
		$(
			impl Add<$t2> for $t1 {
				type Output = $tout;
				fn add(self, rhs: $t2) -> Self::Output {
					self.add_other(rhs)
				}
			}
			impl AddAssign<$t2> for $t1 {
				fn add_assign(&mut self, rhs: $t2) {
					self.add_assign_other(rhs);
				}
			}
		)*
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_sub {
	($($t:ty)*) => {
		$(
			impl Sub<$t> for $t {
				type Output = $t;
				
				fn sub(self, rhs: $t) -> Self::Output {
					self.subtract_other(rhs)
				}
			}
			impl SubAssign<$t> for $t {
				fn sub_assign(&mut self, rhs: $t) {
					self.subtract_assign_other(rhs);
				}
			}
		)*
	};
	($($t1:ty => $t2:ty: $tout:ty)*) => {
		$(
			impl Sub<$t2> for $t1 {
				type Output = $tout;
				fn sub(self, rhs: $t2) -> Self::Output {
					self.subtract_other(rhs)
				}
			}
			impl SubAssign<$t2> for $t1 {
				fn sub_assign(&mut self, rhs: $t2) {
					self.subtract_assign_other(rhs);
				}
			}
		)*
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_mul {
	($($t:ty)*) => {
		$(
			impl Mul<f32> for $t {
				type Output = $t;
				fn mul(self, rhs: f32) -> Self::Output {
					self.multiply_scalar(rhs)
				}
			}
			impl Mul<i32> for $t {
				type Output = $t;
				fn mul(self, rhs: i32) -> Self::Output {
					self.multiply_scalar(rhs as f32)
				}
			}
			impl Mul<$t> for f32 {
				type Output = $t;
				fn mul(self, rhs: $t) -> Self::Output {
					rhs.multiply_scalar(self)
				}
			}
			impl Mul<$t> for i32 {
				type Output = $t;
				fn mul(self, rhs: $t) -> Self::Output {
					rhs.multiply_scalar(self as f32)
				}
			}
			impl MulAssign<f32> for $t {
				fn mul_assign(&mut self, rhs: f32) {
					self.multiply_assign_scalar(rhs);
				}
			}
			impl MulAssign<i32> for $t {
				fn mul_assign(&mut self, rhs: i32) {
					self.multiply_assign_scalar(rhs as f32);
				}
			}
		)*
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_div {
	($($t:ty)*) => {
		$(
			impl Div<f32> for $t {
				type Output = $t;
				fn div(self, rhs: f32) -> Self::Output {
					self.divide_scalar(rhs)
				}
			}
			impl Div<i32> for $t {
				type Output = $t;
				fn div(self, rhs: i32) -> Self::Output {
					self.divide_scalar(rhs as f32)
				}
			}
			impl Div<$t> for f32 {
				type Output = $t;
				fn div(self, rhs: $t) -> Self::Output {
					rhs.reciprocal_scalar(self)
				}
			}
			impl Div<$t> for i32 {
				type Output = $t;
				fn div(self, rhs: $t) -> Self::Output {
					rhs.reciprocal_scalar(self as f32)
				}
			}
			impl DivAssign<f32> for $t {
				fn div_assign(&mut self, rhs: f32) {
					self.divide_assign_scalar(rhs);
				}
			}
			impl DivAssign<i32> for $t {
				fn div_assign(&mut self, rhs: i32) {
					self.divide_assign_scalar(rhs as f32);
				}
			}
		)*
	};
}
