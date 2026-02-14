//! Module JayKoa — Calendrier universel du COG.
//!
//! Structure calquée sur Google Agenda :
//! - Sidebar gauche (agendas, mini-calendrier, services synchronisés)
//! - Vue principale (semaine, jour, mois, planning)
//! - Intégration JayFestival et JayRDV (lecture réfléchie)

mod sidebar;
mod components;
mod calendar_view;
mod week_view;
mod day_view;
mod month_view;
mod schedule_view;
mod event_form;
mod sync_service;

pub use sidebar::JayKoaSidebar;
pub use components::*;
pub use week_view::WeekView;
pub use day_view::DayView;
pub use month_view::MonthView;
pub use schedule_view::ScheduleView;
pub use event_form::EventFormModal;

use dioxus::prelude::*;
use chrono::Datelike;
use crate::data::use_service_connections;
use crate::state::use_app_state;

/// Vue par défaut du calendrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CalendarViewMode {
    Day,
    #[default]
    Week,
    Month,
    #[allow(dead_code)]
    Year,
    Schedule,
}

impl CalendarViewMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Day => "Jour",
            Self::Week => "Semaine",
            Self::Month => "Mois",
            Self::Year => "Année",
            Self::Schedule => "Planning",
        }
    }
}

/// État local de JayKoa.
#[derive(Clone)]
pub struct JayKoaState {
    /// Vue courante (semaine, jour, mois, planning).
    pub view_mode: CalendarViewMode,
    /// Date de référence pour la navigation.
    pub current_date: chrono::NaiveDate,
    /// Agendas visibles (IDs).
    pub visible_agendas: Vec<String>,
    /// Modal de création d'événement ouverte.
    pub show_event_form: bool,
    /// Date/heure pré-remplie pour nouveau événement.
    pub new_event_start: Option<String>,
    /// Synchronisation JayFestival en cours.
    pub syncing_jayfestival: bool,
}

impl Default for JayKoaState {
    fn default() -> Self {
        Self {
            view_mode: CalendarViewMode::Week,
            current_date: chrono::Local::now().date_naive(),
            visible_agendas: Vec::new(),
            show_event_form: false,
            new_event_start: None,
            syncing_jayfestival: false,
        }
    }
}

