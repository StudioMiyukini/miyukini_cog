//! @id     central_service_manager_installer
//! @do     install_package
//! @role   orchestration
//! @layer  app
//! @human  central — module « service manager installer ».

//! Installation et désinstallation de packages service (.msp).
//!
//! Un package .msp est une archive ZIP contenant :
//! - `service.manifest.json` : manifeste du service
//! - `bin/` : binaires du service
//! - `assets/` : icônes, ressources statiques (optionnel)

use std::io::Read;

use super::registry::{service_install_dir, InstalledService, ServiceRegistryData};
use miyumarket::manifest::ServiceManifest;

/// Valide le champ `binary_name` d'un manifeste : simple nom de fichier,
/// sans séparateur, sans `..`, sans nom absolu/lecteur, non vide.
/// Fail-closed : tout nom douteux est refusé (audit C3 — évite `../../x`).
pub fn validate_binary_name(name: &str) -> Result<(), String> {
    let invalid = || format!("binary_name invalide '{name}' : un simple nom de fichier est requis");
    if name.is_empty() || name == "." || name == ".." || name.len() > 255 {
        return Err(invalid());
    }
    if name.contains('/') || name.contains('\\') || name.contains(':') || name.contains('\0') {
        return Err(invalid());
    }
    if name.contains("..") {
        return Err(invalid());
    }
    // Contrôle final : un seul composant « normal » côté std::path.
    let mut comps = std::path::Path::new(name).components();
    match (comps.next(), comps.next()) {
        (Some(std::path::Component::Normal(_)), None) => Ok(()),
        _ => Err(invalid()),
    }
}

/// Nom du binaire (validé) pour un manifeste donné.
fn resolve_binary_name(manifest: &ServiceManifest) -> Result<String, String> {
    match manifest.binary_name.as_deref() {
        Some(n) => {
            validate_binary_name(n)?;
            Ok(n.to_string())
        }
        None => Ok(if cfg!(windows) {
            format!("{}.exe", manifest.id)
        } else {
            manifest.id.clone()
        }),
    }
}

/// Calcule le chemin de sortie d'une entrée d'archive, ou refuse (Zip Slip).
/// Rejette : chemin absolu, composant `..`, lecteur Windows, entrée symlink,
/// et tout chemin qui sortirait de `install_dir` après jointure.
fn safe_entry_path(
    file: &zip::read::ZipFile<'_>,
    install_dir: &std::path::Path,
) -> Result<std::path::PathBuf, String> {
    let raw = file.name().to_string();
    if file.is_symlink() {
        return Err(format!("Entrée symlink refusée dans le package : '{raw}'"));
    }
    if raw.starts_with('/') || raw.starts_with('\\') || raw.contains(':') {
        return Err(format!("Chemin absolu refusé dans le package : '{raw}'"));
    }
    if raw.split(['/', '\\']).any(|c| c == "..") {
        return Err(format!("Chemin '..' refusé dans le package : '{raw}'"));
    }
    let rel = file
        .enclosed_name()
        .ok_or_else(|| format!("Chemin d'entrée dangereux refusé : '{raw}'"))?;
    let out = install_dir.join(&rel);
    // Défense en profondeur : après jointure, rester sous install_dir.
    if !out.starts_with(install_dir)
        || out
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(format!("Chemin hors du répertoire d'installation : '{raw}'"));
    }
    Ok(out)
}

/// Installe un service depuis les bytes d'un package .msp (archive ZIP).
/// `expected_id` : identifiant de service demandé ; le manifeste du package
/// doit porter exactement cet id, sinon refus (audit C3).
/// Retourne l'ID du service installé.
pub fn install_package(
    package_bytes: &[u8],
    registry: &mut ServiceRegistryData,
    expected_id: &str,
) -> Result<String, String> {
    install_package_with_root(
        package_bytes,
        registry,
        expected_id,
        &super::registry::services_root_dir(),
    )
}

