//! Types domaine JayRDV : rendez-vous, créneaux, ressources, services, clients, rappels.
//!
//! @id: jayrdv_data_types
//! @do: define_jayrdv_domain_model
//! @role: data
//! @layer: domain
//! @human: Modèle métier du service JayRDV — entités, statuts, énumérations.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Énumérations
// ---------------------------------------------------------------------------

/// Statut d'un rendez-vous.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppointmentStatus {
    /// En attente de confirmation.
    Pending,
    /// Confirmé.
    Confirmed,
    /// Annulé.
    Cancelled,
    /// Terminé (le client s'est présenté).
    Completed,
    /// Le client ne s'est pas présenté.
    NoShow,
}

impl AppointmentStatus {
    /// Représentation textuelle pour persistance.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Confirmed => "confirmed",
            Self::Cancelled => "cancelled",
            Self::Completed => "completed",
            Self::NoShow => "no_show",
        }
    }
}

impl Default for AppointmentStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Statut d'un créneau.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotStatus {
    /// Disponible à la réservation.
    Available,
    /// Verrouillé temporairement (hold pendant la confirmation).
    Held,
    /// Réservé (un rendez-vous est associé).
    Booked,
    /// Bloqué manuellement par le professionnel.
    Blocked,
}

impl Default for SlotStatus {
    fn default() -> Self {
        Self::Available
    }
}

/// Qui a annulé un rendez-vous.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CancelledBy {
    /// Annulé par le client.
    Client,
    /// Annulé par le professionnel.
    Professional,
}

/// Canal de rappel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReminderChannel {
    /// Rappel par SMS.
    Sms,
    /// Rappel par email.
    Email,
    /// Rappel par notification push.
    Push,
}

/// Rôle du praticien (collaborateur).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PractitionerRole {
    /// Administrateur.
    Admin,
    /// Gestionnaire.
    Manager,
    /// Praticien.
    Practitioner,
}

/// Propriétaire d'un planning (récurrent ou exception).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleOwner {
    /// Praticien (collaborateur).
    Practitioner,
    /// Ressource (salle, équipement).
    Resource,
    /// Professionnel (global).
    Professional,
}

// ---------------------------------------------------------------------------
// Paramètres professionnels
// ---------------------------------------------------------------------------

/// Paramètres du professionnel (politique annulation, préavis, rappels, etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProfessionalSettings {
    /// Délai minimum (heures) avant RDV pour annuler gratuitement.
    pub cancellation_policy_hours: u32,
    /// Autoriser la réservation sans compte.
    pub allow_guest_booking: bool,
    /// OTP téléphone obligatoire.
    pub require_phone_verification: bool,
    /// Durée du verrouillage de créneau (minutes).
    pub hold_duration_min: u32,
    /// Buffer par défaut avant chaque RDV (minutes).
    pub default_buffer_before_min: u32,
    /// Buffer par défaut après chaque RDV (minutes).
    pub default_buffer_after_min: u32,
    /// Préavis minimum (heures).
    pub default_min_notice_hours: u32,
    /// Réservation max à l'avance (jours).
    pub default_max_advance_days: u32,
    /// Rappel à J-7.
    pub reminder_j7: bool,
    /// Rappel à J-1.
    pub reminder_j1: bool,
    /// Rappel à H-2.
    pub reminder_h2: bool,
    /// Canaux de rappel.
    pub reminder_channels: Vec<ReminderChannel>,
    /// Distribution Round Robin activée.
    pub round_robin_enabled: bool,
    /// Stratégie RR (balanced, ordered, weighted).
    pub round_robin_strategy: String,
    /// Liste d'attente si créneaux pleins.
    pub waitlist_enabled: bool,
    /// Acompte obligatoire.
    pub deposit_required: bool,
    /// Paiement intégral à la réservation.
    pub payment_at_booking: bool,
}

