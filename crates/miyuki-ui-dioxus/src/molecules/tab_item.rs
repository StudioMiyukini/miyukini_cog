// @id: MUID-TabItem @do: tab-item-molecule @role: component @layer: 6 @human: miyuk

//! Tab item molecule for tab bars and navigation.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     TabItem {
//!         label: "Dashboard".to_string(),
//!         is_active: true,
//!         on_click: move |_| { /* switch tab */ },
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Tab item molecule.
#[component]
pub fn TabItem(
    /// Tab label text.
    label: String,
    /// Optional icon (emoji/text).
    #[props(default)]
    icon: Option<String>,
    /// Optional count badge.
    #[props(default)]
    count: Option<u32>,
    /// Whether the close button is shown.
    #[props(default)]
    closable: bool,
    /// Whether this tab is active.
    #[props(default)]
    is_active: bool,
    /// Click handler.
    #[props(default)]
    on_click: EventHandler<MouseEvent>,
    /// Close handler.
    #[props(default)]
    on_close: Option<EventHandler<MouseEvent>>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let bg_color = if is_active {
        p.bg_surface.to_css()
    } else {
        "transparent".to_string()
    };
    let text_color = if is_active {
        p.text_high.to_css()
    } else {
        p.text_secondary.to_css()
    };
    let border_bottom = if is_active {
        format!("2px solid {}", p.accent_primary.to_css())
    } else {
        "2px solid transparent".to_string()
    };
    let muted_color = p.text_muted.to_css();
    let accent_subtle = p.accent_primary_subtle.to_css();

    let tab_style = format!(
        "display: inline-flex; align-items: center; gap: 6px; \
         padding: 8px 12px; background: {bg_color}; \
         border-bottom: {border_bottom}; \
         cursor: pointer; transition: all 150ms; user-select: none; \
         font-size: 13px; color: {text_color}; white-space: nowrap;"
    );

    let has_icon = icon.is_some();
    let icon_text = icon.unwrap_or_default();
    let has_count = count.is_some();
    let count_val = count.unwrap_or(0);
    let count_str = format!("{count_val}");
    let has_close = closable && on_close.is_some();

    let badge_style = format!(
        "font-size: 10px; background: {accent_subtle}; color: {text_color}; \
         padding: 1px 5px; border-radius: 8px; line-height: 1.2;"
    );

    let close_style = format!(
        "background: none; border: none; color: {muted_color}; \
         cursor: pointer; font-size: 12px; padding: 0 2px; line-height: 1; \
         margin-left: 2px;"
    );

    rsx! {
        div {
            style: "{tab_style}",
            role: "tab",
            aria_selected: "{is_active}",
            onclick: move |evt| on_click.call(evt),
            if has_icon {
                span { style: "font-size: 14px;", "{icon_text}" }
            }
            span { "{label}" }
            if has_count {
                span { style: "{badge_style}", "{count_str}" }
            }
            if has_close {
                button {
                    style: "{close_style}",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        if let Some(ref handler) = on_close {
                            handler.call(evt);
                        }
                    },
                    "x"
                }
            }
        }
    }
}
