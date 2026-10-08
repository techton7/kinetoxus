use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;

#[component]
pub fn TweensSection() -> Element {
    let x = use_signal(|| 0.0f32);
    let y = use_signal(|| 0.0f32);
    let scale = use_signal(|| 1.0f32);
    let rot = use_signal(|| 0.0f32);
    let opacity = use_signal(|| 1.0f32);

    let motion = use_motion();

    let m_pop = motion.clone();
    let m_slide = motion.clone();
    let m_spin = motion.clone();
    let m_bounce = motion.clone();
    let m_elastic = motion.clone();
    let m_fade = motion.clone();
    let m_center = motion.clone();
    let m_reset = motion;

    rsx! {
        div {
            class: "w-full flex flex-col items-center gap-6",

            div {
                class: "flex flex-wrap items-center justify-center gap-4 px-4 py-2.5 rounded-xl bg-slate-900/90 border border-slate-800 text-xs font-mono text-sky-400",
                span { "x: {x():.1}px" }
                span { "y: {y():.1}px" }
                span { "scale: {scale():.2}" }
                span { "rot: {rot():.1}°" }
                span { "opacity: {opacity():.2}" }
            }

            div {
                class: "w-full h-72 rounded-2xl bg-slate-900/40 border border-slate-800/80 flex items-center justify-center relative overflow-hidden shadow-inner",

                div {
                    class: "w-56 h-36 rounded-2xl bg-gradient-to-br from-blue-500 via-indigo-600 to-violet-700 shadow-2xl border border-white/20 flex flex-col items-center justify-center select-none text-white",
                    style: "transform: translate({x()}px, {y()}px) scale({scale()}) rotate({rot()}deg); opacity: {opacity()}; transition: none;",
                    span { class: "font-bold text-lg", "Tween Target" }
                    span { class: "text-xs opacity-80 font-mono", "Comprehensive Easings" }
                }
            }

            div {
                class: "flex flex-wrap items-center justify-center gap-2",

                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        m_pop.to(scale, 1.3f32, Duration::from_millis(300)).ease(Ease::BackOut);
                    },
                    "💥 Pop"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-cyan-600 hover:bg-cyan-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        m_slide.from_to(x, -150.0f32, 150.0f32, Duration::from_millis(700)).ease(Ease::CubicInOut);
                    },
                    "↔ Slide"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-purple-600 hover:bg-purple-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        m_spin.from_to(rot, 0.0f32, 360.0f32, Duration::from_millis(600)).ease(Ease::QuadInOut);
                    },
                    "💫 Spin"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-amber-600 hover:bg-amber-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        m_bounce.from(y, -160.0f32, Duration::from_millis(700)).ease(Ease::BounceOut);
                    },
                    "🏀 Bounce"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-teal-600 hover:bg-teal-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        m_elastic.from_to(scale, 0.5f32, 1.2f32, Duration::from_millis(800)).ease(Ease::ElasticOut);
                    },
                    "🪃 Elastic"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        let target = if opacity() < 0.5 { 1.0f32 } else { 0.3f32 };
                        m_fade.to(opacity, target, Duration::from_millis(350)).ease(Ease::Linear);
                    },
                    "🌓 Fade"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-blue-600 hover:bg-blue-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        m_center.to(x, 0.0f32, Duration::from_millis(400)).ease(Ease::CubicOut);
                        m_center.to(y, 0.0f32, Duration::from_millis(400)).ease(Ease::CubicOut);
                        m_center.to(scale, 1.0f32, Duration::from_millis(400)).ease(Ease::CubicOut);
                    },
                    "🎯 Center"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        m_reset.set(x, 0.0f32);
                        m_reset.set(y, 0.0f32);
                        m_reset.set(scale, 1.0f32);
                        m_reset.set(rot, 0.0f32);
                        m_reset.set(opacity, 1.0f32);
                    },
                    "↺ Reset"
                }
            }
        }
    }
}
