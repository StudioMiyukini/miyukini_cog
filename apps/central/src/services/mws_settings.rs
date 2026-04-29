//! Configuration persistée du Webway dans Central.

use miyukini_central::CentralMwsConfig;
use reqwest::Url;
use std::path::PathBuf;

const APP_DIR: &str = "Miyukini-COG";
const CONFIG_FILE: &str = "mws-config.json";
const DEFAULT_HOME_BIND: &str = "0.0.0.0:8090";
const DEFAULT_PUBLIC_ADDRESS: &str = "0.0.0.0:8090";

pub fn load_mws_config() -> CentralMwsConfig {
    let path = mws_config_path();
    let raw = match std::fs::read_to_string(&path) {
        Ok(raw) => raw,
        Err(_) => return default_mws_config(),
    };

    match serde_json::from_str::<CentralMwsConfig>(&raw) {
        Ok(mut config) => {
            normalize_config(&mut config);
            config
        }
        Err(err) => {
            tracing::warn!("Config MWS invalide ({}): {}", path.display(), err);
            default_mws_config()
        }
    }
}

pub fn save_mws_config(config: &CentralMwsConfig) -> Result<(), String> {
    let path = mws_config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|err| format!("Création dossier config MWS: {err}"))?;
    }

    let json = serde_json::to_string_pretty(config)
        .map_err(|err| format!("Sérialisation config MWS: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("Écriture config MWS: {err}"))?;
    Ok(())
}

pub fn default_mws_config() -> CentralMwsConfig {
    let mut config = CentralMwsConfig::default();
    normalize_config(&mut config);
    config
}

pub fn apply_origin_url(config: &mut CentralMwsConfig, origin_url: &str) {
    if let Some(host) = origin_host(origin_url) {
        config.relay_address = format!("{host}:7000");
        config.tracker_address = format!("{host}:21000");
        let _ = origin_url;
    }
}

pub fn origin_url_from_env() -> Option<String> {
    std::env::var("MIYUKINI_ORIGIN_URL")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn normalize_config(config: &mut CentralMwsConfig) {
    if let Some(origin_url) = origin_url_from_env() {
        let uses_default_host = config.relay_address == "miyukini.com:7000"
            && config.tracker_address == "miyukini.com:21000";
        let missing_host =
            config.relay_address.trim().is_empty() || config.tracker_address.trim().is_empty();
        if uses_default_host || missing_host {
            apply_origin_url(config, &origin_url);
        }
    }

    if config.home_http_bind.is_none() {
        config.home_http_bind = Some(DEFAULT_HOME_BIND.to_string());
    }

    if config.public_address.trim().is_empty() || config.public_address == "0.0.0.0:0" {
        config.public_address = DEFAULT_PUBLIC_ADDRESS.to_string();
    }
}

fn mws_config_path() -> PathBuf {
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        return PathBuf::from(local).join(APP_DIR).join(CONFIG_FILE);
    }

    if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(config_home).join(APP_DIR).join(CONFIG_FILE);
    }

    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home)
            .join(".config")
            .join(APP_DIR)
            .join(CONFIG_FILE);
    }

    PathBuf::from(".").join(CONFIG_FILE)
}

fn origin_host(origin_url: &str) -> Option<String> {
    let parsed = Url::parse(origin_url).ok()?;
    parsed.host_str().map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_origin_url_updates_relay_and_tracker() {
        let mut config = CentralMwsConfig::default();
        apply_origin_url(&mut config, "https://origin.example.net:8443");
        assert_eq!(config.relay_address, "origin.example.net:7000");
        assert_eq!(config.tracker_address, "origin.example.net:21000");
    }

    #[test]
    fn default_config_enables_central_home_defaults() {
        let config = default_mws_config();
        assert_eq!(config.home_http_bind.as_deref(), Some(DEFAULT_HOME_BIND));
        assert_eq!(config.public_address, DEFAULT_PUBLIC_ADDRESS);
        assert!(config.auto_connect);
    }
}
