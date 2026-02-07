//! Application principale JayKoa — boucle eframe.
//!
//! @id: jaykoa_app
//! @do: orchestrate_jaykoa_ui_and_state
//! @layer: app
//! @human: Boucle principale : thème, router, écrans, popover, synchronisation.
//! INVARIANTS : L'UI est un reflet. Aucune décision métier. Toute écriture via KindMother.

use crate::app_state::{AppState, ViewId};
use crate::data::types::{Agenda, TemporalEntry, UserSettings};
use crate::data::JayKoaDb;
use crate::screens::calendar::{self, CalendarData};
use crate::screens::event_create::{self, EventFormState};
use crate::screens::event_detail;
use crate::screens::export as export_screen;
use crate::screens::settings;
use crate::services::jayfestival::JayFestivalAdapter;
use crate::services::jayrdv::JayRDVAdapter;
use crate::theme::JayKoaTheme;
use crate::ui::organisms;
use chrono::Datelike;
use eframe::egui;
use eframe::App;
use std::collections::HashMap;
use std::sync::Arc;

/// Application JayKoa — calendrier universel du COG.
pub struct JayKoaApp {
    db: Arc<JayKoaDb>,
    theme: JayKoaTheme,
    state: AppState,
    agendas: Vec<Agenda>,
    user_settings: UserSettings,
    cal_data: CalendarData,
    form_state: Option<EventFormState>,
    detail_entry: Option<TemporalEntry>,
    agendas_colors: HashMap<String, String>,
    initial_sync_done: bool,
    needs_reload: bool,
}

impl JayKoaApp {
    /// Crée l'application (standalone avec CreationContext).
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self { Self::default() }
    /// Crée l'application pour mode embarqué (Central).
    pub fn new_embedded() -> Self { Self::default() }

    /// Crée l'application pour mode embarqué (Central), avec un profile_id optionnel.
    pub fn new_embedded_with_profile(profile_id: Option<String>) -> Self {
        let mut app = Self::default();
        if let Some(pid) = profile_id {
            app.state.profile_id = pid.clone();
            // Recharger les données pour le nouveau profil
            let _ = app.db.ensure_default_agenda(&pid);
            app.agendas = app.db.agendas_by_profile(&pid).unwrap_or_default();
            app.agendas_colors = app.agendas.iter()
                .filter_map(|a| Some((a.id.clone()?, a.color.clone().unwrap_or_else(|| "#4285F4".to_string())))).collect();
            app.user_settings = app.db.settings_get(&pid).unwrap_or_default();
        }
        app
    }
}

impl Default for JayKoaApp {
    fn default() -> Self {
        let db = Arc::new(JayKoaDb::open("jaykoa.db").unwrap_or_else(|e| {
            panic!("JayKoa: impossible d'ouvrir la base SQLite (KindMother fille): {}", e)
        }));
        let theme = JayKoaTheme::new(true);
        let state = AppState::new();
        let pid = state.profile_id.clone();
        let user_settings = db.settings_get(&pid).unwrap_or_default();
        let _ = db.ensure_default_agenda(&pid);
        let agendas = db.agendas_by_profile(&pid).unwrap_or_default();
        let agendas_colors: HashMap<String, String> = agendas.iter()
            .filter_map(|a| Some((a.id.clone()?, a.color.clone().unwrap_or_else(|| "#4285F4".to_string())))).collect();
        Self { db, theme, state, agendas, user_settings,
            cal_data: CalendarData { entries: Vec::new(), conflict_pairs: Vec::new() },
            form_state: None, detail_entry: None, agendas_colors, initial_sync_done: false, needs_reload: true }
    }
}

impl App for JayKoaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { self.show_in_ui(ctx); }
}

