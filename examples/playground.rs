//! Kinetoxus Unified Motion Playground (`examples/playground.rs`)
//!
//! # Architecture & Capabilities
//! Merges the exhaustive feature surface of all Kinetoxus motion domains into a single
//! comprehensive, production-grade interactive showcase styled entirely with Tailwind CSS:
//!
//! 1. **Timeline**: Multi-track timeline sequencing (`use_timeline`), keyframes, named labels
//!    (`intro`, `accent`, `outro`), transport controls (play, pause, reverse, restart, seek),
//!    and real-time bidirectional progress scrubbing.
//! 2. **Spring Physics**: Closed-form analytical spring solver (`kinetocore::spring`), damping regimes
//!    (Bouncy, Stiff, Gentle), and mid-flight $C^1$ velocity-preserving retargeting.
//! 3. **Card Stack**: Multi-card 3D perspective stacked depth choreography (Expand, Collapse, Focus, Reset).
//! 4. **Interruption Lab**: High-frequency animation interruption stress-testing, rapid jitter, and settle detection.
//! 5. **Zero-Signal Handles**: Direct memory `Transform2D` handle animation without reactive signal overhead.
//! 6. **Interactive Tweens**: Multi-dimensional tweens (`x`, `y`, `scale`, `rotation`, `opacity`) with standard easings.
//! 7. **Diagnostic Matrix**: Visual card rendering variations under scaling/transformation.
//!
//! # Dual-Host Runtime Execution
//! - Web: `dx serve --example playground --web`
//! - Native: `cargo run --example playground --features native`
//! - Native with Blitz Host: `cargo run --example playground --features native,blitz-host`

use std::time::Duration;

use dioxus::prelude::*;
use kinetoxus::prelude::*;
use oxidase::prelude::*;

const TAILWIND_CSS: &str = include_str!("../assets/tailwind.css");

#[oxidase::main]
fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Timeline,
    Spring,
    CardStack,
    Interruption,
    Handles,
    Tweens,
    Diagnostics,
}

impl Tab {
    fn label(&self) -> &'static str {
        match self {
            Tab::Timeline => "Timeline",
            Tab::Spring => "Spring Physics",
            Tab::CardStack => "Card Stack",
            Tab::Interruption => "Interruption Lab",
            Tab::Handles => "Zero-Signal Handles",
            Tab::Tweens => "Interactive Tweens",
            Tab::Diagnostics => "Diagnostic Matrix",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Tab::Timeline => "⏱️",
            Tab::Spring => "🌀",
            Tab::CardStack => "🗂️",
            Tab::Interruption => "⚡",
            Tab::Handles => "🎯",
            Tab::Tweens => "🎨",
            Tab::Diagnostics => "🔍",
        }
    }
}

