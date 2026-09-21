//! Frame driver abstraction for ticking active animations.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use oxidase::frame::{start_frame_loop, FrameInfo, FrameLoopGuard};

use crate::motion::MotionInner;

/// The driving mechanism used to advance animations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverKind {
    /// Web browser `requestAnimationFrame` driver via `oxidase::frame`.
    WebRaf,
    /// Manual and headless ticking via `oxidase::frame::tick(dt)`.
    Manual,
}

#[derive(Default)]
struct DriverState {
    running: Cell<bool>,
    guard: RefCell<Option<FrameLoopGuard>>,
}

/// Unified driver handle managing active frame ticking through `oxidase::frame`.
#[derive(Clone)]
pub struct Driver {
    state: Rc<DriverState>,
}

impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
}

impl Driver {
    /// Creates a new shared frame driver instance.
    pub fn new() -> Self {
        Self {
            state: Rc::new(DriverState::default()),
        }
    }

    /// Ensures the shared frame loop is running if there are active animations.
    pub fn ensure_running(&self, motion_inner: &Rc<RefCell<MotionInner>>) {
        if self.state.running.replace(true) {
            return;
        }

        let weak_inner = Rc::downgrade(motion_inner);
        let state = self.state.clone();

        match start_frame_loop(move |info: FrameInfo| {
            let Some(inner) = weak_inner.upgrade() else {
                state.running.set(false);
                let _ = state.guard.borrow_mut().take();
                return;
            };

            let has_more = inner.borrow_mut().tick(info.delta);
            if !has_more {
                state.running.set(false);
                let _ = state.guard.borrow_mut().take();
            }
        }) {
            Ok(guard) => {
                *self.state.guard.borrow_mut() = Some(guard);
            }
            Err(_) => {
                self.state.running.set(false);
            }
        }
    }

    /// Stops the shared frame loop immediately.
    pub fn stop(&self) {
        self.state.running.set(false);
        let _ = self.state.guard.borrow_mut().take();
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
