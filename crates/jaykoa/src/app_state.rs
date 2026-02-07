//! État global et router JayKoa — navigation, vue courante, profil.
//!
//! @id: jaykoa_app_state
//! @do: manage_global_navigation_and_current_view
//! @role: state
//! @layer: app

use chrono::{Datelike, Local, NaiveDate};
use std::collections::HashSet;

/// Écran/vue actif dans JayKoa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewId {
    /// Vue semaine (par défaut).
    Week,
    /// Vue jour.
    Day,
    /// Vue mois.
    Month,
    /// Vue année.
    Year,
    /// Vue planning (liste verticale).
    Schedule,
    /// Paramètres utilisateur.
    Settings,
}

impl ViewId {
    /// Libellé affiché dans le sélecteur de vue.
    pub fn label(self) -> &'static str {
        match self {
            Self::Week => "Semaine",
            Self::Day => "Jour",
            Self::Month => "Mois",
            Self::Year => "Année",
            Self::Schedule => "Planning",
            Self::Settings => "Paramètres",
        }
    }
}

/// État global de l'application JayKoa.
pub struct AppState {
    /// Vue courante.
    pub current_view: ViewId,
    /// Date de référence pour la navigation temporelle.
    pub reference_date: NaiveDate,
    /// Identifiant du profil actif (Central `profile_id`).
    pub profile_id: String,
    /// Popover de création rapide ouverte.
    pub show_quick_add: bool,
    /// Popover de détail d'événement ouverte.
    pub show_event_detail: bool,
    /// ID de l'événement affiché dans le détail.
    pub detail_entry_id: Option<String>,
    /// Mode formulaire complet ouvert.
    pub show_full_editor: bool,
    /// ID de l'événement en cours d'édition (None = création).
    pub editing_entry_id: Option<String>,
    /// Recherche active.
    pub search_query: String,
    /// Recherche visible.
    pub show_search: bool,
    /// Filtres actifs pour la vue calendrier.
    pub filter_sources: HashSet<String>,
    /// Filtres par type d'entrée.
    pub filter_entry_types: HashSet<String>,
    /// Si true, les filtres sont actifs (sinon tout est affiché).
    pub filters_active: bool,
    /// Panneau d'export ouvert.
    pub show_export: bool,
}

impl AppState {
    /// Crée un nouvel état avec la date du jour et la vue semaine.
    pub fn new() -> Self {
        Self {
            current_view: ViewId::Week,
            reference_date: Local::now().date_naive(),
            profile_id: "default_user".to_string(),
            show_quick_add: false,
            show_event_detail: false,
            detail_entry_id: None,
            show_full_editor: false,
            editing_entry_id: None,
            search_query: String::new(),
            show_search: false,
            filter_sources: HashSet::new(),
            filter_entry_types: HashSet::new(),
            filters_active: false,
            show_export: false,
        }
    }

    /// Navigue à la date d'aujourd'hui.
    pub fn go_today(&mut self) { self.reference_date = Local::now().date_naive(); }

    /// Navigue en avant (selon la vue courante).
    pub fn go_next(&mut self) {
        self.reference_date = match self.current_view {
            ViewId::Day => self.reference_date.succ_opt().unwrap_or(self.reference_date),
            ViewId::Week => self.reference_date + chrono::Duration::weeks(1),
            ViewId::Month => {
                let (y, m) = (self.reference_date.year(), self.reference_date.month());
                if m == 12 { NaiveDate::from_ymd_opt(y + 1, 1, 1) } else { NaiveDate::from_ymd_opt(y, m + 1, 1) }
                    .unwrap_or(self.reference_date)
            }
            ViewId::Year => NaiveDate::from_ymd_opt(self.reference_date.year() + 1, 1, 1).unwrap_or(self.reference_date),
            _ => self.reference_date + chrono::Duration::weeks(1),
        };
    }

    /// Navigue en arrière (selon la vue courante).
    pub fn go_prev(&mut self) {
        self.reference_date = match self.current_view {
            ViewId::Day => self.reference_date.pred_opt().unwrap_or(self.reference_date),
            ViewId::Week => self.reference_date - chrono::Duration::weeks(1),
            ViewId::Month => {
                let (y, m) = (self.reference_date.year(), self.reference_date.month());
                if m == 1 { NaiveDate::from_ymd_opt(y - 1, 12, 1) } else { NaiveDate::from_ymd_opt(y, m - 1, 1) }
                    .unwrap_or(self.reference_date)
            }
            ViewId::Year => NaiveDate::from_ymd_opt(self.reference_date.year() - 1, 1, 1).unwrap_or(self.reference_date),
            _ => self.reference_date - chrono::Duration::weeks(1),
        };
    }

    /// Retourne le lundi de la semaine contenant `reference_date`.
    pub fn week_start(&self) -> NaiveDate {
        let wd = self.reference_date.weekday().num_days_from_monday();
        self.reference_date - chrono::Duration::days(wd as i64)
    }

    /// Retourne le dimanche de la semaine contenant `reference_date`.
    pub fn week_end(&self) -> NaiveDate { self.week_start() + chrono::Duration::days(6) }

    /// Titre de navigation (ex: "3 - 9 Fév. 2026").
    pub fn navigation_title(&self) -> String {
        match self.current_view {
            ViewId::Day => self.reference_date.format("%A %d %B %Y").to_string(),
            ViewId::Week => {
                let s = self.week_start();
                let e = self.week_end();
                if s.month() == e.month() {
                    format!("{} - {} {} {}", s.day(), e.day(), month_name_fr(s.month()), s.year())
                } else {
                    format!("{} {} - {} {} {}", s.day(), month_short_fr(s.month()), e.day(), month_short_fr(e.month()), e.year())
                }
            }
            ViewId::Month => format!("{} {}", month_name_fr(self.reference_date.month()), self.reference_date.year()),
            ViewId::Year => format!("{}", self.reference_date.year()),
            _ => format!("{} {}", month_name_fr(self.reference_date.month()), self.reference_date.year()),
        }
    }
}

impl Default for AppState { fn default() -> Self { Self::new() } }

/// Nom du mois en français.
pub fn month_name_fr(m: u32) -> &'static str {
    match m {
        1=>"Janvier", 2=>"Février", 3=>"Mars", 4=>"Avril", 5=>"Mai", 6=>"Juin",
        7=>"Juillet", 8=>"Août", 9=>"Septembre", 10=>"Octobre", 11=>"Novembre", 12=>"Décembre", _=>"?",
    }
}

fn month_short_fr(m: u32) -> &'static str {
    match m {
        1=>"Jan.", 2=>"Fév.", 3=>"Mars", 4=>"Avr.", 5=>"Mai", 6=>"Juin",
        7=>"Juil.", 8=>"Août", 9=>"Sep.", 10=>"Oct.", 11=>"Nov.", 12=>"Déc.", _=>"?",
    }
}
