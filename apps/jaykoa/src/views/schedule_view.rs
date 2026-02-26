//! Vue Planning — Liste chronologique des evenements.

use dioxus::prelude::*;
use miyukini_service_ui::use_palette;
use jaykoa::data::{TemporalEntry, TemporalConflict, EventSource, EntryType};
use chrono::{Datelike, NaiveDate};

/// Props pour la vue planning.
#[derive(Props, Clone, PartialEq)]
pub struct ScheduleViewProps {
    /// Entrees a afficher.
    pub entries: Vec<TemporalEntry>,
    /// Conflits detectes.
    pub conflicts: Vec<TemporalConflict>,
}

/// Vue planning — Liste chronologique.
#[component]
pub fn ScheduleView(props: ScheduleViewProps) -> Element {
    let c = use_palette();

    let today = chrono::Local::now().date_naive();

    // Grouper les entrees par jour
    let mut entries_by_day: std::collections::BTreeMap<String, Vec<&TemporalEntry>> = std::collections::BTreeMap::new();

    for entry in &props.entries {
        if let Some(start) = &entry.start_datetime {
            let day = &start[..10]; // YYYY-MM-DD
            entries_by_day.entry(day.to_string()).or_default().push(entry);
        }
    }

    // IDs des entrees en conflit
    let conflict_ids: Vec<String> = props.conflicts.iter()
        .flat_map(|c| [c.entry_a_id.clone(), c.entry_b_id.clone()])
        .flatten()
        .collect();

    // Si aucun evenement
    if entries_by_day.is_empty() {
        return rsx! {
            div {
                style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; color: {c.text_muted};",

                span {
                    style: "font-size: 48px; margin-bottom: 16px; opacity: 0.3;",
                    "[cal]"
                }
                h3 {
                    style: "font-size: 18px; color: {c.text_secondary};",
                    "Aucun evenement"
                }
                p {
                    style: "font-size: 13px; margin-top: 8px;",
                    "Creez un evenement ou synchronisez vos services"
                }
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%; overflow-y: auto; padding: 16px;",

            // Alerte conflits
            if !props.conflicts.is_empty() {
                {
                    let conflicts_count = props.conflicts.len();
                    let plural = if conflicts_count > 1 { "s" } else { "" };
                    let alert_bg = format!("{}15", c.accent_orange);
                    let alert_border = format!("{}40", c.accent_orange);

                    rsx! {
                        div {
                            style: "padding: 12px 16px; background: {alert_bg}; border: 1px solid {alert_border}; border-radius: 8px; margin-bottom: 16px;",

                            div {
                                style: "display: flex; align-items: center; gap: 8px;",
                                span { style: "font-size: 18px;", "!" }
                                span {
                                    style: "font-size: 14px; font-weight: 500; color: {c.accent_orange};",
                                    "{conflicts_count} conflit{plural} detecte{plural}"
                                }
                            }

                            for conflict in props.conflicts.iter().take(3) {
                                if let Some(desc) = &conflict.description {
                                    p {
                                        style: "font-size: 12px; color: {c.text_secondary}; margin-top: 8px; padding-left: 26px;",
                                        "{desc}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Liste par jour
            for (day, day_entries) in entries_by_day.iter() {
                {
                    let date = NaiveDate::parse_from_str(day, "%Y-%m-%d").unwrap_or(today);
                    let is_today = date == today;
                    let is_past = date < today;

                    let weekday = match date.weekday() {
                        chrono::Weekday::Mon => "Lundi",
                        chrono::Weekday::Tue => "Mardi",
                        chrono::Weekday::Wed => "Mercredi",
                        chrono::Weekday::Thu => "Jeudi",
                        chrono::Weekday::Fri => "Vendredi",
                        chrono::Weekday::Sat => "Samedi",
                        chrono::Weekday::Sun => "Dimanche",
                    };

                    let month = match date.month() {
                        1 => "janvier", 2 => "fevrier", 3 => "mars", 4 => "avril",
                        5 => "mai", 6 => "juin", 7 => "juillet", 8 => "aout",
                        9 => "septembre", 10 => "octobre", 11 => "novembre", 12 => "decembre",
                        _ => "?",
                    };

                    {
                        let opacity = if is_past { "0.6" } else { "1" };
                        let weekday_color = if is_today { c.accent_blue.to_string() } else { c.text_muted.to_string() };
                        let day_num_color = if is_today { c.accent_blue.to_string() } else { c.text_primary.to_string() };
                        let day_num = date.day();
                        let year = date.year();

                        rsx! {
                            div {
                                key: "{day}",
                                style: "margin-bottom: 24px; opacity: {opacity};",

                                // En-tete du jour
                                div {
                                    style: "display: flex; align-items: center; gap: 12px; margin-bottom: 12px;",

                                    // Date
                                    div {
                                        style: "display: flex; flex-direction: column; align-items: center; min-width: 60px;",
                                        span {
                                            style: "font-size: 11px; color: {weekday_color}; text-transform: uppercase;",
                                            "{weekday}"
                                        }
                                        span {
                                            style: "font-size: 28px; font-weight: 500; color: {day_num_color};",
                                            "{day_num}"
                                        }
                                    }

                                    // Ligne
                                    div {
                                        style: "flex: 1; height: 1px; background: {c.border};",
                                    }

                                    // Mois annee
                                    span {
                                        style: "font-size: 12px; color: {c.text_muted};",
                                        "{month} {year}"
                                    }
                                }

                                // Evenements du jour
                                div {
                                    style: "display: flex; flex-direction: column; gap: 8px; margin-left: 72px;",

                                    for entry in day_entries.iter() {
                                        {
                                            let has_conflict = entry.id.as_ref()
                                                .is_some_and(|id| conflict_ids.contains(id));

                                            rsx! {
                                                ScheduleEventCard {
                                                    entry: (*entry).clone(),
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

/// Carte d'evenement pour la vue planning.
#[derive(Props, Clone, PartialEq)]
struct ScheduleEventCardProps {
    entry: TemporalEntry,
    #[props(default = false)]
    has_conflict: bool,
}

#[component]
fn ScheduleEventCard(props: ScheduleEventCardProps) -> Element {
    let c = use_palette();

    let color = props.entry.color.as_deref().unwrap_or("#4285F4");
    let title = props.entry.title.as_deref().unwrap_or("Sans titre");
    let entry_type = props.entry.entry_type.as_deref().map_or(EntryType::Internal, EntryType::from_str);
    let is_readonly = entry_type.is_readonly();

    // Heure
    let time_label = if props.entry.all_day {
        "Journee entiere".to_string()
    } else if let (Some(start), Some(end)) = (&props.entry.start_datetime, &props.entry.end_datetime) {
        let start_time = &start[11..16];
        let end_time = &end[11..16];
        format!("{start_time} — {end_time}")
    } else {
        String::new()
    };

    // Source
    let source = props.entry.source_service.as_deref().map(EventSource::from_str);
    let source_label = source.map(jaykoa::data::EventSource::display_label);
    let source_icon = match source {
        Some(EventSource::JayFestival) => Some("F"),
        Some(EventSource::JayRDV) => Some("R"),
        _ => None,
    };

    // Bordure de conflit
    let border = if props.has_conflict {
        format!("3px solid {}", c.accent_orange)
    } else {
        format!("3px solid {color}")
    };

    let conflict_badge_bg = format!("{}20", c.accent_orange);

    rsx! {
        div {
            style: "display: flex; gap: 16px; padding: 12px 16px; background: {c.bg_secondary}; border-radius: 8px; border-left: {border}; cursor: pointer; transition: background 0.2s;",

            // Colonne heure
            div {
                style: "min-width: 100px;",
                span {
                    style: "font-size: 13px; font-weight: 500; color: {color};",
                    "{time_label}"
                }
            }

            // Contenu
            div {
                style: "flex: 1;",

                // Titre
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    span {
                        style: "font-size: 14px; font-weight: 500; color: {c.text_white};",
                        "{title}"
                    }
                    if is_readonly {
                        span {
                            style: "font-size: 11px; padding: 2px 6px; background: {c.bg_hover}; border-radius: 4px; color: {c.text_muted};",
                            "[ro] Lecture seule"
                        }
                    }
                    if props.has_conflict {
                        span {
                            style: "font-size: 11px; padding: 2px 6px; background: {conflict_badge_bg}; border-radius: 4px; color: {c.accent_orange};",
                            "! Conflit"
                        }
                    }
                }

                // Description
                if let Some(desc) = &props.entry.description {
                    if !desc.is_empty() {
                        p {
                            style: "font-size: 12px; color: {c.text_secondary}; margin-top: 4px;",
                            "{desc}"
                        }
                    }
                }

                // Metadonnees
                div {
                    style: "display: flex; gap: 16px; margin-top: 8px;",

                    if let Some(loc) = &props.entry.location {
                        if !loc.is_empty() {
                            span {
                                style: "font-size: 11px; color: {c.text_muted}; display: flex; align-items: center; gap: 4px;",
                                "{loc}"
                            }
                        }
                    }

                    if let (Some(icon), Some(label)) = (source_icon, source_label) {
                        span {
                            style: "font-size: 11px; color: {c.text_muted}; display: flex; align-items: center; gap: 4px;",
                            "{icon} {label}"
                        }
                    }
                }
            }
        }
    }
}
