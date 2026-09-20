# Kinetoxus Roadmap

This document is the working roadmap for `kinetoxus`.

The repository is currently a scaffold with minimal implementation. This roadmap defines the agreed direction for the crate and should be treated as the live architecture guide until the rest of the repository is updated to match it.

## Mission

- Build `kinetoxus` as the Dioxus-facing motion facade for the ecosystem.
- Present one motion language centered on `set`, `from`, `to`, and `from_to`.
- Support both:
  - signal-based motion for standard Dioxus state
  - non-signal motion for renderer-owned handles and controllers
- Keep `kinetocore` as the source of truth for interpolation, easing, springs, and timeline math.

## Product Position

`kinetoxus` should be understood as a **Dioxus motion platform**, not merely as a thin signal helper crate.

That means:

1. It must feel ergonomic inside Dioxus components and hooks.
2. It must not duplicate the mathematical engine that belongs in `kinetocore`.
3. It should be able to drive both UI state and renderer-owned values with the same motion verbs.
4. It must not become a renderer, a scene graph, or a GPU abstraction layer.

## Strategic Decisions

1. `kinetoxus` is **pure Rust** and does not depend on a JavaScript animation engine.
2. `kinetocore` owns the headless motion contract and numerical engine.
3. `kinetoxus` owns Dioxus ergonomics:
   - hooks
   - lifecycle integration
   - frame drivers
   - target adapters for Dioxus applications
4. `kinetoxus` should **not** be permanently constrained to `Signal<T>` targets only.
5. `trioxus` and future crates should be able to expose motion-ready handles that `kinetoxus` can animate without forcing per-frame root signal churn.
6. Browser-specific helpers may use `oxidase` or equivalent adapter infrastructure at the edge, but browser bridging must remain an adapter concern, not the motion core.

## Ownership Map

| Project | Owns | Must not become |
|---|---|---|
| `kinetocore` | Interpolation contracts, easing, springs, tweens, timelines, generic motion semantics | A Dioxus hook crate |
| `kinetoxus` | Dioxus motion facade, hooks, target adapters, frame lifecycle, consumer ergonomics | A renderer or a replacement for `kinetocore` |
| `trioxus` | Graphics runtime, WGPU state, camera and render handles | A signal-only animation surface |
| `nodoxus` | Graph interactions, layout, virtualization, domain state | A low-level motion core |
| `oxidase` | Browser interop and optional browser-side adapter infrastructure | The animation engine |

## Reference Inputs

`kinetoxus` should use external references deliberately rather than treating them as interchangeable inspirations.

| Reference | Primary value | How to use it | Caution |
|---|---|---|---|
| `gsap` | Behavioral semantics and user-facing motion vocabulary | Use as the reference for `from`, `to`, `fromTo`, `set`, lazy init, `immediateRender`, overwrite rules, timelines, repeats, yoyo, callbacks, and stagger behavior | Do not mirror its JS- and DOM-centric architecture too literally |
| `bevy_tweening` | Rust-native architecture patterns | Use as the reference for typed target access, lens-like abstractions, tween composition, sequences, and playback control | Do not pull in ECS-shaped assumptions that do not fit Dioxus |

The recommended working setup is a local `animation/reference/` lane with isolated checkouts used for study, comparison, and behavior mapping.

## Core Design Principle

**Dioxus-first does not mean signal-only.**

For small UI state transitions, `Signal<T>` is the right first target.

For high-frequency graphics motion, scene controllers, camera handles, and future buffer-oriented targets should be animated without pretending everything is a plain signal update.

The user-facing goal is one motion language:

```text
motion.set(target, value)
motion.from(target, from, duration)
motion.to(target, to, duration)
motion.from_to(target, from, to, duration)
```

The internal goal is not one storage model, but one ergonomic contract.

## Recommended Consumer Experience

The same verbs should feel natural across different target kinds:

