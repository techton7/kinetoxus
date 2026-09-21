//! Kinetoxus Motion Lab interactive example.
//!
//! # Runtime & Target Environment
//! This demo is designed for interactive UI exploration across standard Dioxus platforms
//! (e.g. `dx serve` on Web or standard Dioxus desktop). It directly exercises `oxidase`
//! high-level consumer DX (`use_frame` and `next_frame().await`) for reactive physics and
//! real-time telemetry HUD updates.
//!
//! # Native Zero-Wiring Boundary
//! This demo launches via standard `dioxus::launch(App)` and does NOT depend on Blitz
//! or `dioxus-native`. The sovereign native zero-wiring hosted VSync bootstrap story
//! (`#[oxidase::main]`) is exclusively owned, verified, and runtime-proven by the dedicated
//! runner `crates/oxidase-native-runner`.

use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;
use oxidase::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let scale = use_signal(|| 1.0f32);
    let opacity = use_signal(|| 1.0f32);
    let offset_x = use_signal(|| 0.0f32);
    let rotation = use_signal(|| 0.0f32);
    let mut fps = use_signal(|| 60.0f64);
    let motion = use_motion();

    // High-Level DX 1: use_frame for real-time telemetry HUD
    use_frame(move |info| {
        let dt = info.delta.as_secs_f64();
        if dt > 0.001 {
            fps.set(1.0 / dt);
        }
    });

    // High-Level DX 2: next_frame().await for initialization log
    use_future(move || async move {
        let first_frame = next_frame().await;
        println!("[signal_demo] First VSync frame arrived: now={:?}, delta={:?}", first_frame.now, first_frame.delta);
    });

    let m_bounce = motion.clone();
    let m_elastic = motion.clone();
    let m_fade = motion.clone();
    let m_slide = motion.clone();
    let m_spin = motion.clone();
    let m_reset = motion;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; gap: 1.75rem; background: #0b0f19; color: #f8fafc; padding: 2rem;",

            div {
                style: "text-align: center;",
                h1 {
                    style: "font-size: 2.25rem; font-weight: 800; background: linear-gradient(to right, #60a5fa, #a855f7, #ec4899); -webkit-background-clip: text; -webkit-text-fill-color: transparent; margin: 0 0 0.5rem 0;",
                    "Kinetoxus Motion Lab"
                }
                p {
                    style: "color: #94a3b8; font-size: 0.95rem; margin: 0;",
                    "Pure Rust kinetics (kinetocore) driving reactive Dioxus signals via oxidase::frame"
                }
            }

            // Real-time telemetry HUD
            div {
                style: "display: flex; gap: 1.5rem; background: rgba(30, 41, 59, 0.7); backdrop-filter: blur(8px); padding: 0.75rem 1.5rem; border-radius: 9999px; border: 1px solid rgba(71, 85, 105, 0.4); font-size: 0.85rem; font-family: monospace; color: #38bdf8;",
                span { "fps: {fps:.0}" }
                span { "scale: {scale:.2}" }
                span { "opacity: {opacity:.2}" }
                span { "x: {offset_x:.1}px" }
                span { "rot: {rotation:.1}°" }
            }

            // Animated Visual Box
            div {
                id: "animated-box",
                style: "width: 150px; height: 150px; border-radius: 24px; background: linear-gradient(135deg, #6366f1, #d946ef); box-shadow: 0 20px 35px -10px rgba(99, 102, 241, 0.5); transform: translateX({offset_x}px) scale({scale}) rotate({rotation}deg); opacity: {opacity}; display: flex; flex-direction: column; align-items: center; justify-content: center; font-weight: 700; font-size: 1.15rem; color: white; user-select: none; border: 1px solid rgba(255, 255, 255, 0.2);",
                div { "kinetoxus" }
                div { style: "font-size: 0.75rem; font-weight: 400; opacity: 0.8;", "0.1.0" }
            }

            // Interactive Controls Grid
            div {
                style: "display: flex; gap: 0.75rem; flex-wrap: wrap; justify-content: center; max-width: 680px; margin-top: 0.5rem;",
                button {
                    id: "btn-bounce",
                    style: "padding: 0.7rem 1.25rem; border-radius: 10px; border: none; background: #3b82f6; color: white; font-weight: 600; font-size: 0.9rem; cursor: pointer; transition: filter 0.15s;",
                    onclick: move |_| {
                        m_bounce.from_to(scale, 0.5f32, 1.0f32, Duration::from_millis(500))
                            .ease(Ease::BounceOut);
                    },
                    "🏀 BounceOut"
                }
                button {
                    id: "btn-elastic",
                    style: "padding: 0.7rem 1.25rem; border-radius: 10px; border: none; background: #06b6d4; color: white; font-weight: 600; font-size: 0.9rem; cursor: pointer;",
                    onclick: move |_| {
                        m_elastic.from_to(scale, 0.4f32, 1.0f32, Duration::from_millis(800))
                            .ease(Ease::ElasticOut);
                    },
                    "🎯 ElasticOut"
                }
                button {
                    id: "btn-fade",
                    style: "padding: 0.7rem 1.25rem; border-radius: 10px; border: none; background: #10b981; color: white; font-weight: 600; font-size: 0.9rem; cursor: pointer;",
                    onclick: move |_| {
                        m_fade.from_to(opacity, 0.05f32, 1.0f32, Duration::from_millis(400))
                            .ease(Ease::CubicInOut);
                    },
                    "✨ Fade In"
                }
                button {
                    id: "btn-slide",
                    style: "padding: 0.7rem 1.25rem; border-radius: 10px; border: none; background: #8b5cf6; color: white; font-weight: 600; font-size: 0.9rem; cursor: pointer;",
                    onclick: move |_| {
                        m_slide.from_to(offset_x, -120.0f32, 120.0f32, Duration::from_millis(600))
                            .ease(Ease::SineInOut)
                            .yoyo(true)
                            .repeat(RepeatCount::Finite(2), RepeatStrategy::MirroredRepeat);
                    },
                    "↔️ Slide & Yoyo"
                }
                button {
                    id: "btn-spin",
                    style: "padding: 0.7rem 1.25rem; border-radius: 10px; border: none; background: #ec4899; color: white; font-weight: 600; font-size: 0.9rem; cursor: pointer;",
                    onclick: move |_| {
                        m_spin.from_to(rotation, 0.0f32, 360.0f32, Duration::from_millis(700))
                            .ease(Ease::BackOut);
                    },
                    "💫 Spin (BackOut)"
                }
                button {
                    id: "btn-reset",
                    style: "padding: 0.7rem 1.25rem; border-radius: 10px; border: 1px solid #475569; background: #1e293b; color: #cbd5e1; font-weight: 600; font-size: 0.9rem; cursor: pointer;",
                    onclick: move |_| {
                        m_reset.set(scale, 1.0f32);
                        m_reset.set(opacity, 1.0f32);
                        m_reset.set(offset_x, 0.0f32);
                        m_reset.set(rotation, 0.0f32);
                    },
                    "⚡ Reset (set)"
                }
            }
        }
    }
}
