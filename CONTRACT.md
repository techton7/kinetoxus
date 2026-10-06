# Kinetoxus Phase 4 Contract, Substrate Truth-Sync, and Shared Dual-Runner Proof Specification

## 1. Substrate Truth-Sync

- **Runtime Ownership (`oxidase`)**:
  - `oxidase` is established as the cross-host runtime and frame owner (`use_frame`, `next_frame`, VSync loop, `#[oxidase::main]`).
  - No hardcoded release version strings; runtime ownership stays strictly in `oxidase`.
- **Motion Facade (`kinetoxus`)**:
  - `kinetoxus` is the motion facade owning motion semantics, `use_motion()`, `Motion`, and `SignalTarget<T>`.
  - Integrates seamlessly with `oxidase` VSync ticks and reactive signal updates.

## 2. Phase 4 Scope Locks

- **In Scope**:
  - Pressure-testing existing motion verbs: `set`, `from_to`, `to`, `from`.
  - Interruption handling.
  - Current-value dynamic sampling.
  - Overwrite/cancel behavior.
  - Settle-state accuracy across multi-signal properties (`x`, `y`, `scale`, `rotation`, `opacity`).
- **Explicitly Out of Scope**:
  - Timeline trees.
  - Spring hooks.
  - Non-signal targets (`HandleTarget`).
  - Direct DOM mutating shims.

## 3. Scenario Specification: `interactive_showcase`

- **Location**: `examples/interactive_showcase.rs`
- **Semantic DOM Element IDs**:
  - Card target: `#animated-card`
  - Action buttons:
    - `#btn-pop`
    - `#btn-slide`
    - `#btn-spin`
    - `#btn-to-center`
    - `#btn-fade`
    - `#btn-from-top`
    - `#btn-reset`
  - Telemetry HUD:
    - `#hud-fps`
    - `#hud-x`
    - `#hud-y`
    - `#hud-scale`
    - `#hud-rot`
    - `#hud-opacity`
    - `#hud-status`

- **Detailed Button Behavior**:
  - `#btn-pop`: `m_pop.from_to(scale, 0.7, 1.0, 500ms)` with `Ease::BackOut`
  - `#btn-slide`: `m_slide.from_to(x, -150.0, 150.0, 700ms)`
  - `#btn-spin`: `m_spin.from_to(rot, 0.0, 360.0, 600ms)` with `Ease::QuadInOut`
  - `#btn-to-center`: `m_center.to(x, 0.0, 400ms)` and `m_center.to(y, 0.0, 400ms)` (interruption test)
  - `#btn-fade`: `m_fade.to(opacity, 0.2, 300ms)` then toggles back to 1.0
  - `#btn-from-top`: `m_drop.from(y, -200.0, 500ms)` with `Ease::BounceOut`
  - `#btn-reset`: `m_reset.set(...)` all signals to defaults (`x=0, y=0, scale=1, rot=0, opacity=1`) and cancels active motions.

## 4. Shared Dual-Runner Proof Contract

- **Web Lane**:
  - Command: `dx serve --example interactive_showcase --web`
  - Verification driver: `ego-browser`
  - Observable criteria: rendered alignment, click events, HUD updates, zero console errors.
- **Native Lane**:
  - Command: `cargo run --example interactive_showcase --features native,blitz-host`
  - Verification driver: `blitz-host` CLI (`inspect`, `mouse click`, `capture`)
  - Observable criteria: out-of-process socket connection, layout bounds, click response, HUD progression and settle values.
- **Primary Proof**: Semantic DOM and telemetry HUD values.
- **Secondary Proof**: Visual screenshots.
