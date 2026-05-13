//! `snapshots_op` — création, listing, rétention des snapshots.
//!
//! Un snapshot est un instantané d'un `FilesOp` à un moment donné. Il est
//! matérialisé par un **manifest** JSON sérialisé dans le CAS (lui-même
//! chiffré + zstd via `cas_kit`).
//!
//! Chaque entrée du manifest pointe vers un `BlobHash` dans le CAS, ce qui
//! permet la **déduplication entre snapshots** : si deux snapshots
//! consécutifs partagent un fichier identique (mêmes octets), le blob
//! n'est stocké qu'une fois.
//!
//! ## Modèle full / incremental
//! - Premier snapshot d'une cible → `kind = Full`, `parent_id = None`.
//! - Suivants → `kind = Incremental`, `parent_id = <id du précédent>`.
//! - Le **manifest reste self-contained** dans tous les cas (liste toutes
//!   les entrées). Le couple `(kind, parent_id)` sert à reconstruire
//!   l'histoire et à appliquer la politique de rétention.
//!
//! ## Rétention
//! Politique JSON `{daily, weekly, monthly}` : on garde au plus N snapshots
//! par bucket calendaire (UTC). Les snapshots non gardés sont supprimés ;
//! `garbage_collect_orphans()` nettoie ensuite les blobs CAS plus
//! référencés.

use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use std::sync::RwLock;

use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::kits::cas_kit::{self, BlobHash, CasConfig, CasKitError};
use crate::kits::crypto_kit::Key32;
use crate::operators::files_op::{FilesOp, FilesOpError};

/// Type de snapshot.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotKind {
    /// Snapshot complet (pas de parent).
    Full,
    /// Snapshot incrémental (avec `parent_id` pointant vers le précédent du même target).
    Incremental,
}

/// État d'un snapshot.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotStatus {
    /// Création en cours.
    InProgress,
    /// Snapshot complet, intégrité validée.
    Complete,
    /// Échec pendant la création.
    Failed,
    /// Corruption détectée lors d'une vérification d'intégrité.
    Corrupted,
}

/// Résumé d'un snapshot (vue listing).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotSummary {
    /// Identifiant stable.
    pub id: String,
    /// Cible de backup.
    pub target_id: String,
    /// Type.
    pub kind: SnapshotKind,
    /// Snapshot parent (None pour Full).
    pub parent_id: Option<String>,
    /// Création en RFC 3339.
    pub created_at: String,
    /// Nombre d'entrées (fichiers + dossiers).
    pub files_count: u64,
    /// Somme des tailles plaintext (avant dédup CAS).
    pub size_bytes: u64,
    /// Hash du blob de manifest dans le CAS.
    pub manifest_blob: BlobHash,
    /// État.
    pub status: SnapshotStatus,
}

/// Entrée d'un manifest (un fichier ou un dossier).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ManifestEntry {
    /// Chemin canonique relatif.
    pub path: String,
    /// Hash CAS du contenu. `None` pour un dossier.
    pub blob_hash: Option<BlobHash>,
    /// Taille en octets (None pour dossier).
    pub size_bytes: Option<u64>,
    /// Timestamp Unix de la dernière modification.
    pub mtime: i64,
    /// `true` si dossier.
    pub is_dir: bool,
}

/// Manifest complet d'un snapshot. Sérialisé en JSON puis stocké dans le CAS.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Manifest {
    /// ID du snapshot propriétaire.
    pub snapshot_id: String,
    /// Cible.
    pub target_id: String,
    /// Date de création (RFC 3339).
    pub created_at: String,
    /// Type.
    pub kind: SnapshotKind,
    /// Snapshot parent.
    pub parent_id: Option<String>,
    /// Toutes les entrées du snapshot (self-contained).
    pub files: Vec<ManifestEntry>,
}

/// Politique de rétention.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct RetentionPolicy {
    /// N derniers snapshots quotidiens.
    pub daily: u32,
    /// N derniers snapshots hebdomadaires (lundi = début de semaine ISO).
    pub weekly: u32,
    /// N derniers snapshots mensuels.
    pub monthly: u32,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            daily: 7,
            weekly: 4,
            monthly: 12,
        }
    }
}

