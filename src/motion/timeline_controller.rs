//! Timeline controller, target bindings, and frame driver coordinator.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use kinetocore::clock::ClockState;
use kinetocore::ease::Ease;
use kinetocore::interpolate::Interpolate;
use kinetocore::spring::SpringConfig;
use kinetocore::target::Target;
use kinetocore::timeline::clip::SpringClipEnd;
use kinetocore::timeline::mutation::{
    ActiveOverlapPolicy, ClipTarget, DynamicTimeline, InsertClipMutation,
    InsertSubTimelineMutation, RemoveClipMutation, RetargetClipMutation, TargetOwnershipHandoff,
    TimelineMutation,
};
use kinetocore::timeline::tree::{
    CompiledTimelineSnapshot, SubTimelineBuilder, SubTimelineNode, TimelineTreeBuilder,
};
use kinetocore::timeline::{Keyframe, Position, TimelineError, TrackBuilder};
use oxidase::frame::{start_frame_loop, FrameInfo, FrameLoopGuard};

use crate::target::{AnimationTarget, HandleTarget, IntoAnimationTarget, SignalTarget};

/// Trait for applying sampled values from a compiled timeline track to an animation target.
pub trait TargetBinding {
    /// Evaluates the track at `time` and writes the sampled value to the target.
    fn apply(&mut self, snapshot: &CompiledTimelineSnapshot, time: Duration);
}

/// Strongly typed binding connecting a named track in a compiled timeline to an [`AnimationTarget<T>`].
#[derive(Clone)]
pub struct TypedTargetBinding<Target: AnimationTarget<T>, T: Interpolate + 'static> {
    track_name: String,
    target: Target,
    _marker: std::marker::PhantomData<T>,
}

impl<Target: AnimationTarget<T>, T: Interpolate + 'static> TypedTargetBinding<Target, T> {
    /// Creates a new binding for `track_name` writing into `target`.
    pub fn new(track_name: impl Into<String>, target: Target) -> Self {
        Self {
            track_name: track_name.into(),
            target,
            _marker: std::marker::PhantomData,
        }
    }

    /// Track name associated with this binding.
    #[inline]
    pub fn track_name(&self) -> &str {
        &self.track_name
    }

    /// Access reference to the bound target.
    #[inline]
    pub fn target(&self) -> &Target {
        &self.target
    }
}

impl<Target: AnimationTarget<T>, T: Interpolate + 'static> TargetBinding
    for TypedTargetBinding<Target, T>
{
    fn apply(&mut self, snapshot: &CompiledTimelineSnapshot, time: Duration) {
        if let Some(val) = snapshot.sample_track::<T>(&self.track_name, time) {
            self.target.write_value(val);
        }
    }
}

pub(crate) struct TimelineInner {
    pub(crate) timeline: DynamicTimeline,
    pub(crate) bindings: Vec<Box<dyn TargetBinding>>,
}

impl TimelineInner {
    pub(crate) fn apply_bindings(&mut self, time: Duration) {
        let snapshot = self.timeline.snapshot();
        for binding in &mut self.bindings {
            binding.apply(snapshot, time);
        }
    }

    pub(crate) fn step_and_apply(&mut self, dt: Duration) -> bool {
        self.timeline.step(dt);
        let time = self.timeline.time();
        self.apply_bindings(time);
        self.timeline.is_playing() && !self.timeline.is_completed()
    }
}

#[derive(Default)]
struct DriverState {
    running: Cell<bool>,
    guard: RefCell<Option<FrameLoopGuard>>,
}

/// High-level controller managing playback, scrubbing, dynamic mutation, and frame synchronization for a timeline.
#[derive(Clone)]
pub struct TimelineController {
    inner: Rc<RefCell<TimelineInner>>,
    driver_state: Rc<DriverState>,
}

