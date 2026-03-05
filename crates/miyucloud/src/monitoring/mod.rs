//! Monitoring et metriques MiyuCloud.
//!
//! @id: miyucloud_monitoring
//! @do: provide_health_check_metrics_disk_space
//! @role: monitoring
//! @layer: infra
//!
//! Health check detaille, collecte de metriques et
//! verification de l'espace disque pour le stockage.

use crate::errors::MiyucloudError;
use std::sync::OnceLock;
use std::time::Instant;

/// Global start time for uptime tracking.
/// Initialised on first call to `health_check`.
static START_TIME: OnceLock<Instant> = OnceLock::new();

/// Resultat du health check detaille.
pub struct HealthStatus {
    /// Statut global ("healthy", "degraded", "unhealthy").
    pub status: String,
    /// La base de donnees est accessible.
    pub db_accessible: bool,
    /// Le stockage est accessible.
    pub storage_accessible: bool,
    /// Chemin du stockage.
    pub storage_path: String,
    /// Espace disque libre en bytes.
    pub disk_free_bytes: u64,
    /// Espace disque total en bytes.
    pub disk_total_bytes: u64,
    /// Nombre de fichiers stockes.
    pub file_count: u64,
    /// Taille totale des fichiers en bytes.
    pub total_size_bytes: u64,
    /// Duree d'execution en secondes.
    pub uptime_seconds: u64,
}

/// Metriques collectees.
pub struct Metrics {
    /// Nombre total de fichiers.
    pub total_files: u64,
    /// Nombre total de dossiers.
    pub total_folders: u64,
    /// Taille totale en bytes.
    pub total_size_bytes: u64,
    /// Nombre de liens de partage actifs.
    pub active_share_links: u64,
    /// Nombre de sessions actives.
    pub active_sessions: u64,
    /// Nombre de pairs de synchronisation.
    pub sync_peers: u64,
}

/// Effectue un health check detaille.
///
/// Verifie l'accessibilite de la base de donnees et du stockage,
/// collecte les compteurs de fichiers et l'espace disque, puis
/// determine le statut global (healthy / degraded / unhealthy).
#[cfg(feature = "legacy-sqlite")]
pub fn health_check(
    db: &crate::data::MiyucloudDb,
    storage_path: &str,
) -> Result<HealthStatus, MiyucloudError> {
    // Record start time on first invocation.
    let start = START_TIME.get_or_init(Instant::now);
    let uptime_seconds = start.elapsed().as_secs();

    // 1. DB accessibility check.
    let (db_accessible, file_count, total_size_bytes) = match db.lock_conn() {
        Ok(conn) => {
            let (count, size) = conn
                .query_row(
                    "SELECT COUNT(*), COALESCE(SUM(size_bytes), 0) \
                     FROM cloud_files WHERE is_trashed = 0",
                    [],
                    |row| {
                        let count: u64 = row.get(0)?;
                        let size: u64 = row.get(1)?;
                        Ok((count, size))
                    },
                )
                .map_err(|e| MiyucloudError::Db(e.to_string()))?;
            (true, count, size)
        }
        Err(_) => (false, 0, 0),
    };

    // 2. Storage accessibility check.
    let path = std::path::Path::new(storage_path);
    let storage_accessible = path.exists() && path.is_dir();

    // 3. Disk space.
    let (disk_free_bytes, disk_total_bytes) = check_disk_space(storage_path).unwrap_or((0, 0));

    // 4. Status logic: healthy / degraded / unhealthy.
    let status = match (db_accessible, storage_accessible) {
        (true, true) => "healthy",
        (false, false) => "unhealthy",
        _ => "degraded",
    };

    Ok(HealthStatus {
        status: status.to_string(),
        db_accessible,
        storage_accessible,
        storage_path: storage_path.to_string(),
        disk_free_bytes,
        disk_total_bytes,
        file_count,
        total_size_bytes,
        uptime_seconds,
    })
}

