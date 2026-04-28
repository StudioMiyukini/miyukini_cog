//! Section entreprise — informations sur les entreprises liées à l'utilisateur.
//!
//! SIRET, statut juridique, activité, rôle de l'utilisateur dans l'entreprise.

use serde::{Deserialize, Serialize};

/// Rôle de l'utilisateur dans l'entreprise.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseRole {
    #[default]
    Employee,
    Manager,
    Director,
    President,
    Founder,
    CoFounder,
    Associate,
    Freelance,
    Intern,
    Other,
}

/// Informations d'une entreprise.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnterpriseEntry {
    pub id: String,
    /// Raison sociale.
    pub name: String,
    /// Nom commercial (si différent).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trade_name: Option<String>,
    /// Forme juridique (SARL, SAS, EI, auto-entrepreneur…).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_form: Option<String>,
    /// Numéro SIRET (14 chiffres).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub siret: Option<String>,
    /// Numéro SIREN (9 chiffres).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub siren: Option<String>,
    /// Numéro de TVA intracommunautaire.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
    /// Code NAF / APE.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub naf_code: Option<String>,
    /// Activité principale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity: Option<String>,
    /// Date de création (ISO 8601).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// Capital social.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_capital: Option<String>,
    /// Adresse du siège social.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Téléphone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Site web.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// Rôle de l'utilisateur.
    pub user_role: EnterpriseRole,
    /// Entreprise active ?
    #[serde(default = "default_true")]
    pub active: bool,
    /// Documents associés dans le Vault (Kbis, statuts…).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vault_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

fn default_true() -> bool {
    true
}

/// Section entreprises complète.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnterprisesSection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enterprises: Vec<EnterpriseEntry>,
}
