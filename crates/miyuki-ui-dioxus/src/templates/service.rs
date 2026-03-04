// @id: MUID-ServiceLayout @do: service-template @role: component @layer: 6 @human: miyuk

//! Service page layout with header, sidebar, and scrollable content.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     ServiceLayout {
//!         header: rsx! { AppHeader { /* ... */ } },
//!         sidebar: rsx! { AppSidebar { /* ... */ } },
//!         div { "Service content" }
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Service page layout template.
#[component]
pub fn ServiceLayout(
    /// Header element.
    header: Element,
    /// Sidebar element.
    sidebar: Element,
    /// Main content (children).
    children: Element,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let bg_color = p.bg_base.to_css();

    let container_style = format!(
        "display: flex; flex-direction: column; height: 100vh; \
         background: {bg_color}; overflow: hidden;"
    );

    let body_style = "display: flex; flex: 1; overflow: hidden;";
    let content_style = "flex: 1; overflow-y: auto; padding: 16px;";

    rsx! {
        div { style: "{container_style}",
            {header}
            div { style: "{body_style}",
                {sidebar}
                main { style: "{content_style}",
                    {children}
                }
            }
        }
    }
}
