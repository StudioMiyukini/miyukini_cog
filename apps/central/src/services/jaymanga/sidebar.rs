//! Sidebar JayManga — navigation par section.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::{JayMangaSection, JayMangaState};

#[component]
pub fn JayMangaSidebar(state: Signal<JayMangaState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    // Compteurs réels
    let db = &conns.read().jaymanga;
    let work_count = db.work_list(&jaymanga::data::WorkFilters::default()).map(|v| v.len()).unwrap_or(0);
    let series_count = db.series_list().map(|v| v.len()).unwrap_or(0);
    let license_count = db.license_list_all().map(|v| v.len()).unwrap_or(0);
    let favorite_count = db.favorite_list().map(|v| v.len()).unwrap_or(0);
    let promo_count = db.promotion_list_active().map(|v| v.len()).unwrap_or(0);

    rsx! {
        aside {
            style: "width: 220px; background: {c.bg_secondary}; border-right: 1px solid {c.border}; padding: 16px 0; display: flex; flex-direction: column; overflow-y: auto;",

            // En-tête service
            div {
                style: "padding: 0 16px 16px 16px; border-bottom: 1px solid {c.border};",

                div {
                    style: "display: flex; align-items: center; gap: 8px; margin-bottom: 4px;",
                    span { style: "font-size: 20px;", "📚" }
                    h3 {
                        style: "font-size: 16px; color: {c.text_white};",
                        "JayManga"
                    }
                }
                p {
                    style: "font-size: 11px; color: {c.text_muted};",
                    "Manga en ligne"
                }
            }

            // Navigation sections
            nav {
                style: "display: flex; flex-direction: column; gap: 2px; padding-top: 12px; flex: 1;",

                // ── Vendeur ──
                SidebarLabel { text: "Vendeur" }

                SidebarItem {
                    icon: "📊",
                    label: "Dashboard",
                    is_active: state.read().section == JayMangaSection::Dashboard,
                    onclick: move |_| { state.write().section = JayMangaSection::Dashboard; },
                }
                SidebarItem {
                    icon: "📖",
                    label: "Catalogue",
                    badge: if work_count > 0 { Some(format!("{work_count}")) } else { None },
                    is_active: matches!(state.read().section, JayMangaSection::Catalogue | JayMangaSection::NouvelleOeuvre | JayMangaSection::ModifierOeuvre),
                    onclick: move |_| { state.write().section = JayMangaSection::Catalogue; },
                }
                SidebarItem {
                    icon: "📚",
                    label: "Séries",
                    badge: if series_count > 0 { Some(format!("{series_count}")) } else { None },
                    is_active: state.read().section == JayMangaSection::Series,
                    onclick: move |_| { state.write().section = JayMangaSection::Series; },
                }
                SidebarItem {
                    icon: "📑",
                    label: "Chapitres",
                    is_active: state.read().section == JayMangaSection::Chapters,
                    onclick: move |_| { state.write().section = JayMangaSection::Chapters; },
                }
                SidebarItem {
                    icon: "💰",
                    label: "Ventes",
                    badge: if license_count > 0 { Some(format!("{license_count}")) } else { None },
                    is_active: state.read().section == JayMangaSection::Sales,
                    onclick: move |_| { state.write().section = JayMangaSection::Sales; },
                }
                SidebarItem {
                    icon: "🎁",
                    label: "Promotions",
                    badge: if promo_count > 0 { Some(format!("{promo_count}")) } else { None },
                    is_active: state.read().section == JayMangaSection::Promotions,
                    onclick: move |_| { state.write().section = JayMangaSection::Promotions; },
                }

                // ── Lecteur ──
                div { style: "height: 1px; background: {c.border}; margin: 8px 16px;" }
                SidebarLabel { text: "Lecteur" }

                SidebarItem {
                    icon: "📚",
                    label: "Bibliothèque",
                    badge: if favorite_count > 0 { Some(format!("{favorite_count}")) } else { None },
                    is_active: state.read().section == JayMangaSection::Library,
                    onclick: move |_| { state.write().section = JayMangaSection::Library; },
                }
                SidebarItem {
                    icon: "📖",
                    label: "Liseuse",
                    is_active: state.read().section == JayMangaSection::Reader,
                    onclick: move |_| { state.write().section = JayMangaSection::Reader; },
                }
                SidebarItem {
                    icon: "👤",
                    label: "Profil",
                    is_active: state.read().section == JayMangaSection::Profile,
                    onclick: move |_| { state.write().section = JayMangaSection::Profile; },
                }
                SidebarItem {
                    icon: "📈",
                    label: "Statistiques",
                    is_active: state.read().section == JayMangaSection::ReaderStats,
                    onclick: move |_| { state.write().section = JayMangaSection::ReaderStats; },
                }

                // ── Configuration ──
                div { style: "height: 1px; background: {c.border}; margin: 8px 16px;" }
                SidebarLabel { text: "Configuration" }

                SidebarItem {
                    icon: "🏪",
                    label: "Boutique",
                    is_active: state.read().section == JayMangaSection::Boutique,
                    onclick: move |_| { state.write().section = JayMangaSection::Boutique; },
                }
                SidebarItem {
                    icon: "🌐",
                    label: "Agrégation",
                    is_active: state.read().section == JayMangaSection::Aggregation,
                    onclick: move |_| { state.write().section = JayMangaSection::Aggregation; },
                }
            }
        }
    }
}

/// Label de section dans la sidebar.
#[component]
fn SidebarLabel(text: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        p {
            style: "padding: 4px 16px 4px 16px; font-size: 10px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 1px; font-weight: 600;",
            "{text}"
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
            style: "display: flex; align-items: center; gap: 12px; padding: 8px 16px; background: {bg}; color: {color}; border: none; border-left: {border}; cursor: pointer; font-size: 13px; text-align: left; width: 100%; transition: all 0.2s;",
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