impl PartialEq for TimelineController {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for TimelineController {}

impl TimelineController {
    fn ensure_running(&self) {
        if !self.inner.borrow().timeline.is_playing() || self.inner.borrow().timeline.is_completed()
        {
            return;
        }

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

            let has_more = inner_rc.borrow_mut().step_and_apply(info.delta);
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

    fn stop_driver(&self) {
        self.driver_state.running.set(false);
        let _ = self.driver_state.guard.borrow_mut().take();
    }

    /// Starts or resumes playback and activates the frame loop driver.
    pub fn play(&self) {
        let mut inner = self.inner.borrow_mut();
        let was_completed = inner.timeline.is_completed();
        inner.timeline.play();
        if was_completed {
            let time = inner.timeline.time();
            inner.apply_bindings(time);
        }
        drop(inner);
        self.ensure_running();
    }

    /// Pauses playback and deactivates the frame loop driver.
    pub fn pause(&self) {
        self.inner.borrow_mut().timeline.pause();
        self.stop_driver();
    }

    /// Toggles between playing and paused states.
    pub fn toggle(&self) {
        if self.is_playing() {
            self.pause();
        } else {
            self.play();
        }
    }

    /// Reverses playback direction and ensures active playback towards $t = 0$.
    pub fn reverse(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.timeline.reverse();
        inner.timeline.play();
        let time = inner.timeline.time();
        inner.apply_bindings(time);
        drop(inner);
        self.ensure_running();
    }

    /// Restarts playback from $t = 0$ in the forward direction.
    pub fn restart(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.timeline.restart();
        let time = inner.timeline.time();
        inner.apply_bindings(time);
        drop(inner);
        self.ensure_running();
    }

    /// Seeks playback playhead directly to `time`, synchronously updating all bound targets.
    pub fn seek(&self, time: Duration) {
        let mut inner = self.inner.borrow_mut();
        inner.timeline.seek(time);
        inner.apply_bindings(time);
    }

    /// Sets normalized progress fraction in $[0.0, 1.0]$, synchronously updating all bound targets.
    pub fn set_progress(&self, progress: f32) {
        let mut inner = self.inner.borrow_mut();
        inner.timeline.set_progress(progress);
        let time = inner.timeline.time();
        inner.apply_bindings(time);
    }

    /// Current progress fraction in $[0.0, 1.0]$.
    #[inline]
    pub fn progress(&self) -> f32 {
        self.inner.borrow().timeline.progress()
    }

    /// Current playhead position timestamp.
    #[inline]
    pub fn time(&self) -> Duration {
        self.inner.borrow().timeline.time()
    }

    /// Total timeline cycle duration.
    #[inline]
    pub fn duration(&self) -> Duration {
        self.inner.borrow().timeline.duration()
    }

    /// Returns `true` if playback is actively advancing.
    #[inline]
    pub fn is_playing(&self) -> bool {
        self.inner.borrow().timeline.is_playing()
    }

    /// Returns `true` if playback is currently paused.
    #[inline]
    pub fn is_paused(&self) -> bool {
        self.inner.borrow().timeline.is_paused()
    }

    /// Returns `true` if playback has reached completion.
    #[inline]
    pub fn is_completed(&self) -> bool {
        self.inner.borrow().timeline.is_completed()
    }

    /// Samples instantaneous value of track `track_name` at current playhead time.
    #[inline]
    pub fn sample<T: Interpolate + 'static>(&self, track_name: &str) -> Option<T> {
        self.inner.borrow().timeline.sample(track_name)
    }

    /// Statelessly samples track `track_name` at an arbitrary timestamp `time`.
    #[inline]
    pub fn sample_at<T: Interpolate + 'static>(&self, track_name: &str, time: Duration) -> Option<T> {
        self.inner.borrow().timeline.sample_at(track_name, time)
    }

    /// Cancels playback and unregisters the frame loop listener.
    pub fn cancel(&self) {
        self.pause();
    }

