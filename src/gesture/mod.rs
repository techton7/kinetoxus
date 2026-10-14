//! Gesture continuation adapter and reactive controller.

pub mod controller;
pub mod tracker;

pub use controller::{
    GestureContinuationConfig, GestureContinuationController, GesturePhase,
};
pub use tracker::{GestureTracker, TrackerConfig, VelocityWeighting};
