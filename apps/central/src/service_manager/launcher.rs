//! @id     central_service_manager_launcher
//! @do     definir_running_service
//! @role   orchestration
//! @layer  app
//! @human  central — module « service manager launcher ».

//! Lancement et arrêt des processus de services.
//!
//! Chaque service est un binaire indépendant lancé en tant que processus fils.
//! Pattern inspiré de `kindmother_launcher.rs`.

use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

use super::confiance;
use super::registry::InstalledService;
use crate::services::miyucloud_settings::load_miyucloud_config;
use miyumarket::signature::{hex_decoder, verifier_sceau_installe};
use miyusceau::AncresDeConfiance;

/// Service dont le sceau conservé a été re-vérifié contre les éditeurs épinglés.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provenance {
    /// Sceau valide d'un éditeur officiel (bundle, niveau souverain).
    SigneeOfficielle,
    /// Sceau valide d'un éditeur ajouté localement (niveau reconnu) :
    /// jamais de secrets, jamais d'identifiant réservé.
    SigneeTierce,
    /// Aucun sceau valide : lancement toléré par dérogation dev, sans secrets.
    NonSigneeDev,
}

/// Contrôle de provenance avant lancement (CA-01, fail-closed).
///
/// Sceau absent ou invalide → refus, sauf dérogation dev (debug uniquement).
pub fn controler_provenance(
    service: &InstalledService,
    ancres: &AncresDeConfiance,
    maintenant_secs: u64,
    dev_non_signe: bool,
) -> Result<Provenance, String> {
    let id = &service.manifest.id;
    let verdict = match service.sceau_paquet.as_deref() {
        None => Err("aucune signature d'éditeur".to_string()),
        Some(hex) => match hex_decoder(hex) {
            None => Err("sceau conservé illisible".to_string()),
            Some(octets) => verifier_sceau_installe(&octets, id, ancres, maintenant_secs)
                .map_err(|e| e.to_string())
                .and_then(|v| {
                    if v.titre().version != service.manifest.version {
                        return Err("version signée différente de la version installée".to_string());
                    }
                    let officiel = v.niveau() == miyusceau::NiveauConfiance::Souverain;
                    if crate::service_manager::confiance::id_reserve_officiel(id) && !officiel {
                        return Err(format!("identifiant réservé '{id}' signé par un éditeur non officiel"));
                    }
                    Ok(if officiel { Provenance::SigneeOfficielle } else { Provenance::SigneeTierce })
                }),
        },
    };
    match verdict {
        Ok(p) => Ok(p),
        Err(motif) if dev_non_signe && cfg!(debug_assertions) => {
            tracing::error!(
                "⚠⚠ DÉROGATION DEV : lancement de '{id}' sans signature valide ({motif}) — \
                 aucun secret ne lui est transmis. Interdit en production. ⚠⚠"
            );
            Ok(Provenance::NonSigneeDev)
        }
        Err(motif) => Err(format!("Lancement de '{id}' refusé : {motif}")),
    }
}

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

        // CA-01 : provenance re-vérifiée avant toute exécution.
        let provenance = controler_provenance(
            service,
            &confiance::ancres_market(),
            confiance::maintenant_secs(),
            confiance::dev_non_signe_autorise(),
        )?;

        // En dev (build debug + dérogation), préférer le binaire dans le même
        // dossier que Central (target/debug/). Jamais en release.
        let binary_path = if provenance == Provenance::NonSigneeDev {
            dev_binary_override(&service.binary_path).unwrap_or_else(|| service.binary_path.clone())
        } else {
            service.binary_path.clone()
        };

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

        let service_env = build_service_env(service, provenance, profile_id, &data_dir)?;

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

/// Identifiant du service autorisé à recevoir les secrets MiyuCloud.
const ID_MIYUCLOUD: &str = "miyucloud";

/// Vrai si le service peut recevoir les secrets MiyuCloud : identifiant
/// `miyucloud` ET sceau d'un éditeur **officiel** déclarant cet identifiant.
pub fn secrets_miyucloud_autorises(service: &InstalledService, provenance: Provenance) -> bool {
    service.manifest.id == ID_MIYUCLOUD && provenance == Provenance::SigneeOfficielle
}

fn build_service_env(
    service: &InstalledService,
    provenance: Provenance,
    profile_id: &str,
    data_dir: &std::path::Path,
) -> Result<HashMap<String, String>, String> {
    let mut env = HashMap::new();

    if service.manifest.id == ID_MIYUCLOUD && !secrets_miyucloud_autorises(service, provenance) {
        // Jeton COG et phrase de chiffrement : jamais à un paquet non signé (CA-01).
        return Err(
            "MiyuCloud non signé par un éditeur de confiance : jeton et phrase de chiffrement refusés"
                .to_string(),
        );
    }

    if secrets_miyucloud_autorises(service, provenance) {
        let config = load_miyucloud_config();
        let central_db = miyukini_central::chemins::chemin_base_auth()
            .map_err(|e| format!("Chemin de la base Central : {e}"))?;
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
            central_db.to_string_lossy().to_string(),
        );
    }

    Ok(env)
}