    /// Advances the timeline clock by delta duration `dt` and synchronously updates all bound targets.
    ///
    /// Returns `true` if playback should continue, or `false` if completed.
    pub fn step(&self, dt: Duration) -> bool {
        let mut inner = self.inner.borrow_mut();
        let state = inner.timeline.step(dt);
        let time = inner.timeline.time();
        inner.apply_bindings(time);
        state == ClockState::Active
    }

    /// Drives the timeline clock forward by delta duration `dt`, updating bound targets. Alias for [`step`](Self::step).
    #[inline]
    pub fn tick(&self, dt: Duration) -> bool {
        self.step(dt)
    }

    /// Synchronously seeks the timeline to the timestamp of a named label and updates all targets in 0 ticks.
    ///
    /// Supports both root labels (e.g. `"intro"`) and namespaced sub-timeline paths (e.g. `"modal::card::apex"`).
    /// Returns `true` if the label was found and sought, or `false` if the label does not exist.
    pub fn seek_label(&self, label: &str) -> bool {
        let label_time = {
            let inner = self.inner.borrow();
            inner.timeline.snapshot().label_str(label)
        };
        if let Some(time) = label_time {
            self.seek(time);
            true
        } else {
            false
        }
    }

    /// Returns `true` if the underlying frame loop listener is currently registered with `oxidase::frame`.
    #[inline]
    pub fn is_frame_loop_active(&self) -> bool {
        self.driver_state.running.get()
    }

    /// Sets playback speed multiplier (`1.0` is normal speed).
    #[inline]
    pub fn set_time_scale(&self, scale: f64) {
        self.inner.borrow_mut().timeline.set_time_scale(scale);
    }

    /// Returns the current playback speed multiplier.
    #[inline]
    pub fn time_scale(&self) -> f64 {
        self.inner.borrow().timeline.transport().time_scale()
    }

    /// Returns the current published execution snapshot epoch identifier.
    #[inline]
    pub fn epoch(&self) -> u64 {
        self.inner.borrow().timeline.epoch()
    }

    /// Dynamically mutates the underlying timeline graph, atomically swapping the execution snapshot
    /// and immediately updating all bound reactive targets at the preserved playhead position.
    pub fn mutate(
        &self,
        mutation: TimelineMutation,
    ) -> Result<Option<TargetOwnershipHandoff>, TimelineError> {
        let mut inner = self.inner.borrow_mut();
        let (_, handoff) = inner.timeline.mutate(mutation)?;
        let time = inner.timeline.time();
        inner.apply_bindings(time);
        drop(inner);
        self.ensure_running();
        Ok(handoff)
    }

    /// Dynamically inserts a tween clip on `track`.
    pub fn insert_clip<T: Interpolate + Send + Sync + 'static>(
        &self,
        track: impl Into<String>,
        position: Position,
        from: T,
        to: T,
        duration: Duration,
        ease: Ease,
        policy: ActiveOverlapPolicy,
    ) -> Result<(), TimelineError> {
        let mutation = TimelineMutation::InsertClip(InsertClipMutation::tween(
            track, from, to, duration, ease, position, policy,
        ));
        self.mutate(mutation)?;
        Ok(())
    }

    /// Dynamically removes a clip from `track` with explicit target ownership handoff.
    pub fn remove_clip(
        &self,
        track: impl Into<String>,
        target: impl Into<ClipTarget>,
    ) -> Result<Option<TargetOwnershipHandoff>, TimelineError> {
        let mutation = TimelineMutation::RemoveClip(RemoveClipMutation::by_id(track, target));
        self.mutate(mutation)
    }

    /// Dynamically retargets an active clip with guaranteed C0 continuity.
    pub fn retarget_clip<T: Interpolate + Send + Sync + 'static>(
        &self,
        track: impl Into<String>,
        target: impl Into<ClipTarget>,
        new_to: T,
        duration: Duration,
        ease: Ease,
        policy: ActiveOverlapPolicy,
    ) -> Result<(), TimelineError> {
        let mutation = TimelineMutation::RetargetClip(RetargetClipMutation::tween_target(
            track, target, new_to, duration, ease, policy,
        ));
        self.mutate(mutation)?;
        Ok(())
    }

    /// Dynamically swaps or appends a child sub-timeline graph.
    pub fn swap_subtimeline(
        &self,
        parent_scope: Option<String>,
        node: SubTimelineNode,
        policy: ActiveOverlapPolicy,
    ) -> Result<(), TimelineError> {
        let mutation =
            TimelineMutation::InsertSubTimeline(InsertSubTimelineMutation::new(parent_scope, node, policy));
        self.mutate(mutation)?;
        Ok(())
    }

    /// Directly binds a child sub-timeline graph into this controller at runtime.
    pub fn add_timeline(
        &self,
        parent_scope: Option<String>,
        node: SubTimelineNode,
        policy: ActiveOverlapPolicy,
    ) -> Result<(), TimelineError> {
        self.swap_subtimeline(parent_scope, node, policy)
    }
}

