
#![cfg_attr(feature = "no_std", no_std)]

mod math;
pub use math::Math;
pub mod interfaces;

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
pub use rays::{Ray2, Ray3};

#[cfg(not(all(feature = "no_planes", feature = "no_vectors")))]
mod plane;
#[cfg(not(all(feature = "no_planes", feature = "no_vectors")))]
pub use plane::Plane;

#[cfg(not(feature = "no_collision"))]
pub mod collision;

#[cfg(not(feature = "no_colors"))]
mod colors;
#[cfg(not(feature = "no_colors"))]
pub use colors::Color;

#[cfg(not(feature = "no_tweens"))]
pub mod curves;
