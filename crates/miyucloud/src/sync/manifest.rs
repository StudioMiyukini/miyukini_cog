//! Manifeste de synchronisation et calcul de diff bidirectionnel.
//!
//! @id: miyucloud_sync_manifest
//! @do: build_sync_manifest_from_db_and_compute_diff
//! @role: sync
//! @layer: infra
//!
//! Le manifeste est la liste de tous les fichiers connus par un noeud,
//! avec leur horloge vectorielle et leur checksum. La comparaison de
//! deux manifestes (local vs distant) produit un `SyncDiff` qui indique
//! quels fichiers doivent etre telecharges, uploades ou sont en conflit.

use crate::errors::MiyucloudError;
use crate::sync::protocol::ManifestEntry;
use crate::sync::vector_clock::{ClockOrder, VectorClock};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Manifeste de synchronisation : etat local des fichiers.
pub struct SyncManifest;

/// Resultat du calcul de diff entre deux manifestes.
///
/// Contient trois listes :
/// - `to_download` : fichiers a telecharger depuis le pair distant.
/// - `to_upload` : fichiers a envoyer au pair distant.
/// - `conflicts` : fichiers avec des modifications concurrentes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SyncDiff {
    /// Fichiers a telecharger depuis le pair (absents localement ou plus recents chez le pair).
    pub to_download: Vec<ManifestEntry>,
    /// Fichiers a envoyer au pair (absents chez le pair ou plus recents localement).
    pub to_upload: Vec<ManifestEntry>,
    /// Fichiers en conflit (modifications concurrentes).
    pub conflicts: Vec<ConflictPair>,
}

/// Paire de fichiers en conflit (version locale et distante).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictPair {
    /// Version locale du fichier.
    pub local: ManifestEntry,
    /// Version distante du fichier.
    pub remote: ManifestEntry,
}

impl SyncManifest {
    /// Construit le manifeste local depuis la base de donnees.
    ///
    /// Recupere tous les fichiers non supprimes et leur horloge vectorielle,
    /// filtres par les dossiers specifies (vide = tous les fichiers).
    #[cfg(feature = "legacy-sqlite")]
    pub fn build(
        db: &crate::data::MiyucloudDb,
        owner_id: &str,
        folder_ids: &[String],
    ) -> Result<Vec<ManifestEntry>, MiyucloudError> {
        let files = if folder_ids.is_empty() {
            // Get all non-trashed files (root + all folders)
            db.file_list(None, owner_id)?
        } else {
            let mut all_files = Vec::new();
            for folder_id in folder_ids {
                let files = db.file_list(Some(folder_id), owner_id)?;
                all_files.extend(files);
            }
            all_files
        };

        let mut entries = Vec::with_capacity(files.len());
        for file in &files {
            // Build vector clock from DB
            let clock_entries = db.vector_clock_get(&file.id)?;
            let clock = VectorClock::from_entries(
                clock_entries
                    .into_iter()
                    .map(|e| (e.node_id, e.counter))
                    .collect(),
            );

            entries.push(ManifestEntry {
                file_id: file.id.clone(),
                name: file.name.clone(),
                parent_id: file.parent_id.clone(),
                checksum_sha256: file.checksum_sha256.clone(),
                size_bytes: file.size_bytes,
                clock,
                updated_at: file.updated_at.clone(),
            });
        }

        Ok(entries)
    }

