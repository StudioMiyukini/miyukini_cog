//! Sidebar Market — navigation et recherche.

use dioxus::prelude::*;
use crate::state::use_app_state;
use super::{MarketSection, MarketState};

#[component]
pub fn MarketSidebar(state: Signal<MarketState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let app = use_app_state();

    // Compteurs
    let installed_count = app.read().services.iter().filter(|s| s.is_installed).count();

    rsx! {
        aside {
            style: "width: 220px; background: {c.bg_secondary}; border-right: 1px solid {c.border}; padding: 16px 0; display: flex; flex-direction: column;",

            // En-tête
            div {
                style: "padding: 0 16px 16px 16px; border-bottom: 1px solid {c.border};",
                div {
                    style: "display: flex; align-items: center; gap: 8px; margin-bottom: 4px;",
                    span { style: "font-size: 20px;", "\u{1F6D2}" }
                    h3 { style: "font-size: 16px; color: {c.text_white};", "Service Market" }
                }
                p { style: "font-size: 11px; color: {c.text_muted};", "Catalogue des services" }
            }

            // Barre de recherche
            div {
                style: "padding: 12px 16px;",
                input {
                    style: "width: 100%; padding: 8px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_white}; font-size: 12px; outline: none;",
                    r#type: "text",
                    placeholder: "Rechercher un service...",
                    value: "{state.read().search_query}",
                    oninput: move |evt| {
                        state.write().search_query = evt.value();
                        state.write().selected_service_id = None;
                    },
                }
            }

            // Navigation sections
            nav {
                style: "display: flex; flex-direction: column; gap: 4px; flex: 1;",

                SidebarItem {
                    icon: "\u{2728}",
                    label: "D\u{00e9}couvrir",
                    is_active: state.read().section == MarketSection::Decouvrir,
                    onclick: move |_| {
                        let mut s = state.write();
                        s.section = MarketSection::Decouvrir;
                        s.selected_service_id = None;
                        s.type_filter = None;
                    },
                }
                SidebarItem {
                    icon: "\u{2B50}",
                    label: "Officiels",
                    is_active: state.read().section == MarketSection::Officiels,
                    onclick: move |_| {
                        let mut s = state.write();
                        s.section = MarketSection::Officiels;
                        s.selected_service_id = None;
                        s.type_filter = None;
                    },
                }
                SidebarItem {
                    icon: "\u{1F30D}",
                    label: "Communaut\u{00e9}",
                    is_active: state.read().section == MarketSection::Communaute,
                    onclick: move |_| {
                        let mut s = state.write();
                        s.section = MarketSection::Communaute;
                        s.selected_service_id = None;
                        s.type_filter = None;
                    },
                }

                // Séparateur
                div { style: "height: 1px; background: {c.border}; margin: 8px 16px;" }

                SidebarItem {
                    icon: "\u{1F4E6}",
                    label: "Install\u{00e9}s",
                    badge: Some(installed_count.to_string()),
                    is_active: state.read().section == MarketSection::Installes,
                    onclick: move |_| {
                        let mut s = state.write();
                        s.section = MarketSection::Installes;
                        s.selected_service_id = None;
                        s.type_filter = None;
                    },
                }
            }
        }
    }
}

/// Item de navigation sidebar.
#[component]
fn SidebarItem(
    icon: &'static str,
    label: &'static str,
    #[props(default)] badge: Option<String>,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let bg = if is_active { c.bg_hover } else { "transparent" };
    let color = if is_active { c.text_white } else { c.text_secondary };
    let border = if is_active {
        format!("2px solid {}", c.accent_blue)
    } else {
        "2px solid transparent".to_string()
    };

    rsx! {
        button {
            style: "display: flex; align-items: center; gap: 12px; padding: 10px 16px; background: {bg}; color: {color}; border: none; border-left: {border}; cursor: pointer; font-size: 13px; text-align: left; width: 100%; transition: all 0.2s;",
            onclick: move |evt| onclick.call(evt),

            span { "{icon}" }
            span { style: "flex: 1;", "{label}" }
            if let Some(ref count) = badge {
                span {
                    style: "padding: 2px 6px; background: {c.bg_hover}; border-radius: 10px; font-size: 10px; color: {c.text_muted};",
                    "{count}"
                }
            }
        }
    }
}
