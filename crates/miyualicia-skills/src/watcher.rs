//! Watcher de manifestes — recharge incrémentale d'un dossier de services.
//!
//! Le `ManifestWatcher` mémorise les `mtime` des fichiers `service.manifest.json`
//! et ne rappelle `reload_in_dir` que si quelque chose a changé. Le caller
//! (le runtime Alicia) appelle `tick()` à intervalle régulier (typiquement
//! 1–5 s côté tokio).
//!
//! Pas de dépendance sur `notify` — un polling simple, basé sur `std::fs`,
//! couvre les besoins P6 sans introduire de surface de bugs cross-platform.

// @id service.alicia.skills.watcher
// @role manifest_hot_reloader
// @layer 7
// @do watch_and_hot_reload_manifests
// @human Polling watcher des service.manifest.json — déclenche reload_in_dir si les mtimes changent.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::discovery::{reload_in_dir, ReloadReport};
use crate::errors::SkillsError;
use crate::registry::SkillRegistry;

/// Watcher qui détecte les changements via `mtime` puis délègue à
/// `reload_in_dir`.
#[derive(Debug, Default)]
pub struct ManifestWatcher {
    /// Dernier `mtime` connu pour chaque fichier `service.manifest.json`
    /// (clé = chemin absolu).
    seen: HashMap<PathBuf, SystemTime>,
}

/// Résultat d'un `tick`.
#[derive(Debug)]
pub enum TickOutcome {
    /// Aucun changement filesystem détecté — registre intact.
    Unchanged,
    /// Changements détectés et `reload_in_dir` a été appelé.
    Reloaded(ReloadReport),
}

impl ManifestWatcher {
    /// Crée un watcher vierge.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Force un reload au prochain `tick` (vide la cache de mtimes).
    pub fn invalidate(&mut self) {
        self.seen.clear();
    }

    /// Tick : scanne le dossier, compare aux mtimes connus, ne déclenche
    /// `reload_in_dir` que si quelque chose a bougé (apparition,
    /// modification, disparition).
    pub fn tick(
        &mut self,
        dir: &Path,
        registry: &mut SkillRegistry,
    ) -> Result<TickOutcome, SkillsError> {
        let current = current_manifest_mtimes(dir)?;
        let changed = mtimes_changed(&self.seen, &current);
        if !changed {
            return Ok(TickOutcome::Unchanged);
        }
        let report = reload_in_dir(dir, registry)?;
        // Re-snap après reload pour intégrer toute (dis)apparition de fichier.
        self.seen = current_manifest_mtimes(dir)?;
        Ok(TickOutcome::Reloaded(report))
    }
}

fn current_manifest_mtimes(dir: &Path) -> Result<HashMap<PathBuf, SystemTime>, SkillsError> {
    let mut out = HashMap::new();
    if !dir.exists() {
        return Err(SkillsError::Io {
            path: dir.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("dossier introuvable : {}", dir.display()),
            ),
        });
    }
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

        let metadata = match std::fs::metadata(&manifest_path) {
            Ok(m) => m,
            Err(e) => {
                tracing::warn!(?manifest_path, "stat échoué : {e}");
                continue;
            }
        };
        let mtime = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        out.insert(manifest_path, mtime);
    }
    Ok(out)
}

