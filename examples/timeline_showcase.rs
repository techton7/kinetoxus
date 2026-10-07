//! Kinetoxus Multi-Track Choreographed Timeline Showcase Example.
//! Demonstrates multi-channel animation (x, y, scale, opacity, rotation),
//! interactive playback controls, named label seeking, real-time scrubbing,
//! and live telemetry HUD.

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
    let x = use_signal(|| 0.0f64);
    let y = use_signal(|| 0.0f64);
    let scale = use_signal(|| 1.0f64);
    let opacity = use_signal(|| 1.0f64);
    let rotation = use_signal(|| 0.0f64);
    let mut fps = use_signal(|| 60.0f64);
    let mut progress = use_signal(|| 0.0f32);
    let mut time_sec = use_signal(|| 0.0f64);
    let mut status = use_signal(|| "idle".to_string());

    let tl = use_timeline(|b| {
        b.autoplay(false); // start paused
        b.add_label("intro", Duration::ZERO);
        b.add_label("accent", Duration::from_millis(1000));
        b.add_label("outro", Duration::from_millis(2000));

        b.track("x", &x, 0.0, |t| {
            t.tween(140.0, Duration::from_millis(1000), Ease::CubicInOut)
                .tween(-140.0, Duration::from_millis(1000), Ease::CubicInOut)
                .tween(0.0, Duration::from_millis(500), Ease::CubicOut);
        });

        b.track("y", &y, 0.0, |t| {
            t.tween(-50.0, Duration::from_millis(1000), Ease::QuadInOut)
                .tween(50.0, Duration::from_millis(1000), Ease::QuadInOut)
                .tween(0.0, Duration::from_millis(500), Ease::QuadOut);
        });

        b.track("scale", &scale, 1.0, |t| {
            t.tween(1.25, Duration::from_millis(1000), Ease::BackOut)
                .tween(0.85, Duration::from_millis(1000), Ease::CubicInOut)
                .tween(1.0, Duration::from_millis(500), Ease::QuadOut);
        });

        b.track("opacity", &opacity, 1.0, |t| {
            t.tween(0.6, Duration::from_millis(1000), Ease::Linear)
                .tween(0.9, Duration::from_millis(1000), Ease::Linear)
                .tween(1.0, Duration::from_millis(500), Ease::Linear);
        });

        b.track("rotation", &rotation, 0.0, |t| {
            t.tween(15.0, Duration::from_millis(1000), Ease::SineInOut)
                .tween(-15.0, Duration::from_millis(1000), Ease::SineInOut)
                .tween(0.0, Duration::from_millis(500), Ease::SineOut);
        });
    });

    let tl_frame = tl.clone();
    use_frame(move |info| {
        let dt = info.delta.as_secs_f64();
        if dt > 0.001 {
            fps.set(1.0 / dt);
        }
        progress.set(tl_frame.progress());
        time_sec.set(tl_frame.time().as_secs_f64());
        if tl_frame.is_playing() {
            status.set("Playing".to_string());
        } else if tl_frame.is_completed() {
            status.set("Completed".to_string());
        } else {
            status.set("Paused".to_string());
        }
    });

    let tl_play = tl.clone();
    let tl_pause = tl.clone();
    let tl_reverse = tl.clone();
    let tl_restart = tl.clone();
    let tl_intro = tl.clone();
    let tl_accent = tl.clone();
    let tl_outro = tl.clone();
    let tl_scrub = tl.clone();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; gap: 1.5rem; background: #0b0f19; color: #f8fafc; padding: 2rem;",

            // Telemetry HUD
            div {
                id: "telemetry-hud",
                style: "display: flex; flex-wrap: wrap; gap: 1rem; background: rgba(30, 41, 59, 0.85); padding: 0.6rem 1.2rem; border-radius: 8px; font-family: monospace; font-size: 0.85rem; color: #38bdf8; border: 1px solid rgba(56, 189, 248, 0.2); justify-content: center; max-width: 840px;",
                span { id: "hud-fps", "fps: {fps():.0}" }
                span { id: "hud-time", "time: {time_sec():.2}s" }
                span { id: "hud-progress", "prog: {progress():.2}" }
                span { id: "hud-x", "x: {x():.1}px" }
                span { id: "hud-y", "y: {y():.1}px" }
                span { id: "hud-scale", "scale: {scale():.2}" }
                span { id: "hud-rotation", "rot: {rotation():.1}°" }
                span { id: "hud-opacity", "opacity: {opacity():.2}" }
                span { id: "hud-status", "status: {status()}" }
            }

            // Animated Choreographed Card
            div {
                id: "animated-card",
                style: "width: 240px; height: 150px; background: linear-gradient(135deg, #4f46e5 0%, #06b6d4 100%); border-radius: 16px; display: flex; flex-direction: column; align-items: center; justify-content: center; box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5); transform: translate({x()}px, {y()}px) scale({scale()}) rotate({rotation()}deg); opacity: {opacity()}; transition: none; border: 2px solid rgba(255, 255, 255, 0.2);",
                span { style: "font-weight: 700; font-size: 1.25rem;", "Timeline Motion" }
                span { style: "font-size: 0.85rem; opacity: 0.85;", "kinetoxus::use_timeline" }
            }

            // Scrubber Slider
            div {
                style: "display: flex; flex-direction: column; align-items: center; gap: 0.5rem; width: 100%; max-width: 400px;",
                div {
                    style: "display: flex; justify-content: space-between; width: 100%; font-size: 0.8rem; color: #94a3b8; font-family: monospace;",
                    span { "0.0s (intro)" }
                    span { "1.0s (accent)" }
                    span { "2.0s (outro)" }
                    span { "2.5s (end)" }
                }
                input {
                    id: "scrubber",
                    r#type: "range",
                    min: "0",
                    max: "1000",
                    value: "{progress() * 1000.0}",
                    style: "width: 100%; cursor: pointer;",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<f32>() {
                            tl_scrub.set_progress(val / 1000.0);
                        }
                    },
                }
            }

            // Transport Control Buttons
            div {
                style: "display: flex; flex-wrap: wrap; gap: 0.75rem; justify-content: center; max-width: 600px;",
                button {
                    id: "btn-play",
                    style: "padding: 0.6rem 1.2rem; background: #3b82f6; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600; font-size: 0.875rem;",
                    onclick: move |_| tl_play.play(),
                    "Play"
                }
                button {
                    id: "btn-pause",
                    style: "padding: 0.6rem 1.2rem; background: #64748b; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600; font-size: 0.875rem;",
                    onclick: move |_| tl_pause.pause(),
                    "Pause"
                }
                button {
                    id: "btn-reverse",
                    style: "padding: 0.6rem 1.2rem; background: #8b5cf6; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600; font-size: 0.875rem;",
                    onclick: move |_| tl_reverse.reverse(),
                    "Reverse"
                }
                button {
                    id: "btn-restart",
                    style: "padding: 0.6rem 1.2rem; background: #ef4444; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600; font-size: 0.875rem;",
                    onclick: move |_| tl_restart.restart(),
                    "Restart"
                }
            }

            // Label Seeking Buttons
            div {
                style: "display: flex; flex-wrap: wrap; gap: 0.75rem; justify-content: center; max-width: 600px;",
                button {
                    id: "btn-seek-intro",
                    style: "padding: 0.5rem 1rem; background: #0284c7; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 500; font-size: 0.8rem;",
                    onclick: move |_| { tl_intro.seek_label("intro"); },
                    "Seek: intro (0.0s)"
                }
                button {
                    id: "btn-seek-accent",
                    style: "padding: 0.5rem 1rem; background: #0d9488; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 500; font-size: 0.8rem;",
                    onclick: move |_| { tl_accent.seek_label("accent"); },
                    "Seek: accent (1.0s)"
                }
                button {
                    id: "btn-seek-outro",
                    style: "padding: 0.5rem 1rem; background: #d97706; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 500; font-size: 0.8rem;",
                    onclick: move |_| { tl_outro.seek_label("outro"); },
                    "Seek: outro (2.0s)"
                }
            }
        }
    }
}
