//! Erreurs MiyukiniWatch.

use thiserror::Error;

/// Erreur du service MiyukiniWatch.
#[derive(Debug, Error)]
pub enum MiyukiniWatchError {
    /// Erreur de base de données.
    #[error("MiyukiniWatch DB: {0}")]
    Db(String),

    /// Collecte désactivée.
    #[error("Collecte désactivée")]
    CollectionDisabled,

    /// Catégorie de métrique désactivée.
    #[error("Catégorie {0} désactivée")]
    CategoryDisabled(String),

    /// Limite de volumétrie atteinte.
    #[error("Limite de volumétrie atteinte")]
    VolumeLimitReached,

    /// Profil non trouvé.
    #[error("Profil non trouvé")]
    ProfileNotFound,
}
