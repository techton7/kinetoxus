//! Integration test suite for Phase 10: Composable Timeline Graphs (kinetoxus use_timeline).
//!
//! Tests hierarchical choreography, affine timeScale mapping, scoped label paths,
//! and reactive SignalTarget synchronization across nested timelines.

use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;

const EPSILON: f64 = 1e-4;

#[test]
fn test_nested_choreography_signals() {
    let mut dom = VirtualDom::new(|| {
        let card_rot = use_signal(|| 0.0f64);
        let card_scale = use_signal(|| 1.0f32);
        let hud_alpha = use_signal(|| 0.0f32);

        let tl = use_timeline(|b| {
            // Top-level track
            b.track("hud_alpha", hud_alpha, 0.0f32, |t| {
                t.tween(1.0f32, Duration::from_millis(400), Ease::Linear);
            });

            // Scoped nested sub-timeline with 2.0x timeScale acceleration starting at 200ms
            b.sub_timeline("card_anim", Position::Absolute(Duration::from_millis(200)), |sub| {
                sub.set_time_scale(2.0).expect("positive time_scale");

                // In sub coordinates: 400ms duration.
                // In root coordinates: 400ms / 2.0 = 200ms span, active from [200ms, 400ms].
                sub.track("rotation", card_rot, 0.0f64, |t| {
                    t.tween(360.0f64, Duration::from_millis(400), Ease::Linear);
                });

                // In sub coordinates: 200ms duration.
                // In root coordinates: 200ms / 2.0 = 100ms span, active from [200ms, 300ms].
                sub.track("scale", card_scale, 1.0f32, |t| {
                    t.tween(2.0f32, Duration::from_millis(200), Ease::Linear);
                });

                // Local label at 200ms local -> 200ms offset + 200ms/2.0 = 300ms root
                sub.add_label("apex", Position::Absolute(Duration::from_millis(200)));
            });
        });

        // 1. Initial evaluation at t = 0
        assert!((hud_alpha() - 0.0).abs() < 1e-4);
        assert!((card_rot() - 0.0).abs() < EPSILON);
        assert!((card_scale() - 1.0).abs() < 1e-4);
        assert_eq!(tl.duration(), Duration::from_millis(400));
        assert!(tl.is_paused());

        // 2. Play and step to 200ms (sub-timeline start boundary)
        tl.play();
        assert!(tl.is_playing());
        tl.step(Duration::from_millis(200));
        assert_eq!(tl.time(), Duration::from_millis(200));
        assert!((hud_alpha() - 0.5).abs() < 1e-4);
        assert!((card_rot() - 0.0).abs() < EPSILON);
        assert!((card_scale() - 1.0).abs() < 1e-4);

        // 3. Step to 300ms (card_scale reaches completion at 2.0; card_rot is halfway at 180.0)
        tl.step(Duration::from_millis(100));
        assert_eq!(tl.time(), Duration::from_millis(300));
        assert!((hud_alpha() - 0.75).abs() < 1e-4);
        assert!((card_scale() - 2.0).abs() < 1e-4);
        assert!((card_rot() - 180.0).abs() < EPSILON);

        // 4. Step to 400ms (entire hierarchy completes)
        let has_more = tl.step(Duration::from_millis(100));
        assert!(!has_more);
        assert_eq!(tl.time(), Duration::from_millis(400));
        assert!((hud_alpha() - 1.0).abs() < 1e-4);
        assert!((card_scale() - 2.0).abs() < 1e-4);
        assert!((card_rot() - 360.0).abs() < EPSILON);
        assert!(tl.is_completed());

        // 5. Seek using qualified namespaced label path
        let found = tl.seek_label("card_anim::apex");
        assert!(found);
        assert_eq!(tl.time(), Duration::from_millis(300));
        assert!((card_scale() - 2.0).abs() < 1e-4);
        assert!((card_rot() - 180.0).abs() < EPSILON);
        assert!((hud_alpha() - 0.75).abs() < 1e-4);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_deeply_nested_subtimelines_and_labels() {
    let mut dom = VirtualDom::new(|| {
        let root_sig = use_signal(|| 0.0f64);
        let modal_sig = use_signal(|| 0.0f64);
        let button_sig = use_signal(|| 0.0f64);

        let tl = use_timeline(|b| {
            b.track("root_val", root_sig, 0.0f64, |t| {
                t.tween(100.0f64, Duration::from_millis(600), Ease::Linear);
            });

            b.sub_timeline("modal", Position::Absolute(Duration::from_millis(100)), |m| {
                m.set_time_scale(2.0).expect("valid scale");
                m.track("modal_val", modal_sig, 0.0f64, |t| {
                    t.tween(50.0f64, Duration::from_millis(400), Ease::Linear);
                });

                // Level 3 nested sub-timeline: modal::card
                m.sub_timeline("card", Position::Absolute(Duration::from_millis(50)), |c| {
                    c.track("button_val", button_sig, 0.0f64, |t| {
                        t.tween(25.0f64, Duration::from_millis(200), Ease::Linear);
                    });
                    c.add_label("click", Position::Absolute(Duration::from_millis(100)));
                });
            });
        });

        assert_eq!(*root_sig.read(), 0.0);
        assert_eq!(*modal_sig.read(), 0.0);
        assert_eq!(*button_sig.read(), 0.0);

        // In root time:
        // modal starts at 100ms.
        // inside modal (2x speed): card starts at 50ms local -> 100ms + 50ms / 2 = 125ms root.
        // inside card: label click is at 100ms card-local -> 125ms + 100ms / 2 = 175ms root.
        let found = tl.seek_label("modal::card::click");
        assert!(found);
        assert_eq!(tl.time(), Duration::from_millis(175));

        // modal_val at 75ms modal-time (150ms local) -> 150 / 400 * 50 = 18.75
        assert!((*modal_sig.read() - 18.75).abs() < EPSILON);
        // button_val at 50ms card-time (100ms local) -> 100 / 200 * 25 = 12.5
        assert!((*button_sig.read() - 12.5).abs() < EPSILON);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_invalid_nested_time_scale_returns_error() {
    let mut builder = TimelineHookBuilder::new();
    builder.sub_timeline("bad_sub", Position::Absolute(Duration::ZERO), |sub| {
        let err_zero = sub.set_time_scale(0.0);
        assert!(err_zero.is_err());
        let err_neg = sub.set_time_scale(-1.5);
        assert!(err_neg.is_err());
        let err_nan = sub.set_time_scale(f64::NAN);
        assert!(err_nan.is_err());
    });
}
