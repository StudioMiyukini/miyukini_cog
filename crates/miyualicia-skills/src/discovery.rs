//! Découverte des manifestes de service dans un répertoire.
//!
//! Pour P1, le mode standard est : pointer Alicia vers un dossier qui contient
//! plusieurs `service.manifest.json` (un par service installé). À chaque
//! démarrage on scanne, on parse, on enregistre.
//!
//! Les hot-reload (P6) et l'intégration miyumarket (P3+) viendront plus tard.

// @id service.alicia.skills.discovery
// @role manifest_discovery
// @layer 7
// @do discover_service_manifests
// @human Scanne un dossier de manifestes et alimente un SkillRegistry.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use chrono::Utc;

use crate::capability::is_valid_id;
use crate::errors::SkillsError;
use crate::manifest::AliciaCapabilityManifest;
use crate::registry::{Skill, SkillRegistry};

/// Source d'un manifeste découvert — utilisé pour traçabilité.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestSource {
    /// Fichier sur disque.
    File(PathBuf),
    /// Manifeste injecté inline (tests, par défaut).
    Inline(String),
}

impl ManifestSource {
    /// Représentation textuelle pour la persistance / les logs.
    #[must_use]
    pub fn as_label(&self) -> String {
        match self {
            Self::File(p) => p.display().to_string(),
            Self::Inline(s) => format!("inline:{s}"),
        }
    }
}

/// Scanne un dossier et enregistre toutes les capacités trouvées.
///
/// Stratégie : pour chaque sous-élément du dossier, si c'est un fichier
/// `service.manifest.json` (ou si c'est un dossier qui en contient un),
/// on parse et on enregistre les capacités valides. Les capacités invalides
/// sont retournées dans `invalid` pour log côté caller — le scan ne s'arrête
/// pas pour autant.
///
/// Les services existants sont remplacés (`upsert`).
///
/// Retourne `(nombre_capacités_enregistrées, erreurs_non_fatales)`.
pub fn discover_in_dir(
    dir: &Path,
    registry: &mut SkillRegistry,
) -> Result<(usize, Vec<SkillsError>), SkillsError> {
    if !dir.exists() {
        return Err(SkillsError::Io {
            path: dir.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("dossier introuvable : {}", dir.display()),
            ),
        });
    }

    let mut registered = 0;
    let mut errors = Vec::new();

    let entries = std::fs::read_dir(dir).map_err(|e| SkillsError::Io {
        path: dir.to_path_buf(),
        source: e,
    })?;

    for entry in entries.flatten() {
        let path = entry.path();
        let manifest_path = if path.is_dir() {
            let p = path.join("service.manifest.json");
            if p.exists() {
                p
            } else {
                continue;
            }
        } else if path.file_name() == Some(std::ffi::OsStr::new("service.manifest.json")) {
            path
        } else {
            continue;
        };

        match AliciaCapabilityManifest::from_file(&manifest_path) {
            Ok(manifest) => {
                let label = manifest_path.display().to_string();
                let now = Utc::now().to_rfc3339();
                for result in manifest.into_capabilities() {
                    match result {
                        Ok(capability) => {
                            let skill = Skill {
                                capability,
                                source: label.clone(),
                                registered_at: now.clone(),
                            };
                            if let Err(e) = registry.upsert(skill) {
                                errors.push(e);
                            } else {
                                registered += 1;
                            }
                        }
                        Err(e) => errors.push(e),
                    }
                }
            }
            Err(e) => errors.push(e),
        }
    }

    Ok((registered, errors))
}

/// Compte-rendu d'un `reload_in_dir`.
///
/// Toutes les listes contiennent des `service_id` (pas des `qualified_id`),
/// puisque le hot reload travaille au niveau service (un service = un
/// manifeste = N capabilities).
#[derive(Debug, Default)]
pub struct ReloadReport {
    /// Services qui n'étaient pas dans le registre avant ce tour.
    pub added: Vec<String>,
    /// Services déjà connus, ré-upserts (changement éventuel pas détecté ici).
    pub kept: Vec<String>,
    /// Services dont le manifeste a disparu — leurs capabilities sont retirées.
    pub removed: Vec<String>,
    /// Erreurs non fatales rencontrées (manifeste illisible, capability
    /// invalide…). Le scan continue malgré.
    pub errors: Vec<SkillsError>,
}

