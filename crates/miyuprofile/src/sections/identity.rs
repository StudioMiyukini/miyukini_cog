//! Section identité — état civil, nationalité, numéros officiels.
//!
//! Données fondamentales d'identification de la personne physique.

use serde::{Deserialize, Serialize};

/// Civilité.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Civility {
    #[default]
    Unspecified,
    Monsieur,
    Madame,
    Mx,
}

/// Section état civil.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdentitySection {
    pub civility: Civility,
    pub first_name: String,
    pub last_name: String,
    /// Nom de naissance (si différent).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,
    /// Prénoms complets (état civil).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_given_names: Option<String>,
    /// Date de naissance (ISO 8601 : YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// Lieu de naissance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place_of_birth: Option<String>,
    /// Pays de naissance (code ISO 3166-1 alpha-2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_of_birth: Option<String>,
    /// Nationalités (peut être multiple).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nationalities: Vec<String>,
    /// Numéro de sécurité sociale (NIR en France).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_security_number: Option<String>,
    /// Numéro fiscal (SPI en France).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// Situation familiale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<MaritalStatus>,
    /// Nombre d'enfants.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children_count: Option<u32>,
    /// Groupe sanguin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blood_type: Option<String>,
    /// Notes libres.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Situation familiale.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MaritalStatus {
    Single,
    Married,
    Pacs,
    Divorced,
    Widowed,
    Separated,
}
