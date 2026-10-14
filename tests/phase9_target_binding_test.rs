//! Integration test suite for kinetoxus gesture controller binding to SignalTarget and HandleTarget (Phase 9).

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;

#[test]
fn test_signal_target_drag_reactivity() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f64);
        let gesture = use_gesture_continuation(
            signal,
            GestureContinuationConfig::default().with_bounds(-100.0, 100.0),
        );

        assert_eq!(signal(), 0.0);
        assert_eq!(gesture.active_count(), 0);

        // 1. Drag move updates signal reactively
        gesture.drag_start(10.0, Duration::from_millis(0));
        assert_eq!(signal(), 10.0);
        assert_eq!(gesture.current_position(), 10.0);

        gesture.drag_move(35.0, Duration::from_millis(20));
        assert_eq!(signal(), 35.0);

        gesture.drag_move(85.0, Duration::from_millis(40));
        assert_eq!(signal(), 85.0);

        // 2. Clamping bounds apply reactively to signal
        gesture.drag_move(150.0, Duration::from_millis(60));
        assert_eq!(signal(), 100.0);

        rsx! {
            div { "Signal pos: {signal}" }
        }
    });
    dom.rebuild_in_place();
}

#[test]
fn test_handle_target_direct_mutation() {
    let raw_handle = Rc::new(RefCell::new(0.0f64));
    let handle_target = HandleTarget::new(raw_handle.clone());
    let gesture = GestureContinuationController::new(
        handle_target,
        GestureContinuationConfig::default().with_bounds(0.0, 200.0),
    );

    assert_eq!(*raw_handle.borrow(), 0.0);

    // Direct mutation without virtual dom
    gesture.drag_start(20.0, Duration::from_millis(0));
    assert_eq!(*raw_handle.borrow(), 20.0);

    gesture.drag_move(75.0, Duration::from_millis(30));
    assert_eq!(*raw_handle.borrow(), 75.0);

    gesture.drag_move(250.0, Duration::from_millis(60));
    assert_eq!(*raw_handle.borrow(), 200.0, "Handle must clamp to 200.0");
}

