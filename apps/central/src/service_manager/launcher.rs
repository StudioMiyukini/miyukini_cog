//! Lancement et arrêt des processus de services.
//!
//! Chaque service est un binaire indépendant lancé en tant que processus fils.
//! Pattern inspiré de `kindmother_launcher.rs`.

use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

use super::registry::InstalledService;
use crate::services::miyucloud_settings::load_miyucloud_config;

/// Processus en cours d'exécution pour un service.
struct RunningService {
    child: Child,
    started_at: std::time::Instant,
}

/// Gestionnaire des processus de services en cours d'exécution.
pub struct ServiceProcesses {
    processes: Mutex<HashMap<String, RunningService>>,
}

impl ServiceProcesses {
    pub fn new() -> Self {
        Self {
            processes: Mutex::new(HashMap::new()),
        }
    }

    /// Lance un service installé.
    pub fn launch(&self, service: &InstalledService, profile_id: &str) -> Result<(), String> {
        let service_id = &service.manifest.id;

        // Vérifier si déjà en cours d'exécution
        {
            let procs = self.processes.lock().unwrap_or_else(|e| e.into_inner());
            if procs.contains_key(service_id) {
                return Err(format!("Service '{service_id}' déjà en cours d'exécution"));
            }
        }

        // En dev, préférer le binaire dans le même dossier que Central (target/debug/)
        // pour éviter de devoir copier manuellement après chaque build.
        let binary_path = dev_binary_override(&service.binary_path)
            .unwrap_or_else(|| service.binary_path.clone());

        if !binary_path.exists() {
            return Err(format!(
                "Binaire introuvable : {}",
                binary_path.display()
            ));
        }

        // Répertoire de données du service
        let data_dir = service.install_dir.join("data");
        if let Err(e) = std::fs::create_dir_all(&data_dir) {
            tracing::warn!("Impossible de créer le répertoire de données: {e}");
        }

        let service_env = build_service_env(service, profile_id, &data_dir)?;

        tracing::info!(
            "Lancement du service '{}' depuis {:?}",
            service_id,
            binary_path
        );

        let mut command = Command::new(&binary_path);
        command
            .env("MIYUKINI_SERVICE_ID", service_id)
            .env("MIYUKINI_DATA_DIR", data_dir.to_string_lossy().to_string())
            .env("MIYUKINI_PROFILE_ID", profile_id)
            .env("KINDMOTHER_LISTEN_ADDR", "127.0.0.1:50051")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit());

        for (key, value) in &service_env {
            command.env(key, value);
        }

        let child = command
            .spawn()
            .map_err(|e| format!("Échec du lancement de '{service_id}': {e}"))?;

        let pid = child.id();
        tracing::info!("Service '{}' lancé (PID: {})", service_id, pid);

        let mut procs = self.processes.lock().unwrap_or_else(|e| e.into_inner());
        procs.insert(
            service_id.to_string(),
            RunningService {
                child,
                started_at: std::time::Instant::now(),
            },
        );

        Ok(())
    }

    /// Arrête un service en cours d'exécution.
    pub fn stop(&self, service_id: &str) -> Result<(), String> {
        let mut procs = self.processes.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(mut running) = procs.remove(service_id) {
            tracing::info!(
                "Arrêt du service '{}' (durée: {:?})",
                service_id,
                running.started_at.elapsed()
            );
            let _ = running.child.kill();
            let _ = running.child.wait();
            Ok(())
        } else {
            Err(format!("Service '{service_id}' non en cours d'exécution"))
        }
    }

    /// Arrête tous les services en cours d'exécution.
    pub fn stop_all(&self) {
        let mut procs = self.processes.lock().unwrap_or_else(|e| e.into_inner());
        let ids: Vec<String> = procs.keys().cloned().collect();
        for id in &ids {
            if let Some(mut running) = procs.remove(id) {
                tracing::info!("Arrêt du service '{}'", id);
                let _ = running.child.kill();
                let _ = running.child.wait();
            }
        }
        tracing::info!("Tous les services arrêtés ({} total)", ids.len());
    }

    /// Vérifie si un service est en cours d'exécution.
    pub fn is_running(&self, service_id: &str) -> bool {
        let procs = self.processes.lock().unwrap_or_else(|e| e.into_inner());
        procs.contains_key(service_id)
    }

    /// Liste des IDs de services en cours d'exécution.
    pub fn running_service_ids(&self) -> Vec<String> {
        let procs = self.processes.lock().unwrap_or_else(|e| e.into_inner());
        procs.keys().cloned().collect()
    }
}

