//! Gesture continuation controller managing drag tracking, release handoffs,
//! and automated frame loop lifecycle with complete dormancy upon settle.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use kinetocore::physics::boundary::apply_overdrag;
use kinetocore::physics::{BoundaryPolicy, InertiaConfig, InertiaContinuation, SnapPolicy};
use kinetocore::spring::{Spring, SpringConfig};
use oxidase::frame::{start_frame_loop, FrameInfo, FrameLoopGuard};

use crate::gesture::tracker::{GestureTracker, TrackerConfig};
use crate::target::{AnimationTarget, IntoAnimationTarget, SignalTarget};

/// The active kinematic phase of a gesture continuation cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GesturePhase {
    /// Dormant state with no active drag or momentum continuation running.
    #[default]
    Idle,
    /// Host pointer is actively dragging and tracking motion samples.
    Dragging,
    /// Inertia decay continuation is actively decelerating towards natural or snapped target.
    Inertia,
    /// Spring bounce-back continuation is restoring position from a violated boundary limit.
    Bounce,
}

impl GesturePhase {
    /// Returns `true` if the phase is [`GesturePhase::Idle`].
    #[inline]
    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    /// Returns `true` if the phase is [`GesturePhase::Dragging`].
    #[inline]
    pub fn is_dragging(&self) -> bool {
        matches!(self, Self::Dragging)
    }

    /// Returns `true` if the phase is [`GesturePhase::Inertia`].
    #[inline]
    pub fn is_inertia(&self) -> bool {
        matches!(self, Self::Inertia)
    }

    /// Returns `true` if the phase is [`GesturePhase::Bounce`].
    #[inline]
    pub fn is_bounce(&self) -> bool {
        matches!(self, Self::Bounce)
    }

    /// Returns `true` if either inertia or bounce animation is running.
    #[inline]
    pub fn is_animating(&self) -> bool {
        matches!(self, Self::Inertia | Self::Bounce)
    }
}

/// Complete physical and algorithmic configuration for gesture continuation.
#[derive(Debug, Clone, PartialEq)]
pub struct GestureContinuationConfig {
    /// Moving-window pointer tracker configuration.
    pub tracker: TrackerConfig,
    /// Exponential inertia decay deceleration configuration.
    pub inertia: InertiaConfig,
    /// Boundary constraint policy (None, Clamp, or Bounce).
    pub boundary: BoundaryPolicy,
    /// Snapping resolution policy for landing targets.
    pub snap: SnapPolicy,
}

impl Default for GestureContinuationConfig {
    fn default() -> Self {
        Self {
            tracker: TrackerConfig::default(),
            inertia: InertiaConfig::default(),
            boundary: BoundaryPolicy::None,
            snap: SnapPolicy::None,
        }
    }
}

impl GestureContinuationConfig {
    /// Creates a new configuration with explicit components.
    pub fn new(
        tracker: TrackerConfig,
        inertia: InertiaConfig,
        boundary: BoundaryPolicy,
        snap: SnapPolicy,
    ) -> Self {
        Self {
            tracker,
            inertia,
            boundary,
            snap,
        }
    }

    /// Builder method to update the tracker configuration.
    pub fn with_tracker(mut self, tracker: TrackerConfig) -> Self {
        self.tracker = tracker;
        self
    }

    /// Builder method to update the inertia decay configuration.
    pub fn with_inertia(mut self, inertia: InertiaConfig) -> Self {
        self.inertia = inertia;
        self
    }

    /// Builder method to update the boundary policy.
    pub fn with_boundary(mut self, boundary: BoundaryPolicy) -> Self {
        self.boundary = boundary;
        self
    }

    /// Builder method to update the snap policy.
    pub fn with_snap(mut self, snap: SnapPolicy) -> Self {
        self.snap = snap;
        self
    }

    /// Builder shortcut to set hard clamping bounds $[\min, \max]$.
    pub fn with_bounds(mut self, min: f64, max: f64) -> Self {
        self.boundary = BoundaryPolicy::clamp(min, max);
        self
    }

    /// Builder shortcut to set elastic bounce-back bounds $[\min, \max]$.
    pub fn with_bounce(mut self, min: f64, max: f64, spring: SpringConfig) -> Self {
        self.boundary = BoundaryPolicy::bounce(min, max, spring);
        self
    }

