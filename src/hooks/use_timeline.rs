//! Component lifecycle hook for timeline animations.

use dioxus::prelude::*;

use crate::motion::{TimelineController, TimelineHookBuilder};

/// Creates and binds a [`TimelineController`] to the current Dioxus component scope.
///
/// Compiles the timeline upon mount, binds declared tracks to reactive signals or handles,
/// immediately evaluates and applies initial values at $t = 0$, and automatically cancels
/// playback and unregisters frame listeners upon component unmount.
///
/// # Example
///
/// ```rust,no_run
/// use std::time::Duration;
/// use dioxus::prelude::*;
/// use kinetoxus::prelude::*;
///
/// #[component]
/// fn TimelineBox() -> Element {
///     let x = use_signal(|| 0.0f64);
///     let opacity = use_signal(|| 0.0f32);
///
///     let timeline = use_timeline(|b| {
///         b.track("x", &x, 0.0, |t| {
///             t.tween(100.0, Duration::from_millis(500), Ease::QuadInOut);
///         });
///         b.track("opacity", &opacity, 0.0, |t| {
///             t.tween(1.0, Duration::from_millis(300), Ease::Linear);
///         });
///     });
///
///     rsx! {
///         div {
///             onclick: move |_| timeline.play(),
///             "Play Timeline"
///         }
///     }
/// }
/// ```
pub fn use_timeline<F>(init: F) -> TimelineController
where
    F: FnOnce(&mut TimelineHookBuilder),
{
    let controller = use_hook(|| {
        let mut builder = TimelineHookBuilder::new();
        init(&mut builder);
        builder
            .build()
            .expect("Timeline compilation failed in use_timeline")
    });

    let cleanup = controller.clone();
    use_drop(move || {
        cleanup.cancel();
    });

    controller
}
