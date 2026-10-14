//! Component lifecycle hook for gesture continuation.

use dioxus::prelude::*;

use crate::gesture::{GestureContinuationConfig, GestureContinuationController};
use crate::target::IntoAnimationTarget;

/// Creates and binds a [`GestureContinuationController`] to the current Dioxus component scope.
///
/// Binds `target` to a gesture controller, automatically registering `use_drop` cleanup
/// to cancel active animations and frame loop subscriptions upon component unmount.
///
/// Supports binding to reactive Dioxus [`Signal<f64>`] ([`crate::target::SignalTarget`])
/// and interior-mutable [`crate::target::HandleTarget<f64>`].
///
/// # Example
///
/// ```rust,no_run
/// use std::time::Duration;
/// use dioxus::prelude::*;
/// use kinetoxus::prelude::*;
///
/// #[component]
/// fn SwipeBox() -> Element {
///     let x = use_signal(|| 0.0f64);
///     let gesture = use_gesture_continuation(
///         x,
///         GestureContinuationConfig::default().with_bounds(-100.0, 100.0),
///     );
///
///     rsx! {
///         div {
///             onclick: move |_| {
///                 gesture.drag_start(0.0, Duration::ZERO);
///                 gesture.drag_move(50.0, Duration::from_millis(50));
///                 gesture.drag_end(Duration::from_millis(50));
///             },
///             "Click to trigger swipe"
///         }
///     }
/// }
/// ```
pub fn use_gesture_continuation<T>(
    target: T,
    config: GestureContinuationConfig,
) -> GestureContinuationController<T::Target>
where
    T: IntoAnimationTarget<f64>,
    T::Target: Clone + 'static,
{
    let target = target.into_target();
    let controller = use_hook(|| GestureContinuationController::new(target, config));

    let cleanup_controller = controller.clone();
    use_drop(move || {
        cleanup_controller.cancel();
    });

    controller
}
