//! Diagnostic example `examples/bisect_card.rs`
//! Renders 5 diagnostic card variants side-by-side in a grid to isolate scaling band triggers.

use dioxus::prelude::*;
use oxidase::prelude::*;

#[oxidase::main]
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_frame(|_| {});
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; font-family: -apple-system, BlinkMacSystemFont, sans-serif; gap: 3rem; background: #0b0f19; color: #f8fafc; padding: 2rem;",
            
            h1 { style: "font-size: 1.5rem; font-weight: 600; color: #38bdf8;", "Kinetoxus Scaling Band Bisect Matrix" }

            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 4rem; align-items: center; justify-items: center; padding: 3rem;",

                // Card 1: Baseline Original
                div {
                    id: "card-1",
                    style: "width: 200px; height: 120px; background: linear-gradient(135deg, #6366f1 0%, #a855f7 100%); border-radius: 16px; box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5); transform: scale(1.2); border: 2px solid rgba(255, 255, 255, 0.1); display: flex; flex-direction: column; align-items: center; justify-content: center;",
                    span { style: "font-weight: 700; font-size: 1rem;", "Card 1" }
                    span { style: "font-size: 0.75rem; opacity: 0.8;", "Baseline" }
                }

                // Card 2: Solid Background
                div {
                    id: "card-2",
                    style: "width: 200px; height: 120px; background: #6366f1; border-radius: 16px; box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5); transform: scale(1.2); border: 2px solid rgba(255, 255, 255, 0.1); display: flex; flex-direction: column; align-items: center; justify-content: center;",
                    span { style: "font-weight: 700; font-size: 1rem;", "Card 2" }
                    span { style: "font-size: 0.75rem; opacity: 0.8;", "Solid BG" }
                }

                // Card 3: Gradient + background-repeat: no-repeat
                div {
                    id: "card-3",
                    style: "width: 200px; height: 120px; background: linear-gradient(135deg, #6366f1 0%, #a855f7 100%); background-repeat: no-repeat; border-radius: 16px; box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5); transform: scale(1.2); border: 2px solid rgba(255, 255, 255, 0.1); display: flex; flex-direction: column; align-items: center; justify-content: center;",
                    span { style: "font-weight: 700; font-size: 1rem;", "Card 3" }
                    span { style: "font-size: 0.75rem; opacity: 0.8;", "No-Repeat" }
                }

                // Card 4: Gradient + No Shadow
                div {
                    id: "card-4",
                    style: "width: 200px; height: 120px; background: linear-gradient(135deg, #6366f1 0%, #a855f7 100%); border-radius: 16px; transform: scale(1.2); border: 2px solid rgba(255, 255, 255, 0.1); display: flex; flex-direction: column; align-items: center; justify-content: center;",
                    span { style: "font-weight: 700; font-size: 1rem;", "Card 4" }
                    span { style: "font-size: 0.75rem; opacity: 0.8;", "No Shadow" }
                }

                // Card 5: Gradient + No Radius
                div {
                    id: "card-5",
                    style: "width: 200px; height: 120px; background: linear-gradient(135deg, #6366f1 0%, #a855f7 100%); border-radius: 0px; box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5); transform: scale(1.2); border: 2px solid rgba(255, 255, 255, 0.1); display: flex; flex-direction: column; align-items: center; justify-content: center;",
                    span { style: "font-weight: 700; font-size: 1rem;", "Card 5" }
                    span { style: "font-size: 0.75rem; opacity: 0.8;", "No Radius" }
                }
            }
        }
    }
}
