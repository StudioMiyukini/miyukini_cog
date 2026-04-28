//! Export — constitution de dossiers et extraction de données.
//!
//! Permet de constituer rapidement un dossier d'inscription,
//! une demande de souscription, ou tout autre besoin de regroupement
//! d'informations provenant de multiples sections du profil.

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;
use crate::sections::{CentralProfile, SectionName};
use serde::{Deserialize, Serialize};

/// Type de dossier prédéfini.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DossierTemplate {
    /// Inscription (identité + coordonnées + documents).
    Registration,
    /// Candidature emploi (CV + identité + coordonnées).
    JobApplication,
    /// Souscription assurance (identité + coordonnées + finance + documents).
    InsuranceSubscription,
    /// Demande de bail (identité + coordonnées + finance + professionnel).
    LeaseApplication,
    /// Demande administrative (identité + documents + coordonnées).
    AdministrativeRequest,
    /// Dossier médical (identité + santé).
    MedicalFile,
    /// Export complet RGPD.
    GdprExport,
    /// Personnalisé.
    Custom(Vec<SectionName>),
}

impl DossierTemplate {
    /// Sections nécessaires pour ce type de dossier.
    pub fn required_sections(&self) -> Vec<SectionName> {
        match self {
            Self::Registration => vec![
                SectionName::Identity,
                SectionName::Contacts,
                SectionName::Documents,
            ],
            Self::JobApplication => vec![
                SectionName::Identity,
                SectionName::Contacts,
                SectionName::Professional,
            ],
            Self::InsuranceSubscription => vec![
                SectionName::Identity,
                SectionName::Contacts,
                SectionName::Finance,
                SectionName::Documents,
            ],
            Self::LeaseApplication => vec![
                SectionName::Identity,
                SectionName::Contacts,
                SectionName::Finance,
                SectionName::Professional,
                SectionName::Documents,
            ],
            Self::AdministrativeRequest => vec![
                SectionName::Identity,
                SectionName::Contacts,
                SectionName::Documents,
            ],
            Self::MedicalFile => vec![
                SectionName::Identity,
                SectionName::Health,
                SectionName::Contacts,
            ],
            Self::GdprExport => SectionName::all().to_vec(),
            Self::Custom(sections) => sections.clone(),
        }
    }
}

/// Résultat d'un check de complétude.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletenessReport {
    pub template: String,
    /// Sections requises et leur état de remplissage.
    pub sections: Vec<SectionStatus>,
    /// Score global (0.0 — 1.0).
    pub completeness_score: f64,
    /// Champs manquants critiques.
    pub missing_critical: Vec<String>,
}

/// État d'une section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionStatus {
    pub section: SectionName,
    pub present: bool,
    pub label: String,
}

/// tool.profile.export.check_completeness — Vérifie la complétude d'un dossier.
pub fn check_completeness(
    ctx: &GovernedContext,
    profile: &CentralProfile,
    template: &DossierTemplate,
) -> Result<CompletenessReport, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }

    let required = template.required_sections();
    let mut sections = Vec::new();
    let mut present_count = 0;
    let mut missing_critical = Vec::new();

    for section_name in &required {
        let present = is_section_present(profile, section_name);
        if present {
            present_count += 1;
        } else {
            missing_critical.push(section_name.label().to_string());
        }
        sections.push(SectionStatus {
            section: *section_name,
            present,
            label: section_name.label().to_string(),
        });
    }

    let completeness_score = if required.is_empty() {
        1.0
    } else {
        present_count as f64 / required.len() as f64
    };

    Ok(CompletenessReport {
        template: format!("{template:?}"),
        sections,
        completeness_score,
        missing_critical,
    })
}

/// tool.profile.export.extract — Extrait les sections demandées en JSON.
pub fn extract_sections(
    ctx: &GovernedContext,
    profile: &CentralProfile,
    sections: &[SectionName],
) -> Result<serde_json::Value, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }

    let full_json = serde_json::to_value(profile)
        .map_err(|e| MiyuprofileError::InvalidInput(format!("serialize: {e}")))?;

    let mut extracted = serde_json::Map::new();
    extracted.insert(
        "user_id".to_string(),
        serde_json::Value::String(profile.user_id.clone()),
    );

    if let serde_json::Value::Object(map) = &full_json {
        for section_name in sections {
            let key = section_key(section_name);
            if let Some(value) = map.get(key) {
                extracted.insert(key.to_string(), value.clone());
            }
        }
    }

    Ok(serde_json::Value::Object(extracted))
}

/// tool.profile.export.build_dossier — Construit un dossier complet.
pub fn build_dossier(
    ctx: &GovernedContext,
    profile: &CentralProfile,
    template: &DossierTemplate,
) -> Result<serde_json::Value, MiyuprofileError> {
    let sections = template.required_sections();
    extract_sections(ctx, profile, &sections)
}

fn is_section_present(profile: &CentralProfile, section: &SectionName) -> bool {
    match section {
        SectionName::Identity => profile.identity.is_some(),
        SectionName::Contacts => profile.contacts.is_some(),
        SectionName::Documents => profile.documents.is_some(),
        SectionName::Health => profile.health.is_some(),
        SectionName::Professional => profile.professional.is_some(),
        SectionName::Enterprises => profile.enterprises.is_some(),
        SectionName::Contracts => profile.contracts.is_some(),
        SectionName::Finance => profile.finance.is_some(),
        SectionName::Credentials => profile.credentials.is_some(),
        SectionName::Preferences => !profile.preferences.is_empty(),
        SectionName::CustomFields => !profile.custom_fields.is_empty(),
    }
}

fn section_key(section: &SectionName) -> &'static str {
    match section {
        SectionName::Identity => "identity",
        SectionName::Contacts => "contacts",
        SectionName::Documents => "documents",
        SectionName::Health => "health",
        SectionName::Professional => "professional",
        SectionName::Enterprises => "enterprises",
        SectionName::Contracts => "contracts",
        SectionName::Finance => "finance",
        SectionName::Credentials => "credentials",
        SectionName::Preferences => "preferences",
        SectionName::CustomFields => "custom_fields",
    }
}
