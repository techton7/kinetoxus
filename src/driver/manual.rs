//! Manual and headless frame driver for non-WASM targets.

use std::cell::RefCell;
use std::rc::Rc;

use crate::motion::MotionInner;

/// Manual and headless frame driver for non-WASM targets.
///
/// Native background frame driving (e.g. display link or winit integration)
/// is deferred to a future milestone. This driver provides safe, no-op
/// lifecycle management; animations are deterministically advanced via [`crate::motion::Motion::tick`].
#[derive(Debug, Default, Clone)]
pub struct ManualDriver;

impl ManualDriver {
    /// Creates a new `ManualDriver`.
    pub fn new() -> Self {
        Self
    }

    /// In manual mode, starting the driver is a harmless no-op.
    pub fn ensure_running(&self, _inner: &Rc<RefCell<MotionInner>>) {}

    /// In manual mode, stopping the driver is a harmless no-op.
    pub fn stop(&self) {}
}
