//! Erreurs typées du crate `miyualicia-memory`.

// @id service.alicia.memory.errors
// @role error_types
// @layer 7
// @do declare_memory_errors
// @human Types d'erreur exposés par le store mémoire d'Alicia.

/// Erreurs renvoyées par le store mémoire.
#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    /// KindMother injoignable (connexion refusée, coupée, délai dépassé).
    /// Fail-closed : aucune lecture ni écriture de repli sur un fichier local.
    #[error("KindMother indisponible : {0}")]
    KindMotherUnavailable(String),

    /// KindMother a refusé ou fait échouer l'opération (authentification,
    /// permission, intention refusée, contrainte violée, réponse illisible).
    #[error("KindMother : {0}")]
    KindMother(String),

    /// La base KindMother de la mémoire n'est pas classée `nationale`, ou
    /// porte une étiquette locale inattendue : ouverture refusée.
    #[error("base mémoire non nationale ou étiquette invalide : {0}")]
    NotNational(String),

    /// La base KindMother de la mémoire n'est pas chiffrée : ouverture refusée.
    #[error("base mémoire KindMother non chiffrée : ouverture refusée")]
    NotEncrypted,

    /// Ancienne base SQLite présente et non importée, ou import refusé.
    /// Le fichier hérité n'est jamais modifié ni supprimé.
    #[error("ancienne base mémoire : {0}")]
    LegacyDatabase(String),

    /// Ligne lue illisible (colonne absente, type inattendu, valeur inconnue).
    #[error("donnée mémoire illisible : {0}")]
    Corrupt(String),

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

    /// Verrou interne du store empoisonné (panique d'un autre thread pendant
    /// qu'il le tenait). Fail-closed : l'opération est refusée.
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