impl Default for ProfessionalSettings {
    fn default() -> Self {
        Self {
            cancellation_policy_hours: 24,
            allow_guest_booking: true,
            require_phone_verification: true,
            hold_duration_min: 10,
            default_buffer_before_min: 0,
            default_buffer_after_min: 0,
            default_min_notice_hours: 2,
            default_max_advance_days: 90,
            reminder_j7: false,
            reminder_j1: true,
            reminder_h2: true,
            reminder_channels: vec![ReminderChannel::Sms, ReminderChannel::Email],
            round_robin_enabled: false,
            round_robin_strategy: "balanced".to_string(),
            waitlist_enabled: false,
            deposit_required: false,
            payment_at_booking: false,
        }
    }
}

// ---------------------------------------------------------------------------
// Entités
// ---------------------------------------------------------------------------

/// Profil professionnel (établissement ou indépendant).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Professional {
    /// Identifiant unique.
    pub id: String,
    /// Nom affiché (personne ou entreprise).
    pub name: String,
    /// Identifiant URL-friendly (ex. « marie-coiffure-paris »).
    pub slug: String,
    /// Présentation courte.
    pub description: Option<String>,
    /// Secteur d'activité (beauté, coaching, artisanat…).
    pub sector: String,
    /// URL photo de profil.
    pub photo_url: Option<String>,
    /// Email de contact.
    pub contact_email: String,
    /// Téléphone.
    pub contact_phone: Option<String>,
    /// Adresse physique.
    pub address: Option<String>,
    /// Fuseau horaire (ex. « Europe/Paris »).
    pub timezone: String,
    /// Paramètres (annulation, préavis, rappels…).
    pub settings: ProfessionalSettings,
    /// Horodatage de création.
    pub created_at: String,
    /// Horodatage de dernière modification.
    pub updated_at: String,
}

/// Praticien / collaborateur.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Practitioner {
    /// Identifiant unique.
    pub id: String,
    /// Référence au professionnel (établissement).
    pub professional_id: String,
    /// Nom affiché.
    pub name: String,
    /// Rôle (Admin, Gestionnaire, Praticien).
    pub role: PractitionerRole,
    /// URL photo.
    pub photo_url: Option<String>,
    /// Services que ce praticien peut réaliser.
    pub service_ids: Vec<String>,
    /// Horodatage de création.
    pub created_at: String,
    /// Horodatage de dernière modification.
    pub updated_at: String,
}

/// Planning récurrent (plage horaire par jour de la semaine).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Schedule {
    /// Identifiant unique.
    pub id: String,
    /// Propriétaire (Praticien, Ressource ou Professionnel).
    pub owner_type: ScheduleOwner,
    /// Référence au propriétaire.
    pub owner_id: String,
    /// Jour de la semaine (0 = lundi, 6 = dimanche).
    pub day_of_week: u8,
    /// Heure de début (format "HH:MM" ou "HH:MM:SS").
    pub start_time: String,
    /// Heure de fin (format "HH:MM" ou "HH:MM:SS").
    pub end_time: String,
    /// Actif.
    pub active: bool,
}

/// Exception (congés, absences, fermetures).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Exception {
    /// Identifiant unique.
    pub id: String,
    /// Propriétaire (Praticien, Ressource ou Professionnel).
    pub owner_type: ScheduleOwner,
    /// Référence au propriétaire.
    pub owner_id: String,
    /// Jour de l'exception (date ISO "YYYY-MM-DD").
    pub date: String,
    /// Heure de début (si partiel) ; None = journée entière.
    pub start_time: Option<String>,
    /// Heure de fin (si partiel).
    pub end_time: Option<String>,
    /// Motif (congés, formation, maintenance…).
    pub reason: Option<String>,
}

/// Service (prestation proposée par le professionnel).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Service {
    /// Identifiant unique (UUID v4).
    pub id: String,
    /// Référence au professionnel (optionnel pour rétrocompat).
    pub professional_id: Option<String>,
    /// Nom de la prestation (ex. « Coupe homme »).
    pub name: String,
    /// Description détaillée.
    pub description: Option<String>,
    /// Durée en minutes.
    pub duration_min: u32,
    /// Tarif optionnel.
    pub price: Option<f64>,
    /// Catégorie (ex. « Coiffure », « Massage »).
    pub category: Option<String>,
    /// Actif / inactif.
    pub active: bool,
    /// Horodatage de création (ISO 8601).
    pub created_at: String,
    /// Horodatage de dernière modification (ISO 8601).
    pub updated_at: String,
}

