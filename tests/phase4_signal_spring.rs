//! Integration test suite for kinetoxus SignalTarget spring binding & reactive proof (Phase 4).

use std::time::Duration;
use dioxus::prelude::*;
use kinetoxus::prelude::*;
use kinetocore::spring::SpringConfig;

#[test]
fn test_signal_spring_underdamped_overshoot_and_settle() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f64);
        let motion = use_motion();

        // Underdamped configuration: low damping relative to mass/stiffness
        let config = SpringConfig::new(1.0, 100.0, 10.0).unwrap();
        let handle = motion.spring(signal, 100.0f64, config);

        assert_eq!(signal(), 0.0);
        assert_eq!(motion.active_count(), 1);
        assert!(handle.is_active());

        // Step through simulation until overshoot occurs and then settles
        let mut max_val = 0.0f64;
        let mut ticked_count = 0;
        let mut settled = false;

        while ticked_count < 200 {
            let has_more = motion.tick(Duration::from_millis(16));
            let val = signal();
            if val > max_val {
                max_val = val;
            }
            if !has_more {
                settled = true;
                break;
            }
            ticked_count += 1;
        }

        assert!(settled, "Spring should settle");
        assert!(max_val > 100.0, "Underdamped spring must overshoot target 100.0, got max {max_val}");
        assert!((signal() - 100.0).abs() < 1e-6, "Should settle at exact target 100.0, got {}", signal());
        assert_eq!(motion.active_count(), 0);

        // Additional ticks do not re-trigger or mutate target
        motion.tick(Duration::from_millis(16));
        assert_eq!(signal(), 100.0);
        assert_eq!(motion.active_count(), 0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_signal_spring_critically_damped_smooth() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f64);
        let motion = use_motion();

        // Critically damped configuration (e.g. damping = 2 * sqrt(mass * stiffness) = 2 * 10 = 20)
        let config = SpringConfig::new(1.0, 100.0, 20.0).unwrap();
        motion.spring(signal, 100.0f64, config);

        let mut max_val = 0.0f64;
        let mut settled = false;
        for _ in 0..200 {
            let has_more = motion.tick(Duration::from_millis(16));
            let val = signal();
            if val > max_val {
                max_val = val;
            }
            if !has_more {
                settled = true;
                break;
            }
        }

        assert!(settled);
        assert!(max_val <= 100.0 + 1e-4, "Critically damped spring must not overshoot significantly, got max {max_val}");
        assert!((signal() - 100.0).abs() < 1e-6);
        assert_eq!(motion.active_count(), 0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_signal_spring_overdamped_slow_convergence() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f64);
        let motion = use_motion();

        // Overdamped configuration
        let config = SpringConfig::new(1.0, 100.0, 50.0).unwrap();
        motion.spring(signal, 100.0f64, config);

        let mut max_val = 0.0f64;
        let mut settled = false;
        for _ in 0..300 {
            let has_more = motion.tick(Duration::from_millis(16));
            let val = signal();
            if val > max_val {
                max_val = val;
            }
            if !has_more {
                settled = true;
                break;
            }
        }

        assert!(settled);
        assert!(max_val <= 100.0 + 1e-4, "Overdamped spring must not overshoot, got max {max_val}");
        assert!((signal() - 100.0).abs() < 1e-6);
        assert_eq!(motion.active_count(), 0);

        rsx! {}
    });
    dom.rebuild_in_place();
}

#[test]
fn test_signal_spring_mid_flight_retargeting_and_chain() {
    let mut dom = VirtualDom::new(|| {
        let signal = use_signal(|| 0.0f64);
        let motion = use_motion();

        let config = SpringConfig::new(1.0, 100.0, 20.0).unwrap();
        let handle = motion.spring(signal, 100.0f64, config);

        // Advance partially (~100ms)
        for _ in 0..6 {
            motion.tick(Duration::from_millis(16));
        }

        let pos_before = signal();
        assert!(pos_before > 0.0 && pos_before < 100.0);

        // Retarget mid-flight to 200.0 via handle.set_target or motion.spring
        handle.set_target(200.0);
        assert!(handle.is_active());

        // Sample immediately at next small dt - verify smooth progression (velocity carried forward)
        motion.tick(Duration::from_millis(16));
        let pos_after = signal();
        assert!(pos_after > pos_before, "Position should continue forward smoothly without pop");

        // Chain retargeting: 100 -> 50 -> 250 -> 0
        handle.set_target(50.0);
        for _ in 0..5 {
            motion.tick(Duration::from_millis(16));
        }

        handle.set_target(250.0);
        for _ in 0..10 {
            motion.tick(Duration::from_millis(16));
        }

        handle.set_target(0.0);
        let mut settled = false;
        for _ in 0..300 {
            let has_more = motion.tick(Duration::from_millis(16));
            if !has_more {
                settled = true;
                break;
            }
        }

        assert!(settled);
        assert!((signal() - 0.0).abs() < 1e-6);
        assert_eq!(motion.active_count(), 0);

        rsx! {}
    });
    dom.rebuild_in_place();
}
