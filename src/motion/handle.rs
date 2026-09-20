//! Handle to an active animation.

use std::cell::RefCell;
use std::rc::Weak;

use kinetocore::direction::PlaybackDirection;
use kinetocore::ease::Ease;
use kinetocore::repeat::{RepeatCount, RepeatStrategy};

use crate::motion::MotionInner;

/// Handle to an active animation, allowing fluent configuration and explicit cancellation.
///
/// Returned by [`crate::motion::Motion::from_to`] and [`crate::motion::Motion::animate`].
#[derive(Clone, Debug)]
pub struct MotionHandle {
    pub(crate) id: u64,
    pub(crate) inner: Weak<RefCell<MotionInner>>,
}

impl MotionHandle {
    /// Returns the unique numeric ID assigned to this animation.
    #[inline]
    pub fn id(&self) -> u64 {
        self.id
    }

    fn with_anim_mut<F>(&self, f: F)
    where
        F: FnOnce(&mut dyn crate::motion::animation::ActiveAnimation),
    {
        if let Some(inner) = self.inner.upgrade() {
            let mut inner = inner.borrow_mut();
            if let Some(anim) = inner.animations.get_mut(&self.id) {
                f(&mut **anim);
            }
        }
    }

    /// Sets the easing curve of the active animation.
    pub fn ease(self, ease: Ease) -> Self {
        self.with_anim_mut(|anim| anim.set_ease(ease));
        self
    }

    /// Sets the repeat count and strategy of the active animation.
    pub fn repeat(self, count: impl Into<RepeatCount>, strategy: RepeatStrategy) -> Self {
        let count = count.into();
        self.with_anim_mut(|anim| anim.set_repeat(count, strategy));
        self
    }

    /// Enables or disables mirrored yoyo repeat.
    pub fn yoyo(self, yoyo: bool) -> Self {
        self.with_anim_mut(|anim| anim.set_yoyo(yoyo));
        self
    }

    /// Sets the playback direction of the active animation.
    pub fn direction(self, direction: PlaybackDirection) -> Self {
        self.with_anim_mut(|anim| anim.set_direction(direction));
        self
    }

    /// Cancels this animation immediately.
    pub fn cancel(&self) {
        if let Some(inner) = self.inner.upgrade() {
            inner.borrow_mut().cancel_id(self.id);
        }
    }

    /// Returns `true` if this animation is still actively running.
    pub fn is_active(&self) -> bool {
        self.inner.upgrade().is_some_and(|inner| {
            inner
                .borrow()
                .animations
                .get(&self.id)
                .is_some_and(|a| a.is_active())
        })
    }
}
