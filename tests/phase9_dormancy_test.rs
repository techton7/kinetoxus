//! Integration test suite for kinetoxus gesture frame loop lifecycle, dormancy, and drop cleanup (Phase 9).

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;
use oxidase::frame::tick as tick_frame;

#[test]
fn test_dormancy_initial_state() {
    let target = Rc::new(RefCell::new(0.0f64));
    let gesture = GestureContinuationController::bind(
        target.clone(),
        GestureContinuationConfig::default(),
    );

    assert_eq!(gesture.active_count(), 0);
    assert!(gesture.is_dormant());
    assert!(!gesture.is_frame_loop_active());

    // External frame ticks while dormant do NOT mutate target or activate driver
    tick_frame(Duration::from_millis(50));
    assert_eq!(*target.borrow(), 0.0);
    assert_eq!(gesture.active_count(), 0);
    assert!(!gesture.is_frame_loop_active());
}

#[test]
fn test_frame_loop_subscription_on_release_and_dormancy_on_settle() {
    let target = Rc::new(RefCell::new(0.0f64));
    let inertia_config = InertiaConfig::from_tau(0.1)
        .with_velocity_cutoff(10.0)
        .with_position_tolerance(0.5);

    let gesture = GestureContinuationController::bind(
        target.clone(),
        GestureContinuationConfig::default()
            .with_inertia(inertia_config)
            .with_bounds(0.0, 500.0),
    );

    // 1. While actively dragging: active_count is 1, but frame loop driver is NOT running
    gesture.drag_start(0.0, Duration::from_millis(0));
    gesture.drag_move(50.0, Duration::from_millis(25));
    gesture.drag_move(100.0, Duration::from_millis(50));

    assert_eq!(gesture.active_count(), 1);
    assert!(!gesture.is_frame_loop_active(), "Frame loop driver should be dormant during direct drag");

    // 2. Release with momentum: activates frame loop
    gesture.drag_end(Duration::from_millis(50));
    assert!(gesture.is_frame_loop_active(), "Frame loop must be active during inertia continuation");
    assert_eq!(gesture.active_count(), 1);

    // 3. Advancing through shared frame loop
    tick_frame(Duration::from_millis(30));
    assert!(gesture.is_frame_loop_active());
    let mid_val = *target.borrow();
    assert!(mid_val > 100.0);

    // Advance frame loop until settled
    for _ in 0..50 {
        tick_frame(Duration::from_millis(16));
        if !gesture.is_frame_loop_active() {
            break;
        }
    }

    // 4. Upon settle: frame loop unsubscribed, complete dormancy
    assert!(
        !gesture.is_frame_loop_active(),
        "Frame loop must automatically unregister upon settle"
    );
    assert_eq!(gesture.active_count(), 0);
    assert!(gesture.is_dormant());
    assert_eq!(gesture.phase(), GesturePhase::Idle);

    let settled_val = *target.borrow();

    // 5. Subsequent ticks do nothing
    tick_frame(Duration::from_millis(50));
    assert_eq!(*target.borrow(), settled_val);
    assert!(!gesture.is_frame_loop_active());
    assert_eq!(gesture.active_count(), 0);
}

thread_local! {
    static SHOW_GESTURE_APP: RefCell<Option<Signal<bool>>> = const { RefCell::new(None) };
    static CAPTURED_GESTURE: RefCell<Option<GestureContinuationController<SignalTarget<f64>>>> = const { RefCell::new(None) };
}

#[component]
fn ChildGestureComponent() -> Element {
    let pos = use_signal(|| 0.0f64);
    let gesture = use_gesture_continuation(
        pos,
        GestureContinuationConfig::default().with_bounds(0.0, 500.0),
    );

    // Simulate drag and release into momentum immediately
    gesture.drag_start(0.0, Duration::from_millis(0));
    gesture.drag_move(60.0, Duration::from_millis(25));
    gesture.drag_move(120.0, Duration::from_millis(50));
    gesture.drag_end(Duration::from_millis(50));

    CAPTURED_GESTURE.with(|c| *c.borrow_mut() = Some(gesture.clone()));

    rsx! {
        div { "Child pos: {pos}" }
    }
}

#[component]
fn GestureLifecycleApp() -> Element {
    let show = use_signal(|| true);
    SHOW_GESTURE_APP.with(|c| *c.borrow_mut() = Some(show));

    rsx! {
        if show() {
            ChildGestureComponent {}
        }
    }
}

#[test]
fn test_component_unmount_drop_cleanup() {
    SHOW_GESTURE_APP.with(|c| *c.borrow_mut() = None);
    CAPTURED_GESTURE.with(|c| *c.borrow_mut() = None);

    let mut dom = VirtualDom::new(GestureLifecycleApp);
    dom.rebuild_in_place();

    let child_gesture = CAPTURED_GESTURE
        .with(|c| c.borrow().as_ref().expect("child gesture should be captured").clone());

    assert!(
        child_gesture.is_frame_loop_active(),
        "Frame loop must be active while child component is mounted and animating"
    );

    // Advance frame loop partially
    tick_frame(Duration::from_millis(30));

    // Unmount child component
    dom.in_runtime(|| {
        SHOW_GESTURE_APP.with(|c| {
            if let Some(mut sig) = *c.borrow() {
                sig.set(false);
            }
        });
    });

    let _mutations = dom.render_immediate_to_vec();

    // Child unmounted: use_drop ran and cancelled frame loop
    assert!(
        !child_gesture.is_frame_loop_active(),
        "Frame loop must unregister when component is unmounted via use_drop"
    );
    assert_eq!(child_gesture.phase(), GesturePhase::Idle);
    assert_eq!(child_gesture.active_count(), 0);
}

#[test]
fn test_controller_direct_drop_cleanup() {
    let target = Rc::new(RefCell::new(0.0f64));
    let gesture = GestureContinuationController::bind(
        target.clone(),
        GestureContinuationConfig::default().with_bounds(0.0, 500.0),
    );

    gesture.drag_start(0.0, Duration::from_millis(0));
    gesture.drag_move(50.0, Duration::from_millis(25));
    gesture.drag_move(100.0, Duration::from_millis(50));
    gesture.drag_end(Duration::from_millis(50));

    assert!(gesture.is_frame_loop_active());

    // Dropping controller cancels frame loop via RAII
    drop(gesture);

    // Subsequent tick should not crash or advance dropped handle
    tick_frame(Duration::from_millis(50));
}

#[test]
fn test_stale_hold_immediate_dormancy() {
    let target = Rc::new(RefCell::new(0.0f64));
    let gesture = GestureContinuationController::bind(
        target.clone(),
        GestureContinuationConfig::default(),
    );

    // Move to 50.0, then hold stationary for 200ms (> stale_threshold 100ms)
    gesture.drag_start(0.0, Duration::from_millis(0));
    gesture.drag_move(50.0, Duration::from_millis(50));

    // Release at 250ms (200ms after last sample)
    gesture.drag_end(Duration::from_millis(250));

    // Velocity is 0.0, no snap policy -> immediately settled to Idle
    assert_eq!(gesture.phase(), GesturePhase::Idle);
    assert_eq!(gesture.active_count(), 0);
    assert!(gesture.is_dormant());
    assert!(
        !gesture.is_frame_loop_active(),
        "Frame loop must NEVER start for stale release"
    );
    assert_eq!(*target.borrow(), 50.0);
}
