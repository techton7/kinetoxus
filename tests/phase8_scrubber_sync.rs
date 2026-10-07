//! Integration test suite for kinetoxus real-time scrubber synchronization (Phase 8).

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;

#[test]
fn test_scrubber_sync_zero_tick_instant_update() {
    let mut dom = VirtualDom::new(|| {
        let signal_x = use_signal(|| 0.0f64);
        let handle_y = Rc::new(RefCell::new(0.0f64));

        let tl = use_timeline(|b| {
            b.bind_signal("x", signal_x);
            b.bind_handle("y", &handle_y);

            b.tween("x", 0.0f64, 100.0f64, Duration::from_millis(1000), Ease::Linear, Position::End);
            b.tween("y", 0.0f64, 500.0f64, Duration::from_millis(1000), Ease::Linear, Position::RecentStart);
        });

        // Initial t = 0 applied synchronously in 0 ticks
        assert_eq!(signal_x(), 0.0);
        assert_eq!(*handle_y.borrow(), 0.0);

        // 1. Scrub to 25% - must update IMMEDIATELY in 0 ticks
        tl.set_progress(0.25);
        assert_eq!(signal_x(), 25.0);
        assert_eq!(*handle_y.borrow(), 125.0);
        assert_eq!(tl.time(), Duration::from_millis(250));

        // 2. Scrub to 50% - must update IMMEDIATELY in 0 ticks
        tl.set_progress(0.50);
        assert_eq!(signal_x(), 50.0);
        assert_eq!(*handle_y.borrow(), 250.0);
        assert_eq!(tl.time(), Duration::from_millis(500));

        // 3. Scrub to 75% - must update IMMEDIATELY in 0 ticks
        tl.set_progress(0.75);
        assert_eq!(signal_x(), 75.0);
        assert_eq!(*handle_y.borrow(), 375.0);
        assert_eq!(tl.time(), Duration::from_millis(750));

        // 4. Scrub to 100% - must update IMMEDIATELY in 0 ticks
        tl.set_progress(1.00);
        assert_eq!(signal_x(), 100.0);
        assert_eq!(*handle_y.borrow(), 500.0);
        assert_eq!(tl.time(), Duration::from_millis(1000));

        // 5. Scrub back to 0% - must update IMMEDIATELY in 0 ticks
        tl.set_progress(0.00);
        assert_eq!(signal_x(), 0.0);
        assert_eq!(*handle_y.borrow(), 0.0);
        assert_eq!(tl.time(), Duration::ZERO);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_scrubber_rapid_bidirectional_sweep_without_frame_drift() {
    let mut dom = VirtualDom::new(|| {
        let pos = use_signal(|| 0.0f64);

        let tl = use_timeline(|b| {
            b.track("pos", pos, 0.0f64, |t| {
                t.tween(1000.0, Duration::from_millis(2000), Ease::Linear);
            });
        });

        // Simulate a rapid mouse drag or touch scrub gesture across 10 distinct sample points
        let scrub_points = [
            0.12, 0.45, 0.78, 0.95, 0.82, 0.60, 0.33, 0.05, 0.99, 0.50,
        ];

        for &p in &scrub_points {
            tl.set_progress(p);
            let expected = 1000.0 * (p as f64);
            assert!(
                (pos() - expected).abs() < 1e-3,
                "At progress {p}, expected {expected} but got {}",
                pos()
            );
            assert!(
                (tl.progress() - p).abs() < 1e-4,
                "Reported progress must match scrubbed progress"
            );
        }

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_scrubber_seek_time_and_label_synchronous_sync() {
    let mut dom = VirtualDom::new(|| {
        let opacity = use_signal(|| 0.0f32);
        let scale = use_signal(|| 1.0f32);

        let tl = use_timeline(|b| {
            b.bind_signal("opacity", opacity);
            b.bind_signal("scale", scale);

            b.tween("opacity", 0.0f32, 1.0f32, Duration::from_millis(400), Ease::Linear, Position::End);
            b.label("intro_done", Position::RecentEnd);
            b.tween("scale", 1.0f32, 2.0f32, Duration::from_millis(600), Ease::Linear, Position::RecentEnd);
            b.label("outro_done", Position::RecentEnd);
        });

        assert_eq!(tl.duration(), Duration::from_millis(1000));

        // Direct timestamp seek to 200ms (50% opacity, scale remains 1.0)
        tl.seek(Duration::from_millis(200));
        assert!((opacity() - 0.5).abs() < 1e-4);
        assert!((scale() - 1.0).abs() < 1e-4);

        // Seek to label "intro_done" (400ms)
        let found = tl.seek_label("intro_done");
        assert!(found);
        assert_eq!(tl.time(), Duration::from_millis(400));
        assert!((opacity() - 1.0).abs() < 1e-4);
        assert!((scale() - 1.0).abs() < 1e-4);

        // Seek to label "outro_done" (1000ms)
        let found = tl.seek_label("outro_done");
        assert!(found);
        assert_eq!(tl.time(), Duration::from_millis(1000));
        assert!((opacity() - 1.0).abs() < 1e-4);
        assert!((scale() - 2.0).abs() < 1e-4);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_scrubber_clamping_boundary_protection() {
    let mut dom = VirtualDom::new(|| {
        let x = use_signal(|| 0.0f64);

        let tl = use_timeline(|b| {
            b.bind_signal("x", x);
            b.tween("x", 0.0, 100.0, Duration::from_millis(500), Ease::Linear, Position::End);
        });

        // Out-of-bounds negative progress clamps to 0.0
        tl.set_progress(-0.5);
        assert_eq!(tl.progress(), 0.0);
        assert_eq!(x(), 0.0);

        // Out-of-bounds excessive progress clamps to 1.0
        tl.set_progress(1.5);
        assert_eq!(tl.progress(), 1.0);
        assert_eq!(x(), 100.0);

        rsx! {}
    });
    dom.rebuild_in_place();
}
