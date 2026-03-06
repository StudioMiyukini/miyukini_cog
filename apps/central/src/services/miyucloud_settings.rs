//! Configuration persisted for MiyuCloud inside Central.

use crate::service_manager::registry::service_install_dir;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

const APP_DIR: &str = "Miyukini-COG";
const CONFIG_FILE: &str = "miyucloud-config.json";
const DEFAULT_API_PORT: u16 = 11440;
const DEFAULT_WEB_PORT: u16 = 11442;
const DEFAULT_QUOTA_BYTES: u64 = 10 * 1024 * 1024 * 1024;
const DEFAULT_SSH_PORT: u16 = 22;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharePermissionPreset {
    #[default]
    Read,
    ReadWrite,
}

impl SharePermissionPreset {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::ReadWrite => "read_write",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Read => "Lecture seule",
            Self::ReadWrite => "Lecture et ecriture",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct MiyuCloudSharePolicyConfig {
    pub public_links_enabled: bool,
    pub internal_sharing_enabled: bool,
    pub default_permission: SharePermissionPreset,
}

impl Default for MiyuCloudSharePolicyConfig {
    fn default() -> Self {
        Self {
            public_links_enabled: true,
            internal_sharing_enabled: true,
            default_permission: SharePermissionPreset::Read,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct MiyuCloudSshConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub root_path: String,
    pub private_key_path: String,
    pub keepalive: bool,
}

impl Default for MiyuCloudSshConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            host: String::new(),
            port: DEFAULT_SSH_PORT,
            username: String::new(),
            root_path: String::new(),
            private_key_path: String::new(),
            keepalive: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct MiyuCloudServiceConfig {
    pub api_port: u16,
    pub web_port: u16,
    pub web_enabled: bool,
    pub storage_path: String,
    pub quota_bytes: u64,
    pub cog_token: String,
    pub encryption_passphrase: String,
    pub first_launch_completed: bool,
    pub share_policy: MiyuCloudSharePolicyConfig,
    pub ssh: MiyuCloudSshConfig,
}

impl Default for MiyuCloudServiceConfig {
    fn default() -> Self {
        default_miyucloud_config()
    }
}

pub fn load_miyucloud_config() -> MiyuCloudServiceConfig {
    let path = miyucloud_config_path();
    let raw = match std::fs::read_to_string(&path) {
        Ok(raw) => raw,
        Err(_) => return persist_default_config(&path),
    };

    match serde_json::from_str::<MiyuCloudServiceConfig>(&raw) {
        Ok(mut config) => {
            let original = config.clone();
            normalize_config(&mut config);
            if config != original {
                if let Err(err) = persist_config_to_path(&path, &config) {
                    tracing::warn!(
                        "Unable to persist normalized MiyuCloud config ({}): {}",
                        path.display(),
                        err
                    );
                }
            }
            config
        }
        Err(err) => {
            tracing::warn!("Invalid MiyuCloud config ({}): {}", path.display(), err);
            persist_default_config(&path)
        }
    }
}

pub fn save_miyucloud_config(config: &MiyuCloudServiceConfig) -> Result<(), String> {
    let path = miyucloud_config_path();
    let mut config = config.clone();
    normalize_config(&mut config);
    persist_config_to_path(&path, &config)
}

pub fn default_miyucloud_config() -> MiyuCloudServiceConfig {
    let mut config = MiyuCloudServiceConfig {
        api_port: DEFAULT_API_PORT,
        web_port: DEFAULT_WEB_PORT,
        web_enabled: true,
        storage_path: default_storage_path().to_string_lossy().to_string(),
        quota_bytes: DEFAULT_QUOTA_BYTES,
        cog_token: random_secret(),
        encryption_passphrase: random_secret(),
        first_launch_completed: false,
        share_policy: MiyuCloudSharePolicyConfig::default(),
        ssh: MiyuCloudSshConfig::default(),
    };
    normalize_config(&mut config);
    config
}

fn persist_default_config(path: &Path) -> MiyuCloudServiceConfig {
    let config = default_miyucloud_config();
    if let Err(err) = persist_config_to_path(path, &config) {
        tracing::warn!(
            "Unable to persist default MiyuCloud config ({}): {}",
            path.display(),
            err
        );
    }
    config
}

fn persist_config_to_path(path: &Path, config: &MiyuCloudServiceConfig) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|err| format!("Create MiyuCloud config directory: {err}"))?;
    }

