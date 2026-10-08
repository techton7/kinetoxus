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

use dioxus::prelude::*;
use oxidase::prelude::*;

mod card_stack;
mod diagnostics;
mod handles;
mod interruption;
mod spring;
mod timeline;
mod tweens;

use card_stack::CardStackSection;
use diagnostics::DiagnosticsSection;
use handles::HandlesSection;
use interruption::InterruptionSection;
use spring::SpringSection;
use timeline::TimelineSection;
use tweens::TweensSection;

const TAILWIND_CSS: &str = include_str!("../../assets/tailwind.css");

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
