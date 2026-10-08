use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;
use oxidase::prelude::*;

#[component]
pub fn TimelineSection() -> Element {
    let x = use_signal(|| 0.0f64);
    let y = use_signal(|| 0.0f64);
    let scale = use_signal(|| 1.0f64);
    let opacity = use_signal(|| 1.0f64);
    let rotation = use_signal(|| 0.0f64);
    let mut progress = use_signal(|| 0.0f32);
    let mut time_sec = use_signal(|| 0.0f64);
    let mut status = use_signal(|| "Paused".to_string());

    let tl = use_timeline(|b| {
        b.autoplay(false);
        b.add_label("intro", Duration::ZERO);
        b.add_label("accent", Duration::from_millis(1000));
        b.add_label("outro", Duration::from_millis(2000));

        b.track("x", x, 0.0, |t| {
            t.tween(140.0, Duration::from_millis(1000), Ease::CubicInOut)
                .tween(-140.0, Duration::from_millis(1000), Ease::CubicInOut)
                .tween(0.0, Duration::from_millis(500), Ease::CubicOut);
        });

        b.track("y", y, 0.0, |t| {
            t.tween(-50.0, Duration::from_millis(1000), Ease::QuadInOut)
                .tween(50.0, Duration::from_millis(1000), Ease::QuadInOut)
                .tween(0.0, Duration::from_millis(500), Ease::QuadOut);
        });

        b.track("scale", scale, 1.0, |t| {
            t.tween(1.25, Duration::from_millis(1000), Ease::BackOut)
                .tween(0.85, Duration::from_millis(1000), Ease::CubicInOut)
                .tween(1.0, Duration::from_millis(500), Ease::QuadOut);
        });

        b.track("opacity", opacity, 1.0, |t| {
            t.tween(0.6, Duration::from_millis(1000), Ease::Linear)
                .tween(0.9, Duration::from_millis(1000), Ease::Linear)
                .tween(1.0, Duration::from_millis(500), Ease::Linear);
        });

        b.track("rotation", rotation, 0.0, |t| {
            t.tween(15.0, Duration::from_millis(1000), Ease::SineInOut)
                .tween(-15.0, Duration::from_millis(1000), Ease::SineInOut)
                .tween(0.0, Duration::from_millis(500), Ease::SineOut);
        });
    });

    let tl_poll = tl.clone();
    use_frame(move |_| {
        progress.set(tl_poll.progress());
        time_sec.set(tl_poll.time().as_secs_f64());
        if tl_poll.is_playing() {
            status.set("Playing".to_string());
        } else if tl_poll.is_completed() {
            status.set("Completed".to_string());
        } else {
            status.set("Paused".to_string());
        }
    });

    let tl_play = tl.clone();
    let tl_pause = tl.clone();
    let tl_rev = tl.clone();
    let tl_restart = tl.clone();
    let tl_intro = tl.clone();
    let tl_accent = tl.clone();
    let tl_outro = tl.clone();
    let tl_scrub = tl;

    rsx! {
        div {
            class: "w-full flex flex-col items-center gap-6",

            // Telemetry HUD
            div {
                id: "telemetry-hud",
                class: "flex flex-wrap items-center justify-center gap-3 sm:gap-4 px-4 py-2.5 rounded-xl bg-slate-900/90 border border-slate-800 text-xs font-mono text-sky-400 shadow-lg",
                span { id: "hud-time", "t: {time_sec():.2}s" }
                span { id: "hud-progress", "progress: {progress():.3}" }
                span { id: "hud-x", "x: {x():.1}px" }
                span { id: "hud-y", "y: {y():.1}px" }
                span { id: "hud-scale", "scale: {scale():.2}" }
                span { id: "hud-rotation", "rot: {rotation():.1}°" }
                span { id: "hud-opacity", "opacity: {opacity():.2}" }
                span { id: "hud-status", class: "text-amber-400 font-semibold", "status: {status()}" }
            }

            // Canvas Area
            div {
                class: "w-full h-72 rounded-2xl bg-slate-900/40 border border-slate-800/80 flex items-center justify-center relative overflow-hidden shadow-inner",

                div {
                    id: "animated-card",
                    class: "w-56 h-36 rounded-2xl bg-gradient-to-br from-indigo-500 to-purple-600 shadow-2xl border border-white/20 flex flex-col items-center justify-center select-none text-white",
                    style: "transform: translate({x()}px, {y()}px) scale({scale()}) rotate({rotation()}deg); opacity: {opacity()}; transition: none;",
                    span { class: "font-bold text-lg", "Timeline Target" }
                    span { class: "text-xs opacity-80 font-mono", "kinetocore::timeline" }
                }
            }

            // Scrubber Slider
            div {
                class: "w-full max-w-xl flex flex-col gap-2 px-2",
                div {
                    class: "flex justify-between text-xs text-slate-400 font-mono",
                    span { "0.0s (intro)" }
                    span { "1.0s (accent)" }
                    span { "2.5s (outro)" }
                }
                input {
                    id: "scrubber",
                    r#type: "range",
                    min: "0",
                    max: "1000",
                    value: "{progress() * 1000.0}",
                    class: "w-full h-2.5 bg-slate-800 rounded-lg appearance-none cursor-pointer accent-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500/50",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<f64>() {
                            tl_scrub.set_progress((val / 1000.0) as f32);
                        }
                    }
                }
            }

            // Transport Control Buttons
            div {
                class: "flex flex-wrap items-center justify-center gap-2",

                button {
                    id: "btn-play",
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white shadow-md shadow-emerald-600/20 active:scale-95 transition cursor-pointer",
                    onclick: move |_| tl_play.play(),
                    "▶ Play"
                }
                button {
                    id: "btn-pause",
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 active:scale-95 transition cursor-pointer",
                    onclick: move |_| tl_pause.pause(),
                    "⏸ Pause"
                }
                button {
                    id: "btn-reverse",
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white shadow-md shadow-indigo-600/20 active:scale-95 transition cursor-pointer",
                    onclick: move |_| tl_rev.reverse(),
                    "◀ Reverse"
                }
                button {
                    id: "btn-restart",
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 active:scale-95 transition cursor-pointer",
                    onclick: move |_| tl_restart.restart(),
                    "↺ Restart"
                }

                div { class: "w-px h-6 bg-slate-800 mx-1 hidden sm:block" }

                button {
                    id: "btn-seek-intro",
                    class: "px-3 py-2 text-xs font-medium rounded-xl bg-slate-900 hover:bg-slate-800 text-sky-400 border border-slate-800 active:scale-95 transition cursor-pointer font-mono",
                    onclick: move |_| {
                        tl_intro.seek_label("intro");
                    },
                    "@intro"
                }
                button {
                    id: "btn-seek-accent",
                    class: "px-3 py-2 text-xs font-medium rounded-xl bg-slate-900 hover:bg-slate-800 text-sky-400 border border-slate-800 active:scale-95 transition cursor-pointer font-mono",
                    onclick: move |_| {
                        tl_accent.seek_label("accent");
                    },
                    "@accent"
                }
                button {
                    id: "btn-seek-outro",
                    class: "px-3 py-2 text-xs font-medium rounded-xl bg-slate-900 hover:bg-slate-800 text-sky-400 border border-slate-800 active:scale-95 transition cursor-pointer font-mono",
                    onclick: move |_| {
                        tl_outro.seek_label("outro");
                    },
                    "@outro"
                }
            }
        }
    }
}