#[component]
fn App() -> Element {
    let mut current_tab = use_signal(|| Tab::Timeline);
    let mut fps = use_signal(|| 60.0f64);

    use_frame(move |info| {
        let dt = info.delta.as_secs_f64();
        if dt > 0.001 {
            fps.set(1.0 / dt);
        }
    });

    let tabs = [
        Tab::Timeline,
        Tab::Spring,
        Tab::CardStack,
        Tab::Interruption,
        Tab::Handles,
        Tab::Tweens,
        Tab::Diagnostics,
    ];

    rsx! {
        style { "{TAILWIND_CSS}" }

        div {
            class: "min-h-screen bg-slate-950 text-slate-100 flex flex-col items-center font-sans selection:bg-indigo-500 selection:text-white p-4 md:p-8 box-border",

            // Global Header
            header {
                class: "w-full max-w-5xl flex flex-col md:flex-row items-center justify-between gap-4 pb-6 mb-6 border-b border-slate-800/80",

                div {
                    class: "flex items-center gap-3",
                    div {
                        class: "w-10 h-10 rounded-xl bg-gradient-to-tr from-indigo-600 to-purple-500 flex items-center justify-center shadow-lg shadow-indigo-500/20 text-xl font-bold",
                        "K"
                    }
                    div {
                        h1 {
                            class: "text-xl font-bold bg-gradient-to-r from-white via-slate-200 to-slate-400 bg-clip-text text-transparent tracking-tight",
                            "Kinetoxus Motion Playground"
                        }
                        p {
                            class: "text-xs text-slate-400 font-mono",
                            "Unified Dual-Host Motion Architecture • Tailwind CSS"
                        }
                    }
                }

                div {
                    class: "flex items-center gap-3",
                    div {
                        class: "flex items-center gap-2 px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-800 text-xs font-mono text-emerald-400 shadow-inner",
                        span { class: "w-2 h-2 rounded-full bg-emerald-500 animate-pulse" }
                        span { "FPS: {fps():.0}" }
                    }
                    div {
                        class: "px-3 py-1.5 rounded-lg bg-indigo-950/60 border border-indigo-800/50 text-xs font-mono text-indigo-300 font-medium",
                        "{current_tab().icon()} {current_tab().label()}"
                    }
                }
            }

            // Tab Navigation Bar
            nav {
                class: "w-full max-w-5xl flex flex-wrap items-center justify-center gap-1.5 p-1.5 bg-slate-900/90 backdrop-blur rounded-2xl border border-slate-800/80 mb-8 shadow-xl",
                for tab in tabs {
                    {
                        let is_active = current_tab() == tab;
                        let active_class = if is_active {
                            "bg-indigo-600 text-white shadow-md shadow-indigo-600/30 font-semibold"
                        } else {
                            "text-slate-400 hover:text-slate-200 hover:bg-slate-800/60 font-medium"
                        };
                        rsx! {
                            button {
                                key: "{tab.label()}",
                                class: "px-3.5 py-2 rounded-xl text-xs sm:text-sm transition-all duration-150 flex items-center gap-1.5 cursor-pointer {active_class}",
                                onclick: move |_| current_tab.set(tab),
                                span { "{tab.icon()}" }
                                span { "{tab.label()}" }
                            }
                        }
                    }
                }
            }

            // Main Content Area
            main {
                class: "w-full max-w-5xl flex flex-col items-center justify-center",

                match current_tab() {
                    Tab::Timeline => rsx! { TimelineSection {} },
                    Tab::Spring => rsx! { SpringSection {} },
                    Tab::CardStack => rsx! { CardStackSection {} },
                    Tab::Interruption => rsx! { InterruptionSection {} },
                    Tab::Handles => rsx! { HandlesSection {} },
                    Tab::Tweens => rsx! { TweensSection {} },
                    Tab::Diagnostics => rsx! { DiagnosticsSection {} },
                }
            }
        }
    }
}

// =========================================================================
// 1. Timeline Section
// =========================================================================

