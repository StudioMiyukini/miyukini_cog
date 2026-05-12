//! `snapshots_op` — création, listing, rétention des snapshots.
//!
//! Implémentation en PR-3 (P3.b). Responsabilités :
//! - `create(target_id) -> SnapshotId` (détermine full/incremental, hash files,
//!   construit manifest, persiste dans table `snapshots` + CAS)
//! - `list(target_id?, limit) -> Vec<SnapshotSummary>`
//! - `apply_retention(target_id)` (politique JSON `{daily,weekly,monthly}`)
//! - `garbage_collect_orphan_blobs()` (GC du CAS après suppression de snapshots)
