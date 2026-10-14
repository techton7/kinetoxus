//! Integration test suite for kinetoxus gesture drag moves, release momentum,
//! boundary policies, and snapping contracts (Phase 9).

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use kinetocore::spring::SpringConfig;
use kinetoxus::prelude::*;

#[test]
fn test_drag_move_unconstrained() {
    let target = Rc::new(RefCell::new(0.0f64));
    let controller = GestureContinuationController::bind(
        target.clone(),
        GestureContinuationConfig::default(),
    );

    assert_eq!(*target.borrow(), 0.0);
    assert!(controller.phase().is_idle());
    assert_eq!(controller.active_count(), 0);

    // 1. Initiate drag
    controller.drag_start(10.0, Duration::from_millis(0));
    assert_eq!(*target.borrow(), 10.0);
    assert_eq!(controller.phase(), GesturePhase::Dragging);
    assert!(controller.is_dragging());
    assert_eq!(controller.active_count(), 1);

    // 2. Drag moves update target synchronously
    controller.drag_move(25.0, Duration::from_millis(20));
    assert_eq!(*target.borrow(), 25.0);

    controller.drag_move(70.0, Duration::from_millis(50));
    assert_eq!(*target.borrow(), 70.0);

    // 3. Pointer move alias works identically
    controller.pointer_move(120.0, Duration::from_millis(80));
    assert_eq!(*target.borrow(), 120.0);
}

#[test]
fn test_drag_move_boundary_clamp() {
    let target = Rc::new(RefCell::new(0.0f64));
    let controller = GestureContinuationController::bind(
        target.clone(),
        GestureContinuationConfig::default().with_bounds(0.0, 100.0),
    );

    controller.drag_start(50.0, Duration::from_millis(0));
    assert_eq!(*target.borrow(), 50.0);

    // Move within bounds
    controller.drag_move(80.0, Duration::from_millis(20));
    assert_eq!(*target.borrow(), 80.0);

    // Move beyond upper clamp limit
    controller.drag_move(150.0, Duration::from_millis(40));
    assert_eq!(*target.borrow(), 100.0, "Position must be clamped to max 100.0");

    // Move below lower clamp limit
    controller.drag_move(-40.0, Duration::from_millis(60));
    assert_eq!(*target.borrow(), 0.0, "Position must be clamped to min 0.0");
}

#[test]
fn test_drag_move_boundary_bounce_elasticity() {
    let target = Rc::new(RefCell::new(0.0f64));
    let config = GestureContinuationConfig::default().with_bounce(
        0.0,
        100.0,
        SpringConfig::DEFAULT,
    );
    let controller = GestureContinuationController::bind(target.clone(), config);

    controller.drag_start(50.0, Duration::from_millis(0));
    assert_eq!(*target.borrow(), 50.0);

    // Overdrag beyond max 100.0 by 100.0 px
    // With default elasticity 0.35: 100.0 + 100.0 * 0.35 = 135.0
    controller.drag_move(200.0, Duration::from_millis(30));
    let val = *target.borrow();
    assert!(
        (val - 135.0).abs() < 1e-4,
        "Overdrag displacement must scale by 0.35 elasticity, got {val}"
    );

    // Overdrag below min 0.0 by -50.0 px: 0.0 + (-50.0) * 0.35 = -17.5
    controller.drag_move(-50.0, Duration::from_millis(60));
    let val_lo = *target.borrow();
    assert!(
        (val_lo - (-17.5)).abs() < 1e-4,
        "Overdrag displacement must scale by 0.35 elasticity, got {val_lo}"
    );
}

#[test]
fn test_release_momentum_inertia_decay() {
    let target = Rc::new(RefCell::new(0.0f64));
    let inertia_config = InertiaConfig::from_tau(0.2)
        .with_velocity_cutoff(10.0)
        .with_position_tolerance(0.5);

    let config = GestureContinuationConfig::default()
        .with_inertia(inertia_config)
        .with_bounds(0.0, 500.0);

    let controller = GestureContinuationController::bind(target.clone(), config);

    // Rapid drag move simulating fling: 0 to 100 in 50ms = 2000 px/s
    controller.drag_start(0.0, Duration::from_millis(0));
    controller.drag_move(50.0, Duration::from_millis(25));
    controller.drag_move(100.0, Duration::from_millis(50));

    let vel = controller.current_velocity();
    assert!(vel > 1000.0, "Estimated fling velocity should be high, got {vel}");

    // Release at 50ms
    controller.drag_end(Duration::from_millis(50));
    assert_eq!(controller.phase(), GesturePhase::Inertia);
    assert!(controller.is_animating());
    assert_eq!(controller.active_count(), 1);

    // Step simulation forward
    let initial_pos = *target.borrow();
    controller.step(Duration::from_millis(50));
    let mid_pos = *target.borrow();
    assert!(
        mid_pos > initial_pos,
        "Position should advance forward during inertia decay"
    );

    // Step until settled
    let mut settled = false;
    for _ in 0..100 {
        let has_more = controller.step(Duration::from_millis(16));
        if !has_more {
            settled = true;
            break;
        }
    }

    assert!(settled, "Inertia continuation must settle within bounded frames");
    assert_eq!(controller.phase(), GesturePhase::Idle);
    assert_eq!(controller.active_count(), 0);
    assert!(controller.is_dormant());
}

