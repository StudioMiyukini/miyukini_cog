// @id: MUID-AppSidebar @do: app-sidebar-organism @role: component @layer: 6 @human: miyuk

//! Application sidebar organism with sections and items.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     AppSidebar {
//!         title: "Settings".to_string(),
//!         sections: vec![SidebarSectionData {
//!             title: "General".to_string(),
//!             items: vec![SidebarItemData {
//!                 id: "profile".into(),
//!                 label: "Profile".into(),
//!                 icon: None,
//!                 badge_count: None,
//!             }],
//!         }],
//!         active_item: "profile".to_string(),
//!         on_item: move |id| set_active(id),
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Data for a sidebar item.
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarItemData {
    /// Unique identifier.
    pub id: String,
    /// Display label.
    pub label: String,
    /// Optional icon (emoji/text).
    pub icon: Option<String>,
    /// Optional badge count.
    pub badge_count: Option<u32>,
}

/// Data for a sidebar section.
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarSectionData {
    /// Section title.
    pub title: String,
    /// Items in this section.
    pub items: Vec<SidebarItemData>,
}

/// Application sidebar organism.
#[component]
pub fn AppSidebar(
    /// Sidebar title.
    #[props(default)]
    title: String,
    /// Sections with their items.
    #[props(default)]
    sections: Vec<SidebarSectionData>,
    /// Currently active item id.
    #[props(default)]
    active_item: String,
    /// Item click handler (receives item id).
    #[props(default)]
    on_item: EventHandler<String>,
    /// Optional footer element.
    #[props(default)]
    footer: Option<Element>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;
    let sp = &theme.spacing;

    let bg_color = p.bg_secondary.to_css();
    let border_color = p.border_subtle.to_css();
    let title_color = p.text_high.to_css();
    let section_title_color = p.text_muted.to_css();
    let text_color = p.text_primary.to_css();
    let active_bg = p.bg_overlay.to_css();
    let accent_color = p.accent_primary.to_css();
    let badge_bg = p.accent_primary_subtle.to_css();
    let pad = sp.space_3;

    let sidebar_style = format!(
        "display: flex; flex-direction: column; width: 220px; \
         background: {bg_color}; border-right: 1px solid {border_color}; \
         overflow-y: auto; flex-shrink: 0;"
    );

    let title_style = format!(
        "padding: {pad}px; font-size: 15px; font-weight: 700; \
         color: {title_color}; border-bottom: 1px solid {border_color};"
    );

    let has_title = !title.is_empty();
    let has_footer = footer.is_some();

    rsx! {
        aside { style: "{sidebar_style}",
            if has_title {
                div { style: "{title_style}", "{title}" }
            }
            div { style: "flex: 1; padding: 8px;",
                for section in sections.iter() {
                    {
                        let section_style = format!(
                            "font-size: 10px; font-weight: 600; color: {section_title_color}; \
                             text-transform: uppercase; letter-spacing: 1px; \
                             padding: 12px 8px 4px 8px;"
                        );
                        rsx! {
                            div { style: "{section_style}", "{section.title}" }
                            for item in section.items.iter() {
                                {
                                    let is_active = item.id == active_item;
                                    let item_bg = if is_active { active_bg.clone() } else { "transparent".to_string() };
                                    let item_color = if is_active { accent_color.clone() } else { text_color.clone() };
                                    let item_weight = if is_active { "600" } else { "400" };
                                    let left_border = if is_active {
                                        format!("border-left: 2px solid {accent_color};")
                                    } else {
                                        "border-left: 2px solid transparent;".to_string()
                                    };
                                    let item_id = item.id.clone();
                                    let has_icon = item.icon.is_some();
                                    let icon_text = item.icon.clone().unwrap_or_default();
                                    let has_badge = item.badge_count.is_some();
                                    let badge_val = item.badge_count.unwrap_or(0);
                                    let badge_str = format!("{badge_val}");
                                    let item_style = format!(
                                        "display: flex; align-items: center; gap: 8px; \
                                         padding: 6px 8px; background: {item_bg}; \
                                         color: {item_color}; font-size: 13px; \
                                         font-weight: {item_weight}; border-radius: 4px; \
                                         cursor: pointer; transition: background 150ms; \
                                         user-select: none; {left_border}"
                                    );
                                    let badge_style = format!(
                                        "font-size: 10px; background: {badge_bg}; \
                                         padding: 1px 5px; border-radius: 8px; line-height: 1.2;"
                                    );
                                    rsx! {
                                        div {
                                            style: "{item_style}",
                                            onclick: move |_| on_item.call(item_id.clone()),
                                            if has_icon {
                                                span { style: "font-size: 14px; width: 18px; text-align: center;", "{icon_text}" }
                                            }
                                            span { style: "flex: 1;", "{item.label}" }
                                            if has_badge {
                                                span { style: "{badge_style}", "{badge_str}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if has_footer {
                div {
                    style: "padding: 8px; border-top: 1px solid {border_color};",
                    {footer}
                }
            }
        }
    }
}