fn build_service_env(
    service: &InstalledService,
    profile_id: &str,
    data_dir: &std::path::Path,
) -> Result<HashMap<String, String>, String> {
    let mut env = HashMap::new();

    if service.manifest.id == "miyucloud" {
        let config = load_miyucloud_config();
        let current_dir =
            std::env::current_dir().map_err(|e| format!("Lecture du répertoire courant: {e}"))?;
        env.insert("MIYUCLOUD_PORT".to_string(), config.api_port.to_string());
        env.insert("MIYUCLOUD_WEB_PORT".to_string(), config.web_port.to_string());
        env.insert(
            "MIYUCLOUD_WEB_ENABLED".to_string(),
            if config.web_enabled { "1" } else { "0" }.to_string(),
        );
        env.insert("MIYUCLOUD_STORAGE_PATH".to_string(), config.storage_path);
        env.insert(
            "MIYUCLOUD_DB_PATH".to_string(),
            data_dir.join("miyucloud.db").to_string_lossy().to_string(),
        );
        env.insert("MIYUCLOUD_COG_TOKEN".to_string(), config.cog_token);
        env.insert(
            "MIYUCLOUD_PASSPHRASE".to_string(),
            config.encryption_passphrase,
        );
        env.insert("MIYUCLOUD_OWNER_ID".to_string(), profile_id.to_string());
        env.insert(
            "MIYUCLOUD_DEFAULT_QUOTA_BYTES".to_string(),
            config.quota_bytes.to_string(),
        );
        env.insert(
            "MIYUCLOUD_PUBLIC_SHARES_ENABLED".to_string(),
            if config.share_policy.public_links_enabled {
                "1"
            } else {
                "0"
            }
            .to_string(),
        );
        env.insert(
            "MIYUCLOUD_INTERNAL_SHARING_ENABLED".to_string(),
            if config.share_policy.internal_sharing_enabled {
                "1"
            } else {
                "0"
            }
            .to_string(),
        );
        env.insert(
            "MIYUCLOUD_DEFAULT_SHARE_PERMISSION".to_string(),
            config.share_policy.default_permission.as_str().to_string(),
        );
        env.insert(
            "MIYUCLOUD_SSH_ENABLED".to_string(),
            if config.ssh.enabled { "1" } else { "0" }.to_string(),
        );
        env.insert("MIYUCLOUD_SSH_HOST".to_string(), config.ssh.host);
        env.insert("MIYUCLOUD_SSH_PORT".to_string(), config.ssh.port.to_string());
        env.insert("MIYUCLOUD_SSH_USERNAME".to_string(), config.ssh.username);
        env.insert("MIYUCLOUD_SSH_ROOT_PATH".to_string(), config.ssh.root_path);
        env.insert(
            "MIYUCLOUD_SSH_PRIVATE_KEY_PATH".to_string(),
            config.ssh.private_key_path,
        );
        env.insert(
            "MIYUCLOUD_SSH_KEEPALIVE".to_string(),
            if config.ssh.keepalive { "1" } else { "0" }.to_string(),
        );
        env.insert(
            "MIYUCLOUD_CENTRAL_DB_PATH".to_string(),
            current_dir.join("central.db").to_string_lossy().to_string(),
        );
    }

    Ok(env)
}

/// En développement, si le binaire du service existe à côté de l'exécutable Central
/// (ex. `target/debug/`), on le préfère au binaire de l'install dir.
fn dev_binary_override(registered_path: &std::path::Path) -> Option<std::path::PathBuf> {
    let bin_name = registered_path.file_name()?;
    let central_dir = std::env::current_exe().ok()?.parent()?.to_path_buf();
    let dev_path = central_dir.join(bin_name);
    if dev_path.exists() && dev_path != registered_path {
        tracing::debug!(
            "Dev override: {} → {:?}",
            registered_path.display(),
            dev_path
        );
        Some(dev_path)
    } else {
        None
    }
}
