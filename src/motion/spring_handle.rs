//! Handle for an active spring animation.

use std::cell::RefCell;
use std::rc::Weak;

use crate::driver::Driver;
use crate::motion::MotionInner;

/// Handle for controlling an active spring animation.
#[derive(Clone)]
pub struct SpringHandle {
    pub(crate) id: u64,
    pub(crate) inner: Weak<RefCell<MotionInner>>,
    pub(crate) driver: Driver,
}

impl SpringHandle {
    /// Creates a new `SpringHandle`.
    pub(crate) fn new(id: u64, inner: Weak<RefCell<MotionInner>>, driver: Driver) -> Self {
        Self { id, inner, driver }
    }

    /// Retargets the active spring to a new goal position mid-flight.
    ///
    /// Preserves strict $C^1$ continuity (instantaneous position and velocity are inherited).
    pub fn set_target(&self, new_goal: f64) -> bool {
        if let Some(inner_rc) = self.inner.upgrade() {
            let mut inner = inner_rc.borrow_mut();
            if let Some(anim) = inner.animations.get_mut(&self.id) {
                let success = anim.retarget_spring(new_goal);
                if success {
                    self.driver.ensure_running(&inner_rc);
                    return true;
                }
            }
        }
        false
    }

    /// Cancels this spring animation immediately.
    pub fn cancel(&self) {
        if let Some(inner_rc) = self.inner.upgrade() {
            let mut inner = inner_rc.borrow_mut();
            inner.cancel_id(self.id);
            if inner.active_count() == 0 {
                self.driver.stop();
            }
        }
    }

    /// Returns `true` if this spring animation is currently active.
    pub fn is_active(&self) -> bool {
        if let Some(inner_rc) = self.inner.upgrade() {
            let inner = inner_rc.borrow();
            if let Some(anim) = inner.animations.get(&self.id) {
                return anim.is_active();
            }
        }
        false
    }
}
