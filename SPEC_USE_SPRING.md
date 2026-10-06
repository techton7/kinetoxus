# Specification: `use_spring` Lifecycle, Contract, and Scope Locks

## 1. Scalar-First Numeric Contract & Architectural Ownership

- **`kinetocore`**: Owns pure analytical spring physics equations.
  - Types: `Spring`, `SpringConfig`, `sample(t)`, `is_settled(t)`, `retargeted`.
  - Zero framework dependencies; fully testable in pure Rust unit tests.
- **`kinetoxus`**: Owns Dioxus lifecycle bindings, reactive hook ergonomics, frame-loop synchronization via `oxidase::frame`, and target writing via `AnimationTarget`.
- **First-Slice Scope**: Strictly **scalar-first (`f64`)**. Complex vector/struct spring DSL and multi-axis composition are explicitly deferred.

---

## 2. Input Model vs. Output Model

- **Inputs**: Spring goal position (`goal: f64`) and optional physical configuration (`config: SpringConfig`).
- **Outputs**: Target writes (`target.write_value(sampled_position)`).
- **Invariant**: Targets are outputs, NOT implicit controllers. Retargeting is triggered by explicit goal or config updates, **NEVER** by observing passive mutations on the target storage.

---

## 3. Frame-Loop Lifecycle, Active State Tracking & Settle Clamping

1. **Registration**: Registers with `oxidase::frame` upon activation. Active spring state tracks start timestamp (`Instant`).
2. **Frame Ticks**: On each RAF/frame tick, evaluates `elapsed = now - start_time`, and calls `spring.sample(elapsed)`.
3. **Target Writing**: Writes sampled value to target (`target.write_value(state.position)`).
4. **Settle Condition**: Checks `spring.is_settled(elapsed)`.
5. **Exact Settle Clamping**: When settled, clamps the target value to the exact `goal` (`target.write_value(spring.target_position())`), preventing epsilon-offset drift.
6. **Runtime Dormancy**: Stops frame-loop ticking and unregisters active spring state immediately once settled. Zero lingering frame loop overhead or unnecessary VDOM re-renders while at rest.

---

## 4. Dynamic Retargeting & Continuity

When consumer calls `.set_target(new_goal)` or reactive goal signal updates mid-flight:
1. Sample instantaneous position and velocity from current active spring at current `elapsed`.
2. Construct new spring with `initial_position = current.position`, `initial_velocity = current.velocity`, `target = new_goal`.
3. Reset `start_time = now`.
4. **Guarantees strict $C^1$ continuity** (no velocity pop or positional jump).

---

## 5. Target Compatibility

- Works seamlessly with both `SignalTarget<f64>` and `HandleTarget<f64>` via `AnimationTarget<f64>`.
- **Invariant**: "No Fake Signal Bridge for HandleTarget". Updates to `HandleTarget` write directly to `RefCell` memory without roundtripping through fake reactive signals.

---

## 6. Phase Scope Locks

- **In Scope**: Scalar `f64` `use_spring`, `Motion::spring`, frame-loop lifecycle, settle clamping, runtime dormancy, `SignalTarget` and `HandleTarget` compatibility, retargeting.
- **Explicitly Out of Scope**: Multi-axis/vector DSL, timeline sequencing, completion callbacks beyond existing primitives, arbitrary mutation observation.

---

## 7. Validation & Verification

- Markdown formatting validated via `verify-markdown` without frontmatter requirements.