/// Rendez-vous (appointment).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Appointment {
    /// Identifiant unique (UUID v4).
    pub id: String,
    /// Titre affiché.
    pub title: String,
    /// Date/heure de début (ISO 8601).
    pub start_at: String,
    /// Date/heure de fin (ISO 8601).
    pub end_at: String,
    /// Référence à la ressource (optionnel).
    pub resource_id: Option<String>,
    /// Référence au créneau (slot) réservé.
    pub slot_id: Option<String>,
    /// Référence au service.
    pub service_id: Option<String>,
    /// Statut du rendez-vous.
    pub status: AppointmentStatus,
    /// Email du client.
    pub client_email: Option<String>,
    /// Nom du client.
    pub client_name: Option<String>,
    /// Téléphone du client.
    pub client_phone: Option<String>,
    /// Notes / remarques.
    pub notes: Option<String>,
    /// Lieu du rendez-vous.
    pub location: Option<String>,
    /// Motif d'annulation (si annulé).
    pub cancellation_reason: Option<String>,
    /// Qui a annulé (si annulé).
    pub cancelled_by: Option<CancelledBy>,
    /// Horodatage de création.
    pub created_at: String,
    /// Horodatage de dernière modification.
    pub updated_at: String,
}

/// Créneau (slot) — fenêtre de disponibilité.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Slot {
    /// Identifiant unique.
    pub id: String,
    /// Référence à la ressource.
    pub resource_id: String,
    /// Référence au service (optionnel).
    pub service_id: Option<String>,
    /// Date/heure de début.
    pub start_at: String,
    /// Date/heure de fin.
    pub end_at: String,
    /// Statut du créneau.
    pub status: SlotStatus,
    /// Si Held : expiration du verrouillage (ISO 8601).
    pub held_until: Option<String>,
    /// Identifiant de session du client ayant pris le hold.
    pub held_by: Option<String>,
    /// Horodatage de création.
    pub created_at: String,
    /// Horodatage de dernière modification.
    pub updated_at: String,
}

/// Ressource (personne, salle, équipement) pouvant être réservée.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Resource {
    /// Identifiant unique.
    pub id: String,
    /// Référence au professionnel (optionnel pour rétrocompat).
    pub professional_id: Option<String>,
    /// Nom affiché.
    pub name: String,
    /// Type (room, equipment, person…).
    pub kind: Option<String>,
    /// Horodatage de création.
    pub created_at: String,
    /// Horodatage de dernière modification.
    pub updated_at: String,
}

/// Rappel programmé.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Reminder {
    /// Identifiant unique.
    pub id: String,
    /// Référence au rendez-vous.
    pub appointment_id: String,
    /// Canal de rappel.
    pub channel: ReminderChannel,
    /// Date/heure prévue d'envoi.
    pub remind_at: String,
    /// Déjà envoyé.
    pub sent: bool,
    /// Horodatage de création.
    pub created_at: String,
}

/// Fiche client (propre au professionnel — isolation).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Client {
    /// Identifiant unique.
    pub id: String,
    /// Référence au professionnel (fiche isolée par pro).
    pub professional_id: Option<String>,
    /// Nom complet.
    pub name: String,
    /// Email.
    pub email: Option<String>,
    /// Téléphone.
    pub phone: String,
    /// Notes internes du professionnel.
    pub notes: Option<String>,
    /// Nombre total de rendez-vous.
    pub total_appointments: u32,
    /// Nombre de no-show.
    pub no_show_count: u32,
    /// Horodatage de création.
    pub created_at: String,
    /// Horodatage de dernière modification.
    pub updated_at: String,
}
