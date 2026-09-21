//! # Kinetoxus
//!
//! Pure Rust, high-performance physics & timeline motion platform for Dioxus (DOM & WGPU).
//!
//! ## Overview
//!
//! `kinetoxus` is the Dioxus-facing motion facade built on top of [`kinetocore`].
//! It delivers an ergonomic motion vocabulary (`set`, `from_to`, `to`, `from`, `animate`) tailored for
//! Dioxus components, reactive signals ([`SignalTarget`]), and interactive UI lifecycles.
//!
//! ### Core Features
//!
//! - **[`use_motion()`]**: Component-scoped motion hook with automatic unmount cancellation.
//! - **[`Motion`]**: Motion controller supporting immediate `set()`, deterministic `from_to()`, and dynamic `to()` / `from()`.
//! - **[`SignalTarget`]**: Zero-overhead adapter bridging tweens to reactive Dioxus [`dioxus::prelude::Signal`], supporting dynamic current-value sampling.
//! - **Shared Frame Ownership via [`oxidase`]**:
//!   - **Web (`wasm32`)**: Hosted `requestAnimationFrame` via `oxidase::frame`.
//!   - **Non-WASM / Desktop**: Deterministic manual/headless ticking via [`Motion::tick`] and `oxidase::frame::tick(dt)`.
//! - **Full [`kinetocore`] Integration**: Seamless access to 31 Robert Penner [`Ease`] curves, repeats, yoyo, and direction control.

#![warn(missing_docs)]

pub mod driver;
pub mod hooks;
pub mod motion;
pub mod prelude;
pub mod target;

// Re-export primary public types at crate root
pub use driver::DriverKind;
pub use hooks::use_motion;
pub use motion::{Motion, MotionHandle};
pub use target::SignalTarget;

// Re-export kinetocore engine and easing for convenience
pub use kinetocore as core;
pub use kinetocore::ease::Ease;
pub use kinetocore::easing;
pub use kinetocore::interpolate::{lerp, Interpolate};
pub use kinetocore::repeat::{RepeatCount, RepeatStrategy};
pub use kinetocore::state::TweenEndpoints;
pub use kinetocore::target::{IntoTargetSampler, Target, TargetSampler};
pub use kinetocore::tween::Tween;

/// Current package version of kinetoxus.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_easing_reexport() {
        use kinetocore::easing::Easing;
        let v = easing::Quad::ease_in_out(0.5f32, 0.0, 100.0, 1.0);
        assert!((v - 50.0).abs() < 1e-4);
    }
}