/// Erreurs de l'Opérateur Snapshots.
#[derive(Debug, thiserror::Error)]
pub enum SnapshotsOpError {
    /// Snapshot introuvable.
    #[error("snapshot introuvable : {0}")]
    NotFound(String),
    /// Erreur CAS.
    #[error("cas : {0}")]
    Cas(#[from] CasKitError),
    /// Erreur FilesOp.
    #[error("files : {0}")]
    Files(#[from] FilesOpError),
    /// Erreur de sérialisation JSON du manifest.
    #[error("manifest json : {0}")]
    ManifestSerialization(String),
    /// Date de création de snapshot non parseable.
    #[error("date invalide : {0}")]
    InvalidDate(String),
}

/// Opérateur Snapshots.
pub struct SnapshotsOp {
    /// Index `snapshot_id → résumé`.
    snapshots: RwLock<BTreeMap<String, SnapshotSummary>>,
    /// Racine disque du CAS partagée avec `FilesOp`.
    cas_root: PathBuf,
    /// Clé de chiffrement CAS partagée.
    cas_key: Key32,
    /// Configuration CAS.
    cas_config: CasConfig,
}

impl SnapshotsOp {
    /// Construit un opérateur. Doit partager la même racine CAS / clé / config
    /// que le `FilesOp` qu'il snapshotte (cohérence indispensable).
    #[must_use]
    pub fn new(cas_root: PathBuf, cas_key: Key32, cas_config: CasConfig) -> Self {
        Self {
            snapshots: RwLock::new(BTreeMap::new()),
            cas_root,
            cas_key,
            cas_config,
        }
    }

    /// Crée un snapshot du `FilesOp` fourni pour la cible `target_id`.
    ///
    /// Le type (`Full` ou `Incremental`) est déterminé automatiquement :
    /// `Full` si c'est le premier snapshot de la cible, `Incremental` sinon.
    /// `created_at_unix` permet d'injecter l'horloge pour les tests.
    pub fn create(
        &self,
        target_id: &str,
        files: &FilesOp,
        created_at_unix: i64,
    ) -> Result<SnapshotSummary, SnapshotsOpError> {
        // Détermine le parent : dernier snapshot Complete de cette cible.
        let parent_id = self.last_complete_snapshot_id(target_id);
        let kind = if parent_id.is_none() {
            SnapshotKind::Full
        } else {
            SnapshotKind::Incremental
        };

        // Construit le manifest depuis le FilesOp.
        let entries: Vec<ManifestEntry> = files
            .list()
            .into_iter()
            .map(|e| ManifestEntry {
                path: e.path,
                blob_hash: e.blob_hash,
                size_bytes: e.size_bytes,
                mtime: e.mtime,
                is_dir: e.is_dir,
            })
            .collect();

        let snapshot_id = format!("snap_{target_id}_{created_at_unix}");
        let created_at = Utc
            .timestamp_opt(created_at_unix, 0)
            .single()
            .ok_or_else(|| SnapshotsOpError::InvalidDate(format!("ts={created_at_unix}")))?
            .to_rfc3339();

        let manifest = Manifest {
            snapshot_id: snapshot_id.clone(),
            target_id: target_id.to_string(),
            created_at: created_at.clone(),
            kind,
            parent_id: parent_id.clone(),
            files: entries.clone(),
        };

        // Stocke le manifest dans le CAS (json sérialisé).
        let manifest_json = serde_json::to_vec(&manifest)
            .map_err(|e| SnapshotsOpError::ManifestSerialization(e.to_string()))?;
        let manifest_blob = cas_kit::store(
            &manifest_json,
            &self.cas_key,
            &self.cas_root,
            &self.cas_config,
        )?;

        // Calcule les stats.
        let files_count = entries.len() as u64;
        let size_bytes: u64 = entries
            .iter()
            .map(|e| e.size_bytes.unwrap_or(0))
            .sum();

        let summary = SnapshotSummary {
            id: snapshot_id.clone(),
            target_id: target_id.to_string(),
            kind,
            parent_id,
            created_at,
            files_count,
            size_bytes,
            manifest_blob,
            status: SnapshotStatus::Complete,
        };

        self.snapshots
            .write()
            .unwrap()
            .insert(snapshot_id, summary.clone());

        Ok(summary)
    }

    /// Liste les snapshots, filtrés optionnellement par cible, triés par
    /// date décroissante (plus récent d'abord).
    pub fn list(&self, target_id: Option<&str>, limit: usize) -> Vec<SnapshotSummary> {
        let snapshots = self.snapshots.read().unwrap();
        let mut out: Vec<_> = snapshots
            .values()
            .filter(|s| target_id.map_or(true, |t| s.target_id == t))
            .cloned()
            .collect();
        out.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        out.truncate(limit);
        out
    }

    /// Récupère le résumé d'un snapshot par ID.
    pub fn get(&self, snapshot_id: &str) -> Result<SnapshotSummary, SnapshotsOpError> {
        self.snapshots
            .read()
            .unwrap()
            .get(snapshot_id)
            .cloned()
            .ok_or_else(|| SnapshotsOpError::NotFound(snapshot_id.to_string()))
    }

    /// Lit le manifest d'un snapshot.
    pub fn read_manifest(&self, snapshot_id: &str) -> Result<Manifest, SnapshotsOpError> {
        let summary = self.get(snapshot_id)?;
        let bytes = cas_kit::read(&summary.manifest_blob, &self.cas_key, &self.cas_root)?;
        let manifest: Manifest = serde_json::from_slice(&bytes)
            .map_err(|e| SnapshotsOpError::ManifestSerialization(e.to_string()))?;
        Ok(manifest)
    }

    /// Applique la politique de rétention à une cible. Renvoie le nombre de
    /// snapshots supprimés (les blobs orphelins sont libérés par
    /// `garbage_collect_orphans` à appeler ensuite).
    pub fn apply_retention(
        &self,
        target_id: &str,
        policy: RetentionPolicy,
    ) -> Result<usize, SnapshotsOpError> {
        let all = self.list(Some(target_id), usize::MAX);
        let to_keep = compute_kept_snapshots(&all, policy)?;
        let mut snapshots = self.snapshots.write().unwrap();
        let before = snapshots.len();
        snapshots.retain(|id, s| s.target_id != target_id || to_keep.contains(id));
        Ok(before - snapshots.len())
    }

    /// Garbage-collect les blobs CAS plus référencés par aucun snapshot
    /// vivant (ni en données, ni en manifest). Renvoie le nombre de blobs
    /// supprimés.
    pub fn garbage_collect_orphans(&self) -> Result<usize, SnapshotsOpError> {
        let mut referenced: HashSet<BlobHash> = HashSet::new();

        // 1. Tous les manifests sont référencés.
        let snapshot_ids: Vec<_> = {
            let snapshots = self.snapshots.read().unwrap();
            snapshots.keys().cloned().collect()
        };
        for id in &snapshot_ids {
            let summary = self.get(id)?;
            referenced.insert(summary.manifest_blob.clone());
            let manifest = self.read_manifest(id)?;
            for entry in &manifest.files {
                if let Some(h) = &entry.blob_hash {
                    referenced.insert(h.clone());
                }
            }
        }

        Ok(cas_kit::delete_orphans(&referenced, &self.cas_root)?)
    }

    /// Helper interne : ID du dernier snapshot Complete d'une cible.
    fn last_complete_snapshot_id(&self, target_id: &str) -> Option<String> {
        self.list(Some(target_id), 1)
            .into_iter()
            .find(|s| s.status == SnapshotStatus::Complete)
            .map(|s| s.id)
    }
}

/// Calcule les snapshots à conserver selon la politique.
///
/// Pour chaque bucket calendaire (daily, weekly, monthly), on garde le plus
/// récent. On garde aussi le plus récent globalement (always-keep).
fn compute_kept_snapshots(
    all_desc: &[SnapshotSummary],
    policy: RetentionPolicy,
) -> Result<HashSet<String>, SnapshotsOpError> {
    let mut kept: HashSet<String> = HashSet::new();
    if all_desc.is_empty() {
        return Ok(kept);
    }

    // Garde toujours le plus récent.
    kept.insert(all_desc[0].id.clone());

    let mut daily_buckets: BTreeMap<NaiveDate, &SnapshotSummary> = BTreeMap::new();
    let mut weekly_buckets: BTreeMap<(i32, u32), &SnapshotSummary> = BTreeMap::new(); // (year, iso_week)
    let mut monthly_buckets: BTreeMap<(i32, u32), &SnapshotSummary> = BTreeMap::new(); // (year, month)

    for s in all_desc {
        let dt = DateTime::parse_from_rfc3339(&s.created_at)
            .map_err(|e| SnapshotsOpError::InvalidDate(format!("{}: {e}", s.created_at)))?
            .with_timezone(&Utc);
        let date = dt.date_naive();
        let iso = date.iso_week();

        // Pour chaque bucket, on ne garde que le PREMIER vu (qui est le plus récent car all_desc trié desc).
        daily_buckets.entry(date).or_insert(s);
        weekly_buckets
            .entry((iso.year(), iso.week()))
            .or_insert(s);
        monthly_buckets
            .entry((date.year(), date.month()))
            .or_insert(s);
    }

    // Prend les N derniers (par clé décroissante) de chaque bucket.
    fn take_recent_n<K: Ord>(bucket: BTreeMap<K, &SnapshotSummary>, n: u32) -> Vec<String> {
        bucket
            .into_iter()
            .rev()
            .take(n as usize)
            .map(|(_, s)| s.id.clone())
            .collect()
    }

    for id in take_recent_n(daily_buckets, policy.daily) {
        kept.insert(id);
    }
    for id in take_recent_n(weekly_buckets, policy.weekly) {
        kept.insert(id);
    }
    for id in take_recent_n(monthly_buckets, policy.monthly) {
        kept.insert(id);
    }

    Ok(kept)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operators::files_op::FilesOp;

    fn make_ops() -> (FilesOp, SnapshotsOp, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let cas_root = dir.path().join("cas");
        let key = Key32::generate();
        let cfg = CasConfig::default();
        let files = FilesOp::new(cas_root.clone(), key.clone(), cfg.clone());
        let snaps = SnapshotsOp::new(cas_root, key, cfg);
        (files, snaps, dir)
    }

    #[test]
    fn first_snapshot_is_full() {
        let (files, snaps, _dir) = make_ops();
        files.put("a.txt", b"hello", 1).unwrap();
        let s = snaps.create("target_1", &files, 1_700_000_000).unwrap();
        assert_eq!(s.kind, SnapshotKind::Full);
        assert!(s.parent_id.is_none());
        assert_eq!(s.target_id, "target_1");
        assert_eq!(s.status, SnapshotStatus::Complete);
        assert_eq!(s.files_count, 1);
    }

    #[test]
    fn second_snapshot_is_incremental_with_parent() {
        let (files, snaps, _dir) = make_ops();
        files.put("a.txt", b"hello", 1).unwrap();
        let s1 = snaps.create("t", &files, 1_700_000_000).unwrap();
        files.put("b.txt", b"world", 2).unwrap();
        let s2 = snaps.create("t", &files, 1_700_000_100).unwrap();
        assert_eq!(s2.kind, SnapshotKind::Incremental);
        assert_eq!(s2.parent_id, Some(s1.id.clone()));
        assert_eq!(s2.files_count, 2);
    }

    #[test]
    fn first_snapshot_per_target_is_full_even_after_other_target() {
        let (files, snaps, _dir) = make_ops();
        files.put("a.txt", b"x", 1).unwrap();
        snaps.create("t1", &files, 1_700_000_000).unwrap();
        let s = snaps.create("t2", &files, 1_700_000_100).unwrap();
        assert_eq!(s.kind, SnapshotKind::Full);
        assert!(s.parent_id.is_none());
    }

    #[test]
    fn manifest_roundtrip() {
        let (files, snaps, _dir) = make_ops();
        files.put("a.txt", b"hello", 1).unwrap();
        files.put("dir/b.txt", b"world", 2).unwrap();
        let s = snaps.create("t", &files, 1_700_000_000).unwrap();

        let manifest = snaps.read_manifest(&s.id).unwrap();
        assert_eq!(manifest.snapshot_id, s.id);
        assert_eq!(manifest.target_id, "t");
        assert_eq!(manifest.files.len(), 2);
        let paths: Vec<_> = manifest.files.iter().map(|e| e.path.clone()).collect();
        assert!(paths.contains(&"a.txt".to_string()));
        assert!(paths.contains(&"dir/b.txt".to_string()));
    }

    #[test]
    fn list_sorted_desc_by_date() {
        let (files, snaps, _dir) = make_ops();
        files.put("a.txt", b"x", 1).unwrap();
        let s1 = snaps.create("t", &files, 1_700_000_000).unwrap();
        let s2 = snaps.create("t", &files, 1_700_000_500).unwrap();
        let s3 = snaps.create("t", &files, 1_700_000_300).unwrap();
        let list = snaps.list(Some("t"), 10);
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].id, s2.id); // plus récent
        assert_eq!(list[1].id, s3.id);
        assert_eq!(list[2].id, s1.id);
    }

