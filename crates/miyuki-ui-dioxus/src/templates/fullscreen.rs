// @id: MUID-FullscreenLayout @do: fullscreen-template @role: component @layer: 6 @human: miyuk

//! Fullscreen layout with centered content.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     FullscreenLayout {
//!         div { "Centered content" }
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Fullscreen layout template centering its content.
#[component]
pub fn FullscreenLayout(
    /// Centered content (children).
    children: Element,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let bg_color = p.bg_base.to_css();

    let style_str = format!(
        "display: flex; align-items: center; justify-content: center; \
         min-height: 100vh; background: {bg_color}; padding: 24px;"
    );

    rsx! {
        div { style: "{style_str}",
            {children}
        }
    }
}
