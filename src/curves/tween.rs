
use crate::Math;
use crate::curves::InterpolationType;

pub struct Tween {
	func: Box<dyn Fn(f32, f32, f32) -> f32>,
	start: f32,
	end: f32,
	duration: f32,
	time: f32,
	loop_type: InterpolationType,
}

/// Constructors
impl Tween {
	pub fn new<T>(start: f32, end: f32, duration: f32, func: T) -> Self
		where T: Fn(f32, f32, f32) -> f32 + 'static {
		Tween {
			start,
			end,
			duration: Math::abs(duration),
			func: Box::new(func),
			time: 0.0,
			loop_type: InterpolationType::NoLoop
		}
	}
}

/// Properties
impl Tween {
	pub fn func(&self) -> &Box<dyn Fn(f32, f32, f32) -> f32> { &self.func }
	pub fn set_func<T>(&mut self, value: T) where T: Fn(f32, f32, f32) -> f32 + 'static { self.func = Box::new(value); }
	pub fn value(&self) -> f32 {
		(self.func)(self.start, self.end, self.time())
	}
	pub fn time(&self) -> f32 {
		let t = self.time / self.duration;
		
		if (self.loop_type as i32) % 2 == 1 {
			return 1.0 - t;
		}
		
		return t;
	}
	pub fn set_time(&mut self, value: f32) { self.time = Math::clamp(value, 0.0, 1.0) * self.duration; }
	pub fn is_finished(&self) -> bool { self.time >= self.duration && (self.loop_type as i32) < 2 }
}

/// Public Static Methods
impl Tween {
	pub fn linear(start: f32, end: f32, t: f32) -> f32 { start + (end - start) * t }
	pub fn ease_in_quadratic(start: f32, end: f32, t: f32) -> f32 { start + (end - start) * t * t }
	pub fn ease_out_quadratic(start: f32, end: f32, t: f32) -> f32 { start - (end - start) * (t - 2.0) * t }
	pub fn ease_in_out_quadratic(start: f32, end: f32, t: f32) -> f32 {
		if t <= 0.5 {
			start + (end - start) * t * t * 2.0
		}
		else {
			start - (end - start) * (2.0 * t * (t - 2.0) + 1.0)
		}
	}
	pub fn ease_in_exponential(start: f32, end: f32, t: f32) -> f32 { start + (end - start) * Math::pow(2.0, 10.0 * (t - 1.0)) }
	pub fn ease_out_exponential(start: f32, end: f32, t: f32) -> f32 { start + (end - start) * (1.0 - Math::pow(2.0, -10.0 * t)) }
	pub fn ease_in_out_exponential(start: f32, end: f32, t: f32) -> f32 {
		if t <= 0.5 {
			start + (end - start) * Math::pow(2.0, 20.0 * t - 11.0)
		}
		else {
			start + (end - start) * (1.0 - Math::pow(2.0, 9.0 - 20.0 * t))
		}
	}
	pub fn ease_in_circular(start: f32, end: f32, t: f32) -> f32 { start + (end - start) * (1.0 - Math::sqrt(1.0 - t * t)) }
	pub fn ease_out_circular(start: f32, end: f32, t: f32) -> f32 { start + (end - start) * Math::sqrt(1.0 - (t - 1.0) * (t - 1.0)) }
	pub fn ease_in_out_circular(start: f32, end: f32, t: f32) -> f32 {
		if t < 0.5 {
			start + (end - start) * (1.0 - Math::sqrt(1.0 - 4.0 * t * t)) * 0.5
		}
		else {
			start + (end - start) * (1.0 + Math::sqrt(1.0 - 4.0 * (t - 1.0) * (t - 1.0))) * 0.5
		}
	}
}

impl Tween {
	pub fn reset(&mut self) { self.time = 0.0; }
	pub fn update(&mut self, delta_time: f32) {
		if (self.loop_type as i32)  < 2 {
			self.time = Math::clamp(self.time + delta_time, 0.0, self.duration)
		}
		else {
			if self.time + delta_time > self.duration {
				match self.loop_type {
					InterpolationType::YoyoLoop => self.loop_type = InterpolationType::YoyoLoopBackwards,
					InterpolationType::YoyoLoopBackwards => self.loop_type = InterpolationType::YoyoLoop,
					_ => {},
				}
			}
			self.time = Math::repeat(self.time + delta_time, 0.0..self.duration);
		}
	}
}


// Display
#[cfg(not(feature = "no_std"))]
impl std::fmt::Display for Tween {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!("(start: {}, end: {}, time: {}, duration: {})", self.start, self.end, self.time, self.duration))
	}
}

impl Eq for Tween {}
impl PartialEq for Tween {
	fn eq(&self, other: &Self) -> bool {
		Math::approx(self.start, other.start)
		&& Math::approx(self.end, other.end)
		&& Math::approx(self.duration, other.duration)
		&& core::ptr::eq(self.func.as_ref() as *const _, other.func.as_ref() as *const _)
	}
}
