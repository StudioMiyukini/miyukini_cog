//! Vue Semaine — Grille 7 jours x 24 heures type Google Agenda.

use dioxus::prelude::*;
use miyukini_service_ui::use_palette;
use jaykoa::data::{TemporalEntry, TemporalConflict};
use chrono::{Datelike, NaiveDate, Timelike};
use super::components::EventBlock;

/// Props pour la vue semaine.
#[derive(Props, Clone, PartialEq)]
pub struct WeekViewProps {
    /// Premier jour de la semaine (lundi).
    pub start_date: NaiveDate,
    /// Entrees a afficher.
    pub entries: Vec<TemporalEntry>,
    /// Conflits detectes.
    pub conflicts: Vec<TemporalConflict>,
    /// Callback quand on clique sur un creneau vide.
    pub on_slot_click: EventHandler<String>,
}

/// Vue semaine avec grille horaire.
#[component]
pub fn WeekView(props: WeekViewProps) -> Element {
    let c = use_palette();

    let today = chrono::Local::now().date_naive();
    let now_hour = chrono::Local::now().hour();
    let now_minute = chrono::Local::now().minute();

    // Generer les 7 jours de la semaine
    let days: Vec<NaiveDate> = (0..7)
        .map(|i| props.start_date + chrono::Duration::days(i))
        .collect();

    // Heures affichees (0h a 23h)
    let hours: Vec<u32> = (0..24).collect();
    let hour_height = 60; // pixels par heure

    // Noms des jours
    let weekday_names = ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"];

    // Grouper les entrees par jour
    let entries_by_day: Vec<Vec<&TemporalEntry>> = days.iter().map(|day| {
        let day_str = day.format("%Y-%m-%d").to_string();
        props.entries.iter()
            .filter(|e| {
                if let Some(start) = &e.start_datetime {
                    start.starts_with(&day_str)
                } else {
                    false
                }
            })
            .collect()
    }).collect();

    // Entrees journee entiere
    let all_day_entries: Vec<&TemporalEntry> = props.entries.iter()
        .filter(|e| e.all_day)
        .collect();

    // IDs des entrees en conflit
    let conflict_ids: Vec<String> = props.conflicts.iter()
        .flat_map(|c| [c.entry_a_id.clone(), c.entry_b_id.clone()])
        .flatten()
        .collect();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%; overflow: hidden;",

            // Header avec jours de la semaine
            div {
                style: "display: flex; border-bottom: 1px solid {c.border}; background: {c.bg_secondary}; flex-shrink: 0;",

                // Colonne des heures (vide en header)
                div {
                    style: "width: 60px; padding: 8px 0; border-right: 1px solid {c.border}; text-align: center;",
                    span {
                        style: "font-size: 10px; color: {c.text_muted};",
                        "GMT+1"
                    }
                }

                // Colonnes des jours
                for (i, day) in days.iter().enumerate() {
                    {
                        let is_today = *day == today;
                        let bg = if is_today { format!("{}20", c.accent_blue) } else { "transparent".to_string() };
                        let day_num_color = if is_today { "white".to_string() } else { c.text_primary.to_string() };
                        let day_num_bg = if is_today { c.accent_blue.to_string() } else { "transparent".to_string() };
                        let day_num = day.day();

                        rsx! {
                            div {
                                key: "{day}",
                                style: "flex: 1; padding: 8px 4px; text-align: center; background: {bg}; border-right: 1px solid {c.border};",

                                // Nom du jour
                                div {
                                    style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase;",
                                    "{weekday_names[i]}"
                                }

                                // Numero du jour
                                div {
                                    style: "width: 32px; height: 32px; margin: 4px auto; display: flex; align-items: center; justify-content: center; border-radius: 50%; font-size: 18px; font-weight: 500; color: {day_num_color}; background: {day_num_bg};",
                                    "{day_num}"
                                }
                            }
                        }
                    }
                }
            }

            // Zone journee entiere
            if !all_day_entries.is_empty() {
                div {
                    style: "display: flex; border-bottom: 1px solid {c.border}; background: {c.bg_secondary}; min-height: 32px; flex-shrink: 0;",

                    div {
                        style: "width: 60px; padding: 4px; border-right: 1px solid {c.border}; font-size: 10px; color: {c.text_muted};",
                        "Journee"
                    }

                    div {
                        style: "flex: 1; display: flex; padding: 4px; gap: 4px; flex-wrap: wrap;",
                        for entry in all_day_entries.iter() {
                            AllDayEvent { entry: (*entry).clone() }
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
                                style: "font-size: 10px; color: {c.text_muted}; position: absolute; top: -6px; right: 8px;",
                                "{hour:02}:00"
                            }
                        }
                    }
                }

                // Grille des jours
                div {
                    style: "flex: 1; display: flex; position: relative;",

                    // Colonnes des jours
                    for (day_idx, day) in days.iter().enumerate() {
                        {
                            let day_entries = &entries_by_day[day_idx];
                            let is_today = *day == today;

                            rsx! {
                                div {
                                    key: "day-col-{day}",
                                    style: "flex: 1; position: relative; border-right: 1px solid {c.border};",

                                    // Lignes horaires
                                    for hour in hours.iter() {
                                        {
                                            let datetime = format!("{}T{:02}:00:00", day.format("%Y-%m-%d"), hour);
                                            let datetime_clone = datetime.clone();
                                            let border_dashed = format!("{}30", c.border);

                                            rsx! {
                                                div {
                                                    key: "slot-{hour}",
                                                    style: "height: {hour_height}px; border-bottom: 1px solid {c.border}; cursor: pointer;",
                                                    onclick: move |_| props.on_slot_click.call(datetime_clone.clone()),

                                                    // Demi-heure
                                                    div {
                                                        style: "height: 50%; border-bottom: 1px dashed {border_dashed};",
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
                                                    style: "position: absolute; left: 0; right: 0; top: {top}px; height: 2px; background: {c.accent_red}; z-index: 10;",
                                                    div {
                                                        style: "width: 10px; height: 10px; background: {c.accent_red}; border-radius: 50%; position: absolute; left: -5px; top: -4px;",
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Evenements du jour (filtres pour exclure all_day)
                                    for entry in day_entries.iter().filter(|e| !e.all_day) {
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
            }
        }
    }
}

/// Evenement journee entiere.
#[derive(Props, Clone, PartialEq)]
struct AllDayEventProps {
    entry: TemporalEntry,
}

#[component]
fn AllDayEvent(props: AllDayEventProps) -> Element {
    let color = props.entry.color.as_deref().unwrap_or("#4285F4");
    let title = props.entry.title.as_deref().unwrap_or("Sans titre");

    rsx! {
        div {
            style: "padding: 2px 8px; background: {color}; border-radius: 4px; font-size: 11px; color: white; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 150px;",
            "{title}"
        }
    }
}

/// Calcule la position verticale et la hauteur d'un evenement.
fn calculate_event_position(entry: &TemporalEntry, hour_height: u32) -> (u32, u32) {
    let default_top = 8 * hour_height; // 8h par defaut
    let default_height = hour_height; // 1h par defaut

    if let (Some(start), Some(end)) = (&entry.start_datetime, &entry.end_datetime) {
        // Parser l'heure de debut
        if let Ok(start_time) = chrono::NaiveDateTime::parse_from_str(start, "%Y-%m-%dT%H:%M:%S") {
            let start_hour = start_time.hour() as f32 + start_time.minute() as f32 / 60.0;
            let top = (start_hour * hour_height as f32) as u32;

            // Parser l'heure de fin
            if let Ok(end_time) = chrono::NaiveDateTime::parse_from_str(end, "%Y-%m-%dT%H:%M:%S") {
                let end_hour = end_time.hour() as f32 + end_time.minute() as f32 / 60.0;
                let duration_hours = end_hour - start_hour;
                let height = (duration_hours * hour_height as f32).max(20.0) as u32;

                return (top, height);
            }
        }
    }

    (default_top, default_height)
}
