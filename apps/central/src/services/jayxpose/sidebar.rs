//! Sidebar JayXpose — navigation par section (XP-E01..XP-E12).
//!
//! @id: jx_sidebar @do: render_sidebar_navigation
//! @role: ui @layer: service
//! @human: Sidebar JayXpose: navigation par section (XP-E01 a XP-E12).

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::{JayXposeSection, JayXposeState};

#[component]
pub fn JayXposeSidebar(state: Signal<JayXposeState>) -> Element {
    let p = use_palette();
    let conns = use_service_connections();

    // Compteurs reels
    let exposant_id = state.read().exposant_id.clone();
    let (product_count, doc_count, page_count) = if let Some(ref eid) = exposant_id {
        let db = &conns.read().jayxpose;
        let prods = db.produits_by_exposant(eid).map(|v| v.len()).unwrap_or(0);
        let docs = db.documents_by_exposant(eid).map(|v| v.len()).unwrap_or(0);
        let pages = db.vitrine_pages_by_exposant(eid).map(|v| v.len()).unwrap_or(0);
        (prods, docs, pages)
    } else {
        (0, 0, 0)
    };

    rsx! {
        aside {
            style: "width: 220px; background: {p.bg_secondary}; border-right: 1px solid {p.border_default}; padding: 16px 0; display: flex; flex-direction: column;",

            // En-tete service
            div {
                style: "padding: 0 16px 16px 16px; border-bottom: 1px solid {p.border_default};",

                div {
                    style: "display: flex; align-items: center; gap: 8px; margin-bottom: 4px;",
                    span { style: "font-size: 20px;", "🏪" }
                    h3 {
                        style: "font-size: 16px; color: {p.text_high};",
                        "JayXpose"
                    }
                }
                p {
                    style: "font-size: 11px; color: {p.text_muted};",
                    "Profil exposant & vitrine"
                }
            }

            // Navigation sections
            nav {
                style: "display: flex; flex-direction: column; gap: 4px; padding-top: 12px; flex: 1;",

                SidebarItem {
                    icon: "📊",
                    label: "Dashboard",
                    is_active: state.read().section == JayXposeSection::Dashboard,
                    onclick: move |_| { state.write().section = JayXposeSection::Dashboard; },
                }
                SidebarItem {
                    icon: "🏢",
                    label: "Mon entreprise",
                    is_active: state.read().section == JayXposeSection::Entreprise,
                    onclick: move |_| { state.write().section = JayXposeSection::Entreprise; },
                }
                SidebarItem {
                    icon: "📦",
                    label: "Catalogue",
                    badge: if product_count > 0 { Some(product_count.to_string()) } else { None },
                    is_active: matches!(state.read().section, JayXposeSection::Catalogue | JayXposeSection::NouveauProduit | JayXposeSection::ModifierProduit),
                    onclick: move |_| { state.write().section = JayXposeSection::Catalogue; },
                }
                SidebarItem {
                    icon: "🏪",
                    label: "Ma vitrine",
                    badge: if page_count > 0 { Some(page_count.to_string()) } else { None },
                    is_active: state.read().section == JayXposeSection::Vitrine,
                    onclick: move |_| { state.write().section = JayXposeSection::Vitrine; },
                }
                SidebarItem {
                    icon: "📁",
                    label: "Documents",
                    badge: if doc_count > 0 { Some(doc_count.to_string()) } else { None },
                    is_active: state.read().section == JayXposeSection::Documents,
                    onclick: move |_| { state.write().section = JayXposeSection::Documents; },
                }

                // Separateur
                div { style: "height: 1px; background: {p.border_default}; margin: 8px 16px;" }

                SidebarItem {
                    icon: "👁️",
                    label: "Fiche publique",
                    is_active: state.read().section == JayXposeSection::FichePublique,
                    onclick: move |_| { state.write().section = JayXposeSection::FichePublique; },
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
    let p = use_palette();
    let bg = if is_active { p.bg_overlay } else { "transparent" };
    let color = if is_active { p.text_high } else { p.text_secondary };
    let border = if is_active {
        format!("2px solid {}", p.accent_primary)
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
                    style: "padding: 2px 6px; background: {p.bg_overlay}; border-radius: 10px; font-size: 10px; color: {p.text_muted};",
                    "{count}"
                }
            }
        }
    }
}
