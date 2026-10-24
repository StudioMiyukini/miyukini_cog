//! @id     central_services_mws_settings
//! @do     definir_app_dir
//! @role   orchestration
//! @layer  app
//! @human  central — module « services mws settings ».

//! Configuration persistée du Webway dans Central.
//!
//! - Exposition désactivée par défaut (EX-48, C-21) : MWS inactif, pas de
//!   connexion automatique, page Home éteinte.
//! - Page Home activée depuis les réglages : boucle locale [`HOME_BIND_LOCAL`]
//!   (le tunnel MWS suffit) sauf adresse saisie explicitement.
//! - Fichier écrit en 0600 (contient la clé de conformité MWS, CA-03).

use miyukini_central::fichiers_secrets::{durcir_fichier, ecrire_secret};
use miyukini_central::CentralMwsConfig;
use reqwest::Url;
use std::path::PathBuf;

const CONFIG_FILE: &str = "mws-config.json";
/// Adresse de la page Home lorsqu'elle est activée sans adresse explicite.
pub const HOME_BIND_LOCAL: &str = "127.0.0.1:8090";
/// Ancien défaut implicite (exposition LAN) migré vers la boucle locale.
const LEGACY_HOME_BIND: &str = "0.0.0.0:8090";
const DEFAULT_PUBLIC_ADDRESS: &str = "0.0.0.0:8090";

pub fn load_mws_config() -> CentralMwsConfig {
    let path = mws_config_path();
    // CA-03 : un fichier existant (ancienne version, 0644) est ramené en 0600.
    if let Err(err) = durcir_fichier(&path) {
        tracing::warn!("Permissions de {} non corrigées : {err}", path.display());
    }
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
    let json = serde_json::to_string_pretty(config)
        .map_err(|err| format!("Sérialisation config MWS: {err}"))?;
    ecrire_secret(&path, json.as_bytes()).map_err(|err| format!("Écriture config MWS: {err}"))?;
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

    // `None` = page Home désactivée (respecté). L'ancien défaut implicite
    // `0.0.0.0:8090` (jamais choisi par l'utilisateur) passe en boucle locale.
    match config.home_http_bind.as_deref().map(str::trim) {
        Some("") => config.home_http_bind = None,
        Some(LEGACY_HOME_BIND) => config.home_http_bind = Some(HOME_BIND_LOCAL.to_string()),
        _ => {}
    }

    if config.public_address.trim().is_empty() || config.public_address == "0.0.0.0:0" {
        config.public_address = DEFAULT_PUBLIC_ADDRESS.to_string();
    }
}

fn mws_config_path() -> PathBuf {
    match miyukini_central::chemins::dossier_config() {
        Ok(dir) => dir.join(CONFIG_FILE),
        Err(err) => {
            // Release sans dossier utilisateur : chemin inexistant (lecture → défauts
            // sûrs, écriture → erreur), jamais le répertoire courant (CA-04).
            tracing::error!("Dossier de configuration introuvable : {err}");
            PathBuf::from("/nonexistent/miyukini-cog").join(CONFIG_FILE)
        }
    }
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
    fn default_config_exposition_desactivee() {
        let config = default_mws_config();
        assert!(!config.enabled, "MWS inactif par défaut (EX-48)");
        assert!(!config.auto_connect, "pas de connexion automatique par défaut");
        assert_eq!(config.home_http_bind, None, "page Home éteinte par défaut");
        assert_eq!(config.public_address, DEFAULT_PUBLIC_ADDRESS);
    }

    #[test]
    fn ancien_bind_lan_migre_en_boucle_locale() {
        let mut config = CentralMwsConfig {
            home_http_bind: Some(LEGACY_HOME_BIND.to_string()),
            ..CentralMwsConfig::default()
        };
        normalize_config(&mut config);
        assert_eq!(config.home_http_bind.as_deref(), Some(HOME_BIND_LOCAL));
        // Une adresse explicite différente est respectée ; None reste None.
        config.home_http_bind = Some("192.168.1.10:9000".into());
        normalize_config(&mut config);
        assert_eq!(config.home_http_bind.as_deref(), Some("192.168.1.10:9000"));
        config.home_http_bind = None;
        normalize_config(&mut config);
        assert_eq!(config.home_http_bind, None);
    }
}
