//! Types domaine JayFaim : tables, créneaux, réservations, convives.
//!
//! Modèle : lieu/établissement → tables → créneaux de réservation → réservations (convives).

use serde::{Deserialize, Serialize};

/// Statut d'une réservation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReservationStatus {
    /// En attente de confirmation.
    Pending,
    /// Confirmée.
    Confirmed,
    /// Annulée.
    Cancelled,
    /// Terminée (convives venus).
    Completed,
    /// No-show.
    NoShow,
}

impl Default for ReservationStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Table (restaurant, food truck, stand).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    /// Identifiant unique.
    pub id: String,
    /// Nom ou numéro (ex. "Table 1", "Stand A3").
    pub name: String,
    /// Capacité en nombre de convives.
    pub capacity: u32,
    /// Identifiant du lieu/établissement (optionnel).
    pub venue_id: Option<String>,
    /// Actif pour réservation.
    pub active: bool,
}

/// Créneau de réservation (plage horaire pour une table ou un lieu).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationSlot {
    /// Identifiant unique.
    pub id: String,
    /// Table concernée (ou lieu global si table_id = None pour "premier dispo").
    pub table_id: Option<String>,
    /// Lieu/établissement.
    pub venue_id: String,
    /// Date (ISO date).
    pub date: String,
    /// Heure début (ISO time ou "HH:MM").
    pub start_time: String,
    /// Heure fin.
    pub end_time: String,
    /// Capacité max pour ce créneau (défaut = capacité table).
    pub max_guests: u32,
}

/// Convive (lien réservation ↔ client ou invité).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guest {
    /// Identifiant unique.
    pub id: String,
    /// Réservation associée.
    pub reservation_id: String,
    /// Nom affiché ou identifiant client.
    pub name: String,
    /// Email (optionnel).
    pub email: Option<String>,
}

/// Réservation (table, créneau, convives).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
    /// Identifiant unique.
    pub id: String,
    /// Table réservée.
    pub table_id: String,
    /// Lieu/établissement.
    pub venue_id: String,
    /// Date (ISO date).
    pub date: String,
    /// Heure début (ISO time ou "HH:MM").
    pub start_time: String,
    /// Heure fin.
    pub end_time: String,
    /// Nombre de convives.
    pub guest_count: u32,
    /// Identifiant client (mandat ou tiers).
    pub client_id: String,
    /// Statut.
    pub status: ReservationStatus,
    /// Commentaire (optionnel).
    pub note: Option<String>,
    /// Créée à (ISO 8601).
    pub created_at: String,
    /// Mise à jour (ISO 8601).
    pub updated_at: String,
}
