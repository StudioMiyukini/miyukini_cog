// @id: MUID-TabBar @do: tab-bar-organism @role: component @layer: 6 @human: miyuk

//! Tab bar organism managing a list of tabs.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     TabBar {
//!         tabs: vec![
//!             TabData { id: "home".into(), label: "Home".into(), icon: None, closable: false },
//!             TabData { id: "settings".into(), label: "Settings".into(), icon: None, closable: true },
//!         ],
//!         active_tab: "home".to_string(),
//!         on_select: move |id| set_active(id),
//!         on_close: move |id| close_tab(id),
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Data for a single tab.
#[derive(Debug, Clone, PartialEq)]
pub struct TabData {
    /// Unique identifier.
    pub id: String,
    /// Display label.
    pub label: String,
    /// Optional icon (emoji/text).
    pub icon: Option<String>,
    /// Whether the tab can be closed.
    pub closable: bool,
}

/// Tab bar organism.
#[component]
pub fn TabBar(
    /// Tab definitions.
    tabs: Vec<TabData>,
    /// Currently active tab id.
    #[props(default)]
    active_tab: String,
    /// Tab selection handler (receives tab id).
    #[props(default)]
    on_select: EventHandler<String>,
    /// Tab close handler (receives tab id).
    #[props(default)]
    on_close: EventHandler<String>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let bg_color = p.bg_primary.to_css();
    let border_color = p.border_subtle.to_css();
    let active_text = p.text_high.to_css();
    let inactive_text = p.text_secondary.to_css();
    let accent = p.accent_primary.to_css();
    let muted = p.text_muted.to_css();
    let active_bg = p.bg_surface.to_css();

    let bar_style = format!(
        "display: flex; align-items: stretch; \
         background: {bg_color}; border-bottom: 1px solid {border_color}; \
         overflow-x: auto; flex-shrink: 0;"
    );

    rsx! {
        div {
            style: "{bar_style}",
            role: "tablist",
            for tab in tabs.iter() {
                {
                    let is_active = tab.id == active_tab;
                    let text_color = if is_active { active_text.clone() } else { inactive_text.clone() };
                    let tab_bg = if is_active { active_bg.clone() } else { "transparent".to_string() };
                    let border_bottom = if is_active {
                        format!("2px solid {accent}")
                    } else {
                        "2px solid transparent".to_string()
                    };
                    let tab_id = tab.id.clone();
                    let tab_id_close = tab.id.clone();
                    let has_icon = tab.icon.is_some();
                    let icon_text = tab.icon.clone().unwrap_or_default();
                    let tab_style = format!(
                        "display: flex; align-items: center; gap: 6px; \
                         padding: 8px 14px; color: {text_color}; \
                         background: {tab_bg}; font-size: 13px; \
                         border-bottom: {border_bottom}; \
                         cursor: pointer; transition: all 150ms; \
                         user-select: none; white-space: nowrap;"
                    );
                    let close_style = format!(
                        "background: none; border: none; color: {muted}; \
                         cursor: pointer; font-size: 12px; padding: 0 2px; \
                         line-height: 1; margin-left: 4px;"
                    );
                    let closable = tab.closable;
                    rsx! {
                        div {
                            style: "{tab_style}",
                            role: "tab",
                            aria_selected: "{is_active}",
                            onclick: move |_| on_select.call(tab_id.clone()),
                            if has_icon {
                                span { style: "font-size: 14px;", "{icon_text}" }
                            }
                            span { "{tab.label}" }
                            if closable {
                                button {
                                    style: "{close_style}",
                                    onclick: move |evt| {
                                        evt.stop_propagation();
                                        on_close.call(tab_id_close.clone());
                                    },
                                    "x"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