/// Variante avec racine d'installation explicite (utilisée par les tests).
fn install_package_with_root(
    package_bytes: &[u8],
    registry: &mut ServiceRegistryData,
    expected_id: &str,
    root: &std::path::Path,
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

    // Le manifeste doit correspondre au service demandé.
    if manifest.id != expected_id {
        return Err(format!(
            "Identifiant du manifeste ('{}') différent du service demandé ('{expected_id}') : installation refusée",
            manifest.id
        ));
    }
    let binary_name = resolve_binary_name(&manifest)?;

    let service_id = manifest.id.clone();
    let install_dir = root.join(&service_id);

    // 2. Pré-validation de TOUTES les entrées avant d'écrire quoi que ce soit.
    let mut plan: Vec<(usize, std::path::PathBuf, bool)> = Vec::new();
    for i in 0..archive.len() {
        let file = archive
            .by_index(i)
            .map_err(|e| format!("Erreur lecture archive index {i}: {e}"))?;
        let out_path = safe_entry_path(&file, &install_dir)?;
        plan.push((i, out_path, file.is_dir()));
    }

    tracing::info!(
        "Installation du service '{}' v{} dans {:?}",
        service_id,
        manifest.version,
        install_dir
    );

    // 3. Créer le répertoire d'installation
    std::fs::create_dir_all(&install_dir)
        .map_err(|e| format!("Impossible de créer {}: {e}", install_dir.display()))?;
    std::fs::create_dir_all(install_dir.join("bin"))
        .map_err(|e| format!("Impossible de créer bin/ : {e}"))?;
    std::fs::create_dir_all(install_dir.join("data"))
        .map_err(|e| format!("Impossible de créer data/ : {e}"))?;

    // 4. Extraire les fichiers
    for (i, out_path, is_dir) in plan {
        if is_dir {
            std::fs::create_dir_all(&out_path)
                .map_err(|e| format!("Création de '{}' échouée : {e}", out_path.display()))?;
            continue;
        }
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Erreur lecture archive index {i}: {e}"))?;
        let name = file.name().to_string();
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Création de '{}' échouée : {e}", parent.display()))?;
        }
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .map_err(|e| format!("Lecture de '{name}' échouée : {e}"))?;
        std::fs::write(&out_path, &buf)
            .map_err(|e| format!("Écriture de '{}' échouée : {e}", out_path.display()))?;
    }

    // 5. Sauvegarder le manifeste séparément
    let manifest_path = install_dir.join("service.manifest.json");
    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| format!("Sérialisation du manifeste échouée : {e}"))?;
    std::fs::write(&manifest_path, &manifest_json)
        .map_err(|e| format!("Écriture du manifeste échouée : {e}"))?;

    // 6. Chemin du binaire (nom déjà validé)
    let binary_path = install_dir.join("bin").join(&binary_name);

    // 7. Enregistrer dans le registre
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
    manifest
        .validate()
        .map_err(|e| format!("Manifeste invalide : {e}"))?;
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
    validate_binary_name(&binary_name)?;
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
        if let Err(e) = validate_binary_name(&binary_name) {
            tracing::warn!("Service '{service_id}' ignoré par le scan : {e}");
            continue;
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn manifest_json(id: &str, binary_name: Option<&str>) -> String {
        let bn = binary_name
            .map(|b| format!(r#", "binary_name": "{b}""#))
            .unwrap_or_default();
        format!(
            r#"{{"id":"{id}","name":"Test","version":"1.0.0","developer":"dev","description":"d","icon":"x","service_type":"interne_cog","source":"tiers","min_central_version":"0.1.0"{bn}}}"#
        )
    }

    fn build_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut buf = Vec::new();
        {
            let mut w = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
            let opts = zip::write::SimpleFileOptions::default();
            for (name, data) in entries {
                w.start_file(*name, opts).unwrap();
                w.write_all(data).unwrap();
            }
            w.finish().unwrap();
        }
        buf
    }

    fn tmp_root(tag: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("miyu-installer-{tag}-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn try_install(entries: &[(&str, &[u8])], expected: &str, tag: &str) -> (Result<String, String>, std::path::PathBuf) {
        let root = tmp_root(tag);
        let bytes = build_zip(entries);
        let mut reg = ServiceRegistryData::default();
        let r = install_package_with_root(&bytes, &mut reg, expected, &root);
        (r, root)
    }

    #[test]
    fn zip_valide_installe() {
        let m = manifest_json("svc", None);
        let (r, root) = try_install(
            &[("service.manifest.json", m.as_bytes()), ("bin/svc", b"x")],
            "svc",
            "ok",
        );
        assert_eq!(r.unwrap(), "svc");
        assert!(root.join("svc/bin/svc").exists());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn zip_slip_dotdot_refuse() {
        let m = manifest_json("svc", None);
        let (r, root) = try_install(
            &[("service.manifest.json", m.as_bytes()), ("../evil", b"x")],
            "svc",
            "dotdot",
        );
        assert!(r.is_err(), "{r:?}");
        assert!(!root.join("evil").exists());
        assert!(!root.join("svc").exists(), "rien ne doit être écrit");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn zip_chemin_absolu_refuse() {
        let m = manifest_json("svc", None);
        let (r, root) = try_install(
            &[("service.manifest.json", m.as_bytes()), ("/tmp/miyu-evil-abs", b"x")],
            "svc",
            "abs",
        );
        assert!(r.is_err(), "{r:?}");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn zip_symlink_refuse() {
        let m = manifest_json("svc", None);
        let mut buf = Vec::new();
        {
            let mut w = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
            let opts = zip::write::SimpleFileOptions::default();
            w.start_file("service.manifest.json", opts).unwrap();
            w.write_all(m.as_bytes()).unwrap();
            w.add_symlink("bin/link", "/etc/passwd", opts).unwrap();
            w.finish().unwrap();
        }
        let root = tmp_root("symlink");
        let mut reg = ServiceRegistryData::default();
        let r = install_package_with_root(&buf, &mut reg, "svc", &root);
        assert!(r.is_err(), "{r:?}");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn binary_name_invalide_refuse() {
        for bad in ["../x", "a/b", "a\\b", "..", "", "C:x"] {
            assert!(validate_binary_name(bad).is_err(), "{bad:?} devrait être refusé");
        }
        assert!(validate_binary_name("svc.exe").is_ok());
        let m = manifest_json("svc", Some("../../evil"));
        let (r, root) = try_install(&[("service.manifest.json", m.as_bytes())], "svc", "bn");
        assert!(r.is_err(), "{r:?}");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn id_manifeste_different_refuse() {
        let m = manifest_json("autre", None);
        let (r, root) = try_install(&[("service.manifest.json", m.as_bytes())], "svc", "id");
        assert!(r.unwrap_err().contains("différent"));
        let _ = std::fs::remove_dir_all(&root);
    }
}