/// Declarative builder for authoring multi-track timeline graphs and binding targets within Dioxus hooks.
pub struct TimelineHookBuilder {
    builder: TimelineTreeBuilder,
    bindings: Vec<Box<dyn TargetBinding>>,
    autoplay: bool,
}

impl Default for TimelineHookBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TimelineHookBuilder {
    /// Creates a new empty `TimelineHookBuilder`.
    pub fn new() -> Self {
        Self {
            builder: TimelineTreeBuilder::new(),
            bindings: Vec::new(),
            autoplay: false,
        }
    }

    /// Sets whether playback starts automatically on creation. Default is `false`.
    pub fn autoplay(&mut self, autoplay: bool) -> &mut Self {
        self.autoplay = autoplay;
        self
    }

    /// Sets whether the timeline starts in the paused state. Inverse of `autoplay`.
    pub fn paused(&mut self, paused: bool) -> &mut Self {
        self.autoplay = !paused;
        self
    }

    /// Binds a reactive Dioxus [`SignalTarget<T>`] or [`dioxus::prelude::Signal<T>`] to a named timeline track.
    pub fn bind_signal<T: Interpolate + Clone + 'static>(
        &mut self,
        track_name: impl Into<String>,
        signal: impl Into<SignalTarget<T>>,
    ) -> &mut Self {
        let target = signal.into();
        self.bind_target(track_name, target)
    }

    /// Binds an interior-mutable [`HandleTarget<T>`] or [`std::rc::Rc<std::cell::RefCell<T>>`] to a named timeline track.
    pub fn bind_handle<T: Interpolate + Clone + 'static>(
        &mut self,
        track_name: impl Into<String>,
        handle: impl Into<HandleTarget<T>>,
    ) -> &mut Self {
        let target = handle.into();
        self.bind_target(track_name, target)
    }

    /// Binds any generic [`AnimationTarget<T>`] to a named timeline track.
    pub fn bind_target<T: Interpolate + 'static, Target: AnimationTarget<T>>(
        &mut self,
        track_name: impl Into<String>,
        target: Target,
    ) -> &mut Self {
        let binding = TypedTargetBinding::new(track_name, target);
        self.bindings.push(Box::new(binding));
        self
    }

    /// Concurrently scopes clip authoring to a typed track, sets its initial value,
    /// and binds the target in a single call.
    pub fn track<T: Interpolate + Send + Sync + 'static, Target: IntoAnimationTarget<T>>(
        &mut self,
        name: impl Into<String>,
        target: Target,
        initial_val: T,
        f: impl FnOnce(&mut TrackBuilder<T>),
    ) -> &mut Self {
        let name_str = name.into();
        let anim_target = target.into_target();
        self.bind_target(name_str.clone(), anim_target);
        self.builder.track(name_str, initial_val, f);
        self
    }

    /// Concurrently scopes clip authoring to a typed track, samples the initial value from the target,
    /// and binds the target in a single call.
    pub fn track_auto<T: Interpolate + Send + Sync + 'static, Target: IntoAnimationTarget<T>>(
        &mut self,
        name: impl Into<String>,
        target: Target,
        f: impl FnOnce(&mut TrackBuilder<T>),
    ) -> &mut Self {
        let anim_target = target.into_target();
        let initial_val = anim_target.sample();
        self.track(name, anim_target, initial_val, f)
    }

    /// Declares a nested child sub-timeline with its own scoped builder and target bindings.
    pub fn sub_timeline(
        &mut self,
        name: impl Into<String>,
        position: Position,
        f: impl FnOnce(&mut SubTimelineHookBuilder),
    ) -> &mut Self {
        let name_str = name.into();
        let mut sub_builder =
            SubTimelineHookBuilder::new(name_str, position, &mut self.bindings);
        f(&mut sub_builder);
        let node = sub_builder.build_node();
        self.builder.add_sub_timeline_node(node);
        self
    }

    /// Attaches an already composed child sub-timeline node directly into the master parent controller.
    pub fn add_timeline(&mut self, node: SubTimelineNode) -> &mut Self {
        self.builder.add_sub_timeline_node(node);
        self
    }

    /// Adds a named synchronization label at `position`.
    pub fn add_label(
        &mut self,
        name: impl Into<String>,
        position: impl Into<Position>,
    ) -> &mut Self {
        self.builder.add_label(name, position.into());
        self
    }

    /// Alias for [`add_label`](Self::add_label).
    #[inline]
    pub fn label(&mut self, name: impl Into<String>, position: impl Into<Position>) -> &mut Self {
        self.add_label(name, position)
    }

    /// Explicitly sets an initial value for a track.
    pub fn set_initial_value<T: Interpolate + Send + Sync + 'static>(
        &mut self,
        track: impl Into<String>,
        value: T,
    ) -> &mut Self {
        self.builder.set_initial_value(track, value);
        self
    }

    /// Scopes clip authoring without binding a target.
    pub fn raw_track<T: Interpolate + Send + Sync + 'static>(
        &mut self,
        name: impl Into<String>,
        f: impl FnOnce(&mut TrackBuilder<T>),
    ) -> &mut Self {
        self.builder.raw_track(name, f);
        self
    }

    /// Adds a continuous `Tween` clip interpolating between `from` and `to` over `duration`.
    pub fn tween<T: Interpolate + Send + Sync + 'static>(
        &mut self,
        track: impl Into<String>,
        from: T,
        to: T,
        duration: Duration,
        ease: Ease,
        position: impl Into<Position>,
    ) -> &mut Self {
        self.builder.tween(track, from, to, duration, ease, position);
        self
    }

    /// Adds an instant `Set` clip updating `track` to `value` at `position`.
    pub fn set<T: Interpolate + Send + Sync + 'static>(
        &mut self,
        track: impl Into<String>,
        value: T,
        position: impl Into<Position>,
    ) -> &mut Self {
        self.builder.set(track, value, position);
        self
    }

    /// Adds a `Hold` clip maintaining `value` constant over `duration`.
    pub fn hold<T: Interpolate + Send + Sync + 'static>(
        &mut self,
        track: impl Into<String>,
        value: T,
        duration: Duration,
        position: impl Into<Position>,
    ) -> &mut Self {
        self.builder.hold(track, value, duration, position);
        self
    }

    /// Adds a multi-keyframe clip over `duration`.
    pub fn keyframes<T: Interpolate + Send + Sync + 'static>(
        &mut self,
        track: impl Into<String>,
        keyframes: Vec<Keyframe<T>>,
        duration: Duration,
        position: impl Into<Position>,
    ) -> &mut Self {
        self.builder.keyframes(track, keyframes, duration, position);
        self
    }

    /// Adds a physics spring-driven clip with analytical settle resolution.
    pub fn spring<T: Interpolate + Send + Sync + 'static>(
        &mut self,
        track: impl Into<String>,
        from: T,
        to: T,
        config: SpringConfig,
        end_policy: SpringClipEnd,
        position: impl Into<Position>,
    ) -> &mut Self {
        self.builder.spring(track, from, to, config, end_policy, position);
        self
    }

    /// Compiles the authoring IR and builds a [`TimelineController`], immediately applying initial values at $t = 0$.
    pub fn build(self) -> Result<TimelineController, TimelineError> {
        let mut timeline = DynamicTimeline::new(self.builder)?;
        if !self.autoplay {
            timeline.pause();
        }

        let mut inner = TimelineInner {
            timeline,
            bindings: self.bindings,
        };

        // Immediately evaluate and apply t = 0 on creation so targets take the initial values
        let time = inner.timeline.time();
        inner.apply_bindings(time);

        let controller = TimelineController {
            inner: Rc::new(RefCell::new(inner)),
            driver_state: Rc::new(DriverState::default()),
        };

        if self.autoplay {
            controller.play();
        }

        Ok(controller)
    }
}

