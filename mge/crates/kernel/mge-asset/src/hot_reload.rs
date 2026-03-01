//! Hot-reload de fichiers TOML avec `notify`.
//!
//! <!-- @id: sd-data-hotreload @do: define @role: arpg @layer: 3 @human: miyuk -->
//!
//! Watcher de fichiers pour recharger les TOML modifies en developpement.
//! Active uniquement avec le feature flag `dev-hotreload`.

use std::path::{Path, PathBuf};
use std::sync::mpsc;

use notify::{Event, EventKind, RecursiveMode, Watcher};

use crate::registry::GameDataRegistry;
use crate::DataLoadError;

/// Evenement de modification de fichier.
#[derive(Debug)]
pub struct FileChangedEvent {
    /// Chemin du fichier modifie.
    pub path: PathBuf,
    /// Type de modification.
    pub kind: FileChangeKind,
}

/// Type de modification de fichier.
#[derive(Debug)]
pub enum FileChangeKind {
    /// Fichier modifie.
    Modified,
    /// Fichier cree.
    Created,
    /// Fichier supprime.
    Deleted,
}

/// Demarre le watcher sur un repertoire.
///
/// Les evenements sont envoyes via un channel `mpsc`.
/// Le watcher doit etre conserve tant que la surveillance est active.
pub fn start_file_watcher(
    watch_dir: &Path,
) -> Result<(notify::RecommendedWatcher, mpsc::Receiver<FileChangedEvent>), DataLoadError> {
    let (tx, rx) = mpsc::channel();

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            for path in &event.paths {
                let kind = match event.kind {
                    EventKind::Modify(_) => FileChangeKind::Modified,
                    EventKind::Create(_) => FileChangeKind::Created,
                    EventKind::Remove(_) => FileChangeKind::Deleted,
                    _ => continue,
                };
                let _ = tx.send(FileChangedEvent {
                    path: path.clone(),
                    kind,
                });
            }
        }
    })
    .map_err(|e| DataLoadError::WatcherError {
        message: e.to_string(),
    })?;

    watcher
        .watch(watch_dir, RecursiveMode::Recursive)
        .map_err(|e| DataLoadError::WatcherError {
            message: e.to_string(),
        })?;

    Ok((watcher, rx))
}

/// Systeme PreUpdate qui poll les changements de fichiers et recharge les donnees.
///
/// Appeler cette fonction dans la boucle de jeu pour traiter les changements
/// de fichiers et mettre a jour le registre.
pub fn hot_reload_system(
    rx: &mpsc::Receiver<FileChangedEvent>,
    registry: &mut GameDataRegistry,
    data_root: &Path,
) {
    while let Ok(event) = rx.try_recv() {
        if let Some(ext) = event.path.extension() {
            if ext == "toml" {
                tracing::info!("Hot-reload: {:?} {:?}", event.kind, event.path);
                if let Some(category) = detect_category(&event.path, data_root) {
                    let _ = reload_category(registry, &category, data_root);
                }
            }
        }
    }
}

/// Detecte la categorie d'un fichier modifie a partir de son chemin relatif.
fn detect_category(path: &Path, data_root: &Path) -> Option<String> {
    let relative = path.strip_prefix(data_root).ok()?;
    let first_component = relative.components().next()?;
    Some(first_component.as_os_str().to_string_lossy().to_string())
}

/// Recharge une categorie de donnees dans le registre.
///
/// Pour l'instant, log simplement la categorie rechargee.
/// L'implementation complete rechargera les donnees par domaine.
fn reload_category(
    _registry: &mut GameDataRegistry,
    category: &str,
    _data_root: &Path,
) -> Result<(), DataLoadError> {
    tracing::info!("Reloading category: {category}");
    // TODO: Implementation par domaine (classes, skills, items, etc.).
    // Pour chaque categorie, appeler la fonction de chargement correspondante
    // et remplacer la section dans le registre.
    Ok(())
}
