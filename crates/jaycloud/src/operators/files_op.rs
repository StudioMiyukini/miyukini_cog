//! `files_op` — Opérateur de stockage de fichiers, héritier MiyuCloud.
//!
//! En PR-2 (P3.a), cette implémentation reste **minimale** :
//! - un index `path → BlobHash` en mémoire (sera persisté via `storage_kit`
//!   et au-dessus du CAS en PR-3),
//! - lecture / écriture / suppression / liste,
//! - chemins canoniques (anti path-traversal) via `files::tree`.
//!
//! Le pattern de chunking + chiffrement bloc-par-bloc (cf. miyucloud
//! `storage/chunking.rs`) sera rapatrié en PR-3 avec la création de
//! snapshots — pour P3.a on stocke chaque fichier comme un seul blob CAS.
//!
//! Conforme DT-07 de la Spec.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

use crate::files::tree;
use crate::kits::cas_kit::{self, BlobHash, CasConfig, CasKitError};
use crate::kits::crypto_kit::Key32;

/// Erreurs de l'Opérateur Files.
#[derive(Debug, thiserror::Error)]
pub enum FilesOpError {
    /// Chemin invalide (path traversal, caractère interdit, etc.).
    #[error("chemin invalide : {0}")]
    InvalidPath(String),
    /// Fichier introuvable.
    #[error("fichier introuvable : {0}")]
    NotFound(String),
    /// Erreur du Kit CAS.
    #[error("cas : {0}")]
    Cas(#[from] CasKitError),
}

/// Métadonnée d'une entrée Files (vue par les autres modules).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    /// Chemin canonique (relatif au root logique).
    pub path: String,
    /// Hash CAS du contenu (None pour un dossier).
    pub blob_hash: Option<BlobHash>,
    /// Taille en octets (None pour un dossier).
    pub size_bytes: Option<u64>,
    /// Timestamp Unix de la dernière modification.
    pub mtime: i64,
    /// `true` si dossier, `false` si fichier régulier.
    pub is_dir: bool,
}

/// Opérateur Files. Maintient un index en mémoire des entrées.
///
/// En PR-3, l'index sera persisté via `storage_kit` (table dédiée ou via
/// les manifests de snapshots, à arbitrer en revue).
pub struct FilesOp {
    /// Index path canonique → entrée.
    entries: RwLock<HashMap<String, FileEntry>>,
    /// Racine disque du CAS.
    cas_root: PathBuf,
    /// Clé de chiffrement des blobs CAS (dérivée de KindMother en runtime).
    cas_key: Key32,
    /// Configuration CAS (compression).
    cas_config: CasConfig,
}