/// Scoped builder for authoring nested sub-timelines and binding reactive targets within Dioxus hooks.
pub struct SubTimelineHookBuilder<'a> {
    prefix: String,
    builder: SubTimelineBuilder,
    bindings: &'a mut Vec<Box<dyn TargetBinding>>,
}

impl<'a> SubTimelineHookBuilder<'a> {
    /// Creates a new `SubTimelineHookBuilder` scoped with `name` and target binding accumulator.
    pub fn new(
        name: impl Into<String>,
        position: Position,
        bindings: &'a mut Vec<Box<dyn TargetBinding>>,
    ) -> Self {
        let name_str = name.into();
        Self {
            prefix: name_str.clone(),
            builder: SubTimelineBuilder::new(name_str, position),
            bindings,
        }
    }

    fn new_with_prefix(
        name: String,
        prefix: String,
        position: Position,
        bindings: &'a mut Vec<Box<dyn TargetBinding>>,
    ) -> Self {
        Self {
            prefix,
            builder: SubTimelineBuilder::new(name, position),
            bindings,
        }
    }

    /// Sets the local time dilation factor (speed multiplier, strictly > 0.0).
    pub fn set_time_scale(&mut self, scale: f64) -> Result<&mut Self, TimelineError> {
        self.builder.set_time_scale(scale)?;
        Ok(self)
    }

