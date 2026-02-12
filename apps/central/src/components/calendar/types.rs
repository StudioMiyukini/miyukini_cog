//! Types generiques pour les composants calendrier.
//!
//! @id: central.components.calendar.types
//! @do: define_generic_calendar_types_and_traits
//! @role: data
//! @layer: domain
//! @human: Types generiques pour calendrier (events, slots, modes)
//!
//! Note: types generiques réutilisables — certains ne sont pas encore
//! consommés mais font partie de l'API publique du module calendrier.
#![allow(dead_code)]

use chrono::{NaiveDate, NaiveDateTime};

/// Mode d'affichage du calendrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CalendarViewMode {
    /// Vue jour (grille horaire detaillee).
    Day,
    /// Vue semaine (7 colonnes × 24h).
    #[default]
    Week,
    /// Vue mois (grille mensuelle compacte).
    Month,
    /// Vue liste (agenda liste).
    List,
}

/// Alias pour compatibilite.
pub type ViewMode = CalendarViewMode;

impl CalendarViewMode {
    /// Retourne le label en francais.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Day => "Jour",
            Self::Week => "Semaine",
            Self::Month => "Mois",
            Self::List => "Liste",
        }
    }
}

/// Trait pour les evenements de calendrier.
///
/// Permet d'utiliser les composants calendrier avec n'importe quel type d'evenement
/// (TemporalEntry de JayKoa, Animation de JayFestival, etc.)
pub trait CalendarEvent: Clone + PartialEq {
    /// Identifiant unique de l'evenement.
    fn id(&self) -> Option<&str>;

    /// Titre de l'evenement.
    fn title(&self) -> &str;

    /// Description optionnelle.
    fn description(&self) -> Option<&str> {
        None
    }

    /// Date/heure de debut (format ISO: "YYYY-MM-DDTHH:MM:SS").
    fn start_datetime(&self) -> Option<&str>;

    /// Date/heure de fin (format ISO: "YYYY-MM-DDTHH:MM:SS").
    fn end_datetime(&self) -> Option<&str>;

    /// Couleur d'affichage (hex: "#RRGGBB").
    fn color(&self) -> &str {
        "#4285F4"
    }

    /// Indique si c'est un evenement sur toute la journee.
    fn is_all_day(&self) -> bool {
        false
    }

    /// Lieu optionnel.
    fn location(&self) -> Option<&str> {
        None
    }
}

/// Creneau horaire pour la grille.
#[derive(Debug, Clone, PartialEq)]
pub struct TimeSlot {
    /// Heure (0-23).
    pub hour: u32,
    /// Date du creneau.
    pub date: NaiveDate,
}

impl TimeSlot {
    /// Cree un nouveau creneau.
    #[must_use]
    pub fn new(date: NaiveDate, hour: u32) -> Self {
        Self { date, hour }
    }

    /// Retourne le datetime au format ISO.
    #[must_use]
    pub fn to_iso_string(&self) -> String {
        format!("{}T{:02}:00:00", self.date.format("%Y-%m-%d"), self.hour)
    }
}

/// Conflit entre deux evenements.
#[derive(Debug, Clone, PartialEq)]
pub struct EventConflict {
    /// ID du premier evenement.
    pub event_a_id: Option<String>,
    /// ID du second evenement.
    pub event_b_id: Option<String>,
    /// Type de conflit.
    pub conflict_type: ConflictType,
}

/// Type de conflit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictType {
    /// Chevauchement partiel.
    Overlap,
    /// Meme creneau exact.
    Exact,
    /// Evenements adjacents sans pause.
    Adjacent,
}

/// Evenement simple pour utilisation directe.
///
/// Implemente CalendarEvent pour une utilisation sans type externe.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleEvent {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Titre.
    pub title: String,
    /// Description.
    pub description: Option<String>,
    /// Debut (ISO format).
    pub start: Option<String>,
    /// Fin (ISO format).
    pub end: Option<String>,
    /// Couleur hex.
    pub color: String,
    /// Journee entiere.
    pub all_day: bool,
    /// Lieu.
    pub location: Option<String>,
}

impl SimpleEvent {
    /// Cree un nouvel evenement simple.
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: None,
            title: title.into(),
            description: None,
            start: None,
            end: None,
            color: "#4285F4".to_string(),
            all_day: false,
            location: None,
        }
    }

    /// Definit l'ID.
    #[must_use]
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Definit les dates.
    #[must_use]
    pub fn with_datetime(mut self, start: impl Into<String>, end: impl Into<String>) -> Self {
        self.start = Some(start.into());
        self.end = Some(end.into());
        self
    }

    /// Definit la couleur.
    #[must_use]
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    /// Definit comme journee entiere.
    #[must_use]
    pub fn all_day(mut self) -> Self {
        self.all_day = true;
        self
    }
}

impl CalendarEvent for SimpleEvent {
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn start_datetime(&self) -> Option<&str> {
        self.start.as_deref()
    }

    fn end_datetime(&self) -> Option<&str> {
        self.end.as_deref()
    }

    fn color(&self) -> &str {
        &self.color
    }

    fn is_all_day(&self) -> bool {
        self.all_day
    }

    fn location(&self) -> Option<&str> {
        self.location.as_deref()
    }
}

/// Helper pour parser les datetimes.
pub fn parse_datetime(s: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S").ok()
}

/// Helper pour obtenir la date d'un datetime string.
pub fn extract_date(s: &str) -> Option<NaiveDate> {
    if s.len() >= 10 {
        NaiveDate::parse_from_str(&s[..10], "%Y-%m-%d").ok()
    } else {
        None
    }
}

/// Configuration du calendrier.
#[derive(Debug, Clone, PartialEq)]
pub struct CalendarConfig {
    /// Hauteur d'une heure en pixels.
    pub hour_height: u32,
    /// Heure de debut (0-23).
    pub start_hour: u32,
    /// Heure de fin (0-23).
    pub end_hour: u32,
    /// Afficher le weekend.
    pub show_weekend: bool,
}

impl Default for CalendarConfig {
    fn default() -> Self {
        Self {
            hour_height: 48,
            start_hour: 8,
            end_hour: 20,
            show_weekend: true,
        }
    }
}

/// Noms des jours de la semaine en francais (format court).
pub const WEEKDAY_NAMES_SHORT: [&str; 7] = ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"];

/// Noms des jours de la semaine en francais (format long).
pub const WEEKDAY_NAMES_LONG: [&str; 7] = [
    "Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi", "Dimanche"
];

/// Noms des mois en francais.
pub const MONTH_NAMES: [&str; 12] = [
    "Janvier", "Février", "Mars", "Avril", "Mai", "Juin",
    "Juillet", "Août", "Septembre", "Octobre", "Novembre", "Décembre"
];

/// Retourne le nom du mois en francais (1-12).
#[must_use]
pub fn month_name(month: u32) -> &'static str {
    MONTH_NAMES.get((month.saturating_sub(1)) as usize).unwrap_or(&"?")
}

/// Retourne le nom du jour de la semaine en francais.
#[must_use]
pub fn weekday_name(weekday: chrono::Weekday, short: bool) -> &'static str {
    let idx = weekday.num_days_from_monday() as usize;
    if short {
        WEEKDAY_NAMES_SHORT.get(idx).unwrap_or(&"?")
    } else {
        WEEKDAY_NAMES_LONG.get(idx).unwrap_or(&"?")
    }
}
