//! EXP-E04 — Dashboard exposant (candidatures, participations, alertes).

use super::components::{EmptyState, StatCard};
use dioxus::prelude::*;
use miyukini_service_ui::use_palette;

#[component]
pub fn ExpDashboard() -> Element {
    let c = use_palette();
    // Charger les données exposant (premier exposant de la DB pour la démo)
    let exposant = {
        let db = crate::use_db();
        db.exposants_list(false)
            .unwrap_or_default()
            .into_iter()
            .next()
    };

    if exposant.is_none() {
        return rsx! {
            EmptyState {
                title: "Aucun profil exposant".to_string(),
                message: "Creez votre profil exposant pour acceder a votre tableau de bord".to_string(),
                icon: "🏪".to_string(),
            }
        };
    }

    let company = exposant
        .as_ref()
        .and_then(|e| e.company_name.clone())
        .unwrap_or_else(|| "Mon entreprise".to_string());

    // Charger les éditions pour compter les participations
    let editions = {
        let db = crate::use_db();
        db.editions_list().unwrap_or_default()
    };

    let pending_count = {
        let db = crate::use_db();
        db.candidatures_pending_count().unwrap_or(0)
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            h2 {
                style: "font-size: 24px; color: {c.text_white};",
                "Bienvenue, {company}"
            }

            // Stats
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                StatCard {
                    label: "Candidatures en cours".to_string(),
                    value: pending_count.to_string(),
                    icon: "📝".to_string(),
                    color: c.accent_orange.to_string(),
                }
                StatCard {
                    label: "Evenements disponibles".to_string(),
                    value: editions.len().to_string(),
                    icon: "📅".to_string(),
                    color: c.accent_blue.to_string(),
                }
                StatCard {
                    label: "Participations validees".to_string(),
                    value: "0".to_string(),
                    icon: "🎪".to_string(),
                    color: c.accent_green.to_string(),
                }
            }

            // Alertes
            h3 {
                style: "font-size: 16px; color: {c.text_white};",
                "Alertes"
            }
            div {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px; color: {c.text_secondary}; font-size: 14px;",
                "Aucune alerte pour le moment."
            }

            // Prochain événement
            h3 {
                style: "font-size: 16px; color: {c.text_white};",
                "Prochain evenement"
            }
            if let Some(next) = editions.first() {
                div {
                    style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px; display: flex; align-items: center; gap: 12px;",

                    span { style: "font-size: 24px;", "🎪" }
                    div {
                        span {
                            style: "color: {c.text_white}; font-size: 14px; font-weight: 500;",
                            "{next.name.clone().unwrap_or_default()}"
                        }
                        div {
                            style: "font-size: 12px; color: {c.text_secondary}; margin-top: 4px;",
                            "{next.start_date.clone().unwrap_or_default()} — {next.location.clone().unwrap_or_default()}"
                        }
                    }
                }
            } else {
                div {
                    style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px; color: {c.text_muted}; text-align: center; font-size: 14px;",
                    "Aucun evenement a venir"
                }
            }
        }
    }
}