    let json = serde_json::to_string_pretty(config)
        .map_err(|err| format!("Serialize MiyuCloud config: {err}"))?;
    std::fs::write(path, json).map_err(|err| format!("Write MiyuCloud config: {err}"))?;
    Ok(())
}

fn normalize_config(config: &mut MiyuCloudServiceConfig) {
    if config.api_port == 0 {
        config.api_port = DEFAULT_API_PORT;
    }
    if config.web_port == 0 {
        config.web_port = DEFAULT_WEB_PORT;
    }
    if config.storage_path.trim().is_empty() {
        config.storage_path = default_storage_path().to_string_lossy().to_string();
    }
    if config.quota_bytes == 0 {
        config.quota_bytes = DEFAULT_QUOTA_BYTES;
    }
    if config.cog_token.trim().is_empty() {
        config.cog_token = random_secret();
    }
    if config.encryption_passphrase.trim().is_empty() {
        config.encryption_passphrase = random_secret();
    }
    if config.ssh.port == 0 {
        config.ssh.port = DEFAULT_SSH_PORT;
    }
}

fn miyucloud_config_path() -> PathBuf {
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

fn default_storage_path() -> PathBuf {
    service_install_dir("miyucloud").join("data").join("storage")
}

fn random_secret() -> String {
    format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn test_env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn default_config_has_expected_defaults() {
        let config = default_miyucloud_config();
        assert_eq!(config.api_port, DEFAULT_API_PORT);
        assert_eq!(config.web_port, DEFAULT_WEB_PORT);
        assert!(config.web_enabled);
        assert_eq!(config.quota_bytes, DEFAULT_QUOTA_BYTES);
        assert!(!config.cog_token.is_empty());
        assert!(!config.encryption_passphrase.is_empty());
        assert!(!config.first_launch_completed);
        assert!(config.share_policy.public_links_enabled);
        assert!(config.share_policy.internal_sharing_enabled);
        assert_eq!(
            config.share_policy.default_permission,
            SharePermissionPreset::Read
        );
        assert!(!config.ssh.enabled);
        assert_eq!(config.ssh.port, DEFAULT_SSH_PORT);
        assert!(config.ssh.keepalive);
    }

    #[test]
    fn load_config_persists_and_reuses_default_secrets() {
        let _guard = test_env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp_root = std::env::temp_dir().join(format!("miyucloud-config-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&temp_root).expect("temp root");

        let previous_local = std::env::var_os("LOCALAPPDATA");
        std::env::set_var("LOCALAPPDATA", &temp_root);

        let first = load_miyucloud_config();
        let second = load_miyucloud_config();
        let persisted_path = temp_root.join(APP_DIR).join(CONFIG_FILE);
        let persisted_raw = std::fs::read_to_string(&persisted_path).expect("persisted config");
        let persisted: MiyuCloudServiceConfig =
            serde_json::from_str(&persisted_raw).expect("persisted config json");

        assert_eq!(first, second);
        assert_eq!(first, persisted);
        assert!(!first.cog_token.is_empty());
        assert!(!first.encryption_passphrase.is_empty());
        assert_eq!(first.share_policy.default_permission, SharePermissionPreset::Read);
        assert_eq!(first.ssh.port, DEFAULT_SSH_PORT);

        if let Some(value) = previous_local {
            std::env::set_var("LOCALAPPDATA", value);
        } else {
            std::env::remove_var("LOCALAPPDATA");
        }

        let _ = std::fs::remove_dir_all(temp_root);
    }
}
