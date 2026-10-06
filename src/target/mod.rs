//! Target adapters for Dioxus animations.

pub mod handle;
pub mod signal;

pub use handle::{HandleTarget, Transform2D};
pub use signal::SignalTarget;

use std::any::Any;
use kinetocore::target::{IntoTargetSampler, Target};

/// Trait representing an animation target that can receive written values and be sampled.
pub trait AnimationTarget<T>: Target<T> + IntoTargetSampler<T> + IntoAnimationTarget<T, Target = Self> + Clone + 'static {
    /// Writes an interpolated value to the target.
    fn write_value(&self, value: T);
    /// Checks if this target equals another target (downcasted via `Any`).
    fn is_target_equal(&self, other: &dyn Any) -> bool;
}

/// Trait for types that can be converted into an [`AnimationTarget<T>`].
pub trait IntoAnimationTarget<T> {
    /// The target type produced by conversion.
    type Target: AnimationTarget<T>;
    /// Converts `self` into an [`AnimationTarget<T>`].
    fn into_target(self) -> Self::Target;
}