    /// Sets finite repetition count and inter-cycle delay.
    pub fn set_repeat(&mut self, count: usize, delay: Duration) -> &mut Self {
        self.builder.set_repeat(count, delay);
        self
    }

    /// Adds a named synchronization label within this sub-timeline scope.
    pub fn add_label(&mut self, name: impl Into<String>, position: Position) -> &mut Self {
        self.builder.add_label(name, position);
        self
    }

    /// Scopes clip authoring to a typed track, sets its initial value, and binds the target with qualified path.
    pub fn track<T: Interpolate + Send + Sync + 'static, Target: IntoAnimationTarget<T>>(
        &mut self,
        name: impl Into<String>,
        target: Target,
        initial_val: T,
        f: impl FnOnce(&mut TrackBuilder<T>),
    ) -> &mut Self {
        let name_str = name.into();
        let full_name = format!("{}::{}", self.prefix, name_str);
        let anim_target = target.into_target();
        let binding = TypedTargetBinding::new(full_name, anim_target);
        self.bindings.push(Box::new(binding));
        self.builder.track(name_str, initial_val, f);
        self
    }

    /// Binds a reactive Dioxus [`SignalTarget<T>`] or [`dioxus::prelude::Signal<T>`] within this sub-timeline scope.
    pub fn bind_signal<T: Interpolate + Clone + 'static>(
        &mut self,
        track_name: impl Into<String>,
        signal: impl Into<SignalTarget<T>>,
    ) -> &mut Self {
        let target = signal.into();
        self.bind_target(track_name, target)
    }

    /// Binds an interior-mutable [`HandleTarget<T>`] or [`Rc<RefCell<T>>`] within this sub-timeline scope.
    pub fn bind_handle<T: Interpolate + Clone + 'static>(
        &mut self,
        track_name: impl Into<String>,
        handle: impl Into<HandleTarget<T>>,
    ) -> &mut Self {
        let target = handle.into();
        self.bind_target(track_name, target)
    }

    /// Binds any generic [`AnimationTarget<T>`] within this sub-timeline scope with fully-qualified path.
    pub fn bind_target<T: Interpolate + 'static, Target: AnimationTarget<T>>(
        &mut self,
        track_name: impl Into<String>,
        target: Target,
    ) -> &mut Self {
        let name_str = track_name.into();
        let full_name = format!("{}::{}", self.prefix, name_str);
        let binding = TypedTargetBinding::new(full_name, target);
        self.bindings.push(Box::new(binding));
        self
    }

    /// Adds a continuous `Tween` clip within this sub-timeline scope.
    pub fn tween<T: Interpolate + Send + Sync + 'static>(
        &mut self,
        track: impl Into<String>,
        from: T,
        to: T,
        duration: Duration,
        ease: Ease,
        position: impl Into<Position>,
    ) -> &mut Self {
        self.builder.tween(track, from, to, duration, ease, position);
        self
    }

    /// Adds an instant `Set` clip within this sub-timeline scope.
    pub fn set<T: Interpolate + Send + Sync + 'static>(
        &mut self,
        track: impl Into<String>,
        value: T,
        position: impl Into<Position>,
    ) -> &mut Self {
        self.builder.set(track, value, position);
        self
    }

    /// Adds a `Hold` clip within this sub-timeline scope.
    pub fn hold<T: Interpolate + Send + Sync + 'static>(
        &mut self,
        track: impl Into<String>,
        value: T,
        duration: Duration,
        position: impl Into<Position>,
    ) -> &mut Self {
        self.builder.hold(track, value, duration, position);
        self
    }

    /// Adds a multi-keyframe clip within this sub-timeline scope.
    pub fn keyframes<T: Interpolate + Send + Sync + 'static>(
        &mut self,
        track: impl Into<String>,
        keyframes: Vec<Keyframe<T>>,
        duration: Duration,
        position: impl Into<Position>,
    ) -> &mut Self {
        self.builder.keyframes(track, keyframes, duration, position);
        self
    }

    /// Adds a physics spring-driven clip within this sub-timeline scope.
    pub fn spring<T: Interpolate + Send + Sync + 'static>(
        &mut self,
        track: impl Into<String>,
        from: T,
        to: T,
        config: SpringConfig,
        end_policy: SpringClipEnd,
        position: impl Into<Position>,
    ) -> &mut Self {
        self.builder.spring(track, from, to, config, end_policy, position);
        self
    }

    /// Scopes clip authoring to a typed track, sampling initial value from target.
    pub fn track_auto<T: Interpolate + Send + Sync + 'static, Target: IntoAnimationTarget<T>>(
        &mut self,
        name: impl Into<String>,
        target: Target,
        f: impl FnOnce(&mut TrackBuilder<T>),
    ) -> &mut Self {
        let anim_target = target.into_target();
        let initial_val = anim_target.sample();
        self.track(name, anim_target, initial_val, f)
    }

    /// Declares a further nested sub-timeline within this sub-timeline scope.
    pub fn sub_timeline(
        &mut self,
        name: impl Into<String>,
        position: Position,
        f: impl FnOnce(&mut SubTimelineHookBuilder),
    ) -> &mut Self {
        let name_str = name.into();
        let nested_prefix = format!("{}::{}", self.prefix, name_str);
        let mut nested_builder =
            SubTimelineHookBuilder::new_with_prefix(name_str, nested_prefix, position, self.bindings);
        f(&mut nested_builder);
        let node = nested_builder.build_node();
        self.builder.add_sub_timeline_node(node);
        self
    }

    /// Packages this scoped builder into a `SubTimelineNode`.
    pub fn build_node(self) -> SubTimelineNode {
        self.builder.build_node()
    }
}
