use dioxus::prelude::*;
use kinetoxus::prelude::*;

#[component]
pub fn SpringSection() -> Element {
    let x = use_signal(|| 0.0f64);
    let y = use_signal(|| 0.0f64);
    let scale = use_signal(|| 1.0f64);
    let mut preset = use_signal(|| "default".to_string());
    let mut status = use_signal(|| "idle".to_string());

    let motion = use_motion();

    let m_bouncy = motion.clone();
    let m_stiff = motion.clone();
    let m_gentle = motion.clone();
    let m_center = motion.clone();
    let m_reset = motion;

    rsx! {
        div {
            class: "w-full flex flex-col items-center gap-6",

            // Telemetry HUD
            div {
                class: "flex flex-wrap items-center justify-center gap-4 px-4 py-2.5 rounded-xl bg-slate-900/90 border border-slate-800 text-xs font-mono text-sky-400 shadow-lg",
                span { "x: {x():.1}px" }
                span { "y: {y():.1}px" }
                span { "scale: {scale():.2}" }
                span { class: "text-indigo-400", "preset: {preset()}" }
                span { class: "text-amber-400 font-semibold", "status: {status()}" }
            }

            // Canvas Area
            div {
                class: "w-full h-72 rounded-2xl bg-slate-900/40 border border-slate-800/80 flex items-center justify-center relative overflow-hidden shadow-inner",

                div {
                    class: "w-56 h-36 rounded-2xl bg-gradient-to-br from-cyan-500 to-blue-600 shadow-2xl border border-white/20 bg-no-repeat flex flex-col items-center justify-center select-none text-white",
                    style: "transform: translate({x()}px, {y()}px) scale({scale()}); transition: none;",
                    span { class: "font-bold text-lg", "Spring Target" }
                    span { class: "text-xs opacity-80 font-mono", "kinetocore::spring" }
                }
            }

            // Controls
            div {
                class: "flex flex-wrap items-center justify-center gap-2",

                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        preset.set("Bouncy".to_string());
                        status.set("animating".to_string());
                        m_bouncy.spring(x, 120.0, SpringConfig::BOUNCY);
                        m_bouncy.spring(y, -40.0, SpringConfig::BOUNCY);
                        m_bouncy.spring(scale, 1.25, SpringConfig::BOUNCY);
                    },
                    "🌀 Bouncy (Underdamped)"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-sky-600 hover:bg-sky-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        preset.set("Stiff".to_string());
                        status.set("animating".to_string());
                        m_stiff.spring(x, -120.0, SpringConfig::STIFF);
                        m_stiff.spring(y, 40.0, SpringConfig::STIFF);
                        m_stiff.spring(scale, 0.85, SpringConfig::STIFF);
                    },
                    "⚡ Stiff (Fast)"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-teal-600 hover:bg-teal-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        preset.set("Gentle".to_string());
                        status.set("animating".to_string());
                        m_gentle.spring(x, 100.0, SpringConfig::GENTLE);
                        m_gentle.spring(y, 30.0, SpringConfig::GENTLE);
                        m_gentle.spring(scale, 1.1, SpringConfig::GENTLE);
                    },
                    "🍃 Gentle (Overdamped)"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-purple-600 hover:bg-purple-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        preset.set("Mid-flight Retarget".to_string());
                        status.set("retargeting".to_string());
                        let critical = SpringConfig::new(1.0, 100.0, 20.0).unwrap_or(SpringConfig::DEFAULT);
                        m_center.spring(x, 0.0, critical);
                        m_center.spring(y, 0.0, critical);
                        m_center.spring(scale, 1.0, critical);
                    },
                    "🎯 Center (C¹ Retarget)"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        preset.set("default".to_string());
                        status.set("reset".to_string());
                        m_reset.set(x, 0.0);
                        m_reset.set(y, 0.0);
                        m_reset.set(scale, 1.0);
                    },
                    "↺ Reset"
                }
            }
        }
    }
}
