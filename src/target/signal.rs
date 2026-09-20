//! Target adapter wrapping a Dioxus [`Signal<T>`].

use dioxus::prelude::{Signal, WritableExt};

/// A target adapter wrapping a reactive Dioxus [`Signal<T>`].
///
/// When animated, interpolated values are written directly to the underlying
/// signal via [`SignalTarget::set`], triggering reactive updates in any observing
/// components or effects.
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
#[derive(Clone, Copy, Debug)]
pub struct SignalTarget<T: 'static> {
    pub(crate) signal: Signal<T>,
}

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
}
