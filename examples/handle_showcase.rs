//! Kinetoxus Handle Showcase Example (Subtask T-2.4).
//! Demonstrates zero-signal direct handle animation (`Transform2D`) with interactive buttons
//! (`#btn-pop`, `#btn-slide`, `#btn-to-center`, `#btn-reset`) and telemetry HUD (`#handle-card`, `#hud-fps`, `#hud-x`, `#hud-y`, `#hud-scale`, `#hud-rot`).

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
    // Mount Transform2D inside component state via use_hook
    let transform = use_hook(|| Transform2D::new(0.0, 0.0, 1.0, 0.0));
    
    // UI state for reactive HUD display & re-render triggers on frame tick
    let mut fps = use_signal(|| 60.0f64);
    let mut status = use_signal(|| "idle".to_string());
    let mut render_count = use_signal(|| 0_u32);

    let motion = use_motion();

    // Real-time telemetry HUD via use_frame
    use_frame(move |info| {
        let dt = info.delta.as_secs_f64();
        if dt > 0.001 {
            fps.set(1.0 / dt);
        }
        // Increment render_count to trigger component view update sampling latest handle values
        render_count.set(render_count() + 1);
    });

    let t_pop = transform.clone();
    let t_slide = transform.clone();
    let t_center = transform.clone();
    let t_reset = transform.clone();

    let m_pop = motion.clone();
    let m_slide = motion.clone();
    let m_center = motion.clone();
    let m_reset = motion;

    let x_val = *transform.x.borrow();
    let y_val = *transform.y.borrow();
    let scale_val = *transform.scale.borrow();
    let rot_val = *transform.rotation.borrow();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; gap: 1.75rem; background: #0f172a; color: #f8fafc; padding: 2rem; box-sizing: border-box;",

            // Telemetry HUD
            div {
                id: "telemetry-hud",
                style: "display: flex; gap: 1rem; background: rgba(30, 41, 59, 0.9); padding: 0.75rem 1.25rem; border-radius: 8px; font-family: monospace; font-size: 0.85rem; color: #38bdf8; border: 1px solid #334155;",
                span { id: "hud-fps", "fps: {fps():.0}" }
                span { id: "hud-x", "x: {x_val:.1}px" }
                span { id: "hud-y", "y: {y_val:.1}px" }
                span { id: "hud-scale", "scale: {scale_val:.2}" }
                span { id: "hud-rot", "rot: {rot_val:.1}°" }
                span { id: "hud-status", "status: {status()}" }
            }

            // Handle Card (Directly driven by raw non-signal handle values)
            div {
                id: "handle-card",
                style: "width: 140px; height: 140px; background: linear-gradient(135deg, #6366f1, #a855f7); border-radius: 16px; transform: translate({x_val}px, {y_val}px) scale({scale_val}) rotate({rot_val}deg); display: flex; flex-direction: column; align-items: center; justify-content: center; font-weight: bold; color: white; box-shadow: 0 10px 25px -5px rgba(99, 102, 241, 0.4);",
                span { "Handle" }
                span { style: "font-size: 0.7rem; opacity: 0.8;", "Zero-Signal" }
            }

            // Action Buttons
            div {
                style: "display: flex; gap: 0.75rem; flex-wrap: wrap; justify-content: center;",
                button {
                    id: "btn-pop",
                    style: "background: #3b82f6; color: white; border: none; padding: 0.6rem 1.2rem; border-radius: 6px; font-weight: 600; cursor: pointer;",
                    onclick: move |_| {
                        status.set("pop".to_string());
                        m_pop.from_to(std::rc::Rc::clone(&t_pop.scale), 0.6f32, 1.2f32, Duration::from_millis(300))
                            .ease(Ease::BackOut);
                    },
                    "Pop Scale"
                }
                button {
                    id: "btn-slide",
                    style: "background: #8b5cf6; color: white; border: none; padding: 0.6rem 1.2rem; border-radius: 6px; font-weight: 600; cursor: pointer;",
                    onclick: move |_| {
                        status.set("slide".to_string());
                        m_slide.from_to(std::rc::Rc::clone(&t_slide.x), -120.0f32, 120.0f32, Duration::from_millis(600))
                            .ease(Ease::QuadInOut);
                        m_slide.from_to(std::rc::Rc::clone(&t_slide.rotation), 0.0f32, 180.0f32, Duration::from_millis(600))
                            .ease(Ease::QuadInOut);
                    },
                    "Slide & Rotate"
                }
                button {
                    id: "btn-to-center",
                    style: "background: #ec4899; color: white; border: none; padding: 0.6rem 1.2rem; border-radius: 6px; font-weight: 600; cursor: pointer;",
                    onclick: move |_| {
                        status.set("to_center".to_string());
                        m_center.to(std::rc::Rc::clone(&t_center.x), 0.0f32, Duration::from_millis(400)).ease(Ease::QuadOut);
                        m_center.to(std::rc::Rc::clone(&t_center.y), 0.0f32, Duration::from_millis(400)).ease(Ease::QuadOut);
                        m_center.to(std::rc::Rc::clone(&t_center.scale), 1.0f32, Duration::from_millis(400)).ease(Ease::QuadOut);
                        m_center.to(std::rc::Rc::clone(&t_center.rotation), 0.0f32, Duration::from_millis(400)).ease(Ease::QuadOut);
                    },
                    "To Center"
                }
                button {
                    id: "btn-reset",
                    style: "background: #64748b; color: white; border: none; padding: 0.6rem 1.2rem; border-radius: 6px; font-weight: 600; cursor: pointer;",
                    onclick: move |_| {
                        status.set("reset".to_string());
                        m_reset.set(std::rc::Rc::clone(&t_reset.x), 0.0f32);
                        m_reset.set(std::rc::Rc::clone(&t_reset.y), 0.0f32);
                        m_reset.set(std::rc::Rc::clone(&t_reset.scale), 1.0f32);
                        m_reset.set(std::rc::Rc::clone(&t_reset.rotation), 0.0f32);
                    },
                    "Reset"
                }
            }
        }
    }
}
