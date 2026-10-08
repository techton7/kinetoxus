use dioxus::prelude::*;

#[component]
pub fn DiagnosticsSection() -> Element {
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
