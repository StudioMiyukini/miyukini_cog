// @id: MUID-DetailLayout @do: detail-template @role: component @layer: 6 @human: miyuk

//! Detail page layout with header, optional side panel, and content.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     DetailLayout {
//!         header: rsx! { AppHeader { /* ... */ } },
//!         div { "Detail content" }
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Detail page layout template.
#[component]
pub fn DetailLayout(
    /// Header element.
    header: Element,
    /// Optional side panel.
    #[props(default)]
    side_panel: Option<Element>,
    /// Main content (children).
    children: Element,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let bg_color = p.bg_base.to_css();
    let has_side = side_panel.is_some();

    let container_style = format!(
        "display: flex; flex-direction: column; height: 100vh; \
         background: {bg_color}; overflow: hidden;"
    );

    let body_style = "display: flex; flex: 1; overflow: hidden;";
    let content_style = "flex: 1; overflow-y: auto; padding: 24px;";
    let side_style = "width: 320px; overflow-y: auto; flex-shrink: 0;";

    rsx! {
        div { style: "{container_style}",
            {header}
            div { style: "{body_style}",
                main { style: "{content_style}",
                    {children}
                }
                if has_side {
                    aside { style: "{side_style}",
                        {side_panel}
                    }
                }
            }
        }
    }
}
