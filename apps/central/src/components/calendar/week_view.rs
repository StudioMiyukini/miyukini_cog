//! Vue Semaine generique — Grille 7 jours × 24 heures.
//!
//! @id: central.components.calendar.week_view
//! @do: render_generic_week_calendar_view
//! @role: ui
//! @layer: ui
//! @human: Vue calendrier semaine generique reutilisable

use dioxus::prelude::*;
use crate::state::use_app_state;
use chrono::{Datelike, NaiveDate, Timelike};
use super::types::{CalendarEvent, EventConflict, TimeSlot, parse_datetime, WEEKDAY_NAMES_SHORT};

/// Props pour la vue semaine generique.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarWeekViewProps<E: CalendarEvent + 'static> {
    /// Premier jour de la semaine (lundi).
    pub start_date: NaiveDate,
    /// Evenements a afficher.
    pub events: Vec<E>,
    /// Conflits detectes (optionnel).
    #[props(default)]
    pub conflicts: Vec<EventConflict>,
    /// Callback quand on clique sur un creneau vide.
    #[props(default)]
    pub on_slot_click: Option<EventHandler<TimeSlot>>,
    /// Callback quand on clique sur un evenement.
    #[props(default)]
    pub on_event_click: Option<EventHandler<String>>,
    /// Hauteur d'une heure en pixels (defaut: 60).
    #[props(default = 60)]
    pub hour_height: u32,
    /// Heure de debut affichee (defaut: 0).
    #[props(default = 0)]
    pub start_hour: u32,
    /// Heure de fin affichee (defaut: 24).
    #[props(default = 24)]
    pub end_hour: u32,
}

