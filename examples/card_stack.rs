//! Kinetoxus Multi-Card Choreography interactive example.
//!
//! # Runtime & Target Environment
//! This demo exercises multi-signal choreography across 3 interactive cards,
//! coordinating `offset_y`, `scale`, and `opacity` per card.
//!
//! # Hosted Native Runtime
//! Uses `#[oxidase::main]` for unified execution across Web and native Blitz/Vello.

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
    // Card 0 signals (Alpha / Back card)
    let c0_y = use_signal(|| -30.0f32);
    let c0_scale = use_signal(|| 0.90f32);
    let c0_opacity = use_signal(|| 0.70f32);

    // Card 1 signals (Beta / Middle card)
    let c1_y = use_signal(|| -15.0f32);
    let c1_scale = use_signal(|| 0.95f32);
    let c1_opacity = use_signal(|| 0.85f32);

    // Card 2 signals (Gamma / Front card)
    let c2_y = use_signal(|| 0.0f32);
    let c2_scale = use_signal(|| 1.00f32);
    let c2_opacity = use_signal(|| 1.00f32);

    // Layout state and telemetry
    let mut layout_state = use_signal(|| "stacked");
    let mut fps = use_signal(|| 60.0f64);
    let motion = use_motion();

    // High-Level DX: real-time telemetry HUD via use_frame
    use_frame(move |info| {
        let dt = info.delta.as_secs_f64();
        if dt > 0.001 {
            fps.set(1.0 / dt);
        }
    });

    let m_expand = motion.clone();
    let m_collapse = motion.clone();
    let m_focus = motion.clone();
    let m_reset = motion;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; gap: 1.75rem; background: #0b0f19; color: #f8fafc; padding: 2rem; box-sizing: border-box;",

            // Header
            div {
                style: "text-align: center;",
                h1 {
                    style: "font-size: 2.25rem; font-weight: 800; background: linear-gradient(to right, #60a5fa, #a855f7, #ec4899); -webkit-background-clip: text; -webkit-text-fill-color: transparent; margin: 0 0 0.5rem 0;",
                    "Kinetoxus Card Choreography"
                }
                p {
                    style: "color: #94a3b8; font-size: 0.95rem; margin: 0;",
                    "Multi-card coordinated choreography driving offset_y, scale, and opacity signals"
                }
            }

            // Real-time telemetry HUD
            div {
                id: "telemetry-hud",
                style: "display: flex; gap: 1rem; background: rgba(30, 41, 59, 0.7); backdrop-filter: blur(8px); padding: 0.75rem 1.5rem; border-radius: 9999px; border: 1px solid rgba(71, 85, 105, 0.4); font-size: 0.85rem; font-family: monospace; color: #38bdf8; flex-wrap: wrap; justify-content: center; max-width: 900px;",
                span { id: "hud-fps", "fps: {fps:.0}" }
                span { id: "hud-c0-y", "c0-y: {c0_y:.1}px" }
                span { id: "hud-c1-y", "c1-y: {c1_y:.1}px" }
                span { id: "hud-c2-y", "c2-y: {c2_y:.1}px" }
                span { id: "hud-c0-scale", "c0-scale: {c0_scale:.2}" }
                span { id: "hud-c1-scale", "c1-scale: {c1_scale:.2}" }
                span { id: "hud-c2-scale", "c2-scale: {c2_scale:.2}" }
                span { id: "hud-state", "state: {layout_state}" }
            }

            // Interactive Card Stage
            div {
                id: "card-stage",
                style: "position: relative; width: 340px; height: 320px; display: flex; align-items: center; justify-content: center;",

                // Card 0 (Alpha)
                div {
                    id: "card-0",
                    style: "position: absolute; width: 300px; height: 110px; border-radius: 16px; background: linear-gradient(135deg, #1e3a8a, #3b82f6); border: 1px solid rgba(255, 255, 255, 0.15); transform: translateY({c0_y}px) scale({c0_scale}); opacity: {c0_opacity}; display: flex; flex-direction: column; justify-content: center; padding: 1.25rem; box-sizing: border-box; color: white; box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.4); user-select: none;",
                    div { style: "font-weight: 700; font-size: 1.1rem;", "Card 0 · Alpha" }
                    div { style: "font-size: 0.8rem; opacity: 0.85; margin-top: 0.25rem;", "Physics Layer · kinetocore" }
                }

                // Card 1 (Beta)
                div {
                    id: "card-1",
                    style: "position: absolute; width: 300px; height: 110px; border-radius: 16px; background: linear-gradient(135deg, #6d28d9, #8b5cf6); border: 1px solid rgba(255, 255, 255, 0.15); transform: translateY({c1_y}px) scale({c1_scale}); opacity: {c1_opacity}; display: flex; flex-direction: column; justify-content: center; padding: 1.25rem; box-sizing: border-box; color: white; box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.4); user-select: none;",
                    div { style: "font-weight: 700; font-size: 1.1rem;", "Card 1 · Beta" }
                    div { style: "font-size: 0.8rem; opacity: 0.85; margin-top: 0.25rem;", "Reactive Signals · kinetoxus" }
                }

                // Card 2 (Gamma)
                div {
                    id: "card-2",
                    style: "position: absolute; width: 300px; height: 110px; border-radius: 16px; background: linear-gradient(135deg, #be185d, #ec4899); border: 1px solid rgba(255, 255, 255, 0.15); transform: translateY({c2_y}px) scale({c2_scale}); opacity: {c2_opacity}; display: flex; flex-direction: column; justify-content: center; padding: 1.25rem; box-sizing: border-box; color: white; box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.4); user-select: none;",
                    div { style: "font-weight: 700; font-size: 1.1rem;", "Card 2 · Gamma" }
                    div { style: "font-size: 0.8rem; opacity: 0.85; margin-top: 0.25rem;", "Render Driver · oxidase" }
                }
            }

            // Interactive Controls
            div {
                style: "display: flex; gap: 0.75rem; flex-wrap: wrap; justify-content: center; max-width: 680px; margin-top: 0.5rem;",

                button {
                    id: "btn-expand",
                    style: "padding: 0.7rem 1.25rem; border-radius: 10px; border: none; background: #3b82f6; color: white; font-weight: 600; font-size: 0.9rem; cursor: pointer;",
                    onclick: move |_| {
                        layout_state.set("expanded");
                        let dur = Duration::from_millis(500);

                        m_expand.to(c0_y, -90.0f32, dur).ease(Ease::CubicOut);
                        m_expand.to(c0_scale, 1.0f32, dur).ease(Ease::CubicOut);
                        m_expand.to(c0_opacity, 1.0f32, dur).ease(Ease::CubicOut);

                        m_expand.to(c1_y, 0.0f32, dur).ease(Ease::CubicOut);
                        m_expand.to(c1_scale, 1.0f32, dur).ease(Ease::CubicOut);
                        m_expand.to(c1_opacity, 1.0f32, dur).ease(Ease::CubicOut);

                        m_expand.to(c2_y, 90.0f32, dur).ease(Ease::CubicOut);
                        m_expand.to(c2_scale, 1.0f32, dur).ease(Ease::CubicOut);
                        m_expand.to(c2_opacity, 1.0f32, dur).ease(Ease::CubicOut);
                    },
                    "📂 Expand"
                }

                button {
                    id: "btn-collapse",
                    style: "padding: 0.7rem 1.25rem; border-radius: 10px; border: none; background: #8b5cf6; color: white; font-weight: 600; font-size: 0.9rem; cursor: pointer;",
                    onclick: move |_| {
                        layout_state.set("stacked");
                        let dur = Duration::from_millis(500);

                        m_collapse.to(c0_y, -30.0f32, dur).ease(Ease::CubicOut);
                        m_collapse.to(c0_scale, 0.90f32, dur).ease(Ease::CubicOut);
                        m_collapse.to(c0_opacity, 0.70f32, dur).ease(Ease::CubicOut);

                        m_collapse.to(c1_y, -15.0f32, dur).ease(Ease::CubicOut);
                        m_collapse.to(c1_scale, 0.95f32, dur).ease(Ease::CubicOut);
                        m_collapse.to(c1_opacity, 0.85f32, dur).ease(Ease::CubicOut);

                        m_collapse.to(c2_y, 0.0f32, dur).ease(Ease::CubicOut);
                        m_collapse.to(c2_scale, 1.00f32, dur).ease(Ease::CubicOut);
                        m_collapse.to(c2_opacity, 1.00f32, dur).ease(Ease::CubicOut);
                    },
                    "📁 Collapse"
                }

                button {
                    id: "btn-focus-1",
                    style: "padding: 0.7rem 1.25rem; border-radius: 10px; border: none; background: #ec4899; color: white; font-weight: 600; font-size: 0.9rem; cursor: pointer;",
                    onclick: move |_| {
                        layout_state.set("card-1-focus");
                        let dur = Duration::from_millis(400);

                        m_focus.to(c0_scale, 0.95f32, dur).ease(Ease::CubicOut);
                        m_focus.to(c0_opacity, 0.60f32, dur).ease(Ease::CubicOut);

                        m_focus.to(c1_scale, 1.05f32, dur).ease(Ease::CubicOut);
                        m_focus.to(c1_opacity, 1.00f32, dur).ease(Ease::CubicOut);

                        m_focus.to(c2_scale, 0.95f32, dur).ease(Ease::CubicOut);
                        m_focus.to(c2_opacity, 0.60f32, dur).ease(Ease::CubicOut);
                    },
                    "🔍 Focus Card 1"
                }

                button {
                    id: "btn-reset",
                    style: "padding: 0.7rem 1.25rem; border-radius: 10px; border: 1px solid #475569; background: #1e293b; color: #cbd5e1; font-weight: 600; font-size: 0.9rem; cursor: pointer;",
                    onclick: move |_| {
                        layout_state.set("stacked");

                        m_reset.set(c0_y, -30.0f32);
                        m_reset.set(c0_scale, 0.90f32);
                        m_reset.set(c0_opacity, 0.70f32);

                        m_reset.set(c1_y, -15.0f32);
                        m_reset.set(c1_scale, 0.95f32);
                        m_reset.set(c1_opacity, 0.85f32);

                        m_reset.set(c2_y, 0.0f32);
                        m_reset.set(c2_scale, 1.00f32);
                        m_reset.set(c2_opacity, 1.00f32);
                    },
                    "⚡ Reset (set)"
                }
            }
        }
    }
}
