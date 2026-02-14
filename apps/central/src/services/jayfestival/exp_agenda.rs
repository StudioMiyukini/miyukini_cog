//! EXP-E07/E08 — Agenda exposant (animations, RDV, creneaux).

use dioxus::prelude::*;
use crate::state::use_app_state;

/// Agenda de l'exposant.
#[component]
pub fn ExpAgenda() -> Element {
    let c = use_app_state().read().current_theme.palette();
    let mut active_tab = use_signal(|| "semaine".to_string());

    // Note: Les animations seraient chargées depuis la DB
    // Pour la démo, on utilise des données statiques

    let tab = active_tab.read().clone();
    let tab_jour_bg = if tab == "jour" { c.accent_blue } else { c.bg_secondary };
    let tab_jour_color = if tab == "jour" { "white" } else { c.text_primary };
    let tab_semaine_bg = if tab == "semaine" { c.accent_blue } else { c.bg_secondary };
    let tab_semaine_color = if tab == "semaine" { "white" } else { c.text_primary };
    let tab_mois_bg = if tab == "mois" { c.accent_blue } else { c.bg_secondary };
    let tab_mois_color = if tab == "mois" { "white" } else { c.text_primary };

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

                // Onglets vue
                div {
                    style: "display: flex; gap: 4px;",

                    button {
                        style: "padding: 8px 12px; background: {tab_jour_bg}; border: none; border-radius: 6px; color: {tab_jour_color}; cursor: pointer; font-size: 12px;",
                        onclick: move |_| { active_tab.set("jour".to_string()); },
                        "Jour"
                    }
                    button {
                        style: "padding: 8px 12px; background: {tab_semaine_bg}; border: none; border-radius: 6px; color: {tab_semaine_color}; cursor: pointer; font-size: 12px;",
                        onclick: move |_| { active_tab.set("semaine".to_string()); },
                        "Semaine"
                    }
                    button {
                        style: "padding: 8px 12px; background: {tab_mois_bg}; border: none; border-radius: 6px; color: {tab_mois_color}; cursor: pointer; font-size: 12px;",
                        onclick: move |_| { active_tab.set("mois".to_string()); },
                        "Mois"
                    }
                }
            }

            // Navigation date
            div {
                style: "display: flex; align-items: center; gap: 16px;",

                button {
                    style: "padding: 8px 12px; background: {c.bg_secondary}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 16px;",
                    "←"
                }

                span {
                    style: "font-size: 16px; color: {c.text_white}; font-weight: 500;",
                    "Semaine du 15 février 2026"
                }

                button {
                    style: "padding: 8px 12px; background: {c.bg_secondary}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 16px;",
                    "→"
                }

                button {
                    style: "margin-left: auto; padding: 8px 16px; background: {c.accent_blue}; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 13px;",
                    "Aujourd'hui"
                }
            }

            // Vue semaine
            if tab == "semaine" {
                WeekView {}
            }
            if tab == "jour" {
                DayView {}
            }
            if tab == "mois" {
                MonthView {}
            }

            // Prochains événements
            section {
                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                    "Prochains evenements"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    AgendaItem {
                        title: "Montage du stand",
                        time: "Ven 14 fev - 14:00",
                        event_type: "preparation",
                    }
                    AgendaItem {
                        title: "Ouverture salon",
                        time: "Sam 15 fev - 09:00",
                        event_type: "evenement",
                    }
                    AgendaItem {
                        title: "Demonstration produit",
                        time: "Sam 15 fev - 14:00",
                        event_type: "animation",
                    }
                    AgendaItem {
                        title: "RDV client ABC",
                        time: "Sam 15 fev - 16:30",
                        event_type: "rdv",
                    }
                }
            }
        }
    }
}

