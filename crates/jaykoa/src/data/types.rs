//! Types domaine JayKoa — Modèle métier du calendrier universel.
//!
//! @id: jaykoa_domain_types
//! @do: define_jaykoa_domain_model
//! @role: data
//! @layer: domain
//! @human: Agenda, événement interne, reflet externe, engagement temporel,
//!         source, statut, conflit (visualisation uniquement).
//!
//! INVARIANTS (Document Fondateur §4) :
//! - JayKoa ne crée jamais d'événement externe
//! - JayKoa ne modifie jamais un booking
//! - JayKoa ne calcule aucune disponibilité
//! - JayKoa ne décide jamais du temps
//! - Toute écriture passe par KindMother (WriteIntent)
//! - Les reflets externes sont strictement en lecture seule

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Agenda (Calendrier)
// ---------------------------------------------------------------------------

/// Calendrier appartenant à un utilisateur.
///
/// Un utilisateur peut posséder plusieurs agendas (personnel, professionnel,
/// par service synchronisé). Chaque agenda contient des engagements temporels.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Agenda {
    /// Identifiant unique (UUID v4).
    pub id: Option<String>,
    /// Identifiant du profil propriétaire (Central `profile_id`).
    pub profile_id: Option<String>,
    /// Nom affiché du calendrier (ex: "Personnel", "JayRDV").
    pub name: Option<String>,
    /// Description optionnelle.
    pub description: Option<String>,
    /// Couleur d'affichage (hex, ex: "#4285F4").
    pub color: Option<String>,
    /// Type de calendrier.
    pub calendar_type: Option<String>,
    /// Visible dans l'interface (toggle sidebar).
    #[serde(default = "default_true")]
    pub visible: bool,
    /// Calendrier par défaut pour la création d'événements internes.
    #[serde(default)]
    pub is_default: bool,
    /// Service source pour les calendriers synchronisés (None = interne).
    pub source_service: Option<String>,
    /// Horodatage de création.
    pub created_at: Option<String>,
    /// Horodatage de dernière modification.
    pub updated_at: Option<String>,
}

/// Types de calendrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CalendarType {
    /// Calendrier personnel (événements créés par l'utilisateur).
    Personal,
    /// Calendrier professionnel.
    Work,
    /// Calendrier synchronisé depuis un service externe (lecture seule).
    SyncedService,
}

impl CalendarType {
    /// Représentation textuelle pour persistance.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Personal => "personal",
            Self::Work => "work",
            Self::SyncedService => "synced_service",
        }
    }
    /// Parse depuis une chaîne.
    pub fn from_str(s: &str) -> Self {
        match s {
            "personal" => Self::Personal,
            "work" => Self::Work,
            "synced_service" => Self::SyncedService,
            _ => Self::Personal,
        }
    }
}

// ---------------------------------------------------------------------------
// Engagement Temporel (Temporal Entry)
// ---------------------------------------------------------------------------

/// Entrée temporelle dans un agenda : événement interne OU reflet externe.
///
/// C'est l'unité fondamentale de JayKoa. Chaque engagement occupe une plage
/// temporelle et possède un statut, une source, et un contexte visuel.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemporalEntry {
    /// Identifiant unique (UUID v4).
    pub id: Option<String>,
    /// Identifiant de l'agenda parent.
    pub agenda_id: Option<String>,
    /// Titre de l'engagement.
    pub title: Option<String>,
    /// Description / notes.
    pub description: Option<String>,
    /// Date et heure de début (ISO 8601).
    pub start_datetime: Option<String>,
    /// Date et heure de fin (ISO 8601).
    pub end_datetime: Option<String>,
    /// Événement sur toute la journée.
    #[serde(default)]
    pub all_day: bool,
    /// Lieu (texte libre).
    pub location: Option<String>,
    /// Statut temporel (confirmed, tentative, cancelled).
    pub status: Option<String>,
    /// Type d'entrée : interne ou reflet externe.
    pub entry_type: Option<String>,
    /// Source de l'événement (pour reflets externes).
    pub source_service: Option<String>,
    /// ID de l'événement dans le service source (pour reflets externes).
    pub source_event_id: Option<String>,
    /// Couleur personnalisée (override de la couleur agenda).
    pub color: Option<String>,
    /// Règle de récurrence (RRULE iCalendar simplifié, optionnel).
    pub recurrence_rule: Option<String>,
    /// Rappel(s) en minutes avant l'événement (JSON array).
    pub reminders_json: Option<String>,
    /// Horodatage de création.
    pub created_at: Option<String>,
    /// Horodatage de dernière modification.
    pub updated_at: Option<String>,
    /// Horodatage de dernière synchronisation (reflets externes).
    pub last_synced_at: Option<String>,
}

