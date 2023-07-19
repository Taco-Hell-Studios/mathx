
#[cfg(not(feature = "no_tweens"))]
mod interpolation_type;
#[cfg(not(feature = "no_tweens"))]
pub use interpolation_type::InterpolationType;

#[cfg(not(feature = "no_tweens"))]
mod tween;
#[cfg(not(feature = "no_tweens"))]
pub use tween::Tween;
