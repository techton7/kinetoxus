//! Target adapter wrapping an `Rc<RefCell<T>>`.

use std::rc::Rc;
use std::cell::RefCell;
use kinetocore::target::{IntoTargetSampler, Target, TargetSampler};

/// A target adapter wrapping an `Rc<RefCell<T>>`.
///
/// When animated, interpolated values are written directly to the underlying
/// `RefCell` via [`HandleTarget::set`], enabling pure headless operation without
/// requiring a Dioxus `VirtualDom` or reactive signals.
///
/// It also implements [`kinetocore::target::Target`] and [`kinetocore::target::IntoTargetSampler`],
/// allowing it to serve as a dynamic current-value sampler for `to()` and `from()` tweens.
#[derive(Debug)]
pub struct HandleTarget<T: 'static> {
    pub(crate) handle: Rc<RefCell<T>>,
}

impl<T: 'static> Clone for HandleTarget<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

impl<T: 'static> PartialEq for HandleTarget<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.handle, &other.handle)
    }
}

impl<T: 'static> Eq for HandleTarget<T> {}

impl<T: 'static> HandleTarget<T> {
    /// Creates a new `HandleTarget` wrapping the given shared reference cell.
    #[inline]
    pub fn new(handle: Rc<RefCell<T>>) -> Self {
        Self { handle }
    }

    /// Returns a reference to the underlying `Rc<RefCell<T>>`.
    #[inline]
    pub fn handle(&self) -> &Rc<RefCell<T>> {
        &self.handle
    }

    /// Reads and clones the current value inside the `RefCell`.
    #[inline]
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.handle.borrow().clone()
    }

    /// Directly sets the value inside the `RefCell`.
    #[inline]
    pub fn set(&self, value: T) {
        *self.handle.borrow_mut() = value;
    }
}

impl<T: 'static> From<Rc<RefCell<T>>> for HandleTarget<T> {
    #[inline]
    fn from(handle: Rc<RefCell<T>>) -> Self {
        Self::new(handle)
    }
}

impl<T: Clone + 'static> Target<T> for HandleTarget<T> {
    #[inline]
    fn sample(&self) -> T {
        self.handle.borrow().clone()
    }
}

impl<T: Clone + 'static> IntoTargetSampler<T> for HandleTarget<T> {
    #[inline]
    fn into_sampler(self) -> TargetSampler<T> {
        let handle = self.handle.clone();
        TargetSampler::new(move || handle.borrow().clone())
    }
}

impl<T: Clone + 'static> crate::target::AnimationTarget<T> for HandleTarget<T> {
    #[inline]
    fn write_value(&self, value: T) {
        self.set(value);
    }

    #[inline]
    fn is_target_equal(&self, other: &dyn std::any::Any) -> bool {
        if let Some(other_target) = other.downcast_ref::<HandleTarget<T>>() {
            return self == other_target;
        }
        if let Some(other_rc) = other.downcast_ref::<Rc<RefCell<T>>>() {
            return Rc::ptr_eq(&self.handle, other_rc);
        }
        false
    }
}

impl<T: Clone + 'static> crate::target::IntoAnimationTarget<T> for HandleTarget<T> {
    type Target = HandleTarget<T>;

    #[inline]
    fn into_target(self) -> Self::Target {
        self.clone()
    }
}

impl<T: Clone + 'static> crate::target::IntoAnimationTarget<T> for &HandleTarget<T> {
    type Target = HandleTarget<T>;

    #[inline]
    fn into_target(self) -> Self::Target {
        self.clone()
    }
}

impl<T: Clone + 'static> crate::target::IntoAnimationTarget<T> for Rc<RefCell<T>> {
    type Target = HandleTarget<T>;

    #[inline]
    fn into_target(self) -> Self::Target {
        HandleTarget::new(self)
    }
}

impl<T: Clone + 'static> crate::target::IntoAnimationTarget<T> for &Rc<RefCell<T>> {
    type Target = HandleTarget<T>;

    #[inline]
    fn into_target(self) -> Self::Target {
        HandleTarget::new(self.clone())
    }
}


/// A 2D transform pose controller wrapping shared interior-mutable handles
/// for position (`x`, `y`), scale, and rotation.
#[derive(Debug, Clone)]
pub struct Transform2D {
    /// X translation coordinate.
    pub x: Rc<RefCell<f32>>,
    /// Y translation coordinate.
    pub y: Rc<RefCell<f32>>,
    /// Scale factor.
    pub scale: Rc<RefCell<f32>>,
    /// Rotation angle in degrees.
    pub rotation: Rc<RefCell<f32>>,
}

impl Default for Transform2D {
    #[inline]
    fn default() -> Self {
        Self {
            x: Rc::new(RefCell::new(0.0)),
            y: Rc::new(RefCell::new(0.0)),
            scale: Rc::new(RefCell::new(1.0)),
            rotation: Rc::new(RefCell::new(0.0)),
        }
    }
}

impl Transform2D {
    /// Creates a new `Transform2D` with specified coordinates, scale, and rotation.
    #[inline]
    pub fn new(x: f32, y: f32, scale: f32, rotation: f32) -> Self {
        Self {
            x: Rc::new(RefCell::new(x)),
            y: Rc::new(RefCell::new(y)),
            scale: Rc::new(RefCell::new(scale)),
            rotation: Rc::new(RefCell::new(rotation)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_target_basic() {
        let r = Rc::new(RefCell::new(10.0f32));
        let target = HandleTarget::from(r.clone());
        assert_eq!(target.handle(), &r);
        target.set(42.0);
        assert_eq!(*r.borrow(), 42.0);
        assert_eq!(target.get(), 42.0);
    }

    #[test]
    fn test_handle_target_sampler() {
        let r = Rc::new(RefCell::new(15.0f32));
        let target = HandleTarget::from(r.clone());

        // Test Target::sample
        assert_eq!(target.sample(), 15.0);
        assert_eq!(target.get(), 15.0);

        // Test IntoTargetSampler
        let sampler = target.into_sampler();
        assert_eq!(sampler.sample(), 15.0);

        // Mutate refcell and verify dynamic sampling
        *r.borrow_mut() = 30.0;
        assert_eq!(sampler.sample(), 30.0);
    }
}
