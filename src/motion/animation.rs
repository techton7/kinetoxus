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

    /// Attempts to retarget a spring animation to a new goal position mid-flight.
    ///
    /// Returns `true` if this animation was a spring and was successfully retargeted.
    fn retarget_spring(&mut self, _new_goal: f64) -> bool {
        false
    }
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

/// An active animation driving an [`AnimationTarget<f64>`] using an analytical [`kinetocore::spring::Spring`].
pub struct SpringAnimation<Target> {
    target: Target,
    spring: kinetocore::spring::Spring,
    elapsed: Duration,
    active: bool,
}

impl<Target> SpringAnimation<Target>
where
    Target: AnimationTarget<f64>,
{
    /// Creates a new `SpringAnimation` binding a target to an analytical spring.
    pub fn new(target: Target, spring: kinetocore::spring::Spring) -> Self {
        Self {
            target,
            spring,
            elapsed: Duration::ZERO,
            active: true,
        }
    }

    /// Returns reference to the current underlying spring.
    pub fn spring(&self) -> &kinetocore::spring::Spring {
        &self.spring
    }

    /// Returns the current elapsed duration since the last activation/retarget.
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }
}

impl<Target> ActiveAnimation for SpringAnimation<Target>
where
    Target: AnimationTarget<f64>,
{
    fn step(&mut self, dt: Duration) -> bool {
        if !self.active {
            return false;
        }

        self.elapsed += dt;

        if self.spring.is_settled(self.elapsed) {
            // Exact settle clamping to goal position
            self.target.write_value(self.spring.target_position());
            self.active = false;
            false
        } else {
            let state = self.spring.sample(self.elapsed);
            self.target.write_value(state.position);
            true
        }
    }

    fn is_target_equal(&self, target: &dyn Any) -> bool {
        self.target.is_target_equal(target)
    }

    fn cancel(&mut self) {
        self.active = false;
    }

    fn set_ease(&mut self, _ease: Ease) {}

    fn set_repeat(&mut self, _count: RepeatCount, _strategy: RepeatStrategy) {}

    fn set_yoyo(&mut self, _yoyo: bool) {}

    fn set_direction(&mut self, _direction: PlaybackDirection) {}

    fn is_active(&self) -> bool {
        self.active
    }

    fn retarget_spring(&mut self, new_goal: f64) -> bool {
        self.spring = self.spring.retargeted(new_goal, self.elapsed);
        self.elapsed = Duration::ZERO;
        self.active = true;
        true
    }
}