    /// Builder shortcut to set a regular snap interval.
    pub fn with_snap_interval(mut self, step: f64, offset: f64) -> Self {
        self.snap = SnapPolicy::interval(step, offset);
        self
    }

    /// Builder shortcut to set discrete anchor snap points.
    pub fn with_snap_points(mut self, points: Vec<f64>) -> Self {
        self.snap = SnapPolicy::points(points);
        self
    }
}

enum ActiveContinuation {
    Inertia {
        continuation: InertiaContinuation,
        elapsed: Duration,
    },
    BounceSpring {
        spring: Spring,
        elapsed: Duration,
    },
}

pub(crate) struct GestureControllerInner<Target> {
    target: Target,
    config: GestureContinuationConfig,
    tracker: GestureTracker<16>,
    phase: GesturePhase,
    active_continuation: Option<ActiveContinuation>,
}

impl<Target: AnimationTarget<f64>> GestureControllerInner<Target> {
    pub(crate) fn new(target: Target, config: GestureContinuationConfig) -> Self {
        let tracker = GestureTracker::new(config.tracker);
        Self {
            target,
            config,
            tracker,
            phase: GesturePhase::Idle,
            active_continuation: None,
        }
    }

    pub(crate) fn step(&mut self, dt: Duration) -> bool {
        match &mut self.active_continuation {
            Some(ActiveContinuation::Inertia { continuation, elapsed }) => {
                *elapsed += dt;
                if *elapsed >= continuation.duration() {
                    let target_pos = continuation.target();
                    self.target.write_value(target_pos);
                    self.active_continuation = None;
                    self.phase = GesturePhase::Idle;
                    false
                } else {
                    let (pos, _) = continuation.sample_at(*elapsed);
                    self.target.write_value(pos);
                    true
                }
            }
            Some(ActiveContinuation::BounceSpring { spring, elapsed }) => {
                *elapsed += dt;
                if spring.is_settled(*elapsed) {
                    let target_pos = spring.target_position();
                    self.target.write_value(target_pos);
                    self.active_continuation = None;
                    self.phase = GesturePhase::Idle;
                    false
                } else {
                    let state = spring.sample(*elapsed);
                    self.target.write_value(state.position);
                    true
                }
            }
            None => {
                self.phase = GesturePhase::Idle;
                false
            }
        }
    }

    pub(crate) fn current_velocity(&self) -> f64 {
        match &self.active_continuation {
            Some(ActiveContinuation::Inertia { continuation, elapsed }) => {
                continuation.sample_at(*elapsed).1
            }
            Some(ActiveContinuation::BounceSpring { spring, elapsed }) => {
                spring.sample(*elapsed).velocity
            }
            None => {
                if self.phase == GesturePhase::Dragging {
                    self.tracker.estimate_velocity()
                } else {
                    0.0
                }
            }
        }
    }

    pub(crate) fn cancel(&mut self) {
        self.active_continuation = None;
        self.phase = GesturePhase::Idle;
        self.tracker.reset();
    }
}

#[derive(Default)]
struct DriverState {
    running: Cell<bool>,
    guard: RefCell<Option<FrameLoopGuard>>,
}

/// Controller managing interactive pointer drag and momentum continuation.
///
/// Binds directly to an [`AnimationTarget<f64>`] such as [`SignalTarget`] or [`HandleTarget`](crate::target::HandleTarget).
/// Tracks host-side pointer movements, enforces boundary policies, estimates release velocity,
/// and automatically drives inertia decay or bounce spring continuations via `oxidase::frame`.
///
/// Automatically transitions to complete dormancy (`active_count == 0` and unregisters
/// from frame loop) upon settle.
pub struct GestureContinuationController<Target = SignalTarget<f64>> {
    inner: Rc<RefCell<GestureControllerInner<Target>>>,
    driver_state: Rc<DriverState>,
}

impl<Target> Clone for GestureContinuationController<Target> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            driver_state: self.driver_state.clone(),
        }
    }
}

impl<Target> PartialEq for GestureContinuationController<Target> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<Target> Eq for GestureContinuationController<Target> {}

