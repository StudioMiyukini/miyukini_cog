//! Sidebar JayKoa — Panneau lateral gauche type Google Agenda.
//!
//! Contient :
//! - Bouton de creation d'evenement
//! - Mini-calendrier (date picker)
//! - Liste des agendas avec filtrage
//! - Services synchronises (JayRDV)

use chrono::{Datelike, NaiveDate};
use dioxus::prelude::*;
use jaykoa::data::Agenda;
use miyukini_service_ui::use_palette;

/// Props pour la sidebar JayKoa.
#[derive(Props, Clone, PartialEq)]
pub struct JayKoaSidebarProps {
    /// Liste des agendas de l'utilisateur.
    pub agendas: Vec<Agenda>,
    /// Date courante selectionnee.
    pub current_date: NaiveDate,
    /// Callback quand une date est selectionnee dans le mini-calendrier.
    pub on_date_select: EventHandler<NaiveDate>,
    /// Callback quand un agenda est toggle (visible/masque).
    pub on_agenda_toggle: EventHandler<String>,
    /// Callback pour creer un nouvel evenement.
    pub on_create_event: EventHandler<()>,
}

/// Sidebar JayKoa — Panneau lateral gauche.
#[component]
pub fn JayKoaSidebar(props: JayKoaSidebarProps) -> Element {
    let c = use_palette();

    // Etat local pour le mois affiche dans le mini-calendrier
    let mut displayed_month = use_signal(|| props.current_date);

    // Separer les agendas personnels et synchronises
    let personal_agendas: Vec<_> = props
        .agendas
        .iter()
        .filter(|a| a.source_service.is_none())
        .cloned()
        .collect();
    let synced_agendas: Vec<_> = props
        .agendas
        .iter()
        .filter(|a| a.source_service.is_some())
        .cloned()
        .collect();

    rsx! {
        div {
            style: "width: 280px; background: {c.bg_secondary}; border-right: 1px solid {c.border}; display: flex; flex-direction: column; overflow-y: auto;",

            // Bouton Creer
            div {
                style: "padding: 16px;",
                button {
                    style: "display: flex; align-items: center; gap: 12px; padding: 12px 24px; background: {c.accent_blue}; color: white; border: none; border-radius: 24px; cursor: pointer; font-size: 14px; font-weight: 500; box-shadow: 0 2px 8px rgba(26, 159, 255, 0.3); transition: all 0.2s;",
                    onclick: move |_| props.on_create_event.call(()),
                    span { style: "font-size: 20px;", "+" }
                    "Creer"
                }
            }

            // Mini-calendrier
            MiniCalendar {
                displayed_month: *displayed_month.read(),
                selected_date: props.current_date,
                on_date_select: move |date| props.on_date_select.call(date),
                on_month_change: move |date| displayed_month.set(date),
            }

            // Separateur
            div { style: "height: 1px; background: {c.border}; margin: 8px 16px;" }

            // Mes agendas
            div {
                style: "padding: 8px 16px;",
                h3 {
                    style: "font-size: 11px; font-weight: 600; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 8px;",
                    "Mes agendas"
                }
                for agenda in personal_agendas.iter() {
                    AgendaItem {
                        agenda: agenda.clone(),
                        on_toggle: move |id| props.on_agenda_toggle.call(id),
                    }
                }
            }

            // Services synchronises
            if !synced_agendas.is_empty() {
                div {
                    style: "padding: 8px 16px;",
                    h3 {
                        style: "font-size: 11px; font-weight: 600; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 8px;",
                        "Services synchronises"
                    }
                    for agenda in synced_agendas.iter() {
                        AgendaItem {
                            agenda: agenda.clone(),
                            on_toggle: move |id| props.on_agenda_toggle.call(id),
                        }
                    }
                }
            }

            // Spacer
            div { style: "flex: 1;" }

            // Info JayKoa
            div {
                style: "padding: 16px; border-top: 1px solid {c.border};",
                p {
                    style: "font-size: 10px; color: {c.text_muted}; text-align: center;",
                    "JayKoa — Calendrier universel COG"
                }
            }
        }
    }
}

/// Props pour le mini-calendrier.
#[derive(Props, Clone, PartialEq)]
struct MiniCalendarProps {
    displayed_month: NaiveDate,
    selected_date: NaiveDate,
    on_date_select: EventHandler<NaiveDate>,
    on_month_change: EventHandler<NaiveDate>,
}

