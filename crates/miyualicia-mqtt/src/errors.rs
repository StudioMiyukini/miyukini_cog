//! Types d'erreur du client MQTT Alicia.

// @id: toolkit.alicia.mqtt.errors
// @role: error_types
// @layer: 6
// @human: Erreurs MQTT Alicia : connexion, publication, souscription, parsing Z2M.
// @do: define_mqtt_error_types

/// Erreurs du client MQTT Alicia.
#[derive(Debug, thiserror::Error)]
pub enum MqttError {
    /// Le client n'est pas connecte au broker.
    #[error("client MQTT non connecte : operation impossible")]
    NotConnected,

    /// Echec de connexion au broker MQTT.
    #[error("echec connexion broker MQTT {host}:{port} : {source}")]
    ConnectionFailed {
        /// Hote du broker.
        host: String,
        /// Port du broker.
        port: u16,
        /// Erreur rumqttc sous-jacente.
        #[source]
        source: rumqttc::ClientError,
    },

    /// Erreur de publication d'un message.
    #[error("echec publication topic '{topic}' : {source}")]
    PublishFailed {
        /// Topic cible.
        topic: String,
        /// Erreur rumqttc sous-jacente.
        #[source]
        source: rumqttc::ClientError,
    },

    /// Echec de souscription a un topic.
    #[error("echec souscription topic '{topic}' : {source}")]
    SubscribeFailed {
        /// Topic cible.
        topic: String,
        /// Erreur rumqttc sous-jacente.
        #[source]
        source: rumqttc::ClientError,
    },

    /// Erreur de serialisation d'un payload JSON.
    #[error("erreur serialisation payload MQTT : {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Erreur de parsing d'un message entrant.
    #[error("erreur parsing message MQTT sur topic '{topic}' : {message}")]
    ParseError {
        /// Topic source.
        topic: String,
        /// Description du probleme de parsing.
        message: String,
    },

    /// Action inconnue ou non supportee par le protocole Z2M.
    #[error("action '{action}' non supportee par Zigbee2MQTT")]
    UnsupportedAction {
        /// Action tentee.
        action: String,
    },

    /// Le canal broadcast est sature (lagging).
    #[error("canal messages MQTT sature : {0} messages perdus")]
    ChannelLagging(u64),
}