impl<Target> GestureContinuationController<Target> {
    fn stop_driver(&self) {
        self.driver_state.running.set(false);
        let _ = self.driver_state.guard.borrow_mut().take();
    }
}

impl<Target: AnimationTarget<f64>> GestureContinuationController<Target> {
    /// Creates a new `GestureContinuationController` binding the given target and config.
    pub fn new(target: Target, config: GestureContinuationConfig) -> Self {
        Self {
            inner: Rc::new(RefCell::new(GestureControllerInner::new(target, config))),
            driver_state: Rc::new(DriverState::default()),
        }
    }

    /// Creates a new `GestureContinuationController` converting `target` into an animation target.
    pub fn bind<T: IntoAnimationTarget<f64, Target = Target>>(
        target: T,
        config: GestureContinuationConfig,
    ) -> Self {
        Self::new(target.into_target(), config)
    }

    fn ensure_running(&self) {
        if self.driver_state.running.replace(true) {
            return;
        }

        let weak_inner = Rc::downgrade(&self.inner);
        let weak_driver = Rc::downgrade(&self.driver_state);

        match start_frame_loop(move |info: FrameInfo| {
            let Some(driver) = weak_driver.upgrade() else {
                return;
            };
            let Some(inner_rc) = weak_inner.upgrade() else {
                driver.running.set(false);
                let _ = driver.guard.borrow_mut().take();
                return;
            };

            let has_more = inner_rc.borrow_mut().step(info.delta);
            if !has_more {
                driver.running.set(false);
                let _ = driver.guard.borrow_mut().take();
            }
        }) {
            Ok(guard) => {
                *self.driver_state.guard.borrow_mut() = Some(guard);
            }
            Err(_) => {
                self.driver_state.running.set(false);
            }
        }
    }

    /// Initiates or interrupts an active gesture drag at `position` and `time`.
    ///
    /// Mid-flight continuations are interrupted immediately ($C^0$ seamless capture).
    /// Samples are ingested into the ring buffer, boundary resistance is applied,
    /// and the target is updated synchronously.
    pub fn drag_start(&self, position: f64, time: Duration) {
        self.stop_driver();
        let mut inner = self.inner.borrow_mut();
        inner.active_continuation = None;
        inner.phase = GesturePhase::Dragging;
        inner.tracker.reset();
        inner.tracker.add_sample(position, time);

        let effective_pos = match &inner.config.boundary {
            BoundaryPolicy::None => position,
            BoundaryPolicy::Clamp { min, max } => {
                let (lo, hi) = if min <= max { (*min, *max) } else { (*max, *min) };
                position.clamp(lo, hi)
            }
            BoundaryPolicy::Bounce { min, max, overdrag_elasticity, .. } => {
                apply_overdrag(position, *min, *max, *overdrag_elasticity)
            }
        };

        inner.target.write_value(effective_pos);
    }

    /// Updates pointer position during an active drag at `position` and `time`.
    ///
    /// Applies boundary policies (hard clamp or elastic overdrag resistance)
    /// and synchronously writes the resulting coordinate to the target.
    pub fn drag_move(&self, position: f64, time: Duration) {
        let mut inner = self.inner.borrow_mut();
        if inner.phase != GesturePhase::Dragging {
            drop(inner);
            self.drag_start(position, time);
            return;
        }

        inner.tracker.add_sample(position, time);

        let effective_pos = match &inner.config.boundary {
            BoundaryPolicy::None => position,
            BoundaryPolicy::Clamp { min, max } => {
                let (lo, hi) = if min <= max { (*min, *max) } else { (*max, *min) };
                position.clamp(lo, hi)
            }
            BoundaryPolicy::Bounce { min, max, overdrag_elasticity, .. } => {
                apply_overdrag(position, *min, *max, *overdrag_elasticity)
            }
        };

        inner.target.write_value(effective_pos);
    }

