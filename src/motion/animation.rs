//! Internal active animation trait and generic tween animation implementation.

use std::any::Any;
use std::time::Duration;

use kinetocore::clock::ClockState;
use kinetocore::direction::PlaybackDirection;
use kinetocore::ease::Ease;
use kinetocore::interpolate::Interpolate;
use kinetocore::repeat::{RepeatCount, RepeatStrategy};
use kinetocore::tween::Tween;

use crate::target::{AnimationTarget, SignalTarget};

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

/// An active animation driving any [`AnimationTarget<T>`] using a [`Tween<T>`].
pub struct TweenAnimation<Target, T: Interpolate + 'static> {
    target: Target,
    tween: Tween<T>,
    active: bool,
}

impl<Target, T> TweenAnimation<Target, T>
where
    Target: AnimationTarget<T>,
    T: Interpolate + 'static,
{
    /// Creates a new `TweenAnimation` binding a target to a tween.
    pub fn new(target: Target, tween: Tween<T>) -> Self {
        Self {
            target,
            tween,
            active: true,
        }
    }
}

impl<Target, T> ActiveAnimation for TweenAnimation<Target, T>
where
    Target: AnimationTarget<T>,
    T: Interpolate + 'static,
{
    fn step(&mut self, dt: Duration) -> bool {
        if !self.active {
            return false;
        }

        let (value, state) = self.tween.step(dt);
        self.target.write_value(value);

        if state == ClockState::Completed {
            self.active = false;
            false
        } else {
            true
        }
    }

    fn is_target_equal(&self, target: &dyn Any) -> bool {
        self.target.is_target_equal(target)
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

/// Type alias for backward compatibility with signal-based animations.
pub type SignalAnimation<T> = TweenAnimation<SignalTarget<T>, T>;
