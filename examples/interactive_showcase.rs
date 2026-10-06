//! Kinetoxus Interactive Showcase Example.
//! Matches CONTRACT.md specifications exactly.

use std::time::Duration;
use dioxus::prelude::*;
use kinetoxus::prelude::*;
use oxidase::prelude::*;

#[oxidase::main]
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut x = use_signal(|| 0.0f32);
    let mut y = use_signal(|| 0.0f32);
    let mut scale = use_signal(|| 1.0f32);
    let mut rot = use_signal(|| 0.0f32);
    let mut opacity = use_signal(|| 1.0f32);
    let mut fps = use_signal(|| 60.0f64);
    let mut status = use_signal(|| "idle".to_string());

    let motion = use_motion();

    use_frame(move |info| {
        let dt = info.delta.as_secs_f64();
        if dt > 0.001 {
            fps.set(1.0 / dt);
        }
    });

    let m_pop = motion.clone();
    let m_slide = motion.clone();
    let m_spin = motion.clone();
    let m_center = motion.clone();
    let m_fade = motion.clone();
    let m_drop = motion.clone();
    let m_reset = motion;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; gap: 1.5rem; background: #0f172a; color: #f8fafc; padding: 2rem;",

            // Telemetry HUD
            div {
                id: "telemetry-hud",
                style: "display: flex; gap: 1rem; background: rgba(30, 41, 59, 0.8); padding: 0.5rem 1rem; border-radius: 8px; font-family: monospace; font-size: 0.85rem; color: #38bdf8;",
                span { id: "hud-fps", "fps: {fps():.0}" }
                span { id: "hud-x", "x: {x():.1}px" }
                span { id: "hud-y", "y: {y():.1}px" }
                span { id: "hud-scale", "scale: {scale():.2}" }
                span { id: "hud-rot", "rot: {rot():.1}°" }
                span { id: "hud-opacity", "opacity: {opacity():.2}" }
                span { id: "hud-status", "status: {status()}" }
            }

            // Card element
            div {
                id: "animated-card",
                style: "width: 120px; height: 120px; background: #6366f1; border-radius: 12px; transform: translate({x()}px, {y()}px) scale({scale()}) rotate({rot()}deg); opacity: {opacity()}; display: flex; align-items: center; justify-content: center; font-weight: bold; color: white;",
                "Card"
            }

            // Action buttons
            div {
                style: "display: flex; gap: 0.5rem; flex-wrap: wrap; justify-content: center;",
                button {
                    id: "btn-pop",
                    onclick: move |_| {
                        status.set("animating".to_string());
                        m_pop.from_to(scale, 0.7f32, 1.0f32, Duration::from_millis(500))
                            .ease(Ease::BackOut);
                        status.set("settled".to_string());
                    },
                    "Pop"
                }
                button {
                    id: "btn-slide",
                    onclick: move |_| {
                        status.set("animating".to_string());
                        m_slide.from_to(x, -150.0f32, 150.0f32, Duration::from_millis(700))
                            .ease(Ease::QuadInOut);
                        status.set("settled".to_string());
                    },
                    "Slide"
                }
                button {
                    id: "btn-spin",
                    onclick: move |_| {
                        status.set("animating".to_string());
                        m_spin.from_to(rot, 0.0f32, 360.0f32, Duration::from_millis(600))
                            .ease(Ease::QuadInOut);
                        status.set("settled".to_string());
                    },
                    "Spin"
                }
                button {
                    id: "btn-to-center",
                    onclick: move |_| {
                        status.set("interrupted".to_string());
                        m_center.to(x, 0.0f32, Duration::from_millis(400));
                        m_center.to(y, 0.0f32, Duration::from_millis(400));
                        status.set("settled".to_string());
                    },
                    "To Center"
                }
                button {
                    id: "btn-fade",
                    onclick: move |_| {
                        status.set("animating".to_string());
                        m_fade.to(opacity, 0.2f32, Duration::from_millis(300));
                        m_fade.to(opacity, 1.0f32, Duration::from_millis(300));
                        status.set("settled".to_string());
                    },
                    "Fade"
                }
                button {
                    id: "btn-from-top",
                    onclick: move |_| {
                        status.set("animating".to_string());
                        m_drop.from(y, -200.0f32, Duration::from_millis(500))
                            .ease(Ease::BounceOut);
                        status.set("settled".to_string());
                    },
                    "From Top"
                }
                button {
                    id: "btn-reset",
                    onclick: move |_| {
                        status.set("reset".to_string());
                        m_reset.cancel_all();
                        x.set(0.0);
                        y.set(0.0);
                        scale.set(1.0);
                        rot.set(0.0);
                        opacity.set(1.0);
                    },
                    "Reset"
                }
            }
        }
    }
}
