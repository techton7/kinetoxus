//! Integration test suite for Phase 10: Composable Timeline Graphs (HandleTarget Zero-VDOM Proof).
//!
//! Tests headless interior-mutable HandleTarget bindings inside nested sub-timelines,
//! proving direct memory mutation without triggering Dioxus reactive signal re-renders.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;

const EPSILON: f64 = 1e-4;

#[test]
fn test_nested_subtimeline_headless_handle_progression() {
    let root_x = Rc::new(RefCell::new(0.0f64));
    let child_y = Rc::new(RefCell::new(10.0f64));
    let child_scale = Rc::new(RefCell::new(1.0f32));

    let mut builder = TimelineHookBuilder::new();
    builder.bind_handle("root_x", &root_x);
    builder.tween("root_x", 0.0f64, 100.0f64, Duration::from_millis(500), Ease::Linear, Position::End);

    builder.sub_timeline("card", Position::Absolute(Duration::from_millis(100)), |sub| {
        sub.set_time_scale(2.0).expect("positive scale");
        sub.track("y", &child_y, 10.0f64, |t| {
            t.tween(50.0f64, Duration::from_millis(400), Ease::Linear);
        });
        sub.track("scale", &child_scale, 1.0f32, |t| {
            t.tween(3.0f32, Duration::from_millis(200), Ease::Linear);
        });
    });

    let tl = builder.build().expect("nested timeline builds cleanly");

    // Immediate initial values at t = 0
    assert_eq!(*root_x.borrow(), 0.0);
    assert_eq!(*child_y.borrow(), 10.0);
    assert_eq!(*child_scale.borrow(), 1.0);

    // Root duration: max(500ms, 100ms + 400ms/2.0) = max(500, 300) = 500ms
    assert_eq!(tl.duration(), Duration::from_millis(500));

    // Play and step to 100ms
    tl.play();
    assert!(tl.is_playing());
    tl.step(Duration::from_millis(100));
    assert!((*root_x.borrow() - 20.0).abs() < EPSILON);
    assert!((*child_y.borrow() - 10.0).abs() < EPSILON);
    assert!((*child_scale.borrow() - 1.0).abs() < 1e-4);

    // Step to 200ms: child_scale was 200ms local / 2.0 = 100ms root -> finished at 200ms!
    tl.step(Duration::from_millis(100));
    assert!((*root_x.borrow() - 40.0).abs() < EPSILON);
    // y at 100ms root (200ms local / 400ms * 40 + 10 = 30.0)
    assert!((*child_y.borrow() - 30.0).abs() < EPSILON);
    assert!((*child_scale.borrow() - 3.0).abs() < 1e-4);

    // Step to 300ms: card finishes
    tl.step(Duration::from_millis(100));
    assert!((*root_x.borrow() - 60.0).abs() < EPSILON);
    assert!((*child_y.borrow() - 50.0).abs() < EPSILON);
    assert!((*child_scale.borrow() - 3.0).abs() < 1e-4);

    // Step to 500ms: root finishes
    let has_more = tl.step(Duration::from_millis(200));
    assert!(!has_more);
    assert!((*root_x.borrow() - 100.0).abs() < EPSILON);
    assert!(tl.is_completed());
}

static RENDER_COUNT: AtomicUsize = AtomicUsize::new(0);
static HANDLE_VAL: AtomicU64 = AtomicU64::new(0);

fn nested_handle_app() -> Element {
    RENDER_COUNT.fetch_add(1, Ordering::SeqCst);
    let handle_cell = use_hook(|| Rc::new(RefCell::new(0.0f64)));
    let handle_for_tl = handle_cell.clone();

    let tl = use_timeline(move |b| {
        b.sub_timeline("nested_box", Position::Absolute(Duration::ZERO), move |sub| {
            sub.track("direct_mem", &handle_for_tl, 0.0f64, |t| {
                t.tween(500.0f64, Duration::from_millis(1000), Ease::Linear);
            });
        });
    });

    tl.play();
    tl.step(Duration::from_millis(500));
    HANDLE_VAL.store(handle_cell.borrow().to_bits(), Ordering::SeqCst);

    rsx! {
        div { "Render count: {RENDER_COUNT.load(Ordering::SeqCst)}" }
    }
}

#[test]
fn test_nested_handle_zero_signal_isolation_in_vdom() {
    RENDER_COUNT.store(0, Ordering::SeqCst);
    HANDLE_VAL.store(0, Ordering::SeqCst);

    let mut dom = VirtualDom::new(nested_handle_app);
    dom.rebuild_in_place();

    // Exactly 1 initial render on mount
    assert_eq!(RENDER_COUNT.load(Ordering::SeqCst), 1);
    // Direct memory was updated in-place to 250.0
    let val = f64::from_bits(HANDLE_VAL.load(Ordering::SeqCst));
    assert!((val - 250.0).abs() < EPSILON);

    // Run DOM render loop passes - handles cause ZERO dirty signal notifications or re-renders
    dom.process_events();
    assert_eq!(RENDER_COUNT.load(Ordering::SeqCst), 1, "render count must remain exactly 1");
}

#[test]
fn test_nested_handle_transform_2d_multi_field_pose() {
    let pose = Transform2D::new(0.0, 0.0, 1.0, 0.0);

    let mut builder = TimelineHookBuilder::new();
    builder.sub_timeline("card_pose", Position::Absolute(Duration::ZERO), |sub| {
        sub.bind_handle("x", &pose.x);
        sub.bind_handle("y", &pose.y);
        sub.bind_handle("scale", &pose.scale);
        sub.bind_handle("rotation", &pose.rotation);

        sub.tween("x", 0.0f32, 100.0f32, Duration::from_millis(400), Ease::Linear, Position::End);
        sub.tween("y", 0.0f32, 200.0f32, Duration::from_millis(400), Ease::Linear, Position::RecentStart);
        sub.tween("scale", 1.0f32, 1.5f32, Duration::from_millis(400), Ease::Linear, Position::RecentStart);
        sub.tween("rotation", 0.0f32, 360.0f32, Duration::from_millis(400), Ease::Linear, Position::RecentStart);
    });

    let tl = builder.build().expect("timeline compiles");
    assert_eq!(*pose.x.borrow(), 0.0);
    assert_eq!(*pose.scale.borrow(), 1.0);

    tl.seek(Duration::from_millis(200));
    assert!((*pose.x.borrow() - 50.0).abs() < 1e-3);
    assert!((*pose.y.borrow() - 100.0).abs() < 1e-3);
    assert!((*pose.scale.borrow() - 1.25).abs() < 1e-3);
    assert!((*pose.rotation.borrow() - 180.0).abs() < 1e-3);

    tl.seek(Duration::from_millis(400));
    assert!((*pose.x.borrow() - 100.0).abs() < 1e-3);
    assert!((*pose.y.borrow() - 200.0).abs() < 1e-3);
    assert!((*pose.scale.borrow() - 1.5).abs() < 1e-3);
    assert!((*pose.rotation.borrow() - 360.0).abs() < 1e-3);
}
