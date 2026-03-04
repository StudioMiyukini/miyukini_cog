// @id: MUID-Pagination @do: pagination-molecule @role: component @layer: 6 @human: miyuk

//! Pagination molecule for navigating through pages.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Pagination {
//!         current_page: 3,
//!         total_pages: 10,
//!         on_page: move |page| set_current_page(page),
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Pagination molecule.
#[component]
pub fn Pagination(
    /// Current page (1-indexed).
    current_page: usize,
    /// Total number of pages.
    total_pages: usize,
    /// Page change handler (receives the target page number, 1-indexed).
    #[props(default)]
    on_page: EventHandler<usize>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let bg_active = p.accent_primary.to_css();
    let text_active = p.text_high.to_css();
    let text_normal = p.text_secondary.to_css();
    let text_disabled = p.text_muted.to_css();
    let radius = theme.radius.md;

    let container_style = "display: flex; align-items: center; gap: 4px;";

    let can_prev = current_page > 1;
    let can_next = current_page < total_pages;

    // Simple page range: show up to 7 pages around current
    let start = current_page.saturating_sub(3).max(1);
    let end = (start + 6).min(total_pages);
    let pages: Vec<usize> = (start..=end).collect();

    let nav_btn_color = if can_prev {
        text_normal.clone()
    } else {
        text_disabled.clone()
    };
    let nav_btn_next_color = if can_next {
        text_normal.clone()
    } else {
        text_disabled.clone()
    };

    let btn_base = format!(
        "display: inline-flex; align-items: center; justify-content: center; \
         min-width: 32px; height: 32px; border: none; \
         border-radius: {radius}px; font-size: 13px; \
         cursor: pointer; transition: background 150ms; padding: 0 6px;"
    );

    rsx! {
        nav {
            style: "{container_style}",
            aria_label: "Pagination",
            button {
                style: "{btn_base} background: transparent; color: {nav_btn_color};",
                disabled: !can_prev,
                onclick: move |_| {
                    if can_prev {
                        on_page.call(current_page - 1);
                    }
                },
                "<"
            }
            for page in pages.iter() {
                {
                    let pg = *page;
                    let is_current = pg == current_page;
                    let bg = if is_current { bg_active.clone() } else { "transparent".to_string() };
                    let color = if is_current { text_active.clone() } else { text_normal.clone() };
                    let weight = if is_current { "600" } else { "400" };
                    let page_str = format!("{pg}");
                    rsx! {
                        button {
                            style: "{btn_base} background: {bg}; color: {color}; font-weight: {weight};",
                            onclick: move |_| on_page.call(pg),
                            aria_current: if is_current { "page" } else { "" },
                            "{page_str}"
                        }
                    }
                }
            }
            button {
                style: "{btn_base} background: transparent; color: {nav_btn_next_color};",
                disabled: !can_next,
                onclick: move |_| {
                    if can_next {
                        on_page.call(current_page + 1);
                    }
                },
                ">"
            }
        }
    }
}
