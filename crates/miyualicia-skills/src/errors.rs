//! Types d'erreur pour `miyualicia-skills`.

// @id service.alicia.skills.errors
// @role error_types
// @layer 7
// @do model_skill_errors
// @human Erreurs — parsing manifeste, validation capacité, IO.

use std::path::PathBuf;

/// Erreurs émises par le registre de skills.
#[derive(Debug, thiserror::Error)]
pub enum SkillsError {
    /// Erreur de parsing JSON d'un manifeste.
    #[error("manifeste JSON invalide ({path}): {source}")]
    Json {
        /// Chemin du manifeste fautif.
        path: PathBuf,
        /// Erreur serde_json sous-jacente.
        #[source]
        source: serde_json::Error,
    },

    /// Erreur de lecture de fichier.
    #[error("lecture manifeste impossible ({path}): {source}")]
    Io {
        /// Chemin du fichier.
        path: PathBuf,
        /// Erreur IO sous-jacente.
        #[source]
        source: std::io::Error,
    },

    /// Champ requis manquant dans une capacité.
    #[error("capacité '{capability}' du service '{service}' : champ requis manquant '{field}'")]
    MissingField {
        /// Identifiant du service.
        service: String,
        /// Nom de l'intention.
        capability: String,
        /// Nom du champ manquant.
        field: String,
    },

    /// Identifiant invalide (caractères interdits ou trop long).
    #[error("identifiant invalide '{0}' (alphanumérique, _ ou - uniquement, 1..=64 caractères)")]
    InvalidId(String),

    /// Type de paramètre inconnu.
    #[error("type de paramètre inconnu : '{0}'")]
    UnknownParamType(String),

    /// Doublon dans le registre.
    #[error("capacité déjà enregistrée : service='{service}', intent='{intent}'")]
    DuplicateSkill {
        /// Identifiant du service.
        service: String,
        /// Identifiant de l'intent.
        intent: String,
    },

    /// Arguments d'un tool call non parseables en JSON.
    #[error("arguments tool call invalides : {0}")]
    InvalidArguments(#[from] serde_json::Error),
}