/// Collecte les metriques actuelles.
///
/// Execute des requetes aggregees sur la base de donnees pour compter
/// les fichiers, dossiers, liens de partage actifs, sessions actives
/// et pairs de synchronisation.
#[cfg(feature = "legacy-sqlite")]
pub fn collect_metrics(db: &crate::data::MiyucloudDb) -> Result<Metrics, MiyucloudError> {
    let conn = db.lock_conn()?;
    let now = chrono::Utc::now().to_rfc3339();

    let total_files: u64 = conn
        .query_row(
            "SELECT COUNT(*) FROM cloud_files WHERE is_trashed = 0",
            [],
            |row| row.get(0),
        )
        .map_err(|e| MiyucloudError::Db(e.to_string()))?;

    let total_folders: u64 = conn
        .query_row(
            "SELECT COUNT(*) FROM cloud_folders WHERE is_trashed = 0",
            [],
            |row| row.get(0),
        )
        .map_err(|e| MiyucloudError::Db(e.to_string()))?;

    let total_size_bytes: u64 = conn
        .query_row(
            "SELECT COALESCE(SUM(size_bytes), 0) FROM cloud_files WHERE is_trashed = 0",
            [],
            |row| row.get(0),
        )
        .map_err(|e| MiyucloudError::Db(e.to_string()))?;

    let active_share_links: u64 = conn
        .query_row(
            "SELECT COUNT(*) FROM cloud_share_links \
             WHERE is_revoked = 0 AND expires_at >= ?1",
            rusqlite::params![now],
            |row| row.get(0),
        )
        .map_err(|e| MiyucloudError::Db(e.to_string()))?;

    let active_sessions: u64 = conn
        .query_row(
            "SELECT COUNT(*) FROM cloud_sessions \
             WHERE is_revoked = 0 AND expires_at >= ?1",
            rusqlite::params![now],
            |row| row.get(0),
        )
        .map_err(|e| MiyucloudError::Db(e.to_string()))?;

    let sync_peers: u64 = conn
        .query_row("SELECT COUNT(*) FROM cloud_sync_peers", [], |row| {
            row.get(0)
        })
        .map_err(|e| MiyucloudError::Db(e.to_string()))?;

    Ok(Metrics {
        total_files,
        total_folders,
        total_size_bytes,
        active_share_links,
        active_sessions,
        sync_peers,
    })
}

