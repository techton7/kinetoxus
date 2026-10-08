use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;
use oxidase::prelude::*;

#[component]
pub fn HandlesSection() -> Element {
    let transform = use_hook(|| Transform2D::new(0.0, 0.0, 1.0, 0.0));
    let mut render_count = use_signal(|| 0_u32);

    let motion = use_motion();

    use_frame(move |_| {
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
            class: "w-full flex flex-col items-center gap-6",

            div {
                class: "flex flex-wrap items-center justify-center gap-4 px-4 py-2.5 rounded-xl bg-slate-900/90 border border-slate-800 text-xs font-mono text-sky-400",
                span { "x: {x_val:.1}px" }
                span { "y: {y_val:.1}px" }
                span { "scale: {scale_val:.2}" }
                span { "rot: {rot_val:.1}°" }
                span { class: "text-emerald-400 font-semibold", "Direct Memory (0 Signals)" }
            }

            div {
                class: "w-full h-72 rounded-2xl bg-slate-900/40 border border-slate-800/80 flex items-center justify-center relative overflow-hidden shadow-inner",

                div {
                    class: "w-56 h-36 rounded-2xl bg-gradient-to-br from-fuchsia-600 to-indigo-700 shadow-2xl border border-white/20 flex flex-col items-center justify-center select-none text-white",
                    style: "transform: translate({x_val}px, {y_val}px) scale({scale_val}) rotate({rot_val}deg); transition: none;",
                    span { class: "font-bold text-lg", "Handle Target" }
                    span { class: "text-xs opacity-80 font-mono", "Transform2D Direct Mut" }
                }
            }

            div {
                class: "flex flex-wrap items-center justify-center gap-2",

                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        m_pop.to(t_pop.scale.clone(), 1.35f32, Duration::from_millis(400)).ease(Ease::BackOut);
                    },
                    "💥 Pop Scale"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-fuchsia-600 hover:bg-fuchsia-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        m_slide.from_to(t_slide.x.clone(), -140.0f32, 140.0f32, Duration::from_millis(700)).ease(Ease::CubicInOut);
                    },
                    "↔ Slide Range"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-sky-600 hover:bg-sky-500 text-white shadow-md active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        m_center.to(t_center.x.clone(), 0.0f32, Duration::from_millis(400)).ease(Ease::CubicOut);
                        m_center.to(t_center.y.clone(), 0.0f32, Duration::from_millis(400)).ease(Ease::CubicOut);
                        m_center.to(t_center.scale.clone(), 1.0f32, Duration::from_millis(400)).ease(Ease::CubicOut);
                    },
                    "🎯 To Center"
                }
                button {
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        m_reset.set(t_reset.x.clone(), 0.0f32);
                        m_reset.set(t_reset.y.clone(), 0.0f32);
                        m_reset.set(t_reset.scale.clone(), 1.0f32);
                        m_reset.set(t_reset.rotation.clone(), 0.0f32);
                    },
                    "↺ Reset"
                }
            }
        }
    }
}
