//! Section santé — carnet de santé, ordonnances, médecins, allergies.
//!
//! Données médicales personnelles pour suivi et constitution de dossiers.

use serde::{Deserialize, Serialize};

/// Entrée médecin/praticien.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DoctorEntry {
    pub id: String,
    /// Spécialité (généraliste, dermatologue, dentiste…).
    pub specialty: String,
    pub full_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Numéro RPPS / ADELI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    /// Médecin traitant ?
    #[serde(default)]
    pub is_primary: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Allergie.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AllergyEntry {
    pub allergen: String,
    /// Sévérité : mild, moderate, severe, life_threatening.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reaction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnosed_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Vaccination.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VaccinationEntry {
    pub id: String,
    pub vaccine_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disease: Option<String>,
    /// Date d'injection (ISO 8601).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Numéro de lot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<String>,
    /// Médecin/centre de vaccination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administered_by: Option<String>,
    /// Date du prochain rappel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_booster_date: Option<String>,
    /// Référence vers le scan dans le Vault.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vault_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Ordonnance médicale.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrescriptionEntry {
    pub id: String,
    /// Médecin prescripteur.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doctor_id: Option<String>,
    /// Date de prescription (ISO 8601).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Date de fin de validité.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
    /// Médicaments prescrits.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub medications: Vec<MedicationEntry>,
    /// Renouvellement possible ?
    #[serde(default)]
    pub renewable: bool,
    /// Nombre de renouvellements restants.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renewals_remaining: Option<u32>,
    /// Scan de l'ordonnance dans le Vault.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vault_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Médicament.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MedicationEntry {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dosage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Couverture santé (sécurité sociale + mutuelle).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthCoverage {
    /// Caisse d'assurance maladie.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_fund: Option<String>,
    /// Numéro d'assuré.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_number: Option<String>,
    /// Mutuelle / complémentaire santé.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_name: Option<String>,
    /// Numéro de contrat mutuelle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_contract: Option<String>,
    /// Numéro d'adhérent mutuelle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_member: Option<String>,
    /// Date de fin de couverture mutuelle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementary_expiry: Option<String>,
    /// Scans carte vitale / attestation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vault_refs: Vec<String>,
}

/// Condition médicale / antécédent.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MedicalCondition {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnosed_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub treating_doctor_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Contact d'urgence.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmergencyContact {
    pub full_name: String,
    pub relationship: String,
    pub phone: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default)]
    pub primary: bool,
}

/// Section santé complète.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthSection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage: Option<HealthCoverage>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub doctors: Vec<DoctorEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allergies: Vec<AllergyEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<MedicalCondition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vaccinations: Vec<VaccinationEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prescriptions: Vec<PrescriptionEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emergency_contacts: Vec<EmergencyContact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