// ---------------------------------------------------------------------------
// Type d'entrée (Entry Type)
// ---------------------------------------------------------------------------

/// Distingue un événement interne (créé par l'utilisateur) d'un reflet
/// (projection en lecture seule d'un événement provenant d'un autre Service).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryType {
    /// Événement interne créé par l'utilisateur dans JayKoa.
    Internal,
    /// Reflet d'un rendez-vous JayRDV (lecture seule).
    ReflectJayRDV,
    /// Reflet d'un service futur (lecture seule).
    ReflectOther,
}

impl EntryType {
    /// Représentation textuelle pour persistance.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::ReflectJayRDV => "reflect_jayrdv",
            Self::ReflectOther => "reflect_other",
        }
    }
    /// Parse depuis une chaîne.
    pub fn from_str(s: &str) -> Self {
        match s {
            "internal" => Self::Internal,
            "reflect_jayrdv" => Self::ReflectJayRDV,
            "reflect_other" => Self::ReflectOther,
            _ => Self::Internal,
        }
    }
    /// L'entrée est-elle en lecture seule (reflet externe) ?
    pub fn is_readonly(self) -> bool {
        !matches!(self, Self::Internal)
    }
}

// ---------------------------------------------------------------------------
// Statut Temporel
// ---------------------------------------------------------------------------

/// Statut d'un engagement temporel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemporalStatus {
    /// Confirmé.
    Confirmed,
    /// Provisoire / à confirmer.
    Tentative,
    /// Annulé.
    Cancelled,
}

impl TemporalStatus {
    /// Représentation textuelle pour persistance.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Confirmed => "confirmed",
            Self::Tentative => "tentative",
            Self::Cancelled => "cancelled",
        }
    }
    /// Parse depuis une chaîne.
    pub fn from_str(s: &str) -> Self {
        match s {
            "confirmed" => Self::Confirmed,
            "tentative" => Self::Tentative,
            "cancelled" => Self::Cancelled,
            _ => Self::Confirmed,
        }
    }
}

// ---------------------------------------------------------------------------
// Source d'événement (Event Source)
// ---------------------------------------------------------------------------

/// Identifiant du service source d'un événement réfléchi.
///
/// INVARIANT : JayKoa ne possède pas les données sources. Il ne fait que
/// refléter ce que les services émetteurs déclarent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventSource {
    /// Créé localement dans JayKoa.
    JayKoa,
    /// Provient de JayRDV (rendez-vous confirmés).
    JayRDV,
    /// Provient d'un service futur non encore défini.
    Other,
}

impl EventSource {
    /// Représentation textuelle pour persistance.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::JayKoa => "jaykoa",
            Self::JayRDV => "jayrdv",
            Self::Other => "other",
        }
    }
    /// Parse depuis une chaîne.
    pub fn from_str(s: &str) -> Self {
        match s {
            "jaykoa" => Self::JayKoa,
            "jayrdv" => Self::JayRDV,
            _ => Self::Other,
        }
    }
    /// Libellé affiché dans l'UI.
    pub fn display_label(self) -> &'static str {
        match self {
            Self::JayKoa => "JayKoa",
            Self::JayRDV => "JayRDV",
            Self::Other => "Autre service",
        }
    }
    /// Couleur par défaut du badge source.
    pub fn default_color(self) -> &'static str {
        match self {
            Self::JayKoa => "#4285F4",
            Self::JayRDV => "#33B679",
            Self::Other => "#8E8E93",
        }
    }
}

// ---------------------------------------------------------------------------
// Conflit Temporel (visualisation uniquement)
// ---------------------------------------------------------------------------

/// Conflit temporel détecté entre deux engagements qui se chevauchent.
///
/// INVARIANT : Le conflit est un indicateur VISUEL uniquement.
/// JayKoa ne résout jamais les conflits. Il les signale à l'utilisateur.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemporalConflict {
    /// ID du premier engagement en conflit.
    pub entry_a_id: Option<String>,
    /// ID du second engagement en conflit.
    pub entry_b_id: Option<String>,
    /// Début du chevauchement.
    pub overlap_start: Option<String>,
    /// Fin du chevauchement.
    pub overlap_end: Option<String>,
    /// Description du conflit (générée automatiquement).
    pub description: Option<String>,
}