    /// Releases the active pointer drag at timestamp `time`, transitioning into momentum continuation.
    ///
    /// Deterministically estimates release velocity using recent-window weighting,
    /// verifies boundary constraints, and launches an inertia decay continuation
    /// or bounce spring. Automatically subscribes to `oxidase::frame` and transitions
    /// to dormancy upon settle.
    pub fn drag_end(&self, time: Duration) {
        let mut inner = self.inner.borrow_mut();
        if inner.phase != GesturePhase::Dragging {
            return;
        }

        let vel = inner.tracker.estimate_velocity_at(time);
        let current_pos = inner.target.sample();

        // 1. Boundary bounce check: are we currently beyond bounds?
        if let BoundaryPolicy::Bounce { min, max, spring, .. } = inner.config.boundary {
            let (lo, hi) = if min <= max { (min, max) } else { (max, min) };
            if current_pos < lo {
                let s = Spring::new(spring, current_pos, lo, vel).unwrap_or_else(|_| {
                    Spring::new(SpringConfig::DEFAULT, current_pos, lo, vel)
                        .expect("Default spring config is always valid")
                });
                inner.active_continuation = Some(ActiveContinuation::BounceSpring {
                    spring: s,
                    elapsed: Duration::ZERO,
                });
                inner.phase = GesturePhase::Bounce;
                drop(inner);
                self.ensure_running();
                return;
            } else if current_pos > hi {
                let s = Spring::new(spring, current_pos, hi, vel).unwrap_or_else(|_| {
                    Spring::new(SpringConfig::DEFAULT, current_pos, hi, vel)
                        .expect("Default spring config is always valid")
                });
                inner.active_continuation = Some(ActiveContinuation::BounceSpring {
                    spring: s,
                    elapsed: Duration::ZERO,
                });
                inner.phase = GesturePhase::Bounce;
                drop(inner);
                self.ensure_running();
                return;
            }
        }

        // 2. Normal region: evaluate velocity cutoff and snapping
        let cutoff = inner.config.inertia.velocity_cutoff;
        if vel.abs() < cutoff {
            // Low or zero release velocity
            let snapped = inner.config.snap.resolve(current_pos);
            let diff = (snapped - current_pos).abs();
            if diff > inner.config.inertia.position_tolerance {
                let continuation = InertiaContinuation::with_target(
                    current_pos,
                    0.0,
                    snapped,
                    inner.config.inertia,
                );
                if continuation.duration() == Duration::ZERO {
                    inner.target.write_value(snapped);
                    inner.phase = GesturePhase::Idle;
                    inner.active_continuation = None;
                    drop(inner);
                    self.stop_driver();
                } else {
                    inner.active_continuation = Some(ActiveContinuation::Inertia {
                        continuation,
                        elapsed: Duration::ZERO,
                    });
                    inner.phase = GesturePhase::Inertia;
                    drop(inner);
                    self.ensure_running();
                }
            } else {
                // Already settled at snap point
                inner.target.write_value(snapped);
                inner.phase = GesturePhase::Idle;
                inner.active_continuation = None;
                drop(inner);
                self.stop_driver();
            }
        } else {
            // High momentum fling: project natural target
            let natural_target = current_pos + vel * inner.config.inertia.tau();
            let mut target = inner.config.snap.resolve(natural_target);

            // Enforce boundary bounds on target
            if let Some((lo, hi)) = inner.config.boundary.bounds() {
                target = target.clamp(lo, hi);
            }

            let continuation = InertiaContinuation::with_target(
                current_pos,
                vel,
                target,
                inner.config.inertia,
            );

            if continuation.duration() == Duration::ZERO {
                inner.target.write_value(target);
                inner.phase = GesturePhase::Idle;
                inner.active_continuation = None;
                drop(inner);
                self.stop_driver();
            } else {
                inner.active_continuation = Some(ActiveContinuation::Inertia {
                    continuation,
                    elapsed: Duration::ZERO,
                });
                inner.phase = GesturePhase::Inertia;
                drop(inner);
                self.ensure_running();
            }
        }
    }

    /// Pointer down alias for [`drag_start`](Self::drag_start).
    #[inline]
    pub fn pointer_down(&self, position: f64, time: Duration) {
        self.drag_start(position, time);
    }

    /// Pointer move alias for [`drag_move`](Self::drag_move).
    #[inline]
    pub fn pointer_move(&self, position: f64, time: Duration) {
        self.drag_move(position, time);
    }