/// Vue principale JayKoa — Calendrier universel.
#[component]
pub fn JayKoaView() -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();
    let conns = use_service_connections();
    
    // État local JayKoa
    let mut koa_state = use_signal(JayKoaState::default);
    
    // Profil par défaut
    const DEFAULT_PROFILE: &str = "default";
    
    // Charger les agendas depuis la DB
    let agendas = {
        let db = &conns.read().jaykoa;
        // S'assurer qu'un agenda par défaut existe
        let _ = db.ensure_default_agenda(DEFAULT_PROFILE);
        db.agendas_by_profile(DEFAULT_PROFILE).unwrap_or_default()
    };
    
    // Cloner pour éviter les problèmes de borrow
    let agendas_for_effect = agendas.clone();
    let agendas_for_sidebar = agendas.clone();
    let agendas_for_toggle = agendas.clone();
    let agendas_for_form = agendas.clone();
    
    // Mettre à jour les agendas visibles
    use_effect(move || {
        let visible: Vec<String> = agendas_for_effect.iter()
            .filter(|a| a.visible)
            .filter_map(|a| a.id.clone())
            .collect();
        koa_state.write().visible_agendas = visible;
    });
    
    // Calculer la plage de dates selon la vue
    let (start_date, end_date) = {
        let date = koa_state.read().current_date;
        let mode = koa_state.read().view_mode;
        match mode {
            CalendarViewMode::Day => (date, date),
            CalendarViewMode::Week => {
                let weekday = date.weekday().num_days_from_monday();
                let start = date - chrono::Duration::days(weekday as i64);
                let end = start + chrono::Duration::days(6);
                (start, end)
            }
            CalendarViewMode::Month => {
                let start = chrono::NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap_or(date);
                let end = if date.month() == 12 {
                    chrono::NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap_or(date)
                } else {
                    chrono::NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1).unwrap_or(date)
                } - chrono::Duration::days(1);
                (start, end)
            }
            CalendarViewMode::Year | CalendarViewMode::Schedule => {
                let start = date - chrono::Duration::days(30);
                let end = date + chrono::Duration::days(90);
                (start, end)
            }
        }
    };
    
    // Charger les entrées dans la plage
    let entries = {
        let agenda_ids = koa_state.read().visible_agendas.clone();
        if agenda_ids.is_empty() {
            vec![]
        } else {
            conns.read().jaykoa
                .entries_in_range(
                    &agenda_ids,
                    &start_date.format("%Y-%m-%d").to_string(),
                    &end_date.format("%Y-%m-%d").to_string(),
                )
                .unwrap_or_default()
        }
    };
    
    // Détecter les conflits
    let conflicts = {
        let agenda_ids = koa_state.read().visible_agendas.clone();
        if agenda_ids.is_empty() {
            vec![]
        } else {
            conns.read().jaykoa
                .detect_conflicts(
                    &agenda_ids,
                    &start_date.format("%Y-%m-%dT00:00:00").to_string(),
                    &end_date.format("%Y-%m-%dT23:59:59").to_string(),
                )
                .unwrap_or_default()
        }
    };

    rsx! {
        div {
            style: "display: flex; height: 100%; background: {c.bg_main};",
            
            // Sidebar gauche
            JayKoaSidebar {
                agendas: agendas_for_sidebar.clone(),
                current_date: koa_state.read().current_date,
                on_date_select: move |date: chrono::NaiveDate| {
                    koa_state.write().current_date = date;
                },
                on_agenda_toggle: {
                    let agendas_toggle = agendas_for_toggle.clone();
                    move |agenda_id: String| {
                        let conns = conns.read();
                        let db = &conns.jaykoa;
                        // Toggle la visibilité
                        if let Some(agenda) = agendas_toggle.iter().find(|a| a.id.as_deref() == Some(&agenda_id)) {
                            let _ = db.agenda_set_visible(&agenda_id, !agenda.visible);
                        }
                    }
                },
                on_create_event: move |_| {
                    koa_state.write().show_event_form = true;
                },
                on_sync_jayfestival: move |_| {
                    koa_state.write().syncing_jayfestival = true;
                    // Lancer la synchronisation JayFestival
                    let conns = conns.read();
                    let db = &conns.jaykoa;
                    // Trouver ou créer l'agenda JayFestival
                    let agendas = db.agendas_by_profile(DEFAULT_PROFILE).unwrap_or_default();
                    let jf_agenda = agendas.iter().find(|a| a.source_service.as_deref() == Some("jayfestival"));
                    let target_id = if let Some(a) = jf_agenda {
                        a.id.clone().unwrap_or_default()
                    } else {
                        // Créer l'agenda JayFestival
                        let id = uuid::Uuid::new_v4().to_string();
                        let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
                        let agenda = jaykoa::data::Agenda {
                            id: Some(id.clone()),
                            profile_id: Some(DEFAULT_PROFILE.to_string()),
                            name: Some("JayFestival".to_string()),
                            description: Some("Événements synchronisés depuis JayFestival".to_string()),
                            color: Some("#E67C73".to_string()),
                            calendar_type: Some("synced_service".to_string()),
                            visible: true,
                            is_default: false,
                            source_service: Some("jayfestival".to_string()),
                            created_at: Some(now.clone()),
                            updated_at: Some(now),
                        };
                        let _ = db.agenda_insert(&agenda);
                        id
                    };
                    // Synchroniser les éditions
                    // Note: sync_editions attends un Arc, mais nous avons déjà un Arc via conns
                    // Pour l'instant, on crée les entrées mock directement
                    let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
                    let mock_entries = vec![
                        ("fest_ed_2026_printemps", "Festival Printemps 2026", "2026-03-15", "2026-03-17", "Parc des Expositions"),
                        ("fest_ed_2026_ete", "Festival Été 2026", "2026-07-01", "2026-07-03", "Esplanade du Lac"),
                    ];
                    for (sid, title, start, end, loc) in mock_entries {
                        let entry = jaykoa::data::TemporalEntry {
                            id: Some(uuid::Uuid::new_v4().to_string()),
                            agenda_id: Some(target_id.clone()),
                            title: Some(title.to_string()),
                            description: Some(format!("Édition JayFestival : {}", title)),
                            start_datetime: Some(format!("{}T00:00:00", start)),
                            end_datetime: Some(format!("{}T23:59:59", end)),
                            all_day: true,
                            location: Some(loc.to_string()),
                            status: Some("confirmed".to_string()),
                            entry_type: Some("reflect_jayfestival".to_string()),
                            source_service: Some("jayfestival".to_string()),
                            source_event_id: Some(sid.to_string()),
                            color: Some("#E67C73".to_string()),
                            created_at: Some(now.clone()),
                            updated_at: Some(now.clone()),
                            last_synced_at: Some(now.clone()),
                            ..Default::default()
                        };
                        let _ = db.reflect_upsert(&entry);
                    }
                    koa_state.write().syncing_jayfestival = false;
                },
            }
            
            // Zone principale
            div {
                style: "flex: 1; display: flex; flex-direction: column; overflow: hidden;",
                
                // Header avec navigation et contrôles
                CalendarHeader {
                    view_mode: koa_state.read().view_mode,
                    current_date: koa_state.read().current_date,
                    conflicts_count: conflicts.len(),
                    on_view_change: move |mode: CalendarViewMode| {
                        koa_state.write().view_mode = mode;
                    },
                    on_today: move |_| {
                        koa_state.write().current_date = chrono::Local::now().date_naive();
                    },
                    on_prev: move |_| {
                        let mode = koa_state.read().view_mode;
                        let date = koa_state.read().current_date;
                        let new_date = match mode {
                            CalendarViewMode::Day => date - chrono::Duration::days(1),
                            CalendarViewMode::Week => date - chrono::Duration::weeks(1),
                            CalendarViewMode::Month => {
                                if date.month() == 1 {
                                    chrono::NaiveDate::from_ymd_opt(date.year() - 1, 12, date.day().min(31)).unwrap_or(date)
                                } else {
                                    let prev_month = date.month() - 1;
                                    let max_day = chrono::NaiveDate::from_ymd_opt(date.year(), prev_month + 1, 1)
                                        .map(|d| d.pred_opt().unwrap_or(d).day())
                                        .unwrap_or(28);
                                    chrono::NaiveDate::from_ymd_opt(date.year(), prev_month, date.day().min(max_day)).unwrap_or(date)
                                }
                            }
                            CalendarViewMode::Year => chrono::NaiveDate::from_ymd_opt(date.year() - 1, date.month(), date.day()).unwrap_or(date),
                            CalendarViewMode::Schedule => date - chrono::Duration::weeks(2),
                        };
                        koa_state.write().current_date = new_date;
                    },
                    on_next: move |_| {
                        let mode = koa_state.read().view_mode;
                        let date = koa_state.read().current_date;
                        let new_date = match mode {
                            CalendarViewMode::Day => date + chrono::Duration::days(1),
                            CalendarViewMode::Week => date + chrono::Duration::weeks(1),
                            CalendarViewMode::Month => {
                                if date.month() == 12 {
                                    chrono::NaiveDate::from_ymd_opt(date.year() + 1, 1, date.day().min(31)).unwrap_or(date)
                                } else {
                                    let next_month = date.month() + 1;
                                    let max_day = if next_month == 12 {
                                        31
                                    } else {
                                        chrono::NaiveDate::from_ymd_opt(date.year(), next_month + 1, 1)
                                            .map(|d| d.pred_opt().unwrap_or(d).day())
                                            .unwrap_or(28)
                                    };
                                    chrono::NaiveDate::from_ymd_opt(date.year(), next_month, date.day().min(max_day)).unwrap_or(date)
                                }
                            }
                            CalendarViewMode::Year => chrono::NaiveDate::from_ymd_opt(date.year() + 1, date.month(), date.day()).unwrap_or(date),
                            CalendarViewMode::Schedule => date + chrono::Duration::weeks(2),
                        };
                        koa_state.write().current_date = new_date;
                    },
                }
                
                // Zone de contenu principal selon la vue
                div {
                    style: "flex: 1; overflow: hidden;",
                    
                    match koa_state.read().view_mode {
                        CalendarViewMode::Day => rsx! {
                            DayView {
                                date: koa_state.read().current_date,
                                entries: entries.clone(),
                                conflicts: conflicts.clone(),
                                on_slot_click: move |datetime: String| {
                                    koa_state.write().new_event_start = Some(datetime);
                                    koa_state.write().show_event_form = true;
                                },
                            }
                        },
                        CalendarViewMode::Week => rsx! {
                            WeekView {
                                start_date: start_date,
                                entries: entries.clone(),
                                conflicts: conflicts.clone(),
                                on_slot_click: move |datetime: String| {
                                    koa_state.write().new_event_start = Some(datetime);
                                    koa_state.write().show_event_form = true;
                                },
                            }
                        },
                        CalendarViewMode::Month => rsx! {
                            MonthView {
                                year: koa_state.read().current_date.year(),
                                month: koa_state.read().current_date.month(),
                                entries: entries.clone(),
                                on_day_click: move |date: chrono::NaiveDate| {
                                    koa_state.write().current_date = date;
                                    koa_state.write().view_mode = CalendarViewMode::Day;
                                },
                            }
                        },
                        CalendarViewMode::Year => rsx! {
                            div {
                                style: "padding: 24px; color: {c.text_secondary}; text-align: center;",
                                "Vue Année — À implémenter"
                            }
                        },
                        CalendarViewMode::Schedule => rsx! {
                            ScheduleView {
                                entries: entries.clone(),
                                conflicts: conflicts.clone(),
                            }
                        },
                    }
                }
            }
            
            // Modal de création d'événement
            if koa_state.read().show_event_form {
                EventFormModal {
                    initial_datetime: koa_state.read().new_event_start.clone(),
                    agendas: agendas_for_form.clone(),
                    on_close: move |_| {
                        koa_state.write().show_event_form = false;
                        koa_state.write().new_event_start = None;
                    },
                    on_save: move |entry: jaykoa::data::TemporalEntry| {
                        let conns = conns.read();
                        let _ = conns.jaykoa.entry_insert(&entry);
                        koa_state.write().show_event_form = false;
                        koa_state.write().new_event_start = None;
                    },
                }
            }
        }
    }
}
