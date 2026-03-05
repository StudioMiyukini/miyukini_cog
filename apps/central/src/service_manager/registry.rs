//! Registre persisté des services installés.
//!
//! Stocké dans `%LOCALAPPDATA%/Miyukini-COG/services/registry.json`.
//! Chaque service installé a un répertoire dédié avec son binaire et ses données.

use miyumarket::manifest::ServiceManifest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Répertoire racine pour toutes les installations de services.
/// Windows : `%LOCALAPPDATA%/Miyukini-COG/services/`
pub fn services_root_dir() -> PathBuf {
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        PathBuf::from(local).join("Miyukini-COG").join("services")
    } else {
        // Fallback : répertoire courant
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
