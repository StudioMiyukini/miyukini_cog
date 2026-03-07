//! Vue Mois — Grille mensuelle compacte type Google Agenda.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::state::use_app_state;
use jaykoa::data::TemporalEntry;
use chrono::{Datelike, NaiveDate};

/// Props pour la vue mois.
#[derive(Props, Clone, PartialEq)]
pub struct MonthViewProps {
    /// Année.
    pub year: i32,
    /// Mois (1-12).
    pub month: u32,
    /// Entrées à afficher.
    pub entries: Vec<TemporalEntry>,
    /// Callback quand on clique sur un jour.
    pub on_day_click: EventHandler<NaiveDate>,
}

/// Vue mois avec grille compacte.
#[component]
pub fn MonthView(props: MonthViewProps) -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();
    
    let today = chrono::Local::now().date_naive();
    
    // Premier jour du mois
    let first_day = NaiveDate::from_ymd_opt(props.year, props.month, 1).unwrap_or(today);
    // Décalage pour commencer le lundi
    let start_offset = i64::from(first_day.weekday().num_days_from_monday());
    // Premier jour affiché
    let start_date = first_day - chrono::Duration::days(start_offset);
    
    // Nombre de jours dans le mois
    let days_in_month = if props.month == 12 {
        NaiveDate::from_ymd_opt(props.year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(props.year, props.month + 1, 1)
    }.map_or(30, |d| d.pred_opt().unwrap_or(d).day());
    
    // Nombre de semaines à afficher (5 ou 6)
    let total_days = start_offset as u32 + days_in_month;
    let weeks = total_days.div_ceil(7).max(5);
    
    // Noms des jours
    let weekday_names = ["Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi", "Dimanche"];

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%; overflow: hidden;",
            
            // Header avec jours de la semaine
            div {
                style: "display: grid; grid-template-columns: repeat(7, 1fr); border-bottom: 1px solid {p.border_default}; background: {p.bg_secondary}; flex-shrink: 0;",
                
                for name in weekday_names.iter() {
                    div {
                        style: "padding: 12px; text-align: center; font-size: 12px; font-weight: 500; color: {p.text_secondary}; text-transform: uppercase;",
                        "{name}"
                    }
                }
            }
            
            // Grille des jours
            div {
                style: "flex: 1; display: grid; grid-template-rows: repeat({weeks}, 1fr);",
                
                for week in 0..weeks {
                    div {
                        key: "week-{week}",
                        style: "display: grid; grid-template-columns: repeat(7, 1fr); border-bottom: 1px solid {p.border_default};",
                        
                        for day_of_week in 0..7u32 {
                            {
                                let day_index = i64::from(week * 7 + day_of_week);
                                let date = start_date + chrono::Duration::days(day_index);
                                let is_current_month = date.month() == props.month;
                                let is_today = date == today;
                                
                                // Entrées du jour
                                let day_str = date.format("%Y-%m-%d").to_string();
                                let day_entries: Vec<&TemporalEntry> = props.entries.iter()
                                    .filter(|e| {
                                        if let Some(start) = &e.start_datetime {
                                            start.starts_with(&day_str)
                                        } else {
                                            false
                                        }
                                    })
                                    .collect();
                                
                                let bg = if is_today {
                                    p.bg_overlay.to_string()
                                } else if !is_current_month {
                                    p.bg_base.to_string()
                                } else {
                                    p.bg_surface.to_string()
                                };
                                
                                {
                                    let day_color = if is_today { "white".to_string() } else if !is_current_month { p.text_muted.to_string() } else { p.text_primary.to_string() };
                                    let day_bg = if is_today { p.accent_primary.to_string() } else { "transparent".to_string() };
                                    let day_num = date.day();
                                    let more_count = day_entries.len().saturating_sub(3);
                                    
                                    rsx! {
                                        div {
                                            key: "{date}",
                                            style: "padding: 4px; border-right: 1px solid {p.border_default}; background: {bg}; min-height: 80px; cursor: pointer; overflow: hidden;",
                                            onclick: move |_| props.on_day_click.call(date),
                                            
                                            // Numéro du jour
                                            div {
                                                style: "display: flex; justify-content: flex-end; margin-bottom: 4px;",
                                                span {
                                                    style: "width: 28px; height: 28px; display: flex; align-items: center; justify-content: center; border-radius: 50%; font-size: 13px; color: {day_color}; background: {day_bg};",
                                                    "{day_num}"
                                                }
                                            }
                                            
                                            // Événements (max 3 affichés)
                                            div {
                                                style: "display: flex; flex-direction: column; gap: 2px;",
                                                for (idx, entry) in day_entries.iter().take(3).enumerate() {
                                                    MonthEventPill { 
                                                        entry: (*entry).clone(),
                                                        key: "{date}-{idx}",
                                                    }
                                                }
                                                
                                                if more_count > 0 {
                                                    span {
                                                        style: "font-size: 10px; color: {p.accent_primary}; padding: 2px 4px;",
                                                        "+{more_count} autres"
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
        }
    }
}

/// Pilule d'événement pour la vue mois.
#[derive(Props, Clone, PartialEq)]
struct MonthEventPillProps {
    entry: TemporalEntry,
}

#[component]
fn MonthEventPill(props: MonthEventPillProps) -> Element {
    let color = props.entry.color.as_deref().unwrap_or("#4285F4");
    let title = props.entry.title.as_deref().unwrap_or("Sans titre");
    
    // Heure de début
    let time = props.entry.start_datetime.as_ref()
        .and_then(|s| s.get(11..16))
        .unwrap_or("");
    
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 4px; padding: 2px 4px; background: {color}20; border-left: 3px solid {color}; border-radius: 2px; overflow: hidden;",
            
            if !time.is_empty() && !props.entry.all_day {
                span {
                    style: "font-size: 10px; color: {color}; font-weight: 500;",
                    "{time}"
                }
            }
            
            span {
                style: "font-size: 11px; color: {color}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                "{title}"
            }
        }
    }
}
