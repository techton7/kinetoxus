//! Integration tests for kinetoxus Phase-2 to/from motion verbs.

use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;

#[test]
fn test_motion_to_current_value_sampling() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 10.0f32);
        let motion = use_motion();

        // Animate from current (10.0) to 50.0 over 100ms
        let handle = motion
            .to(signal, 50.0f32, Duration::from_millis(100))
            .ease(Ease::Linear);

        // Before first tick, signal remains at initial state
        assert_eq!(signal(), 10.0);
        assert_eq!(motion.active_count(), 1);
        assert!(handle.is_active());

        // Step 50ms (50% of delta 40.0 = +20.0 -> 30.0)
        let has_more = motion.tick(Duration::from_millis(50));
        assert!(has_more);
        assert!((signal() - 30.0).abs() < 1e-4);

        // Step 50ms (100% -> 50.0)
        let has_more = motion.tick(Duration::from_millis(50));
        assert!(!has_more);
        assert!((signal() - 50.0).abs() < 1e-4);
        assert_eq!(motion.active_count(), 0);
        assert!(!handle.is_active());

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_motion_to_interrupt_running_animation() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f32);
        let motion = use_motion();

        // First tween: 0.0 -> 100.0 over 100ms
        motion
            .to(signal, 100.0f32, Duration::from_millis(100))
            .ease(Ease::Linear);

        // Advance 40ms -> signal reaches 40.0
        motion.tick(Duration::from_millis(40));
        assert!((signal() - 40.0).abs() < 1e-4);

        // Interrupt with new tween: redirect to 0.0 over 100ms
        // Should sample current 40.0 as start, animating down to 0.0
        let handle2 = motion
            .to(signal, 0.0f32, Duration::from_millis(100))
            .ease(Ease::Linear);

        assert_eq!(motion.active_count(), 1);
        assert!(handle2.is_active());

        // Advance 50ms (50% of 40.0 -> 0.0 = 20.0)
        let has_more = motion.tick(Duration::from_millis(50));
        assert!(has_more);
        assert!((signal() - 20.0).abs() < 1e-4);

        // Advance 50ms (completion at 0.0)
        let has_more = motion.tick(Duration::from_millis(50));
        assert!(!has_more);
        assert!((signal() - 0.0).abs() < 1e-4);
        assert_eq!(motion.active_count(), 0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_motion_from_explicit_start_and_destination_sampling() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 80.0f32);
        let motion = use_motion();

        // Animate from 0.0 to current value (80.0) over 100ms
        let handle = motion
            .from(signal, 0.0f32, Duration::from_millis(100))
            .ease(Ease::Linear);

        // Immediately renders start value (0.0) to prevent flicker
        assert_eq!(signal(), 0.0);
        assert_eq!(motion.active_count(), 1);
        assert!(handle.is_active());

        // Step 50ms (50% of 0.0 to 80.0 = 40.0)
        let has_more = motion.tick(Duration::from_millis(50));
        assert!(has_more);
        assert!((signal() - 40.0).abs() < 1e-4);

        // Step 50ms (100% -> 80.0)
        let has_more = motion.tick(Duration::from_millis(50));
        assert!(!has_more);
        assert!((signal() - 80.0).abs() < 1e-4);
        assert_eq!(motion.active_count(), 0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_motion_to_and_from_overwrite_by_set() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 10.0f32);
        let motion = use_motion();

        motion.to(signal, 100.0f32, Duration::from_millis(100));
        assert_eq!(motion.active_count(), 1);

        motion.tick(Duration::from_millis(20));

        // Immediate set cancels the running `to` tween
        motion.set(signal, 999.0);
        assert_eq!(signal(), 999.0);
        assert_eq!(motion.active_count(), 0);

        // Subsequent ticks do nothing
        let has_more = motion.tick(Duration::from_millis(50));
        assert!(!has_more);
        assert_eq!(signal(), 999.0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_motion_to_array_target() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| [0.0f32, 10.0f32]);
        let motion = use_motion();

        motion
            .to(signal, [100.0f32, 50.0f32], Duration::from_millis(100))
            .ease(Ease::Linear);

        // 50ms -> 50%
        motion.tick(Duration::from_millis(50));
        assert!((signal()[0] - 50.0).abs() < 1e-4);
        assert!((signal()[1] - 30.0).abs() < 1e-4);

        // 100ms -> 100%
        motion.tick(Duration::from_millis(50));
        assert!((signal()[0] - 100.0).abs() < 1e-4);
        assert!((signal()[1] - 50.0).abs() < 1e-4);
        assert_eq!(motion.active_count(), 0);

        rsx! {}
    });
    dom.rebuild_in_place();
}
