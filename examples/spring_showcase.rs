//! Kinetoxus Spring Showcase Example.
//! Demonstrates analytical spring motion across underdamped, stiff, and gentle presets with mid-flight interruption retargeting.

use dioxus::prelude::*;
use kinetoxus::prelude::*;
use oxidase::prelude::*;

#[oxidase::main]
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let x = use_signal(|| 0.0f64);
    let y = use_signal(|| 0.0f64);
    let scale = use_signal(|| 1.0f64);
    let mut fps = use_signal(|| 60.0f64);
    let mut status = use_signal(|| "idle".to_string());
    let mut preset_name = use_signal(|| "default".to_string());

    let motion = use_motion();

    use_frame(move |info| {
        let dt = info.delta.as_secs_f64();
        if dt > 0.001 {
            fps.set(1.0 / dt);
        }
    });

    let m_bouncy = motion.clone();
    let m_stiff = motion.clone();
    let m_gentle = motion.clone();
    let m_center = motion.clone();
    let m_reset = motion;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; gap: 1.5rem; background: #0b0f19; color: #f8fafc; padding: 2rem;",

            // Telemetry HUD
            div {
                id: "telemetry-hud",
                style: "display: flex; gap: 1rem; background: rgba(30, 41, 59, 0.85); padding: 0.6rem 1.2rem; border-radius: 8px; font-family: monospace; font-size: 0.9rem; color: #38bdf8; border: 1px solid rgba(56, 189, 248, 0.2);",
                span { id: "hud-fps", "fps: {fps():.0}" }
                span { id: "hud-x", "x: {x():.1}px" }
                span { id: "hud-y", "y: {y():.1}px" }
                span { id: "hud-scale", "scale: {scale():.2}" }
                span { id: "hud-preset", "preset: {preset_name()}" }
                span { id: "hud-status", "status: {status()}" }
            }

            // Animated Spring Card
            div {
                id: "animated-card",
                style: "width: 220px; height: 140px; background: linear-gradient(135deg, #6366f1 0%, #a855f7 100%); border-radius: 16px; display: flex; flex-direction: column; align-items: center; justify-content: center; box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5); transform: translate({x()}px, {y()}px) scale({scale()}); transition: none; border: 2px solid rgba(255, 255, 255, 0.1);",
                span { style: "font-weight: 700; font-size: 1.25rem;", "Spring Motion" }
                span { style: "font-size: 0.85rem; opacity: 0.8;", "kinetocore::spring" }
            }

            // Interactive Controls
            div {
                style: "display: flex; flex-wrap: wrap; gap: 0.75rem; justify-content: center; max-width: 600px;",
                button {
                    id: "btn-bouncy",
                    style: "padding: 0.6rem 1.2rem; background: #3b82f6; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600; font-size: 0.875rem;",
                    onclick: move |_| {
                        status.set("bouncy".to_string());
                        preset_name.set("bouncy".to_string());
                        m_bouncy.spring(x, 150.0, SpringConfig::BOUNCY);
                        m_bouncy.spring(scale, 1.2, SpringConfig::BOUNCY);
                    },
                    "Bouncy Spring"
                }
                button {
                    id: "btn-stiff",
                    style: "padding: 0.6rem 1.2rem; background: #8b5cf6; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600; font-size: 0.875rem;",
                    onclick: move |_| {
                        status.set("stiff".to_string());
                        preset_name.set("stiff".to_string());
                        m_stiff.spring(x, -150.0, SpringConfig::STIFF);
                        m_stiff.spring(scale, 0.9, SpringConfig::STIFF);
                    },
                    "Stiff Spring"
                }
                button {
                    id: "btn-gentle",
                    style: "padding: 0.6rem 1.2rem; background: #10b981; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600; font-size: 0.875rem;",
                    onclick: move |_| {
                        status.set("gentle".to_string());
                        preset_name.set("gentle".to_string());
                        m_gentle.spring(y, -60.0, SpringConfig::GENTLE);
                        m_gentle.spring(scale, 1.05, SpringConfig::GENTLE);
                    },
                    "Gentle Spring"
                }
                button {
                    id: "btn-to-center",
                    style: "padding: 0.6rem 1.2rem; background: #f59e0b; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600; font-size: 0.875rem;",
                    onclick: move |_| {
                        status.set("interrupted_to_center".to_string());
                        preset_name.set("critically_damped".to_string());
                        let critical = SpringConfig::new(1.0, 100.0, 20.0).unwrap_or(SpringConfig::DEFAULT);
                        m_center.spring(x, 0.0, critical);
                        m_center.spring(y, 0.0, critical);
                        m_center.spring(scale, 1.0, critical);
                    },
                    "To Center (Interrupt)"
                }
                button {
                    id: "btn-reset",
                    style: "padding: 0.6rem 1.2rem; background: #ef4444; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600; font-size: 0.875rem;",
                    onclick: move |_| {
                        status.set("reset".to_string());
                        preset_name.set("none".to_string());
                        m_reset.set(x, 0.0);
                        m_reset.set(y, 0.0);
                        m_reset.set(scale, 1.0);
                    },
                    "Reset (set)"
                }
            }
        }
    }
}
