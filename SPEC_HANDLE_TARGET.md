# Specification: Non-Signal Target Handle Abstraction (`HandleTarget<T>`)

## 1. Non-Signal Target Read/Write Synchronization & Ownership Model

### 1.1 Concept Definition
`HandleTarget<T>` is an interior-mutable storage primitive designed for high-frequency imperative and declarative animations in Dioxus and rendering subsystems without relying on Dioxus reactive signals (`Signal<T>`). 
- **Storage Backend**: Canonical `Rc<RefCell<T>>` for same-thread UI/component lifetime.
- **Purpose**: Enables low-overhead, zero-overhead-reactivity reads and writes during animation ticks, avoiding unnecessary component re-renders or reactive graph invalidations for high-frequency motion properties (e.g., 60fps/120fps transforms, layout values, or camera properties).

### 1.2 Explicit Invariant: "No Fake Signal Bridge"
> **CRITICAL INVARIANT**: `HandleTarget<T>` must NEVER route writes through a hidden or proxy Dioxus `Signal<T>`.

- Updates write directly to the underlying `RefCell<T>` storage.
- Animation frames sample and mutate the target directly via direct memory access or borrow guards (`borrow_mut()`), preventing reactive cycle overhead, scheduler latency, and signal tracking corruption.

### 1.3 Memory and Borrowing Model
- **Ownership**: The handle holds an `Rc<RefCell<T>>`. Clones of `HandleTarget<T>` share ownership of the same underlying storage.
- **Accessors**:
  - `get() -> T` (where `T: Copy`) or `with<F, R>(&self, f: F) -> R` for non-Copy types.
  - `set(&self, val: T)`: Mutates the inner value directly.
  - `try_borrow_mut(&self)` / internal borrow guards for interpolator tick updates.

---

## 2. Motion Verb Parity & Semantic Rules

`HandleTarget<T>` supports core motion verbs mirroring robust animation engines (such as GSAP / Web Animations API semantics):

| Verb | Semantics |
| :--- | :--- |
| **`set`** | Immediately writes target value to the handle, canceling any active tweens targeting the same handle. |
| **`from_to`** | Immediately assigns the `from` value to the handle, then interpolates smoothly to `to` over the specified duration and easing curve. |
| **`to`** | Dynamically samples the current handle value at animation start time as `from`, interpolating smoothly to `to` over duration and easing. |
| **`from`** | Dynamically samples the current handle value as the destination `to`, immediately assigns `from` to the handle, and interpolates back to the current value (`to`). |

### 2.1 Interruption & Retargeting
- When a new animation verb (`to`, `from_to`, etc.) is dispatched on an active handle:
  1. Any existing active tween or animation task driving the handle is cancelled or superseded.
  2. The current in-flight value is sampled directly from the handle storage.
  3. A new animation timeline is initialized starting from the sampled value toward the new destination, preventing visual pops or discontinuities.

### 2.2 Target Identity & Collision Detection
- **Identity**: Targets are identified by their unique handle instance pointer (`Rc::as_ptr` or a unique `HandleId` generation counter).
- **Collision / Overwrite**: When multiple animations target the same handle concurrently, the most recently spawned animation takes exclusive write ownership, safely overriding previous tweens.

---

## 3. First Proof-Consumer Handle Shape Selection

### 3.1 Selected Shape: `Transform2D` Handle
For the initial proof consumer of `HandleTarget<T>`, we select a 2D transform struct:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform2D {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub rotation: f32, // in radians or degrees
}
```

### 3.2 Rationale & Workload Representation
- **Representative Scope**: `Transform2D` captures translation (`x`, `y`), scaling (`scale`), and rotation (`rotation`), representing standard 2D UI, canvas, and scene-graph manipulation.
- **Decoupling**: It mirrors real-world graphics, renderer, and controller workloads (such as Camera / Transform handles in `trioxus` / `tricore`) while remaining entirely self-contained and free of complex WGPU or rendering pipeline dependencies.
- **Interpolatability**: Each field is an `f32`, allowing straightforward linear and eased interpolation implementations (`Lerp` trait).

---

## 4. Phase Scope Locks

### 4.1 In Scope
- `HandleTarget<T>` struct definition and shared storage model (`Rc<RefCell<T>>`).
- Enforcement of the "No Fake Signal Bridge" invariant.
- Motion verb parity: `set`, `from_to`, `to`, `from`.
- Dynamic value sampling at start/interruption time.
- Overwrite and cancellation mechanics for active animations.
- `Transform2D` concrete shape implementation and unit test proof consumer.

### 4.2 Explicitly Out of Scope
- Timeline composition trees (`use_timeline`).
- Physics-based spring simulations (`use_spring`).
- Lifecycle completion callbacks (`on_complete`, `on_update` hooks).
- Multi-threaded thread-safe synchronization (`Arc<RwLock<T>>`).
- Full renderer scene graph integration or WGPU pipeline bindings.

---

## 5. Validation
- Markdown format validated via `verify-markdown.sh`.
