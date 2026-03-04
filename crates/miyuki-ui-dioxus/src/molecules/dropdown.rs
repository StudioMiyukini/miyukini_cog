// @id: MUID-Dropdown @do: dropdown-molecule @role: component @layer: 6 @human: miyuk

//! Dropdown menu molecule.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Dropdown {
//!         trigger: rsx! { Button { "Options" } },
//!         items: vec![
//!             DropdownItem { id: "edit".into(), label: "Edit".into(), icon: None },
//!             DropdownItem { id: "delete".into(), label: "Delete".into(), icon: None },
//!         ],
//!         on_select: move |id| { /* handle selection */ },
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// A dropdown menu item.
#[derive(Debug, Clone, PartialEq)]
pub struct DropdownItem {
    /// Unique identifier.
    pub id: String,
    /// Display label.
    pub label: String,
    /// Optional icon (emoji/text).
    pub icon: Option<String>,
}

/// Dropdown menu molecule.
#[component]
pub fn Dropdown(
    /// The trigger element that opens the dropdown.
    trigger: Element,
    /// Menu items.
    items: Vec<DropdownItem>,
    /// Selection handler (called with the item id).
    #[props(default)]
    on_select: EventHandler<String>,
) -> Element {
    let mut open = use_signal(|| false);
    let theme = use_theme();
    let p = &theme.palette;
    let z = theme.z_index.dropdown;

    let bg_color = p.bg_elevated.to_css();
    let border_color = p.border_default.to_css();
    let text_color = p.text_primary.to_css();
    let radius = theme.radius.md;

    let is_open = *open.read();
    let display = if is_open { "block" } else { "none" };

    let menu_style = format!(
        "position: absolute; top: 100%; left: 0; margin-top: 4px; \
         display: {display}; min-width: 160px; \
         background: {bg_color}; border: 1px solid {border_color}; \
         border-radius: {radius}px; \
         box-shadow: 0 4px 12px rgba(0,0,0,0.3); \
         z-index: {z}; padding: 4px 0; overflow: hidden;"
    );

    let item_style = format!(
        "display: flex; align-items: center; gap: 8px; \
         padding: 8px 12px; color: {text_color}; font-size: 14px; \
         cursor: pointer; transition: background 100ms; user-select: none;"
    );

    rsx! {
        div {
            style: "position: relative; display: inline-flex;",
            div {
                onclick: move |_| {
                    let current = *open.read();
                    open.set(!current);
                },
                {trigger}
            }
            div { style: "{menu_style}",
                for item in items.iter() {
                    {
                        let item_id = item.id.clone();
                        let has_icon = item.icon.is_some();
                        let icon_text = item.icon.clone().unwrap_or_default();
                        rsx! {
                            div {
                                style: "{item_style}",
                                onclick: move |_| {
                                    on_select.call(item_id.clone());
                                    open.set(false);
                                },
                                if has_icon {
                                    span { style: "font-size: 16px; width: 20px; text-align: center;", "{icon_text}" }
                                }
                                span { "{item.label}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
