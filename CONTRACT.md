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
  - Pressure-testing existing public motion verbs: `set`, `from_to`, `to`, `from`.
  - Interruption handling & dynamic start-value sampling.
  - Overwrite/cancel behavior when new motions are dispatched on active signals.
  - Quantitative settle-state verification across multi-signal properties (`x`, `y`, `scale`, `rotation`, `opacity`).
- **Explicitly Out of Scope**:
  - Timeline trees (`use_timeline`).
  - Spring physics hooks (`use_spring`).
  - Asynchronous lifecycle / completion callbacks (`on_complete`, etc. deferred to Phase 5).
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
    - `#hud-fps`: Live frame rate
    - `#hud-x`: Quantitative horizontal offset (`px`)
    - `#hud-y`: Quantitative vertical offset (`px`)
    - `#hud-scale`: Quantitative scale factor
    - `#hud-rot`: Quantitative rotation angle (`°`)
    - `#hud-opacity`: Quantitative opacity value
    - `#hud-status`: Synchronously dispatched action tag (`idle`, `pop`, `slide`, `interrupted_to_center`, `spin`, `fade`, `from_top`, `reset`). Explicit note: this field identifies user intent/dispatch, NOT asynchronous completion callbacks.

- **Detailed Button Behavior & Verb Mapping**:
  - `#btn-pop`: `m_pop.from_to(scale, 0.7, 1.0, 500ms)` with `Ease::BackOut` (exercises `from_to` with overshoot)
  - `#btn-slide`: `m_slide.from_to(x, -150.0, 150.0, 700ms)` (exercises `from_to` across range)
  - `#btn-spin`: `m_spin.from_to(rot, 0.0, 360.0, 600ms)` with `Ease::QuadInOut` (exercises `from_to` rotational interpolation)
  - `#btn-to-center`: `m_center.to(x, 0.0, 400ms)` and `m_center.to(y, 0.0, 400ms)` (exercises `to` mid-flight dynamic sampling and interruption)
  - `#btn-fade`: `m_fade.to(opacity, target, 300ms)` (exercises `to` opacity targeting)
  - `#btn-from-top`: `m_drop.from(y, -200.0, 500ms)` with `Ease::BounceOut` (exercises `from` dynamic current-endpoint latching)
  - `#btn-reset`: `m_reset.set(x, 0.0)`, `m_reset.set(y, 0.0)`, `m_reset.set(scale, 1.0)`, `m_reset.set(rot, 0.0)`, `m_reset.set(opacity, 1.0)` (exercises public `set` verb, canceling active animations and immediately applying defaults)

## 4. Shared Dual-Runner Proof Contract

- **Web Lane**:
  - Command: `dx serve --example interactive_showcase --web`
  - Verification driver: `ego-browser`
  - Observable criteria: rendered alignment, click events, HUD numeric progressions, zero console errors.
- **Native Lane**:
  - Command: `cargo run --example interactive_showcase --features native,blitz-host`
  - Verification driver: `blitz-host` CLI (`inspect`, `mouse click`, `capture`)
  - Observable criteria: out-of-process socket connection, layout bounds, click response, HUD numeric progressions and settle values.
- **Primary Proof**: Semantic DOM and quantitative telemetry HUD numeric values (`#hud-x`, `#hud-y`, `#hud-scale`, `#hud-rot`, `#hud-opacity`).
- **Secondary Proof**: Visual screenshots.
