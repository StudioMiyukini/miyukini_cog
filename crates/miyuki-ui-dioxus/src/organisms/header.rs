// @id: MUID-AppHeader @do: app-header-organism @role: component @layer: 6 @human: miyuk

//! Application header organism with navigation, search, and user profile.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     AppHeader {
//!         nav_items: vec![
//!             NavItem { id: "home".into(), label: "Home".into(), icon: None },
//!             NavItem { id: "store".into(), label: "Store".into(), icon: None },
//!         ],
//!         active_nav: "home".to_string(),
//!         on_nav: move |id| set_nav(id),
//!         user_name: "Player".to_string(),
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// A navigation item in the header.
#[derive(Debug, Clone, PartialEq)]
pub struct NavItem {
    /// Unique identifier.
    pub id: String,
    /// Display label.
    pub label: String,
    /// Optional icon (emoji/text).
    pub icon: Option<String>,
}

/// Application header organism.
#[component]
pub fn AppHeader(
    /// Navigation items.
    #[props(default)]
    nav_items: Vec<NavItem>,
    /// Currently active nav item id.
    #[props(default)]
    active_nav: String,
    /// Navigation change handler.
    #[props(default)]
    on_nav: EventHandler<String>,
    /// Search bar value.
    #[props(default)]
    search_value: String,
    /// Search change handler.
    #[props(default)]
    on_search: EventHandler<String>,
    /// User display name.
    #[props(default)]
    user_name: String,
    /// Optional user avatar URL.
    #[props(default)]
    user_avatar: Option<String>,
    /// Profile click handler.
    #[props(default)]
    on_profile: EventHandler<MouseEvent>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;
    let sp = &theme.spacing;
    let z = theme.z_index.sticky;

    let bg_color = p.bg_secondary.to_css();
    let border_color = p.border_subtle.to_css();
    let text_color = p.text_primary.to_css();
    let accent_color = p.accent_primary.to_css();
    let muted_color = p.text_secondary.to_css();
    let header_h = sp.space_10;

    let header_style = format!(
        "display: flex; align-items: center; gap: 16px; \
         height: {header_h}px; padding: 0 16px; \
         background: {bg_color}; border-bottom: 1px solid {border_color}; \
         z-index: {z}; flex-shrink: 0;"
    );

    let nav_style = "display: flex; align-items: center; gap: 4px; flex: 1;";

    let search_style = format!(
        "flex: 0 1 240px; padding: 4px 8px; \
         background: {bg}; border: 1px solid {brd}; \
         border-radius: 4px; color: {text_color}; \
         font-size: 13px; outline: none;",
        bg = p.bg_surface.to_css(),
        brd = p.border_default.to_css(),
    );

    let profile_style = "display: flex; align-items: center; gap: 8px; \
         cursor: pointer; padding: 4px 8px; border-radius: 4px; \
         transition: background 150ms;"
        .to_string();

    let avatar_size = 28.0_f32;
    let has_avatar = user_avatar.is_some();
    let avatar_url = user_avatar.unwrap_or_default();

    let avatar_style = format!(
        "width: {avatar_size}px; height: {avatar_size}px; \
         border-radius: 50%; background: {accent_color}; \
         display: flex; align-items: center; justify-content: center; \
         color: white; font-size: 12px; font-weight: 600; overflow: hidden;"
    );

    let initials = user_name
        .chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .to_string();

    rsx! {
        header { style: "{header_style}",
            nav { style: "{nav_style}",
                for item in nav_items.iter() {
                    {
                        let is_active = item.id == active_nav;
                        let item_color = if is_active { accent_color.clone() } else { muted_color.clone() };
                        let item_weight = if is_active { "600" } else { "400" };
                        let item_id = item.id.clone();
                        let has_icon = item.icon.is_some();
                        let icon_text = item.icon.clone().unwrap_or_default();
                        let item_style = format!(
                            "display: flex; align-items: center; gap: 4px; \
                             padding: 6px 10px; color: {item_color}; font-size: 13px; \
                             font-weight: {item_weight}; cursor: pointer; \
                             border-radius: 4px; transition: color 150ms; user-select: none;"
                        );
                        rsx! {
                            div {
                                style: "{item_style}",
                                onclick: move |_| on_nav.call(item_id.clone()),
                                if has_icon {
                                    span { style: "font-size: 14px;", "{icon_text}" }
                                }
                                span { "{item.label}" }
                            }
                        }
                    }
                }
            }
            input {
                style: "{search_style}",
                r#type: "search",
                placeholder: "Search...",
                value: "{search_value}",
                oninput: move |evt: FormEvent| on_search.call(evt.value()),
            }
            div {
                style: "{profile_style}",
                onclick: move |evt| on_profile.call(evt),
                div { style: "{avatar_style}",
                    if has_avatar {
                        img { src: "{avatar_url}", style: "width: 100%; height: 100%; object-fit: cover;" }
                    } else {
                        "{initials}"
                    }
                }
                span { style: "font-size: 13px; color: {text_color};", "{user_name}" }
            }
        }
    }
}
