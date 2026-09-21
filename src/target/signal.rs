//! Target adapter wrapping a Dioxus [`Signal<T>`].

use dioxus::prelude::{ReadableExt, Signal, WritableExt};
use kinetocore::target::{IntoTargetSampler, Target, TargetSampler};

/// A target adapter wrapping a reactive Dioxus [`Signal<T>`].
///
/// When animated, interpolated values are written directly to the underlying
/// signal via [`SignalTarget::set`], triggering reactive updates in any observing
/// components or effects.
///
/// It also implements [`kinetocore::target::Target`] and [`kinetocore::target::IntoTargetSampler`],
/// allowing it to serve as a dynamic current-value sampler for `to()` and `from()` tweens.
///
/// # Example
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use kinetoxus::target::SignalTarget;
///
/// fn example() {
///     let mut signal = Signal::new(0.0f32);
///     let mut target = SignalTarget::new(signal);
///     target.set(10.0);
///     assert_eq!(signal(), 10.0);
/// }
/// ```
#[derive(Debug)]
pub struct SignalTarget<T: 'static> {
    pub(crate) signal: Signal<T>,
}

impl<T: 'static> Clone for SignalTarget<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static> Copy for SignalTarget<T> {}

impl<T: 'static> PartialEq for SignalTarget<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.signal == other.signal
    }
}

impl<T: 'static> Eq for SignalTarget<T> {}

impl<T: 'static> SignalTarget<T> {
    /// Creates a new `SignalTarget` wrapping the given signal.
    #[inline]
    pub fn new(signal: Signal<T>) -> Self {
        Self { signal }
    }

    /// Returns a copy of the underlying Dioxus [`Signal<T>`].
    #[inline]
    pub fn signal(&self) -> Signal<T> {
        self.signal
    }

    /// Reads and clones the current value of the underlying signal.
    #[inline]
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.signal.cloned()
    }

    /// Directly sets the value of the underlying signal.
    #[inline]
    pub fn set(&self, value: T) {
        let mut sig = self.signal;
        sig.set(value);
    }
}

impl<T: 'static> From<Signal<T>> for SignalTarget<T> {
    #[inline]
    fn from(signal: Signal<T>) -> Self {
        Self::new(signal)
    }
}

impl<T: Clone + 'static> Target<T> for SignalTarget<T> {
    #[inline]
    fn sample(&self) -> T {
        self.signal.cloned()
    }
}

impl<T: Clone + 'static> IntoTargetSampler<T> for SignalTarget<T> {
    #[inline]
    fn into_sampler(self) -> TargetSampler<T> {
        let sig = self.signal;
        TargetSampler::new(move || sig.cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    #[test]
    fn test_signal_target_basic() {
        let mut dom = VirtualDom::new(|| {
            let s = use_signal(|| 10.0f32);
            let target = SignalTarget::from(s);
            assert_eq!(target.signal(), s);
            target.set(42.0);
            assert_eq!(s(), 42.0);
            rsx! {}
        });
        dom.rebuild_in_place();
    }

    #[test]
    fn test_signal_target_sampler() {
        let mut dom = VirtualDom::new(|| {
            let s = use_signal(|| 15.0f32);
            let target = SignalTarget::from(s);

            // Test Target::sample
            assert_eq!(target.sample(), 15.0);
            assert_eq!(target.get(), 15.0);

            // Test IntoTargetSampler
            let sampler = target.into_sampler();
            assert_eq!(sampler.sample(), 15.0);

            // Mutate signal and verify dynamic sampling
            target.set(30.0);
            assert_eq!(sampler.sample(), 30.0);

            rsx! {}
        });
        dom.rebuild_in_place();
    }
}
