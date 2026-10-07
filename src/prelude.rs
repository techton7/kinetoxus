//! Convenient re-exports for users of `kinetoxus`.

pub use crate::driver::DriverKind;
pub use crate::hooks::{use_motion, use_spring, use_timeline};
pub use crate::motion::{
    Motion, MotionHandle, SpringHandle, TargetBinding, TimelineController, TimelineHookBuilder,
    TypedTargetBinding,
};
pub use crate::target::{AnimationTarget, HandleTarget, IntoAnimationTarget, SignalTarget, Transform2D};

// Re-export core math and tweening primitives from kinetocore
pub use kinetocore::direction::PlaybackDirection;
pub use kinetocore::ease::Ease;
pub use kinetocore::interpolate::{lerp, Interpolate};
pub use kinetocore::repeat::{RepeatCount, RepeatStrategy};
pub use kinetocore::spring::{DampingRegime, Spring, SpringConfig, SpringError, SpringState};
pub use kinetocore::state::TweenEndpoints;
pub use kinetocore::target::{IntoTargetSampler, Target, TargetSampler};
pub use kinetocore::timeline::{
    CompiledTimeline, Keyframe, OffsetSign, Position, SignedDuration, Timeline, TimelineBuilder,
    TimelineError, TrackBuilder,
};
pub use kinetocore::tween::Tween;