    #[test]
    fn list_filters_by_target() {
        let (files, snaps, _dir) = make_ops();
        files.put("a.txt", b"x", 1).unwrap();
        snaps.create("t1", &files, 1_700_000_000).unwrap();
        snaps.create("t2", &files, 1_700_000_100).unwrap();
        snaps.create("t1", &files, 1_700_000_200).unwrap();
        assert_eq!(snaps.list(Some("t1"), 10).len(), 2);
        assert_eq!(snaps.list(Some("t2"), 10).len(), 1);
        assert_eq!(snaps.list(None, 10).len(), 3);
    }

    #[test]
    fn list_limit_respected() {
        let (files, snaps, _dir) = make_ops();
        files.put("a.txt", b"x", 1).unwrap();
        for ts in 1..=5 {
            snaps.create("t", &files, 1_700_000_000 + ts).unwrap();
        }
        assert_eq!(snaps.list(Some("t"), 3).len(), 3);
    }

    #[test]
    fn get_unknown_fails() {
        let (_files, snaps, _dir) = make_ops();
        let r = snaps.get("ghost");
        assert!(matches!(r, Err(SnapshotsOpError::NotFound(_))));
    }

    #[test]
    fn retention_keeps_most_recent_globally() {
        let (files, snaps, _dir) = make_ops();
        files.put("a.txt", b"x", 1).unwrap();
        // 1 snapshot par jour sur 10 jours.
        for d in 0..10 {
            let ts = 1_700_000_000 + d * 86400;
            snaps.create("t", &files, ts).unwrap();
        }
        let policy = RetentionPolicy {
            daily: 3,
            weekly: 0,
            monthly: 0,
        };
        let removed = snaps.apply_retention("t", policy).unwrap();
        // 10 snapshots, daily=3 + le plus récent (déjà dans daily) → 3 gardés.
        let kept = snaps.list(Some("t"), 100);
        assert_eq!(kept.len(), 3);
        assert!(removed > 0);
    }

