//! Component lifecycle hook for motion.

use dioxus::prelude::*;

use crate::motion::Motion;

/// Creates and binds a [`Motion`] controller to the current Dioxus component scope.
///
/// Animations created through this hook are scoped to the component and will be
/// automatically cancelled when the component is unmounted.
///
/// # Example
///
/// ```rust,no_run
/// use std::time::Duration;
/// use dioxus::prelude::*;
/// use kinetoxus::prelude::*;
///
/// #[component]
/// fn FadeBox() -> Element {
///     let opacity = use_signal(|| 0.0f32);
///     let motion = use_motion();
///
///     rsx! {
///         div {
///             style: "opacity: {opacity};",
///             onclick: move |_| {
///                 motion.from_to(opacity, 0.0f32, 1.0f32, Duration::from_millis(400))
///                     .ease(Ease::CubicInOut);
///             },
///             "Click to Fade In"
///         }
///     }
/// }
/// ```
pub fn use_motion() -> Motion {
    let motion = use_hook(Motion::new);

    let cleanup_motion = motion.clone();
    use_drop(move || {
        cleanup_motion.cancel_all();
    });

    motion
}
