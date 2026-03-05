//! Module JayKoa — Calendrier universel du COG.
//!
//! Structure calquee sur Google Agenda :
//! - Sidebar gauche (agendas, mini-calendrier, services synchronises)
//! - Vue principale (semaine, jour, mois, planning)
//! - Integration JayFestival et JayRDV (lecture reflechie)

mod calendar_view;
mod components;
mod day_view;
mod event_form;
mod month_view;
mod schedule_view;
mod sidebar;
pub mod sync_service;
mod week_view;

pub use components::*;
pub use day_view::DayView;
pub use event_form::EventFormModal;
pub use month_view::MonthView;
pub use schedule_view::ScheduleView;
pub use sidebar::JayKoaSidebar;
pub use week_view::WeekView;

use chrono::Datelike;
use dioxus::prelude::*;
use miyukini_service_ui::use_palette;

/// Vue par defaut du calendrier.
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
            Self::Year => "Annee",
            Self::Schedule => "Planning",
        }
    }
}

/// Etat local de JayKoa.
#[derive(Clone)]
pub struct JayKoaState {
    /// Vue courante (semaine, jour, mois, planning).
    pub view_mode: CalendarViewMode,
    /// Date de reference pour la navigation.
    pub current_date: chrono::NaiveDate,
    /// Agendas visibles (IDs).
    pub visible_agendas: Vec<String>,
    /// Modal de creation d'evenement ouverte.
    pub show_event_form: bool,
    /// Date/heure pre-remplie pour nouveau evenement.
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
    let c = use_palette();
    let db = crate::use_db();

    // Etat local JayKoa
    let mut koa_state = use_signal(JayKoaState::default);

    // Profil par defaut
    const DEFAULT_PROFILE: &str = "default";

    // Charger les agendas depuis la DB
    let agendas = {
        // S'assurer qu'un agenda par defaut existe
        let _ = db.ensure_default_agenda(DEFAULT_PROFILE);
        db.agendas_by_profile(DEFAULT_PROFILE).unwrap_or_default()
    };

    // Cloner pour eviter les problemes de borrow
    let agendas_for_effect = agendas.clone();
    let agendas_for_sidebar = agendas.clone();
    let agendas_for_toggle = agendas.clone();
    let agendas_for_form = agendas.clone();

    // Mettre a jour les agendas visibles
    use_effect(move || {
        let visible: Vec<String> = agendas_for_effect
            .iter()
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
                let start = date - chrono::Duration::days(i64::from(weekday));
                let end = start + chrono::Duration::days(6);
                (start, end)
            }
            CalendarViewMode::Month => {
                let start =
                    chrono::NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap_or(date);
                let end = if date.month() == 12 {
                    chrono::NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap_or(date)
                } else {
                    chrono::NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1)
                        .unwrap_or(date)
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

    // Charger les entrees dans la plage
    let entries = {
        let agenda_ids = koa_state.read().visible_agendas.clone();
        if agenda_ids.is_empty() {
            vec![]
        } else {
            db.entries_in_range(
                &agenda_ids,
                &start_date.format("%Y-%m-%d").to_string(),
                &end_date.format("%Y-%m-%d").to_string(),
            )
            .unwrap_or_default()
        }
    };

    // Detecter les conflits
    let conflicts = {
        let agenda_ids = koa_state.read().visible_agendas.clone();
        if agenda_ids.is_empty() {
            vec![]
        } else {
            db.detect_conflicts(
                &agenda_ids,
                &start_date.format("%Y-%m-%dT00:00:00").to_string(),
                &end_date.format("%Y-%m-%dT23:59:59").to_string(),
            )
            .unwrap_or_default()
        }
    };

    // Clone DB for closures
    let db_toggle = db.clone();
    let db_sync = db.clone();
    let db_save = db.clone();

    let c_year = c.text_secondary.to_string();

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
                        // Toggle la visibilite
                        if let Some(agenda) = agendas_toggle.iter().find(|a| a.id.as_deref() == Some(&agenda_id)) {
                            let _ = db_toggle.agenda_set_visible(&agenda_id, !agenda.visible);
                        }
                    }
                },
                on_create_event: move |()| {
                    koa_state.write().show_event_form = true;
                },
                on_sync_jayfestival: move |()| {
                    koa_state.write().syncing_jayfestival = true;
                    // Standalone: sync JayFestival non disponible (necessite JayFestival DB).
                    // TODO: implementer quand le service JayFestival est accessible en standalone.
                    tracing::warn!("Sync JayFestival non disponible en mode standalone");
                    koa_state.write().syncing_jayfestival = false;
                },
            }

            // Zone principale
            div {
                style: "flex: 1; display: flex; flex-direction: column; overflow: hidden;",

                // Header avec navigation et controles
                CalendarHeader {
                    view_mode: koa_state.read().view_mode,
                    current_date: koa_state.read().current_date,
                    conflicts_count: conflicts.len(),
                    on_view_change: move |mode: CalendarViewMode| {
                        koa_state.write().view_mode = mode;
                    },
                    on_today: move |()| {
                        koa_state.write().current_date = chrono::Local::now().date_naive();
                    },
                    on_prev: move |()| {
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
                                        .map_or(28, |d| d.pred_opt().unwrap_or(d).day());
                                    chrono::NaiveDate::from_ymd_opt(date.year(), prev_month, date.day().min(max_day)).unwrap_or(date)
                                }
                            }
                            CalendarViewMode::Year => chrono::NaiveDate::from_ymd_opt(date.year() - 1, date.month(), date.day()).unwrap_or(date),
                            CalendarViewMode::Schedule => date - chrono::Duration::weeks(2),
                        };
                        koa_state.write().current_date = new_date;
                    },
                    on_next: move |()| {
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
                                            .map_or(28, |d| d.pred_opt().unwrap_or(d).day())
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
                                style: "padding: 24px; color: {c_year}; text-align: center;",
                                "Vue Annee — A implementer"
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

            // Modal de creation d'evenement
            if koa_state.read().show_event_form {
                EventFormModal {
                    initial_datetime: koa_state.read().new_event_start.clone(),
                    agendas: agendas_for_form.clone(),
                    on_close: move |()| {
                        koa_state.write().show_event_form = false;
                        koa_state.write().new_event_start = None;
                    },
                    on_save: move |entry: jaykoa::data::TemporalEntry| {
                        let _ = db_save.entry_insert(&entry);
                        koa_state.write().show_event_form = false;
                        koa_state.write().new_event_start = None;
                    },
                }
            }
        }
    }
}
