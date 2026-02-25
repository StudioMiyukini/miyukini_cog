//! Agrégation JayManga — portail agrégé, vendeurs indexés, configuration.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::{Badge, EmptyState, PageHeader, StatCard};
use super::JayMangaState;

#[component]
pub fn Aggregation(state: Signal<JayMangaState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    let db = &conns.read().jaymanga;
    let agg_config = db.aggregator_config_get()
        .unwrap_or_else(|_| jaymanga::data::AggregatorConfig::defaults());
    let indexed_sellers = db.indexed_seller_list().unwrap_or_default();
    let aggregated_entries = db.aggregated_entry_list(&jaymanga::data::AggregatorFilters::default()).unwrap_or_default();

    let is_enabled = agg_config.enabled.unwrap_or(false);
    let portal_name = agg_config.name.clone().unwrap_or_else(|| "Portail Agrégé".to_string());
    let seller_count = indexed_sellers.len();
    let entry_count = aggregated_entries.len();

    let online_sellers = indexed_sellers.iter()
        .filter(|s| s.online_status.as_deref() == Some("online"))
        .count();
    let bg_toggle = if is_enabled { c.accent_red } else { c.accent_green };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            PageHeader {
                title: "🌐 Agrégation".to_string(),
                subtitle: format!("{portal_name} — Portail inter-COG"),
            }

            // Statut
            div {
                style: "display: flex; align-items: center; gap: 12px; padding: 16px; background: {c.bg_secondary}; border-radius: 8px;",

                span { style: "font-size: 24px;", if is_enabled { "🟢" } else { "🔴" } }
                div {
                    h3 {
                        style: "font-size: 16px; color: {c.text_white};",
                        if is_enabled { "Portail agrégé actif" } else { "Portail agrégé désactivé" }
                    }
                    p {
                        style: "font-size: 12px; color: {c.text_muted}; margin-top: 2px;",
                        if is_enabled {
                            {format!("Synchronisation toutes les {} minutes", agg_config.sync_interval_minutes.unwrap_or(30))}
                        } else {
                            "Activez le portail pour agréger les catalogues d\u{2019}autres COG."
                        }
                    }
                }

                div { style: "flex: 1;" }

                button {
                    style: "padding: 10px 20px; background: {bg_toggle}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 13px;",
                    onclick: move |_| {
                        let db = &conns.read().jaymanga;
                        let mut cfg = db.aggregator_config_get()
                            .unwrap_or_else(|_| jaymanga::data::AggregatorConfig::defaults());
                        cfg.enabled = Some(!is_enabled);
                        let _ = db.aggregator_config_upsert(&cfg);
                    },
                    if is_enabled { "Désactiver" } else { "Activer" }
                }
            }

            // Statistiques
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                StatCard { label: "Vendeurs indexés".to_string(), value: seller_count.to_string(), icon: "👤".to_string(), color: c.accent_blue.to_string() }
                StatCard { label: "Œuvres agrégées".to_string(), value: entry_count.to_string(), icon: "📚".to_string(), color: "#8b5cf6".to_string() }
                StatCard { label: "En ligne".to_string(), value: online_sellers.to_string(), icon: "🟢".to_string(), color: c.accent_green.to_string() }
            }

            // Vendeurs indexés
            h3 {
                style: "font-size: 16px; color: {c.text_white};",
                "Vendeurs indexés"
            }

            if indexed_sellers.is_empty() {
                EmptyState {
                    title: "Aucun vendeur".to_string(),
                    message: "Les vendeurs découverts via le Webway apparaîtront ici.".to_string(),
                    icon: "👤".to_string(),
                }
            } else {
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    for seller in indexed_sellers.iter() {
                        {
                            let cog_id = seller.cog_id.clone().unwrap_or_default();
                            let cog_short = if cog_id.len() > 12 { format!("{}...", &cog_id[..12]) } else { cog_id.clone() };
                            let name = seller.shop_name.clone().unwrap_or_else(|| "Boutique sans nom".to_string());
                            let work_count = seller.work_count.unwrap_or(0);
                            let online = seller.online_status.clone().unwrap_or_else(|| "unknown".to_string());
                            let blocked = seller.blocked.unwrap_or(false);

                            let (status_color, status_label) = match online.as_str() {
                                "online" => (c.accent_green, "En ligne"),
                                "offline" => (c.text_muted, "Hors ligne"),
                                _ => (c.accent_orange, "Inconnu"),
                            };

                            let cid_block = cog_id.clone();

                            rsx! {
                                div {
                                    style: "display: flex; align-items: center; gap: 16px; padding: 12px 16px; background: {c.bg_secondary}; border-radius: 4px; border: 1px solid {c.border};",

                                    // Avatar placeholder
                                    div {
                                        style: "width: 40px; height: 40px; background: {c.bg_hover}; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 18px; flex-shrink: 0;",
                                        "👤"
                                    }

                                    div {
                                        style: "flex: 1;",
                                        div {
                                            style: "display: flex; align-items: center; gap: 8px;",
                                            p {
                                                style: "font-size: 14px; color: {c.text_white}; font-weight: 500;",
                                                "{name}"
                                            }
                                            Badge { text: status_label.to_string(), color: status_color.to_string() }
                                            if blocked {
                                                Badge { text: "Bloqué".to_string(), color: c.accent_red.to_string() }
                                            }
                                        }
                                        div {
                                            style: "font-size: 12px; color: {c.text_muted}; margin-top: 2px;",
                                            span { "{cog_short}" }
                                            span { " — {work_count} œuvre(s)" }
                                        }
                                    }

                                    button {
                                        style: "padding: 6px 12px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 12px;",
                                        onclick: move |_| {
                                            let db = &conns.read().jaymanga;
                                            if let Ok(Some(mut s)) = db.indexed_seller_by_id(&cid_block) {
                                                s.blocked = Some(!blocked);
                                                let _ = db.indexed_seller_upsert(&s);
                                            }
                                        },
                                        if blocked { "Débloquer" } else { "Bloquer" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
