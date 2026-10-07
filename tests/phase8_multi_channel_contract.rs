//! Phase 8 Multi-Channel Visual Choreography Contract Test.
//!
//! Verifies exact state synchronization across 5 simultaneous visual channels:
//! `x` (f64), `y` (f64), `scale` (f32), `opacity` (f32), and `rotation` (f64)
//! under reactive signals and headless handles across `play()`, `pause()`,
//! `seek()`, `set_progress()`, and `reverse()`.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;

const EPSILON: f64 = 1e-4;

#[test]
fn test_multi_channel_reactive_signals_full_choreography() {
    let mut dom = VirtualDom::new(|| {
        let x = use_signal(|| 0.0f64);
        let y = use_signal(|| 0.0f64);
        let scale = use_signal(|| 1.0f32);
        let opacity = use_signal(|| 0.0f32);
        let rotation = use_signal(|| 0.0f64);

        let tl = use_timeline(|b| {
            b.bind_signal("x", x);
            b.bind_signal("y", y);
            b.bind_signal("scale", scale);
            b.bind_signal("opacity", opacity);
            b.bind_signal("rotation", rotation);

            // Channel 1: x translation (0.0 -> 200.0 over 1000ms, Linear)
            b.tween("x", 0.0f64, 200.0f64, Duration::from_millis(1000), Ease::Linear, Position::Absolute(Duration::ZERO));

            // Channel 2: y translation (0.0 -> 400.0 over 1000ms, QuadInOut)
            b.tween("y", 0.0f64, 400.0f64, Duration::from_millis(1000), Ease::QuadInOut, Position::Absolute(Duration::ZERO));

            // Channel 3: scale factor (1.0 -> 2.5 over 500ms, Linear; holds terminal 2.5 from 500ms to 1000ms)
            b.tween("scale", 1.0f32, 2.5f32, Duration::from_millis(500), Ease::Linear, Position::Absolute(Duration::ZERO));

            // Channel 4: opacity (0.0 -> 1.0 over 600ms, starts at 200ms; holds initial 0.0 pre-roll, holds terminal 1.0 post-roll)
            b.tween("opacity", 0.0f32, 1.0f32, Duration::from_millis(600), Ease::Linear, Position::Absolute(Duration::from_millis(200)));

            // Channel 5: rotation (0.0 -> 360.0 over 1000ms, Linear)
            b.tween("rotation", 0.0f64, 360.0f64, Duration::from_millis(1000), Ease::Linear, Position::Absolute(Duration::ZERO));
        });

        // 1. Initial state at t = 0 on mount
        assert_eq!(tl.time(), Duration::ZERO);
        assert_eq!(tl.progress(), 0.0);
        assert!(tl.is_paused());
        assert!(!tl.is_playing());
        assert!(!tl.is_completed());

        assert!((x() - 0.0).abs() < EPSILON);
        assert!((y() - 0.0).abs() < EPSILON);
        assert!((scale() - 1.0).abs() < 1e-4);
        assert!((opacity() - 0.0).abs() < 1e-4);
        assert!((rotation() - 0.0).abs() < EPSILON);

        // 2. Playback progression to 200ms
        tl.play();
        assert!(tl.is_playing());
        assert!(!tl.is_paused());

        let has_more = tl.step(Duration::from_millis(200));
        assert!(has_more);
        assert_eq!(tl.time(), Duration::from_millis(200));
        assert!((tl.progress() - 0.2).abs() < 1e-4);

        // At 200ms:
        // x: 200 * 0.2 = 40.0
        assert!((x() - 40.0).abs() < EPSILON);
        // y: QuadInOut(0.2) = 2 * 0.2^2 = 0.08 -> 400 * 0.08 = 32.0
        assert!((y() - 32.0).abs() < EPSILON);
        // scale: 1.0 + 1.5 * (200/500) = 1.0 + 0.6 = 1.6
        assert!((scale() - 1.6).abs() < 1e-4);
        // opacity: at 200ms, pre-roll ends, clip starts at 0.0
        assert!((opacity() - 0.0).abs() < 1e-4);
        // rotation: 360 * 0.2 = 72.0
        assert!((rotation() - 72.0).abs() < EPSILON);

        // 3. Pause behavior
        tl.pause();
        assert!(tl.is_paused());
        assert!(!tl.is_playing());

        // Step while paused must NOT advance any of the 5 channels
        tl.step(Duration::from_millis(100));
        assert_eq!(tl.time(), Duration::from_millis(200));
        assert!((x() - 40.0).abs() < EPSILON);
        assert!((y() - 32.0).abs() < EPSILON);
        assert!((scale() - 1.6).abs() < 1e-4);
        assert!((opacity() - 0.0).abs() < 1e-4);
        assert!((rotation() - 72.0).abs() < EPSILON);

        // 4. Resume playback and advance to 500ms
        tl.play();
        assert!(tl.is_playing());

        tl.step(Duration::from_millis(300));
        assert_eq!(tl.time(), Duration::from_millis(500));
        assert!((tl.progress() - 0.5).abs() < 1e-4);

        // At 500ms:
        // x: 200 * 0.5 = 100.0
        assert!((x() - 100.0).abs() < EPSILON);
        // y: QuadInOut(0.5) = 0.5 -> 400 * 0.5 = 200.0
        assert!((y() - 200.0).abs() < EPSILON);
        // scale: 500ms reaches clip end -> 2.5
        assert!((scale() - 2.5).abs() < 1e-4);
        // opacity: (500 - 200) / 600 = 300 / 600 = 0.5
        assert!((opacity() - 0.5).abs() < 1e-4);
        // rotation: 360 * 0.5 = 180.0
        assert!((rotation() - 180.0).abs() < EPSILON);

        // 5. Seek directly to 800ms (0-tick instant update)
        tl.seek(Duration::from_millis(800));
        assert_eq!(tl.time(), Duration::from_millis(800));
        assert!((tl.progress() - 0.8).abs() < 1e-4);

        // At 800ms:
        // x: 200 * 0.8 = 160.0
        assert!((x() - 160.0).abs() < EPSILON);
        // y: QuadInOut(0.8) = 1 - 2*(1-0.8)^2 = 1 - 2*0.04 = 0.92 -> 400 * 0.92 = 368.0
        assert!((y() - 368.0).abs() < EPSILON);
        // scale: post-roll holds terminal 2.5
        assert!((scale() - 2.5).abs() < 1e-4);
        // opacity: (800 - 200) / 600 = 600 / 600 = 1.0 (terminal)
        assert!((opacity() - 1.0).abs() < 1e-4);
        // rotation: 360 * 0.8 = 288.0
        assert!((rotation() - 288.0).abs() < EPSILON);

        // 6. Set progress to 1.0 (end)
        tl.set_progress(1.0);
        assert_eq!(tl.time(), Duration::from_millis(1000));
        assert!((tl.progress() - 1.0).abs() < 1e-4);
        assert!(tl.is_completed());

        assert!((x() - 200.0).abs() < EPSILON);
        assert!((y() - 400.0).abs() < EPSILON);
        assert!((scale() - 2.5).abs() < 1e-4);
        assert!((opacity() - 1.0).abs() < 1e-4);
        assert!((rotation() - 360.0).abs() < EPSILON);

        // 7. Set progress to 0.0 (rewind instantly)
        tl.set_progress(0.0);
        assert_eq!(tl.time(), Duration::ZERO);
        assert!((tl.progress() - 0.0).abs() < 1e-4);

        assert!((x() - 0.0).abs() < EPSILON);
        assert!((y() - 0.0).abs() < EPSILON);
        assert!((scale() - 1.0).abs() < 1e-4);
        assert!((opacity() - 0.0).abs() < 1e-4);
        assert!((rotation() - 0.0).abs() < EPSILON);

        // 8. Reverse playback from end
        tl.seek(Duration::from_millis(1000));
        tl.reverse();
        assert!(tl.is_playing());

        // Step backwards 200ms -> 800ms
        let has_more = tl.step(Duration::from_millis(200));
        assert!(has_more);
        assert_eq!(tl.time(), Duration::from_millis(800));
        assert!((x() - 160.0).abs() < EPSILON);
        assert!((y() - 368.0).abs() < EPSILON);
        assert!((scale() - 2.5).abs() < 1e-4);
        assert!((opacity() - 1.0).abs() < 1e-4);
        assert!((rotation() - 288.0).abs() < EPSILON);

        // Step backwards 300ms -> 500ms
        let has_more = tl.step(Duration::from_millis(300));
        assert!(has_more);
        assert_eq!(tl.time(), Duration::from_millis(500));
        assert!((x() - 100.0).abs() < EPSILON);
        assert!((y() - 200.0).abs() < EPSILON);
        assert!((scale() - 2.5).abs() < 1e-4);
        assert!((opacity() - 0.5).abs() < 1e-4);
        assert!((rotation() - 180.0).abs() < EPSILON);

        // Step backwards 300ms -> 200ms
        let has_more = tl.step(Duration::from_millis(300));
        assert!(has_more);
        assert_eq!(tl.time(), Duration::from_millis(200));
        assert!((x() - 40.0).abs() < EPSILON);
        assert!((y() - 32.0).abs() < EPSILON);
        assert!((scale() - 1.6).abs() < 1e-4);
        assert!((opacity() - 0.0).abs() < 1e-4);
        assert!((rotation() - 72.0).abs() < EPSILON);

        // Step backwards 200ms -> 0ms (completed in reverse)
        let has_more = tl.step(Duration::from_millis(200));
        assert!(!has_more);
        assert_eq!(tl.time(), Duration::ZERO);
        assert!(tl.is_completed());
        assert!((x() - 0.0).abs() < EPSILON);
        assert!((y() - 0.0).abs() < EPSILON);
        assert!((scale() - 1.0).abs() < 1e-4);
        assert!((opacity() - 0.0).abs() < 1e-4);
        assert!((rotation() - 0.0).abs() < EPSILON);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_multi_channel_headless_handles_full_choreography() {
    let x = Rc::new(RefCell::new(0.0f64));
    let y = Rc::new(RefCell::new(0.0f64));
    let scale = Rc::new(RefCell::new(1.0f32));
    let opacity = Rc::new(RefCell::new(0.0f32));
    let rotation = Rc::new(RefCell::new(0.0f64));

    let mut builder = TimelineHookBuilder::new();
    builder
        .bind_handle("x", &x)
        .bind_handle("y", &y)
        .bind_handle("scale", &scale)
        .bind_handle("opacity", &opacity)
        .bind_handle("rotation", &rotation);

    builder
        .tween("x", 0.0f64, 100.0f64, Duration::from_millis(500), Ease::Linear, Position::Absolute(Duration::ZERO))
        .tween("y", 0.0f64, 200.0f64, Duration::from_millis(500), Ease::Linear, Position::Absolute(Duration::ZERO))
        .tween("scale", 1.0f32, 2.0f32, Duration::from_millis(500), Ease::Linear, Position::Absolute(Duration::ZERO))
        .tween("opacity", 0.0f32, 1.0f32, Duration::from_millis(500), Ease::Linear, Position::Absolute(Duration::ZERO))
        .tween("rotation", 0.0f64, 180.0f64, Duration::from_millis(500), Ease::Linear, Position::Absolute(Duration::ZERO));

    let tl = builder.build().expect("timeline should compile cleanly");

    // Initial values applied immediately on build
    assert_eq!(*x.borrow(), 0.0);
    assert_eq!(*y.borrow(), 0.0);
    assert_eq!(*scale.borrow(), 1.0);
    assert_eq!(*opacity.borrow(), 0.0);
    assert_eq!(*rotation.borrow(), 0.0);

    // Play & step halfway (250ms)
    tl.play();
    tl.step(Duration::from_millis(250));
    assert_eq!(tl.time(), Duration::from_millis(250));
    assert!((*x.borrow() - 50.0).abs() < EPSILON);
    assert!((*y.borrow() - 100.0).abs() < EPSILON);
    assert!((*scale.borrow() - 1.5).abs() < 1e-4);
    assert!((*opacity.borrow() - 0.5).abs() < 1e-4);
    assert!((*rotation.borrow() - 90.0).abs() < EPSILON);

    // Pause verification
    tl.pause();
    assert!(tl.is_paused());
    tl.step(Duration::from_millis(100));
    assert_eq!(tl.time(), Duration::from_millis(250));
    assert!((*x.borrow() - 50.0).abs() < EPSILON);

    // Seek directly to 100ms
    tl.seek(Duration::from_millis(100));
    assert_eq!(tl.time(), Duration::from_millis(100));
    assert!((*x.borrow() - 20.0).abs() < EPSILON);
    assert!((*y.borrow() - 40.0).abs() < EPSILON);
    assert!((*scale.borrow() - 1.2).abs() < 1e-4);
    assert!((*opacity.borrow() - 0.2).abs() < 1e-4);
    assert!((*rotation.borrow() - 36.0).abs() < EPSILON);

    // Set progress to 60% (300ms)
    tl.set_progress(0.6);
    assert!((tl.time().as_secs_f64() - 0.300).abs() < 1e-4);
    assert!((tl.progress() - 0.6).abs() < 1e-4);
    assert!((*y.borrow() - 120.0).abs() < EPSILON);
    assert!((*scale.borrow() - 1.6).abs() < 1e-4);
    assert!((*opacity.borrow() - 0.6).abs() < 1e-4);
    assert!((*rotation.borrow() - 108.0).abs() < EPSILON);

    // Reverse and step to completion (0ms)
    tl.reverse();
    assert!(tl.is_playing());
    let has_more = tl.step(Duration::from_millis(350));
    assert!(!has_more);
    assert_eq!(tl.time(), Duration::ZERO);
    assert!(tl.is_completed());
    assert!((*x.borrow() - 0.0).abs() < EPSILON);
    assert!((*y.borrow() - 0.0).abs() < EPSILON);
    assert!((*scale.borrow() - 1.0).abs() < 1e-4);
    assert!((*opacity.borrow() - 0.0).abs() < 1e-4);
    assert!((*rotation.borrow() - 0.0).abs() < EPSILON);
}

#[test]
fn test_multi_channel_keyframes_and_instant_set_synchronization() {
    let mut dom = VirtualDom::new(|| {
        let x = use_signal(|| 0.0f64);
        let y = use_signal(|| 0.0f64);
        let scale = use_signal(|| 1.0f32);
        let opacity = use_signal(|| 0.0f32);
        let rotation = use_signal(|| 0.0f64);

        let tl = use_timeline(|b| {
            b.bind_signal("x", x);
            b.bind_signal("y", y);
            b.bind_signal("scale", scale);
            b.bind_signal("opacity", opacity);
            b.bind_signal("rotation", rotation);

            // Channel 1: x keyframes with distinct segment easing
            b.keyframes(
                "x",
                vec![
                    Keyframe::linear(0.0, 0.0f64),
                    Keyframe::new(0.5, 80.0f64, Ease::Linear),
                    Keyframe::linear(1.0, 100.0f64),
                ],
                Duration::from_millis(400),
                Position::Absolute(Duration::ZERO),
            );

            // Channel 2: y standard tween
            b.tween("y", 0.0f64, 200.0f64, Duration::from_millis(400), Ease::Linear, Position::Absolute(Duration::ZERO));

            // Channel 3: scale standard tween
            b.tween("scale", 1.0f32, 2.0f32, Duration::from_millis(400), Ease::Linear, Position::Absolute(Duration::ZERO));

            // Channel 4: opacity instant set at 200ms
            b.set("opacity", 1.0f32, Position::Absolute(Duration::from_millis(200)));

            // Channel 5: rotation instant set at 200ms
            b.set("rotation", 90.0f64, Position::Absolute(Duration::from_millis(200)));
        });

        // Pre-roll before 200ms
        tl.seek(Duration::from_millis(100));
        assert_eq!(tl.time(), Duration::from_millis(100));
        // x at 100ms (25% progress): halfway between 0.0 and 80.0 = 40.0
        assert!((x() - 40.0).abs() < EPSILON);
        assert!((y() - 50.0).abs() < EPSILON);
        assert!((scale() - 1.25).abs() < 1e-4);
        // opacity & rotation pre-roll hold initial (0.0)
        assert_eq!(opacity(), 0.0);
        assert_eq!(rotation(), 0.0);

        // At 200ms: instant set triggers
        tl.seek(Duration::from_millis(200));
        assert!((x() - 80.0).abs() < EPSILON);
        assert!((y() - 100.0).abs() < EPSILON);
        assert!((scale() - 1.5).abs() < 1e-4);
        assert_eq!(opacity(), 1.0);
        assert_eq!(rotation(), 90.0);

        // At 300ms: forward fill maintains instant set terminal values
        tl.seek(Duration::from_millis(300));
        assert!((x() - 90.0).abs() < EPSILON);
        assert!((y() - 150.0).abs() < EPSILON);
        assert!((scale() - 1.75).abs() < 1e-4);
        assert_eq!(opacity(), 1.0);
        assert_eq!(rotation(), 90.0);

        // Reverse back through the set point
        tl.reverse();
        tl.step(Duration::from_millis(200)); // 300ms -> 100ms
        assert_eq!(tl.time(), Duration::from_millis(100));
        assert!((x() - 40.0).abs() < EPSILON);
        assert!((y() - 50.0).abs() < EPSILON);
        assert!((scale() - 1.25).abs() < 1e-4);
        // Reversed back past 200ms into pre-roll -> initial values 0.0
        assert_eq!(opacity(), 0.0);
        assert_eq!(rotation(), 0.0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_multi_channel_label_seeking_and_relative_offsets() {
    let mut dom = VirtualDom::new(|| {
        let x = use_signal(|| 0.0f64);
        let y = use_signal(|| 0.0f64);
        let scale = use_signal(|| 1.0f32);
        let opacity = use_signal(|| 0.0f32);
        let rotation = use_signal(|| 0.0f64);

        let tl = use_timeline(|b| {
            b.bind_signal("x", x);
            b.bind_signal("y", y);
            b.bind_signal("scale", scale);
            b.bind_signal("opacity", opacity);
            b.bind_signal("rotation", rotation);

            b.label("start", Position::Absolute(Duration::ZERO));

            // Intro phase: 0..300ms
            b.tween("x", 0.0f64, 50.0f64, Duration::from_millis(300), Ease::Linear, Position::label("start"));
            b.tween("y", 0.0f64, 50.0f64, Duration::from_millis(300), Ease::Linear, Position::label("start"));
            b.tween("scale", 1.0f32, 1.2f32, Duration::from_millis(300), Ease::Linear, Position::label("start"));
            b.tween("opacity", 0.0f32, 0.5f32, Duration::from_millis(300), Ease::Linear, Position::label("start"));
            b.tween("rotation", 0.0f64, 45.0f64, Duration::from_millis(300), Ease::Linear, Position::label("start"));

            b.label("burst", Position::RecentEnd);

            // Burst phase: 300..600ms
            b.tween("x", 50.0f64, 200.0f64, Duration::from_millis(300), Ease::Linear, Position::label("burst"));
            b.tween("y", 50.0f64, 200.0f64, Duration::from_millis(300), Ease::Linear, Position::label("burst"));
            b.tween("scale", 1.2f32, 2.0f32, Duration::from_millis(300), Ease::Linear, Position::label("burst"));
            b.tween("opacity", 0.5f32, 1.0f32, Duration::from_millis(300), Ease::Linear, Position::label("burst"));
            b.tween("rotation", 45.0f64, 180.0f64, Duration::from_millis(300), Ease::Linear, Position::label("burst"));

            b.label("settle", Position::RecentEnd);
        });

        // Seek to "burst" label (300ms)
        let found = tl.seek_label("burst");
        assert!(found);
        assert_eq!(tl.time(), Duration::from_millis(300));
        assert!((x() - 50.0).abs() < EPSILON);
        assert!((y() - 50.0).abs() < EPSILON);
        assert!((scale() - 1.2).abs() < 1e-4);
        assert!((opacity() - 0.5).abs() < 1e-4);
        assert!((rotation() - 45.0).abs() < EPSILON);

        // Seek to "settle" label (600ms)
        let found = tl.seek_label("settle");
        assert!(found);
        assert_eq!(tl.time(), Duration::from_millis(600));
        assert!((x() - 200.0).abs() < EPSILON);
        assert!((y() - 200.0).abs() < EPSILON);
        assert!((scale() - 2.0).abs() < 1e-4);
        assert!((opacity() - 1.0).abs() < 1e-4);
        assert!((rotation() - 180.0).abs() < EPSILON);

        // Seek back to "start" label (0ms)
        let found = tl.seek_label("start");
        assert!(found);
        assert_eq!(tl.time(), Duration::ZERO);
        assert!((x() - 0.0).abs() < EPSILON);
        assert!((y() - 0.0).abs() < EPSILON);
        assert!((scale() - 1.0).abs() < 1e-4);
        assert!((opacity() - 0.0).abs() < 1e-4);
        assert!((rotation() - 0.0).abs() < EPSILON);

        rsx! {}
    });
    dom.rebuild_in_place();
}

