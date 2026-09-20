//! Internal active animation trait and signal-based implementation.

use std::any::Any;
use std::time::Duration;

use dioxus::prelude::Signal;
use kinetocore::clock::ClockState;
use kinetocore::direction::PlaybackDirection;
use kinetocore::ease::Ease;
use kinetocore::interpolate::Interpolate;
use kinetocore::repeat::{RepeatCount, RepeatStrategy};
use kinetocore::tween::Tween;

use crate::target::SignalTarget;

/// Internal trait representing any active, running animation regardless of target type.
pub trait ActiveAnimation: 'static {
    /// Steps the animation forward by `dt` and applies the interpolated value to the target.
    ///
    /// Returns `true` if the animation remains active, or `false` if it has completed.
    fn step(&mut self, dt: Duration) -> bool;

    /// Checks whether this animation targets the given target identity.
    ///
    /// Used for automatic overwrite detection and target-specific cancellation.
    fn is_target_equal(&self, target: &dyn Any) -> bool;

    /// Cancels this animation immediately.
    fn cancel(&mut self);

    /// Updates the easing function of the active tween.
    fn set_ease(&mut self, ease: Ease);

    /// Updates the repeat configuration of the active tween.
    fn set_repeat(&mut self, count: RepeatCount, strategy: RepeatStrategy);

    /// Enables or disables mirrored yoyo repeat.
    fn set_yoyo(&mut self, yoyo: bool);

    /// Updates the playback direction of the active tween.
    fn set_direction(&mut self, direction: PlaybackDirection);

    /// Returns `true` if this animation is currently active.
    fn is_active(&self) -> bool;
}

/// An active animation driving a [`SignalTarget<T>`] using a [`Tween<T>`].
pub struct SignalAnimation<T: Interpolate + 'static> {
    target: SignalTarget<T>,
    tween: Tween<T>,
    active: bool,
}

impl<T: Interpolate + 'static> SignalAnimation<T> {
    /// Creates a new `SignalAnimation` binding a target to a tween.
    pub fn new(target: SignalTarget<T>, tween: Tween<T>) -> Self {
        Self {
            target,
            tween,
            active: true,
        }
    }
}

impl<T: Interpolate + 'static> ActiveAnimation for SignalAnimation<T> {
    fn step(&mut self, dt: Duration) -> bool {
        if !self.active {
            return false;
        }

        let (value, state) = self.tween.step(dt);
        self.target.set(value);

        if state == ClockState::Completed {
            self.active = false;
            false
        } else {
            true
        }
    }

    fn is_target_equal(&self, target: &dyn Any) -> bool {
        if let Some(other) = target.downcast_ref::<SignalTarget<T>>() {
            return self.target == *other;
        }
        if let Some(other) = target.downcast_ref::<Signal<T>>() {
            return self.target.signal() == *other;
        }
        false
    }

    fn cancel(&mut self) {
        self.active = false;
    }

    fn set_ease(&mut self, ease: Ease) {
        self.tween = self.tween.clone().ease(ease);
    }

    fn set_repeat(&mut self, count: RepeatCount, strategy: RepeatStrategy) {
        self.tween = self.tween.clone().repeat(count).repeat_strategy(strategy);
    }

    fn set_yoyo(&mut self, yoyo: bool) {
        self.tween = self.tween.clone().yoyo(yoyo);
    }

    fn set_direction(&mut self, direction: PlaybackDirection) {
        self.tween = self.tween.clone().direction(direction);
    }

    fn is_active(&self) -> bool {
        self.active
    }
}
