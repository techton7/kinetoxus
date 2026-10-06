//! Empirical proof test suite for kinetoxus HandleTarget (Subtask T-2.4).
//! Verifies zero-signal direct interior-mutable memory updates and concurrent pose animation.

use std::time::Duration;
use kinetoxus::prelude::*;

#[test]
fn test_handle_direct_memory_progression() {
    let transform = Transform2D::default();
    let motion = Motion::new();

    // Animate transform.x from 0.0 to 100.0 over 100ms
    let handle = motion
        .from_to(std::rc::Rc::clone(&transform.x), 0.0f32, 100.0f32, Duration::from_millis(100))
        .ease(Ease::Linear);

    assert_eq!(*transform.x.borrow(), 0.0);
    assert_eq!(motion.active_count(), 1);
    assert!(handle.is_active());

    // Advance 25ms -> should be 25.0
    let has_more = motion.tick(Duration::from_millis(25));
    assert!(has_more);
    assert!((*transform.x.borrow() - 25.0).abs() < 1e-4);

    // Advance 25ms -> should be 50.0
    let has_more = motion.tick(Duration::from_millis(25));
    assert!(has_more);
    assert!((*transform.x.borrow() - 50.0).abs() < 1e-4);

    // Advance 25ms -> should be 75.0
    let has_more = motion.tick(Duration::from_millis(25));
    assert!(has_more);
    assert!((*transform.x.borrow() - 75.0).abs() < 1e-4);

    // Advance 25ms -> completion (100.0)
    let has_more = motion.tick(Duration::from_millis(25));
    assert!(!has_more);
    assert!((*transform.x.borrow() - 100.0).abs() < 1e-4);
    assert_eq!(motion.active_count(), 0);
}

#[test]
fn test_handle_multi_field_pose_animation() {
    let transform = Transform2D::new(0.0, 0.0, 1.0, 0.0);
    let motion = Motion::new();

    // Simultaneously animate x, y, scale, rotation across 4 independent handles
    motion.from_to(std::rc::Rc::clone(&transform.x), 0.0f32, 200.0f32, Duration::from_millis(100)).ease(Ease::Linear);
    motion.from_to(std::rc::Rc::clone(&transform.y), 0.0f32, -100.0f32, Duration::from_millis(100)).ease(Ease::Linear);
    motion.from_to(std::rc::Rc::clone(&transform.scale), 1.0f32, 2.5f32, Duration::from_millis(100)).ease(Ease::Linear);
    motion.from_to(std::rc::Rc::clone(&transform.rotation), 0.0f32, 360.0f32, Duration::from_millis(100)).ease(Ease::Linear);

    assert_eq!(motion.active_count(), 4);

    // Tick 50ms (halfway)
    motion.tick(Duration::from_millis(50));

    assert!((*transform.x.borrow() - 100.0).abs() < 1e-4);
    assert!((*transform.y.borrow() - (-50.0)).abs() < 1e-4);
    assert!((*transform.scale.borrow() - 1.75).abs() < 1e-4);
    assert!((*transform.rotation.borrow() - 180.0).abs() < 1e-4);

    // Tick remaining 50ms (settled)
    motion.tick(Duration::from_millis(50));

    assert!((*transform.x.borrow() - 200.0).abs() < 1e-4);
    assert!((*transform.y.borrow() - (-100.0)).abs() < 1e-4);
    assert!((*transform.scale.borrow() - 2.5).abs() < 1e-4);
    assert!((*transform.rotation.borrow() - 360.0).abs() < 1e-4);
    assert_eq!(motion.active_count(), 0);
}

#[test]
fn test_handle_zero_signal_guarantee() {
    // Run 50 frames of animation completely outside any Dioxus VirtualDom scope or component lifecycle,
    // proving the animation engine operates natively on raw handles with zero signal proxying.
    let transform = Transform2D::new(10.0, 20.0, 0.5, 45.0);
    let motion = Motion::new();

    motion.from_to(std::rc::Rc::clone(&transform.x), 10.0f32, 110.0f32, Duration::from_millis(500)).ease(Ease::Linear);
    motion.from_to(std::rc::Rc::clone(&transform.y), 20.0f32, 220.0f32, Duration::from_millis(500)).ease(Ease::Linear);

    for frame in 1..=50 {
        motion.tick(Duration::from_millis(10));
        let t = (frame as f32 * 10.0) / 500.0;
        let expected_x_val = 10.0 + t * 100.0;
        let expected_y_val = 20.0 + t * 200.0;
        assert!((*transform.x.borrow() - expected_x_val).abs() < 1e-4);
        assert!((*transform.y.borrow() - expected_y_val).abs() < 1e-4);
    }

    assert_eq!(motion.active_count(), 0);
    assert!((*transform.x.borrow() - 110.0).abs() < 1e-4);
    assert!((*transform.y.borrow() - 220.0).abs() < 1e-4);
}