impl FilesOp {
    /// Construit un Opérateur Files avec une racine CAS et une clé.
    #[must_use]
    pub fn new(cas_root: PathBuf, cas_key: Key32, cas_config: CasConfig) -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            cas_root,
            cas_key,
            cas_config,
        }
    }

    /// Écrit un fichier (créé ou écrasé). Renvoie l'entrée stockée.
    pub fn put(&self, path: &str, content: &[u8], mtime: i64) -> Result<FileEntry, FilesOpError> {
        let canonical = tree::canonical_path(path)
            .map_err(|e| FilesOpError::InvalidPath(e.to_string()))?;
        let blob_hash = cas_kit::store(content, &self.cas_key, &self.cas_root, &self.cas_config)?;
        let entry = FileEntry {
            path: canonical.clone(),
            blob_hash: Some(blob_hash),
            size_bytes: Some(content.len() as u64),
            mtime,
            is_dir: false,
        };
        self.entries.write().unwrap().insert(canonical, entry.clone());
        Ok(entry)
    }

    /// Lit le contenu d'un fichier.
    pub fn get(&self, path: &str) -> Result<Vec<u8>, FilesOpError> {
        let canonical = tree::canonical_path(path)
            .map_err(|e| FilesOpError::InvalidPath(e.to_string()))?;
        let entry = {
            let entries = self.entries.read().unwrap();
            entries
                .get(&canonical)
                .cloned()
                .ok_or_else(|| FilesOpError::NotFound(canonical.clone()))?
        };
        match entry.blob_hash {
            Some(hash) => Ok(cas_kit::read(&hash, &self.cas_key, &self.cas_root)?),
            None => Err(FilesOpError::NotFound(format!("{canonical} (dossier)"))),
        }
    }

    /// Métadonnée d'une entrée.
    pub fn stat(&self, path: &str) -> Result<FileEntry, FilesOpError> {
        let canonical = tree::canonical_path(path)
            .map_err(|e| FilesOpError::InvalidPath(e.to_string()))?;
        self.entries
            .read()
            .unwrap()
            .get(&canonical)
            .cloned()
            .ok_or_else(|| FilesOpError::NotFound(canonical))
    }

    /// Liste les entrées (P3.a : flat ; sub-dirs en P3.b).
    pub fn list(&self) -> Vec<FileEntry> {
        let entries = self.entries.read().unwrap();
        let mut out: Vec<_> = entries.values().cloned().collect();
        out.sort_by(|a, b| a.path.cmp(&b.path));
        out
    }

    /// Supprime une entrée (sans GC du blob CAS — le snapshot_op fera le GC).
    pub fn delete(&self, path: &str) -> Result<(), FilesOpError> {
        let canonical = tree::canonical_path(path)
            .map_err(|e| FilesOpError::InvalidPath(e.to_string()))?;
        let removed = self.entries.write().unwrap().remove(&canonical);
        if removed.is_none() {
            return Err(FilesOpError::NotFound(canonical));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_op() -> (FilesOp, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let op = FilesOp::new(
            dir.path().join("cas"),
            Key32::generate(),
            CasConfig::default(),
        );
        (op, dir)
    }

    #[test]
    fn put_and_get_roundtrip() {
        let (op, _dir) = make_op();
        op.put("notes/hello.txt", b"hello world", 1_000).unwrap();
        let content = op.get("notes/hello.txt").unwrap();
        assert_eq!(content, b"hello world");
    }

    #[test]
    fn stat_returns_entry() {
        let (op, _dir) = make_op();
        op.put("doc.md", b"abc", 2_000).unwrap();
        let entry = op.stat("doc.md").unwrap();
        assert_eq!(entry.path, "doc.md");
        assert_eq!(entry.size_bytes, Some(3));
        assert_eq!(entry.mtime, 2_000);
        assert!(!entry.is_dir);
    }

    #[test]
    fn list_returns_sorted() {
        let (op, _dir) = make_op();
        op.put("b.txt", b"b", 1).unwrap();
        op.put("a.txt", b"a", 1).unwrap();
        op.put("c.txt", b"c", 1).unwrap();
        let paths: Vec<_> = op.list().into_iter().map(|e| e.path).collect();
        assert_eq!(paths, vec!["a.txt", "b.txt", "c.txt"]);
    }

    #[test]
    fn get_missing_fails() {
        let (op, _dir) = make_op();
        let r = op.get("ghost.txt");
        assert!(matches!(r, Err(FilesOpError::NotFound(_))));
    }

    #[test]
    fn delete_removes_entry() {
        let (op, _dir) = make_op();
        op.put("tmp.txt", b"x", 1).unwrap();
        assert!(op.stat("tmp.txt").is_ok());
        op.delete("tmp.txt").unwrap();
        assert!(matches!(op.stat("tmp.txt"), Err(FilesOpError::NotFound(_))));
    }

    #[test]
    fn delete_missing_fails() {
        let (op, _dir) = make_op();
        let r = op.delete("ghost.txt");
        assert!(matches!(r, Err(FilesOpError::NotFound(_))));
    }

    #[test]
    fn path_traversal_rejected() {
        let (op, _dir) = make_op();
        let r = op.put("../escape.txt", b"x", 1);
        assert!(matches!(r, Err(FilesOpError::InvalidPath(_))));
    }

    #[test]
    fn same_content_dedups_in_cas() {
        let (op, dir) = make_op();
        op.put("a.bin", b"identical content", 1).unwrap();
        op.put("b.bin", b"identical content", 1).unwrap();
        // 1 shard, 1 blob → dédup.
        let shards: Vec<_> = std::fs::read_dir(dir.path().join("cas")).unwrap().collect();
        assert_eq!(shards.len(), 1);
        let blobs: Vec<_> = std::fs::read_dir(shards[0].as_ref().unwrap().path())
            .unwrap()
            .collect();
        assert_eq!(blobs.len(), 1);
    }
}