fn mtimes_changed(
    previous: &HashMap<PathBuf, SystemTime>,
    current: &HashMap<PathBuf, SystemTime>,
) -> bool {
    if previous.len() != current.len() {
        return true;
    }
    for (path, mt) in current {
        match previous.get(path) {
            None => return true,
            Some(prev) if prev != mt => return true,
            _ => {}
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_manifest(dir: &Path, name: &str, contents: &str) -> PathBuf {
        let sub = dir.join(name);
        std::fs::create_dir_all(&sub).unwrap();
        let p = sub.join("service.manifest.json");
        std::fs::write(&p, contents).unwrap();
        p
    }

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

    /// Avance la mtime d'un fichier d'au moins quelques secondes.
    fn touch_manifest(path: &Path) {
        // Petit sleep pour s'assurer que la mtime est différente sur les FS
        // qui n'ont qu'une résolution seconde (FAT, certains exFAT).
        std::thread::sleep(std::time::Duration::from_millis(1100));
        let body = std::fs::read_to_string(path).unwrap();
        std::fs::write(path, body).unwrap();
    }

    #[test]
    fn first_tick_loads_existing_manifests() {
        let tmp = tempfile::tempdir().unwrap();
        write_manifest(tmp.path(), "jaykoa", &manifest_with_intent("jaykoa", "x"));

        let mut watcher = ManifestWatcher::new();
        let mut reg = SkillRegistry::new();
        match watcher.tick(tmp.path(), &mut reg).unwrap() {
            TickOutcome::Reloaded(r) => {
                assert_eq!(r.added, vec!["jaykoa".to_string()]);
            }
            TickOutcome::Unchanged => panic!("first tick must reload"),
        }
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn second_tick_unchanged_when_nothing_moves() {
        let tmp = tempfile::tempdir().unwrap();
        write_manifest(tmp.path(), "jaykoa", &manifest_with_intent("jaykoa", "x"));
        let mut watcher = ManifestWatcher::new();
        let mut reg = SkillRegistry::new();
        watcher.tick(tmp.path(), &mut reg).unwrap();
        let r2 = watcher.tick(tmp.path(), &mut reg).unwrap();
        assert!(matches!(r2, TickOutcome::Unchanged));
    }

    #[test]
    fn tick_after_new_manifest_added_reloads_with_added() {
        let tmp = tempfile::tempdir().unwrap();
        write_manifest(tmp.path(), "jaykoa", &manifest_with_intent("jaykoa", "x"));
        let mut watcher = ManifestWatcher::new();
        let mut reg = SkillRegistry::new();
        watcher.tick(tmp.path(), &mut reg).unwrap();

        write_manifest(
            tmp.path(),
            "jaykonta",
            &manifest_with_intent("jaykonta", "y"),
        );
        match watcher.tick(tmp.path(), &mut reg).unwrap() {
            TickOutcome::Reloaded(r) => {
                assert_eq!(r.added, vec!["jaykonta".to_string()]);
                assert_eq!(r.kept, vec!["jaykoa".to_string()]);
            }
            TickOutcome::Unchanged => panic!("expected reload after add"),
        }
    }

    #[test]
    fn tick_after_manifest_removal_reloads_with_removed() {
        let tmp = tempfile::tempdir().unwrap();
        write_manifest(tmp.path(), "jaykoa", &manifest_with_intent("jaykoa", "x"));
        write_manifest(
            tmp.path(),
            "jaykonta",
            &manifest_with_intent("jaykonta", "y"),
        );
        let mut watcher = ManifestWatcher::new();
        let mut reg = SkillRegistry::new();
        watcher.tick(tmp.path(), &mut reg).unwrap();
        assert_eq!(reg.len(), 2);

        std::fs::remove_dir_all(tmp.path().join("jaykonta")).unwrap();
        match watcher.tick(tmp.path(), &mut reg).unwrap() {
            TickOutcome::Reloaded(r) => {
                assert_eq!(r.removed, vec!["jaykonta".to_string()]);
                assert_eq!(reg.len(), 1);
            }
            TickOutcome::Unchanged => panic!("expected reload after remove"),
        }
    }

    #[test]
    fn tick_after_manifest_modified_reloads() {
        let tmp = tempfile::tempdir().unwrap();
        let path = write_manifest(tmp.path(), "jaykoa", &manifest_with_intent("jaykoa", "x"));
        let mut watcher = ManifestWatcher::new();
        let mut reg = SkillRegistry::new();
        watcher.tick(tmp.path(), &mut reg).unwrap();

        // Réécriture avec un nouvel intent
        std::fs::write(
            &path,
            r#"{
                "id": "jaykoa",
                "alicia_capabilities": [
                    {"intent": "x", "phrases_examples": ["a"], "params": {}, "writes": false},
                    {"intent": "y", "phrases_examples": ["b"], "params": {}, "writes": false}
                ]
            }"#,
        )
        .unwrap();
        // S'assurer que la mtime a bougé (même résolution seconde)
        touch_manifest(&path);

        match watcher.tick(tmp.path(), &mut reg).unwrap() {
            TickOutcome::Reloaded(r) => {
                assert_eq!(r.kept, vec!["jaykoa".to_string()]);
                assert!(reg.get("jaykoa.x").is_some());
                assert!(reg.get("jaykoa.y").is_some());
            }
            TickOutcome::Unchanged => panic!("expected reload after modify"),
        }
    }

    #[test]
    fn invalidate_forces_reload_on_next_tick() {
        let tmp = tempfile::tempdir().unwrap();
        write_manifest(tmp.path(), "jaykoa", &manifest_with_intent("jaykoa", "x"));
        let mut watcher = ManifestWatcher::new();
        let mut reg = SkillRegistry::new();
        watcher.tick(tmp.path(), &mut reg).unwrap();
        assert!(matches!(
            watcher.tick(tmp.path(), &mut reg).unwrap(),
            TickOutcome::Unchanged
        ));
        watcher.invalidate();
        assert!(matches!(
            watcher.tick(tmp.path(), &mut reg).unwrap(),
            TickOutcome::Reloaded(_)
        ));
    }

    #[test]
    fn missing_dir_returns_io_error() {
        let mut watcher = ManifestWatcher::new();
        let mut reg = SkillRegistry::new();
        let err = watcher
            .tick(Path::new("/__nope_watch__"), &mut reg)
            .unwrap_err();
        assert!(matches!(err, SkillsError::Io { .. }));
    }
}
