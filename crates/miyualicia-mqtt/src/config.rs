//! Configuration du client MQTT Alicia.
//!
//! Chargee depuis la section `[mqtt]` de `alicia.toml`.

// @id: toolkit.alicia.mqtt.config
// @role: mqtt_configuration
// @layer: 6
// @human: Configuration du client MQTT : broker, keepalive, reconnexion.
// @do: define_mqtt_config

use serde::{Deserialize, Serialize};

/// Configuration du client MQTT Alicia.
///
/// Les valeurs par defaut correspondent a une installation Mosquitto locale standard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttConfig {
    /// Hote du broker MQTT. Doit etre une adresse locale (LAN ou localhost).
    /// Aucune adresse cloud n'est admise (Loi d'Autonomie LOI-1).
    #[serde(default = "MqttConfig::default_host")]
    pub broker_host: String,

    /// Port du broker MQTT. Port standard : 1883.
    #[serde(default = "MqttConfig::default_port")]
    pub broker_port: u16,

    /// Identifiant client MQTT. Doit etre unique sur le broker.
    #[serde(default = "MqttConfig::default_client_id")]
    pub client_id: String,

    /// Intervalle keepalive en secondes.
    #[serde(default = "MqttConfig::default_keepalive")]
    pub keepalive_secs: u64,

    /// Delai de reconnexion automatique en secondes.
    #[serde(default = "MqttConfig::default_reconnect_delay")]
    pub reconnect_delay_secs: u64,

    /// Taille maximale du canal interne d'evenements rumqttc.
    #[serde(default = "MqttConfig::default_channel_capacity")]
    pub channel_capacity: usize,
}

impl MqttConfig {
    fn default_host() -> String {
        "localhost".to_string()
    }
    fn default_port() -> u16 {
        1883
    }
    fn default_client_id() -> String {
        "alicia-home".to_string()
    }
    fn default_keepalive() -> u64 {
        60
    }
    fn default_reconnect_delay() -> u64 {
        5
    }
    fn default_channel_capacity() -> usize {
        256
    }
}

impl Default for MqttConfig {
    fn default() -> Self {
        Self {
            broker_host: Self::default_host(),
            broker_port: Self::default_port(),
            client_id: Self::default_client_id(),
            keepalive_secs: Self::default_keepalive(),
            reconnect_delay_secs: Self::default_reconnect_delay(),
            channel_capacity: Self::default_channel_capacity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TC-MQTT-01 : MqttConfig::default() produit des valeurs coherentes
    #[test]
    fn test_config_default_values() {
        let config = MqttConfig::default();
        assert_eq!(config.broker_host, "localhost");
        assert_eq!(config.broker_port, 1883);
        assert_eq!(config.client_id, "alicia-home");
        assert_eq!(config.keepalive_secs, 60);
        assert_eq!(config.reconnect_delay_secs, 5);
        assert_eq!(config.channel_capacity, 256);
    }

    // TC-MQTT-02 : deserialisation JSON partielle (champs optionnels)
    #[test]
    fn test_config_partial_json_deserialization() {
        let json = r#"{"broker_host": "192.168.1.50", "broker_port": 1884}"#;
        let config: MqttConfig = serde_json::from_str(json).expect("deserialize partial config");
        assert_eq!(config.broker_host, "192.168.1.50");
        assert_eq!(config.broker_port, 1884);
        // Les autres champs prennent les valeurs par defaut
        assert_eq!(config.client_id, "alicia-home");
        assert_eq!(config.keepalive_secs, 60);
    }

    #[test]
    fn test_config_serde_roundtrip() {
        let config = MqttConfig::default();
        let json = serde_json::to_string(&config).expect("serialize");
        let back: MqttConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.broker_host, config.broker_host);
        assert_eq!(back.broker_port, config.broker_port);
    }
}