    /// Pointer up alias for [`drag_end`](Self::drag_end).
    #[inline]
    pub fn pointer_up(&self, time: Duration) {
        self.drag_end(time);
    }

    /// Cancels any active drag or continuation, resets to [`GesturePhase::Idle`],
    /// and deactivates the frame loop driver.
    pub fn cancel(&self) {
        self.stop_driver();
        self.inner.borrow_mut().cancel();
    }

    /// Returns the current kinematic phase of the controller.
    #[inline]
    pub fn phase(&self) -> GesturePhase {
        self.inner.borrow().phase
    }

    /// Returns `true` if this controller is actively dragging or animating.
    #[inline]
    pub fn is_active(&self) -> bool {
        self.inner.borrow().phase != GesturePhase::Idle
    }

    /// Returns the number of active animations managed by this controller (0 when settled/idle, 1 when active).
    #[inline]
    pub fn active_count(&self) -> usize {
        if self.is_active() {
            1
        } else {
            0
        }
    }

    /// Returns `true` if the controller is in the [`GesturePhase::Dragging`] phase.
    #[inline]
    pub fn is_dragging(&self) -> bool {
        self.inner.borrow().phase == GesturePhase::Dragging
    }

    /// Returns `true` if the controller is actively running an inertia decay or bounce spring continuation.
    #[inline]
    pub fn is_animating(&self) -> bool {
        self.inner.borrow().phase.is_animating()
    }

    /// Returns `true` if the controller is completely dormant (`active_count == 0`).
    #[inline]
    pub fn is_dormant(&self) -> bool {
        self.active_count() == 0
    }

    /// Returns `true` if the underlying frame loop listener is currently registered with `oxidase::frame`.
    #[inline]
    pub fn is_frame_loop_active(&self) -> bool {
        self.driver_state.running.get()
    }

    /// Returns the current value sampled from the target.
    #[inline]
    pub fn current_position(&self) -> f64 {
        self.inner.borrow().target.sample()
    }

    /// Returns the instantaneous kinematic velocity in px/s.
    #[inline]
    pub fn current_velocity(&self) -> f64 {
        self.inner.borrow().current_velocity()
    }

    /// Access reference to the bound target.
    #[inline]
    pub fn target(&self) -> Target {
        self.inner.borrow().target.clone()
    }

    /// Returns a copy of the active controller configuration.
    #[inline]
    pub fn config(&self) -> GestureContinuationConfig {
        self.inner.borrow().config.clone()
    }

    /// Updates the active controller configuration.
    pub fn set_config(&self, config: GestureContinuationConfig) {
        let mut inner = self.inner.borrow_mut();
        inner.tracker = GestureTracker::new(config.tracker);
        inner.config = config;
    }

    /// Updates the boundary policy.
    pub fn set_boundary(&self, boundary: BoundaryPolicy) {
        self.inner.borrow_mut().config.boundary = boundary;
    }

    /// Updates the snap policy.
    pub fn set_snap(&self, snap: SnapPolicy) {
        self.inner.borrow_mut().config.snap = snap;
    }

    /// Updates the inertia decay configuration.
    pub fn set_inertia(&self, inertia: InertiaConfig) {
        self.inner.borrow_mut().config.inertia = inertia;
    }

    /// Steps active continuation forward by `dt` and writes the evaluated value to the target.
    ///
    /// Returns `true` if more frames are needed, or `false` if motion has settled.
    pub fn step(&self, dt: Duration) -> bool {
        let has_more = self.inner.borrow_mut().step(dt);
        if !has_more {
            self.stop_driver();
        }
        has_more
    }

    /// Steps active continuation forward by `dt`.
    ///
    /// On non-wasm, advances via `oxidase::frame::tick(dt)` if frame loop is active,
    /// or calls [`Self::step`] directly.
    pub fn tick(&self, dt: Duration) -> bool {
        if self.is_frame_loop_active() {
            #[cfg(not(target_arch = "wasm32"))]
            {
                oxidase::frame::tick(dt);
                return self.is_active();
            }
        }
        self.step(dt)
    }
}

impl<Target> Drop for GestureContinuationController<Target> {
    fn drop(&mut self) {
        if Rc::strong_count(&self.inner) <= 1 {
            self.stop_driver();
        }
    }
}