    #[test]
    fn retention_with_zero_policy_keeps_only_latest() {
        let (files, snaps, _dir) = make_ops();
        files.put("a.txt", b"x", 1).unwrap();
        for d in 0..5 {
            let ts = 1_700_000_000 + d * 86400;
            snaps.create("t", &files, ts).unwrap();
        }
        let policy = RetentionPolicy {
            daily: 0,
            weekly: 0,
            monthly: 0,
        };
        snaps.apply_retention("t", policy).unwrap();
        // always-keep le plus récent.
        let kept = snaps.list(Some("t"), 100);
        assert_eq!(kept.len(), 1);
    }

    #[test]
    fn retention_does_not_affect_other_targets() {
        let (files, snaps, _dir) = make_ops();
        files.put("a.txt", b"x", 1).unwrap();
        for d in 0..3 {
            let ts = 1_700_000_000 + d * 86400;
            snaps.create("t1", &files, ts).unwrap();
            snaps.create("t2", &files, ts).unwrap();
        }
        let policy = RetentionPolicy {
            daily: 0,
            weekly: 0,
            monthly: 0,
        };
        snaps.apply_retention("t1", policy).unwrap();
        assert_eq!(snaps.list(Some("t1"), 100).len(), 1);
        assert_eq!(snaps.list(Some("t2"), 100).len(), 3);
    }

