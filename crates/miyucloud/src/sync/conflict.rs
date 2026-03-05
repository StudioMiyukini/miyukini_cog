//! Detection et resolution des conflits de synchronisation.
//!
//! @id: miyucloud_sync_conflict
//! @do: detect_and_resolve_sync_conflicts_no_silent_loss
//! @role: sync
//! @layer: infra
//!
//! INVARIANT CRITIQUE : jamais de perte silencieuse de donnees.
//! Quand deux horloges sont concurrentes, une copie de conflit est creee
//! (fichier separe) et le conflit est enregistre en base pour resolution
//! manuelle par l'utilisateur.

use crate::data::types::{ConflictStatus, FileEntry, SyncConflict};
use crate::errors::MiyucloudError;
use crate::sync::vector_clock::{ClockOrder, VectorClock};

/// Type de conflit detecte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictType {
    /// Les deux versions ont ete modifiees independamment (horloges concurrentes).
    ConcurrentModification,
}

/// Resolution choisie par l'utilisateur.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictResolution {
    /// Garder la version locale.
    Local,
    /// Garder la version distante.
    Remote,
    /// Garder les deux versions.
    Both,
}

/// Detecteur et resolveur de conflits de synchronisation.
///
/// Utilise les horloges vectorielles pour detecter les modifications
/// concurrentes et gere la creation de copies de conflit.
pub struct ConflictResolver;

impl ConflictResolver {
    /// Detecte un conflit entre deux horloges vectorielles.
    ///
    /// Retourne `Some(ConflictType::ConcurrentModification)` si les horloges
    /// sont concurrentes, `None` si l'une est strictement avant/apres l'autre
    /// ou si elles sont egales.
    pub fn detect(local_clock: &VectorClock, remote_clock: &VectorClock) -> Option<ConflictType> {
        match local_clock.compare(remote_clock) {
            ClockOrder::Concurrent => Some(ConflictType::ConcurrentModification),
            ClockOrder::Equal | ClockOrder::Before | ClockOrder::After => None,
        }
    }

    /// Cree une copie de conflit pour un fichier.
    ///
    /// Le fichier de conflit est nomme `filename.conflict-{node_id_court}-{timestamp}.ext`.
    /// Le conflit est enregistre avec le statut `Pending`.
    ///
    /// INVARIANT : cette methode ne supprime jamais de donnees. Elle cree
    /// toujours une copie supplementaire.
    pub fn create_conflict_copy(file_entry: &FileEntry, remote_node_id: &str) -> SyncConflict {
        let now = chrono::Utc::now().to_rfc3339();
        let conflict_id = uuid::Uuid::new_v4().to_string();

        // Build conflict filename: "name.conflict-{node_short}-{ts}.ext"
        let node_short = if remote_node_id.len() > 8 {
            &remote_node_id[..8]
        } else {
            remote_node_id
        };
        let ts = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
        let conflict_name = build_conflict_filename(&file_entry.name, node_short, &ts);

        SyncConflict {
            id: conflict_id,
            file_id: file_entry.id.clone(),
            local_version: file_entry.name.clone(),
            remote_version: conflict_name,
            remote_node_id: remote_node_id.to_string(),
            status: ConflictStatus::Pending,
            created_at: now,
            resolved_at: None,
        }
    }

    /// Resout un conflit en base.
    ///
    /// La resolution applique le choix de l'utilisateur :
    /// - `Local` : garde la version locale, marque le conflit comme `ResolvedLocal`.
    /// - `Remote` : garde la version distante, marque comme `ResolvedRemote`.
    /// - `Both` : les deux versions sont conservees, marque comme `ResolvedBoth`.
    #[cfg(feature = "legacy-sqlite")]
    pub fn resolve(
        db: &crate::data::MiyucloudDb,
        conflict_id: &str,
        resolution: ConflictResolution,
    ) -> Result<(), MiyucloudError> {
        let status = match resolution {
            ConflictResolution::Local => ConflictStatus::ResolvedLocal,
            ConflictResolution::Remote => ConflictStatus::ResolvedRemote,
            ConflictResolution::Both => ConflictStatus::ResolvedBoth,
        };
        db.sync_conflict_resolve(conflict_id, status)
    }
}

