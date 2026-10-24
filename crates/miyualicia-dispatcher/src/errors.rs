//! Erreurs typées du dispatcher.

// @id service.alicia.dispatcher.errors
// @role dispatcher_errors
// @layer 7
// @do declare_dispatch_errors
// @human Erreurs du dispatcher Alicia.

use miyualicia_memory::MemoryError;

/// Erreurs émises par le dispatcher.
#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    /// La capability `qualified_id` n'a pas été trouvée dans le registre.
    #[error("capability inconnue : '{0}'")]
    UnknownCapability(String),

    /// Les cores ont refusé l'exécution.
    #[error("refus cores : {0}")]
    CoresDenied(String),

    /// L'utilisateur a annulé pendant la confirmation.
    #[error("annulé par l'utilisateur")]
    UserCancelled,

    /// Le `ServiceClient` a retourné une erreur métier.
    #[error("échec du service '{service}.{intent}' : {message}")]
    ServiceFailed {
        /// Service ciblé.
        service: String,
        /// Intent ciblé.
        intent: String,
        /// Message d'erreur du service.
        message: String,
    },

    /// Erreur d'enregistrement de l'épisode dans la mémoire.
    #[error("audit mémoire impossible : {0}")]
    Memory(#[from] MemoryError),

    /// Erreur de transport / connexion vers le core ou le service.
    #[error("transport : {0}")]
    Transport(String),
}
