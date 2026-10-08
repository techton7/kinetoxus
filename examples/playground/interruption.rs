use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;
use oxidase::prelude::*;

#[component]
pub fn InterruptionSection() -> Element {
    let offset_x = use_signal(|| 0.0f32);
    let scale = use_signal(|| 1.0f32);
    let mut interrupt_count = use_signal(|| 0usize);
    let mut is_settled = use_signal(|| true);

    let motion = use_motion();
    let m_poll = motion.clone();

    use_frame(move |_| {
        let settled = m_poll.active_count() == 0;
        if is_settled() != settled {
            is_settled.set(settled);
        }
    });

    let m_left = motion.clone();
    let m_right = motion.clone();
    let m_jitter = motion.clone();
    let m_from = motion.clone();
    let m_abort = motion;

    rsx! {
        div {
            class: "w-full flex flex-col items-center gap-6",

            div {
                class: "flex flex-wrap items-center justify-center gap-4 px-4 py-2.5 rounded-xl bg-slate-900/90 border border-slate-800 text-xs font-mono text-sky-400",
                span { "offset_x: {offset_x():.1}px" }
                span { "scale: {scale():.2}" }
                span { class: "text-amber-400 font-bold", "interrupts: {interrupt_count()}" }
                span {
                    class: if is_settled() { "text-emerald-400 font-semibold" } else { "text-rose-400 font-semibold animate-pulse" },
                    if is_settled() { "Status: Settled" } else { "Status: In Flight" }
                }
            }

            div {
                class: "w-full h-72 rounded-2xl bg-slate-900/40 border border-slate-800/80 flex items-center justify-center relative overflow-hidden shadow-inner",

                div {
                    class: "w-52 h-32 rounded-2xl bg-gradient-to-r from-rose-500 to-amber-500 shadow-2xl border border-white/20 flex flex-col items-center justify-center select-none text-white",
                    style: "transform: translateX({offset_x()}px) scale({scale()}); transition: none;",
                    span { class: "font-bold text-lg", "Stress Target" }
                    span { class: "text-xs opacity-80 font-mono", "Mid-Flight Retargeting" }
                }
            }

            div {
                class: "flex flex-wrap items-center justify-center gap-2",

                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-blue-600 hover:bg-blue-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        interrupt_count.set(interrupt_count() + 1);
                        m_left.to(offset_x, -160.0f32, Duration::from_millis(800)).ease(Ease::CubicOut);
                        m_left.to(scale, 1.15f32, Duration::from_millis(800)).ease(Ease::CubicOut);
                    },
                    "◀ Fly Left"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        interrupt_count.set(interrupt_count() + 1);
                        m_right.to(offset_x, 160.0f32, Duration::from_millis(800)).ease(Ease::CubicOut);
                        m_right.to(scale, 1.15f32, Duration::from_millis(800)).ease(Ease::CubicOut);
                    },
                    "▶ Fly Right"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-amber-600 hover:bg-amber-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        interrupt_count.set(interrupt_count() + 1);
                        let target = if offset_x() > 0.0 { -120.0f32 } else { 120.0f32 };
                        m_jitter.to(offset_x, target, Duration::from_millis(250)).ease(Ease::QuadInOut);
                    },
                    "⚡ Rapid Jitter"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        interrupt_count.set(interrupt_count() + 1);
                        m_from.from(offset_x, -200.0f32, Duration::from_millis(700)).ease(Ease::ElasticOut);
                    },
                    "↺ From -200px"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-rose-700 hover:bg-rose-600 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        m_abort.cancel_all();
                        m_abort.set(offset_x, 0.0f32);
                        m_abort.set(scale, 1.0f32);
                    },
                    "🛑 Emergency Abort"
                }
            }
        }
    }
}
