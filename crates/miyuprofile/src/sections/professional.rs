//! Section professionnelle — CV, expériences, compétences, formations.
//!
//! Données structurées pour exposer un CV, constituer un dossier
//! de candidature, ou fournir un contexte professionnel à une assistante.

use serde::{Deserialize, Serialize};

/// Niveau de compétence.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SkillLevel {
    #[default]
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// Niveau de langue.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LanguageLevel {
    #[default]
    A1,
    A2,
    B1,
    B2,
    C1,
    C2,
    Native,
}

/// Expérience professionnelle.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExperienceEntry {
    pub id: String,
    pub job_title: String,
    pub company: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Date de début (ISO 8601 : YYYY-MM ou YYYY-MM-DD).
    pub start_date: String,
    /// Date de fin (vide = en cours).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Poste actuel ?
    #[serde(default)]
    pub current: bool,
    /// Description du poste / missions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Réalisations clés.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub achievements: Vec<String>,
    /// Type de contrat (CDI, CDD, freelance…).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// Formation / diplôme.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EducationEntry {
    pub id: String,
    pub institution: String,
    pub degree: String,
    /// Domaine d'étude.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_of_study: Option<String>,
    pub start_date: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default)]
    pub current: bool,
    /// Mention / distinction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub honors: Option<String>,
    /// Référence scan diplôme dans le Vault.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vault_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Compétence.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillEntry {
    pub name: String,
    pub level: SkillLevel,
    /// Catégorie (technique, soft, métier…).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Années d'expérience.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub years: Option<u32>,
}

/// Certification.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CertificationEntry {
    pub id: String,
    pub name: String,
    pub issuing_organization: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_url: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vault_refs: Vec<String>,
}

/// Langue parlée.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguageEntry {
    pub language: String,
    pub level: LanguageLevel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certification: Option<String>,
}

/// Section professionnelle complète (CV structuré).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfessionalSection {
    /// Titre professionnel actuel (ex. "Développeur Full-Stack Senior").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Résumé / accroche CV.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub experiences: Vec<ExperienceEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub education: Vec<EducationEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<SkillEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certifications: Vec<CertificationEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub languages: Vec<LanguageEntry>,
    /// Lien portfolio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portfolio_url: Option<String>,
    /// Lien LinkedIn.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linkedin_url: Option<String>,
    /// Disponibilité (immédiate, 1 mois, 3 mois…).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability: Option<String>,
    /// Prétentions salariales.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub salary_expectation: Option<String>,
    /// Mobilité géographique.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mobility: Vec<String>,
    /// Permis de conduire (catégories : B, A2, etc.).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub driving_licenses: Vec<String>,
}
