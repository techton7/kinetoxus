//! Motion controller and animation coordination.

pub mod animation;
pub mod handle;
pub mod spring_handle;
pub mod timeline_controller;

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use kinetocore::interpolate::Interpolate;
use kinetocore::spring::SpringConfig;
use kinetocore::target::Target;
use kinetocore::tween::Tween;
#[cfg(not(target_arch = "wasm32"))]
use oxidase::frame::tick as tick_shared_frame;

use crate::driver::{Driver, DriverKind};
use crate::motion::animation::{ActiveAnimation, SpringAnimation, TweenAnimation};
pub use crate::motion::handle::MotionHandle;
pub use crate::motion::spring_handle::SpringHandle;
pub use crate::motion::timeline_controller::{
    TargetBinding, TimelineController, TimelineHookBuilder, TypedTargetBinding,
};
use crate::target::{AnimationTarget, IntoAnimationTarget};

/// Internal state managing currently running animations.
pub struct MotionInner {
    next_id: u64,
    pub(crate) animations: HashMap<u64, Box<dyn ActiveAnimation>>,
}

impl Default for MotionInner {
    fn default() -> Self {
        Self::new()
    }
}

impl MotionInner {
    /// Creates a new `MotionInner` container.
    pub fn new() -> Self {
        Self {
            next_id: 1,
            animations: HashMap::new(),
        }
    }

    /// Allocates a new unique animation ID.
    pub(crate) fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        id
    }

    /// Cancels any active animation targeting the given identity.
    pub fn cancel_for_target(&mut self, target: &dyn Any) {
        let mut to_remove = Vec::new();
        for (id, anim) in &mut self.animations {
            if anim.is_target_equal(target) {
                anim.cancel();
                to_remove.push(*id);
            }
        }
        for id in to_remove {
            self.animations.remove(&id);
        }
    }

    /// Attempts to retarget an active spring animation on the given target identity.
    pub fn retarget_spring_for_target(&mut self, target: &dyn Any, new_goal: f64) -> Option<u64> {
        for (id, anim) in &mut self.animations {
            if anim.is_target_equal(target) && anim.retarget_spring(new_goal) {
                return Some(*id);
            }
        }
        None
    }

    /// Cancels an animation by ID.
    pub fn cancel_id(&mut self, id: u64) {
        if let Some(mut anim) = self.animations.remove(&id) {
            anim.cancel();
        }
    }

    /// Cancels all active animations.
    pub fn cancel_all(&mut self) {
        for anim in self.animations.values_mut() {
            anim.cancel();
        }
        self.animations.clear();
    }

    /// Steps all active animations forward by `dt`.
    ///
    /// Returns `true` if there are still active animations remaining, or `false`
    /// if all animations have completed.
    pub fn tick(&mut self, dt: Duration) -> bool {
        let mut completed = Vec::new();
        for (id, anim) in &mut self.animations {
            let still_active = anim.step(dt);
            if !still_active {
                completed.push(*id);
            }
        }
        for id in completed {
            self.animations.remove(&id);
        }
        !self.animations.is_empty()
    }

    /// Returns the number of active animations currently running.
    #[inline]
    pub fn active_count(&self) -> usize {
        self.animations.len()
    }
}

/// Motion controller for a Dioxus component scope.
///
/// Provides ergonomic motion verbs (`set`, `from_to`, `animate`) to smoothly
/// drive reactive state over time.
///
/// When using the [`crate::hooks::use_motion()`] hook, this controller is automatically
/// bound to the component's lifecycle and will cancel running animations when the
/// component unmounts.
#[derive(Clone)]
pub struct Motion {
    pub(crate) inner: Rc<RefCell<MotionInner>>,
    pub(crate) driver: Driver,
}

impl Default for Motion {
    fn default() -> Self {
        Self::new()
    }
}

