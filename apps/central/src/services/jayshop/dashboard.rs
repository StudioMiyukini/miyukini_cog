//! Dashboard JayShop (JSH-A01) — Tableau de bord avec KPIs et résumé.
//!
//! @id: central.services.jayshop.dashboard
//! @do: render_jayshop_dashboard_kpis
//! @role: ui
//! @layer: ui

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::{JayShopSection, JayShopState};

/// Tableau de bord JayShop — KPIs et résumé des ventes.
#[component]
pub fn Dashboard(state: Signal<JayShopState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let _conns = use_service_connections();

    // KPIs placeholder (seront remplis depuis la DB)
    let kpis = vec![
        ("Ventes brutes", "0,00 €", "📈"),
        ("Remboursements", "0,00 €", "↩️"),
        ("Remises", "0,00 €", "🏷️"),
        ("Ventes nettes", "0,00 €", "💰"),
        ("Profit brut", "0,00 €", "✨"),
    ];

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // Titre
            div {
                style: "display: flex; align-items: center; justify-content: space-between;",

                div {
                    style: "display: flex; align-items: center; gap: 12px;",
                    h2 {
                        style: "font-size: 24px; color: {c.text_white};",
                        "📊 Tableau de bord"
                    }
                }

                // Bouton ouvrir le PoS
                button {
                    style: "display: flex; align-items: center; gap: 8px; padding: 10px 20px; background: {c.accent_blue}; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 14px; font-weight: 600;",
                    onclick: move |_| {
                        state.write().section = JayShopSection::Pos;
                    },
                    span { "🖥️" }
                    span { "Ouvrir le PoS" }
                }
            }

            // Bande KPIs
            div {
                style: "display: grid; grid-template-columns: repeat(5, 1fr); gap: 12px;",

                for (label, value, icon) in kpis.iter() {
                    div {
                        style: "padding: 16px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 8px;",

                        div {
                            style: "display: flex; align-items: center; gap: 8px; margin-bottom: 8px;",
                            span { style: "font-size: 16px;", "{icon}" }
                            span { style: "font-size: 12px; color: {c.text_muted};", "{label}" }
                        }
                        div {
                            style: "font-size: 20px; font-weight: 700; color: {c.text_white};",
                            "{value}"
                        }
                    }
                }
            }

            // Raccourcis rapides
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px;",

                // Dernières ventes
                div {
                    style: "padding: 20px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 8px; min-height: 200px;",

                    h3 {
                        style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                        "🧾 Dernières ventes"
                    }
                    p {
                        style: "font-size: 13px; color: {c.text_muted};",
                        "Aucune vente pour le moment."
                    }
                }

                // Produits les plus vendus
                div {
                    style: "padding: 20px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 8px; min-height: 200px;",

                    h3 {
                        style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                        "🏆 Produits populaires"
                    }
                    p {
                        style: "font-size: 13px; color: {c.text_muted};",
                        "Pas de données de vente disponibles."
                    }
                }

                // Événements actifs
                div {
                    style: "padding: 20px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 8px; min-height: 200px;",

                    h3 {
                        style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                        "🎪 Événements actifs"
                    }
                    p {
                        style: "font-size: 13px; color: {c.text_muted};",
                        "Aucun événement en cours."
                    }
                }
            }

            // Section caisse
            div {
                style: "padding: 20px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 8px;",

                div {
                    style: "display: flex; align-items: center; justify-content: space-between;",

                    div {
                        h3 {
                            style: "font-size: 16px; color: {c.text_white}; margin-bottom: 4px;",
                            "💰 Session de caisse"
                        }
                        p {
                            style: "font-size: 13px; color: {c.text_muted};",
                            "Aucune session ouverte."
                        }
                    }

                    button {
                        style: "padding: 8px 16px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; cursor: pointer; font-size: 13px;",
                        onclick: move |_| {
                            state.write().section = JayShopSection::CashOpen;
                        },
                        "Ouvrir une caisse"
                    }
                }
            }
        }
    }
}
