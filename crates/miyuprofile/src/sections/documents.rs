//! Section documents officiels — CNI, passeport, permis, titres, etc.
//!
//! Métadonnées des pièces d'identité et documents administratifs.
//! Les scans/photos sont stockés dans le Vault (référence `vault_ref`).

use serde::{Deserialize, Serialize};

/// Type de document officiel.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DocumentType {
    /// Carte nationale d'identité.
    Cni,
    /// Passeport.
    Passport,
    /// Permis de conduire.
    DrivingLicense,
    /// Titre de séjour.
    ResidencePermit,
    /// Carte vitale.
    CarteVitale,
    /// Livret de famille.
    LivretFamille,
    /// Acte de naissance.
    BirthCertificate,
    /// Acte de mariage.
    MarriageCertificate,
    /// Justificatif de domicile.
    ProofOfAddress,
    /// Carte d'électeur.
    VoterCard,
    /// Carte professionnelle.
    ProfessionalCard,
    /// Carte étudiant.
    StudentCard,
    /// RIB (relevé d'identité bancaire).
    Rib,
    /// Attestation de droits (CPAM, etc.).
    RightsAttestation,
    /// Diplôme.
    Diploma,
    /// Certificat de travail.
    WorkCertificate,
    /// Autre.
    Other(String),
}

/// Métadonnées d'un document officiel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentEntry {
    /// Identifiant unique local.
    pub id: String,
    pub doc_type: DocumentType,
    /// Nom descriptif (ex. "CNI Jean Dupont").
    pub label: String,
    /// Numéro du document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// Autorité de délivrance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuing_authority: Option<String>,
    /// Pays de délivrance (ISO 3166-1 alpha-2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuing_country: Option<String>,
    /// Date de délivrance (ISO 8601).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,
    /// Date d'expiration (ISO 8601).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    /// Référence vers le scan dans le Vault.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vault_refs: Vec<String>,
    /// Tags pour recherche rapide.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Section documents complète.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentsSection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub documents: Vec<DocumentEntry>,
}
