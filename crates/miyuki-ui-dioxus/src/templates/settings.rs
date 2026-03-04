// @id: MUID-SettingsLayout @do: settings-template @role: component @layer: 6 @human: miyuk

//! Settings page layout with sidebar navigation and content area.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     SettingsLayout {
//!         sidebar: rsx! { AppSidebar { /* settings nav */ } },
//!         div { "Settings content" }
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Settings page layout template.
#[component]
pub fn SettingsLayout(
    /// Settings navigation sidebar.
    sidebar: Element,
    /// Settings content (children).
    children: Element,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let bg_color = p.bg_base.to_css();

    let container_style = format!(
        "display: flex; height: 100vh; background: {bg_color}; overflow: hidden;"
    );

    let content_style = "flex: 1; overflow-y: auto; padding: 24px;";

    rsx! {
        div { style: "{container_style}",
            {sidebar}
            main { style: "{content_style}",
                {children}
            }
        }
    }
}