```rust
let mut motion = use_motion();

motion.to(opacity_signal, 1.0f32, Duration::from_millis(180));
motion.from_to(scale_signal, 0.96f32, 1.0f32, Duration::from_millis(180));
motion.to(camera_handle, next_pose, Duration::from_secs_f32(1.2));
motion.set(grid_offset_handle, 24.0f32);
```

The exact API may evolve, but the roadmap direction is stable:

- Dioxus users should learn one motion vocabulary.
- Different target families should plug into that vocabulary.
- `kinetoxus` should remain a facade over `kinetocore`, not a fork of it.

## Target Families

| Target family | Purpose | Early priority | Notes |
|---|---|---|---|
| `SignalTarget<T>` | Drive Dioxus `Signal<T>` values | High | First PoC target |
| `HandleTarget<T>` | Drive interior-mutable handles and controllers | High | Needed for `trioxus` and future non-signal consumers |
| `StyleTarget` | Drive DOM style-like properties directly | Medium | Browser adapter lane, likely Web-first |
| `BufferLikeTarget<T>` | Drive numeric arrays or uniform-like values | Medium | Useful for graphics-facing workloads |

## Two Delivery Tracks

### Track A: Fast PoC

This is the shortest path to dogfooding and should happen first.

1. Implement the minimum useful `kinetocore` foundation needed by `kinetoxus`.
2. Expose `set`, `from`, `to`, and `from_to` for `SignalTarget<T>`.
3. Add a Web-first frame driver and Dioxus hook lifecycle management.
4. Dogfood the result in `monoxus` and, when useful, in `oxidase` playground scenarios.
5. Prove interruption, restart, completion, and cleanup behavior in a small real demo.

### Track B: Unified Facade Expansion

This is the recommended long-term direction.

1. Preserve the same verbs while widening the supported target kinds.
2. Introduce non-signal target adapters without breaking the signal-based API.
3. Make `trioxus` a first-class consumer of `kinetoxus` ergonomics.
4. Keep the motion contract rooted in `kinetocore`.
5. Avoid turning the public API into separate signal and graphics dialects.

## Recommendation

Start with **Track A**, but shape the public API so that **Track B** can be added without a conceptual rewrite.

In practice, that means:

- the first real target should be `SignalTarget<T>`
- the verb model should already assume generic targets
- hooks and drivers should be separate from interpolation math
- `trioxus` handle support should be planned early, even if implemented later

## Roadmap Phases

| Phase | Goal | Deliverables | Proof target |
|---|---|---|---|
| 0 | Direction reset | Align docs and crate positioning around pure Rust motion and dual target support | `[contract-implemented]` |
| 1 | Thin facade over `kinetocore` | Reuse core interpolation and tween semantics instead of duplicating math in `kinetoxus` | `[contract-implemented]` |
| 2 | Frame driver and lifecycle | Web RAF driver, cleanup on unmount, and a stable per-component motion owner | `[contract-implemented]` |
| 3 | Signal-based MVP | `set`, `from`, `to`, `from_to` for `SignalTarget<T>` with Dioxus hooks | `[runtime-proven]` in a small app |
| 4 | Timeline and spring hooks | `use_timeline`, `use_spring`, interruptible playback, reverse, seek, and completion hooks | `[runtime-proven]` |
| 5 | Dogfood in `monoxus` | Real transitions on one or more playground surfaces to prove ergonomics | `[runtime-proven]` |
| 6 | Non-signal target support | `HandleTarget<T>` and related adapters for renderer-owned values | `[contract-implemented]` + `[runtime-proven]` |
| 7 | `trioxus` integration | Animate camera, transform, or uniform-like handles through `kinetoxus` verbs | `[runtime-proven]` |
| 8 | Browser-edge adapters | Optional `StyleTarget` or browser-facing motion helpers where they clearly reduce overhead | `[runtime-proven]` |

## Phase Details

### Phase 0 - Direction Reset

