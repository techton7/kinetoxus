//! Kinetoxus Interruption Lab interactive example.
//!
//! # Runtime & Target Environment
//! Stress tests high-frequency animation interruptions, target retargeting mid-flight,
//! and `use_motion()` dynamic current-value latching (`to` and `from`).
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
    let offset_x = use_signal(|| 0.0f32);
    let scale = use_signal(|| 1.0f32);
    let mut interrupt_count = use_signal(|| 0usize);
    let mut is_settled = use_signal(|| true);
    let mut fps = use_signal(|| 60.0f64);
    let motion = use_motion();

    // High-Level DX: real-time telemetry HUD via use_frame
    let m_hud = motion.clone();
    use_frame(move |info| {
        let dt = info.delta.as_secs_f64();
        if dt > 0.001 {
            fps.set(1.0 / dt);
        }
        let settled = m_hud.active_count() == 0;
        if is_settled() != settled {
            is_settled.set(settled);
        }
    });

    let m_left = motion.clone();
    let m_right = motion.clone();
    let m_rapid = motion.clone();
    let m_from = motion.clone();
    let m_abort = motion;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; gap: 1.75rem; background: #0b0f19; color: #f8fafc; padding: 2rem; box-sizing: border-box;",

            // Header
            div {
                style: "text-align: center;",
                h1 {
                    style: "font-size: 2.25rem; font-weight: 800; background: linear-gradient(to right, #10b981, #06b6d4, #3b82f6); -webkit-background-clip: text; -webkit-text-fill-color: transparent; margin: 0 0 0.5rem 0;",
                    "Kinetoxus Interruption Lab"
                }
                p {
                    style: "color: #94a3b8; font-size: 0.95rem; margin: 0;",
                    "High-frequency motion interruption and dynamic current-value retargeting stress lab"
                }
            }

            // Real-time telemetry HUD
            div {
                id: "telemetry-hud",
                style: "display: flex; gap: 1.25rem; background: rgba(30, 41, 59, 0.7); backdrop-filter: blur(8px); padding: 0.75rem 1.5rem; border-radius: 9999px; border: 1px solid rgba(71, 85, 105, 0.4); font-size: 0.85rem; font-family: monospace; color: #38bdf8; flex-wrap: wrap; justify-content: center; max-width: 800px;",
                span { id: "hud-fps", "fps: {fps:.0}" }
                span { id: "hud-x", "x: {offset_x:.1}px" }
                span { id: "hud-scale", "scale: {scale:.2}" }
                span { id: "hud-interrupts", "interrupts: {interrupt_count}" }
                span { id: "hud-settled", "settled: {is_settled}" }
            }

            // Visual Stage & Runner Box
            div {
                id: "stage",
                style: "width: 440px; height: 180px; display: flex; align-items: center; justify-content: center; border: 1px dashed rgba(71, 85, 105, 0.4); border-radius: 20px; background: rgba(15, 23, 42, 0.4);",

                div {
                    id: "runner-box",
                    style: "width: 120px; height: 120px; border-radius: 20px; background: linear-gradient(135deg, #10b981, #06b6d4); box-shadow: 0 15px 30px -8px rgba(16, 185, 129, 0.4); transform: translateX({offset_x}px) scale({scale}); display: flex; flex-direction: column; align-items: center; justify-content: center; font-weight: 700; font-size: 1rem; color: white; user-select: none; border: 1px solid rgba(255, 255, 255, 0.2);",
                    div { "Runner" }
                    div { style: "font-size: 0.75rem; font-weight: 400; opacity: 0.85; margin-top: 0.2rem;", "{offset_x:.0}px" }
                }
            }

            // Interactive Controls Grid
            div {
                style: "display: flex; gap: 0.75rem; flex-wrap: wrap; justify-content: center; max-width: 680px; margin-top: 0.5rem;",

                button {
                    id: "btn-left",
                    style: "padding: 0.7rem 1.25rem; border-radius: 10px; border: none; background: #3b82f6; color: white; font-weight: 600; font-size: 0.9rem; cursor: pointer;",
                    onclick: move |_| {
                        if !is_settled() {
                            interrupt_count += 1;
                        }
                        is_settled.set(false);
                        m_left.to(offset_x, -140.0f32, Duration::from_millis(500))
                            .ease(Ease::CubicOut);
                    },
                    "⬅️ Move Left"
                }

                button {
                    id: "btn-right",
                    style: "padding: 0.7rem 1.25rem; border-radius: 10px; border: none; background: #06b6d4; color: white; font-weight: 600; font-size: 0.9rem; cursor: pointer;",
                    onclick: move |_| {
                        if !is_settled() {
                            interrupt_count += 1;
                        }
                        is_settled.set(false);
                        m_right.to(offset_x, 140.0f32, Duration::from_millis(500))
                            .ease(Ease::CubicOut);
                    },
                    "➡️ Move Right"
                }

                button {
                    id: "btn-rapid-toggle",
                    style: "padding: 0.7rem 1.25rem; border-radius: 10px; border: none; background: #f59e0b; color: white; font-weight: 600; font-size: 0.9rem; cursor: pointer;",
                    onclick: move |_| {
                        interrupt_count += 1;
                        is_settled.set(false);
                        let target = if offset_x() > 0.0 { -140.0f32 } else { 140.0f32 };
                        m_rapid.to(offset_x, target, Duration::from_millis(500))
                            .ease(Ease::CubicOut);
                    },
                    "⚡ Rapid Toggle"
                }

                button {
                    id: "btn-from",
                    style: "padding: 0.7rem 1.25rem; border-radius: 10px; border: none; background: #8b5cf6; color: white; font-weight: 600; font-size: 0.9rem; cursor: pointer;",
                    onclick: move |_| {
                        if !is_settled() {
                            interrupt_count += 1;
                        }
                        is_settled.set(false);
                        m_from.from(scale, 0.3f32, Duration::from_millis(500))
                            .ease(Ease::BackOut);
                    },
                    "🌟 From (0.3 -> Cur)"
                }

                button {
                    id: "btn-abort",
                    style: "padding: 0.7rem 1.25rem; border-radius: 10px; border: 1px solid #ef4444; background: rgba(239, 68, 68, 0.15); color: #fca5a5; font-weight: 600; font-size: 0.9rem; cursor: pointer;",
                    onclick: move |_| {
                        m_abort.set(offset_x, 0.0f32);
                        m_abort.set(scale, 1.0f32);
                        is_settled.set(true);
                    },
                    "🛑 Abort (set)"
                }
            }
        }
    }
}
