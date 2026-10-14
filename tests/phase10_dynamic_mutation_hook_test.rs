//! Integration test suite for Phase 10: Dynamic Timeline Mutation & Epoch Snapshots (kinetoxus).
//!
//! Tests runtime mutation of active timelines without unmounting components,
//! monotonic epoch counter progression, playhead continuity, and live target synchronization.

use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;

const EPSILON: f64 = 1e-4;

#[test]
fn test_dynamic_mutation_epoch_and_target_synchronization() {
    let mut dom = VirtualDom::new(|| {
        let x = use_signal(|| 0.0f64);
        let y = use_signal(|| 0.0f64);

        let tl = use_timeline(|b| {
            b.track("x", x, 0.0f64, |t| {
                t.tween(100.0f64, Duration::from_millis(1000), Ease::Linear);
            });
            b.bind_signal("y", y);
        });

        // Initial epoch is 1
        assert_eq!(tl.epoch(), 1);
        assert_eq!(x(), 0.0);

        // Step to 400ms (x = 40.0)
        tl.step(Duration::from_millis(400));
        assert_eq!(tl.time(), Duration::from_millis(400));
        assert!((x() - 40.0).abs() < EPSILON);

        // 1. Dynamic insertion ahead of playhead: insert clip on "y" at t = 200ms to 600ms
        let res = tl.insert_clip(
            "y",
            Position::Absolute(Duration::from_millis(200)),
            0.0f64,
            50.0f64,
            Duration::from_millis(400),
            Ease::Linear,
            ActiveOverlapPolicy::RejectIfActiveOverlap,
        );
        assert!(res.is_ok());

        // Epoch increments monotonically
        assert_eq!(tl.epoch(), 2);
        // Playhead preserved at 400ms
        assert_eq!(tl.time(), Duration::from_millis(400));
        // Bound signal y is immediately sampled at 400ms:
        // y interval is [200ms, 600ms] (span 400ms), at 400ms it is 50% through -> 25.0!
        assert!((y() - 25.0).abs() < EPSILON);
        assert!((x() - 40.0).abs() < EPSILON);

        // 2. Dynamic removal of clip with target ownership handoff
        let handoff = tl
            .remove_clip("y", ClipTarget::Index(0))
            .expect("clip removal should succeed");
        assert!(handoff.is_some());
        let handoff = handoff.unwrap();
        assert_eq!(handoff.track, "y");
        assert_eq!(handoff.handed_off_at, Duration::from_millis(400));

        // Epoch advanced to 3
        assert_eq!(tl.epoch(), 3);
        assert_eq!(tl.time(), Duration::from_millis(400));

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_dynamic_swap_subtimeline() {
    let mut dom = VirtualDom::new(|| {
        let card_x = use_signal(|| 0.0f64);

        let tl = use_timeline(|b| {
            b.bind_signal("card::x", card_x);
            b.sub_timeline("card", Position::Absolute(Duration::ZERO), |sub| {
                sub.track("x", card_x, 0.0f64, |t| {
                    t.tween(100.0f64, Duration::from_millis(500), Ease::Linear);
                });
            });
        });

        assert_eq!(tl.epoch(), 1);
        tl.step(Duration::from_millis(250));
        assert!((card_x() - 50.0).abs() < EPSILON);

        // Build a replacement sub-timeline
        let mut sub_builder = SubTimelineBuilder::new("card", Position::Absolute(Duration::ZERO));
        sub_builder.track("x", 0.0f64, |t| {
            t.tween(300.0f64, Duration::from_millis(500), Ease::Linear);
        });
        let new_node = sub_builder.build_node();

        // Swap sub-timeline using ApplyAtCurrentTime policy
        let res = tl.swap_subtimeline(None, new_node, ActiveOverlapPolicy::ApplyAtCurrentTime);
        assert!(res.is_ok());

        assert_eq!(tl.epoch(), 2);
        assert_eq!(tl.time(), Duration::from_millis(250));

        // Advance to 500ms
        tl.step(Duration::from_millis(250));
        assert_eq!(tl.time(), Duration::from_millis(500));

        rsx! {}
    });
    dom.rebuild_in_place();
}
