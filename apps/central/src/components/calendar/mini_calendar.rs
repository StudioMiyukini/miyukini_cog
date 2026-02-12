//! Mini calendrier / picker de date.
//!
//! @id: central.components.calendar.mini_calendar
//! @do: render_mini_date_picker_calendar
//! @role: ui
//! @layer: ui
//! @human: Petit calendrier pour sélection de date

use dioxus::prelude::*;
use crate::state::use_app_state;
use chrono::{Datelike, NaiveDate};

/// Props pour le mini calendrier.
#[derive(Props, Clone, PartialEq)]
pub struct MiniCalendarProps {
    /// Date actuellement sélectionnée.
    pub selected_date: NaiveDate,
    /// Dates avec des événements (pour les points indicateurs).
    #[props(default)]
    pub dates_with_events: Vec<NaiveDate>,
    /// Callback quand on sélectionne une date.
    pub on_date_select: EventHandler<NaiveDate>,
    /// Callback pour naviguer au mois précédent.
    pub on_prev_month: Option<EventHandler<()>>,
    /// Callback pour naviguer au mois suivant.
    pub on_next_month: Option<EventHandler<()>>,
}

/// Mini calendrier pour sélection de date.
#[component]
pub fn MiniCalendar(props: MiniCalendarProps) -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();
    
    let today = chrono::Local::now().date_naive();
    let year = props.selected_date.year();
    let month = props.selected_date.month();
    
    // Premier jour du mois
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap_or(today);
    // Décalage pour commencer le lundi
    let start_offset = first_day.weekday().num_days_from_monday() as i64;
    // Premier jour affiché
    let start_date = first_day - chrono::Duration::days(start_offset);
    
    // Nombre de jours dans le mois
    let days_in_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }.map(|d| d.pred_opt().unwrap_or(d).day()).unwrap_or(30);
    
    // Nombre de semaines à afficher
    let total_days = start_offset as u32 + days_in_month;
    let weeks = ((total_days + 6) / 7).max(5);
    
    let month_name = match month {
        1 => "Janvier", 2 => "Février", 3 => "Mars", 4 => "Avril",
        5 => "Mai", 6 => "Juin", 7 => "Juillet", 8 => "Août",
        9 => "Septembre", 10 => "Octobre", 11 => "Novembre", 12 => "Décembre",
        _ => "?",
    };

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 12px; width: 280px;",
            
            // Navigation mois
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;",
                
                button {
                    style: "padding: 4px 8px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 14px;",
                    onclick: move |_| {
                        if let Some(handler) = &props.on_prev_month {
                            handler.call(());
                        }
                    },
                    "←"
                }
                
                span {
                    style: "font-size: 14px; color: {c.text_white}; font-weight: 600;",
                    "{month_name} {year}"
                }
                
                button {
                    style: "padding: 4px 8px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 14px;",
                    onclick: move |_| {
                        if let Some(handler) = &props.on_next_month {
                            handler.call(());
                        }
                    },
                    "→"
                }
            }
            
            // En-têtes jours
            div {
                style: "display: grid; grid-template-columns: repeat(7, 1fr); gap: 2px; margin-bottom: 4px;",
                
                for day in ["L", "M", "M", "J", "V", "S", "D"] {
                    div {
                        style: "text-align: center; font-size: 10px; color: {c.text_muted}; padding: 4px;",
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
                            let day_index = (week * 7 + day_of_week) as i64;
                            let date = start_date + chrono::Duration::days(day_index);
                            let is_current_month = date.month() == month;
                            let is_today = date == today;
                            let is_selected = date == props.selected_date;
                            let has_event = props.dates_with_events.contains(&date);
                            
                            let bg = if is_selected {
                                c.accent_blue.to_string()
                            } else if is_today {
                                format!("{}40", c.accent_blue)
                            } else {
                                "transparent".to_string()
                            };
                            
                            let color = if is_selected {
                                "white".to_string()
                            } else if !is_current_month {
                                c.text_muted.to_string()
                            } else {
                                c.text_primary.to_string()
                            };
                            
                            let day_num = date.day();
                            
                            rsx! {
                                div {
                                    key: "{date}",
                                    style: "aspect-ratio: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; background: {bg}; border-radius: 4px; cursor: pointer; position: relative; font-size: 12px; color: {color};",
                                    onclick: move |_| props.on_date_select.call(date),
                                    
                                    "{day_num}"
                                    
                                    if has_event && !is_selected {
                                        div {
                                            style: "width: 4px; height: 4px; background: {c.accent_blue}; border-radius: 50%; position: absolute; bottom: 2px;",
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
}
