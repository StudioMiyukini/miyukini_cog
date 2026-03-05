//! Installation et désinstallation de packages service (.msp).
//!
//! Un package .msp est une archive ZIP contenant :
//! - `service.manifest.json` : manifeste du service
//! - `bin/` : binaires du service
//! - `assets/` : icônes, ressources statiques (optionnel)

use std::io::Read;

use super::registry::{service_install_dir, InstalledService, ServiceRegistryData};
use miyumarket::manifest::ServiceManifest;

/// Installe un service depuis les bytes d'un package .msp (archive ZIP).
/// Retourne l'ID du service installé.
pub fn install_package(
    package_bytes: &[u8],
    registry: &mut ServiceRegistryData,
) -> Result<String, String> {
    let cursor = std::io::Cursor::new(package_bytes);
    let mut archive = zip::ZipArchive::new(cursor)
        .map_err(|e| format!("Package .msp invalide (pas une archive ZIP) : {e}"))?;

    // 1. Lire le manifeste
    let manifest: ServiceManifest = {
        let mut manifest_file = archive
            .by_name("service.manifest.json")
            .map_err(|_| "service.manifest.json manquant dans le package")?;
        let mut content = String::new();
        manifest_file
            .read_to_string(&mut content)
            .map_err(|e| format!("Lecture du manifeste échouée : {e}"))?;
        serde_json::from_str(&content).map_err(|e| format!("Manifeste JSON invalide : {e}"))?
    };

    manifest
        .validate()
        .map_err(|e| format!("Manifeste invalide : {e}"))?;

    let service_id = manifest.id.clone();
    let install_dir = service_install_dir(&service_id);

    tracing::info!(
        "Installation du service '{}' v{} dans {:?}",
        service_id,
        manifest.version,
        install_dir
    );

    // 2. Créer le répertoire d'installation
    std::fs::create_dir_all(&install_dir)
        .map_err(|e| format!("Impossible de créer {}: {e}", install_dir.display()))?;
    std::fs::create_dir_all(install_dir.join("bin"))
        .map_err(|e| format!("Impossible de créer bin/ : {e}"))?;
    std::fs::create_dir_all(install_dir.join("data"))
        .map_err(|e| format!("Impossible de créer data/ : {e}"))?;

    // 3. Extraire les fichiers
    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Erreur lecture archive index {i}: {e}"))?;

        let name = file.name().to_string();
        if name.ends_with('/') {
            // Répertoire
            let dir_path = install_dir.join(&name);
            let _ = std::fs::create_dir_all(&dir_path);
            continue;
        }

        let out_path = install_dir.join(&name);
        if let Some(parent) = out_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .map_err(|e| format!("Lecture de '{name}' échouée : {e}"))?;
        std::fs::write(&out_path, &buf)
            .map_err(|e| format!("Écriture de '{}' échouée : {e}", out_path.display()))?;
    }

    // 4. Sauvegarder le manifeste séparément
    let manifest_path = install_dir.join("service.manifest.json");
    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| format!("Sérialisation du manifeste échouée : {e}"))?;
    std::fs::write(&manifest_path, &manifest_json)
        .map_err(|e| format!("Écriture du manifeste échouée : {e}"))?;

    // 5. Déterminer le chemin du binaire
    let binary_name = manifest.binary_name.clone().unwrap_or_else(|| {
        if cfg!(windows) {
            format!("{}.exe", service_id)
        } else {
            service_id.clone()
        }
    });
    let binary_path = install_dir.join("bin").join(&binary_name);

    // 6. Enregistrer dans le registre
    let installed = InstalledService {
        manifest,
        install_dir,
        binary_path,
        installed_at: chrono::Utc::now().to_rfc3339(),
        updated_at: None,
    };
    registry.services.insert(service_id.clone(), installed);

    tracing::info!("Service '{}' installé avec succès", service_id);
    Ok(service_id)
}

