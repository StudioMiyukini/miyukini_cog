// @id: MUID-DashboardLayout @do: dashboard-template @role: component @layer: 6 @human: miyuk

//! Dashboard layout template with header, optional sidebar, and scrollable content.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     DashboardLayout {
//!         header: rsx! { AppHeader { /* ... */ } },
//!         sidebar: Some(rsx! { AppSidebar { /* ... */ } }),
//!         div { "Main content area" }
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Dashboard layout template.
#[component]
pub fn DashboardLayout(
    /// Header element.
    header: Element,
    /// Optional sidebar element.
    #[props(default)]
    sidebar: Option<Element>,
    /// Main content (children).
    children: Element,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let bg_color = p.bg_base.to_css();
    let has_sidebar = sidebar.is_some();

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
                if has_sidebar {
                    {sidebar}
                }
                main { style: "{content_style}",
                    {children}
                }
            }
        }
    }
}
