//! Section identifiants — mots de passe, comptes en ligne, codes PIN.
//!
//! Les mots de passe sont stockés chiffrés (jamais en clair).
//! Le chiffrement/déchiffrement est délégué à BorderGuard.
//! Le toolkit stocke uniquement la forme chiffrée (`encrypted_password`).

use serde::{Deserialize, Serialize};

/// Catégorie d'identifiant.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CredentialCategory {
    #[default]
    Website,
    App,
    Email,
    Banking,
    SocialMedia,
    Government,
    Work,
    Wifi,
    Pin,
    ApiKey,
    Other,
}

/// Entrée identifiant / mot de passe.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CredentialEntry {
    pub id: String,
    /// Nom du service (ex. "Gmail", "Impots.gouv", "Netflix").
    pub service: String,
    pub category: CredentialCategory,
    /// URL de connexion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Identifiant / login.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Email associé.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Mot de passe chiffré (opaque pour le toolkit ; déchiffrement = BorderGuard).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted_password: Option<String>,
    /// Secret OTP / 2FA (chiffré).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted_otp_secret: Option<String>,
    /// Codes de récupération (chiffrés).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub encrypted_recovery_codes: Vec<String>,
    /// Question secrète.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_question: Option<String>,
    /// Réponse chiffrée à la question secrète.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted_security_answer: Option<String>,
    /// Date de dernière modification du mot de passe (ISO 8601).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password_changed_at: Option<String>,
    /// Tags pour recherche rapide.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Section identifiants complète.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CredentialsSection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub credentials: Vec<CredentialEntry>,
}
