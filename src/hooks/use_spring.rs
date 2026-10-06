//! Component lifecycle hook for spring animation.

use dioxus::prelude::*;
use kinetocore::spring::SpringConfig;

use crate::hooks::use_motion;
use crate::motion::SpringHandle;
use crate::target::IntoAnimationTarget;

/// Creates and binds a spring animation to the current Dioxus component scope.
///
/// Drives `target` towards `initial_goal` using the specified physical `config`.
/// Automatically cancels the animation when the component unmounts.
///
/// # Example
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use kinetoxus::prelude::*;
/// use kinetocore::spring::SpringConfig;
///
/// #[component]
/// fn SpringBox() -> Element {
///     let x = use_signal(|| 0.0f64);
///     let spring = use_spring(x, 100.0, SpringConfig::DEFAULT);
///
///     rsx! {
///         div {
///             onclick: move |_| {
///                 spring.set_target(200.0);
///             },
///             "Click to Retarget"
///         }
///     }
/// }
/// ```
pub fn use_spring(
    target: impl IntoAnimationTarget<f64>,
    initial_goal: f64,
    config: SpringConfig,
) -> SpringHandle {
    let motion = use_motion();
    let target = target.into_target();

    let handle = use_hook(|| motion.spring(target, initial_goal, config));

    let cleanup_handle = handle.clone();
    use_drop(move || {
        cleanup_handle.cancel();
    });

    handle
}
