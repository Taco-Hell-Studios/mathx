
use mathx::curves::Tween;
use mathx::Math;

fn main() {
	let mut a = Tween::new(0.0, 1.0, 1.0, linear);
	let b = Tween::new(0.0, 1.0, 1.0, Math::lerp);
	
	a.set_func(Math::lerp);
	
	println!("A: {}", (a.func())(-1.0, 10.0, 0.7));
	println!("B: {}", (b.func())(-1.0, 10.0, 0.7));
	println!("X: {}", a);
}

fn linear(a: f32, b: f32, t: f32) -> f32 { a - b * t }
