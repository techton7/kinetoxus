//! Integration test suite for kinetoxus HandleTarget (Phase 3).

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use kinetoxus::prelude::*;

#[test]
fn test_handle_target_headless_basic_and_set() {
    let handle_cell = Rc::new(RefCell::new(0.0f32));
    let motion = Motion::new();

    // Verify immediate set
    motion.set(handle_cell.clone(), 42.0);
    assert_eq!(*handle_cell.borrow(), 42.0);
    assert_eq!(motion.active_count(), 0);

    // Verify HandleTarget direct conversion / usage
    let target = HandleTarget::from(handle_cell.clone());
    motion.set(&target, 100.0);
    assert_eq!(target.get(), 100.0);
}

#[test]
fn test_handle_target_from_to_progression() {
    let handle_cell = Rc::new(RefCell::new(0.0f32));
    let motion = Motion::new();

    let handle = motion
        .from_to(handle_cell.clone(), 0.0f32, 100.0f32, Duration::from_millis(100))
        .ease(Ease::Linear);

    assert_eq!(*handle_cell.borrow(), 0.0);
    assert_eq!(motion.active_count(), 1);
    assert!(handle.is_active());

    // Tick 25ms (25%)
    let has_more = motion.tick(Duration::from_millis(25));
    assert!(has_more);
    assert!((*handle_cell.borrow() - 25.0).abs() < 1e-4);

    // Tick 50ms (75%)
    let has_more = motion.tick(Duration::from_millis(50));
    assert!(has_more);
    assert!((*handle_cell.borrow() - 75.0).abs() < 1e-4);

    // Tick 25ms (100% - completion)
    let has_more = motion.tick(Duration::from_millis(25));
    assert!(!has_more);
    assert!((*handle_cell.borrow() - 100.0).abs() < 1e-4);
    assert_eq!(motion.active_count(), 0);
    assert!(!handle.is_active());
}

#[test]
fn test_handle_target_to_dynamic_sampling() {
    let handle_cell = Rc::new(RefCell::new(20.0f32));
    let motion = Motion::new();

    // Animate from current value (20.0) to 120.0 over 100ms
    let handle = motion
        .to(handle_cell.clone(), 120.0f32, Duration::from_millis(100))
        .ease(Ease::Linear);

    assert_eq!(*handle_cell.borrow(), 20.0);
    assert!(handle.is_active());

    // Tick 50ms (should be at 70.0)
    motion.tick(Duration::from_millis(50));
    assert!((*handle_cell.borrow() - 70.0).abs() < 1e-4);

    // Finish
    motion.tick(Duration::from_millis(50));
    assert!((*handle_cell.borrow() - 120.0).abs() < 1e-4);
    assert_eq!(motion.active_count(), 0);
}

#[test]
fn test_handle_target_mid_flight_interruption_retargeting() {
    let handle_cell = Rc::new(RefCell::new(0.0f32));
    let motion = Motion::new();

    // Start 0 -> 100 over 100ms
    let h1 = motion
        .from_to(handle_cell.clone(), 0.0f32, 100.0f32, Duration::from_millis(100))
        .ease(Ease::Linear);

    motion.tick(Duration::from_millis(40));
    assert!((*handle_cell.borrow() - 40.0).abs() < 1e-4);
    assert!(h1.is_active());

    // Mid-flight retarget via motion.to(...) to 200 over 100ms
    // Dynamically samples current value (40.0) and animates 40 -> 200
    let h2 = motion
        .to(handle_cell.clone(), 200.0f32, Duration::from_millis(100))
        .ease(Ease::Linear);

    assert!(!h1.is_active(), "First animation should be cancelled");
    assert!(h2.is_active());
    assert_eq!(*handle_cell.borrow(), 40.0);

    // Tick 50ms of second animation (halfway between 40 and 200 = 120)
    motion.tick(Duration::from_millis(50));
    assert!((*handle_cell.borrow() - 120.0).abs() < 1e-4);

    // Finish second animation
    motion.tick(Duration::from_millis(50));
    assert!((*handle_cell.borrow() - 200.0).abs() < 1e-4);
    assert_eq!(motion.active_count(), 0);
}

#[test]
fn test_handle_target_from_verb() {
    let handle_cell = Rc::new(RefCell::new(999.0f32));
    let motion = Motion::new();

    // Animate from 0.0 to current value (999.0 captured at invocation) over 100ms
    let handle = motion
        .from(handle_cell.clone(), 0.0f32, Duration::from_millis(100))
        .ease(Ease::Linear);

    // Immediately set to 'from' (0.0)
    assert_eq!(*handle_cell.borrow(), 0.0);
    assert!(handle.is_active());

    motion.tick(Duration::from_millis(50));
    assert!((*handle_cell.borrow() - 499.5).abs() < 1e-4);

    motion.tick(Duration::from_millis(50));
    assert!((*handle_cell.borrow() - 999.0).abs() < 1e-4);
    assert_eq!(motion.active_count(), 0);
}

#[test]
fn test_handle_target_set_cancels_running_animation() {
    let handle_cell = Rc::new(RefCell::new(0.0f32));
    let motion = Motion::new();

    let h = motion.from_to(handle_cell.clone(), 0.0f32, 100.0f32, Duration::from_millis(100));
    assert_eq!(motion.active_count(), 1);

    // motion.set should immediately cancel running animation on HandleTarget
    motion.set(handle_cell.clone(), 555.0);
    assert_eq!(*handle_cell.borrow(), 555.0);
    assert_eq!(motion.active_count(), 0);
    assert!(!h.is_active());

    // Subsequent ticks do nothing
    motion.tick(Duration::from_millis(50));
    assert_eq!(*handle_cell.borrow(), 555.0);
}