impl JayKoaApp {
    /// Affiche l'UI JayKoa dans le contexte egui (embarqué ou standalone).
    pub fn show_in_ui(&mut self, ctx: &egui::Context) {
        // Sync initiale (stub MVP)
        if !self.initial_sync_done { self.run_initial_sync(); self.initial_sync_done = true; self.needs_reload = true; }
        // Rechargement données
        if self.needs_reload { self.reload_data(); self.needs_reload = false; }

        // --- Popovers ---
        if self.state.show_quick_add {
            if self.form_state.is_none() {
                let aid = self.agendas.iter().find(|a| a.is_default).or(self.agendas.first())
                    .and_then(|a| a.id.as_deref()).unwrap_or("default");
                self.form_state = Some(EventFormState::new_empty(aid));
            }
            if let Some(form) = &mut self.form_state {
                egui::CentralPanel::default().show(ctx, |ui| { event_create::show_quick_add(ui, &mut self.state, form, &self.db, &self.theme); });
                if form.saved || !self.state.show_quick_add {
                    if form.saved { self.needs_reload = true; }
                    if !self.state.show_full_editor { self.form_state = None; }
                }
            }
        }

        if self.state.show_full_editor {
            if self.form_state.is_none() {
                if let Some(eid) = &self.state.editing_entry_id {
                    if let Ok(Some(entry)) = self.db.entry_by_id(eid) { self.form_state = Some(EventFormState::from_entry(&entry)); }
                } else {
                    let aid = self.agendas.iter().find(|a| a.is_default).or(self.agendas.first())
                        .and_then(|a| a.id.as_deref()).unwrap_or("default");
                    self.form_state = Some(EventFormState::new_empty(aid));
                }
            }
            let al: Vec<(String, String)> = self.agendas.iter()
                .filter_map(|a| Some((a.id.clone()?, a.name.clone().unwrap_or_else(|| "Sans nom".to_string())))).collect();
            if let Some(form) = &mut self.form_state {
                egui::CentralPanel::default().show(ctx, |ui| { event_create::show_full_editor(ui, &mut self.state, form, &al, &self.db, &self.theme); });
                if form.saved || !self.state.show_full_editor { if form.saved { self.needs_reload = true; } self.form_state = None; }
            }
        }

        if self.state.show_event_detail {
            if let Some(eid) = &self.state.detail_entry_id {
                if self.detail_entry.as_ref().and_then(|e| e.id.as_deref()) != Some(eid.as_str()) {
                    self.detail_entry = self.db.entry_by_id(eid).ok().flatten();
                }
                if let Some(entry) = &self.detail_entry.clone() {
                    egui::CentralPanel::default().show(ctx, |ui| { event_detail::show_event_detail(ui, &mut self.state, entry, &self.theme, &self.db); });
                }
            }
            if !self.state.show_event_detail { self.detail_entry = None; self.needs_reload = true; }
        }

        // --- Popover Export iCal ---
        if self.state.show_export {
            egui::CentralPanel::default().show(ctx, |ui| {
                export_screen::show_export(ui, &mut self.state, &self.db, &self.theme);
            });
        }

        // --- Layout principal ---
        if !self.state.show_quick_add && !self.state.show_full_editor && !self.state.show_event_detail && !self.state.show_export {
            // Snapshot des filtres avant rendu pour détecter les changements
            let prev_filters_active = self.state.filters_active;
            let prev_filter_sources = self.state.filter_sources.clone();
            let prev_filter_entry_types = self.state.filter_entry_types.clone();

            egui::TopBottomPanel::top("jaykoa_header").show(ctx, |ui| { organisms::header_bar(ui, &mut self.state, &self.theme); });
            egui::SidePanel::left("jaykoa_sidebar").default_width(self.theme.sidebar_width).show(ctx, |ui| {
                organisms::sidebar(ui, &mut self.state, &mut self.agendas, &self.theme, &self.db);
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                if self.state.current_view == ViewId::Settings {
                    settings::show_settings(ui, &mut self.user_settings, &self.db, &self.theme);
                } else {
                    calendar::show_calendar(ui, &mut self.state, &self.cal_data, &self.agendas_colors, &self.theme);
                }
            });

            // Détecter si les filtres ont changé et déclencher un rechargement
            if self.state.filters_active != prev_filters_active
                || self.state.filter_sources != prev_filter_sources
                || self.state.filter_entry_types != prev_filter_entry_types
            {
                self.needs_reload = true;
            }
        }
    }

