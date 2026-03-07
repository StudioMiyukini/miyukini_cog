//! VIS-E04 — Dashboard visiteur.
//!
//! @id: jf_vis_dashboard @do: render_vis_dashboard
//! @role: ui @layer: service
//! @human: Ecran VIS-E04 JayFestival: tableau de bord visiteur.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::StatCard;

/// Dashboard visiteur avec aperçu des événements et activités.
#[component]
pub fn VisDashboard() -> Element {
    let p = use_palette();
    let conns = use_service_connections();

    // Charger les éditions publiées
    let editions = {
        let db = &conns.read().jayfestival;
        db.editions_published().unwrap_or_default()
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            h2 {
                style: "font-size: 24px; color: {p.text_high};",
                "Mon espace visiteur"
            }

            // Stats
            div {
                style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px;",

                StatCard {
                    label: "Billets actifs".to_string(),
                    value: "2".to_string(),
                    icon: "🎟️".to_string(),
                    color: p.success.to_string(),
                }
                StatCard {
                    label: "Reservations".to_string(),
                    value: "3".to_string(),
                    icon: "📅".to_string(),
                    color: p.accent_primary.to_string(),
                }
                StatCard {
                    label: "Jeux en cours".to_string(),
                    value: "1".to_string(),
                    icon: "🎮".to_string(),
                    color: p.warning.to_string(),
                }
                StatCard {
                    label: "Points cumules".to_string(),
                    value: "350".to_string(),
                    icon: "⭐".to_string(),
                    color: p.text_primary.to_string(),
                }
            }

            // Prochain événement
            section {
                h3 {
                    style: "font-size: 16px; color: {p.text_high}; margin-bottom: 12px;",
                    "Prochain evenement"
                }

                if let Some(next_edition) = editions.first() {
                    {
                        let name = next_edition.name.clone().unwrap_or_default();
                        let location = next_edition.location.clone().unwrap_or_default();
                        let date = next_edition.start_date.clone().unwrap_or_default();
                        rsx! {
                            div {
                                style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px; display: flex; align-items: center; gap: 16px;",

                                div {
                                    style: "width: 64px; height: 64px; background: {p.accent_primary}20; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 32px;",
                                    "🎪"
                                }

                                div {
                                    style: "flex: 1;",
                                    h4 {
                                        style: "font-size: 16px; color: {p.text_high}; margin-bottom: 4px;",
                                        "{name}"
                                    }
                                    div {
                                        style: "display: flex; gap: 16px; font-size: 13px; color: {p.text_muted};",
                                        span { "📆 {date}" }
                                        span { "📍 {location}" }
                                    }
                                }

                                button {
                                    style: "padding: 10px 20px; background: {p.accent_primary}; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 14px;",
                                    "Voir les details"
                                }
                            }
                        }
                    }
                } else {
                    div {
                        style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px; text-align: center; color: {p.text_muted}; font-size: 14px;",
                        "Aucun evenement a venir"
                    }
                }
            }

            // Actions rapides
            section {
                h3 {
                    style: "font-size: 16px; color: {p.text_high}; margin-bottom: 12px;",
                    "Actions rapides"
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px;",

                    QuickAction { icon: "🎟️", label: "Mes billets" }
                    QuickAction { icon: "📅", label: "Mon agenda" }
                    QuickAction { icon: "🎮", label: "Jouer" }
                    QuickAction { icon: "🏆", label: "Concours" }
                }
            }

            // Activités récentes
            section {
                h3 {
                    style: "font-size: 16px; color: {p.text_high}; margin-bottom: 12px;",
                    "Activite recente"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    ActivityItem {
                        icon: "🎟️",
                        title: "Billet achete",
                        description: "Salon Printemps 2026 - Entree standard",
                        time: "Il y a 2 jours",
                    }
                    ActivityItem {
                        icon: "📅",
                        title: "Reservation confirmee",
                        description: "Atelier peinture - Samedi 15 fev 14h",
                        time: "Il y a 3 jours",
                    }
                    ActivityItem {
                        icon: "⭐",
                        title: "Points gagnes",
                        description: "+50 points - Quiz du jour complete",
                        time: "Il y a 4 jours",
                    }
                }
            }
        }
    }
}

#[component]
fn QuickAction(icon: &'static str, label: &'static str) -> Element {
    let p = use_palette();

    rsx! {
        button {
            style: "display: flex; flex-direction: column; align-items: center; gap: 8px; padding: 20px; background: {p.bg_secondary}; border: none; border-radius: 8px; cursor: pointer;",

            span { style: "font-size: 28px;", "{icon}" }
            span { style: "font-size: 13px; color: {p.text_primary};", "{label}" }
        }
    }
}

#[component]
fn ActivityItem(icon: &'static str, title: &'static str, description: &'static str, time: &'static str) -> Element {
    let p = use_palette();

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; background: {p.bg_secondary}; border-radius: 8px; padding: 12px 16px;",

            span { style: "font-size: 20px;", "{icon}" }

            div {
                style: "flex: 1;",
                p {
                    style: "font-size: 14px; color: {p.text_high}; margin-bottom: 2px;",
                    "{title}"
                }
                p {
                    style: "font-size: 12px; color: {p.text_muted};",
                    "{description}"
                }
            }

            span {
                style: "font-size: 11px; color: {p.text_muted};",
                "{time}"
            }
        }
    }
}
