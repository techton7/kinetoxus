//! Frame driver abstraction for ticking active animations.

use std::cell::RefCell;
use std::rc::Rc;

use crate::motion::MotionInner;

#[cfg(not(target_arch = "wasm32"))]
mod manual;
#[cfg(target_arch = "wasm32")]
mod wasm_raf;

#[cfg(not(target_arch = "wasm32"))]
pub use manual::ManualDriver as PlatformDriver;
#[cfg(target_arch = "wasm32")]
pub use wasm_raf::WebRafDriver as PlatformDriver;

/// The driving mechanism used to advance animations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverKind {
    /// Web browser `requestAnimationFrame` driver.
    WebRaf,
    /// Manual and headless driver. Real native driver is deferred to a future milestone.
    Manual,
}

/// Unified driver handle managing active frame ticking.
#[derive(Clone)]
pub struct Driver {
    inner: Rc<PlatformDriver>,
}

impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
}

impl Driver {
    /// Creates a new platform driver instance.
    pub fn new() -> Self {
        Self {
            inner: Rc::new(PlatformDriver::new()),
        }
    }

    /// Ensures the driver is running if there are active animations.
    pub fn ensure_running(&self, motion_inner: &Rc<RefCell<MotionInner>>) {
        self.inner.ensure_running(motion_inner);
    }

    /// Stops the frame driver.
    pub fn stop(&self) {
        self.inner.stop();
    }

    /// Returns the active driver kind.
    #[inline]
    pub fn kind(&self) -> DriverKind {
        #[cfg(target_arch = "wasm32")]
        {
            DriverKind::WebRaf
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            DriverKind::Manual
        }
    }
}