    fn reload_data(&mut self) {
        self.agendas = self.db.agendas_by_profile(&self.state.profile_id).unwrap_or_default();
        self.agendas_colors = self.agendas.iter()
            .filter_map(|a| Some((a.id.clone()?, a.color.clone().unwrap_or_else(|| "#4285F4".to_string())))).collect();
        let (start, end) = match self.state.current_view {
            ViewId::Day => { let d = self.state.reference_date; (format!("{}T00:00:00", d), format!("{}T23:59:59", d)) }
            ViewId::Week => { let ws = self.state.week_start(); let we = self.state.week_end(); (format!("{}T00:00:00", ws), format!("{}T23:59:59", we)) }
            ViewId::Month => {
                let (y, m) = (self.state.reference_date.year(), self.state.reference_date.month());
                let first = chrono::NaiveDate::from_ymd_opt(y, m, 1).unwrap_or(self.state.reference_date);
                let last = (if m == 12 { chrono::NaiveDate::from_ymd_opt(y + 1, 1, 1) } else { chrono::NaiveDate::from_ymd_opt(y, m + 1, 1) })
                    .unwrap_or(self.state.reference_date) - chrono::Duration::days(1);
                (format!("{}T00:00:00", first), format!("{}T23:59:59", last))
            }
            _ => { let ws = self.state.week_start(); (format!("{}T00:00:00", ws), format!("{}T23:59:59", ws + chrono::Duration::days(30))) }
        };
        let vis: Vec<String> = self.agendas.iter().filter(|a| a.visible).filter_map(|a| a.id.clone()).collect();
        let mut entries = self.db.entries_in_range(&vis, &start, &end).unwrap_or_default();

        // Appliquer les filtres si actifs
        if self.state.filters_active {
            entries.retain(|e| {
                let source_ok = self.state.filter_sources.is_empty()
                    || e.source_service.as_ref().map(|s| self.state.filter_sources.contains(s)).unwrap_or(
                        self.state.filter_sources.contains("jaykoa") // entrées internes sans source = jaykoa
                    );
                let type_ok = self.state.filter_entry_types.is_empty()
                    || e.entry_type.as_ref().map(|t| self.state.filter_entry_types.contains(t)).unwrap_or(true);
                source_ok && type_ok
            });
        }

        let conflicts = self.db.detect_conflicts(&vis, &start, &end).unwrap_or_default();
        let cp: Vec<(String, String)> = conflicts.iter().filter_map(|c| Some((c.entry_a_id.clone()?, c.entry_b_id.clone()?))).collect();
        self.cal_data = CalendarData { entries, conflict_pairs: cp };
    }

    fn run_initial_sync(&self) {
        let pid = &self.state.profile_id;
        if let Some(aid) = self.ensure_synced_agenda(pid, "jayfestival", "JayFestival", "#E67C73") {
            let _ = JayFestivalAdapter::sync_editions(&self.db, &aid);
        }
        if let Some(aid) = self.ensure_synced_agenda(pid, "jayrdv", "JayRDV", "#33B679") {
            let _ = JayRDVAdapter::sync_appointments(&self.db, &aid);
        }
    }

    fn ensure_synced_agenda(&self, profile_id: &str, source_service: &str, name: &str, color: &str) -> Option<String> {
        let agendas = self.db.agendas_by_profile(profile_id).ok()?;
        if let Some(existing) = agendas.iter().find(|a| a.source_service.as_deref() == Some(source_service)) {
            return existing.id.clone();
        }
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let agenda = Agenda {
            id: Some(id.clone()), profile_id: Some(profile_id.to_string()),
            name: Some(name.to_string()), description: Some(format!("Événements depuis {}", name)),
            color: Some(color.to_string()), calendar_type: Some("synced_service".to_string()),
            visible: true, is_default: false, source_service: Some(source_service.to_string()),
            created_at: Some(now.clone()), updated_at: Some(now),
        };
        self.db.agenda_insert(&agenda).ok()?;
        Some(id)
    }
}
