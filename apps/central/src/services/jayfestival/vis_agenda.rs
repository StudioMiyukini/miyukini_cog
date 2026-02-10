//! VIS-E05 — Agenda visiteur.

use dioxus::prelude::*;
use crate::state::use_app_state;
use super::components::Badge;

/// Agenda personnel du visiteur.
#[component]
pub fn VisAgenda() -> Element {
    let c = use_app_state().read().current_theme.palette();
    let mut active_view = use_signal(|| "liste".to_string());

    let view = active_view.read().clone();
    let view_liste_bg = if view == "liste" { c.accent_blue } else { c.bg_secondary };
    let view_liste_color = if view == "liste" { "white" } else { c.text_primary };
    let view_cal_bg = if view == "calendrier" { c.accent_blue } else { c.bg_secondary };
    let view_cal_color = if view == "calendrier" { "white" } else { c.text_primary };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h2 {
                    style: "font-size: 24px; color: {c.text_white};",
                    "Mon agenda"
                }

                div {
                    style: "display: flex; gap: 8px;",

                    button {
                        style: "padding: 8px 16px; background: {view_liste_bg}; border: none; border-radius: 6px; color: {view_liste_color}; cursor: pointer; font-size: 13px;",
                        onclick: move |_| { active_view.set("liste".to_string()); },
                        "Liste"
                    }
                    button {
                        style: "padding: 8px 16px; background: {view_cal_bg}; border: none; border-radius: 6px; color: {view_cal_color}; cursor: pointer; font-size: 13px;",
                        onclick: move |_| { active_view.set("calendrier".to_string()); },
                        "Calendrier"
                    }
                }
            }

            // Vue selon l'onglet actif
            if view == "liste" {
                AgendaListView {}
            } else {
                AgendaCalendarView {}
            }
        }
    }
}

#[component]
fn AgendaListView() -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // Aujourd'hui
            section {
                h3 {
                    style: "font-size: 14px; color: {c.text_muted}; margin-bottom: 12px;",
                    "Aujourd'hui - Mardi 10 février"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    AgendaEvent {
                        time: "10:00",
                        title: "Visite Salon Printemps",
                        location: "Hall A - Paris Expo",
                        event_type: "visite",
                    }
                    AgendaEvent {
                        time: "14:00",
                        title: "Atelier peinture",
                        location: "Salle 3",
                        event_type: "reservation",
                    }
                }
            }

            // Demain
            section {
                h3 {
                    style: "font-size: 14px; color: {c.text_muted}; margin-bottom: 12px;",
                    "Demain - Mercredi 11 février"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    AgendaEvent {
                        time: "11:00",
                        title: "Conference marketing",
                        location: "Amphi principal",
                        event_type: "conference",
                    }
                }
            }

            // Samedi
            section {
                h3 {
                    style: "font-size: 14px; color: {c.text_muted}; margin-bottom: 12px;",
                    "Samedi 15 février"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    AgendaEvent {
                        time: "09:00",
                        title: "Ouverture Salon Artisanat",
                        location: "Centre des Congres",
                        event_type: "visite",
                    }
                    AgendaEvent {
                        time: "15:00",
                        title: "Tirage concours",
                        location: "Scene principale",
                        event_type: "concours",
                    }
                }
            }
        }
    }
}

#[component]
fn AgendaEvent(time: &'static str, title: &'static str, location: &'static str, event_type: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let (icon, color) = match event_type {
        "visite" => ("🎪", c.accent_blue),
        "reservation" => ("📅", c.accent_green),
        "conference" => ("🎤", c.accent_orange),
        "concours" => ("🏆", c.accent_blue),
        _ => ("📌", c.text_muted),
    };

    rsx! {
        div {
            style: "display: flex; gap: 16px; background: {c.bg_secondary}; border-radius: 8px; padding: 16px; border-left: 3px solid {color};",

            // Heure
            div {
                style: "width: 60px; text-align: center;",
                span {
                    style: "font-size: 16px; color: {c.text_white}; font-weight: 600;",
                    "{time}"
                }
            }

            // Icône
            span { style: "font-size: 24px;", "{icon}" }

            // Contenu
            div {
                style: "flex: 1;",
                h4 {
                    style: "font-size: 14px; color: {c.text_white}; margin-bottom: 4px;",
                    "{title}"
                }
                p {
                    style: "font-size: 12px; color: {c.text_muted};",
                    "📍 {location}"
                }
            }

            // Actions
            button {
                style: "padding: 6px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 12px;",
                "Voir"
            }
        }
    }
}

#[component]
fn AgendaCalendarView() -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

            // Navigation mois
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",

                button {
                    style: "padding: 8px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer;",
                    "←"
                }

                span {
                    style: "font-size: 18px; color: {c.text_white}; font-weight: 600;",
                    "Février 2026"
                }

                button {
                    style: "padding: 8px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer;",
                    "→"
                }
            }

            // En-têtes jours
            div {
                style: "display: grid; grid-template-columns: repeat(7, 1fr); gap: 4px; margin-bottom: 8px;",

                for day in ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"] {
                    div {
                        style: "text-align: center; font-size: 12px; color: {c.text_muted}; padding: 8px;",
                        "{day}"
                    }
                }
            }

            // Grille calendrier
            div {
                style: "display: grid; grid-template-columns: repeat(7, 1fr); gap: 4px;",

                // Jours vides du début
                for _ in 0..6 {
                    div { style: "aspect-ratio: 1;" }
                }

                // Jours du mois
                for day in 1..=28 {
                    {
                        let has_event = day == 10 || day == 11 || day == 15;
                        let is_today = day == 10;
                        let bg = if is_today { c.accent_blue.to_string() } else if has_event { format!("{}20", c.accent_blue) } else { "transparent".to_string() };
                        let color = if is_today { "white" } else { c.text_primary };
                        rsx! {
                            div {
                                style: "aspect-ratio: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; background: {bg}; border-radius: 8px; cursor: pointer; position: relative;",

                                span {
                                    style: "font-size: 14px; color: {color};",
                                    "{day}"
                                }

                                if has_event && !is_today {
                                    div {
                                        style: "width: 6px; height: 6px; background: {c.accent_blue}; border-radius: 50%; position: absolute; bottom: 6px;",
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
