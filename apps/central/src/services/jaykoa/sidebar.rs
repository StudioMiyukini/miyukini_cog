//! Sidebar JayKoa — Panneau latéral gauche type Google Agenda.
//!
//! Contient :
//! - Bouton de création d'événement
//! - Mini-calendrier (date picker)
//! - Liste des agendas avec filtrage
//! - Services synchronisés (JayFestival, JayRDV)

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::state::use_app_state;
use jaykoa::data::Agenda;
use chrono::{Datelike, NaiveDate};

/// Props pour la sidebar JayKoa.
#[derive(Props, Clone, PartialEq)]
pub struct JayKoaSidebarProps {
    /// Liste des agendas de l'utilisateur.
    pub agendas: Vec<Agenda>,
    /// Date courante sélectionnée.
    pub current_date: NaiveDate,
    /// Callback quand une date est sélectionnée dans le mini-calendrier.
    pub on_date_select: EventHandler<NaiveDate>,
    /// Callback quand un agenda est togglé (visible/masqué).
    pub on_agenda_toggle: EventHandler<String>,
    /// Callback pour créer un nouvel événement.
    pub on_create_event: EventHandler<()>,
    /// Callback pour synchroniser JayFestival.
    pub on_sync_jayfestival: EventHandler<()>,
}

