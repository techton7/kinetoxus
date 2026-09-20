//! Motion controller and animation coordination.

pub mod animation;
pub mod handle;

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use kinetocore::interpolate::Interpolate;
use kinetocore::tween::Tween;

use crate::driver::{Driver, DriverKind};
use crate::motion::animation::{ActiveAnimation, SignalAnimation};
pub use crate::motion::handle::MotionHandle;
use crate::target::SignalTarget;

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
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dioxus::prelude::*;
    /// use kinetoxus::prelude::*;
    ///
    /// fn example(motion: &Motion, signal: Signal<f32>) {
    ///     motion.set(signal, 100.0);
    ///     assert_eq!(signal(), 100.0);
    /// }
    /// ```
    pub fn set<T: Interpolate + 'static>(&self, target: impl Into<SignalTarget<T>>, value: T) {
        let target = target.into();
        self.inner.borrow_mut().cancel_for_target(&target);
        target.set(value);
    }

    /// Animates `target` from `from` to `to` over `duration`.
    ///
    /// Any previous animation running on `target` is cancelled automatically.
    /// The target value is set to `from` immediately on invocation.
    ///
    /// Returns a [`MotionHandle`] for fluent configuration (such as [`MotionHandle::ease`])
    /// or cancellation.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::time::Duration;
    /// use dioxus::prelude::*;
    /// use kinetoxus::prelude::*;
    ///
    /// fn example(motion: &Motion, signal: Signal<f32>) {
    ///     motion.from_to(signal, 0.0, 1.0, Duration::from_millis(300))
    ///         .ease(Ease::CubicOut);
    /// }
    /// ```
    pub fn from_to<T: Interpolate + 'static>(
        &self,
        target: impl Into<SignalTarget<T>>,
        from: T,
        to: T,
        duration: Duration,
    ) -> MotionHandle {
        let target = target.into();
        // Immediately render the start value to avoid a one-frame flicker
        target.set(from.clone());
        let tween = Tween::from_to(from, to, duration);
        self.start_animation(target, tween)
    }

    /// Animates `target` using a pre-configured [`Tween<T>`].
    ///
    /// This allows reusing any tween constructed via `kinetocore`'s fluent builder.
    pub fn animate<T: Interpolate + 'static>(
        &self,
        target: impl Into<SignalTarget<T>>,
        tween: Tween<T>,
    ) -> MotionHandle {
        let target = target.into();
        self.start_animation(target, tween)
    }

    fn start_animation<T: Interpolate + 'static>(
        &self,
        target: SignalTarget<T>,
        tween: Tween<T>,
    ) -> MotionHandle {
        let mut inner = self.inner.borrow_mut();
        // Cancel any previous animation on the same target
        inner.cancel_for_target(&target);

        let id = inner.next_id();
        let anim = SignalAnimation::new(target, tween);
        inner.animations.insert(id, Box::new(anim));

        // Ensure frame driver is actively ticking
        self.driver.ensure_running(&self.inner);

        MotionHandle {
            id,
            inner: Rc::downgrade(&self.inner),
        }
    }

    /// Cancels any active animation targeting the given signal.
    pub fn cancel_target<T: 'static>(&self, target: impl Into<SignalTarget<T>>) {
        let target = target.into();
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
    ///
    /// Returns `true` if there are still active animations remaining.
    ///
    /// In non-WASM / headless environments or unit tests, this is the primary
    /// mechanism for advancing animations forward in time. On Web (WASM), animations
    /// are automatically advanced by the `requestAnimationFrame` driver.
    pub fn tick(&self, dt: Duration) -> bool {
        let has_more = self.inner.borrow_mut().tick(dt);
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