/// En développement, si le binaire du service existe à côté de l'exécutable Central
/// (ex. `target/debug/`), on le préfère au binaire de l'install dir.
/// Uniquement pour un service lancé par dérogation dev (jamais en release).
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

#[cfg(test)]
mod tests {
    use super::*;
    use miyumarket::signature::{hex_encoder, signer_paquet, separer_enveloppe};
    use miyusceau::{CleSignature, NiveauConfiance};

    fn service(id: &str, sceau: Option<String>) -> InstalledService {
        let manifest = serde_json::from_str(&format!(
            r#"{{"id":"{id}","name":"x","version":"1.0.0","developer":"d","description":"","icon":"x","service_type":"interne_cog","source":"officiel","min_central_version":"0.1.0"}}"#
        ))
        .unwrap();
        InstalledService {
            manifest,
            install_dir: std::path::PathBuf::from("/nonexistent"),
            binary_path: std::path::PathBuf::from("/nonexistent/bin/x"),
            installed_at: String::new(),
            updated_at: None,
            sceau_paquet: sceau,
        }
    }

    fn sceau_pour(id: &str, cle: &CleSignature) -> String {
        let paquet = signer_paquet(b"PK zip", id, "1.0.0", None, cle).unwrap();
        let (sceau, _) = separer_enveloppe(&paquet).unwrap();
        hex_encoder(sceau)
    }

    /// Ancres d'un éditeur officiel (bundle).
    fn ancres(cle: &CleSignature) -> AncresDeConfiance {
        let mut a = AncresDeConfiance::new();
        a.ajouter(&cle.cle_publique(), NiveauConfiance::Souverain).unwrap();
        a
    }

    /// Ancres d'un éditeur ajouté localement.
    fn ancres_tierces(cle: &CleSignature) -> AncresDeConfiance {
        let mut a = AncresDeConfiance::new();
        a.ajouter(&cle.cle_publique(), NiveauConfiance::Reconnu).unwrap();
        a
    }

    #[test]
    fn editeur_tiers_ne_peut_ni_lancer_un_id_reserve_ni_recevoir_de_secrets() {
        let cle = CleSignature::generer();
        // miyucloud signé par un éditeur tiers : lancement refusé.
        let s = service("miyucloud", Some(sceau_pour("miyucloud", &cle)));
        assert!(controler_provenance(&s, &ancres_tierces(&cle), 0, false).is_err());
        // Service ordinaire signé par un tiers : accepté, sans secrets.
        let s = service("svc", Some(sceau_pour("svc", &cle)));
        assert_eq!(
            controler_provenance(&s, &ancres_tierces(&cle), 0, false),
            Ok(Provenance::SigneeTierce)
        );
        assert!(!secrets_miyucloud_autorises(&s, Provenance::SigneeTierce));
    }

    #[test]
    fn non_signe_refuse_au_lancement() {
        let cle = CleSignature::generer();
        assert!(controler_provenance(&service("svc", None), &ancres(&cle), 0, false).is_err());
    }

    #[test]
    fn signe_par_editeur_epingle_accepte() {
        let cle = CleSignature::generer();
        let s = service("svc", Some(sceau_pour("svc", &cle)));
        assert_eq!(controler_provenance(&s, &ancres(&cle), 0, false), Ok(Provenance::SigneeOfficielle));
    }

    #[test]
    fn secrets_miyucloud_exigent_un_sceau_miyucloud_epingle() {
        let cle = CleSignature::generer();
        let autre = CleSignature::generer();
        // Sceau valide mais pour un autre id : refus.
        let s = service("miyucloud", Some(sceau_pour("jaykoa", &cle)));
        assert!(controler_provenance(&s, &ancres(&cle), 0, false).is_err());
        // Sceau miyucloud d'un éditeur non épinglé : refus.
        let s = service("miyucloud", Some(sceau_pour("miyucloud", &autre)));
        assert!(controler_provenance(&s, &ancres(&cle), 0, false).is_err());
        // Sceau miyucloud épinglé : secrets autorisés.
        let s = service("miyucloud", Some(sceau_pour("miyucloud", &cle)));
        let p = controler_provenance(&s, &ancres(&cle), 0, false).unwrap();
        assert!(secrets_miyucloud_autorises(&s, p));
        // Dérogation dev : jamais de secrets.
        assert!(!secrets_miyucloud_autorises(&s, Provenance::NonSigneeDev));
        let tmp = std::env::temp_dir();
        assert!(build_service_env(&s, Provenance::NonSigneeDev, "p", &tmp).is_err());
    }
}
