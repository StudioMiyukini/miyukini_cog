//! EXP-E05/E08/E10 — Candidatures exposant + annuaire événements.
//!
//! @id: jf_exp_candidatures @do: render_exp_candidatures
//! @role: ui @layer: service
//! @human: Ecrans EXP-E05/E08/E10 JayFestival: candidatures exposant et annuaire evenements.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::{Badge, EmptyState};

#[component]
pub fn ExpCandidatures() -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    // Charger les éditions disponibles (annuaire)
    let editions = {
        let db = &conns.read().jayfestival;
        db.editions_list().unwrap_or_default()
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            h2 {
                style: "font-size: 24px; color: {c.text_white};",
                "Candidatures"
            }

            // Annuaire des événements ouverts
            h3 {
                style: "font-size: 16px; color: {c.text_white};",
                "Evenements ouverts aux candidatures"
            }

            if editions.is_empty() {
                EmptyState {
                    title: "Aucun evenement disponible".to_string(),
                    message: "Les evenements ouverts aux candidatures apparaitront ici".to_string(),
                    icon: "📋".to_string(),
                }
            } else {
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    for edition in editions.iter() {
                        {
                            let name = edition.name.clone().unwrap_or_else(|| "Sans nom".to_string());
                            let location = edition.location.clone().unwrap_or_default();
                            let start = edition.start_date.clone().unwrap_or_default();
                            let status = edition.status.clone().unwrap_or_else(|| "brouillon".to_string());
                            let eid = edition.id.clone().unwrap_or_default();
                            let conns_apply = conns;
                            rsx! {
                                div {
                                    style: "display: flex; align-items: center; gap: 12px; background: {c.bg_secondary}; border-radius: 8px; padding: 14px 16px;",

                                    span { style: "font-size: 24px;", "🎪" }

                                    div {
                                        style: "flex: 1;",
                                        div {
                                            style: "color: {c.text_white}; font-size: 14px; font-weight: 500;",
                                            "{name}"
                                        }
                                        div {
                                            style: "font-size: 12px; color: {c.text_secondary}; margin-top: 4px;",
                                            if !start.is_empty() {
                                                span { "📆 {start}" }
                                            }
                                            if !location.is_empty() {
                                                span { " | 📍 {location}" }
                                            }
                                        }
                                    }

                                    Badge {
                                        text: status.clone(),
                                        color: c.text_secondary.to_string(),
                                    }

                                    button {
                                        style: "padding: 8px 16px; background: {c.accent_blue}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 12px;",
                                        onclick: move |_| {
                                            // Déposer une candidature (utilise un exposant fictif pour la démo)
                                            let db = &conns_apply.read().jayfestival;
                                            // Prendre le premier exposant existant
                                            if let Some(exp) = db.exposants_list(false).unwrap_or_default().first() {
                                                if let Some(ref exp_id) = exp.id {
                                                    let _ = db.editions_exposants_create(exp_id, &eid);
                                                }
                                            }
                                        },
                                        "Candidater"
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
