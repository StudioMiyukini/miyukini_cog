//! @id     central_service_manager_mod
//! @do     declarer_module_installer
//! @role   orchestration
//! @layer  app
//! @human  central — module « service manager mod ».

//! Gestion du cycle de vie des services — installation, lancement, arrêt, mise à jour.
//!
//! Chaque service est un binaire indépendant. Central gère :
//! - Le registre des services installés (registry.json)
//! - Le lancement/arrêt des processus service
//! - L'installation/désinstallation depuis le Market

pub mod confiance;
pub mod gouvernance;
pub mod installer;
pub mod launcher;
pub mod registry;

use gouvernance::{ActionMarket, GouvernanceMarket, OperationMarket};
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
    /// Porte StrongFather du domaine Market (K-11) : absente ⇒ tout refuser.
    pub gouvernance: GouvernanceMarket,
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
            gouvernance: GouvernanceMarket::charger(),
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
        self.processes.launch(service, profile_id, &self.gouvernance)
    }

    /// Arrête un service en cours d'exécution.
    pub fn stop_service(&self, service_id: &str) -> Result<(), String> {
        self.processes.stop(service_id)
    }

    /// Arrête tous les services (appelé à la fermeture de Central).
    pub fn stop_all(&self) {
        self.processes.stop_all();
    }

    /// Installe un service depuis des bytes de package (.msp signé).
    /// `expected_id` / `expected_version` : service et version demandés ; la
    /// signature d'un éditeur épinglé est vérifiée AVANT toute écriture (CA-01),
    /// puis StrongFather décide (K-11).
    pub fn install_from_package(
        &self,
        expected_id: &str,
        expected_version: &str,
        package_bytes: &[u8],
    ) -> Result<String, String> {
        self.installer_gouverne(
            expected_id,
            expected_version,
            package_bytes,
            OperationMarket::Installer,
        )
    }

    /// Installation (ou réinstallation de mise à jour) soumise à la décision `operation`.
    fn installer_gouverne(
        &self,
        expected_id: &str,
        expected_version: &str,
        package_bytes: &[u8],
        operation: OperationMarket,
    ) -> Result<String, String> {
        let ancres = confiance::ancres_market();
        let contexte = installer::ContexteVerification {
            ancres: &ancres,
            maintenant_secs: confiance::maintenant_secs(),
            dev_non_signe: confiance::dev_non_signe_autorise(),
        };
        let mut reg = self
            .registry
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let service_id = installer::verifier_puis_installer(
            package_bytes,
            expected_id,
            Some(expected_version),
            contexte,
            &self.gouvernance,
            operation,
            &mut reg,
            &registry::services_root_dir(),
        )?;
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
                if is_newer_version(available_version, &installed.manifest.version) {
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
    ///
    /// Signature vérifiée puis décision StrongFather `mettre_a_jour` AVANT d'arrêter
    /// quoi que ce soit ; la réinstallation repasse par la vérification et la porte.
    pub fn update_from_package(
        &self,
        service_id: &str,
        version: &str,
        package_bytes: &[u8],
    ) -> Result<String, String> {
        let paquet = installer::verifier_paquet_market(
            package_bytes,
            service_id,
            Some(version),
            &confiance::ancres_market(),
            confiance::maintenant_secs(),
            confiance::dev_non_signe_autorise(),
        )?;
        let action = paquet.action_market(OperationMarket::MettreAJour, service_id, Some(version));
        self.gouvernance.executer_action_market(&action, |_jeton| {
            // Arrêter le service s'il tourne
            if self.is_running(service_id) {
                self.stop_service(service_id)?;
            }
            Ok::<(), String>(())
        })??;
        // Réinstaller (vérifie de nouveau, redemande la décision, écrase l'entrée)
        self.installer_gouverne(service_id, version, package_bytes, OperationMarket::MettreAJour)
    }

    /// Désinstalle un service (arrête le processus, supprime les fichiers).
    /// Décision StrongFather `desinstaller` avant tout arrêt ou suppression.
    pub fn uninstall_service(&self, service_id: &str) -> Result<(), String> {
        let action = self.action_service_installe(OperationMarket::Desinstaller, service_id);
        self.gouvernance.executer_action_market(&action, |_jeton| {
            // Arrêter si en cours d'exécution
            if self.is_running(service_id) {
                self.stop_service(service_id)?;
            }
            let mut reg = self.registry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            installer::uninstall_service(service_id, &mut reg)?;
            registry::save_registry(&reg)?;
            Ok(())
        })?
    }

    /// Action décrivant un service installé ; provenance re-vérifiée sans dérogation
    /// (sceau absent ou invalide ⇒ `non_signee`).
    fn action_service_installe(&self, operation: OperationMarket, service_id: &str) -> ActionMarket {
        let reg = self.registry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match reg.services.get(service_id) {
            Some(service) => match launcher::controler_provenance_signee(
                service,
                &confiance::ancres_market(),
                confiance::maintenant_secs(),
                false,
            ) {
                Ok(controle) => launcher::action_pour(operation, service, &controle),
                Err(_) => ActionMarket::nouvelle(operation, service_id, &service.manifest.version),
            },
            None => ActionMarket::nouvelle(operation, service_id, ""),
        }
    }
}

/// Parse « X.Y.Z[-pre][+build] » en semver ; tolère un préfixe `v` et les
/// versions à 1 ou 2 composantes (« 1.2 » -> 1.2.0).
fn parse_version(v: &str) -> Option<semver::Version> {
    let v = v.trim().trim_start_matches('v');
    if let Ok(parsed) = semver::Version::parse(v) {
        return Some(parsed);
    }
    // Complète les composantes manquantes (avant tout -pre / +build).
    let split = v.find(['-', '+']).unwrap_or(v.len());
    let (core, rest) = v.split_at(split);
    let n = core.split('.').count();
    let padded = match n {
        1 => format!("{core}.0.0{rest}"),
        2 => format!("{core}.0{rest}"),
        _ => return None,
    };
    semver::Version::parse(&padded).ok()
}

/// Version entière du canal officiel (lot 4) d'une version semver de service :
/// `majeure × 10¹² + mineure × 10⁶ + correctif` (mineure et correctif < 10⁶).
///
/// Convention de Central, non spécifiée par la doc gelée (à amender) : l'ordre
/// est celui de semver pour les versions publiées ; une pré-version ou une
/// métadonnée de build n'a **pas** de version de canal (`None`).
pub fn version_canal(v: &str) -> Option<u64> {
    let p = parse_version(v)?;
    if !p.pre.is_empty() || !p.build.is_empty() || p.minor >= 1_000_000 || p.patch >= 1_000_000 {
        return None;
    }
    p.major.checked_mul(1_000_000_000_000)?.checked_add(p.minor * 1_000_000 + p.patch)
}

/// Vrai si `available` est strictement plus récente que `installed` (semver).
/// Fail-closed : une version illisible ne déclenche jamais de mise à jour.
pub fn is_newer_version(available: &str, installed: &str) -> bool {
    match (parse_version(available), parse_version(installed)) {
        (Some(a), Some(i)) => a > i,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{is_newer_version, version_canal};

    #[test]
    fn version_canal_suit_semver() {
        assert_eq!(version_canal("2.0.0"), Some(2_000_000_000_000));
        assert_eq!(version_canal("v0.10.3"), Some(10_000_003));
        assert!(version_canal("0.10.0") > version_canal("0.9.999"));
        assert_eq!(version_canal("1.0.0-beta.1"), None);
        assert_eq!(version_canal("1.0.0+build"), None);
        assert_eq!(version_canal("1.1000000.0"), None);
        assert_eq!(version_canal("n'importe quoi"), None);
    }

    #[test]
    fn semver_0_10_plus_recent_que_0_9() {
        assert!(is_newer_version("0.10.0", "0.9.0"));
        assert!(!is_newer_version("0.9.0", "0.10.0"));
        assert!(is_newer_version("0.10", "0.9"));
        assert!(!is_newer_version("1.0.0", "1.0.0"));
        assert!(!is_newer_version("n'importe quoi", "1.0.0"));
        assert!(is_newer_version("1.0.0", "1.0.0-beta.1"));
    }
}