/// Vue semaine avec grille horaire.
#[component]
pub fn CalendarWeekView<E: CalendarEvent + 'static>(props: CalendarWeekViewProps<E>) -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();

    let today = chrono::Local::now().date_naive();
    let now_hour = chrono::Local::now().hour();
    let now_minute = chrono::Local::now().minute();

    // Generer les 7 jours de la semaine
    let days: Vec<NaiveDate> = (0..7)
        .map(|i| props.start_date + chrono::Duration::days(i))
        .collect();

    // Heures affichees
    let hours: Vec<u32> = (props.start_hour..props.end_hour).collect();
    let hour_height = props.hour_height;

    // Grouper les evenements par jour
    let events_by_day: Vec<Vec<&E>> = days.iter().map(|day| {
        let day_str = day.format("%Y-%m-%d").to_string();
        props.events.iter()
            .filter(|e| {
                if let Some(start) = e.start_datetime() {
                    start.starts_with(&day_str)
                } else {
                    false
                }
            })
            .collect()
    }).collect();

    // Evenements journee entiere
    let all_day_events: Vec<&E> = props.events.iter()
        .filter(|e| e.is_all_day())
        .collect();

    // IDs des evenements en conflit
    let conflict_ids: Vec<String> = props.conflicts.iter()
        .flat_map(|c| [c.event_a_id.clone(), c.event_b_id.clone()])
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
                                    "{WEEKDAY_NAMES_SHORT[i]}"
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
            if !all_day_events.is_empty() {
                div {
                    style: "display: flex; border-bottom: 1px solid {c.border}; background: {c.bg_secondary}; min-height: 32px; flex-shrink: 0;",

                    div {
                        style: "width: 60px; padding: 4px; border-right: 1px solid {c.border}; font-size: 10px; color: {c.text_muted};",
                        "Journée"
                    }

                    div {
                        style: "flex: 1; display: flex; padding: 4px; gap: 4px; flex-wrap: wrap;",
                        for event in all_day_events.iter() {
                            AllDayEvent { event: (*event).clone() }
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
                            let day_events = &events_by_day[day_idx];
                            let is_today = *day == today;

                            rsx! {
                                div {
                                    key: "day-col-{day}",
                                    style: "flex: 1; position: relative; border-right: 1px solid {c.border};",

                                    // Lignes horaires
                                    for hour in hours.iter() {
                                        {
                                            let slot = TimeSlot::new(*day, *hour);
                                            let cursor = if props.on_slot_click.is_some() { "pointer" } else { "default" };

                                            rsx! {
                                                div {
                                                    key: "slot-{hour}",
                                                    style: "height: {hour_height}px; border-bottom: 1px solid {c.border}; cursor: {cursor};",
                                                    onclick: move |_| {
                                                        if let Some(ref handler) = props.on_slot_click {
                                                            handler.call(slot.clone());
                                                        }
                                                    },

                                                    // Demi-heure
                                                    div {
                                                        style: "height: 50%; border-bottom: 1px dashed {c.border}30;",
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Ligne de l'heure actuelle
                                    if is_today && now_hour >= props.start_hour && now_hour < props.end_hour {
                                        {
                                            let top = ((now_hour - props.start_hour) as f32 + now_minute as f32 / 60.0) * hour_height as f32;
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
                                    for event in day_events.iter().filter(|e| !e.is_all_day()) {
                                        {
                                            let (top_px, height_px) = calculate_event_position(*event, hour_height, props.start_hour);
                                            let has_conflict = event.id()
                                                .map(|id| conflict_ids.contains(&id.to_string()))
                                                .unwrap_or(false);
                                            let event_id = event.id().map(|s| s.to_string());

                                            rsx! {
                                                EventBlock {
                                                    event: (*event).clone(),
                                                    top_px: top_px,
                                                    height_px: height_px,
                                                    width_percent: 95,
                                                    left_percent: 2,
                                                    has_conflict: has_conflict,
                                                    on_click: move |_| {
                                                        if let (Some(ref handler), Some(ref id)) = (&props.on_event_click, &event_id) {
                                                            handler.call(id.clone());
                                                        }
                                                    },
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
struct AllDayEventProps<E: CalendarEvent + 'static> {
    event: E,
}

#[component]
fn AllDayEvent<E: CalendarEvent + 'static>(props: AllDayEventProps<E>) -> Element {
    let color = props.event.color();
    let title = props.event.title();

    rsx! {
        div {
            style: "padding: 2px 8px; background: {color}; border-radius: 4px; font-size: 11px; color: white; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 150px;",
            "{title}"
        }
    }
}

/// Block d'evenement positionne.
#[derive(Props, Clone, PartialEq)]
struct EventBlockProps<E: CalendarEvent + 'static> {
    event: E,
    top_px: u32,
    height_px: u32,
    width_percent: u32,
    left_percent: u32,
    has_conflict: bool,
    on_click: EventHandler<MouseEvent>,
}

#[component]
fn EventBlock<E: CalendarEvent + 'static>(props: EventBlockProps<E>) -> Element {
    let color = props.event.color();
    let title = props.event.title();

    let border = if props.has_conflict {
        "2px solid #FF0000"
    } else {
        "none"
    };

    rsx! {
        div {
            style: "position: absolute; top: {props.top_px}px; left: {props.left_percent}%; width: {props.width_percent}%; height: {props.height_px}px; background: {color}; border-radius: 4px; padding: 2px 4px; overflow: hidden; cursor: pointer; border: {border}; box-sizing: border-box;",
            onclick: move |evt| props.on_click.call(evt),

            div {
                style: "font-size: 11px; font-weight: 500; color: white; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                "{title}"
            }
        }
    }
}

/// Calcule la position verticale et la hauteur d'un evenement.
fn calculate_event_position<E: CalendarEvent>(event: &E, hour_height: u32, start_hour: u32) -> (u32, u32) {
    let default_top = 8 * hour_height;
    let default_height = hour_height;

    if let (Some(start), Some(end)) = (event.start_datetime(), event.end_datetime()) {
        if let Some(start_time) = parse_datetime(start) {
            let start_h = start_time.hour().saturating_sub(start_hour);
            let start_minutes = start_time.minute();
            let top = (start_h as f32 + start_minutes as f32 / 60.0) * hour_height as f32;

            if let Some(end_time) = parse_datetime(end) {
                let end_h = end_time.hour().saturating_sub(start_hour);
                let end_minutes = end_time.minute();
                let end_pos = (end_h as f32 + end_minutes as f32 / 60.0) * hour_height as f32;
                let height = (end_pos - top).max(20.0);

                return (top as u32, height as u32);
            }
        }
    }

    (default_top, default_height)
}
