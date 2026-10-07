//! Integration test suite for kinetoxus timeline frame loop lifecycle, dormancy, and drop cleanup (Phase 8).

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;
use oxidase::frame::tick as tick_frame;

#[test]
fn test_lifecycle_frame_loop_dormancy_on_pause() {
    let mut dom = VirtualDom::new(|| {
        let x = use_signal(|| 0.0f64);

        let tl = use_timeline(|b| {
            b.bind_signal("x", x);
            b.tween("x", 0.0, 100.0, Duration::from_millis(500), Ease::Linear, Position::End);
        });

        // 1. Initially created paused: frame loop must NOT be active
        assert!(!tl.is_frame_loop_active(), "Frame loop must be dormant on paused timeline");
        assert!(tl.is_paused());

        // External frame tick while paused does NOT advance timeline
        tick_frame(Duration::from_millis(100));
        assert_eq!(x(), 0.0);
        assert_eq!(tl.time(), Duration::ZERO);

        // 2. Play activates frame loop
        tl.play();
        assert!(tl.is_frame_loop_active(), "Frame loop must be active while playing");
        assert!(tl.is_playing());

        // Frame tick advances active timeline
        tick_frame(Duration::from_millis(100));
        assert!((x() - 20.0).abs() < 1e-4);
        assert_eq!(tl.time(), Duration::from_millis(100));

        // 3. Pause immediately unregisters frame loop
        tl.pause();
        assert!(!tl.is_frame_loop_active(), "Frame loop must unregister on pause");
        assert!(tl.is_paused());

        // Subsequent ticks do NOT advance timeline
        tick_frame(Duration::from_millis(100));
        assert!((x() - 20.0).abs() < 1e-4);
        assert_eq!(tl.time(), Duration::from_millis(100));

        // 4. Resume playing reactivates frame loop
        tl.play();
        assert!(tl.is_frame_loop_active());
        tick_frame(Duration::from_millis(100));
        assert!((x() - 40.0).abs() < 1e-4);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_lifecycle_automatic_unregistration_on_completion() {
    let mut dom = VirtualDom::new(|| {
        let val = use_signal(|| 0.0f64);

        let tl = use_timeline(|b| {
            b.bind_signal("val", val);
            b.tween("val", 0.0, 100.0, Duration::from_millis(200), Ease::Linear, Position::End);
        });

        tl.play();
        assert!(tl.is_frame_loop_active());

        // Step to 150ms (75%)
        tick_frame(Duration::from_millis(150));
        assert!(tl.is_frame_loop_active());
        assert!((val() - 75.0).abs() < 1e-4);

        // Step past completion (reaches 200ms)
        tick_frame(Duration::from_millis(100));
        assert!(tl.is_completed());
        assert!(!tl.is_playing());
        assert!(!tl.is_frame_loop_active(), "Frame loop must automatically unregister upon completion");
        assert!((val() - 100.0).abs() < 1e-4);

        // Additional ticks while completed do nothing
        tick_frame(Duration::from_millis(50));
        assert_eq!(val(), 100.0);
        assert!(!tl.is_frame_loop_active());

        rsx! {}
    });
    dom.rebuild_in_place();
}

thread_local! {
    static SHOW_SIG: RefCell<Option<Signal<bool>>> = const { RefCell::new(None) };
    static CAPTURED_TL: RefCell<Option<TimelineController>> = const { RefCell::new(None) };
}

#[component]
fn ChildTimelineComponent() -> Element {
    let count = use_signal(|| 0.0f64);

    let tl = use_timeline(|b| {
        b.bind_signal("count", count);
        b.tween("count", 0.0, 100.0, Duration::from_millis(500), Ease::Linear, Position::End);
    });

    tl.play();

    CAPTURED_TL.with(|c| *c.borrow_mut() = Some(tl.clone()));

    rsx! {
        div { "Child count: {count}" }
    }
}

#[component]
fn LifecycleTestApp() -> Element {
    let show = use_signal(|| true);
    SHOW_SIG.with(|c| *c.borrow_mut() = Some(show));

    rsx! {
        if show() {
            ChildTimelineComponent {}
        }
    }
}

#[test]
fn test_lifecycle_component_unmount_drop_cleanup() {
    SHOW_SIG.with(|c| *c.borrow_mut() = None);
    CAPTURED_TL.with(|c| *c.borrow_mut() = None);

    let mut dom = VirtualDom::new(LifecycleTestApp);

    // Mount ChildTimelineComponent
    dom.rebuild_in_place();

    // Verify child timeline started playing and registered frame loop
    let child_tl = CAPTURED_TL
        .with(|c| c.borrow().as_ref().expect("child controller should be captured").clone());
    assert!(
        child_tl.is_frame_loop_active(),
        "Frame loop must be active while child component is mounted"
    );

    // Tick frame while mounted to advance
    tick_frame(Duration::from_millis(50));

    // Toggle show signal to false inside Dioxus runtime
    dom.in_runtime(|| {
        SHOW_SIG.with(|c| {
            if let Some(mut sig) = *c.borrow() {
                sig.set(false);
            }
        });
    });

    // Render immediate to process unmount
    let _mutations = dom.render_immediate_to_vec();

    // Child is unmounted, so use_drop ran and cancelled the frame loop
    assert!(
        !child_tl.is_frame_loop_active(),
        "Frame loop must unregister when component is unmounted via use_drop"
    );
    assert!(child_tl.is_paused());
}

#[test]
fn test_lifecycle_direct_controller_drop_cleanup() {
    let x = Rc::new(RefCell::new(0.0f64));
    let mut builder = TimelineHookBuilder::new();
    builder.bind_handle("x", &x);
    builder.tween("x", 0.0, 100.0, Duration::from_millis(500), Ease::Linear, Position::End);
    let tl = builder.build().unwrap();

    tl.play();
    assert!(tl.is_frame_loop_active());

    // Explicit cancel stops the driver
    tl.cancel();
    assert!(!tl.is_frame_loop_active());

    // Playing again reactivates
    tl.play();
    assert!(tl.is_frame_loop_active());

    // Dropping the controller stops the frame loop via RAII
    drop(tl);

    // Subsequent tick does not crash or advance dropped handle
    tick_frame(Duration::from_millis(50));
    assert_eq!(*x.borrow(), 0.0);
}