#[test]
fn test_release_snapping_interval() {
    let target = Rc::new(RefCell::new(0.0f64));
    let config = GestureContinuationConfig::default()
        .with_inertia(InertiaConfig::from_tau(0.2))
        .with_snap_interval(50.0, 0.0);

    let controller = GestureContinuationController::bind(target.clone(), config);

    // Controlled fling: 0 to 20 over 100ms = 200 px/s.
    // Natural target = 20 + 200 * 0.2 = 60.0.
    // Snaps to nearest interval 50.0.
    controller.drag_start(0.0, Duration::from_millis(0));
    controller.drag_move(20.0, Duration::from_millis(100));
    controller.drag_end(Duration::from_millis(100));

    // Step until settle
    for _ in 0..100 {
        if !controller.step(Duration::from_millis(16)) {
            break;
        }
    }

    assert_eq!(controller.phase(), GesturePhase::Idle);
    assert_eq!(controller.active_count(), 0);
    let final_pos = *target.borrow();
    assert!(
        (final_pos - 50.0).abs() < 1e-4,
        "Should snap to nearest interval 50.0, got {final_pos}"
    );
}

#[test]
fn test_release_snapping_points() {
    let target = Rc::new(RefCell::new(0.0f64));
    let config = GestureContinuationConfig::default()
        .with_inertia(InertiaConfig::from_tau(0.2))
        .with_snap_points(vec![0.0, 100.0, 250.0]);

    let controller = GestureContinuationController::bind(target.clone(), config);

    // Controlled fling: 0 to 80 over 100ms = 800 px/s.
    // Natural target = 80 + 800 * 0.2 = 240.0.
    // Nearest anchor in [0.0, 100.0, 250.0] is 250.0 (|250 - 240| = 10 vs |100 - 240| = 140).
    controller.drag_start(0.0, Duration::from_millis(0));
    controller.drag_move(80.0, Duration::from_millis(100));
    controller.drag_end(Duration::from_millis(100));

    for _ in 0..100 {
        if !controller.step(Duration::from_millis(16)) {
            break;
        }
    }

    assert_eq!(controller.phase(), GesturePhase::Idle);
    let final_pos = *target.borrow();
    assert!(
        (final_pos - 250.0).abs() < 1e-4,
        "Should snap to discrete anchor 250.0, got {final_pos}"
    );
}

#[test]
fn test_release_bounce_spring_restoration() {
    let target = Rc::new(RefCell::new(0.0f64));
    let spring_config = SpringConfig::new(1.0, 150.0, 20.0).unwrap();
    let config = GestureContinuationConfig::default().with_bounce(
        0.0,
        100.0,
        spring_config,
    );

    let controller = GestureContinuationController::bind(target.clone(), config);

    // Drag past 100.0 to 180.0
    controller.drag_start(50.0, Duration::from_millis(0));
    controller.drag_move(180.0, Duration::from_millis(50));
    let overdrag_pos = *target.borrow();
    assert!(overdrag_pos > 100.0);

    // Release while out of bounds
    controller.drag_end(Duration::from_millis(50));
    assert_eq!(controller.phase(), GesturePhase::Bounce);
    assert!(controller.is_animating());
    assert_eq!(controller.active_count(), 1);

    // Spring should restore back to max 100.0
    let mut settled = false;
    for _ in 0..150 {
        if !controller.step(Duration::from_millis(16)) {
            settled = true;
            break;
        }
    }

    assert!(settled, "Bounce spring must settle back to boundary");
    assert_eq!(controller.phase(), GesturePhase::Idle);
    assert_eq!(controller.active_count(), 0);
    let final_pos = *target.borrow();
    assert!(
        (final_pos - 100.0).abs() < 1e-4,
        "Bounce spring must settle exactly at 100.0, got {final_pos}"
    );
}

#[test]
fn test_mid_flight_drag_interruption() {
    let target = Rc::new(RefCell::new(0.0f64));
    let controller = GestureContinuationController::bind(
        target.clone(),
        GestureContinuationConfig::default().with_bounds(0.0, 500.0),
    );

    // High velocity fling
    controller.drag_start(0.0, Duration::from_millis(0));
    controller.drag_move(50.0, Duration::from_millis(25));
    controller.drag_move(100.0, Duration::from_millis(50));
    controller.drag_end(Duration::from_millis(50));

    assert_eq!(controller.phase(), GesturePhase::Inertia);

    // Advance 50ms into inertia continuation
    controller.step(Duration::from_millis(50));
    let pos_mid = *target.borrow();
    assert!(pos_mid > 100.0);

    // User grabs / touches down again mid-flight
    controller.drag_start(pos_mid, Duration::from_millis(100));
    assert_eq!(controller.phase(), GesturePhase::Dragging);
    assert!(controller.is_dragging());
    assert_eq!(*target.borrow(), pos_mid);

    // Move from captured position
    controller.drag_move(pos_mid + 20.0, Duration::from_millis(120));
    assert_eq!(*target.borrow(), pos_mid + 20.0);
}
