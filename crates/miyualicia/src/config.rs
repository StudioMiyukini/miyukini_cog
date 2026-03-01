//! Configuration de l'orchestrateur Alicia.
//!
//! Chargee depuis `alicia.toml` a la racine du repertoire de donnees.

// @id: service.alicia.config
// @role: configuration
// @layer: 7
// @human: Configuration TOML de l'orchestrateur Alicia : MQTT, API, NLU, pieces, wake word.
// @do: define_alicia_config

use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::errors::AliciaError;
use miyualicia_mqtt::MqttConfig;

/// Configuration d'une piece domotique.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomConfig {
    /// Identifiant unique de la piece (ex: "salon", "chambre-parentale").
    pub id: String,
    /// Nom lisible de la piece (ex: "Salon", "Chambre parentale").
    pub name: String,
}

/// Configuration du wake word.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeWordConfig {
    /// Mot declencheur. Par defaut : "hey alicia".
    #[serde(default = "WakeWordConfig::default_keyword")]
    pub keyword: String,
    /// Seuil de confiance minimum (0.0 - 1.0). Par defaut : 0.5.
    #[serde(default = "WakeWordConfig::default_threshold")]
    pub threshold: f32,
    /// Activer la detection vocale. Par defaut : true.
    #[serde(default = "WakeWordConfig::default_enabled")]
    pub enabled: bool,
}

impl WakeWordConfig {
    fn default_keyword() -> String {
        "hey alicia".to_string()
    }
    fn default_threshold() -> f32 {
        0.5
    }
    fn default_enabled() -> bool {
        true
    }
}

impl Default for WakeWordConfig {
    fn default() -> Self {
        Self {
            keyword: Self::default_keyword(),
            threshold: Self::default_threshold(),
            enabled: Self::default_enabled(),
        }
    }
}

/// Configuration de l'API REST.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPortConfig {
    /// Port d'ecoute de l'API REST. Par defaut : 7890.
    #[serde(default = "ApiPortConfig::default_port")]
    pub port: u16,
    /// Hote d'ecoute. Par defaut : "0.0.0.0".
    #[serde(default = "ApiPortConfig::default_host")]
    pub host: String,
}

impl ApiPortConfig {
    fn default_port() -> u16 {
        7890
    }
    fn default_host() -> String {
        "0.0.0.0".to_string()
    }
}

impl Default for ApiPortConfig {
    fn default() -> Self {
        Self {
            port: Self::default_port(),
            host: Self::default_host(),
        }
    }
}

/// Configuration complete de l'orchestrateur Alicia.
///
/// Chargee depuis `alicia.toml`. Tous les champs ont des valeurs par defaut
/// fonctionnelles pour un demarrage sans configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaConfig {
    /// Configuration MQTT. `None` desactive le transport MQTT.
    #[serde(default)]
    pub mqtt: Option<MqttConfig>,

    /// Configuration de l'API REST.
    #[serde(default)]
    pub api: ApiPortConfig,

    /// URL du bridge NLU (miou-llm-bridge). Par defaut : "http://127.0.0.1:3003".
    #[serde(default = "AliciaConfig::default_llm_bridge_url")]
    pub llm_bridge_url: String,

    /// Configuration du wake word.
    #[serde(default)]
    pub wake_word: WakeWordConfig,

    /// Liste des pieces configurees.
    #[serde(default = "AliciaConfig::default_rooms")]
    pub rooms: Vec<RoomConfig>,

    /// Chemin vers la base de donnees KindMother. Par defaut : "alicia.db".
    #[serde(default = "AliciaConfig::default_db_path")]
    pub db_path: String,
}

impl AliciaConfig {
    fn default_llm_bridge_url() -> String {
        "http://127.0.0.1:3003".to_string()
    }

    fn default_rooms() -> Vec<RoomConfig> {
        vec![
            RoomConfig {
                id: "salon".to_string(),
                name: "Salon".to_string(),
            },
            RoomConfig {
                id: "chambre-parentale".to_string(),
                name: "Chambre parentale".to_string(),
            },
            RoomConfig {
                id: "chambre-theresa".to_string(),
                name: "Chambre Theresa".to_string(),
            },
            RoomConfig {
                id: "chambre-eleanore".to_string(),
                name: "Chambre Eleanore".to_string(),
            },
        ]
    }

    fn default_db_path() -> String {
        "alicia.db".to_string()
    }

    /// Charge la configuration depuis un fichier TOML.
    pub fn from_file(path: &Path) -> Result<Self, AliciaError> {
        let content = std::fs::read_to_string(path).map_err(|e| AliciaError::ConfigError {
            message: format!("impossible de lire {}: {e}", path.display()),
        })?;
        Self::from_toml(&content)
    }

    /// Parse la configuration depuis une chaine TOML.
    pub fn from_toml(content: &str) -> Result<Self, AliciaError> {
        toml::from_str(content).map_err(|e| AliciaError::ConfigError {
            message: format!("erreur de parsing TOML : {e}"),
        })
    }
}

impl Default for AliciaConfig {
    fn default() -> Self {
        Self {
            mqtt: Some(MqttConfig::default()),
            api: ApiPortConfig::default(),
            llm_bridge_url: Self::default_llm_bridge_url(),
            wake_word: WakeWordConfig::default(),
            rooms: Self::default_rooms(),
            db_path: Self::default_db_path(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AliciaConfig::default();
        assert!(config.mqtt.is_some());
        assert_eq!(config.api.port, 7890);
        assert_eq!(config.llm_bridge_url, "http://127.0.0.1:3003");
        assert_eq!(config.wake_word.keyword, "hey alicia");
        assert_eq!(config.rooms.len(), 4);
        assert_eq!(config.db_path, "alicia.db");
    }

    #[test]
    fn test_config_from_toml_minimal() {
        let toml_str = r#"
            llm_bridge_url = "http://127.0.0.1:3003"
            db_path = "test.db"
        "#;
        let config = AliciaConfig::from_toml(toml_str).expect("parse minimal TOML");
        assert_eq!(config.db_path, "test.db");
        // mqtt est None car non specifie et Option
        assert!(config.mqtt.is_none());
        // api prend les defauts
        assert_eq!(config.api.port, 7890);
    }

    #[test]
    fn test_config_from_toml_with_mqtt() {
        let toml_str = r#"
            llm_bridge_url = "http://127.0.0.1:3003"

            [mqtt]
            broker_host = "192.168.1.50"
            broker_port = 1884

            [api]
            port = 9999

            [wake_word]
            keyword = "hey miyuki"
            threshold = 0.7

            [[rooms]]
            id = "salon"
            name = "Salon"

            [[rooms]]
            id = "cuisine"
            name = "Cuisine"
        "#;
        let config = AliciaConfig::from_toml(toml_str).expect("parse full TOML");
        let mqtt = config.mqtt.expect("mqtt present");
        assert_eq!(mqtt.broker_host, "192.168.1.50");
        assert_eq!(mqtt.broker_port, 1884);
        assert_eq!(config.api.port, 9999);
        assert_eq!(config.wake_word.keyword, "hey miyuki");
        assert_eq!(config.rooms.len(), 2);
    }

    #[test]
    fn test_config_from_toml_invalid() {
        let result = AliciaConfig::from_toml("invalid [[ toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_serde_roundtrip() {
        let config = AliciaConfig::default();
        let toml_str = toml::to_string(&config).expect("serialize to TOML");
        let back = AliciaConfig::from_toml(&toml_str).expect("parse back");
        assert_eq!(back.api.port, config.api.port);
        assert_eq!(back.rooms.len(), config.rooms.len());
    }
}
