//! API REST interne du service JayCloud.
//!
//! En P3.b, ce module expose la **logique** des handlers REST sous forme de
//! méthodes async sur `BackupApi`. Le **binding HTTP** (axum routes) sera
//! ajouté en PR-4 quand le portail / WebDAV arrive. Cette séparation
//! permet de tester chaque endpoint sans démarrer de serveur.
//!
//! Endpoints couverts :
//! - `trigger_backup(target_id, files_op) → SnapshotSummary`
//! - `list_snapshots(target_id?, limit) → Vec<SnapshotSummary>`
//! - `restore_file(snapshot_id, file_path) → Vec<u8>`
//! - `create_share(...)` → délégation à `share_op`
//! - `list_shares(owner)` → liste des liens de l'utilisateur
//! - `revoke_share(token)` → suppression
//!
//! Authentification, scoping, rate-limit etc. seront ajoutés en PR-5 au
//! moment du binding axum.

use std::sync::Arc;

use crate::operators::{
    files_op::FilesOp,
    restore_op::{RestoreOp, RestoreOpError},
    share_op::{ShareLink, ShareOp, ShareOpError},
    snapshots_op::{RetentionPolicy, SnapshotSummary, SnapshotsOp, SnapshotsOpError},
};

/// Erreurs unifiées de l'API.
#[derive(Debug, thiserror::Error)]
pub enum BackupApiError {
    /// Erreur de l'opérateur Snapshots.
    #[error("snapshots : {0}")]
    Snapshots(#[from] SnapshotsOpError),
    /// Erreur de l'opérateur Restore.
    #[error("restore : {0}")]
    Restore(#[from] RestoreOpError),
    /// Erreur de l'opérateur Share.
    #[error("share : {0}")]
    Share(#[from] ShareOpError),
    /// Paramètre d'API invalide.
    #[error("paramètre invalide : {0}")]
    InvalidParam(&'static str),
}

/// Façade qui agrège les Opérateurs JayCloud.
///
/// `Arc` partout pour permettre une consommation concurrente depuis axum
/// en PR-4 (chaque request handler clone l'Arc).
pub struct BackupApi {
    /// Opérateur snapshots (création + listing + rétention + GC).
    pub snapshots: Arc<SnapshotsOp>,
    /// Opérateur restore.
    pub restore: Arc<RestoreOp>,
    /// Opérateur share.
    pub shares: Arc<ShareOp>,
}

impl BackupApi {
    /// Construit l'API autour des trois opérateurs.
    #[must_use]
    pub fn new(snapshots: Arc<SnapshotsOp>, restore: Arc<RestoreOp>, shares: Arc<ShareOp>) -> Self {
        Self {
            snapshots,
            restore,
            shares,
        }
    }

    /// Déclenche un backup : crée un snapshot du `FilesOp` pour `target_id`.
    pub fn trigger_backup(
        &self,
        target_id: &str,
        files: &FilesOp,
        now_unix: i64,
    ) -> Result<SnapshotSummary, BackupApiError> {
        if target_id.trim().is_empty() {
            return Err(BackupApiError::InvalidParam("target_id vide"));
        }
        Ok(self.snapshots.create(target_id, files, now_unix)?)
    }

    /// Liste les snapshots (filtrés par cible optionnellement, plus récent
    /// d'abord, limite configurable).
    pub fn list_snapshots(
        &self,
        target_id: Option<&str>,
        limit: usize,
    ) -> Result<Vec<SnapshotSummary>, BackupApiError> {
        if limit == 0 {
            return Err(BackupApiError::InvalidParam("limit doit être > 0"));
        }
        Ok(self.snapshots.list(target_id, limit))
    }

    /// Restaure un fichier en mémoire (bytes).
    pub fn restore_file_bytes(
        &self,
        snapshot_id: &str,
        file_path: &str,
    ) -> Result<Vec<u8>, BackupApiError> {
        if snapshot_id.trim().is_empty() {
            return Err(BackupApiError::InvalidParam("snapshot_id vide"));
        }
        if file_path.trim().is_empty() {
            return Err(BackupApiError::InvalidParam("file_path vide"));
        }
        Ok(self
            .restore
            .restore_file_bytes(&self.snapshots, snapshot_id, file_path)?)
    }

    /// Applique la politique de rétention sur une cible. Renvoie le nombre
    /// de snapshots supprimés.
    pub fn apply_retention(
        &self,
        target_id: &str,
        policy: RetentionPolicy,
    ) -> Result<usize, BackupApiError> {
        if target_id.trim().is_empty() {
            return Err(BackupApiError::InvalidParam("target_id vide"));
        }
        Ok(self.snapshots.apply_retention(target_id, policy)?)
    }

    /// Garbage-collect les blobs CAS orphelins. Renvoie le nombre supprimé.
    pub fn garbage_collect_orphans(&self) -> Result<usize, BackupApiError> {
        Ok(self.snapshots.garbage_collect_orphans()?)
    }

    /// Crée un lien public.
    pub fn create_share(
        &self,
        owner_user_id: &str,
        snapshot_id: &str,
        resource_path: Option<&str>,
        expires_at: Option<i64>,
        password: Option<&str>,
        now_unix: i64,
    ) -> Result<ShareLink, BackupApiError> {
        if owner_user_id.trim().is_empty() {
            return Err(BackupApiError::InvalidParam("owner_user_id vide"));
        }
        if snapshot_id.trim().is_empty() {
            return Err(BackupApiError::InvalidParam("snapshot_id vide"));
        }
        // Vérifie que le snapshot existe.
        self.snapshots.get(snapshot_id)?;
        Ok(self.shares.create(
            owner_user_id,
            snapshot_id,
            resource_path,
            expires_at,
            password,
            now_unix,
        )?)
    }

    /// Liste les liens d'un utilisateur.
    pub fn list_shares(&self, owner_user_id: &str) -> Vec<ShareLink> {
        self.shares.list_for_owner(owner_user_id)
    }

    /// Révoque un lien.
    pub fn revoke_share(&self, token: &str) -> Result<(), BackupApiError> {
        Ok(self.shares.revoke(token)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kits::cas_kit::CasConfig;
    use crate::kits::crypto_kit::Key32;

    fn setup() -> (FilesOp, BackupApi, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let cas_root = dir.path().join("cas");
        let key = Key32::generate();
        let cfg = CasConfig::default();

        let files = FilesOp::new(cas_root.clone(), key.clone(), cfg.clone());
        let snaps = Arc::new(SnapshotsOp::new(cas_root.clone(), key.clone(), cfg.clone()));
        let restore = Arc::new(RestoreOp::new(cas_root, key));
        let shares = Arc::new(ShareOp::new());
        let api = BackupApi::new(snaps, restore, shares);
        (files, api, dir)
    }

    // ─── E2E : backup → list → restore ────────────────────────────────────

    #[test]
    fn e2e_backup_list_restore() {
        let (files, api, _dir) = setup();
        // Setup state initial.
        files.put("readme.md", b"# JayCloud", 1).unwrap();
        files.put("src/main.rs", b"fn main() {}", 2).unwrap();
        files.put("src/lib.rs", b"// lib v1", 3).unwrap();

        // Trigger backup.
        let snap = api.trigger_backup("project_alpha", &files, 1_700_000_000).unwrap();
        assert_eq!(snap.target_id, "project_alpha");
        assert_eq!(snap.files_count, 3);

        // List.
        let list = api.list_snapshots(Some("project_alpha"), 10).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, snap.id);

        // Restore un fichier.
        let bytes = api.restore_file_bytes(&snap.id, "src/lib.rs").unwrap();
        assert_eq!(bytes, b"// lib v1");

        // Restore un autre.
        let bytes = api.restore_file_bytes(&snap.id, "readme.md").unwrap();
        assert_eq!(bytes, b"# JayCloud");
    }

    #[test]
    fn e2e_multiple_snapshots_keep_history() {
        let (files, api, _dir) = setup();
        files.put("file.txt", b"v1", 1).unwrap();
        let s1 = api.trigger_backup("t", &files, 1_700_000_000).unwrap();

        files.put("file.txt", b"v2", 2).unwrap();
        let s2 = api.trigger_backup("t", &files, 1_700_000_100).unwrap();

        files.put("file.txt", b"v3", 3).unwrap();
        let s3 = api.trigger_backup("t", &files, 1_700_000_200).unwrap();

        // List : 3 snapshots, le plus récent d'abord.
        let list = api.list_snapshots(Some("t"), 10).unwrap();
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].id, s3.id);

        // Chaque snapshot voit sa version.
        let v1 = api.restore_file_bytes(&s1.id, "file.txt").unwrap();
        let v2 = api.restore_file_bytes(&s2.id, "file.txt").unwrap();
        let v3 = api.restore_file_bytes(&s3.id, "file.txt").unwrap();
        assert_eq!(v1, b"v1");
        assert_eq!(v2, b"v2");
        assert_eq!(v3, b"v3");
    }

    #[test]
    fn e2e_retention_then_gc() {
        let (files, api, _dir) = setup();
        files.put("file.txt", b"content", 1).unwrap();
        // 5 snapshots espacés d'un jour.
        for d in 0..5 {
            let ts = 1_700_000_000 + d * 86400;
            api.trigger_backup("t", &files, ts).unwrap();
        }
        assert_eq!(api.list_snapshots(Some("t"), 10).unwrap().len(), 5);

        // Politique zéro : garde uniquement le plus récent.
        let removed = api
            .apply_retention(
                "t",
                RetentionPolicy {
                    daily: 0,
                    weekly: 0,
                    monthly: 0,
                },
            )
            .unwrap();
        assert_eq!(removed, 4);
        assert_eq!(api.list_snapshots(Some("t"), 10).unwrap().len(), 1);

        // GC : peut supprimer les manifests des snapshots supprimés. Le contenu
        // identique reste référencé par le snapshot survivant donc pas supprimé.
        // On ne fait pas d'assertion exacte sur le nombre — juste qu'il est >= 0.
        api.garbage_collect_orphans().unwrap();
    }

    #[test]
    fn e2e_share_creation_and_resolution() {
        let (files, api, _dir) = setup();
        files.put("doc.pdf", b"%PDF-1.4 ...", 1).unwrap();
        let s = api.trigger_backup("t", &files, 1_700_000_000).unwrap();

        // Crée un lien sans password ni expiration.
        let link = api
            .create_share("alice", &s.id, Some("doc.pdf"), None, None, 1_700_000_100)
            .unwrap();

        // Résolution.
        let res = api.shares.resolve(&link.token, None, 1_700_000_200).unwrap();
        assert_eq!(res.snapshot_id, s.id);
        assert_eq!(res.resource_path, Some("doc.pdf".into()));
    }

    #[test]
    fn e2e_share_with_password() {
        let (files, api, _dir) = setup();
        files.put("doc.pdf", b"x", 1).unwrap();
        let s = api.trigger_backup("t", &files, 1_700_000_000).unwrap();

        let link = api
            .create_share(
                "alice",
                &s.id,
                Some("doc.pdf"),
                None,
                Some("hunter2"),
                1_700_000_100,
            )
            .unwrap();

        // Sans password.
        assert!(matches!(
            api.shares.resolve(&link.token, None, 2_000_000_000),
            Err(ShareOpError::PasswordRequired)
        ));
        // Mauvais password.
        assert!(matches!(
            api.shares.resolve(&link.token, Some("wrong"), 2_000_000_000),
            Err(ShareOpError::InvalidPassword)
        ));
        // Bon password.
        api.shares
            .resolve(&link.token, Some("hunter2"), 2_000_000_000)
            .unwrap();
    }

    #[test]
    fn e2e_share_revoke() {
        let (files, api, _dir) = setup();
        files.put("a.txt", b"x", 1).unwrap();
        let s = api.trigger_backup("t", &files, 1_700_000_000).unwrap();
        let link = api
            .create_share("alice", &s.id, Some("a.txt"), None, None, 1_700_000_100)
            .unwrap();

        api.revoke_share(&link.token).unwrap();
        assert!(matches!(
            api.shares.resolve(&link.token, None, 2_000_000_000),
            Err(ShareOpError::NotFound)
        ));
    }

    #[test]
    fn e2e_share_unknown_snapshot_fails() {
        let (_files, api, _dir) = setup();
        let r = api.create_share("alice", "ghost_snap", None, None, None, 1000);
        assert!(matches!(r, Err(BackupApiError::Snapshots(_))));
    }

    // ─── Validation paramètres ────────────────────────────────────────────

    #[test]
    fn trigger_backup_empty_target_fails() {
        let (files, api, _dir) = setup();
        let r = api.trigger_backup("", &files, 1000);
        assert!(matches!(r, Err(BackupApiError::InvalidParam(_))));
    }

    #[test]
    fn list_snapshots_zero_limit_fails() {
        let (_files, api, _dir) = setup();
        let r = api.list_snapshots(None, 0);
        assert!(matches!(r, Err(BackupApiError::InvalidParam(_))));
    }

    #[test]
    fn restore_file_empty_args_fails() {
        let (_files, api, _dir) = setup();
        let r = api.restore_file_bytes("", "a.txt");
        assert!(matches!(r, Err(BackupApiError::InvalidParam(_))));
        let r = api.restore_file_bytes("snap_1", "");
        assert!(matches!(r, Err(BackupApiError::InvalidParam(_))));
    }
}