// ---------------------------------------------------------------------------
// Paramètres utilisateur JayKoa
// ---------------------------------------------------------------------------

/// Préférences utilisateur pour l'affichage du calendrier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserSettings {
    /// Identifiant du profil propriétaire.
    pub profile_id: Option<String>,
    /// Vue par défaut (week, day, month, year, schedule).
    pub default_view: String,
    /// Premier jour de la semaine (0 = lundi, 6 = dimanche).
    pub week_start_day: u8,
    /// Heure de début d'affichage (0-23).
    pub day_start_hour: u8,
    /// Heure de fin d'affichage (0-23).
    pub day_end_hour: u8,
    /// Fuseau horaire (ex: "Europe/Paris").
    pub timezone: String,
    /// Rappel par défaut (minutes avant l'événement).
    pub default_reminder_minutes: i32,
    /// Format horaire (true = 24h, false = 12h).
    pub use_24h_format: bool,
    /// Afficher les numéros de semaine.
    pub show_week_numbers: bool,
    /// Afficher les événements refusés/annulés.
    pub show_cancelled_events: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            profile_id: None,
            default_view: "week".to_string(),
            week_start_day: 0,
            day_start_hour: 8,
            day_end_hour: 20,
            timezone: "Europe/Paris".to_string(),
            default_reminder_minutes: 30,
            use_24h_format: true,
            show_week_numbers: false,
            show_cancelled_events: false,
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Helper serde pour les champs bool par défaut `true`.
fn default_true() -> bool {
    true
}

// ---------------------------------------------------------------------------
// Tests unitaires
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_type_roundtrip() {
        let variants = [
            EntryType::Internal,
            EntryType::ReflectJayRDV,
            EntryType::ReflectOther,
        ];
        for v in variants {
            let roundtripped = EntryType::from_str(v.as_str());
            assert_eq!(roundtripped, v, "roundtrip failed for {:?}", v);
        }
    }

    #[test]
    fn test_entry_type_readonly() {
        assert!(
            !EntryType::Internal.is_readonly(),
            "Internal should NOT be readonly"
        );
        assert!(
            EntryType::ReflectJayRDV.is_readonly(),
            "ReflectJayRDV should be readonly"
        );
        assert!(
            EntryType::ReflectOther.is_readonly(),
            "ReflectOther should be readonly"
        );
    }

    #[test]
    fn test_temporal_status_roundtrip() {
        let variants = [
            TemporalStatus::Confirmed,
            TemporalStatus::Tentative,
            TemporalStatus::Cancelled,
        ];
        for v in variants {
            let roundtripped = TemporalStatus::from_str(v.as_str());
            assert_eq!(roundtripped, v, "roundtrip failed for {:?}", v);
        }
    }

    #[test]
    fn test_event_source_roundtrip() {
        let variants = [
            EventSource::JayKoa,
            EventSource::JayRDV,
            EventSource::Other,
        ];
        for v in variants {
            let roundtripped = EventSource::from_str(v.as_str());
            assert_eq!(roundtripped, v, "roundtrip failed for {:?}", v);
        }
    }

    #[test]
    fn test_calendar_type_roundtrip() {
        let variants = [
            CalendarType::Personal,
            CalendarType::Work,
            CalendarType::SyncedService,
        ];
        for v in variants {
            let roundtripped = CalendarType::from_str(v.as_str());
            assert_eq!(roundtripped, v, "roundtrip failed for {:?}", v);
        }
    }

    #[test]
    fn test_default_user_settings() {
        let s = UserSettings::default();
        assert!(
            s.profile_id.is_none(),
            "profile_id should be None by default"
        );
        assert_eq!(s.default_view, "week");
        assert_eq!(s.week_start_day, 0);
        assert_eq!(s.day_start_hour, 8);
        assert_eq!(s.day_end_hour, 20);
        assert_eq!(s.timezone, "Europe/Paris");
        assert_eq!(s.default_reminder_minutes, 30);
        assert!(s.use_24h_format, "use_24h_format should be true");
        assert!(!s.show_week_numbers, "show_week_numbers should be false");
        assert!(
            !s.show_cancelled_events,
            "show_cancelled_events should be false"
        );
    }
}
