//! VIS-E07 — Reservations visiteur.
//!
//! @id: jf_vis_reservations @do: render_vis_reservations
//! @role: ui @layer: service
//! @human: Ecran VIS-E07 JayFestival: reservations visiteur.

use dioxus::prelude::*;
use crate::state::use_app_state;
use super::components::{Badge, ActionButton};

/// Gestion des réservations du visiteur.
#[component]
pub fn VisReservations() -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h2 {
                    style: "font-size: 24px; color: {c.text_white};",
                    "Mes reservations"
                }

                ActionButton {
                    label: "Nouvelle reservation".to_string(),
                    icon: "📅".to_string(),
                    accent: true,
                    onclick: move |_| {},
                }
            }

            // Réservations à venir
            section {
                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                    "A venir"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    ReservationCard {
                        title: "Atelier peinture sur bois",
                        event: "Salon Printemps 2026",
                        date: "Sam 15 fev - 14:00",
                        location: "Salle 3 - Espace ateliers",
                        places: "2 places",
                        status: "confirmee",
                    }
                    ReservationCard {
                        title: "Conference marketing digital",
                        event: "Salon Printemps 2026",
                        date: "Sam 15 fev - 16:00",
                        location: "Amphitheatre principal",
                        places: "1 place",
                        status: "confirmee",
                    }
                    ReservationCard {
                        title: "Demonstration culinaire",
                        event: "Salon Artisanat",
                        date: "Dim 16 mars - 11:00",
                        location: "Scene cuisine",
                        places: "2 places",
                        status: "en_attente",
                    }
                }
            }

            // Réservations passées
            section {
                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                    "Passees"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    ReservationCard {
                        title: "Atelier decoration",
                        event: "Salon Hiver 2025",
                        date: "Sam 12 dec 2025 - 10:00",
                        location: "Salle 2",
                        places: "1 place",
                        status: "terminee",
                    }
                }
            }
        }
    }
}

#[component]
fn ReservationCard(
    title: &'static str,
    event: &'static str,
    date: &'static str,
    location: &'static str,
    places: &'static str,
    status: &'static str,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let (status_label, status_color) = match status {
        "confirmee" => ("Confirmee", c.accent_green),
        "en_attente" => ("En attente", c.accent_orange),
        "terminee" => ("Terminee", c.text_muted),
        "annulee" => ("Annulee", c.accent_red),
        _ => ("Inconnue", c.text_muted),
    };

    let border = if status == "confirmee" {
        format!("1px solid {}40", c.accent_green)
    } else {
        format!("1px solid {}", c.border)
    };

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border: {border}; border-radius: 8px; padding: 16px;",

            div {
                style: "display: flex; gap: 16px;",

                // Icône
                div {
                    style: "width: 48px; height: 48px; background: {c.bg_hover}; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 24px;",
                    "📅"
                }

                // Contenu
                div {
                    style: "flex: 1;",

                    div {
                        style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 8px;",

                        div {
                            h4 {
                                style: "font-size: 14px; color: {c.text_white}; margin-bottom: 4px;",
                                "{title}"
                            }
                            p {
                                style: "font-size: 12px; color: {c.text_muted};",
                                "{event}"
                            }
                        }

                        Badge {
                            text: status_label.to_string(),
                            color: status_color.to_string(),
                        }
                    }

                    div {
                        style: "display: flex; gap: 16px; font-size: 12px; color: {c.text_secondary};",

                        span { "🕐 {date}" }
                        span { "📍 {location}" }
                        span { "👥 {places}" }
                    }
                }
            }

            // Actions pour les réservations à venir
            if status == "confirmee" || status == "en_attente" {
                div {
                    style: "display: flex; gap: 8px; margin-top: 12px; padding-top: 12px; border-top: 1px solid {c.border};",

                    button {
                        style: "padding: 6px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 12px;",
                        "📅 Ajouter au calendrier"
                    }
                    button {
                        style: "padding: 6px 12px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_muted}; cursor: pointer; font-size: 12px;",
                        "Annuler"
                    }
                }
            }
        }
    }
}
