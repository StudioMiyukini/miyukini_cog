//! Erreurs typées du crate `miyualicia-memory`.

// @id service.alicia.memory.errors
// @role error_types
// @layer 7
// @do declare_memory_errors
// @human Types d'erreur exposés par le store mémoire d'Alicia.

use std::path::PathBuf;

/// Erreurs renvoyées par le store mémoire.
#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    /// Erreur SQLite (création de table, requête, contrainte...).
    #[error("erreur SQLite : {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// Erreur d'I/O (création du dossier parent, ouverture du fichier...).
    #[error("erreur I/O sur {path} : {source}")]
    Io {
        /// Chemin concerné par l'erreur.
        path: PathBuf,
        /// Erreur d'origine.
        #[source]
        source: std::io::Error,
    },

    /// Sérialisation / désérialisation JSON (params, contexte...).
    #[error("erreur JSON : {0}")]
    Json(#[from] serde_json::Error),

    /// Identifiant invalide (UUID malformé, slug vide...).
    #[error("identifiant invalide : {0}")]
    InvalidId(String),

    /// Persona inconnue dans le store.
    #[error("persona inconnue : {0}")]
    UnknownPersona(String),

    /// Préférence introuvable pour la clé donnée.
    #[error("préférence introuvable : {scope}.{key}")]
    PreferenceNotFound {
        /// Scope de la préférence (persona, service, "global").
        scope: String,
        /// Clé.
        key: String,
    },

    /// Skill apprise introuvable.
    #[error("skill apprise introuvable : {0}")]
    LearnedSkillNotFound(uuid::Uuid),

    /// Confirmation en attente introuvable.
    #[error("confirmation en attente introuvable : {0}")]
    PendingConfirmationNotFound(uuid::Uuid),

    /// Verrou interne de la connexion SQLite empoisonné (panique d'un autre
    /// thread pendant qu'il le tenait). Fail-closed : l'opération est refusée.
    #[error("verrou du store mémoire empoisonné")]
    LockPoisoned,

    /// Confirmation déjà résolue (confirmée, annulée, en cours d'exécution
    /// ou échouée) — ne peut plus être réclamée ni modifiée.
    #[error("confirmation {id} déjà résolue (statut {status})")]
    PendingConfirmationAlreadyResolved {
        /// Identifiant.
        id: uuid::Uuid,
        /// Statut courant (sérialisé).
        status: String,
    },

    /// Fait oublié à la demande : il ne peut pas être réécrit sous le même
    /// identifiant (la pierre tombale fait foi).
    #[error("fait oublié (pierre tombale) : {0}")]
    FactForgotten(uuid::Uuid),

    /// Demande d'oubli invalide (auteur ou motif manquant, trop long).
    #[error("demande d'oubli invalide : {0}")]
    InvalidForgetRequest(String),
}