/// Mini-calendrier compact (date picker).
#[component]
fn MiniCalendar(props: MiniCalendarProps) -> Element {
    let c = use_palette();

    let year = props.displayed_month.year();
    let month = props.displayed_month.month();
    let today = chrono::Local::now().date_naive();

    // Premier jour du mois
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap_or(today);
    // Decalage pour commencer le lundi
    let start_offset = i64::from(first_day.weekday().num_days_from_monday());
    // Premier jour affiche (peut etre du mois precedent)
    let start_date = first_day - chrono::Duration::days(start_offset);

    // Nombre de jours dans le mois
    let days_in_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }
    .map_or(30, |d| d.pred_opt().unwrap_or(d).day());

    // Nombre de semaines a afficher
    let total_days = start_offset as u32 + days_in_month;
    let weeks = total_days.div_ceil(7);

    // Noms des jours de la semaine
    let weekdays = ["L", "M", "M", "J", "V", "S", "D"];

    // Nom du mois en francais
    let month_name = match month {
        1 => "Janvier",
        2 => "Fevrier",
        3 => "Mars",
        4 => "Avril",
        5 => "Mai",
        6 => "Juin",
        7 => "Juillet",
        8 => "Aout",
        9 => "Septembre",
        10 => "Octobre",
        11 => "Novembre",
        12 => "Decembre",
        _ => "?",
    };

    rsx! {
        div {
            style: "padding: 8px 16px;",

            // Header du mini-calendrier
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;",

                button {
                    style: "padding: 4px 8px; background: transparent; border: none; color: {c.text_secondary}; cursor: pointer; font-size: 14px;",
                    onclick: move |_| {
                        let new_date = if month == 1 {
                            NaiveDate::from_ymd_opt(year - 1, 12, 1).unwrap_or(props.displayed_month)
                        } else {
                            NaiveDate::from_ymd_opt(year, month - 1, 1).unwrap_or(props.displayed_month)
                        };
                        props.on_month_change.call(new_date);
                    },
                    "<"
                }

                span {
                    style: "font-size: 13px; font-weight: 500; color: {c.text_primary};",
                    "{month_name} {year}"
                }

                button {
                    style: "padding: 4px 8px; background: transparent; border: none; color: {c.text_secondary}; cursor: pointer; font-size: 14px;",
                    onclick: move |_| {
                        let new_date = if month == 12 {
                            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap_or(props.displayed_month)
                        } else {
                            NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap_or(props.displayed_month)
                        };
                        props.on_month_change.call(new_date);
                    },
                    ">"
                }
            }

            // Jours de la semaine
            div {
                style: "display: grid; grid-template-columns: repeat(7, 1fr); gap: 2px; margin-bottom: 4px;",
                for day in weekdays.iter() {
                    span {
                        style: "text-align: center; font-size: 10px; color: {c.text_muted}; padding: 4px 0;",
                        "{day}"
                    }
                }
            }

            // Grille des jours
            div {
                style: "display: grid; grid-template-columns: repeat(7, 1fr); gap: 2px;",
                for week in 0..weeks {
                    for day_of_week in 0..7u32 {
                        {
                            let day_index = i64::from(week * 7 + day_of_week);
                            let date = start_date + chrono::Duration::days(day_index);
                            let is_current_month = date.month() == month;
                            let is_today = date == today;
                            let is_selected = date == props.selected_date;

                            let bg = if is_selected {
                                c.accent_blue.to_string()
                            } else if is_today {
                                format!("{}40", c.accent_blue)
                            } else {
                                "transparent".to_string()
                            };

                            let color = if is_selected {
                                "white"
                            } else if !is_current_month {
                                c.text_muted
                            } else {
                                c.text_primary
                            };

                            rsx! {
                                button {
                                    key: "{date}",
                                    style: "width: 28px; height: 28px; display: flex; align-items: center; justify-content: center; background: {bg}; border: none; border-radius: 50%; color: {color}; font-size: 11px; cursor: pointer;",
                                    onclick: move |_| props.on_date_select.call(date),
                                    "{date.day()}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Props pour un item d'agenda.
#[derive(Props, Clone, PartialEq)]
struct AgendaItemProps {
    agenda: Agenda,
    on_toggle: EventHandler<String>,
}

/// Item d'agenda dans la liste.
#[component]
fn AgendaItem(props: AgendaItemProps) -> Element {
    let c = use_palette();

    let color = props.agenda.color.as_deref().unwrap_or("#4285F4");
    let name = props.agenda.name.as_deref().unwrap_or("Sans nom");
    let id = props.agenda.id.clone().unwrap_or_default();
    let visible = props.agenda.visible;

    // Icone de source pour les agendas synchronises
    let source_icon = match props.agenda.source_service.as_deref() {
        Some("jayrdv") => Some("R"),
        _ => None,
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 8px; padding: 6px 8px; border-radius: 4px; cursor: pointer; transition: background 0.2s;",
            onclick: move |_| props.on_toggle.call(id.clone()),

            // Checkbox
            {
                let checkbox_bg = if visible { color.to_string() } else { "transparent".to_string() };
                rsx! {
                    div {
                        style: "width: 16px; height: 16px; border-radius: 3px; border: 2px solid {color}; background: {checkbox_bg}; display: flex; align-items: center; justify-content: center;",
                        if visible {
                            span {
                                style: "color: white; font-size: 10px; font-weight: bold;",
                                "v"
                            }
                        }
                    }
                }
            }

            // Nom de l'agenda
            span {
                style: "flex: 1; font-size: 13px; color: {c.text_primary}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                "{name}"
            }

            // Icone de source
            if let Some(icon) = source_icon {
                span {
                    style: "font-size: 12px;",
                    "{icon}"
                }
            }
        }
    }
}
