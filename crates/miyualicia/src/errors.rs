//! Types d'erreur de l'orchestrateur Alicia.

// @id: service.alicia.errors
// @role: error_types
// @layer: 7
// @human: Erreurs de l'orchestrateur Alicia : config, device, MQTT, NLU, commande.
// @do: define_alicia_error_types

use miyualicia_devices::DeviceError;
use miyualicia_http::HttpError;
use miyualicia_mqtt::MqttError;

/// Erreurs de l'orchestrateur central Alicia.
#[derive(Debug, thiserror::Error)]
pub enum AliciaError {
    /// Erreur de configuration (fichier absent, parsing, valeur invalide).
    #[error("erreur de configuration : {message}")]
    ConfigError {
        /// Description detaillee de l'erreur.
        message: String,
    },

    /// Erreur du registre de dispositifs.
    #[error("erreur dispositif : {0}")]
    DeviceError(#[from] DeviceError),

    /// Erreur MQTT (connexion, publication, abonnement).
    #[error("erreur MQTT : {0}")]
    MqttError(#[from] MqttError),

    /// Erreur HTTP (requete, timeout, authentification).
    #[error("erreur HTTP : {0}")]
    HttpError(#[from] HttpError),

    /// Erreur NLU (bridge indisponible, timeout, reponse invalide).
    #[error("erreur NLU : {0}")]
    NluError(String),

    /// Dispositif introuvable pour une commande.
    #[error("dispositif introuvable : {device_id}")]
    DeviceNotFound {
        /// UUID du dispositif cherche.
        device_id: String,
    },

    /// Commande non supportee par le dispositif.
    #[error("commande '{action}' non supportee par le dispositif {device_id}")]
    UnsupportedCommand {
        /// UUID du dispositif.
        device_id: String,
        /// Action tentee.
        action: String,
    },

    /// Erreur interne imprevue.
    #[error("erreur interne : {0}")]
    InternalError(String),
}
