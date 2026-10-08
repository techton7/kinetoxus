use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;

#[component]
pub fn CardStackSection() -> Element {
    let c0_y = use_signal(|| -30.0f32);
    let c0_scale = use_signal(|| 0.90f32);
    let c0_opacity = use_signal(|| 0.70f32);

    let c1_y = use_signal(|| -15.0f32);
    let c1_scale = use_signal(|| 0.95f32);
    let c1_opacity = use_signal(|| 0.85f32);

    let c2_y = use_signal(|| 0.0f32);
    let c2_scale = use_signal(|| 1.00f32);
    let c2_opacity = use_signal(|| 1.00f32);

    let mut state = use_signal(|| "stacked".to_string());
    let motion = use_motion();

    let m_expand = motion.clone();
    let m_collapse = motion.clone();
    let m_focus = motion.clone();
    let m_reset = motion;

    rsx! {
        div {
            class: "w-full flex flex-col items-center gap-6",

            div {
                class: "px-4 py-2 rounded-xl bg-slate-900 border border-slate-800 text-xs font-mono text-sky-400",
                "State: {state()} • Coordinated 9-signal 3D depth choreography"
            }

            // Stacking Area
            div {
                class: "w-full h-80 rounded-2xl bg-slate-900/40 border border-slate-800/80 flex items-center justify-center relative overflow-hidden shadow-inner",

                // Card 0 (Back / Alpha)
                div {
                    class: "w-64 h-36 rounded-2xl bg-gradient-to-r from-emerald-600 to-teal-700 shadow-2xl border border-white/10 flex flex-col items-center justify-center absolute text-white bg-no-repeat",
                    style: "transform: translateY({c0_y()}px) scale({c0_scale()}); opacity: {c0_opacity()}; z-index: 10; transition: none;",
                    span { class: "font-bold text-base", "Card Alpha" }
                    span { class: "text-xs opacity-75 font-mono", "Background Layer" }
                }

                // Card 1 (Middle / Beta)
                div {
                    class: "w-64 h-36 rounded-2xl bg-gradient-to-r from-blue-600 to-indigo-700 shadow-2xl border border-white/10 flex flex-col items-center justify-center absolute text-white bg-no-repeat",
                    style: "transform: translateY({c1_y()}px) scale({c1_scale()}); opacity: {c1_opacity()}; z-index: 20; transition: none;",
                    span { class: "font-bold text-base", "Card Beta" }
                    span { class: "text-xs opacity-75 font-mono", "Middle Layer" }
                }

                // Card 2 (Front / Gamma)
                div {
                    class: "w-64 h-36 rounded-2xl bg-gradient-to-r from-purple-600 to-pink-600 shadow-2xl border border-white/20 flex flex-col items-center justify-center absolute text-white bg-no-repeat",
                    style: "transform: translateY({c2_y()}px) scale({c2_scale()}); opacity: {c2_opacity()}; z-index: 30; transition: none;",
                    span { class: "font-bold text-base", "Card Gamma" }
                    span { class: "text-xs opacity-75 font-mono", "Foreground Layer" }
                }
            }

            // Controls
            div {
                class: "flex flex-wrap items-center justify-center gap-2",

                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        state.set("expanded".to_string());
                        m_expand.to(c0_y, -90.0f32, Duration::from_millis(450)).ease(Ease::CubicOut);
                        m_expand.to(c0_scale, 1.0f32, Duration::from_millis(450)).ease(Ease::CubicOut);
                        m_expand.to(c0_opacity, 1.0f32, Duration::from_millis(450)).ease(Ease::CubicOut);

                        m_expand.to(c1_y, 0.0f32, Duration::from_millis(450)).ease(Ease::CubicOut);
                        m_expand.to(c1_scale, 1.0f32, Duration::from_millis(450)).ease(Ease::CubicOut);
                        m_expand.to(c1_opacity, 1.0f32, Duration::from_millis(450)).ease(Ease::CubicOut);

                        m_expand.to(c2_y, 90.0f32, Duration::from_millis(450)).ease(Ease::CubicOut);
                        m_expand.to(c2_scale, 1.0f32, Duration::from_millis(450)).ease(Ease::CubicOut);
                        m_expand.to(c2_opacity, 1.0f32, Duration::from_millis(450)).ease(Ease::CubicOut);
                    },
                    "↕ Expand Stack"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        state.set("stacked".to_string());
                        m_collapse.to(c0_y, -30.0f32, Duration::from_millis(450)).ease(Ease::BackOut);
                        m_collapse.to(c0_scale, 0.90f32, Duration::from_millis(450)).ease(Ease::BackOut);
                        m_collapse.to(c0_opacity, 0.70f32, Duration::from_millis(450)).ease(Ease::BackOut);

                        m_collapse.to(c1_y, -15.0f32, Duration::from_millis(450)).ease(Ease::BackOut);
                        m_collapse.to(c1_scale, 0.95f32, Duration::from_millis(450)).ease(Ease::BackOut);
                        m_collapse.to(c1_opacity, 0.85f32, Duration::from_millis(450)).ease(Ease::BackOut);

                        m_collapse.to(c2_y, 0.0f32, Duration::from_millis(450)).ease(Ease::BackOut);
                        m_collapse.to(c2_scale, 1.00f32, Duration::from_millis(450)).ease(Ease::BackOut);
                        m_collapse.to(c2_opacity, 1.00f32, Duration::from_millis(450)).ease(Ease::BackOut);
                    },
                    "⇊ Collapse"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-purple-600 hover:bg-purple-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        state.set("focus-middle".to_string());
                        m_focus.to(c1_y, 0.0f32, Duration::from_millis(400)).ease(Ease::CubicOut);
                        m_focus.to(c1_scale, 1.08f32, Duration::from_millis(400)).ease(Ease::BackOut);
                        m_focus.to(c1_opacity, 1.0f32, Duration::from_millis(400)).ease(Ease::CubicOut);

                        m_focus.to(c0_y, -60.0f32, Duration::from_millis(400)).ease(Ease::CubicOut);
                        m_focus.to(c0_opacity, 0.5f32, Duration::from_millis(400)).ease(Ease::CubicOut);

                        m_focus.to(c2_y, 60.0f32, Duration::from_millis(400)).ease(Ease::CubicOut);
                        m_focus.to(c2_opacity, 0.5f32, Duration::from_millis(400)).ease(Ease::CubicOut);
                    },
                    "🎯 Focus Middle (Beta)"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        state.set("reset".to_string());
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
                    "↺ Reset"
                }
            }
        }
    }
}
