//! VIS — Catalogue des éditions publiées + agenda visiteur.
//!
//! @id: jf_vis_catalogue @do: render_vis_catalogue
//! @role: ui @layer: service
//! @human: Ecran VIS JayFestival: catalogue des editions publiees et agenda visiteur.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use crate::services::jaykoa::sync_service;
use super::components::{Badge, EmptyState, format_date_range};

const DEFAULT_PROFILE: &str = "default";

#[component]
pub fn VisCatalogue() -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    // Charger les éditions (filtre publiées/en cours de préférence)
    let editions = {
        let db = &conns.read().jayfestival;
        db.editions_list().unwrap_or_default()
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            h2 {
                style: "font-size: 24px; color: {c.text_white};",
                "Catalogue des evenements"
            }

            p {
                style: "font-size: 14px; color: {c.text_secondary};",
                "Decouvrez les evenements, consultez les programmes et ajoutez-les a votre agenda."
            }

            if editions.is_empty() {
                EmptyState {
                    title: "Aucun evenement publie".to_string(),
                    message: "Les evenements publies apparaitront ici".to_string(),
                    icon: "📋".to_string(),
                }
            } else {
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 16px;",

                    for edition in editions.iter() {
                        {
                            let name = edition.name.clone().unwrap_or_else(|| "Sans nom".to_string());
                            let location = edition.location.clone().unwrap_or_default();
                            let date_range = format_date_range(edition.start_date.as_ref(), edition.end_date.as_ref());
                            let status = edition.status.clone().unwrap_or_else(|| "brouillon".to_string());
                            let theme_text = edition.theme.clone().unwrap_or_default();
                            let edition_id = edition.id.clone().unwrap_or_default();
                            let status_color = match status.as_str() {
                                "en_cours" | "En cours" => c.accent_green,
                                "publie" | "Publie" => c.accent_blue,
                                _ => c.text_secondary,
                            };
                            rsx! {
                                div {
                                    style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; display: flex; flex-direction: column; gap: 12px; cursor: pointer; transition: all 0.2s;",

                                    // En-tête
                                    div {
                                        style: "display: flex; justify-content: space-between; align-items: flex-start;",
                                        h3 {
                                            style: "font-size: 16px; color: {c.text_white};",
                                            "{name}"
                                        }
                                        Badge {
                                            text: status.clone(),
                                            color: status_color.to_string(),
                                        }
                                    }

                                    // Détails
                                    div {
                                        style: "display: flex; flex-direction: column; gap: 6px; font-size: 13px; color: {c.text_secondary};",

                                        div { "📆 {date_range}" }
                                        if !location.is_empty() {
                                            div { "📍 {location}" }
                                        }
                                        if !theme_text.is_empty() {
                                            div { "🎨 {theme_text}" }
                                        }
                                    }

                                    // Action — Ajouter au calendrier JayKoa
                                    button {
                                        style: "align-self: flex-start; padding: 8px 16px; background: {c.accent_blue}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 12px;",
                                        onclick: move |_| {
                                            if !edition_id.is_empty() {
                                                let conns_ref = conns.read();
                                                let _ = sync_service::JayFestivalSync::sync_single_edition(
                                                    &conns_ref.jaykoa,
                                                    &conns_ref.jayfestival,
                                                    DEFAULT_PROFILE,
                                                    &edition_id,
                                                );
                                            }
                                        },
                                        "📅 Ajouter à mon agenda"
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
