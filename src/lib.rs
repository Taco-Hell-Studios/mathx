
#![cfg_attr(feature = "no_std", no_std)]

mod math;
pub use math::Math;

#[cfg(not(feature = "no_vectors"))]
mod arithmetic;
#[cfg(not(feature = "no_vectors"))]
pub(crate) use arithmetic::*;

#[cfg(not(feature = "no_vectors"))]
mod vectors;
#[cfg(not(feature = "no_vectors"))]
pub use vectors::{Vector3, Vector2};

#[cfg(not(feature = "no_colors"))]
mod colors;
#[cfg(not(feature = "no_colors"))]
pub use colors::Color;
