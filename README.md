<p align="center">
  <img src="https://raw.githubusercontent.com/techton7/kinetoxus/main/assets/icon.svg" alt="kinetoxus logo" width="160" height="160" />
</p>

<h1 align="center">kinetoxus</h1>

<p align="center">
  <strong>Pure Rust, high-performance physics & timeline motion platform for Dioxus (DOM & WGPU).</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/status-active%20dogfooding-orange.svg" alt="Status: Active Dogfooding" />
  <a href="https://crates.io/crates/kinetoxus"><img src="https://img.shields.io/crates/v/kinetoxus.svg" alt="Crates.io" /></a>
  <a href="LICENSE-MIT"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg" alt="License" /></a>
</p>

---

> [!NOTE]
> **Active Dogfooding Lane**: `kinetoxus` is currently in an active internal dogfooding and boundary-shaping phase across [`kinetocore`](https://github.com/techton7/kinetocore) and [`oxidase`](https://github.com/techton7/oxidase). In accordance with workspace policy, internal dependencies are pinned via reproducible `git + tag` releases ([`kinetocore v0.1.1`](https://github.com/techton7/kinetocore/releases/tag/v0.1.1) and [`oxidase v0.1.4`](https://github.com/techton7/oxidase/releases/tag/oxidase-v0.1.4)). Public crates.io releases and native windowed display-link drivers remain under active development.

**Kinetoxus** is a 100% pure Rust motion facade and multi-target animation platform purpose-built for the [Dioxus](https://dioxuslabs.com) ecosystem.
Operating on top of [`kinetocore`](https://github.com/techton7/kinetocore), it presents a single, ergonomic motion vocabulary (`set`, `from_to`, `to`, `from`, `animate`) tailored for Dioxus components, reactive signals, and interactive UI lifecycles.

Built with **zero JavaScript dependencies**, it runs with shared host/runtime frame timing via [`oxidase`](https://github.com/techton7/oxidase) across both Web and native/headless environments.

## 🧬 Brand & Etymology

- **Kinesis** (Greek for motion) + **Kinetix** + **-oxus** (ecosystem suffix).
- **Biological Motif**: Inspired by the **Kinetochore**, the cellular multi-protein complex that physically attaches to and moves chromosomes along spindle fibers during cell division. Kinetoxus is the physical force engine that moves graphics and UI elements across space and time.

---

## ⚡ Core Architectural Pillars

### 1. 100% Pure Rust Clean-Room Architecture
- **Zero JS Dependencies**: Free of browser runtime animation frameworks, Webflow/GSAP licensing constraints, or JavaScript evaluation overhead.
- **Universal Multi-Platform**: Runs on Web (WASM via high-resolution RAF) and native desktop (deterministic manual stepping for tests, headless, and future native drivers).

### 2. Dioxus Reactive Ergonomics (`SignalTarget<T>`)
- **`use_motion()` Hook**: Component-scoped motion owner with automatic unmount cancellation and memory leak prevention via `use_drop`.
- **`SignalTarget<T>`**: Zero-overhead adapter bridging `Tween<T>` interpolation directly to Dioxus reactive signals (`Signal<T>`).
- **Deterministic Overwrite**: Automatically detects target collisions and cancels prior active animations on the same signal when a new animation is scheduled.

### 3. Shared Frame Driver Architecture (`oxidase::frame`)
- **Web (`wasm32`)**: High-resolution browser `requestAnimationFrame` driven by `oxidase::frame`.
- **Non-WASM / Desktop**: Deterministic manual/headless ticking via `Motion::tick(dt)`, delegating to `oxidase::frame::tick(dt)`. Real hosted native display-link drivers remain explicitly deferred.

---

## 💻 Code Preview (Phase-1 Signal MVP)

```rust
use std::time::Duration;
use dioxus::prelude::*;
use kinetoxus::prelude::*;

#[component]
pub fn AnimatedButton() -> Element {
    let scale = use_signal(|| 1.0f32);
    let opacity = use_signal(|| 1.0f32);
    let motion = use_motion();

    let m1 = motion.clone();
    let m2 = motion;

    rsx! {
        div {
            style: "transform: scale({scale}); opacity: {opacity};",
            button {
                onclick: move |_| {
                    // Smooth bounce scale over 400ms
                    m1.from_to(scale, 0.7f32, 1.0f32, Duration::from_millis(400))
                        .ease(Ease::BounceOut);
                },
                "Bounce"
            }
            button {
                onclick: move |_| {
                    // Immediate value reset
                    m2.set(scale, 1.0f32);
                    m2.set(opacity, 1.0f32);
                },
                "Reset"
            }
        }
    }
}
```

---

## 🗺️ Development Milestones

- [x] **Project Initialization**: Repository scaffold, dual MIT/Apache-2.0 licenses, Release-plz CI setup.
- [x] **Phase-1 (Dioxus Signal MVP)**: `use_motion()`, `motion.set()`, `motion.from_to()`, `SignalTarget<T>`, and Web RAF driver.
- [x] **Phase-2 (Current-Value Tweens)**: `to()`, `from()`, dynamic start-value sampling, and lazy endpoint initialization.
- [ ] **Phase-3 (Timelines & Spring Physics)**: `use_timeline()`, multi-track sequencing, and damped harmonic oscillator springs (`use_spring()`).
- [ ] **Phase-4 (Non-Signal Target Adapters)**: `HandleTarget<T>` and direct buffer-writing adapters for `trioxus` (WGPU 3D) and `nodoxus` (2D node graph).

---

## 📜 License

Dual-licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