    #[test]
    fn gc_orphans_keeps_referenced_blobs() {
        let (files, snaps, _dir) = make_ops();
        files.put("a.txt", b"keep", 1).unwrap();
        snaps.create("t", &files, 1_700_000_000).unwrap();
        // GC ne devrait rien supprimer.
        let removed = snaps.garbage_collect_orphans().unwrap();
        assert_eq!(removed, 0);
    }

    #[test]
    fn gc_orphans_removes_unreferenced_after_retention() {
        let (files, snaps, _dir) = make_ops();
        files.put("unique_a.txt", b"unique content A", 1).unwrap();
        let s1 = snaps.create("t", &files, 1_700_000_000).unwrap();

        // Modifie le fichier → nouveau blob (le précédent ne sera référencé
        // que par s1).
        files.put("unique_a.txt", b"unique content B", 2).unwrap();
        let s2 = snaps.create("t", &files, 1_700_000_100).unwrap();
        assert_ne!(s1.id, s2.id);

        // Politique zéro → on supprime tout sauf le dernier.
        snaps.apply_retention(
            "t",
            RetentionPolicy {
                daily: 0,
                weekly: 0,
                monthly: 0,
            },
        )
        .unwrap();
        let kept = snaps.list(Some("t"), 100);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].id, s2.id);

        // GC : doit retirer le blob de "unique content A" + le manifest de s1.
        let removed = snaps.garbage_collect_orphans().unwrap();
        assert!(removed >= 2, "GC devait retirer >= 2 blobs, retiré {removed}");
    }
}
