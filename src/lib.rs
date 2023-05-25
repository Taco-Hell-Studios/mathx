
#![cfg_attr(feature = "no_std", no_std)]

mod math;
pub use math::Math;

#[cfg(not(all(feature = "no_vectors", feature = "no_quaternions")))]
mod arithmetic;
#[cfg(not(all(feature = "no_vectors", feature = "no_quaternions")))]
pub(crate) use arithmetic::*;

#[cfg(not(feature = "no_quaternions"))]
mod quaternions;
#[cfg(not(feature = "no_quaternions"))]
pub use quaternions::Quaternion;

#[cfg(not(feature = "no_vectors"))]
mod vectors;
#[cfg(not(feature = "no_vectors"))]
pub use vectors::{Vector3, Vector2};

#[cfg(not(all(feature = "no_rays", feature = "no_vectors")))]
mod rays;
#[cfg(not(all(feature = "no_rays", feature = "no_vectors")))]
pub use rays::Ray3;

#[cfg(not(feature = "no_colors"))]
mod colors;
#[cfg(not(feature = "no_colors"))]
pub use colors::Color;
