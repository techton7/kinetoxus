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

    // Sub-timeline targets: nested badge inside the card
    let sub_badge_y = use_signal(|| 0.0f64);
    let sub_badge_rot = use_signal(|| 0.0f64);
    let sub_badge_scale = use_signal(|| 1.0f64);

    let mut progress = use_signal(|| 0.0f32);
    let mut time_sec = use_signal(|| 0.0f64);
    let mut status = use_signal(|| "Paused".to_string());
    let mut epoch_display = use_signal(|| 1u64);
    let mut mutation_log = use_signal(|| "Initial Snapshot (Epoch 1)".to_string());

    let tl = use_timeline(|b| {
        b.autoplay(false);
        b.add_label("intro", Duration::ZERO);
        b.add_label("accent", Duration::from_millis(1000));
        b.add_label("outro", Duration::from_millis(2000));

        // Master tracks
        b.track("x", x, 0.0, |t| {
            t.tween(120.0, Duration::from_millis(1000), Ease::CubicInOut)
                .tween(-120.0, Duration::from_millis(1000), Ease::CubicInOut)
                .tween(0.0, Duration::from_millis(500), Ease::CubicOut);
        });

        b.track("y", y, 0.0, |t| {
            t.tween(-40.0, Duration::from_millis(1000), Ease::QuadInOut)
                .tween(40.0, Duration::from_millis(1000), Ease::QuadInOut)
                .tween(0.0, Duration::from_millis(500), Ease::QuadOut);
        });

        b.track("scale", scale, 1.0, |t| {
            t.tween(1.2, Duration::from_millis(1000), Ease::BackOut)
                .tween(0.9, Duration::from_millis(1000), Ease::CubicInOut)
                .tween(1.0, Duration::from_millis(500), Ease::QuadOut);
        });

        b.track("opacity", opacity, 1.0, |t| {
            t.tween(0.7, Duration::from_millis(1000), Ease::Linear)
                .tween(0.9, Duration::from_millis(1000), Ease::Linear)
                .tween(1.0, Duration::from_millis(500), Ease::Linear);
        });

        b.track("rotation", rotation, 0.0, |t| {
            t.tween(15.0, Duration::from_millis(1000), Ease::SineInOut)
                .tween(-15.0, Duration::from_millis(1000), Ease::SineInOut)
                .tween(0.0, Duration::from_millis(500), Ease::SineOut);
        });

        // Nested sub-timeline: badge coordinates relative to card, starts at 400ms
        b.sub_timeline("sub_badge", Position::Absolute(Duration::from_millis(400)), |sub| {
            sub.set_time_scale(1.5).expect("valid scale");
            sub.track("badge_y", sub_badge_y, 0.0, |t| {
                t.tween(-25.0, Duration::from_millis(600), Ease::BackOut)
                    .tween(0.0, Duration::from_millis(600), Ease::BounceOut);
            });
            sub.track("badge_rot", sub_badge_rot, 0.0, |t| {
                t.tween(360.0, Duration::from_millis(1200), Ease::CubicInOut);
            });
            sub.track("badge_scale", sub_badge_scale, 1.0, |t| {
                t.tween(1.3, Duration::from_millis(600), Ease::QuadOut)
                    .tween(1.0, Duration::from_millis(600), Ease::QuadIn);
            });
            sub.add_label("apex", Position::Absolute(Duration::from_millis(600)));
        });
    });

    let tl_poll = tl.clone();
    use_frame(move |_| {
        progress.set(tl_poll.progress());
        time_sec.set(tl_poll.time().as_secs_f64());
        epoch_display.set(tl_poll.epoch());
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
    let tl_apex = tl.clone();
    let tl_scrub = tl.clone();
    let tl_speed_half = tl.clone();
    let tl_speed_1 = tl.clone();
    let tl_speed_2 = tl.clone();
    let tl_mutate_pulse = tl.clone();
    let tl_mutate_swap = tl;

    rsx! {
        div {
            class: "w-full flex flex-col items-center gap-3",

            // Telemetry HUD
            div {
                id: "telemetry-hud",
                class: "flex flex-wrap items-center justify-center gap-3 sm:gap-4 px-4 py-2 rounded-xl bg-slate-900/90 border border-slate-800 text-xs font-mono text-sky-400 shadow-lg",
                span { id: "hud-epoch", class: "text-purple-400 font-bold", "epoch: {epoch_display()}" }
                span { id: "hud-time", "t: {time_sec():.2}s" }
                span { id: "hud-progress", "prog: {progress():.3}" }
                span { id: "hud-x", "x: {x():.1}px" }
                span { id: "hud-y", "y: {y():.1}px" }
                span { id: "hud-scale", "scale: {scale():.2}" }
                span { id: "hud-badge-y", class: "text-amber-300", "sub_y: {sub_badge_y():.1}px" }
                span { id: "hud-badge-rot", class: "text-amber-300", "sub_rot: {sub_badge_rot():.1}°" }
                span { id: "hud-status", class: "text-emerald-400 font-semibold", "status: {status()}" }
            }

            // Mutation Status Pill
            div {
                id: "mutation-pill",
                class: "px-3 py-1 text-xs font-mono rounded-full bg-indigo-950/70 border border-indigo-700/50 text-indigo-300",
                "Graph: {mutation_log()}"
            }

            // Canvas Area
            div {
                class: "w-full h-48 rounded-2xl bg-slate-900/40 border border-slate-800/80 flex items-center justify-center relative overflow-hidden shadow-inner",

                div {
                    id: "animated-card",
                    class: "w-60 h-36 rounded-2xl bg-gradient-to-br from-indigo-500 to-purple-600 shadow-2xl border border-white/20 bg-no-repeat flex flex-col items-center justify-center select-none text-white relative",
                    style: "transform: translate({x()}px, {y()}px) scale({scale()}) rotate({rotation()}deg); opacity: {opacity()}; transition: none;",
                    span { class: "font-bold text-base", "Master Timeline Graph" }
                    span { class: "text-xs opacity-80 font-mono", "kinetocore::timeline::tree" }

                    // Child nested sub-timeline target
                    div {
                        id: "nested-badge",
                        class: "mt-2 px-3 py-1 rounded-full bg-amber-400 text-slate-950 text-xs font-bold shadow-md border border-amber-200 select-none",
                        style: "transform: translateY({sub_badge_y()}px) rotate({sub_badge_rot()}deg) scale({sub_badge_scale()}); transition: none;",
                        "⚡ SubTimeline Scope"
                    }
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

                // Label navigation buttons (including nested scoped path)
                button {
                    id: "btn-seek-intro",
                    class: "px-3 py-2 text-xs font-medium rounded-xl bg-slate-900 hover:bg-slate-800 text-sky-400 border border-slate-800 active:scale-95 transition cursor-pointer font-mono",
                    onclick: move |_| { tl_intro.seek_label("intro"); },
                    "@intro"
                }
                button {
                    id: "btn-seek-accent",
                    class: "px-3 py-2 text-xs font-medium rounded-xl bg-slate-900 hover:bg-slate-800 text-sky-400 border border-slate-800 active:scale-95 transition cursor-pointer font-mono",
                    onclick: move |_| { tl_accent.seek_label("accent"); },
                    "@accent"
                }
                button {
                    id: "btn-seek-sub-apex",
                    class: "px-3 py-2 text-xs font-medium rounded-xl bg-slate-900 hover:bg-slate-800 text-amber-300 border border-amber-900/50 active:scale-95 transition cursor-pointer font-mono",
                    onclick: move |_| { tl_apex.seek_label("sub_badge::apex"); },
                    "@sub::apex"
                }
                button {
                    id: "btn-seek-outro",
                    class: "px-3 py-2 text-xs font-medium rounded-xl bg-slate-900 hover:bg-slate-800 text-sky-400 border border-slate-800 active:scale-95 transition cursor-pointer font-mono",
                    onclick: move |_| { tl_outro.seek_label("outro"); },
                    "@outro"
                }
            }

            // Phase 10 Dynamic Mutation Controls & Speed Warping
            div {
                class: "flex flex-wrap items-center justify-center gap-2 mt-1",

                span { class: "text-xs font-mono text-slate-500 mr-1", "Speed:" }
                button {
                    id: "btn-speed-half",
                    class: "px-2.5 py-1.5 text-xs font-mono rounded-lg bg-slate-900 hover:bg-slate-800 text-slate-300 border border-slate-800 cursor-pointer",
                    onclick: move |_| tl_speed_half.set_time_scale(0.5),
                    "0.5x"
                }
                button {
                    id: "btn-speed-1",
                    class: "px-2.5 py-1.5 text-xs font-mono rounded-lg bg-slate-900 hover:bg-slate-800 text-slate-300 border border-slate-800 cursor-pointer",
                    onclick: move |_| tl_speed_1.set_time_scale(1.0),
                    "1.0x"
                }
                button {
                    id: "btn-speed-2",
                    class: "px-2.5 py-1.5 text-xs font-mono rounded-lg bg-slate-900 hover:bg-slate-800 text-slate-300 border border-slate-800 cursor-pointer",
                    onclick: move |_| tl_speed_2.set_time_scale(2.0),
                    "2.0x"
                }

                div { class: "w-px h-6 bg-slate-800 mx-1 hidden sm:block" }

                button {
                    id: "btn-splice-clip",
                    class: "px-3 py-1.5 text-xs font-semibold rounded-xl bg-purple-700 hover:bg-purple-600 text-white shadow-md shadow-purple-700/20 active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        let curr = tl_mutate_pulse.time();
                        match tl_mutate_pulse.insert_clip(
                            "scale",
                            Position::Absolute(curr),
                            1.0f64,
                            1.5f64,
                            Duration::from_millis(400),
                            Ease::ElasticOut,
                            ActiveOverlapPolicy::ApplyAtCurrentTime,
                        ) {
                            Ok(_) => {
                                epoch_display.set(tl_mutate_pulse.epoch());
                                mutation_log.set(format!("Spliced Elastic Pulse at {:.2}s (Epoch {})", curr.as_secs_f64(), tl_mutate_pulse.epoch()));
                            }
                            Err(e) => {
                                mutation_log.set(format!("Splice Error: {e:?}"));
                            }
                        }
                    },
                    "⚡ Splice Clip (C⁰ Continuity)"
                }

                button {
                    id: "btn-swap-subtimeline",
                    class: "px-3 py-1.5 text-xs font-semibold rounded-xl bg-amber-600 hover:bg-amber-500 text-white shadow-md shadow-amber-600/20 active:scale-95 transition cursor-pointer",
                    onclick: move |_| {
                        let mut new_sub = SubTimelineBuilder::new("sub_badge", Position::Absolute(Duration::from_millis(200)));
                        new_sub.set_time_scale(2.0).ok();
                        new_sub.track("badge_rot", 0.0f64, |t| {
                            t.tween(-720.0f64, Duration::from_millis(800), Ease::BackInOut);
                        });
                        new_sub.track("badge_scale", 1.0f64, |t| {
                            t.tween(1.6f64, Duration::from_millis(400), Ease::QuadOut)
                                .tween(1.0f64, Duration::from_millis(400), Ease::QuadIn);
                        });
                        let node = new_sub.build_node();
                        match tl_mutate_swap.swap_subtimeline(None, node, ActiveOverlapPolicy::ApplyAtCurrentTime) {
                            Ok(_) => {
                                epoch_display.set(tl_mutate_swap.epoch());
                                mutation_log.set(format!("Swapped SubTimeline with 2.0x Spin (Epoch {})", tl_mutate_swap.epoch()));
                            }
                            Err(e) => {
                                mutation_log.set(format!("Swap Error: {e:?}"));
                            }
                        }
                    },
                    "🔄 Swap SubTimeline"
                }
            }
        }
    }
}