impl ReloadReport {
    /// `true` si rien n'a changé (ni ajout ni retrait, pas d'erreur).
    #[must_use]
    pub fn is_noop(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.errors.is_empty()
    }

    /// Total des services présents après reload.
    #[must_use]
    pub fn total_present(&self) -> usize {
        self.added.len() + self.kept.len()
    }
}

/// Hot reload du registre depuis un dossier de manifestes.
///
/// Différence vs `discover_in_dir` : **les services dont le manifeste a
/// disparu sont retirés** du registre. Renvoie un `ReloadReport` détaillant
/// ce qui a bougé.
///
/// Les services existants sont remplacés par upsert (les éventuelles
/// modifications du manifeste sont prises en compte). Les capabilities
/// invalides sont collectées dans `errors` sans interrompre le scan.
pub fn reload_in_dir(
    dir: &Path,
    registry: &mut SkillRegistry,
) -> Result<ReloadReport, SkillsError> {
    if !dir.exists() {
        return Err(SkillsError::Io {
            path: dir.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("dossier introuvable : {}", dir.display()),
            ),
        });
    }

    // Snapshot des services présents avant ce tour.
    let before: BTreeSet<String> = registry
        .iter()
        .map(|s| s.capability.service_id.clone())
        .collect();

    let entries = std::fs::read_dir(dir).map_err(|e| SkillsError::Io {
        path: dir.to_path_buf(),
        source: e,
    })?;

    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut errors = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        let manifest_path = if path.is_dir() {
            let p = path.join("service.manifest.json");
            if p.exists() {
                p
            } else {
                continue;
            }
        } else if path.file_name() == Some(std::ffi::OsStr::new("service.manifest.json")) {
            path
        } else {
            continue;
        };

        match AliciaCapabilityManifest::from_file(&manifest_path) {
            Ok(manifest) => {
                let manifest_id = manifest.id.clone();
                let label = manifest_path.display().to_string();
                let now = Utc::now().to_rfc3339();
                for result in manifest.into_capabilities() {
                    match result {
                        Ok(capability) => {
                            let skill = Skill {
                                capability,
                                source: label.clone(),
                                registered_at: now.clone(),
                            };
                            if let Err(e) = registry.upsert(skill) {
                                errors.push(e);
                            }
                        }
                        Err(e) => errors.push(e),
                    }
                }
                // On ne marque le service comme « vu » que si son identifiant
                // est conforme — un manifeste avec un id invalide ne doit pas
                // protéger ses (éventuelles) anciennes capabilities de la
                // suppression du registre.
                if is_valid_id(&manifest_id) {
                    seen.insert(manifest_id);
                }
            }
            Err(e) => errors.push(e),
        }
    }

    // Suppression des services disparus.
    let removed: Vec<String> = before.difference(&seen).cloned().collect();
    for sid in &removed {
        registry.remove_service(sid);
    }

    let added: Vec<String> = seen.difference(&before).cloned().collect();
    let kept: Vec<String> = seen.intersection(&before).cloned().collect();

    Ok(ReloadReport {
        added,
        kept,
        removed,
        errors,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_manifest(dir: &Path, name: &str, contents: &str) {
        let sub = dir.join(name);
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::write(sub.join("service.manifest.json"), contents).unwrap();
    }

    #[test]
    fn discover_loads_two_services() {
        let tmp = tempfile::tempdir().unwrap();

        write_manifest(
            tmp.path(),
            "jaykoa",
            r#"{
                "id": "jaykoa",
                "alicia_capabilities": [
                    {
                        "intent": "schedule_event",
                        "phrases_examples": ["ajoute une réunion"],
                        "params": {
                            "title": { "type": "string", "required": true }
                        },
                        "writes": true
                    }
                ]
            }"#,
        );

        write_manifest(
            tmp.path(),
            "jaykonta",
            r#"{
                "id": "jaykonta",
                "alicia_capabilities": [
                    {
                        "intent": "add_expense",
                        "phrases_examples": ["note une dépense"],
                        "params": {
                            "amount": { "type": "float", "required": true }
                        },
                        "writes": true
                    }
                ]
            }"#,
        );

        let mut reg = SkillRegistry::new();
        let (count, errors) = discover_in_dir(tmp.path(), &mut reg).unwrap();
        assert_eq!(count, 2);
        assert!(errors.is_empty());
        assert_eq!(reg.len(), 2);
        assert!(reg.get("jaykoa.schedule_event").is_some());
        assert!(reg.get("jaykonta.add_expense").is_some());
    }

    #[test]
    fn discover_collects_errors_without_aborting() {
        let tmp = tempfile::tempdir().unwrap();

        write_manifest(
            tmp.path(),
            "good",
            r#"{
                "id": "jaykoa",
                "alicia_capabilities": [
                    {"intent": "x", "phrases_examples": ["x"], "params": {}, "writes": false}
                ]
            }"#,
        );

        write_manifest(tmp.path(), "broken", r#"{ "id": "bad service" }"#);

        let mut reg = SkillRegistry::new();
        let (count, errors) = discover_in_dir(tmp.path(), &mut reg).unwrap();
        assert_eq!(count, 1);
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], SkillsError::InvalidId(_)));
    }

    #[test]
    fn discover_returns_io_error_on_missing_dir() {
        let mut reg = SkillRegistry::new();
        let err = discover_in_dir(Path::new("/__nope__nope__nope"), &mut reg).unwrap_err();
        assert!(matches!(err, SkillsError::Io { .. }));
    }

    #[test]
    fn discover_skips_non_manifest_files() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("README.md"), "# hello").unwrap();
        std::fs::write(tmp.path().join("config.toml"), "key = 1").unwrap();

        let mut reg = SkillRegistry::new();
        let (count, errors) = discover_in_dir(tmp.path(), &mut reg).unwrap();
        assert_eq!(count, 0);
        assert!(errors.is_empty());
    }

    #[test]
    fn manifest_source_label_round_trip() {
        let f = ManifestSource::File(PathBuf::from("/tmp/x.json"));
        assert!(f.as_label().ends_with("x.json"));
        let i = ManifestSource::Inline("test".into());
        assert_eq!(i.as_label(), "inline:test");
    }

    // ----------------------------------------------------------------------
    // P6.1 — reload_in_dir
    // ----------------------------------------------------------------------

    fn manifest_with_intent(service: &str, intent: &str) -> String {
        format!(
            r#"{{
                "id": "{service}",
                "alicia_capabilities": [
                    {{
                        "intent": "{intent}",
                        "phrases_examples": ["x"],
                        "params": {{}},
                        "writes": false
                    }}
                ]
            }}"#
        )
    }

    #[test]
    fn reload_empty_dir_yields_noop_on_empty_registry() {
        let tmp = tempfile::tempdir().unwrap();
        let mut reg = SkillRegistry::new();
        let report = reload_in_dir(tmp.path(), &mut reg).unwrap();
        assert!(report.is_noop());
        assert_eq!(report.total_present(), 0);
    }

    #[test]
    fn reload_adds_new_services() {
        let tmp = tempfile::tempdir().unwrap();
        write_manifest(tmp.path(), "jaykoa", &manifest_with_intent("jaykoa", "x"));
        write_manifest(
            tmp.path(),
            "jaykonta",
            &manifest_with_intent("jaykonta", "y"),
        );
        let mut reg = SkillRegistry::new();
        let report = reload_in_dir(tmp.path(), &mut reg).unwrap();
        assert_eq!(report.added.len(), 2);
        assert!(report.kept.is_empty());
        assert!(report.removed.is_empty());
        assert!(report.errors.is_empty());
        assert_eq!(reg.len(), 2);
    }

    #[test]
    fn reload_keeps_existing_and_does_not_double_register() {
        let tmp = tempfile::tempdir().unwrap();
        write_manifest(tmp.path(), "jaykoa", &manifest_with_intent("jaykoa", "x"));
        let mut reg = SkillRegistry::new();
        let r1 = reload_in_dir(tmp.path(), &mut reg).unwrap();
        assert_eq!(r1.added, vec!["jaykoa".to_string()]);

        let r2 = reload_in_dir(tmp.path(), &mut reg).unwrap();
        assert!(r2.added.is_empty());
        assert_eq!(r2.kept, vec!["jaykoa".to_string()]);
        assert!(r2.removed.is_empty());
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn reload_removes_disappeared_services() {
        let tmp = tempfile::tempdir().unwrap();
        write_manifest(tmp.path(), "jaykoa", &manifest_with_intent("jaykoa", "x"));
        write_manifest(
            tmp.path(),
            "jaykonta",
            &manifest_with_intent("jaykonta", "y"),
        );
        let mut reg = SkillRegistry::new();
        reload_in_dir(tmp.path(), &mut reg).unwrap();
        assert_eq!(reg.len(), 2);

        // Suppression du dossier jaykonta
        std::fs::remove_dir_all(tmp.path().join("jaykonta")).unwrap();
        let report = reload_in_dir(tmp.path(), &mut reg).unwrap();
        assert!(report.added.is_empty());
        assert_eq!(report.kept, vec!["jaykoa".to_string()]);
        assert_eq!(report.removed, vec!["jaykonta".to_string()]);
        assert_eq!(reg.len(), 1);
        assert!(reg.get("jaykoa.x").is_some());
        assert!(reg.get("jaykonta.y").is_none());
    }

    #[test]
    fn reload_picks_up_new_intent_in_updated_manifest() {
        let tmp = tempfile::tempdir().unwrap();
        write_manifest(tmp.path(), "jaykoa", &manifest_with_intent("jaykoa", "x"));
        let mut reg = SkillRegistry::new();
        reload_in_dir(tmp.path(), &mut reg).unwrap();
        assert!(reg.get("jaykoa.x").is_some());
        assert!(reg.get("jaykoa.y").is_none());

        // Réécriture du manifeste avec un nouvel intent
        std::fs::write(
            tmp.path().join("jaykoa").join("service.manifest.json"),
            r#"{
                "id": "jaykoa",
                "alicia_capabilities": [
                    {"intent": "x", "phrases_examples": ["a"], "params": {}, "writes": false},
                    {"intent": "y", "phrases_examples": ["b"], "params": {}, "writes": false}
                ]
            }"#,
        )
        .unwrap();
        let report = reload_in_dir(tmp.path(), &mut reg).unwrap();
        assert!(report.added.is_empty());
        assert_eq!(report.kept, vec!["jaykoa".to_string()]);
        assert!(reg.get("jaykoa.x").is_some());
        assert!(reg.get("jaykoa.y").is_some());
    }

    #[test]
    fn reload_collects_errors_without_aborting() {
        let tmp = tempfile::tempdir().unwrap();
        write_manifest(tmp.path(), "good", &manifest_with_intent("jaykoa", "x"));
        write_manifest(tmp.path(), "broken", r#"{ "id": "bad service" }"#);
        let mut reg = SkillRegistry::new();
        let report = reload_in_dir(tmp.path(), &mut reg).unwrap();
        // jaykoa OK + le manifeste broken garde son service_id "bad service"
        // (refusé par validation) → pas dans le registre, mais erreur posée
        assert_eq!(report.added, vec!["jaykoa".to_string()]);
        assert!(!report.errors.is_empty());
    }

    #[test]
    fn reload_returns_io_error_on_missing_dir() {
        let mut reg = SkillRegistry::new();
        let err = reload_in_dir(Path::new("/__nope_reload__"), &mut reg).unwrap_err();
        assert!(matches!(err, SkillsError::Io { .. }));
    }
}