- Update the crate's roadmap and architectural narrative.
- Make the signal and non-signal target split explicit.
- Keep the current repository honest about its scaffold status.
- Establish the external reference strategy:
  - `gsap` for semantics
  - `bevy_tweening` for Rust architecture

### Phase 1 - Thin Facade over `kinetocore`

- Do not reimplement easing or interpolation math in `kinetoxus`.
- Consume `kinetocore` primitives and expose Dioxus-oriented ergonomics on top.
- Keep the shared contract obvious enough that future target adapters compose cleanly.

### Phase 2 - Frame Driver and Lifecycle

- Add a motion owner that lives safely inside a Dioxus component lifecycle.
- Start with a Web-first requestAnimationFrame driver.
- Define the native path without pretending it is already solved if it is not.

### Phase 3 - Signal-Based MVP

- Ship the smallest useful user surface:
  - `set`
  - `from`
  - `to`
  - `from_to`
- Support the first practical types before broadening:
  - `f32`
  - `f64`
  - small float arrays
- Prefer correctness and interruption behavior over ambitious API breadth.

### Phase 4 - Timeline and Spring Hooks

- Add higher-level Dioxus hooks once the verb layer is credible.
- Prioritize:
  - play
  - pause
  - reverse
  - seek
  - progress
  - completion callbacks
- Springs should feel native to Dioxus state updates, not bolted on as a separate library.

### Phase 5 - Dogfood in `monoxus`

- Use `monoxus` playgrounds as the first ergonomic proving ground.
- Start with simple surfaces where animation success is easy to judge:
  - tabs
  - accordion or collapsible
  - a dedicated motion demo page
- Avoid coupling too early to primitives whose runtime lifecycle already depends on CSS animation completion semantics unless that coupling is intentional.

### Phase 6 - Non-Signal Target Support

- Add adapters for interior-mutable handles and controllers.
- Keep the same verbs and the same mental model.
- Ensure the implementation does not require fake signals or unnecessary VDOM diff pressure.

### Phase 7 - `trioxus` Integration

- Make `trioxus` the first serious non-signal consumer.
- Prioritize camera and transform handles before more ambitious graphics targets.
- Prove that `kinetoxus` can drive renderer-owned state while `trioxus` remains the renderer owner.

### Phase 8 - Browser-Edge Adapters

- Only add browser-edge direct style writers or related helpers if they materially improve performance or fidelity.
- Keep these adapters optional and boundary-specific.
- Avoid entangling browser plumbing with the core motion contract.

## Suggested Module Layout

```text
src/
  lib.rs
  prelude.rs
  motion/
  driver/
  hooks/
  target/
    signal.rs
    handle.rs
    style.rs
```

The exact structure can change, but the conceptual split should remain:

- math in `kinetocore`
- Dioxus ergonomics in `kinetoxus`
- target-specific adapters in isolated modules

## Dogfood Plan

The preferred validation ladder is:

1. **Small standalone `kinetoxus` example**
   - one value
   - one button
   - one animation lifecycle
2. **`monoxus` motion proof**
   - verify that the verbs feel good in real Dioxus component code
3. **`trioxus` handle proof**
   - verify the same motion language can drive non-signal targets

This sequence keeps the first delivery small without locking the crate into a signal-only future.

## Non-Goals for Early Releases

- Full GSAP feature parity
- A giant choreography DSL before the core verbs are proven
- Owning WGPU renderer responsibilities
- Treating every animation problem as a `Signal<T>` update
- Browser-runtime dependence at the motion-core layer

## Immediate Next Actions

1. Keep this roadmap as the reference for the crate direction.
2. Populate a local `animation/reference/` lane for `gsap` and `bevy_tweening`, then extract the behavior and architecture checkpoints that matter for this crate.
3. Implement the minimum `kinetocore` pieces required for real interpolation and tweens.
4. Build `kinetoxus` around a signal-based MVP without freezing the crate into signal-only architecture.
5. Dogfood the first verbs in `monoxus`.
6. Add the first non-signal target adapter with `trioxus` in mind.
