//! Integration test suite for kinetoxus SignalTarget timeline binding & reactive proof (Phase 8).

use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;

#[test]
fn test_signal_timeline_immediate_initial_value() {
    let mut dom = VirtualDom::new(|| {
        let x = use_signal(|| 999.0f64);
        let y = use_signal(|| -50.0f64);

        let tl = use_timeline(|b| {
            b.track("x", x, 0.0f64, |t| {
                t.tween(0.0, 100.0, Duration::from_millis(500), Ease::Linear, Position::End);
            });
            b.track("y", y, 10.0f64, |t| {
                t.tween(10.0, 50.0, Duration::from_millis(500), Ease::Linear, Position::End);
            });
        });

        // Immediately evaluated and applied t = 0 on creation so targets take the initial values
        assert_eq!(x(), 0.0);
        assert_eq!(y(), 10.0);
        assert!(tl.is_paused());
        assert_eq!(tl.time(), Duration::ZERO);
        assert_eq!(tl.progress(), 0.0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_signal_timeline_play_progression_and_settle() {
    let mut dom = VirtualDom::new(|| {
        let x = use_signal(|| 0.0f64);

        let tl = use_timeline(|b| {
            b.bind_signal("x", x);
            b.tween("x", 0.0, 100.0, Duration::from_millis(200), Ease::Linear, Position::End);
        });

        assert_eq!(x(), 0.0);
        assert!(tl.is_paused());

        // Start playback
        tl.play();
        assert!(tl.is_playing());
        assert!(!tl.is_paused());

        // Step 50ms (25%)
        let has_more = tl.step(Duration::from_millis(50));
        assert!(has_more);
        assert!((x() - 25.0).abs() < 1e-4);
        assert_eq!(tl.time(), Duration::from_millis(50));
        assert!((tl.progress() - 0.25).abs() < 1e-4);

        // Step 50ms (50%)
        let has_more = tl.step(Duration::from_millis(50));
        assert!(has_more);
        assert!((x() - 50.0).abs() < 1e-4);
        assert_eq!(tl.time(), Duration::from_millis(100));
        assert!((tl.progress() - 0.50).abs() < 1e-4);

        // Step 100ms (100% - complete)
        let has_more = tl.step(Duration::from_millis(100));
        assert!(!has_more);
        assert!((x() - 100.0).abs() < 1e-4);
        assert!(tl.is_completed());
        assert!(!tl.is_playing());

        // Further steps do not mutate past terminal value
        let has_more = tl.step(Duration::from_millis(50));
        assert!(!has_more);
        assert_eq!(x(), 100.0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_signal_timeline_seek_and_set_progress() {
    let mut dom = VirtualDom::new(|| {
        let x = use_signal(|| 0.0f64);
        let opacity = use_signal(|| 0.0f32);

        let tl = use_timeline(|b| {
            b.track("x", x, 0.0f64, |t| {
                t.tween(0.0, 200.0, Duration::from_millis(1000), Ease::Linear, Position::End);
            });
            b.track("opacity", opacity, 0.0f32, |t| {
                t.tween(0.0, 1.0, Duration::from_millis(1000), Ease::Linear, Position::End);
            });
        });

        // Seek directly to 400ms
        tl.seek(Duration::from_millis(400));
        assert_eq!(tl.time(), Duration::from_millis(400));
        assert!((x() - 80.0).abs() < 1e-4);
        assert!((opacity() - 0.40).abs() < 1e-4);

        // Scrub via set_progress to 85%
        tl.set_progress(0.85);
        assert!((tl.progress() - 0.85).abs() < 1e-4);
        assert!((x() - 170.0).abs() < 1e-4);
        assert!((opacity() - 0.85).abs() < 1e-4);

        // Scrub back to 10%
        tl.set_progress(0.10);
        assert!((tl.progress() - 0.10).abs() < 1e-4);
        assert!((x() - 20.0).abs() < 1e-4);
        assert!((opacity() - 0.10).abs() < 1e-4);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_signal_timeline_reverse_and_restart() {
    let mut dom = VirtualDom::new(|| {
        let x = use_signal(|| 0.0f64);

        let tl = use_timeline(|b| {
            b.bind_signal("x", x);
            b.tween("x", 0.0, 100.0, Duration::from_millis(200), Ease::Linear, Position::End);
        });

        tl.play();
        // Step to 100ms (50%)
        tl.step(Duration::from_millis(100));
        assert!((x() - 50.0).abs() < 1e-4);

        // Reverse playback
        tl.reverse();
        assert!(tl.is_playing());

        // Step backwards 50ms
        let has_more = tl.step(Duration::from_millis(50));
        assert!(has_more);
        assert!((x() - 25.0).abs() < 1e-4);

        // Step backwards 50ms (reaches 0 - complete)
        let has_more = tl.step(Duration::from_millis(50));
        assert!(!has_more);
        assert!((x() - 0.0).abs() < 1e-4);
        assert!(tl.is_completed());

        // Restart playback from 0 in forward direction
        tl.restart();
        assert!(tl.is_playing());
        assert!(!tl.is_completed());
        assert_eq!(tl.time(), Duration::ZERO);
        assert_eq!(x(), 0.0);

        // Step forward 100ms
        tl.step(Duration::from_millis(100));
        assert!((x() - 50.0).abs() < 1e-4);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_signal_timeline_label_seeking() {
    let mut dom = VirtualDom::new(|| {
        let x = use_signal(|| 0.0f64);

        let tl = use_timeline(|b| {
            b.bind_signal("x", x);
            b.tween("x", 0.0, 50.0, Duration::from_millis(200), Ease::Linear, Position::End);
            b.label("midpoint", Position::RecentEnd);
            b.tween("x", 50.0, 100.0, Duration::from_millis(200), Ease::Linear, Position::RecentEnd);
        });

        assert_eq!(x(), 0.0);

        // Seek to named label "midpoint" (200ms)
        let found = tl.seek_label("midpoint");
        assert!(found);
        assert_eq!(tl.time(), Duration::from_millis(200));
        assert!((x() - 50.0).abs() < 1e-4);

        // Seek to non-existent label
        let not_found = tl.seek_label("invalid");
        assert!(!not_found);
        // Time remains at midpoint
        assert_eq!(tl.time(), Duration::from_millis(200));

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_signal_timeline_toggle_and_pause() {
    let mut dom = VirtualDom::new(|| {
        let x = use_signal(|| 0.0f64);

        let tl = use_timeline(|b| {
            b.bind_signal("x", x);
            b.tween("x", 0.0, 100.0, Duration::from_millis(200), Ease::Linear, Position::End);
        });

        assert!(tl.is_paused());

        // Toggle from paused to playing
        tl.toggle();
        assert!(tl.is_playing());

        tl.step(Duration::from_millis(50));
        assert!((x() - 25.0).abs() < 1e-4);

        // Toggle from playing to paused
        tl.toggle();
        assert!(tl.is_paused());
        assert!(!tl.is_playing());

        // Step while paused does not advance
        tl.step(Duration::from_millis(50));
        assert!((x() - 25.0).abs() < 1e-4);

        // Pause directly
        tl.play();
        assert!(tl.is_playing());
        tl.pause();
        assert!(tl.is_paused());

        rsx! {}
    });
    dom.rebuild_in_place();
}
