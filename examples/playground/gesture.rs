use std::time::Duration;

use dioxus::prelude::*;
use kinetocore::spring::SpringConfig;
use kinetoxus::prelude::*;
use oxidase::prelude::*;

#[component]
pub fn GestureSection() -> Element {
    let mut x = use_signal(|| 0.0f64);
    let mut vel = use_signal(|| 0.0f64);
    let mut state = use_signal(|| "Settled".to_string());

    let mut is_dragging = use_signal(|| false);
    let mut drag_origin_client_x = use_signal(|| 0.0f64);
    let mut drag_start_pos = use_signal(|| 0.0f64);

    let start_instant = use_hook(web_time::Instant::now);

    let spring = SpringConfig::DEFAULT;
    let config = GestureContinuationConfig::default()
        .with_snap_points(vec![-150.0, 0.0, 150.0])
        .with_bounce(-220.0, 220.0, spring);

    let gesture = use_gesture_continuation(x, config);

    // Telemetry polling: actively tracks velocity and state during motion, idles when settled
    let g_poll = gesture.clone();
    use_frame(move |_info| {
        let phase = g_poll.phase();
        if phase != GesturePhase::Idle {
            let current_vel = g_poll.current_velocity();
            vel.set(current_vel);

            let state_str = match phase {
                GesturePhase::Dragging => "Dragging",
                GesturePhase::Inertia | GesturePhase::Bounce => "Inertia Decay",
                GesturePhase::Idle => "Settled",
            };
            if state() != state_str {
                state.set(state_str.to_string());
            }
        } else {
            if vel() != 0.0 {
                vel.set(0.0);
            }
            if state() != "Settled" {
                state.set("Settled".to_string());
            }
        }
    });

    let g_down = gesture.clone();
    let g_move = gesture.clone();
    let g_up = gesture.clone();
    let g_leave = gesture.clone();
    let g_flick_l = gesture.clone();
    let g_flick_r = gesture.clone();
    let g_reset = gesture;

    rsx! {
        div {
            class: "w-full flex flex-col items-center gap-6",

            // Visual Telemetry HUD
            div {
                class: "flex flex-wrap items-center justify-center gap-4 px-5 py-2.5 rounded-xl bg-slate-900/90 border border-slate-800 text-xs font-mono shadow-lg",

                div {
                    class: "flex items-center gap-2",
                    span { class: "text-slate-400", "Position:" }
                    span {
                        id: "hud-gesture-pos",
                        class: "text-sky-400 font-bold",
                        "{x():.1}px"
                    }
                }

                div {
                    class: "flex items-center gap-2",
                    span { class: "text-slate-400", "Velocity:" }
                    span {
                        id: "hud-gesture-vel",
                        class: "text-indigo-400 font-bold",
                        "{vel():.1}px/s"
                    }
                }

                div {
                    class: "flex items-center gap-2",
                    span { class: "text-slate-400", "State:" }
                    span {
                        id: "hud-gesture-state",
                        class: if state() == "Settled" {
                            "text-emerald-400 font-bold"
                        } else if state() == "Dragging" {
                            "text-amber-400 font-bold animate-pulse"
                        } else {
                            "text-purple-400 font-bold"
                        },
                        "{state()}"
                    }
                }
            }

            // Interactive Playground Canvas
            div {
                class: "w-full h-80 rounded-2xl bg-slate-900/40 border border-slate-800/80 flex flex-col items-center justify-center relative overflow-hidden shadow-inner select-none",
                onpointermove: move |evt: PointerEvent| {
                    if is_dragging() {
                        let client_x = evt.client_coordinates().x;
                        let delta = client_x - drag_origin_client_x();
                        let target_x = drag_start_pos() + delta;
                        g_move.drag_move(target_x, start_instant.elapsed());
                    }
                },
                onpointerup: move |_| {
                    if is_dragging() {
                        is_dragging.set(false);
                        g_up.drag_end(start_instant.elapsed());
                    }
                },
                onpointerleave: move |_| {
                    if is_dragging() {
                        is_dragging.set(false);
                        g_leave.drag_end(start_instant.elapsed());
                    }
                },

                // Snapping anchor markers & boundary indicators
                div {
                    class: "absolute inset-x-0 h-0.5 bg-slate-800 flex items-center justify-center pointer-events-none",

                    // Boundary Left (-220px)
                    div {
                        class: "absolute top-1/2 -translate-y-1/2 -translate-x-1/2 flex flex-col items-center",
                        style: "left: calc(50% - 220px);",
                        div { class: "w-px h-16 bg-rose-500/50" }
                        span { class: "text-[10px] font-mono text-rose-400/80 mt-1", "-220" }
                    }

                    // Snap Left (-150px)
                    div {
                        class: "absolute top-1/2 -translate-y-1/2 -translate-x-1/2 flex flex-col items-center",
                        style: "left: calc(50% - 150px);",
                        div { class: "w-2 h-2 rounded-full bg-cyan-400/80 shadow-sm shadow-cyan-500/50" }
                        span { class: "text-[10px] font-mono text-cyan-400/90 mt-1", "-150" }
                    }

                    // Snap Center (0px)
                    div {
                        class: "absolute top-1/2 -translate-y-1/2 -translate-x-1/2 flex flex-col items-center",
                        style: "left: 50%;",
                        div { class: "w-2.5 h-2.5 rounded-full bg-indigo-400 shadow-md shadow-indigo-500/50" }
                        span { class: "text-[10px] font-mono text-indigo-400 mt-1 font-semibold", "0" }
                    }

                    // Snap Right (+150px)
                    div {
                        class: "absolute top-1/2 -translate-y-1/2 -translate-x-1/2 flex flex-col items-center",
                        style: "left: calc(50% + 150px);",
                        div { class: "w-2 h-2 rounded-full bg-cyan-400/80 shadow-sm shadow-cyan-500/50" }
                        span { class: "text-[10px] font-mono text-cyan-400/90 mt-1", "+150" }
                    }

                    // Boundary Right (+220px)
                    div {
                        class: "absolute top-1/2 -translate-y-1/2 -translate-x-1/2 flex flex-col items-center",
                        style: "left: calc(50% + 220px);",
                        div { class: "w-px h-16 bg-rose-500/50" }
                        span { class: "text-[10px] font-mono text-rose-400/80 mt-1", "+220" }
                    }
                }

                // Interactive Draggable Card
                div {
                    id: "gesture-card",
                    class: "w-64 h-36 rounded-2xl bg-gradient-to-br from-indigo-500 via-purple-600 to-pink-600 shadow-2xl border border-white/20 bg-no-repeat flex flex-col items-center justify-center cursor-grab active:cursor-grabbing text-white touch-none z-10",
                    style: "transform: translateX({x()}px); transition: none;",
                    onpointerdown: move |evt: PointerEvent| {
                        is_dragging.set(true);
                        let client_x = evt.client_coordinates().x;
                        drag_origin_client_x.set(client_x);
                        drag_start_pos.set(x());
                        g_down.drag_start(x(), start_instant.elapsed());
                    },
                    span { class: "font-bold text-lg pointer-events-none", "Gesture Card" }
                    span { class: "text-xs opacity-80 font-mono pointer-events-none", "Drag or Flick Me" }
                    span { class: "text-[11px] opacity-60 font-mono mt-1 pointer-events-none", "Snaps to [-150, 0, 150]" }
                }
            }

            // Controls & Synthetic Gesture Triggers
            div {
                class: "flex flex-wrap items-center justify-center gap-3",

                button {
                    id: "btn-flick-left",
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white shadow-md active:scale-95 transition cursor-pointer flex items-center gap-1.5",
                    onclick: move |_| {
                        let now = start_instant.elapsed();
                        let curr = x();
                        g_flick_l.drag_start(curr, now);
                        g_flick_l.drag_move(curr - 70.0, now + Duration::from_millis(20));
                        g_flick_l.drag_move(curr - 150.0, now + Duration::from_millis(40));
                        g_flick_l.drag_end(now + Duration::from_millis(40));
                    },
                    "⬅️ Flick Left"
                }

                button {
                    id: "btn-flick-right",
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-purple-600 hover:bg-purple-500 text-white shadow-md active:scale-95 transition cursor-pointer flex items-center gap-1.5",
                    onclick: move |_| {
                        let now = start_instant.elapsed();
                        let curr = x();
                        g_flick_r.drag_start(curr, now);
                        g_flick_r.drag_move(curr + 70.0, now + Duration::from_millis(20));
                        g_flick_r.drag_move(curr + 150.0, now + Duration::from_millis(40));
                        g_flick_r.drag_end(now + Duration::from_millis(40));
                    },
                    "Flick Right ➡️"
                }

                button {
                    id: "btn-reset",
                    class: "px-4 py-2 text-xs font-semibold rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 active:scale-95 transition cursor-pointer flex items-center gap-1.5",
                    onclick: move |_| {
                        g_reset.cancel();
                        x.set(0.0);
                        vel.set(0.0);
                        state.set("Settled".to_string());
                    },
                    "🔄 Reset (0.0)"
                }
            }
        }
    }
}