    /// Calcule le diff bidirectionnel entre un manifeste local et un manifeste distant.
    ///
    /// Retourne un `SyncDiff` contenant :
    /// - `to_download` : fichiers presents chez le pair mais pas localement,
    ///   ou fichiers plus recents chez le pair (clock distant > clock local).
    /// - `to_upload` : fichiers presents localement mais pas chez le pair,
    ///   ou fichiers plus recents localement (clock local > clock distant).
    /// - `conflicts` : fichiers avec des horloges concurrentes.
    pub fn compute_diff(local: &[ManifestEntry], remote: &[ManifestEntry]) -> SyncDiff {
        let local_map: HashMap<&str, &ManifestEntry> =
            local.iter().map(|e| (e.file_id.as_str(), e)).collect();

        let remote_map: HashMap<&str, &ManifestEntry> =
            remote.iter().map(|e| (e.file_id.as_str(), e)).collect();

        let mut diff = SyncDiff::default();

        // Check remote files against local
        for (file_id, remote_entry) in &remote_map {
            match local_map.get(file_id) {
                None => {
                    // File only exists remotely -> download
                    diff.to_download.push((*remote_entry).clone());
                }
                Some(local_entry) => {
                    // File exists in both -> compare clocks
                    match local_entry.clock.compare(&remote_entry.clock) {
                        ClockOrder::Equal => {
                            // Same version, nothing to do
                            // But verify checksums match (integrity check)
                            if local_entry.checksum_sha256 != remote_entry.checksum_sha256 {
                                // Checksums differ despite equal clocks -> conflict
                                diff.conflicts.push(ConflictPair {
                                    local: (*local_entry).clone(),
                                    remote: (*remote_entry).clone(),
                                });
                            }
                        }
                        ClockOrder::Before => {
                            // Local is older -> download from remote
                            diff.to_download.push((*remote_entry).clone());
                        }
                        ClockOrder::After => {
                            // Local is newer -> upload to remote
                            diff.to_upload.push((*local_entry).clone());
                        }
                        ClockOrder::Concurrent => {
                            // Concurrent modifications -> CONFLICT
                            diff.conflicts.push(ConflictPair {
                                local: (*local_entry).clone(),
                                remote: (*remote_entry).clone(),
                            });
                        }
                    }
                }
            }
        }

        // Check local files not present remotely -> upload
        for (file_id, local_entry) in &local_map {
            if !remote_map.contains_key(file_id) {
                diff.to_upload.push((*local_entry).clone());
            }
        }

        diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(file_id: &str, checksum: &str, clock: VectorClock) -> ManifestEntry {
        ManifestEntry {
            file_id: file_id.into(),
            name: format!("{file_id}.txt"),
            parent_id: None,
            checksum_sha256: checksum.into(),
            size_bytes: 100,
            clock,
            updated_at: "2026-03-01T00:00:00Z".into(),
        }
    }

    #[test]
    fn test_build_manifest_from_db() {
        let db = crate::data::MiyucloudDb::open(":memory:").unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        let file = crate::data::types::FileEntry {
            id: "file-1".into(),
            parent_id: None,
            owner_id: "owner-1".into(),
            name: "test.txt".into(),
            mime_type: "text/plain".into(),
            size_bytes: 100,
            checksum_sha256: "abc123".into(),
            encryption_iv: None,
            chunk_count: 1,
            is_trashed: false,
            trashed_at: None,
            created_at: now.clone(),
            updated_at: now,
        };
        db.file_create(&file).unwrap();
        db.vector_clock_update("file-1", "node-a", 3).unwrap();

        let manifest = SyncManifest::build(&db, "owner-1", &[]).unwrap();
        assert_eq!(manifest.len(), 1);
        assert_eq!(manifest[0].file_id, "file-1");
        assert_eq!(manifest[0].clock.get("node-a"), 3);
    }

    #[test]
    fn test_compute_diff_new_files() {
        let local: Vec<ManifestEntry> = vec![];
        let remote = vec![
            make_entry(
                "file-1",
                "abc",
                VectorClock::from_entries(vec![("node-b".into(), 1)]),
            ),
            make_entry(
                "file-2",
                "def",
                VectorClock::from_entries(vec![("node-b".into(), 1)]),
            ),
        ];

        let diff = SyncManifest::compute_diff(&local, &remote);
        assert_eq!(diff.to_download.len(), 2);
        assert_eq!(diff.to_upload.len(), 0);
        assert_eq!(diff.conflicts.len(), 0);
    }

    #[test]
    fn test_compute_diff_local_only_files() {
        let local = vec![make_entry(
            "file-1",
            "abc",
            VectorClock::from_entries(vec![("node-a".into(), 1)]),
        )];
        let remote: Vec<ManifestEntry> = vec![];

        let diff = SyncManifest::compute_diff(&local, &remote);
        assert_eq!(diff.to_download.len(), 0);
        assert_eq!(diff.to_upload.len(), 1);
        assert_eq!(diff.to_upload[0].file_id, "file-1");
    }

    #[test]
    fn test_compute_diff_updated_files() {
        // Local clock is older than remote -> download
        let local = vec![make_entry(
            "file-1",
            "abc",
            VectorClock::from_entries(vec![("node-a".into(), 1)]),
        )];
        let remote = vec![make_entry(
            "file-1",
            "def",
            VectorClock::from_entries(vec![("node-a".into(), 2)]),
        )];

        let diff = SyncManifest::compute_diff(&local, &remote);
        assert_eq!(diff.to_download.len(), 1);
        assert_eq!(diff.to_upload.len(), 0);
        assert_eq!(diff.conflicts.len(), 0);
    }

    #[test]
    fn test_compute_diff_local_newer() {
        // Local clock is newer -> upload
        let local = vec![make_entry(
            "file-1",
            "xyz",
            VectorClock::from_entries(vec![("node-a".into(), 5)]),
        )];
        let remote = vec![make_entry(
            "file-1",
            "abc",
            VectorClock::from_entries(vec![("node-a".into(), 2)]),
        )];

        let diff = SyncManifest::compute_diff(&local, &remote);
        assert_eq!(diff.to_download.len(), 0);
        assert_eq!(diff.to_upload.len(), 1);
        assert_eq!(diff.conflicts.len(), 0);
    }

    #[test]
    fn test_compute_diff_no_changes() {
        let clock = VectorClock::from_entries(vec![("node-a".into(), 2)]);
        let local = vec![make_entry("file-1", "abc", clock.clone())];
        let remote = vec![make_entry("file-1", "abc", clock)];

        let diff = SyncManifest::compute_diff(&local, &remote);
        assert_eq!(diff.to_download.len(), 0);
        assert_eq!(diff.to_upload.len(), 0);
        assert_eq!(diff.conflicts.len(), 0);
    }

    #[test]
    fn test_compute_diff_conflicts() {
        // Concurrent clocks -> conflict
        let local = vec![make_entry(
            "file-1",
            "abc",
            VectorClock::from_entries(vec![("node-a".into(), 2), ("node-b".into(), 1)]),
        )];
        let remote = vec![make_entry(
            "file-1",
            "def",
            VectorClock::from_entries(vec![("node-a".into(), 1), ("node-b".into(), 2)]),
        )];

        let diff = SyncManifest::compute_diff(&local, &remote);
        assert_eq!(diff.to_download.len(), 0);
        assert_eq!(diff.to_upload.len(), 0);
        assert_eq!(diff.conflicts.len(), 1);
        assert_eq!(diff.conflicts[0].local.file_id, "file-1");
    }

    #[test]
    fn test_compute_diff_mixed() {
        let local = vec![
            // Only local
            make_entry(
                "file-local",
                "aaa",
                VectorClock::from_entries(vec![("node-a".into(), 1)]),
            ),
            // Same on both
            make_entry(
                "file-same",
                "bbb",
                VectorClock::from_entries(vec![("node-a".into(), 2)]),
            ),
            // Conflict
            make_entry(
                "file-conflict",
                "ccc",
                VectorClock::from_entries(vec![("node-a".into(), 2), ("node-b".into(), 1)]),
            ),
        ];
        let remote = vec![
            // Only remote
            make_entry(
                "file-remote",
                "ddd",
                VectorClock::from_entries(vec![("node-b".into(), 1)]),
            ),
            // Same on both
            make_entry(
                "file-same",
                "bbb",
                VectorClock::from_entries(vec![("node-a".into(), 2)]),
            ),
            // Conflict
            make_entry(
                "file-conflict",
                "eee",
                VectorClock::from_entries(vec![("node-a".into(), 1), ("node-b".into(), 2)]),
            ),
        ];

        let diff = SyncManifest::compute_diff(&local, &remote);
        assert_eq!(diff.to_download.len(), 1); // file-remote
        assert_eq!(diff.to_upload.len(), 1); // file-local
        assert_eq!(diff.conflicts.len(), 1); // file-conflict
    }
}