/// Verifie l'espace disque disponible.
///
/// Retourne `(free_bytes, total_bytes)` pour le volume contenant `path`.
///
/// NOTE: L'implementation actuelle retourne `(0, 0)` car une solution
/// veritablement cross-platform necessite une dependance externe (ex: `sysinfo`)
/// ou des appels plateforme-specifiques (`statvfs` sur Unix, `GetDiskFreeSpaceExW`
/// sur Windows). Cette fonction compile et fonctionne sur toutes les cibles;
/// les valeurs seront substituees lorsqu'un backend plateforme sera integre.
pub fn check_disk_space(path: &str) -> Result<(u64, u64), MiyucloudError> {
    // Verify the path exists so callers get an early error for bad paths.
    let p = std::path::Path::new(path);
    if !p.exists() {
        return Err(MiyucloudError::NotFound(format!(
            "Storage path does not exist: {path}"
        )));
    }

    // TODO: Platform-specific disk space retrieval.
    // - Unix: libc::statvfs
    // - Windows: winapi GetDiskFreeSpaceExW
    // For now return (0, 0) as a safe placeholder.
    Ok((0, 0))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[cfg(feature = "legacy-sqlite")]
mod tests {
    use super::*;
    use crate::data::MiyucloudDb;

    /// Helper: open an in-memory DB with full schema.
    fn mem_db() -> MiyucloudDb {
        MiyucloudDb::open(":memory:").expect("in-memory DB should open")
    }

    // -- health_check -------------------------------------------------------

    #[test]
    fn health_check_healthy_db() {
        let db = mem_db();
        // Use a storage path that exists on any OS: the current directory.
        let cwd = std::env::current_dir().expect("cwd should be readable");
        let cwd_str = cwd.to_string_lossy();

        let result = health_check(&db, &cwd_str);
        assert!(result.is_ok(), "health_check should succeed with mem DB");

        let status = result.expect("already checked");
        assert_eq!(status.status, "healthy");
        assert!(status.db_accessible);
        assert!(status.storage_accessible);
        assert_eq!(status.file_count, 0);
        assert_eq!(status.total_size_bytes, 0);
    }

    #[test]
    fn health_check_degraded_when_storage_missing() {
        let db = mem_db();
        let bad_path = "/nonexistent_miyucloud_path_42";

        let result = health_check(&db, bad_path);
        assert!(result.is_ok());

        let status = result.expect("already checked");
        // DB is accessible but storage is not -> degraded.
        assert_eq!(status.status, "degraded");
        assert!(status.db_accessible);
        assert!(!status.storage_accessible);
    }

    // -- collect_metrics ----------------------------------------------------

    #[test]
    fn collect_metrics_empty_db() {
        let db = mem_db();

        let result = collect_metrics(&db);
        assert!(result.is_ok(), "collect_metrics should succeed on empty DB");

        let m = result.expect("already checked");
        assert_eq!(m.total_files, 0);
        assert_eq!(m.total_folders, 0);
        assert_eq!(m.total_size_bytes, 0);
        assert_eq!(m.active_share_links, 0);
        assert_eq!(m.active_sessions, 0);
        assert_eq!(m.sync_peers, 0);
    }

    #[test]
    fn collect_metrics_with_data() {
        let db = mem_db();
        let conn = db.lock_conn().expect("lock_conn should succeed");
        let now = chrono::Utc::now().to_rfc3339();

        // Insert 2 files (one active, one trashed).
        conn.execute(
            "INSERT INTO cloud_files \
             (id, owner_id, name, mime_type, size_bytes, checksum_sha256, \
              is_trashed, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                "file-1",
                "owner-1",
                "photo.jpg",
                "image/jpeg",
                2048_i64,
                "abc123",
                0,
                &now,
                &now
            ],
        )
        .expect("insert file-1");

        conn.execute(
            "INSERT INTO cloud_files \
             (id, owner_id, name, mime_type, size_bytes, checksum_sha256, \
              is_trashed, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                "file-2",
                "owner-1",
                "trashed.jpg",
                "image/jpeg",
                512_i64,
                "def456",
                1,
                &now,
                &now
            ],
        )
        .expect("insert file-2 (trashed)");

        // Insert 1 folder (active).
        conn.execute(
            "INSERT INTO cloud_folders \
             (id, owner_id, name, is_trashed, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params!["folder-1", "owner-1", "Photos", 0, &now, &now],
        )
        .expect("insert folder-1");

        // Insert 1 sync peer.
        conn.execute(
            "INSERT INTO cloud_sync_peers \
             (id, cog_id, public_key, created_at) \
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params!["peer-1", "cog-1", "pubkey-base64", &now],
        )
        .expect("insert peer-1");

        // Insert 1 active share link (expires in the future).
        let future = "2099-01-01T00:00:00Z";
        conn.execute(
            "INSERT INTO cloud_share_links \
             (id, file_id, creator_id, token, expires_at, is_revoked, created_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params!["share-1", "file-1", "owner-1", "token123", future, 0, &now],
        )
        .expect("insert share-1");

        // Insert 1 active session (expires in the future).
        conn.execute(
            "INSERT INTO cloud_sessions \
             (id, owner_id, token, created_at, expires_at, last_activity_at, is_revoked) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                "session-1",
                "owner-1",
                "sess-token-1",
                &now,
                future,
                &now,
                0
            ],
        )
        .expect("insert session-1");

        // Drop conn before calling collect_metrics (which locks again).
        drop(conn);

        let m = collect_metrics(&db).expect("collect_metrics with data");

        assert_eq!(m.total_files, 1, "only non-trashed files count");
        assert_eq!(m.total_folders, 1);
        assert_eq!(m.total_size_bytes, 2048, "only non-trashed file size");
        assert_eq!(m.active_share_links, 1);
        assert_eq!(m.active_sessions, 1);
        assert_eq!(m.sync_peers, 1);
    }

    // -- check_disk_space ---------------------------------------------------

    #[test]
    fn check_disk_space_existing_path() {
        let cwd = std::env::current_dir().expect("cwd should be readable");
        let cwd_str = cwd.to_string_lossy();

        let result = check_disk_space(&cwd_str);
        assert!(result.is_ok());

        // Current placeholder returns (0, 0).
        let (free, total) = result.expect("already checked");
        assert_eq!(free, 0);
        assert_eq!(total, 0);
    }

    #[test]
    fn check_disk_space_nonexistent_path() {
        let result = check_disk_space("/nonexistent_miyucloud_path_42");
        assert!(result.is_err(), "nonexistent path should return an error");
    }
}