#[test]
fn test_signal_target_inertia_continuation_and_settle() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f64);
        let gesture = use_gesture_continuation(
            signal,
            GestureContinuationConfig::default()
                .with_inertia(InertiaConfig::from_tau(0.15))
                .with_bounds(0.0, 300.0),
        );

        // Fling gesture
        gesture.drag_start(0.0, Duration::from_millis(0));
        gesture.drag_move(40.0, Duration::from_millis(20));
        gesture.drag_move(90.0, Duration::from_millis(40));
        gesture.drag_end(Duration::from_millis(40));

        assert_eq!(gesture.phase(), GesturePhase::Inertia);

        // Step through frames and observe signal reactivity
        let mut prev = signal();
        for _ in 0..100 {
            let has_more = gesture.step(Duration::from_millis(16));
            let curr = signal();
            assert!(curr >= prev, "Signal value must progress forward monotonically");
            prev = curr;
            if !has_more {
                break;
            }
        }

        assert_eq!(gesture.phase(), GesturePhase::Idle);
        assert_eq!(gesture.active_count(), 0);
        assert!(signal() > 90.0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_handle_target_bounce_settle() {
    let handle = Rc::new(RefCell::new(0.0f64));
    let spring = kinetocore::spring::SpringConfig::new(1.0, 150.0, 25.0).unwrap();
    let gesture = GestureContinuationController::bind(
        handle.clone(),
        GestureContinuationConfig::default().with_bounce(
            0.0,
            100.0,
            spring,
        ),
    );

    // Overdrag below 0.0
    gesture.drag_start(50.0, Duration::from_millis(0));
    gesture.drag_move(-60.0, Duration::from_millis(30));
    assert!(*handle.borrow() < 0.0);

    // Release
    gesture.drag_end(Duration::from_millis(30));
    assert_eq!(gesture.phase(), GesturePhase::Bounce);

    // Step until settle
    let mut settled = false;
    for _ in 0..300 {
        if !gesture.step(Duration::from_millis(16)) {
            settled = true;
            break;
        }
    }

    assert!(settled, "Bounce spring must settle back to min 0.0");
    assert_eq!(gesture.phase(), GesturePhase::Idle);
    assert_eq!(gesture.active_count(), 0);
    assert!((*handle.borrow() - 0.0).abs() < 1e-4, "Handle must bounce back to min 0.0");
}

#[test]
fn test_signal_target_snap_points_settle() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f64);
        let gesture = use_gesture_continuation(
            signal,
            GestureContinuationConfig::default()
                .with_snap_points(vec![-100.0, 0.0, 100.0])
                .with_inertia(InertiaConfig::from_tau(0.15)),
        );

        // Fling right towards +100 snap point
        gesture.drag_start(0.0, Duration::from_millis(0));
        gesture.drag_move(30.0, Duration::from_millis(20));
        gesture.drag_move(70.0, Duration::from_millis(40));
        gesture.drag_end(Duration::from_millis(40));

        assert_eq!(gesture.phase(), GesturePhase::Inertia);

        // Advance frames until settle
        let mut settled = false;
        for _ in 0..100 {
            if !gesture.step(Duration::from_millis(16)) {
                settled = true;
                break;
            }
        }

        assert!(settled, "Must settle to snap point");
        assert_eq!(gesture.phase(), GesturePhase::Idle);
        assert_eq!(gesture.active_count(), 0);
        assert_eq!(signal(), 100.0, "Signal must settle exactly at snap point 100.0");

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_signal_target_snap_interval_settle() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f64);
        let gesture = use_gesture_continuation(
            signal,
            GestureContinuationConfig::default()
                .with_snap_interval(50.0, 0.0)
                .with_inertia(InertiaConfig::from_tau(0.15)),
        );

        // Fling to settle near 150.0
        gesture.drag_start(0.0, Duration::from_millis(0));
        gesture.drag_move(45.0, Duration::from_millis(20));
        gesture.drag_move(110.0, Duration::from_millis(40));
        gesture.drag_end(Duration::from_millis(40));

        assert_eq!(gesture.phase(), GesturePhase::Inertia);

        let mut settled = false;
        for _ in 0..100 {
            if !gesture.step(Duration::from_millis(16)) {
                settled = true;
                break;
            }
        }

        assert!(settled);
        assert_eq!(gesture.phase(), GesturePhase::Idle);
        // Position must be a multiple of 50.0
        let remainder = (signal() % 50.0).abs();
        assert!(remainder < 1e-4 || (50.0 - remainder).abs() < 1e-4, "Signal must settle on 50px grid: {}", signal());

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_signal_target_bounce_settle() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f64);
        let spring = kinetocore::spring::SpringConfig::new(1.0, 150.0, 25.0).unwrap();
        let gesture = use_gesture_continuation(
            signal,
            GestureContinuationConfig::default().with_bounce(-50.0, 50.0, spring),
        );

        // Overdrag beyond max 50.0
        gesture.drag_start(0.0, Duration::from_millis(0));
        gesture.drag_move(100.0, Duration::from_millis(30));
        // Elasticity applies: 50.0 + (100 - 50) * 0.35 = 67.5
        assert!((signal() - 67.5).abs() < 1e-4);

        // Release -> bounce spring activates
        gesture.drag_end(Duration::from_millis(30));
        assert_eq!(gesture.phase(), GesturePhase::Bounce);

        let mut settled = false;
        for _ in 0..300 {
            if !gesture.step(Duration::from_millis(16)) {
                settled = true;
                break;
            }
        }

        assert!(settled, "Bounce spring must settle back to boundary limit");
        assert_eq!(gesture.phase(), GesturePhase::Idle);
        assert!((signal() - 50.0).abs() < 1e-4, "Signal must bounce back to max 50.0");

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_signal_target_interruption_mid_flight() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f64);
        let gesture = use_gesture_continuation(
            signal,
            GestureContinuationConfig::default().with_bounds(-200.0, 200.0),
        );

        // Launch inertia
        gesture.drag_start(0.0, Duration::from_millis(0));
        gesture.drag_move(50.0, Duration::from_millis(20));
        gesture.drag_move(100.0, Duration::from_millis(40));
        gesture.drag_end(Duration::from_millis(40));
        assert_eq!(gesture.phase(), GesturePhase::Inertia);

        // Step 2 frames
        gesture.step(Duration::from_millis(16));
        gesture.step(Duration::from_millis(16));
        let pos_mid = signal();

        // Mid-flight drag interrupt
        gesture.drag_start(pos_mid, Duration::from_millis(80));
        assert_eq!(gesture.phase(), GesturePhase::Dragging);
        assert_eq!(signal(), pos_mid, "Interruption must seamlessly capture position C^0");

        // Drag backwards
        gesture.drag_move(pos_mid - 20.0, Duration::from_millis(100));
        assert_eq!(signal(), pos_mid - 20.0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_handle_target_inertia_continuation_and_settle() {
    let handle = Rc::new(RefCell::new(0.0f64));
    let gesture = GestureContinuationController::bind(
        handle.clone(),
        GestureContinuationConfig::default().with_inertia(InertiaConfig::from_tau(0.15)),
    );

    gesture.drag_start(0.0, Duration::from_millis(0));
    gesture.drag_move(30.0, Duration::from_millis(20));
    gesture.drag_move(80.0, Duration::from_millis(40));
    gesture.drag_end(Duration::from_millis(40));

    assert_eq!(gesture.phase(), GesturePhase::Inertia);

    let mut prev = *handle.borrow();
    let mut settled = false;
    for _ in 0..100 {
        let has_more = gesture.step(Duration::from_millis(16));
        let curr = *handle.borrow();
        assert!(curr >= prev, "Handle target must progress forward monotonically");
        prev = curr;
        if !has_more {
            settled = true;
            break;
        }
    }

    assert!(settled);
    assert_eq!(gesture.phase(), GesturePhase::Idle);
    assert_eq!(gesture.active_count(), 0);
    assert!(*handle.borrow() > 80.0);
}

#[test]
fn test_handle_target_snap_points_settle() {
    let handle = Rc::new(RefCell::new(0.0f64));
    let gesture = GestureContinuationController::bind(
        handle.clone(),
        GestureContinuationConfig::default()
            .with_snap_points(vec![-200.0, 0.0, 200.0])
            .with_inertia(InertiaConfig::from_tau(0.15)),
    );

    // Fling right towards +200 snap point
    gesture.drag_start(0.0, Duration::from_millis(0));
    gesture.drag_move(50.0, Duration::from_millis(20));
    gesture.drag_move(120.0, Duration::from_millis(40));
    gesture.drag_end(Duration::from_millis(40));

    assert_eq!(gesture.phase(), GesturePhase::Inertia);

    let mut settled = false;
    for _ in 0..100 {
        if !gesture.step(Duration::from_millis(16)) {
            settled = true;
            break;
        }
    }

    assert!(settled);
    assert_eq!(gesture.phase(), GesturePhase::Idle);
    assert_eq!(gesture.active_count(), 0);
    assert_eq!(*handle.borrow(), 200.0, "Handle must settle exactly at snap point 200.0");
}

#[test]
fn test_handle_target_snap_interval_settle() {
    let handle = Rc::new(RefCell::new(0.0f64));
    let gesture = GestureContinuationController::bind(
        handle.clone(),
        GestureContinuationConfig::default()
            .with_snap_interval(80.0, 0.0)
            .with_inertia(InertiaConfig::from_tau(0.15)),
    );

    // Fling right
    gesture.drag_start(0.0, Duration::from_millis(0));
    gesture.drag_move(40.0, Duration::from_millis(20));
    gesture.drag_move(90.0, Duration::from_millis(40));
    gesture.drag_end(Duration::from_millis(40));

    assert_eq!(gesture.phase(), GesturePhase::Inertia);

    let mut settled = false;
    for _ in 0..100 {
        if !gesture.step(Duration::from_millis(16)) {
            settled = true;
            break;
        }
    }

    assert!(settled);
    assert_eq!(gesture.phase(), GesturePhase::Idle);
    let val = *handle.borrow();
    let remainder = (val % 80.0).abs();
    assert!(remainder < 1e-4 || (80.0 - remainder).abs() < 1e-4, "Handle must settle on 80px grid: {val}");
}

#[test]
fn test_handle_target_interruption_mid_flight() {
    let handle = Rc::new(RefCell::new(0.0f64));
    let gesture = GestureContinuationController::bind(
        handle.clone(),
        GestureContinuationConfig::default().with_bounds(-150.0, 150.0),
    );

    gesture.drag_start(0.0, Duration::from_millis(0));
    gesture.drag_move(40.0, Duration::from_millis(20));
    gesture.drag_move(90.0, Duration::from_millis(40));
    gesture.drag_end(Duration::from_millis(40));
    assert_eq!(gesture.phase(), GesturePhase::Inertia);

    gesture.step(Duration::from_millis(16));
    gesture.step(Duration::from_millis(16));
    let pos_mid = *handle.borrow();

    // Interrupt mid-flight
    gesture.drag_start(pos_mid, Duration::from_millis(80));
    assert_eq!(gesture.phase(), GesturePhase::Dragging);
    assert_eq!(*handle.borrow(), pos_mid);

    gesture.drag_move(pos_mid - 30.0, Duration::from_millis(100));
    assert_eq!(*handle.borrow(), pos_mid - 30.0);
}

