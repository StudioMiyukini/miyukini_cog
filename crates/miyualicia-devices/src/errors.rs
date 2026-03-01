//! Types d'erreur du registre de dispositifs.

// @id: toolkit.alicia.devices.errors
// @role: error_types
// @layer: 6
// @human: Erreurs du registre de dispositifs Alicia : NotFound, AlreadyExists, etc.
// @do: define_device_error_types

use uuid::Uuid;

/// Erreurs du registre de dispositifs.
#[derive(Debug, thiserror::Error)]
pub enum DeviceError {
    /// Le dispositif demande n'existe pas dans le registre.
    #[error("dispositif {0} introuvable dans le registre")]
    NotFound(Uuid),

    /// Un dispositif avec le meme UUID existe deja.
    #[error("dispositif {0} deja present dans le registre")]
    AlreadyExists(Uuid),

    /// L'action demandee n'est pas supportee par les capacites du dispositif.
    #[error("action '{action}' non supportee par le dispositif {device_id} (type: {device_type})")]
    UnsupportedAction {
        /// UUID du dispositif concerne.
        device_id: Uuid,
        /// Type du dispositif (pour le message d'erreur).
        device_type: String,
        /// Action tentee.
        action: String,
    },

    /// Valeur fournie hors de la plage valide.
    #[error("valeur hors plage pour '{field}' : {message}")]
    InvalidValue {
        /// Nom du champ concerne.
        field: String,
        /// Description de l'erreur de plage.
        message: String,
    },

    /// Erreur de serialisation/deserialisation de l'etat JSON.
    #[error("erreur de serialisation etat : {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Le verrou RwLock a ete empoisonne (panique dans un thread concurrent).
    #[error("le registre est dans un etat corrompu (lock poison) : {0}")]
    LockPoisoned(String),
}
