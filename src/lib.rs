//! # Kinetoxus
//!
//! Pure Rust, high-performance physics & timeline motion platform for Dioxus (DOM & WGPU).
//!
//! ## Overview
//!
//! `kinetoxus` is a 100% pure Rust motion engine and multi-track timeline orchestrator designed for Dioxus.
//! Built with zero JavaScript dependencies, it operates with nanosecond numerical interpolation precision
//! across both native Blitz desktop and Web (WASM) environments.
//!
//! ### Key Pillars
//!
//! - **Multi-Target Motion Pipeline**:
//!   - **Target A (DOM / Signal)**: Fluidly updates Dioxus reactive signals and component CSS styles.
//!   - **Target B (WGPU Graphic Primitives)**: Directly mutates numeric arrays (`[f32; N]`) to update
//!     WGPU uniform and instance buffers (`queue.write_buffer`) with zero VDOM diffing overhead.
//! - **Dual Mathematics Engine**:
//!   - Industry-standard **Robert Penner Easing** formulas via [`easer`].
//!   - Velocity-based **Damped Harmonic Oscillator** (Spring Physics) for natural, interruptible UI gestures.
//! - **Multi-Track Timeline**: First-class scrubbing (`seek`, `progress`, `reverse`, `time_scale`) across
//!   DOM components, 2D node graphs ([`nodoxus`]), and 3D scenes ([`trioxus`]).

#![warn(missing_docs)]

/// Re-export easing functions from easer.
pub use easer::functions as easing;

/// Early scaffold version of kinetoxus.
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
        use easer::functions::Easing;
        let v = easing::Quad::ease_in_out(0.5f32, 0.0, 100.0, 1.0);
        assert!((v - 50.0).abs() < 1e-4);
    }
}
