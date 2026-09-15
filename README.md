<p align="center">
  <img src="https://raw.githubusercontent.com/techton7/kinetoxus/main/assets/icon.svg" alt="kinetoxus logo" width="160" height="160" />
</p>

<h1 align="center">kinetoxus</h1>

<p align="center">
  <strong>Pure Rust, high-performance physics & timeline motion platform for Dioxus (DOM & WGPU).</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/kinetoxus"><img src="https://img.shields.io/crates/v/kinetoxus.svg" alt="Crates.io" /></a>
  <a href="https://docs.rs/kinetoxus"><img src="https://docs.rs/kinetoxus/badge.svg" alt="docs.rs" /></a>
  <a href="LICENSE-MIT"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg" alt="License" /></a>
</p>

---

**Kinetoxus** is a 100% pure Rust motion engine and multi-track timeline orchestrator purpose-built for the [Dioxus](https://dioxuslabs.com) ecosystem.

Built with **zero JavaScript dependencies**, it operates with nanosecond numerical interpolation precision across both native Blitz desktop and Web (WASM) environments.

## 🧬 Brand & Etymology

- **Kinesis** (Greek for motion) + **Kinetix** + **-oxus** (ecosystem suffix).
- **Biological Motif**: Inspired by the **Kinetochore**, the cellular multi-protein complex that physically attaches to and moves chromosomes along spindle fibers during cell division. Kinetoxus is the physical force engine that moves graphics and UI elements across space and time.

---

## 🗺️ Ecosystem Synergy Map

```text
                     ┌─────────────────────────────────────────┐
                     │   kinetoxus (Timeline & Motion Engine)  │
                     └────────────────────┬────────────────────┘
                                          │ (Nanosecond Numerical Interpolation)
       ┌──────────────────────────────────┼──────────────────────────────────┐
       ▼                                  ▼                                  ▼
[shadcn-dioxus / monoxus]            [nodoxus (Node Engine)]         [trioxus (WGPU Viewport)]
- Fluid modal & drawer springs        - Sugiyama layout sliding       - 3D camera flight & orbits
- Accordion & tab transitions        - Edge neon pulse flows         - Exploded assembly sequences
- Scroll & gesture tracking           - Viewport FitView gliding      - Lighting, materials, opacity
```

---

## ⚡ Core Architectural Pillars

### 1. 100% Pure Rust Clean-Room Architecture
- **Zero JS Dependencies**: Completely free of browser runtime dependencies, Webflow/GSAP licensing constraints, or FFI overhead.
- **Universal Multi-Platform**: Runs with identical performance on Blitz native desktop (via Metal, Vulkan, DirectX) and Web (via WebGPU / WASM).

### 2. Multi-Target Motion Pipeline (DOM vs WGPU)
```text
[ kinetoxus Timeline & Motion Sequencer ]
                   │ (Time t Easing & Spring Physics Calculation)
         ┌─────────┴─────────┐
         ▼                   ▼
[ Target A: Dioxus RSX DOM ] [ Target B: WGPU Graphic Primitives ]
- HTML / Blitz CSS styles     - nodoxus: Node coordinates (f32, f32), edge dash offsets
- transform, opacity, width   - trioxus: 3D camera [f32; 3], PBR mesh rotation
```
- **Target A (DOM / Signal)**: Real-time updates to Dioxus reactive signals and component styles.
- **Target B (WGPU Direct Gliding)**: Directly mutates numeric arrays (`[f32; N]`) to write directly to GPU buffers (`queue.write_buffer`) with **zero VDOM diffing overhead**, keeping UI components dormant while the GPU renders at 120fps.

### 3. Dual Mathematics Engine
- **Robert Penner Easing Formulas** (via [`easer`]): Time-tested mathematical ease-in, ease-out, bounce, and elastic curves.
- **Damped Harmonic Oscillator** (Spring Physics): Velocity-based physics that naturally handles gesture interrupts, flicks, and velocity preservation without snapping.

### 4. Blender-Style Timeline Control
- `.seek(seconds: f64)`: Instant jump to a timestamp.
- `.progress(ratio: f64)`: 0.0 to 1.0 ratio scrubbing (ideal for slider/scrollbar sync).
- `.reverse()`, `.time_scale(rate: f64)`: Smooth backward playback and speed scaling.
- `use_timeline_scrubber`: Two-way synchronization with Dioxus reactive signals.

---

## 💻 Code Preview (Target Ergonomics)

```rust
use dioxus::prelude::*;
use kinetoxus::prelude::*;

#[component]
pub fn AnimatedScene() -> Element {
    let mut timeline = use_timeline();

    let on_start = move |_| {
        timeline.new_sequence()
            // Animate WGPU camera coordinates directly
            .to(&mut camera_pos, [0.0, 5.0, 10.0], Duration::from_secs_f32(1.5))
            .ease(Ease::ElasticOut)
            .stagger(Duration::from_millis(50))
            .on_update(move || {
                trioxus_ctx.request_render();
            })
            .play();
    };

    rsx! {
        button { onclick: on_start, "Play Cinematic Flight" }
    }
}
```

---

## 🗺️ Development Milestones

- [x] **Project Initialization**: Repository scaffold, dual MIT/Apache-2.0 licenses, Release-plz CI setup.
- [ ] **M1 (Math & Interpolation Core)**: Penner easing integration and spring physics numerical interpolators.
- [ ] **M2 (Tween & Timeline Engine)**: Time-based track sequencer, `play()`, `reverse()`, and `seek()` controls.
- [ ] **M3 (Dioxus Signal & DOM Binding)**: `use_timeline()` hook for Dioxus components and reactive signals.
- [ ] **M4 (WGPU Direct Buffer Binding)**: Zero-copy direct buffer updating adapter for `trioxus` and `nodoxus`.

---

## 📜 License

Dual-licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
