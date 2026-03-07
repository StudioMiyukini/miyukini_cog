//! Vue Jour — Grille horaire sur une journée unique.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::state::use_app_state;
use jaykoa::data::{TemporalEntry, TemporalConflict};
use chrono::{Datelike, NaiveDate, Timelike};
use super::components::EventBlock;

/// Props pour la vue jour.
#[derive(Props, Clone, PartialEq)]
pub struct DayViewProps {
    /// Date affichée.
    pub date: NaiveDate,
    /// Entrées à afficher.
    pub entries: Vec<TemporalEntry>,
    /// Conflits détectés.
    pub conflicts: Vec<TemporalConflict>,
    /// Callback quand on clique sur un créneau vide.
    pub on_slot_click: EventHandler<String>,
}

/// Vue jour avec grille horaire détaillée.
#[component]
pub fn DayView(props: DayViewProps) -> Element {
    let state = use_app_state();
    let p = use_palette();
    
    let today = chrono::Local::now().date_naive();
    let now_hour = chrono::Local::now().hour();
    let now_minute = chrono::Local::now().minute();
    let is_today = props.date == today;
    
    // Heures affichées
    let hours: Vec<u32> = (0..24).collect();
    let hour_height = 80; // plus grand pour la vue jour
    
    // Nom du jour
    let weekday_name = match props.date.weekday() {
        chrono::Weekday::Mon => "Lundi",
        chrono::Weekday::Tue => "Mardi",
        chrono::Weekday::Wed => "Mercredi",
        chrono::Weekday::Thu => "Jeudi",
        chrono::Weekday::Fri => "Vendredi",
        chrono::Weekday::Sat => "Samedi",
        chrono::Weekday::Sun => "Dimanche",
    };
    
    // Filtrer les entrées du jour
    let day_str = props.date.format("%Y-%m-%d").to_string();
    let day_entries: Vec<&TemporalEntry> = props.entries.iter()
        .filter(|e| {
            if let Some(start) = &e.start_datetime {
                start.starts_with(&day_str)
            } else {
                false
            }
        })
        .filter(|e| !e.all_day)
        .collect();
    
    let all_day_entries: Vec<&TemporalEntry> = props.entries.iter()
        .filter(|e| e.all_day)
        .filter(|e| {
            if let Some(start) = &e.start_datetime {
                start.starts_with(&day_str)
            } else {
                false
            }
        })
        .collect();
    
    // IDs des entrées en conflit
    let conflict_ids: Vec<String> = props.conflicts.iter()
        .flat_map(|c| [c.entry_a_id.clone(), c.entry_b_id.clone()])
        .flatten()
        .collect();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%; overflow: hidden;",
            
            // Header du jour
            div {
                style: "display: flex; padding: 16px; border-bottom: 1px solid {p.border_default}; background: {p.bg_secondary}; flex-shrink: 0;",
                
                div {
                    style: "width: 60px;",
                }
                
                {
                    let day_color = if is_today { "white".to_string() } else { p.text_primary.to_string() };
                    let day_bg = if is_today { p.accent_primary.to_string() } else { "transparent".to_string() };
                    let day_num = props.date.day();
                    
                    rsx! {
                        div {
                            style: "flex: 1; display: flex; flex-direction: column; align-items: center;",
                            
                            span {
                                style: "font-size: 12px; color: {p.text_secondary}; text-transform: uppercase;",
                                "{weekday_name}"
                            }
                            
                            div {
                                style: "width: 48px; height: 48px; display: flex; align-items: center; justify-content: center; border-radius: 50%; font-size: 24px; font-weight: 500; color: {day_color}; background: {day_bg}; margin-top: 8px;",
                                "{day_num}"
                            }
                        }
                    }
                }
            }
            
            // Zone journée entière
            if !all_day_entries.is_empty() {
                div {
                    style: "display: flex; border-bottom: 1px solid {p.border_default}; background: {p.bg_secondary}; min-height: 40px; padding: 8px; flex-shrink: 0;",
                    
                    div {
                        style: "width: 60px; font-size: 10px; color: {p.text_muted}; padding-right: 8px;",
                        "Journée"
                    }
                    
                    div {
                        style: "flex: 1; display: flex; flex-wrap: wrap; gap: 8px;",
                        for entry in all_day_entries.iter() {
                            AllDayEventLarge { entry: (*entry).clone() }
                        }
                    }
                }
            }
            
            // Grille horaire scrollable
            div {
                style: "flex: 1; overflow-y: auto; display: flex;",
                
                // Colonne des heures
                div {
                    style: "width: 60px; flex-shrink: 0;",
                    for hour in hours.iter() {
                        div {
                            key: "hour-{hour}",
                            style: "height: {hour_height}px; padding-right: 8px; text-align: right; position: relative;",
                            span {
                                style: "font-size: 11px; color: {p.text_muted}; position: absolute; top: -8px; right: 8px;",
                                "{hour:02}:00"
                            }
                        }
                    }
                }
                
                // Colonne principale
                div {
                    style: "flex: 1; position: relative; border-left: 1px solid {p.border_default};",
                    
                    // Lignes horaires
                    for hour in hours.iter() {
                        {
                            let datetime = format!("{}T{:02}:00:00", props.date.format("%Y-%m-%d"), hour);
                            let datetime_clone = datetime.clone();
                            
                            rsx! {
                                div {
                                    key: "slot-{hour}",
                                    style: "height: {hour_height}px; border-bottom: 1px solid {p.border_default}; cursor: pointer; position: relative;",
                                    onclick: move |_| props.on_slot_click.call(datetime_clone.clone()),
                                    
                                    // Demi-heure
                                    div {
                                        style: "height: 50%; border-bottom: 1px dashed {p.border_default}30;",
                                    }
                                    
                                    // Quarts d'heure
                                    div {
                                        style: "height: 50%;",
                                    }
                                }
                            }
                        }
                    }
                    
                    // Ligne de l'heure actuelle
                    if is_today {
                        {
                            let top = (now_hour as f32 + now_minute as f32 / 60.0) * hour_height as f32;
                            rsx! {
                                div {
                                    style: "position: absolute; left: 0; right: 0; top: {top}px; height: 2px; background: {p.error}; z-index: 10;",
                                    div {
                                        style: "width: 12px; height: 12px; background: {p.error}; border-radius: 50%; position: absolute; left: -6px; top: -5px;",
                                    }
                                }
                            }
                        }
                    }
                    
                    // Événements
                    for entry in day_entries.iter() {
                        {
                            let (top_px, height_px) = calculate_event_position(entry, hour_height);
                            let has_conflict = entry.id.as_ref()
                                .is_some_and(|id| conflict_ids.contains(id));
                            
                            rsx! {
                                EventBlock {
                                    entry: (*entry).clone(),
                                    top_px: top_px,
                                    height_px: height_px,
                                    width_percent: 95,
                                    left_percent: 2,
                                    has_conflict: has_conflict,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Événement journée entière (version large).
#[derive(Props, Clone, PartialEq)]
struct AllDayEventLargeProps {
    entry: TemporalEntry,
}

#[component]
fn AllDayEventLarge(props: AllDayEventLargeProps) -> Element {
    let state = use_app_state();
    let _p = use_palette();
    
    let color = props.entry.color.as_deref().unwrap_or("#4285F4");
    let title = props.entry.title.as_deref().unwrap_or("Sans titre");
    let desc = props.entry.description.as_deref();
    
    rsx! {
        div {
            style: "padding: 8px 12px; background: {color}; border-radius: 4px; min-width: 200px;",
            
            div {
                style: "font-size: 13px; font-weight: 500; color: white;",
                "{title}"
            }
            
            if let Some(d) = desc {
                div {
                    style: "font-size: 11px; color: rgba(255,255,255,0.8); margin-top: 4px;",
                    "{d}"
                }
            }
        }
    }
}

/// Calcule la position verticale et la hauteur d'un événement.
fn calculate_event_position(entry: &TemporalEntry, hour_height: u32) -> (u32, u32) {
    let default_top = 8 * hour_height;
    let default_height = hour_height;
    
    if let (Some(start), Some(end)) = (&entry.start_datetime, &entry.end_datetime) {
        if let Ok(start_time) = chrono::NaiveDateTime::parse_from_str(start, "%Y-%m-%dT%H:%M:%S") {
            let start_hour = start_time.hour() as f32 + start_time.minute() as f32 / 60.0;
            let top = (start_hour * hour_height as f32) as u32;
            
            if let Ok(end_time) = chrono::NaiveDateTime::parse_from_str(end, "%Y-%m-%dT%H:%M:%S") {
                let end_hour = end_time.hour() as f32 + end_time.minute() as f32 / 60.0;
                let duration_hours = end_hour - start_hour;
                let height = (duration_hours * hour_height as f32).max(24.0) as u32;
                
                return (top, height);
            }
        }
    }
    
    (default_top, default_height)
}
