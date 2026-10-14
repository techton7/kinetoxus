//! Convenient re-exports for users of `kinetoxus`.

pub use crate::driver::DriverKind;
pub use crate::gesture::{
    GestureContinuationConfig, GestureContinuationController, GesturePhase, GestureTracker,
    TrackerConfig, VelocityWeighting,
};
pub use crate::hooks::{use_gesture_continuation, use_motion, use_spring, use_timeline};
pub use crate::motion::{
    Motion, MotionHandle, SpringHandle, SubTimelineHookBuilder, TargetBinding, TimelineController,
    TimelineHookBuilder, TypedTargetBinding,
};
pub use crate::target::{AnimationTarget, HandleTarget, IntoAnimationTarget, SignalTarget, Transform2D};

// Re-export core math and physics primitives from kinetocore
pub use kinetocore::direction::PlaybackDirection;
pub use kinetocore::ease::Ease;
pub use kinetocore::interpolate::{lerp, Interpolate};
pub use kinetocore::physics::{
    apply_overdrag, BoundaryPolicy as PhysicsBoundaryPolicy, InertiaConfig, InertiaContinuation,
    KinematicSampleBuffer, SnapPolicy,
};
pub use kinetocore::repeat::{RepeatCount, RepeatStrategy};
pub use kinetocore::spring::{DampingRegime, Spring, SpringConfig, SpringError, SpringState};
pub use kinetocore::state::TweenEndpoints;
pub use kinetocore::target::{IntoTargetSampler, Target, TargetSampler};
pub use kinetocore::timeline::{
    ActiveOverlapPolicy, BoundaryPolicy, ClipId, ClipTarget, CompiledTimeline,
    CompiledTimelineSnapshot, DynamicTimeline, InsertClipMutation, InsertSubTimelineMutation,
    Keyframe, LabelPath, OffsetSign, Position, RemoveClipMutation, RetargetClipMutation,
    SignedDuration, SubTimelineBuilder, SubTimelineNode, TargetOwnershipHandoff, Timeline,
    TimelineBuilder, TimelineError, TimelineMutation, TimelineTreeBuilder, TrackBoundaryPolicy,
    TrackBuilder,
};
pub use kinetocore::tween::Tween;
