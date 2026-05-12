//! `restore_op` — récupération depuis un snapshot.
//!
//! Implémentation en PR-3 (P3.b). Responsabilités :
//! - `restore_file(snapshot_id, file_path, destination?)` — récupère un
//!   fichier précis
//! - `restore_snapshot(snapshot_id, destination)` — récupération complète
//! - Vérification d'intégrité SHA-256 systématique pendant la restauration
