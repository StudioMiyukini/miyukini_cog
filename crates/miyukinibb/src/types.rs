//! Types du service MiyukiniBB.
//!
//! Contrat d’échange avec l’API auth forum sur Origin.

use serde::{Deserialize, Serialize};

/// Données requises pour synchroniser un profil Central vers le forum (Origin).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumProfileSync {
    /// ID du profil Central (UUID).
    pub central_id: String,
    /// Email (identifiant de connexion forum).
    pub email: String,
    /// Hash SHA256 hex du mot de passe (identique à Central).
    pub password_hash: String,
    /// Pseudonyme affiché (optionnel).
    pub pseudonyme: Option<String>,
}

impl ForumProfileSync {
    /// Construit un sync à partir des champs bruts (hash déjà calculé).
    pub fn new(
        central_id: impl Into<String>,
        email: impl Into<String>,
        password_hash: impl Into<String>,
        pseudonyme: Option<impl Into<String>>,
    ) -> Self {
        Self {
            central_id: central_id.into(),
            email: email.into(),
            password_hash: password_hash.into(),
            pseudonyme: pseudonyme.map(Into::into),
        }
    }
}
