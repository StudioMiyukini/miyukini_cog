//! Configuration du serveur API REST Alicia.
//!
//! Chargee depuis la section `[api]` de `alicia.toml`.

// @id: service.alicia.api.config
// @role: api_configuration
// @layer: 7
// @human: Configuration API REST Alicia : port, JWT TTL, rate limit, CORS.
// @do: define_api_config

use serde::{Deserialize, Serialize};

/// Configuration du serveur API REST Alicia.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Port d'ecoute. Defaut : 7890.
    #[serde(default = "ApiConfig::default_port")]
    pub port: u16,

    /// Hote d'ecoute. "0.0.0.0" pour le reseau LAN.
    #[serde(default = "ApiConfig::default_host")]
    pub host: String,

    /// Duree de vie des tokens JWT en secondes. Defaut : 3600 (1h).
    #[serde(default = "ApiConfig::default_token_ttl")]
    pub token_ttl_secs: u64,

    /// Nombre maximal de requetes par minute par adresse IP.
    #[serde(default = "ApiConfig::default_rate_limit")]
    pub rate_limit_per_min: u32,

    /// Activer les headers CORS.
    #[serde(default)]
    pub cors_enabled: bool,

    /// Origines CORS autorisees. Ignoree si `cors_enabled = false`.
    #[serde(default)]
    pub cors_origins: Vec<String>,
}

impl ApiConfig {
    fn default_port() -> u16 {
        7890
    }
    fn default_host() -> String {
        "0.0.0.0".to_string()
    }
    fn default_token_ttl() -> u64 {
        3600
    }
    fn default_rate_limit() -> u32 {
        100
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            port: Self::default_port(),
            host: Self::default_host(),
            token_ttl_secs: Self::default_token_ttl(),
            rate_limit_per_min: Self::default_rate_limit(),
            cors_enabled: false,
            cors_origins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_api_config() {
        let cfg = ApiConfig::default();
        assert_eq!(cfg.port, 7890);
        assert_eq!(cfg.host, "0.0.0.0");
        assert_eq!(cfg.token_ttl_secs, 3600);
        assert_eq!(cfg.rate_limit_per_min, 100);
        assert!(!cfg.cors_enabled);
        assert!(cfg.cors_origins.is_empty());
    }

    #[test]
    fn test_api_config_serde_roundtrip() {
        let cfg = ApiConfig::default();
        let json = serde_json::to_string(&cfg).expect("serialize");
        let back: ApiConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.port, cfg.port);
        assert_eq!(back.token_ttl_secs, cfg.token_ttl_secs);
    }
}
