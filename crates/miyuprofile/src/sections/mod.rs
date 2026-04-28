//! Sections structurées du profil Central.
//!
//! Chaque section modélise un domaine de données personnelles
//! nécessaire au fonctionnement d'une assistante personnelle.

pub mod contact;
pub mod contracts;
pub mod credentials;
pub mod documents;
pub mod enterprise;
pub mod finance;
pub mod health;
pub mod identity;
pub mod professional;

pub use contact::ContactSection;
pub use contracts::ContractsSection;
pub use credentials::CredentialsSection;
pub use documents::DocumentsSection;
pub use enterprise::EnterprisesSection;
pub use finance::FinanceSection;
pub use health::HealthSection;
pub use identity::IdentitySection;
pub use professional::ProfessionalSection;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Profil Central complet — banque de données personnelles.
///
/// Regroupe toutes les sections structurées qu'une assistante personnelle
/// a besoin pour effectuer ses tâches déléguées : CRUD de documents,
/// suivi, correspondance, constitution de dossiers, rappels, etc.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CentralProfile {
    pub user_id: String,

    // ── Sections structurées ──────────────────────────────────────
    /// État civil, nationalité, numéros officiels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentitySection>,

    /// Coordonnées (adresses, téléphones, emails, web).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts: Option<ContactSection>,

    /// Documents officiels (CNI, passeport, permis…).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsSection>,

    /// Santé (carnet, ordonnances, médecins, allergies).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<HealthSection>,

    /// CV structuré (expériences, formations, compétences).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub professional: Option<ProfessionalSection>,

    /// Entreprises liées.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprises: Option<EnterprisesSection>,

    /// Contrats, assurances, abonnements.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts: Option<ContractsSection>,

    /// Coordonnées bancaires, moyens de paiement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finance: Option<FinanceSection>,

    /// Identifiants et mots de passe (chiffrés).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<CredentialsSection>,

    // ── Données libres (rétro-compatibilité) ─────────────────────
    /// Préférences utilisateur.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub preferences: HashMap<String, String>,

    /// Champs libres (anciens `ProfileData.fields` ; rétro-compatibilité).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub custom_fields: HashMap<String, String>,
}

/// Nom de section pour accès dynamique.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SectionName {
    Identity,
    Contacts,
    Documents,
    Health,
    Professional,
    Enterprises,
    Contracts,
    Finance,
    Credentials,
    Preferences,
    CustomFields,
}

impl SectionName {
    /// Toutes les sections disponibles.
    pub fn all() -> &'static [SectionName] {
        &[
            SectionName::Identity,
            SectionName::Contacts,
            SectionName::Documents,
            SectionName::Health,
            SectionName::Professional,
            SectionName::Enterprises,
            SectionName::Contracts,
            SectionName::Finance,
            SectionName::Credentials,
            SectionName::Preferences,
            SectionName::CustomFields,
        ]
    }

    /// Nom lisible.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Identity => "État civil",
            Self::Contacts => "Coordonnées",
            Self::Documents => "Documents officiels",
            Self::Health => "Santé",
            Self::Professional => "Professionnel / CV",
            Self::Enterprises => "Entreprises",
            Self::Contracts => "Contrats & assurances",
            Self::Finance => "Finance & bancaire",
            Self::Credentials => "Identifiants & mots de passe",
            Self::Preferences => "Préférences",
            Self::CustomFields => "Champs personnalisés",
        }
    }
}