#[component]
fn WeekView() -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; overflow: hidden;",

            // En-têtes jours
            div {
                style: "display: grid; grid-template-columns: 60px repeat(7, 1fr); border-bottom: 1px solid {c.border};",

                div { style: "padding: 12px; font-size: 12px; color: {c.text_muted};", "" }
                DayHeader { day: "Lun", date: "10" }
                DayHeader { day: "Mar", date: "11" }
                DayHeader { day: "Mer", date: "12" }
                DayHeader { day: "Jeu", date: "13" }
                DayHeader { day: "Ven", date: "14" }
                DayHeader { day: "Sam", date: "15", is_today: true }
                DayHeader { day: "Dim", date: "16" }
            }

            // Grille horaire (simplifiée)
            div {
                style: "max-height: 400px; overflow-y: auto;",

                TimeRow { hour: "09:00", has_event_at: Some(5) }
                TimeRow { hour: "10:00", has_event_at: None }
                TimeRow { hour: "11:00", has_event_at: None }
                TimeRow { hour: "12:00", has_event_at: None }
                TimeRow { hour: "13:00", has_event_at: None }
                TimeRow { hour: "14:00", has_event_at: Some(5) }
                TimeRow { hour: "15:00", has_event_at: None }
                TimeRow { hour: "16:00", has_event_at: Some(5) }
                TimeRow { hour: "17:00", has_event_at: None }
            }
        }
    }
}

#[component]
fn DayHeader(day: &'static str, date: &'static str, #[props(default = false)] is_today: bool) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let bg = if is_today { c.accent_blue } else { "transparent" };
    let color = if is_today { "white" } else { c.text_primary };

    rsx! {
        div {
            style: "padding: 12px; text-align: center;",

            p {
                style: "font-size: 11px; color: {c.text_muted}; margin-bottom: 4px;",
                "{day}"
            }
            div {
                style: "width: 28px; height: 28px; background: {bg}; border-radius: 50%; display: inline-flex; align-items: center; justify-content: center; font-size: 13px; color: {color}; font-weight: 500;",
                "{date}"
            }
        }
    }
}

#[component]
fn TimeRow(hour: &'static str, has_event_at: Option<usize>) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: grid; grid-template-columns: 60px repeat(7, 1fr); border-bottom: 1px solid {c.border}; min-height: 48px;",

            div {
                style: "padding: 4px 8px; font-size: 11px; color: {c.text_muted}; text-align: right;",
                "{hour}"
            }

            // Cellules jours
            for i in 0..7 {
                {
                    let has_event = has_event_at.map(|idx| idx == i).unwrap_or(false);
                    rsx! {
                        div {
                            style: "border-left: 1px solid {c.border}; padding: 2px;",
                            if has_event {
                                div {
                                    style: "background: {c.accent_blue}40; border-left: 3px solid {c.accent_blue}; border-radius: 2px; padding: 2px 4px; font-size: 10px; color: {c.accent_blue};",
                                    "Event"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DayView() -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

            p {
                style: "text-align: center; color: {c.text_muted}; font-size: 14px;",
                "Vue jour - Selectionnez un jour pour voir le detail"
            }
        }
    }
}

#[component]
fn MonthView() -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

            // En-têtes jours
            div {
                style: "display: grid; grid-template-columns: repeat(7, 1fr); gap: 4px; margin-bottom: 8px;",

                for day in ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"] {
                    div {
                        style: "text-align: center; font-size: 11px; color: {c.text_muted}; padding: 8px;",
                        "{day}"
                    }
                }
            }

            // Grille mois (simplifiée)
            div {
                style: "display: grid; grid-template-columns: repeat(7, 1fr); gap: 4px;",

                for i in 1..=28 {
                    {
                        let has_event = i == 14 || i == 15 || i == 16;
                        let bg = if has_event { c.accent_blue.to_string() + "20" } else { "transparent".to_string() };
                        rsx! {
                            div {
                                style: "aspect-ratio: 1; display: flex; align-items: center; justify-content: center; background: {bg}; border-radius: 4px; font-size: 13px; color: {c.text_primary}; cursor: pointer;",
                                "{i}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AgendaItem(title: &'static str, time: &'static str, event_type: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let (icon, color) = match event_type {
        "preparation" => ("🔧", c.accent_orange),
        "evenement" => ("🎪", c.accent_blue),
        "animation" => ("🎤", c.accent_green),
        "rdv" => ("📅", c.accent_blue),
        _ => ("📌", c.text_muted),
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; background: {c.bg_secondary}; border-radius: 8px; padding: 12px 16px; border-left: 3px solid {color};",

            span { style: "font-size: 20px;", "{icon}" }

            div {
                style: "flex: 1;",
                p {
                    style: "font-size: 14px; color: {c.text_white}; margin-bottom: 2px;",
                    "{title}"
                }
                p {
                    style: "font-size: 12px; color: {c.text_muted};",
                    "{time}"
                }
            }

            button {
                style: "padding: 6px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 12px;",
                "Voir"
            }
        }
    }
}
