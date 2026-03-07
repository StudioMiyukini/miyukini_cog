//! Composants UI réutilisables pour JayKoa.
//!
//! - CalendarHeader : barre d'en-tête avec navigation et sélecteur de vue
//! - EventBlock : bloc d'événement dans la grille
//! - ConflictBadge : indicateur de conflit

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::state::use_app_state;
use super::CalendarViewMode;
use jaykoa::data::{TemporalEntry, TemporalConflict, EventSource, EntryType};
use chrono::{Datelike, NaiveDate};

/// Props pour le header du calendrier.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarHeaderProps {
    pub view_mode: CalendarViewMode,
    pub current_date: NaiveDate,
    pub conflicts_count: usize,
    pub on_view_change: EventHandler<CalendarViewMode>,
    pub on_today: EventHandler<()>,
    pub on_prev: EventHandler<()>,
    pub on_next: EventHandler<()>,
}

/// Barre d'en-tête du calendrier avec navigation et contrôles.
#[component]
pub fn CalendarHeader(props: CalendarHeaderProps) -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();
    
    // Formater le libellé de période selon la vue
    let period_label = match props.view_mode {
        CalendarViewMode::Day => {
            let weekday = match props.current_date.weekday() {
                chrono::Weekday::Mon => "Lundi",
                chrono::Weekday::Tue => "Mardi",
                chrono::Weekday::Wed => "Mercredi",
                chrono::Weekday::Thu => "Jeudi",
                chrono::Weekday::Fri => "Vendredi",
                chrono::Weekday::Sat => "Samedi",
                chrono::Weekday::Sun => "Dimanche",
            };
            format!("{} {} {}", weekday, props.current_date.day(), month_name(props.current_date.month()))
        }
        CalendarViewMode::Week => {
            let week_num = props.current_date.iso_week().week();
            format!("{} {} — Semaine {}", month_name(props.current_date.month()), props.current_date.year(), week_num)
        }
        CalendarViewMode::Month | CalendarViewMode::Schedule => {
            format!("{} {}", month_name(props.current_date.month()), props.current_date.year())
        }
        CalendarViewMode::Year => {
            format!("{}", props.current_date.year())
        }
    };
    
    let today = chrono::Local::now().date_naive();
    let is_today_visible = props.current_date == today;

    rsx! {
        div {
            style: "display: flex; align-items: center; justify-content: space-between; padding: 12px 16px; background: {p.bg_secondary}; border-bottom: 1px solid {p.border_default};",
            
            // Partie gauche : navigation temporelle
            div {
                style: "display: flex; align-items: center; gap: 12px;",
                
                // Bouton Aujourd'hui
                {
                    let today_color = if is_today_visible { p.text_muted.to_string() } else { p.text_primary.to_string() };
                    rsx! {
                        button {
                            style: "padding: 8px 16px; background: {p.bg_overlay}; border: 1px solid {p.border_default}; border-radius: 4px; color: {today_color}; font-size: 13px; cursor: pointer; transition: all 0.2s;",
                            disabled: is_today_visible,
                            onclick: move |_| props.on_today.call(()),
                            "Aujourd'hui"
                        }
                    }
                }
                
                // Flèches de navigation
                div {
                    style: "display: flex; gap: 4px;",
                    button {
                        style: "width: 32px; height: 32px; display: flex; align-items: center; justify-content: center; background: transparent; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_primary}; cursor: pointer;",
                        onclick: move |_| props.on_prev.call(()),
                        "◀"
                    }
                    button {
                        style: "width: 32px; height: 32px; display: flex; align-items: center; justify-content: center; background: transparent; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_primary}; cursor: pointer;",
                        onclick: move |_| props.on_next.call(()),
                        "▶"
                    }
                }
                
                // Libellé de période
                h2 {
                    style: "font-size: 18px; font-weight: 500; color: {p.text_high}; margin: 0;",
                    "{period_label}"
                }
                
                // Badge de conflits
                if props.conflicts_count > 0 {
                    {
                        let plural = if props.conflicts_count > 1 { "s" } else { "" };
                        let badge_bg = format!("{}20", p.warning);
                        rsx! {
                            span {
                                style: "padding: 4px 8px; background: {badge_bg}; color: {p.warning}; border-radius: 4px; font-size: 11px; font-weight: 500;",
                                "⚠️ {props.conflicts_count} conflit{plural}"
                            }
                        }
                    }
                }
            }
            
            // Partie droite : sélecteur de vue
            div {
                style: "display: flex; gap: 4px; background: {p.bg_overlay}; padding: 4px; border-radius: 6px;",
                
                for mode in [CalendarViewMode::Day, CalendarViewMode::Week, CalendarViewMode::Month, CalendarViewMode::Schedule] {
                    {
                        let is_active = props.view_mode == mode;
                        let bg = if is_active { p.accent_primary.to_string() } else { "transparent".to_string() };
                        let color = if is_active { "white".to_string() } else { p.text_secondary.to_string() };
                        
                        rsx! {
                            button {
                                key: "{mode:?}",
                                style: "padding: 6px 12px; background: {bg}; border: none; border-radius: 4px; color: {color}; font-size: 12px; cursor: pointer; transition: all 0.2s;",
                                onclick: move |_| props.on_view_change.call(mode),
                                "{mode.label()}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Props pour un bloc d'événement.
#[derive(Props, Clone, PartialEq)]
pub struct EventBlockProps {
    pub entry: TemporalEntry,
    /// Hauteur du bloc en pixels (calculée selon la durée).
    #[props(default = 60)]
    pub height_px: u32,
    /// Décalage vertical en pixels.
    #[props(default = 0)]
    pub top_px: u32,
    /// Largeur en pourcentage (pour les événements qui se chevauchent).
    #[props(default = 100)]
    pub width_percent: u32,
    /// Décalage horizontal en pourcentage.
    #[props(default = 0)]
    pub left_percent: u32,
    /// L'événement est en conflit.
    #[props(default = false)]
    pub has_conflict: bool,
    /// Callback au clic.
    pub on_click: Option<EventHandler<String>>,
}

/// Bloc d'événement dans la grille du calendrier.
#[component]
pub fn EventBlock(props: EventBlockProps) -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();
    
    let title = props.entry.title.as_deref().unwrap_or("Sans titre");
    let color = props.entry.color.as_deref().unwrap_or("#4285F4");
    let entry_type = props.entry.entry_type.as_deref().map_or(EntryType::Internal, EntryType::from_str);
    let is_readonly = entry_type.is_readonly();
    
    // Formater l'heure
    let time_label = if let (Some(start), Some(end)) = (&props.entry.start_datetime, &props.entry.end_datetime) {
        let start_time = &start[11..16];
        let end_time = &end[11..16];
        format!("{start_time} — {end_time}")
    } else {
        String::new()
    };
    
    // Icône de source
    let source_icon = match props.entry.source_service.as_deref().map(EventSource::from_str) {
        Some(EventSource::JayFestival) => Some("🎪"),
        Some(EventSource::JayRDV) => Some("📅"),
        _ => None,
    };
    
    // Style de bordure pour les conflits
    let border = if props.has_conflict {
        format!("3px solid {}", p.warning)
    } else {
        "none".to_string()
    };
    
    let id = props.entry.id.clone().unwrap_or_default();

    rsx! {
        div {
            style: "position: absolute; top: {props.top_px}px; left: {props.left_percent}%; width: calc({props.width_percent}% - 4px); height: {props.height_px}px; background: {color}; border-radius: 4px; padding: 4px 8px; overflow: hidden; cursor: pointer; transition: transform 0.1s, box-shadow 0.2s; border-left: {border};",
            onclick: move |_| {
                if let Some(handler) = &props.on_click {
                    handler.call(id.clone());
                }
            },
            
            // Titre
            div {
                style: "display: flex; align-items: center; gap: 4px;",
                span {
                    style: "font-size: 12px; font-weight: 500; color: white; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                    "{title}"
                }
                if let Some(icon) = source_icon {
                    span {
                        style: "font-size: 10px;",
                        "{icon}"
                    }
                }
                if is_readonly {
                    span {
                        style: "font-size: 10px; opacity: 0.7;",
                        "🔒"
                    }
                }
            }
            
            // Heure
            if !time_label.is_empty() && props.height_px > 40 {
                span {
                    style: "font-size: 10px; color: rgba(255,255,255,0.8);",
                    "{time_label}"
                }
            }
            
            // Lieu
            if let Some(loc) = &props.entry.location {
                if props.height_px > 60 && !loc.is_empty() {
                    div {
                        style: "font-size: 10px; color: rgba(255,255,255,0.7); margin-top: 2px;",
                        "📍 {loc}"
                    }
                }
            }
        }
    }
}

/// Props pour un badge de conflit.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictBadgeProps {
    pub conflict: TemporalConflict,
}

/// Badge indiquant un conflit temporel.
#[component]
pub fn ConflictBadge(props: ConflictBadgeProps) -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();
    
    let desc = props.conflict.description.as_deref().unwrap_or("Conflit détecté");

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: {p.warning}15; border: 1px solid {p.warning}40; border-radius: 6px;",
            
            span {
                style: "font-size: 16px;",
                "⚠️"
            }
            
            span {
                style: "font-size: 12px; color: {p.warning};",
                "{desc}"
            }
        }
    }
}

/// Composant vue calendrier principale (wrapper).
#[derive(Props, Clone, PartialEq)]
pub struct CalendarMainViewProps {
    pub children: Element,
}

#[component]
pub fn CalendarMainView(props: CalendarMainViewProps) -> Element {
    rsx! {
        div {
            style: "flex: 1; display: flex; flex-direction: column; overflow: hidden;",
            {props.children}
        }
    }
}

// Helper : nom du mois en français
fn month_name(month: u32) -> &'static str {
    match month {
        1 => "Janvier", 2 => "Février", 3 => "Mars", 4 => "Avril",
        5 => "Mai", 6 => "Juin", 7 => "Juillet", 8 => "Août",
        9 => "Septembre", 10 => "Octobre", 11 => "Novembre", 12 => "Décembre",
        _ => "?",
    }
}
