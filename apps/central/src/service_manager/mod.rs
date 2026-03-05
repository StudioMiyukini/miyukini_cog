//! Gestion du cycle de vie des services — installation, lancement, arrêt, mise à jour.
//!
//! Chaque service est un binaire indépendant. Central gère :
//! - Le registre des services installés (registry.json)
//! - Le lancement/arrêt des processus service
//! - L'installation/désinstallation depuis le Market

pub mod installer;
pub mod launcher;
pub mod registry;

use launcher::ServiceProcesses;
use registry::{InstalledService, ServiceRegistryData};
use std::sync::{Arc, Mutex};

/// Service Manager — point d'entrée unique pour le cycle de vie des services.
#[derive(Clone)]
pub struct ServiceManager {
    /// Registre des services installés (persisté sur disque).
    pub registry: Arc<Mutex<ServiceRegistryData>>,
    /// Processus en cours d'exécution.
    pub processes: Arc<ServiceProcesses>,
}

impl ServiceManager {
    /// Initialise le Service Manager : charge le registre depuis le disque,
    /// puis scanne les dossiers pour détecter des services installés manuellement.
    pub fn init() -> Self {
        let mut registry_data = registry::load_registry();

        // Scanner les dossiers pour détecter des services non enregistrés
        let count_before = registry_data.services.len();
        installer::scan_installed_services(&mut registry_data);
        let count_after = registry_data.services.len();
        if count_after > count_before {
            tracing::info!(
                "Scan: {} nouveau(x) service(s) détecté(s) ({} total)",
                count_after - count_before,
                count_after
            );
            // Persister les découvertes
            if let Err(e) = registry::save_registry(&registry_data) {
                tracing::warn!("Sauvegarde du registre après scan échouée: {e}");
            }
        }

        Self {
            registry: Arc::new(Mutex::new(registry_data)),
            processes: Arc::new(ServiceProcesses::new()),
        }
    }

    /// Liste les services installés.
    pub fn installed_services(&self) -> Vec<InstalledService> {
        let reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
        reg.services.values().cloned().collect()
    }

    /// Vérifie si un service est installé.
    pub fn is_installed(&self, service_id: &str) -> bool {
        let reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
        reg.services.contains_key(service_id)
    }

    /// Vérifie si un service est en cours d'exécution.
    pub fn is_running(&self, service_id: &str) -> bool {
        self.processes.is_running(service_id)
    }

    /// Lance un service installé. Retourne Ok si lancé, Err si échec.
    pub fn launch_service(&self, service_id: &str, profile_id: &str) -> Result<(), String> {
        let reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
        let service = reg
            .services
            .get(service_id)
            .ok_or_else(|| format!("Service '{service_id}' non installé"))?;
        self.processes.launch(service, profile_id)
    }

    /// Arrête un service en cours d'exécution.
    pub fn stop_service(&self, service_id: &str) -> Result<(), String> {
        self.processes.stop(service_id)
    }

    /// Arrête tous les services (appelé à la fermeture de Central).
    pub fn stop_all(&self) {
        self.processes.stop_all();
    }

    /// Installe un service depuis des bytes de package (.msp).
    pub fn install_from_package(&self, package_bytes: &[u8]) -> Result<String, String> {
        let mut reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
        let service_id = installer::install_package(package_bytes, &mut reg)?;
        registry::save_registry(&reg)?;
        Ok(service_id)
    }

    /// Vérifie les mises à jour disponibles.
    /// Compare les versions installées avec celles du catalogue distant.
    /// Retourne les paires (service_id, version_installée, version_disponible).
    pub fn check_updates(
        &self,
        catalog_versions: &[(String, String)],
    ) -> Vec<(String, String, String)> {
        let reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
        let mut updates = Vec::new();
        for (service_id, available_version) in catalog_versions {
            if let Some(installed) = reg.services.get(service_id) {
                if installed.manifest.version != *available_version
                    && *available_version > installed.manifest.version
                {
                    updates.push((
                        service_id.clone(),
                        installed.manifest.version.clone(),
                        available_version.clone(),
                    ));
                }
            }
        }
        updates
    }

    /// Met à jour un service (même flow que l'installation : stop + réinstall).
    pub fn update_from_package(
        &self,
        service_id: &str,
        package_bytes: &[u8],
    ) -> Result<String, String> {
        // Arrêter le service s'il tourne
        if self.is_running(service_id) {
            self.stop_service(service_id)?;
        }
        // Réinstaller (install_package écrase l'entrée existante)
        self.install_from_package(package_bytes)
    }

    /// Désinstalle un service (arrête le processus, supprime les fichiers).
    pub fn uninstall_service(&self, service_id: &str) -> Result<(), String> {
        // Arrêter si en cours d'exécution
        if self.is_running(service_id) {
            self.stop_service(service_id)?;
        }
        let mut reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
        installer::uninstall_service(service_id, &mut reg)?;
        registry::save_registry(&reg)?;
        Ok(())
    }
}
