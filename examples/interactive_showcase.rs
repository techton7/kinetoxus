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
    let x = use_signal(|| 0.0f32);
    let y = use_signal(|| 0.0f32);
    let scale = use_signal(|| 1.0f32);
    let rot = use_signal(|| 0.0f32);
    let opacity = use_signal(|| 1.0f32);
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
                        status.set("pop".to_string());
                        m_pop.from_to(scale, 0.7f32, 1.0f32, Duration::from_millis(500))
                            .ease(Ease::BackOut);
                    },
                    "Pop"
                }
                button {
                    id: "btn-slide",
                    onclick: move |_| {
                        status.set("slide".to_string());
                        m_slide.from_to(x, -150.0f32, 150.0f32, Duration::from_millis(700))
                            .ease(Ease::QuadInOut);
                    },
                    "Slide"
                }
                button {
                    id: "btn-spin",
                    onclick: move |_| {
                        status.set("spin".to_string());
                        m_spin.from_to(rot, 0.0f32, 360.0f32, Duration::from_millis(600))
                            .ease(Ease::QuadInOut);
                    },
                    "Spin"
                }
                button {
                    id: "btn-to-center",
                    onclick: move |_| {
                        status.set("interrupted_to_center".to_string());
                        m_center.to(x, 0.0f32, Duration::from_millis(400));
                        m_center.to(y, 0.0f32, Duration::from_millis(400));
                    },
                    "To Center"
                }
                button {
                    id: "btn-fade",
                    onclick: move |_| {
                        status.set("fade".to_string());
                        let target = if opacity() < 0.5 { 1.0f32 } else { 0.2f32 };
                        m_fade.to(opacity, target, Duration::from_millis(300));
                    },
                    "Fade"
                }
                button {
                    id: "btn-from-top",
                    onclick: move |_| {
                        status.set("from_top".to_string());
                        m_drop.from(y, -200.0f32, Duration::from_millis(500))
                            .ease(Ease::BounceOut);
                    },
                    "From Top"
                }
                button {
                    id: "btn-reset",
                    onclick: move |_| {
                        status.set("reset".to_string());
                        m_reset.set(x, 0.0f32);
                        m_reset.set(y, 0.0f32);
                        m_reset.set(scale, 1.0f32);
                        m_reset.set(rot, 0.0f32);
                        m_reset.set(opacity, 1.0f32);
                    },
                    "Reset"
                }
            }
        }
    }
}
