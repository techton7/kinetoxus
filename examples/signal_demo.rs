//! Interactive Phase-1 Dioxus Signal MVP demo.

use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let scale = use_signal(|| 1.0f32);
    let opacity = use_signal(|| 1.0f32);
    let offset_x = use_signal(|| 0.0f32);
    let motion = use_motion();

    let m1 = motion.clone();
    let m2 = motion.clone();
    let m3 = motion.clone();
    let m4 = motion;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; font-family: system-ui, sans-serif; gap: 2rem; background: #0f172a; color: #f8fafc;",
            h1 { "Kinetoxus Phase-1 Signal MVP" }
            p { style: "color: #94a3b8;", "Pure Rust animation driving reactive Dioxus Signal<T> via Web RAF" }

            div {
                id: "animated-box",
                style: "width: 140px; height: 140px; border-radius: 16px; background: linear-gradient(135deg, #6366f1, #ec4899); box-shadow: 0 10px 25px rgba(99, 102, 241, 0.4); transform: translateX({offset_x}px) scale({scale}); opacity: {opacity}; display: flex; align-items: center; justify-content: center; font-weight: bold; font-size: 1.1rem;",
                "Kinetoxus"
            }

            div {
                style: "display: flex; gap: 1rem; flex-wrap: wrap; justify-content: center;",
                button {
                    id: "btn-bounce",
                    style: "padding: 0.75rem 1.5rem; border-radius: 8px; border: none; background: #3b82f6; color: white; font-weight: 600; cursor: pointer;",
                    onclick: move |_| {
                        m1.from_to(scale, 0.6f32, 1.0f32, Duration::from_millis(400))
                            .ease(Ease::BounceOut);
                    },
                    "Bounce Scale"
                }
                button {
                    id: "btn-fade",
                    style: "padding: 0.75rem 1.5rem; border-radius: 8px; border: none; background: #10b981; color: white; font-weight: 600; cursor: pointer;",
                    onclick: move |_| {
                        m2.from_to(opacity, 0.1f32, 1.0f32, Duration::from_millis(300))
                            .ease(Ease::CubicInOut);
                    },
                    "Fade In"
                }
                button {
                    id: "btn-slide",
                    style: "padding: 0.75rem 1.5rem; border-radius: 8px; border: none; background: #8b5cf6; color: white; font-weight: 600; cursor: pointer;",
                    onclick: move |_| {
                        m3.from_to(offset_x, -100.0f32, 100.0f32, Duration::from_millis(500))
                            .ease(Ease::SineInOut)
                            .yoyo(true)
                            .repeat(RepeatCount::Finite(2), RepeatStrategy::MirroredRepeat);
                    },
                    "Slide & Yoyo"
                }
                button {
                    id: "btn-reset",
                    style: "padding: 0.75rem 1.5rem; border-radius: 8px; border: 1px solid #475569; background: #1e293b; color: #cbd5e1; font-weight: 600; cursor: pointer;",
                    onclick: move |_| {
                        m4.set(scale, 1.0f32);
                        m4.set(opacity, 1.0f32);
                        m4.set(offset_x, 0.0f32);
                    },
                    "Immediate Reset (set)"
                }
            }
        }
    }
}
