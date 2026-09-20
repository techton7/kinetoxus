//! Integration test suite for kinetoxus Phase-1 Signal MVP.

use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;

#[test]
fn test_motion_set_immediate() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f32);
        let motion = use_motion();

        motion.set(signal, 42.0);
        assert_eq!(signal(), 42.0);
        assert_eq!(motion.active_count(), 0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_motion_from_to_progression() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f32);
        let motion = use_motion();

        // 0 to 100 over 100ms with linear easing
        let handle = motion
            .from_to(signal, 0.0f32, 100.0f32, Duration::from_millis(100))
            .ease(Ease::Linear);

        // Immediate render of start value
        assert_eq!(signal(), 0.0);
        assert_eq!(motion.active_count(), 1);
        assert!(handle.is_active());

        // Step 25ms (25%)
        let has_more = motion.tick(Duration::from_millis(25));
        assert!(has_more);
        assert!((signal() - 25.0).abs() < 1e-4);

        // Step 25ms (50%)
        let has_more = motion.tick(Duration::from_millis(25));
        assert!(has_more);
        assert!((signal() - 50.0).abs() < 1e-4);

        // Step 50ms (100% - completion)
        let has_more = motion.tick(Duration::from_millis(50));
        assert!(!has_more);
        assert!((signal() - 100.0).abs() < 1e-4);
        assert_eq!(motion.active_count(), 0);
        assert!(!handle.is_active());

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_motion_overwrite_semantics() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f32);
        let motion = use_motion();

        // Start Animation 1: 0 -> 100 over 100ms
        let h1 = motion
            .from_to(signal, 0.0f32, 100.0f32, Duration::from_millis(100))
            .ease(Ease::Linear);

        motion.tick(Duration::from_millis(30));
        assert!((signal() - 30.0).abs() < 1e-4);
        assert!(h1.is_active());

        // Start Animation 2 on the SAME signal: 50 -> 0 over 50ms
        // This should overwrite (cancel) Animation 1 immediately
        let h2 = motion
            .from_to(signal, 50.0f32, 0.0f32, Duration::from_millis(50))
            .ease(Ease::Linear);

        assert!(!h1.is_active(), "Animation 1 must be overwritten");
        assert!(h2.is_active());
        assert_eq!(motion.active_count(), 1);
        assert_eq!(signal(), 50.0);

        // Step 50ms to finish Animation 2
        motion.tick(Duration::from_millis(50));
        assert!((signal() - 0.0).abs() < 1e-4);
        assert!(!h2.is_active());
        assert_eq!(motion.active_count(), 0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_motion_set_overwrites_active_animation() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f32);
        let motion = use_motion();

        let h = motion.from_to(signal, 0.0f32, 100.0f32, Duration::from_millis(100));
        assert_eq!(motion.active_count(), 1);

        // motion.set should immediately cancel the running animation
        motion.set(signal, 999.0);
        assert_eq!(signal(), 999.0);
        assert_eq!(motion.active_count(), 0);
        assert!(!h.is_active());

        // Further ticks should not mutate signal
        motion.tick(Duration::from_millis(50));
        assert_eq!(signal(), 999.0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_motion_handle_manual_cancel() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f32);
        let motion = use_motion();

        let handle = motion
            .from_to(signal, 0.0f32, 100.0f32, Duration::from_millis(100))
            .ease(Ease::Linear);

        motion.tick(Duration::from_millis(40));
        assert!((signal() - 40.0).abs() < 1e-4);

        handle.cancel();
        assert!(!handle.is_active());
        assert_eq!(motion.active_count(), 0);

        // Further ticks do not mutate signal
        motion.tick(Duration::from_millis(60));
        assert!((signal() - 40.0).abs() < 1e-4);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_motion_array_target() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| [0.0f32, 0.0f32]);
        let motion = use_motion();

        motion
            .from_to(
                signal,
                [0.0f32, 10.0f32],
                [100.0f32, 50.0f32],
                Duration::from_millis(100),
            )
            .ease(Ease::Linear);

        // Initial
        assert_eq!(signal(), [0.0, 10.0]);

        // Halfway
        motion.tick(Duration::from_millis(50));
        let mid = signal();
        assert!((mid[0] - 50.0).abs() < 1e-4);
        assert!((mid[1] - 30.0).abs() < 1e-4);

        // Complete
        motion.tick(Duration::from_millis(50));
        assert_eq!(signal(), [100.0, 50.0]);
        assert_eq!(motion.active_count(), 0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_motion_repeat_and_yoyo() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f32);
        let motion = use_motion();

        // 2 cycles with yoyo (mirrored repeat)
        motion
            .from_to(signal, 0.0f32, 100.0f32, Duration::from_millis(100))
            .ease(Ease::Linear)
            .repeat(RepeatCount::Finite(2), RepeatStrategy::MirroredRepeat);

        // Cycle 1: 0ms -> 0.0, 50ms -> 50.0, 100ms -> 100.0
        motion.tick(Duration::from_millis(50));
        assert!((signal() - 50.0).abs() < 1e-4);

        motion.tick(Duration::from_millis(50));
        assert!((signal() - 100.0).abs() < 1e-4);

        // Cycle 2 (mirrored): 150ms -> 50.0, 200ms -> 0.0
        motion.tick(Duration::from_millis(50));
        assert!((signal() - 50.0).abs() < 1e-4);

        let has_more = motion.tick(Duration::from_millis(50));
        assert!(!has_more);
        assert!((signal() - 0.0).abs() < 1e-4);
        assert_eq!(motion.active_count(), 0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_driver_kind_reporting() {
    let motion = Motion::new();
    #[cfg(target_arch = "wasm32")]
    assert_eq!(motion.driver_kind(), DriverKind::WebRaf);
    #[cfg(not(target_arch = "wasm32"))]
    assert_eq!(motion.driver_kind(), DriverKind::Manual);
}

#[test]
fn test_precomputed_tween_animate() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f64);
        let motion = use_motion();

        let tween = Tween::from_to(0.0f64, 200.0f64, Duration::from_millis(80))
            .ease(Ease::QuadOut);

        let handle = motion.animate(signal, tween);
        assert!(handle.is_active());

        motion.tick(Duration::from_millis(80));
        assert!((signal() - 200.0).abs() < 1e-4);
        assert_eq!(motion.active_count(), 0);

        rsx! {}
    });
    dom.rebuild_in_place();
}
