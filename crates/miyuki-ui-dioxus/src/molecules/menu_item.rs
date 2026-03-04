// @id: MUID-MenuItem @do: menu-item-molecule @role: component @layer: 6 @human: miyuk

//! Menu item molecule for navigation and action lists.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     MenuItem {
//!         label: "Settings".to_string(),
//!         icon: Some("gear".to_string()),
//!         shortcut: Some("Ctrl+,".to_string()),
//!         on_click: move |_| { /* open settings */ },
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Menu item molecule.
#[component]
pub fn MenuItem(
    /// Icon text/emoji.
    #[props(default)]
    icon: Option<String>,
    /// Label text.
    label: String,
    /// Optional keyboard shortcut display.
    #[props(default)]
    shortcut: Option<String>,
    /// Whether this item opens a submenu.
    #[props(default)]
    has_submenu: bool,
    /// Whether this item is currently active.
    #[props(default)]
    active: bool,
    /// Click handler.
    #[props(default)]
    on_click: EventHandler<MouseEvent>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;
    let sp = &theme.spacing;

    let bg_color = if active {
        p.bg_overlay.to_css()
    } else {
        "transparent".to_string()
    };
    let text_color = p.text_primary.to_css();
    let muted_color = p.text_muted.to_css();
    let pad_v = sp.space_2;
    let pad_h = sp.space_3;
    let radius = theme.radius.md;

    let container_style = format!(
        "display: flex; align-items: center; gap: 8px; \
         padding: {pad_v}px {pad_h}px; \
         background: {bg_color}; border-radius: {radius}px; \
         cursor: pointer; transition: background 150ms; user-select: none;"
    );

    let has_icon = icon.is_some();
    let icon_text = icon.unwrap_or_default();
    let has_shortcut = shortcut.is_some();
    let shortcut_text = shortcut.unwrap_or_default();

    rsx! {
        div {
            style: "{container_style}",
            role: "menuitem",
            onclick: move |evt| on_click.call(evt),
            if has_icon {
                span { style: "font-size: 16px; width: 20px; text-align: center; flex-shrink: 0;", "{icon_text}" }
            }
            span { style: "flex: 1; color: {text_color}; font-size: 14px;", "{label}" }
            if has_shortcut {
                span { style: "font-size: 12px; color: {muted_color}; flex-shrink: 0;", "{shortcut_text}" }
            }
            if has_submenu {
                span { style: "font-size: 12px; color: {muted_color}; flex-shrink: 0;", ">" }
            }
        }
    }
}
