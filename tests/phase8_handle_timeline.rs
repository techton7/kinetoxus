//! Integration test suite for kinetoxus HandleTarget timeline binding & headless direct mutation (Phase 8).

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use kinetoxus::prelude::*;

#[test]
fn test_handle_timeline_headless_direct_memory_progression() {
    let x = Rc::new(RefCell::new(0.0f64));
    let y = Rc::new(RefCell::new(10.0f64));

    let mut builder = TimelineHookBuilder::new();
    builder.bind_handle("x", &x);
    builder.bind_handle("y", &y);
    builder.tween("x", 0.0, 100.0, Duration::from_millis(500), Ease::Linear, Position::End);
    builder.tween("y", 10.0, 50.0, Duration::from_millis(500), Ease::Linear, Position::RecentStart);

    let tl = builder.build().expect("timeline should compile cleanly");

    // $t = 0$ is applied immediately on build
    assert_eq!(*x.borrow(), 0.0);
    assert_eq!(*y.borrow(), 10.0);
    assert!(tl.is_paused());

    tl.play();
    assert!(tl.is_playing());

    // Step 250ms (50%)
    let has_more = tl.step(Duration::from_millis(250));
    assert!(has_more);
    assert!((*x.borrow() - 50.0).abs() < 1e-4);
    assert!((*y.borrow() - 30.0).abs() < 1e-4);

    // Step 250ms (100% - complete)
    let has_more = tl.step(Duration::from_millis(250));
    assert!(!has_more);
    assert!((*x.borrow() - 100.0).abs() < 1e-4);
    assert!((*y.borrow() - 50.0).abs() < 1e-4);
    assert!(tl.is_completed());
}

#[test]
fn test_handle_timeline_track_builder_ergonomics() {
    let handle_x = Rc::new(RefCell::new(0.0f64));
    let handle_target = HandleTarget::new(handle_x.clone());

    let mut builder = TimelineHookBuilder::new();
    builder.track("x", &handle_target, 0.0f64, |t| {
        t.tween(0.0, 50.0, Duration::from_millis(200), Ease::Linear, Position::End)
            .tween(50.0, 100.0, Duration::from_millis(200), Ease::Linear, Position::RecentEnd);
    });

    let tl = builder.build().expect("timeline should compile");
    assert_eq!(tl.duration(), Duration::from_millis(400));
    assert_eq!(*handle_x.borrow(), 0.0);

    // Synchronous scrub to 200ms
    tl.seek(Duration::from_millis(200));
    assert!((*handle_x.borrow() - 50.0).abs() < 1e-4);

    // Scrub to 75% (300ms)
    tl.set_progress(0.75);
    assert!((*handle_x.borrow() - 75.0).abs() < 1e-4);
}

#[test]
fn test_handle_timeline_transform_2d_multi_field_pose() {
    let pose = Transform2D::new(0.0, 0.0, 1.0, 0.0);

    let mut builder = TimelineHookBuilder::new();
    builder
        .bind_handle("x", &pose.x)
        .bind_handle("y", &pose.y)
        .bind_handle("scale", &pose.scale)
        .bind_handle("rotation", &pose.rotation);

    builder
        .tween("x", 0.0f32, 100.0f32, Duration::from_millis(400), Ease::QuadOut, Position::End)
        .tween("y", 0.0f32, 200.0f32, Duration::from_millis(400), Ease::QuadOut, Position::RecentStart)
        .tween("scale", 1.0f32, 1.5f32, Duration::from_millis(400), Ease::Linear, Position::RecentStart)
        .tween("rotation", 0.0f32, 360.0f32, Duration::from_millis(400), Ease::Linear, Position::RecentStart);

    let tl = builder.build().expect("transform timeline should compile");
    assert_eq!(*pose.x.borrow(), 0.0);
    assert_eq!(*pose.y.borrow(), 0.0);
    assert_eq!(*pose.scale.borrow(), 1.0);
    assert_eq!(*pose.rotation.borrow(), 0.0);

    // Scrub to 50%
    tl.set_progress(0.5);
    assert!((*pose.scale.borrow() - 1.25).abs() < 1e-3);
    assert!((*pose.rotation.borrow() - 180.0).abs() < 1e-3);

    // Scrub to 100%
    tl.set_progress(1.0);
    assert!((*pose.x.borrow() - 100.0).abs() < 1e-3);
    assert!((*pose.y.borrow() - 200.0).abs() < 1e-3);
    assert!((*pose.scale.borrow() - 1.5).abs() < 1e-3);
    assert!((*pose.rotation.borrow() - 360.0).abs() < 1e-3);
}

#[test]
fn test_handle_timeline_zero_signal_isolation_guarantee() {
    // Proves that handle timeline runs without requiring any Dioxus VirtualDom or Scope runtime
    let val = Rc::new(RefCell::new(0.0f64));
    let mut builder = TimelineHookBuilder::new();
    builder.bind_handle("val", &val);
    builder.keyframes(
        "val",
        vec![
            Keyframe::linear(0.0, 0.0f64),
            Keyframe::new(0.5, 80.0f64, Ease::QuadIn),
            Keyframe::linear(1.0, 100.0f64),
        ],
        Duration::from_millis(1000),
        Position::End,
    );

    let tl = builder.build().unwrap();
    tl.play();

    // Advance 500ms
    tl.step(Duration::from_millis(500));
    assert!((*val.borrow() - 80.0).abs() < 1e-4);

    // Scrub backwards
    tl.set_progress(0.0);
    assert_eq!(*val.borrow(), 0.0);
}
