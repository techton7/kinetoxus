//! Integration test suite for HandleTarget spring binding and zero-signal proof (Subtask T-4.4).

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use kinetoxus::prelude::*;
use kinetocore::spring::SpringConfig;

#[test]
fn test_handle_target_f64_spring_direct_mutation() {
    let cell = Rc::new(RefCell::new(0.0f64));
    let motion = Motion::new();

    // Animate HandleTarget<f64> via spring directly to Rc<RefCell<f64>>
    let config = SpringConfig::new(1.0, 100.0, 10.0).unwrap(); // underdamped
    let handle = motion.spring(cell.clone(), 100.0f64, config);

    assert_eq!(*cell.borrow(), 0.0);
    assert_eq!(motion.active_count(), 1);
    assert!(handle.is_active());

    // Tick simulation until settlement
    let mut max_val = 0.0f64;
    let mut settled = false;
    for _ in 0..300 {
        let has_more = motion.tick(Duration::from_millis(16));
        let val = *cell.borrow();
        if val > max_val {
            max_val = val;
        }
        if !has_more {
            settled = true;
            break;
        }
    }

    assert!(settled, "Spring on HandleTarget<f64> must settle");
    assert!(max_val > 100.0, "Underdamped spring must overshoot target 100.0, got max {max_val}");
    assert!((*cell.borrow() - 100.0).abs() < 1e-6, "Must settle at exact target 100.0, got {}", *cell.borrow());
    assert_eq!(motion.active_count(), 0);

    // Runtime dormancy check: subsequent ticks do not mutate
    motion.tick(Duration::from_millis(16));
    assert_eq!(*cell.borrow(), 100.0);
    assert_eq!(motion.active_count(), 0);
}

#[test]
fn test_handle_target_zero_signal_isolation_proof() {
    // Pure headless Rust thread test with zero Dioxus VDOM scope or signal proxying
    let x = Rc::new(RefCell::new(0.0f64));
    let y = Rc::new(RefCell::new(0.0f64));
    let scale = Rc::new(RefCell::new(1.0f64));
    let motion = Motion::new();

    let config = SpringConfig::new(1.0, 100.0, 20.0).unwrap(); // critically damped

    // Simultaneous multi-handle spring animation
    motion.spring(Rc::clone(&x), 100.0f64, config);
    motion.spring(Rc::clone(&y), 200.0f64, config);
    motion.spring(Rc::clone(&scale), 2.0f64, config);

    assert_eq!(motion.active_count(), 3);

    let mut settled = false;
    for _ in 0..300 {
        let has_more = motion.tick(Duration::from_millis(16));
        if !has_more {
            settled = true;
            break;
        }
    }

    assert!(settled);
    assert!((*x.borrow() - 100.0).abs() < 1e-5);
    assert!((*y.borrow() - 200.0).abs() < 1e-5);
    assert!((*scale.borrow() - 2.0).abs() < 1e-5);
    assert_eq!(motion.active_count(), 0);
}

#[test]
fn test_handle_target_spring_damping_regimes() {
    // 1. Critically damped
    let cell_crit = Rc::new(RefCell::new(0.0f64));
    let motion_crit = Motion::new();
    let config_crit = SpringConfig::new(1.0, 100.0, 20.0).unwrap();
    motion_crit.spring(cell_crit.clone(), 100.0f64, config_crit);

    let mut max_crit = 0.0f64;
    for _ in 0..200 {
        motion_crit.tick(Duration::from_millis(16));
        let v = *cell_crit.borrow();
        if v > max_crit { max_crit = v; }
    }
    assert!(max_crit <= 100.0 + 1e-4, "Critically damped should not overshoot, got {max_crit}");
    assert!((*cell_crit.borrow() - 100.0).abs() < 1e-6);

    // 2. Overdamped
    let cell_over = Rc::new(RefCell::new(0.0f64));
    let motion_over = Motion::new();
    let config_over = SpringConfig::new(1.0, 100.0, 50.0).unwrap();
    motion_over.spring(cell_over.clone(), 100.0f64, config_over);

    let mut max_over = 0.0f64;
    for _ in 0..500 {
        let has_more = motion_over.tick(Duration::from_millis(16));
        let v = *cell_over.borrow();
        if v > max_over { max_over = v; }
        if !has_more { break; }
    }
    assert!(max_over <= 100.0 + 1e-4, "Overdamped should not overshoot, got {max_over}");
    assert!((*cell_over.borrow() - 100.0).abs() < 1e-6);
}

#[test]
fn test_handle_target_mid_flight_spring_retargeting() {
    let cell = Rc::new(RefCell::new(0.0f64));
    let motion = Motion::new();
    let config = SpringConfig::new(1.0, 100.0, 20.0).unwrap();

    let handle = motion.spring(cell.clone(), 100.0f64, config);

    // Tick partially
    for _ in 0..5 {
        motion.tick(Duration::from_millis(16));
    }

    let pos_before = *cell.borrow();
    assert!(pos_before > 0.0 && pos_before < 100.0);

    // Retarget mid-flight
    handle.set_target(250.0f64);
    assert!(handle.is_active());

    // Tick and verify smooth progression
    motion.tick(Duration::from_millis(16));
    let pos_after = *cell.borrow();
    assert!(pos_after > pos_before, "Should continue forward smoothly without pop");

    // Settle at 250.0
    let mut settled = false;
    for _ in 0..300 {
        let has_more = motion.tick(Duration::from_millis(16));
        if !has_more {
            settled = true;
            break;
        }
    }

    assert!(settled);
    assert!((*cell.borrow() - 250.0).abs() < 1e-6);
    assert_eq!(motion.active_count(), 0);
}
