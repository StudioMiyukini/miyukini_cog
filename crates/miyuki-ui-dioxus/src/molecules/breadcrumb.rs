// @id: MUID-Breadcrumb @do: breadcrumb-molecule @role: component @layer: 6 @human: miyuk

//! Breadcrumb navigation molecule.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Breadcrumb {
//!         items: vec![
//!             BreadcrumbItem { label: "Home".to_string() },
//!             BreadcrumbItem { label: "Settings".to_string() },
//!             BreadcrumbItem { label: "Profile".to_string() },
//!         ],
//!         on_navigate: move |idx| { /* navigate to idx */ },
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// A single breadcrumb entry.
#[derive(Debug, Clone, PartialEq)]
pub struct BreadcrumbItem {
    /// Display label.
    pub label: String,
}

/// Breadcrumb navigation molecule.
#[component]
pub fn Breadcrumb(
    /// Ordered list of breadcrumb items (last = current page).
    items: Vec<BreadcrumbItem>,
    /// Navigation handler (called with the index of the clicked item).
    #[props(default)]
    on_navigate: EventHandler<usize>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let link_color = p.accent_primary.to_css();
    let current_color = p.text_primary.to_css();
    let separator_color = p.text_muted.to_css();

    let container_style = "display: flex; align-items: center; gap: 4px; font-size: 13px;";

    let last_idx = if items.is_empty() { 0 } else { items.len() - 1 };

    rsx! {
        nav {
            style: "{container_style}",
            aria_label: "Breadcrumb",
            for (idx, item) in items.iter().enumerate() {
                {
                    let is_last = idx == last_idx;
                    let color = if is_last { current_color.clone() } else { link_color.clone() };
                    let cursor = if is_last { "default" } else { "pointer" };
                    let weight = if is_last { "600" } else { "400" };
                    let item_style = format!(
                        "color: {color}; cursor: {cursor}; font-weight: {weight};"
                    );
                    let show_sep = !is_last;
                    let sep_style = format!("color: {separator_color}; margin: 0 4px;");
                    rsx! {
                        span {
                            style: "{item_style}",
                            onclick: move |_| {
                                if !is_last {
                                    on_navigate.call(idx);
                                }
                            },
                            "{item.label}"
                        }
                        if show_sep {
                            span { style: "{sep_style}", "/" }
                        }
                    }
                }
            }
        }
    }
}
