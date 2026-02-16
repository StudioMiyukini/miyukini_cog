//! Erreurs du service MiyukiniBB.

use thiserror::Error;

/// Erreur du client MiyukiniBB (sync profil, appel API Origin).
#[derive(Debug, Error)]
pub enum MiyukiniBbError {
    /// URL Origin invalide ou manquante.
    #[error("URL Origin invalide ou manquante")]
    InvalidOriginUrl,

    /// Appel HTTP vers Origin échoué.
    #[error("Appel HTTP: {0}")]
    Http(#[from] ureq::Error),

    /// Réponse API non 200 (détail dans le corps).
    #[error("API Origin: {status} — {body}")]
    Api {
        /// Code HTTP retourné par Origin.
        status: u16,
        /// Corps de la réponse.
        body: String,
    },

    /// Réponse JSON invalide.
    #[error("Réponse JSON invalide: {0}")]
    Json(String),

    /// Données de profil invalides (email vide, etc.).
    #[error("Profil invalide: {0}")]
    InvalidProfile(String),
}
