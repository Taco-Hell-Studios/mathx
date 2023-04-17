
#![cfg_attr(feature = "no_std", no_std)]

mod math;
pub use math::Math;

mod arithmetic;
pub(crate) use arithmetic::*;

mod vectors;
pub use vectors::{Vector3, Vector2};

mod colors;
pub use colors::Color;