impl Motion {
    /// Creates a new `Motion` controller.
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(MotionInner::new())),
            driver: Driver::new(),
        }
    }

    /// Immediately sets the target to `value` without animating.
    ///
    /// Any currently running animation on `target` is cancelled immediately,
    /// matching standard GSAP `set()` overwrite semantics.
    pub fn set<T: Interpolate + 'static>(&self, target: impl IntoAnimationTarget<T>, value: T) {
        let target = target.into_target();
        self.inner.borrow_mut().cancel_for_target(&target);
        target.write_value(value);
    }

    /// Animates `target` from `from` to `to` over `duration`.
    ///
    /// Any previous animation running on `target` is cancelled automatically.
    /// The target value is set to `from` immediately on invocation.
    pub fn from_to<T: Interpolate + 'static>(
        &self,
        target: impl IntoAnimationTarget<T>,
        from: T,
        to: T,
        duration: Duration,
    ) -> MotionHandle {
        let target = target.into_target();
        target.write_value(from.clone());
        let tween = Tween::from_to(from, to, duration);
        self.start_animation(target, tween)
    }

    /// Animates `target` from its current value to `to` over `duration`.
    ///
    /// The starting value is lazily sampled from `target` upon playback initialization.
    /// Any previous animation running on `target` is cancelled automatically,
    /// enabling smooth interruptible redirection toward the new destination.
    pub fn to<T: Interpolate + 'static>(
        &self,
        target: impl IntoAnimationTarget<T>,
        to: T,
        duration: Duration,
    ) -> MotionHandle {
        let target = target.into_target();
        let tween = Tween::to(target.clone(), to, duration);
        self.start_animation(target, tween)
    }

    /// Animates `target` from an explicit `from` value to its current value over `duration`.
    ///
    /// The destination value is captured from `target` upon invocation, and `target` is
    /// set to `from` immediately to avoid visual flicker.
    pub fn from<T: Interpolate + 'static>(
        &self,
        target: impl IntoAnimationTarget<T>,
        from: T,
        duration: Duration,
    ) -> MotionHandle {
        let target = target.into_target();
        let mut tween = Tween::from(target.clone(), from.clone(), duration);
        tween.ensure_initialized();
        target.write_value(from);
        self.start_animation(target, tween)
    }

    /// Animates `target` using a pre-configured [`Tween<T>`].
    pub fn animate<T: Interpolate + 'static>(
        &self,
        target: impl IntoAnimationTarget<T>,
        tween: Tween<T>,
    ) -> MotionHandle {
        let target = target.into_target();
        self.start_animation(target, tween)
    }

    /// Animates `target` towards `goal` using an analytical spring governed by `config`.
    ///
    /// If an active spring animation is already running on this target, it is seamlessly
    /// retargeted mid-flight, preserving instantaneous position and velocity ($C^1$ continuity).
    pub fn spring(
        &self,
        target: impl IntoAnimationTarget<f64>,
        goal: f64,
        config: SpringConfig,
    ) -> SpringHandle {
        let target = target.into_target();
        let mut inner = self.inner.borrow_mut();

        if let Some(id) = inner.retarget_spring_for_target(&target, goal) {
            self.driver.ensure_running(&self.inner);
            return SpringHandle::new(id, Rc::downgrade(&self.inner), self.driver.clone());
        }

        inner.cancel_for_target(&target);

        let initial = target.sample();
        let spring = kinetocore::spring::Spring::new(config, initial, goal, 0.0)
            .unwrap_or_else(|_| {
                kinetocore::spring::Spring::new(SpringConfig::DEFAULT, initial, goal, 0.0)
                    .expect("Default spring config is always valid")
            });

        let id = inner.next_id();
        let anim = SpringAnimation::new(target, spring);
        inner.animations.insert(id, Box::new(anim));

        self.driver.ensure_running(&self.inner);

        SpringHandle::new(id, Rc::downgrade(&self.inner), self.driver.clone())
    }

    fn start_animation<Target, T>(
        &self,
        target: Target,
        tween: Tween<T>,
    ) -> MotionHandle
    where
        Target: AnimationTarget<T>,
        T: Interpolate + 'static,
    {
        let mut inner = self.inner.borrow_mut();
        inner.cancel_for_target(&target);

        let id = inner.next_id();
        let anim = TweenAnimation::new(target, tween);
        inner.animations.insert(id, Box::new(anim));

        self.driver.ensure_running(&self.inner);

        MotionHandle {
            id,
            inner: Rc::downgrade(&self.inner),
        }
    }

    /// Cancels any active animation targeting the given target.
    pub fn cancel_target<T: 'static>(&self, target: impl IntoAnimationTarget<T>) {
        let target = target.into_target();
        self.inner.borrow_mut().cancel_for_target(&target);
        if self.inner.borrow().active_count() == 0 {
            self.driver.stop();
        }
    }

    /// Cancels all active animations managed by this motion controller.
    pub fn cancel_all(&self) {
        self.inner.borrow_mut().cancel_all();
        self.driver.stop();
    }

    /// Steps all active animations forward by `dt`.
    pub fn tick(&self, dt: Duration) -> bool {
        #[cfg(target_arch = "wasm32")]
        let has_more = self.inner.borrow_mut().tick(dt);

        #[cfg(not(target_arch = "wasm32"))]
        {
            tick_shared_frame(dt);
        }

        #[cfg(not(target_arch = "wasm32"))]
        let has_more = self.inner.borrow().active_count() > 0;

        if !has_more {
            self.driver.stop();
        }
        has_more
    }

    /// Returns the active driving mechanism used on this platform.
    #[inline]
    pub fn driver_kind(&self) -> DriverKind {
        self.driver.kind()
    }

    /// Returns the number of currently active animations.
    #[inline]
    pub fn active_count(&self) -> usize {
        self.inner.borrow().active_count()
    }
}