#[component]
fn TimelineSection() -> Element {
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

// =========================================================================
// 2. Spring Physics Section
// =========================================================================

#[component]
fn SpringSection() -> Element {
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
                    class: "w-56 h-36 rounded-2xl bg-gradient-to-br from-cyan-500 to-blue-600 shadow-2xl border border-white/20 flex flex-col items-center justify-center select-none text-white",
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

// =========================================================================
// 3. Card Stack Section
// =========================================================================

#[component]
fn CardStackSection() -> Element {
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
                    class: "w-64 h-36 rounded-2xl bg-gradient-to-r from-emerald-600 to-teal-700 shadow-2xl border border-white/10 flex flex-col items-center justify-center absolute text-white",
                    style: "transform: translateY({c0_y()}px) scale({c0_scale()}); opacity: {c0_opacity()}; z-index: 10; transition: none;",
                    span { class: "font-bold text-base", "Card Alpha" }
                    span { class: "text-xs opacity-75 font-mono", "Background Layer" }
                }

                // Card 1 (Middle / Beta)
                div {
                    class: "w-64 h-36 rounded-2xl bg-gradient-to-r from-blue-600 to-indigo-700 shadow-2xl border border-white/10 flex flex-col items-center justify-center absolute text-white",
                    style: "transform: translateY({c1_y()}px) scale({c1_scale()}); opacity: {c1_opacity()}; z-index: 20; transition: none;",
                    span { class: "font-bold text-base", "Card Beta" }
                    span { class: "text-xs opacity-75 font-mono", "Middle Layer" }
                }

                // Card 2 (Front / Gamma)
                div {
                    class: "w-64 h-36 rounded-2xl bg-gradient-to-r from-purple-600 to-pink-600 shadow-2xl border border-white/20 flex flex-col items-center justify-center absolute text-white",
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

// =========================================================================
// 4. Interruption Lab Section
// =========================================================================

#[component]
fn InterruptionSection() -> Element {
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

// =========================================================================
// 5. Zero-Signal Handles Section
// =========================================================================

#[component]
fn HandlesSection() -> Element {
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

// =========================================================================
// 6. Interactive Tweens Section
// =========================================================================

#[component]
fn TweensSection() -> Element {
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

// =========================================================================
// 7. Diagnostic Matrix Section
// =========================================================================

#[component]
fn DiagnosticsSection() -> Element {
    rsx! {
        div {
            class: "w-full flex flex-col items-center gap-6",

            div {
                class: "px-4 py-2 rounded-xl bg-slate-900 border border-slate-800 text-xs font-mono text-sky-400 text-center",
                "Blitz scale(1.2) Transformation Seam & Border Diagnostic Matrix"
            }

            div {
                class: "grid grid-cols-1 md:grid-cols-3 gap-8 items-center justify-items-center p-4",

                // Card 1: Baseline
                div {
                    class: "w-52 h-32 bg-gradient-to-br from-indigo-600 to-purple-600 rounded-2xl shadow-2xl border border-white/20 flex flex-col items-center justify-center select-none text-white",
                    style: "transform: scale(1.15);",
                    span { class: "font-bold text-sm", "Card 1" }
                    span { class: "text-xs opacity-75 font-mono", "Baseline Gradient" }
                }

                // Card 2: Solid Background
                div {
                    class: "w-52 h-32 bg-indigo-600 rounded-2xl shadow-2xl border border-white/20 flex flex-col items-center justify-center select-none text-white",
                    style: "transform: scale(1.15);",
                    span { class: "font-bold text-sm", "Card 2" }
                    span { class: "text-xs opacity-75 font-mono", "Solid Background" }
                }

                // Card 3: No-Repeat Gradient
                div {
                    class: "w-52 h-32 bg-gradient-to-br from-indigo-600 to-purple-600 rounded-2xl shadow-2xl border border-white/20 flex flex-col items-center justify-center select-none text-white bg-no-repeat",
                    style: "transform: scale(1.15);",
                    span { class: "font-bold text-sm", "Card 3" }
                    span { class: "text-xs opacity-75 font-mono", "bg-no-repeat" }
                }

                // Card 4: No Shadow
                div {
                    class: "w-52 h-32 bg-gradient-to-br from-indigo-600 to-purple-600 rounded-2xl border border-white/20 flex flex-col items-center justify-center select-none text-white",
                    style: "transform: scale(1.15);",
                    span { class: "font-bold text-sm", "Card 4" }
                    span { class: "text-xs opacity-75 font-mono", "No Shadow" }
                }

                // Card 5: No Radius
                div {
                    class: "w-52 h-32 bg-gradient-to-br from-indigo-600 to-purple-600 shadow-2xl border border-white/20 flex flex-col items-center justify-center select-none text-white",
                    style: "transform: scale(1.15);",
                    span { class: "font-bold text-sm", "Card 5" }
                    span { class: "text-xs opacity-75 font-mono", "No Radius (Sharp)" }
                }
            }
        }
    }
}
