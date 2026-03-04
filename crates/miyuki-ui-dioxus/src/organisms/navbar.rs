// @id: MUID-NavBar @do: nav-bar-organism @role: component @layer: 6 @human: miyuk

//! Navigation bar organism for top-level navigation.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     NavBar {
//!         items: vec![
//!             NavBarItem { id: "home".into(), label: "Home".into(), icon: None },
//!             NavBarItem { id: "store".into(), label: "Store".into(), icon: None },
//!         ],
//!         active: "home".to_string(),
//!         on_navigate: move |id| set_active(id),
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Navigation bar item data.
#[derive(Debug, Clone, PartialEq)]
pub struct NavBarItem {
    /// Unique identifier.
    pub id: String,
    /// Display label.
    pub label: String,
    /// Optional icon (emoji/text).
    pub icon: Option<String>,
}

/// Navigation bar organism.
#[component]
pub fn NavBar(
    /// Navigation items.
    items: Vec<NavBarItem>,
    /// Currently active item id.
    #[props(default)]
    active: String,
    /// Navigation handler (receives item id).
    #[props(default)]
    on_navigate: EventHandler<String>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let bg_color = p.bg_secondary.to_css();
    let border_color = p.border_subtle.to_css();
    let active_color = p.accent_primary.to_css();
    let inactive_color = p.text_secondary.to_css();

    let nav_style = format!(
        "display: flex; align-items: stretch; \
         background: {bg_color}; border-bottom: 1px solid {border_color}; \
         height: 36px; flex-shrink: 0;"
    );

    rsx! {
        nav { style: "{nav_style}",
            for item in items.iter() {
                {
                    let is_active = item.id == active;
                    let text_color = if is_active { active_color.clone() } else { inactive_color.clone() };
                    let weight = if is_active { "600" } else { "400" };
                    let border_bottom = if is_active {
                        format!("2px solid {active_color}")
                    } else {
                        "2px solid transparent".to_string()
                    };
                    let item_id = item.id.clone();
                    let has_icon = item.icon.is_some();
                    let icon_text = item.icon.clone().unwrap_or_default();
                    let item_style = format!(
                        "display: flex; align-items: center; gap: 4px; \
                         padding: 0 14px; color: {text_color}; font-size: 13px; \
                         font-weight: {weight}; border-bottom: {border_bottom}; \
                         cursor: pointer; transition: color 150ms; user-select: none;"
                    );
                    rsx! {
                        div {
                            style: "{item_style}",
                            onclick: move |_| on_navigate.call(item_id.clone()),
                            if has_icon {
                                span { style: "font-size: 14px;", "{icon_text}" }
                            }
                            span { "{item.label}" }
                        }
                    }
                }
            }
        }
    }
}
