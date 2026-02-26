//! Lancement et arrêt des processus de services.
//!
//! Chaque service est un binaire indépendant lancé en tant que processus fils.
//! Pattern inspiré de `kindmother_launcher.rs`.

use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

use super::registry::InstalledService;

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

        // Vérifier que le binaire existe
        if !service.binary_path.exists() {
            return Err(format!(
                "Binaire introuvable : {}",
                service.binary_path.display()
            ));
        }

        // Répertoire de données du service
        let data_dir = service.install_dir.join("data");
        if let Err(e) = std::fs::create_dir_all(&data_dir) {
            tracing::warn!("Impossible de créer le répertoire de données: {e}");
        }

        tracing::info!("Lancement du service '{}' depuis {:?}", service_id, service.binary_path);

        let child = Command::new(&service.binary_path)
            .env("MIYUKINI_SERVICE_ID", service_id)
            .env("MIYUKINI_DATA_DIR", data_dir.to_string_lossy().to_string())
            .env("MIYUKINI_PROFILE_ID", profile_id)
            .env("KINDMOTHER_LISTEN_ADDR", "127.0.0.1:50051")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
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
