//! @id     central_service_manager_registry
//! @do     services_root_dir
//! @role   orchestration
//! @layer  app
//! @human  central — module « service manager registry ».

//! Registre persisté des services installés.
//!
//! Stocké dans `%LOCALAPPDATA%/Miyukini-COG/services/registry.json`.
//! Chaque service installé a un répertoire dédié avec son binaire et ses données.

use miyumarket::manifest::ServiceManifest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Répertoire racine pour toutes les installations de services.
/// Windows : `%LOCALAPPDATA%/Miyukini-COG/services/`.
/// Ailleurs : debug → `./services` (historique) ; release → dossier de données
/// utilisateur (jamais le répertoire courant, CA-04).
pub fn services_root_dir() -> PathBuf {
    if let Some(local) = std::env::var_os("LOCALAPPDATA").filter(|v| !v.is_empty()) {
        return PathBuf::from(local).join("Miyukini-COG").join("services");
    }
    if miyukini_central::chemins::est_release() {
        match miyukini_central::chemins::dossier_donnees() {
            Ok(d) => d.join("services"),
            Err(e) => {
                // Fail-closed : chemin inexistant et non relatif plutôt que le cwd.
                tracing::error!("Dossier des services introuvable : {e}");
                PathBuf::from("/nonexistent/miyukini-cog/services")
            }
        }
    } else {
        PathBuf::from("./services")
    }
}

/// Chemin vers le fichier registry.json.
pub fn registry_path() -> PathBuf {
    services_root_dir().join("registry.json")
}

/// Enregistrement d'un service installé sur disque.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledService {
    /// Manifeste du service (métadonnées complètes).
    pub manifest: ServiceManifest,
    /// Répertoire d'installation absolu.
    pub install_dir: PathBuf,
    /// Chemin absolu vers le binaire du service.
    pub binary_path: PathBuf,
    /// Date d'installation (RFC 3339).
    pub installed_at: String,
    /// Date de dernière mise à jour (RFC 3339).
    pub updated_at: Option<String>,
    /// Sceau `ManifestePaquet` (hex) du paquet installé : re-vérifié au lancement
    /// et avant toute remise de secrets (CA-01). `None` = service non signé.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sceau_paquet: Option<String>,
}

/// Registre complet persisté sur disque.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceRegistryData {
    /// Services installés, indexés par id.
    pub services: HashMap<String, InstalledService>,
}

/// Charge le registre depuis le disque. Retourne un registre vide si le fichier n'existe pas.
pub fn load_registry() -> ServiceRegistryData {
    let path = registry_path();
    if !path.exists() {
        return ServiceRegistryData::default();
    }
    match std::fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|e| {
            tracing::warn!("Registre corrompu, réinitialisation: {e}");
            ServiceRegistryData::default()
        }),
        Err(e) => {
            tracing::warn!("Impossible de lire le registre: {e}");
            ServiceRegistryData::default()
        }
    }
}

/// Sauvegarde le registre sur disque.
pub fn save_registry(registry: &ServiceRegistryData) -> Result<(), String> {
    let path = registry_path();
    // Créer le répertoire parent si nécessaire
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Impossible de créer le répertoire du registre: {e}"))?;
    }
    let json = serde_json::to_string_pretty(registry)
        .map_err(|e| format!("Sérialisation du registre échouée: {e}"))?;
    std::fs::write(&path, json).map_err(|e| format!("Écriture du registre échouée: {e}"))?;
    tracing::debug!("Registre sauvegardé: {} services", registry.services.len());
    Ok(())
}

/// Répertoire d'installation d'un service donné.
pub fn service_install_dir(service_id: &str) -> PathBuf {
    services_root_dir().join(service_id)
}
