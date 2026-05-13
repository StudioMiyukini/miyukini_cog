//! `restore_op` — récupération de fichiers depuis un snapshot.
//!
//! Trois modes :
//! - `restore_file_bytes(snapshot_id, file_path)` — renvoie le contenu en mémoire
//!   (utile pour les petits fichiers, le webmail, les previews).
//! - `restore_file_to_disk(snapshot_id, file_path, dest_path)` — écrit sur disque.
//! - `restore_snapshot_to_disk(snapshot_id, dest_root)` — restauration complète
//!   de tout le snapshot vers un répertoire.
//!
//! Toute lecture passe par `cas_kit::read` qui **vérifie l'intégrité**
//! systématiquement (SHA-256 recalculé == hash demandé).

use std::path::{Path, PathBuf};

use crate::kits::cas_kit::{self, CasKitError};
use crate::kits::crypto_kit::Key32;
use crate::operators::snapshots_op::{
    Manifest, ManifestEntry, SnapshotsOp, SnapshotsOpError,
};

/// Erreurs de l'Opérateur Restore.
#[derive(Debug, thiserror::Error)]
pub enum RestoreOpError {
    /// Snapshot introuvable.
    #[error("snapshot introuvable : {0}")]
    SnapshotNotFound(String),
    /// Fichier introuvable dans le snapshot.
    #[error("fichier introuvable dans le snapshot {snapshot_id} : {file_path}")]
    FileNotFound {
        /// ID du snapshot consulté.
        snapshot_id: String,
        /// Chemin demandé.
        file_path: String,
    },
    /// Le chemin demandé est un dossier (pas un fichier).
    #[error("le chemin '{0}' est un dossier, pas un fichier")]
    IsDirectory(String),
    /// Erreur d'I/O sur la destination.
    #[error("io destination : {0}")]
    Io(#[from] std::io::Error),
    /// Erreur CAS.
    #[error("cas : {0}")]
    Cas(#[from] CasKitError),
    /// Erreur Snapshots.
    #[error("snapshots : {0}")]
    Snapshots(#[from] SnapshotsOpError),
}

/// Résultat d'un restore complet (un par fichier).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoredFile {
    /// Chemin canonique dans le snapshot.
    pub path: String,
    /// Chemin sur disque où le fichier a été écrit.
    pub disk_path: PathBuf,
    /// Taille restaurée.
    pub size_bytes: u64,
}

/// Opérateur Restore.
pub struct RestoreOp {
    cas_root: PathBuf,
    cas_key: Key32,
}

impl RestoreOp {
    /// Construit l'opérateur. Doit partager la même racine + clé CAS que
    /// `SnapshotsOp` / `FilesOp`.
    #[must_use]
    pub fn new(cas_root: PathBuf, cas_key: Key32) -> Self {
        Self { cas_root, cas_key }
    }

    /// Renvoie le contenu d'un fichier (en mémoire). Vérifie l'intégrité.
    pub fn restore_file_bytes(
        &self,
        snapshots: &SnapshotsOp,
        snapshot_id: &str,
        file_path: &str,
    ) -> Result<Vec<u8>, RestoreOpError> {
        let manifest = read_manifest(snapshots, snapshot_id)?;
        let entry = find_file_entry(&manifest, snapshot_id, file_path)?;
        let hash = entry
            .blob_hash
            .clone()
            .ok_or_else(|| RestoreOpError::IsDirectory(entry.path.clone()))?;
        Ok(cas_kit::read(&hash, &self.cas_key, &self.cas_root)?)
    }

    /// Restaure un fichier vers un emplacement disque (crée les dossiers
    /// parents au besoin). Renvoie le nombre d'octets écrits.
    pub fn restore_file_to_disk(
        &self,
        snapshots: &SnapshotsOp,
        snapshot_id: &str,
        file_path: &str,
        dest_path: &Path,
    ) -> Result<u64, RestoreOpError> {
        let bytes = self.restore_file_bytes(snapshots, snapshot_id, file_path)?;
        if let Some(parent) = dest_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::write(dest_path, &bytes)?;
        Ok(bytes.len() as u64)
    }

    /// Restaure l'intégralité d'un snapshot vers `dest_root`.
    ///
    /// Les dossiers du manifest sont créés ; les fichiers sont écrits via
    /// `restore_file_to_disk`.
    pub fn restore_snapshot_to_disk(
        &self,
        snapshots: &SnapshotsOp,
        snapshot_id: &str,
        dest_root: &Path,
    ) -> Result<Vec<RestoredFile>, RestoreOpError> {
        let manifest = read_manifest(snapshots, snapshot_id)?;
        std::fs::create_dir_all(dest_root)?;
        let mut restored = Vec::new();

        for entry in &manifest.files {
            let dest_path = dest_root.join(&entry.path);

            if entry.is_dir {
                std::fs::create_dir_all(&dest_path)?;
                continue;
            }

            let hash = entry
                .blob_hash
                .clone()
                .ok_or_else(|| RestoreOpError::IsDirectory(entry.path.clone()))?;
            let bytes = cas_kit::read(&hash, &self.cas_key, &self.cas_root)?;
            if let Some(parent) = dest_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&dest_path, &bytes)?;
            restored.push(RestoredFile {
                path: entry.path.clone(),
                disk_path: dest_path,
                size_bytes: bytes.len() as u64,
            });
        }

        Ok(restored)
    }
}

fn read_manifest(snapshots: &SnapshotsOp, snapshot_id: &str) -> Result<Manifest, RestoreOpError> {
    snapshots.read_manifest(snapshot_id).map_err(|e| match e {
        SnapshotsOpError::NotFound(id) => RestoreOpError::SnapshotNotFound(id),
        other => RestoreOpError::Snapshots(other),
    })
}

fn find_file_entry<'a>(
    manifest: &'a Manifest,
    snapshot_id: &str,
    file_path: &str,
) -> Result<&'a ManifestEntry, RestoreOpError> {
    manifest
        .files
        .iter()
        .find(|e| e.path == file_path)
        .ok_or_else(|| RestoreOpError::FileNotFound {
            snapshot_id: snapshot_id.to_string(),
            file_path: file_path.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kits::cas_kit::CasConfig;
    use crate::operators::files_op::FilesOp;
    use crate::operators::snapshots_op::SnapshotsOp;

    fn setup() -> (FilesOp, SnapshotsOp, RestoreOp, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let cas_root = dir.path().join("cas");
        let key = Key32::generate();
        let cfg = CasConfig::default();
        let files = FilesOp::new(cas_root.clone(), key.clone(), cfg.clone());
        let snaps = SnapshotsOp::new(cas_root.clone(), key.clone(), cfg);
        let restore = RestoreOp::new(cas_root, key);
        (files, snaps, restore, dir)
    }

    #[test]
    fn restore_file_bytes_roundtrip() {
        let (files, snaps, restore, _dir) = setup();
        files.put("notes/a.txt", b"hello jaycloud", 1).unwrap();
        let s = snaps.create("t", &files, 1_700_000_000).unwrap();

        let bytes = restore
            .restore_file_bytes(&snaps, &s.id, "notes/a.txt")
            .unwrap();
        assert_eq!(bytes, b"hello jaycloud");
    }

    #[test]
    fn restore_file_bytes_missing_snapshot() {
        let (_files, snaps, restore, _dir) = setup();
        let r = restore.restore_file_bytes(&snaps, "ghost_snap", "a.txt");
        assert!(matches!(r, Err(RestoreOpError::SnapshotNotFound(_))));
    }

    #[test]
    fn restore_file_bytes_missing_file() {
        let (files, snaps, restore, _dir) = setup();
        files.put("a.txt", b"x", 1).unwrap();
        let s = snaps.create("t", &files, 1_700_000_000).unwrap();
        let r = restore.restore_file_bytes(&snaps, &s.id, "ghost.txt");
        assert!(matches!(r, Err(RestoreOpError::FileNotFound { .. })));
    }

    #[test]
    fn restore_file_to_disk_writes_content() {
        let (files, snaps, restore, dir) = setup();
        files.put("a.txt", b"disk content", 1).unwrap();
        let s = snaps.create("t", &files, 1_700_000_000).unwrap();

        let dest = dir.path().join("restored_a.txt");
        let written = restore
            .restore_file_to_disk(&snaps, &s.id, "a.txt", &dest)
            .unwrap();
        assert_eq!(written, "disk content".len() as u64);
        let read_back = std::fs::read(&dest).unwrap();
        assert_eq!(read_back, b"disk content");
    }

    #[test]
    fn restore_file_to_disk_creates_parents() {
        let (files, snaps, restore, dir) = setup();
        files.put("a.txt", b"x", 1).unwrap();
        let s = snaps.create("t", &files, 1_700_000_000).unwrap();

        let dest = dir.path().join("deeply/nested/restored.txt");
        restore
            .restore_file_to_disk(&snaps, &s.id, "a.txt", &dest)
            .unwrap();
        assert!(dest.exists());
    }

    #[test]
    fn restore_snapshot_to_disk_full() {
        let (files, snaps, restore, dir) = setup();
        files.put("readme.md", b"# Hello", 1).unwrap();
        files.put("src/main.rs", b"fn main() {}", 2).unwrap();
        files.put("src/lib.rs", b"// lib", 3).unwrap();
        let s = snaps.create("t", &files, 1_700_000_000).unwrap();

        let dest_root = dir.path().join("restored_project");
        let restored = restore
            .restore_snapshot_to_disk(&snaps, &s.id, &dest_root)
            .unwrap();
        assert_eq!(restored.len(), 3);

        assert_eq!(
            std::fs::read(dest_root.join("readme.md")).unwrap(),
            b"# Hello"
        );
        assert_eq!(
            std::fs::read(dest_root.join("src/main.rs")).unwrap(),
            b"fn main() {}"
        );
        assert_eq!(
            std::fs::read(dest_root.join("src/lib.rs")).unwrap(),
            b"// lib"
        );
    }

    #[test]
    fn restore_after_multiple_snapshots_uses_correct_version() {
        let (files, snaps, restore, _dir) = setup();
        files.put("a.txt", b"version 1", 1).unwrap();
        let s1 = snaps.create("t", &files, 1_700_000_000).unwrap();

        files.put("a.txt", b"version 2", 2).unwrap();
        let s2 = snaps.create("t", &files, 1_700_000_100).unwrap();

        let v1 = restore.restore_file_bytes(&snaps, &s1.id, "a.txt").unwrap();
        let v2 = restore.restore_file_bytes(&snaps, &s2.id, "a.txt").unwrap();
        assert_eq!(v1, b"version 1");
        assert_eq!(v2, b"version 2");
    }
}