/// Sidebar JayKoa — Panneau latéral gauche.
#[component]
pub fn JayKoaSidebar(props: JayKoaSidebarProps) -> Element {
    let state = use_app_state();
    let p = use_palette();
    
    // État local pour le mois affiché dans le mini-calendrier
    let mut displayed_month = use_signal(|| props.current_date);
    
    // Séparer les agendas personnels et synchronisés
    let personal_agendas: Vec<_> = props.agendas.iter()
        .filter(|a| a.source_service.is_none())
        .cloned()
        .collect();
    let synced_agendas: Vec<_> = props.agendas.iter()
        .filter(|a| a.source_service.is_some())
        .cloned()
        .collect();

    rsx! {
        div {
            style: "width: 280px; background: {p.bg_secondary}; border-right: 1px solid {p.border_default}; display: flex; flex-direction: column; overflow-y: auto;",
            
            // Bouton Créer
            div {
                style: "padding: 16px;",
                button {
                    style: "display: flex; align-items: center; gap: 12px; padding: 12px 24px; background: {p.accent_primary}; color: white; border: none; border-radius: 24px; cursor: pointer; font-size: 14px; font-weight: 500; box-shadow: 0 2px 8px rgba(26, 159, 255, 0.3); transition: all 0.2s;",
                    onclick: move |_| props.on_create_event.call(()),
                    span { style: "font-size: 20px;", "+" }
                    "Créer"
                }
            }
            
            // Mini-calendrier
            MiniCalendar {
                displayed_month: *displayed_month.read(),
                selected_date: props.current_date,
                on_date_select: move |date| props.on_date_select.call(date),
                on_month_change: move |date| displayed_month.set(date),
            }
            
            // Séparateur
            div { style: "height: 1px; background: {p.border_default}; margin: 8px 16px;" }
            
            // Mes agendas
            div {
                style: "padding: 8px 16px;",
                h3 {
                    style: "font-size: 11px; font-weight: 600; color: {p.text_muted}; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 8px;",
                    "Mes agendas"
                }
                for agenda in personal_agendas.iter() {
                    AgendaItem {
                        agenda: agenda.clone(),
                        on_toggle: move |id| props.on_agenda_toggle.call(id),
                    }
                }
            }
            
            // Services synchronisés
            div {
                style: "padding: 8px 16px;",
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;",
                    h3 {
                        style: "font-size: 11px; font-weight: 600; color: {p.text_muted}; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Services synchronisés"
                    }
                    button {
                        style: "padding: 4px 8px; background: transparent; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_secondary}; font-size: 10px; cursor: pointer;",
                        onclick: move |_| props.on_sync_jayfestival.call(()),
                        "🔄 Sync"
                    }
                }
                
                if synced_agendas.is_empty() {
                    div {
                        style: "padding: 12px; background: {p.bg_overlay}; border-radius: 4px; text-align: center;",
                        p {
                            style: "font-size: 12px; color: {p.text_muted}; margin-bottom: 8px;",
                            "Aucun service synchronisé"
                        }
                        button {
                            style: "padding: 6px 12px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.accent_primary}; font-size: 11px; cursor: pointer;",
                            onclick: move |_| props.on_sync_jayfestival.call(()),
                            "➕ Synchroniser JayFestival"
                        }
                    }
                } else {
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
                style: "padding: 16px; border-top: 1px solid {p.border_default};",
                p {
                    style: "font-size: 10px; color: {p.text_muted}; text-align: center;",
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
    let state = use_app_state();
    let p = use_palette();
    
    let year = props.displayed_month.year();
    let month = props.displayed_month.month();
    let today = chrono::Local::now().date_naive();
    
    // Premier jour du mois
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap_or(today);
    // Décalage pour commencer le lundi
    let start_offset = i64::from(first_day.weekday().num_days_from_monday());
    // Premier jour affiché (peut être du mois précédent)
    let start_date = first_day - chrono::Duration::days(start_offset);
    
    // Nombre de jours dans le mois
    let days_in_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }.map_or(30, |d| d.pred_opt().unwrap_or(d).day());
    
    // Nombre de semaines à afficher
    let total_days = start_offset as u32 + days_in_month;
    let weeks = total_days.div_ceil(7);
    
    // Noms des jours de la semaine
    let weekdays = ["L", "M", "M", "J", "V", "S", "D"];
    
    // Nom du mois en français
    let month_name = match month {
        1 => "Janvier", 2 => "Février", 3 => "Mars", 4 => "Avril",
        5 => "Mai", 6 => "Juin", 7 => "Juillet", 8 => "Août",
        9 => "Septembre", 10 => "Octobre", 11 => "Novembre", 12 => "Décembre",
        _ => "?",
    };

    rsx! {
        div {
            style: "padding: 8px 16px;",
            
            // Header du mini-calendrier
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;",
                
                button {
                    style: "padding: 4px 8px; background: transparent; border: none; color: {p.text_secondary}; cursor: pointer; font-size: 14px;",
                    onclick: move |_| {
                        let new_date = if month == 1 {
                            NaiveDate::from_ymd_opt(year - 1, 12, 1).unwrap_or(props.displayed_month)
                        } else {
                            NaiveDate::from_ymd_opt(year, month - 1, 1).unwrap_or(props.displayed_month)
                        };
                        props.on_month_change.call(new_date);
                    },
                    "◀"
                }
                
                span {
                    style: "font-size: 13px; font-weight: 500; color: {p.text_primary};",
                    "{month_name} {year}"
                }
                
                button {
                    style: "padding: 4px 8px; background: transparent; border: none; color: {p.text_secondary}; cursor: pointer; font-size: 14px;",
                    onclick: move |_| {
                        let new_date = if month == 12 {
                            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap_or(props.displayed_month)
                        } else {
                            NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap_or(props.displayed_month)
                        };
                        props.on_month_change.call(new_date);
                    },
                    "▶"
                }
            }
            
            // Jours de la semaine
            div {
                style: "display: grid; grid-template-columns: repeat(7, 1fr); gap: 2px; margin-bottom: 4px;",
                for day in weekdays.iter() {
                    span {
                        style: "text-align: center; font-size: 10px; color: {p.text_muted}; padding: 4px 0;",
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
                                p.accent_primary.to_string()
                            } else if is_today {
                                format!("{}40", p.accent_primary)
                            } else {
                                "transparent".to_string()
                            };
                            
                            let color = if is_selected {
                                "white"
                            } else if !is_current_month {
                                p.text_muted
                            } else {
                                p.text_primary
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
    let state = use_app_state();
    let p = use_palette();
    
    let color = props.agenda.color.as_deref().unwrap_or("#4285F4");
    let name = props.agenda.name.as_deref().unwrap_or("Sans nom");
    let id = props.agenda.id.clone().unwrap_or_default();
    let visible = props.agenda.visible;
    
    // Icône de source pour les agendas synchronisés
    let source_icon = match props.agenda.source_service.as_deref() {
        Some("jayfestival") => Some("🎪"),
        Some("jayrdv") => Some("📅"),
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
                                "✓"
                            }
                        }
                    }
                }
            }
            
            // Nom de l'agenda
            span {
                style: "flex: 1; font-size: 13px; color: {p.text_primary}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                "{name}"
            }
            
            // Icône de source
            if let Some(icon) = source_icon {
                span {
                    style: "font-size: 12px;",
                    "{icon}"
                }
            }
        }
    }
}