/// Installe un service directement depuis un binaire (sans package .msp).
/// Utilisé pour le développement et les services construits localement.
pub fn install_from_binary(
    manifest: ServiceManifest,
    binary_source: &std::path::Path,
    registry: &mut ServiceRegistryData,
) -> Result<String, String> {
    let service_id = manifest.id.clone();
    let install_dir = service_install_dir(&service_id);

    tracing::info!(
        "Installation directe du service '{}' depuis {:?}",
        service_id,
        binary_source
    );

    // Créer les répertoires
    std::fs::create_dir_all(install_dir.join("bin"))
        .map_err(|e| format!("Impossible de créer bin/ : {e}"))?;
    std::fs::create_dir_all(install_dir.join("data"))
        .map_err(|e| format!("Impossible de créer data/ : {e}"))?;

    // Copier le binaire
    let binary_name = manifest.binary_name.clone().unwrap_or_else(|| {
        binary_source
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| {
                if cfg!(windows) {
                    format!("{}.exe", service_id)
                } else {
                    service_id.clone()
                }
            })
    });
    let binary_path = install_dir.join("bin").join(&binary_name);
    std::fs::copy(binary_source, &binary_path)
        .map_err(|e| format!("Copie du binaire échouée : {e}"))?;

    // Sauvegarder le manifeste
    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| format!("Sérialisation du manifeste : {e}"))?;
    std::fs::write(install_dir.join("service.manifest.json"), &manifest_json)
        .map_err(|e| format!("Écriture du manifeste : {e}"))?;

    // Enregistrer
    let installed = InstalledService {
        manifest,
        install_dir: install_dir.clone(),
        binary_path: binary_path.clone(),
        installed_at: chrono::Utc::now().to_rfc3339(),
        updated_at: None,
    };
    registry.services.insert(service_id.clone(), installed);

    tracing::info!(
        "Service '{}' installé (binaire: {:?})",
        service_id,
        binary_path
    );
    Ok(service_id)
}

/// Désinstalle un service : supprime le répertoire d'installation et l'entrée du registre.
pub fn uninstall_service(
    service_id: &str,
    registry: &mut ServiceRegistryData,
) -> Result<(), String> {
    let service = registry
        .services
        .remove(service_id)
        .ok_or_else(|| format!("Service '{service_id}' non trouvé dans le registre"))?;

    tracing::info!(
        "Désinstallation du service '{}' depuis {:?}",
        service_id,
        service.install_dir
    );

    // Supprimer le répertoire d'installation
    if service.install_dir.exists() {
        std::fs::remove_dir_all(&service.install_dir).map_err(|e| {
            format!(
                "Suppression de {} échouée : {e}. Supprimez manuellement.",
                service.install_dir.display()
            )
        })?;
    }

    tracing::info!("Service '{}' désinstallé", service_id);
    Ok(())
}

/// Scanne le répertoire des services pour détecter des services installés manuellement
/// (par ex. en copiant un dossier avec un service.manifest.json + binaire).
pub fn scan_installed_services(registry: &mut ServiceRegistryData) {
    let root = super::registry::services_root_dir();
    if !root.exists() {
        return;
    }

    let Ok(entries) = std::fs::read_dir(&root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let manifest_path = path.join("service.manifest.json");
        if !manifest_path.exists() {
            continue;
        }

        // Lire le manifeste
        let Ok(content) = std::fs::read_to_string(&manifest_path) else {
            continue;
        };
        let Ok(manifest): Result<ServiceManifest, _> = serde_json::from_str(&content) else {
            continue;
        };

        let service_id = manifest.id.clone();

        // Ne pas écraser un service déjà enregistré
        if registry.services.contains_key(&service_id) {
            continue;
        }

        // Déduire le chemin du binaire
        let binary_name = manifest.binary_name.clone().unwrap_or_else(|| {
            if cfg!(windows) {
                format!("{}.exe", service_id)
            } else {
                service_id.clone()
            }
        });
        let binary_path = path.join("bin").join(&binary_name);

        if binary_path.exists() {
            tracing::info!(
                "Service détecté par scan : '{}' dans {:?}",
                service_id,
                path
            );
            registry.services.insert(
                service_id,
                InstalledService {
                    manifest,
                    install_dir: path,
                    binary_path,
                    installed_at: String::new(),
                    updated_at: None,
                },
            );
        }
    }
}
