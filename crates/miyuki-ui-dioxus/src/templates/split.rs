// @id: MUID-SplitLayout @do: split-template @role: component @layer: 6 @human: miyuk

//! Split panel layout dividing the viewport into left and right sections.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     SplitLayout {
//!         left: rsx! { div { "Left panel" } },
//!         right: rsx! { div { "Right panel" } },
//!         ratio: 40.0,
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Split panel layout template.
#[component]
pub fn SplitLayout(
    /// Left panel content.
    left: Element,
    /// Right panel content.
    right: Element,
    /// Left panel width percentage (default 50).
    #[props(default = 50.0)]
    ratio: f32,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let bg_color = p.bg_base.to_css();
    let border_color = p.border_subtle.to_css();
    let right_pct = 100.0 - ratio;

    let container_style =
        format!("display: flex; height: 100vh; background: {bg_color}; overflow: hidden;");

    let left_style =
        format!("width: {ratio}%; overflow-y: auto; border-right: 1px solid {border_color};");

    let right_style = format!("width: {right_pct}%; overflow-y: auto;");

    rsx! {
        div { style: "{container_style}",
            div { style: "{left_style}",
                {left}
            }
            div { style: "{right_style}",
                {right}
            }
        }
    }
}