/// Construit le nom du fichier de conflit.
///
/// Exemple : "photo.jpg" -> "photo.conflict-abc12345-20260301120000.jpg"
fn build_conflict_filename(original_name: &str, node_short: &str, timestamp: &str) -> String {
    if let Some(dot_pos) = original_name.rfind('.') {
        let (stem, ext) = original_name.split_at(dot_pos);
        format!("{stem}.conflict-{node_short}-{timestamp}{ext}")
    } else {
        format!("{original_name}.conflict-{node_short}-{timestamp}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_file_entry(name: &str) -> FileEntry {
        let now = chrono::Utc::now().to_rfc3339();
        FileEntry {
            id: uuid::Uuid::new_v4().to_string(),
            parent_id: None,
            owner_id: "owner-1".to_string(),
            name: name.to_string(),
            mime_type: "text/plain".to_string(),
            size_bytes: 100,
            checksum_sha256: "abc123".to_string(),
            encryption_iv: None,
            chunk_count: 1,
            is_trashed: false,
            trashed_at: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    #[test]
    fn test_detect_conflict() {
        // Concurrent clocks -> conflict
        let local = VectorClock::from_entries(vec![("node-a".into(), 2), ("node-b".into(), 1)]);
        let remote = VectorClock::from_entries(vec![("node-a".into(), 1), ("node-b".into(), 2)]);
        let result = ConflictResolver::detect(&local, &remote);
        assert_eq!(result, Some(ConflictType::ConcurrentModification));
    }

    #[test]
    fn test_no_conflict_when_ordered_before() {
        let local = VectorClock::from_entries(vec![("node-a".into(), 1)]);
        let remote = VectorClock::from_entries(vec![("node-a".into(), 2)]);
        assert!(ConflictResolver::detect(&local, &remote).is_none());
    }

    #[test]
    fn test_no_conflict_when_ordered_after() {
        let local = VectorClock::from_entries(vec![("node-a".into(), 3)]);
        let remote = VectorClock::from_entries(vec![("node-a".into(), 1)]);
        assert!(ConflictResolver::detect(&local, &remote).is_none());
    }

    #[test]
    fn test_no_conflict_when_equal() {
        let local = VectorClock::from_entries(vec![("node-a".into(), 2)]);
        let remote = VectorClock::from_entries(vec![("node-a".into(), 2)]);
        assert!(ConflictResolver::detect(&local, &remote).is_none());
    }

    #[test]
    fn test_create_conflict_copy() {
        let file = make_file_entry("document.pdf");
        let conflict = ConflictResolver::create_conflict_copy(&file, "remote-node-abc12345");

        assert_eq!(conflict.file_id, file.id);
        assert_eq!(conflict.local_version, "document.pdf");
        assert!(conflict.remote_version.contains("conflict-remote-n"));
        assert!(conflict.remote_version.ends_with(".pdf"));
        assert_eq!(conflict.remote_node_id, "remote-node-abc12345");
        assert_eq!(conflict.status, ConflictStatus::Pending);
        assert!(conflict.resolved_at.is_none());
    }

    #[test]
    fn test_create_conflict_copy_no_extension() {
        let file = make_file_entry("Makefile");
        let conflict = ConflictResolver::create_conflict_copy(&file, "node-xyz");

        assert!(conflict.remote_version.starts_with("Makefile.conflict-"));
        // Without extension, the name should NOT end with a file extension
        // (it ends with the timestamp, e.g. "Makefile.conflict-node-xyz-20260301120000")
        assert!(
            !conflict.remote_version.ends_with(".txt"),
            "Should not have an extension appended"
        );
    }

    #[test]
    fn test_resolve_conflict_local() {
        let db = crate::data::MiyucloudDb::open(":memory:").unwrap();
        let file = make_file_entry("test.txt");
        db.file_create(&file).unwrap();

        let conflict = ConflictResolver::create_conflict_copy(&file, "remote-1");
        db.sync_conflict_create(&conflict).unwrap();

        ConflictResolver::resolve(&db, &conflict.id, ConflictResolution::Local).unwrap();
        let pending = db.sync_conflict_list_pending().unwrap();
        assert!(pending.is_empty());
    }

    #[test]
    fn test_resolve_conflict_remote() {
        let db = crate::data::MiyucloudDb::open(":memory:").unwrap();
        let file = make_file_entry("test.txt");
        db.file_create(&file).unwrap();

        let conflict = ConflictResolver::create_conflict_copy(&file, "remote-2");
        db.sync_conflict_create(&conflict).unwrap();

        ConflictResolver::resolve(&db, &conflict.id, ConflictResolution::Remote).unwrap();
        let pending = db.sync_conflict_list_pending().unwrap();
        assert!(pending.is_empty());
    }

    #[test]
    fn test_build_conflict_filename_with_extension() {
        let name = build_conflict_filename("photo.jpg", "abc12345", "20260301120000");
        assert_eq!(name, "photo.conflict-abc12345-20260301120000.jpg");
    }

    #[test]
    fn test_build_conflict_filename_without_extension() {
        let name = build_conflict_filename("Makefile", "abc12345", "20260301120000");
        assert_eq!(name, "Makefile.conflict-abc12345-20260301120000");
    }
}
