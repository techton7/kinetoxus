//! Convenient re-exports for users of `kinetoxus`.

pub use crate::driver::DriverKind;
pub use crate::hooks::use_motion;
pub use crate::motion::{Motion, MotionHandle};
pub use crate::target::SignalTarget;

// Re-export core math and tweening primitives from kinetocore
pub use kinetocore::direction::PlaybackDirection;
pub use kinetocore::ease::Ease;
pub use kinetocore::interpolate::{lerp, Interpolate};
pub use kinetocore::repeat::{RepeatCount, RepeatStrategy};
pub use kinetocore::tween::Tween;
