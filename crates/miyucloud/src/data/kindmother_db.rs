//! Base de donnees fille SQLite MiyuCloud (KindMother Daughter).
//!
//! @id: miyucloud_kindmother_db
//! @do: provide_sqlite_persistence_for_miyucloud
//! @role: persistence
//! @layer: infra
//!
//! CRUD complet pour fichiers, dossiers, liens de partage, permissions,
//! pairs sync, horloges vectorielles, conflits, journal d'acces, quotas
//! et configuration cryptographique.

use crate::data::types::{
    AccessAction, AccessLogEntry, ConflictStatus, CryptoConfig, FileEntry, FolderContents,
    FolderEntry, GranteeType, OnboardingRow, PermissionLevel, RecoveryCodeRow, SessionRow,
    ShareLink, SharePermission, SyncConflict, SyncPeer, TotpRow, UserQuota, VectorClockEntry,
};
use crate::errors::MiyucloudError;
use crate::storage::compression::{decompress, maybe_compress};
use crate::storage::dedup::{ContentAddressableStorage, ContentHash};
use kindmother::{InstanceIdentity, InstanceType};
use rusqlite::params;
use std::path::Path;
use std::sync::Mutex;

/// Retourne la date courante en RFC3339 (ISO 8601).
fn now_iso() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// Base de donnees MiyuCloud (Instance Daughter).
pub struct MiyucloudDb {
    conn: Mutex<rusqlite::Connection>,
    /// Identite de l'instance KindMother.
    pub instance: InstanceIdentity,
}

impl MiyucloudDb {
    /// Ouvre ou cree la base a `path` et initialise le schema.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, MiyucloudError> {
        let path = path.as_ref();
        let conn = rusqlite::Connection::open(path)?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA foreign_keys = ON;
             PRAGMA busy_timeout = 5000;",
        )?;
        let instance = InstanceIdentity::new(InstanceType::Daughter);
        let db = Self {
            conn: Mutex::new(conn),
            instance,
        };
        db.init_schema()?;
        Ok(db)
    }

    /// Verrouille et retourne la connexion SQLite.
    pub fn lock_conn(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, rusqlite::Connection>, MiyucloudError> {
        self.conn
            .lock()
            .map_err(|e| MiyucloudError::Db(format!("Mutex poisoned: {e}")))
    }

    fn init_schema(&self) -> Result<(), MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute_batch(
            r"
            CREATE TABLE IF NOT EXISTS cloud_files (
                id              TEXT PRIMARY KEY,
                parent_id       TEXT,
                owner_id        TEXT NOT NULL,
                name            TEXT NOT NULL,
                mime_type       TEXT NOT NULL DEFAULT 'application/octet-stream',
                size_bytes      INTEGER NOT NULL DEFAULT 0,
                checksum_sha256 TEXT NOT NULL,
                encryption_iv   TEXT,
                chunk_count     INTEGER NOT NULL DEFAULT 1,
                is_trashed      INTEGER NOT NULL DEFAULT 0,
                trashed_at      TEXT,
                created_at      TEXT NOT NULL,
                updated_at      TEXT NOT NULL,
                FOREIGN KEY (parent_id) REFERENCES cloud_folders(id)
            );
            CREATE INDEX IF NOT EXISTS idx_cloud_files_parent ON cloud_files(parent_id);
            CREATE INDEX IF NOT EXISTS idx_cloud_files_owner ON cloud_files(owner_id);
            CREATE INDEX IF NOT EXISTS idx_cloud_files_trashed ON cloud_files(is_trashed);

            CREATE TABLE IF NOT EXISTS cloud_folders (
                id              TEXT PRIMARY KEY,
                parent_id       TEXT,
                owner_id        TEXT NOT NULL,
                name            TEXT NOT NULL,
                is_trashed      INTEGER NOT NULL DEFAULT 0,
                trashed_at      TEXT,
                created_at      TEXT NOT NULL,
                updated_at      TEXT NOT NULL,
                FOREIGN KEY (parent_id) REFERENCES cloud_folders(id)
            );
            CREATE INDEX IF NOT EXISTS idx_cloud_folders_parent ON cloud_folders(parent_id);
            CREATE INDEX IF NOT EXISTS idx_cloud_folders_owner ON cloud_folders(owner_id);

            CREATE TABLE IF NOT EXISTS cloud_share_links (
                id              TEXT PRIMARY KEY,
                file_id         TEXT,
                folder_id       TEXT,
                creator_id      TEXT NOT NULL,
                token           TEXT NOT NULL UNIQUE,
                password_hash   TEXT,
                expires_at      TEXT NOT NULL,
                max_downloads   INTEGER,
                download_count  INTEGER NOT NULL DEFAULT 0,
                is_revoked      INTEGER NOT NULL DEFAULT 0,
                created_at      TEXT NOT NULL,
                FOREIGN KEY (file_id) REFERENCES cloud_files(id),
                FOREIGN KEY (folder_id) REFERENCES cloud_folders(id),
                CHECK (
                    (file_id IS NOT NULL AND folder_id IS NULL)
                    OR (file_id IS NULL AND folder_id IS NOT NULL)
                )
            );
            CREATE UNIQUE INDEX IF NOT EXISTS idx_cloud_share_links_token ON cloud_share_links(token);

            CREATE TABLE IF NOT EXISTS cloud_share_permissions (
                id              TEXT PRIMARY KEY,
                file_id         TEXT,
                folder_id       TEXT,
                grantee_id      TEXT NOT NULL,
                grantee_type    TEXT NOT NULL DEFAULT 'profile',
                permission      TEXT NOT NULL DEFAULT 'read',
                created_at      TEXT NOT NULL,
                FOREIGN KEY (file_id) REFERENCES cloud_files(id),
                FOREIGN KEY (folder_id) REFERENCES cloud_folders(id),
                CHECK (
                    (file_id IS NOT NULL AND folder_id IS NULL)
                    OR (file_id IS NULL AND folder_id IS NOT NULL)
                )
            );

            CREATE TABLE IF NOT EXISTS cloud_sync_peers (
                id              TEXT PRIMARY KEY,
                cog_id          TEXT NOT NULL UNIQUE,
                display_name    TEXT,
                public_key      TEXT NOT NULL,
                last_seen_at    TEXT,
                last_sync_at    TEXT,
                is_trusted      INTEGER NOT NULL DEFAULT 0,
                created_at      TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS cloud_vector_clocks (
                file_id         TEXT NOT NULL,
                node_id         TEXT NOT NULL,
                counter         INTEGER NOT NULL DEFAULT 0,
                updated_at      TEXT NOT NULL,
                PRIMARY KEY (file_id, node_id),
                FOREIGN KEY (file_id) REFERENCES cloud_files(id)
            );

            CREATE TABLE IF NOT EXISTS cloud_sync_conflicts (
                id              TEXT PRIMARY KEY,
                file_id         TEXT NOT NULL,
                local_version   TEXT NOT NULL,
                remote_version  TEXT NOT NULL,
                remote_node_id  TEXT NOT NULL,
                status          TEXT NOT NULL DEFAULT 'pending',
                created_at      TEXT NOT NULL,
                resolved_at     TEXT,
                FOREIGN KEY (file_id) REFERENCES cloud_files(id)
            );

            CREATE TABLE IF NOT EXISTS cloud_access_log (
                id              TEXT PRIMARY KEY,
                share_link_id   TEXT,
                accessor_ip     TEXT,
                accessor_ua     TEXT,
                action          TEXT NOT NULL,
                file_id         TEXT,
                success         INTEGER NOT NULL DEFAULT 1,
                created_at      TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_cloud_access_log_share ON cloud_access_log(share_link_id);
            CREATE INDEX IF NOT EXISTS idx_cloud_access_log_date ON cloud_access_log(created_at);

            CREATE TABLE IF NOT EXISTS cloud_quotas (
                owner_id        TEXT PRIMARY KEY,
                max_bytes       INTEGER NOT NULL DEFAULT 0,
                used_bytes      INTEGER NOT NULL DEFAULT 0,
                updated_at      TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS cloud_sync_folders (
                id              TEXT PRIMARY KEY,
                folder_id       TEXT NOT NULL,
                local_path      TEXT NOT NULL,
                is_active       INTEGER NOT NULL DEFAULT 1,
                created_at      TEXT NOT NULL,
                FOREIGN KEY (folder_id) REFERENCES cloud_folders(id)
            );

            CREATE TABLE IF NOT EXISTS cloud_jay1tribu_migrations (
                id              TEXT PRIMARY KEY,
                jay1tribu_msg_id TEXT NOT NULL,
                file_id         TEXT NOT NULL,
                migrated_at     TEXT NOT NULL,
                FOREIGN KEY (file_id) REFERENCES cloud_files(id)
            );

            CREATE TABLE IF NOT EXISTS cloud_crypto_config (
                owner_id          TEXT PRIMARY KEY,
                argon2_salt       TEXT NOT NULL,
                canary_ciphertext TEXT,
                canary_nonce      TEXT,
                key_version       INTEGER NOT NULL DEFAULT 1,
                created_at        TEXT NOT NULL,
                updated_at        TEXT NOT NULL
            );

            -- V2: Sessions + 2FA + Onboarding
            CREATE TABLE IF NOT EXISTS cloud_sessions (
                id                TEXT PRIMARY KEY,
                owner_id          TEXT NOT NULL,
                token             TEXT NOT NULL UNIQUE,
                created_ip_hash   TEXT,
                created_ua        TEXT,
                created_at        TEXT NOT NULL,
                expires_at        TEXT NOT NULL,
                last_activity_at  TEXT NOT NULL,
                is_revoked        INTEGER NOT NULL DEFAULT 0,
                totp_verified     INTEGER NOT NULL DEFAULT 0
            );
            CREATE UNIQUE INDEX IF NOT EXISTS idx_cloud_sessions_token
                ON cloud_sessions(token);
            CREATE INDEX IF NOT EXISTS idx_cloud_sessions_owner
                ON cloud_sessions(owner_id);
            CREATE INDEX IF NOT EXISTS idx_cloud_sessions_expires
                ON cloud_sessions(expires_at);

            CREATE TABLE IF NOT EXISTS cloud_totp_secrets (
                id                TEXT PRIMARY KEY,
                owner_id          TEXT NOT NULL UNIQUE,
                encrypted_secret  TEXT NOT NULL,
                encryption_nonce  TEXT NOT NULL,
                issuer            TEXT NOT NULL DEFAULT 'MiyuCloud',
                account_name      TEXT NOT NULL,
                algorithm         TEXT NOT NULL DEFAULT 'SHA1',
                digits            INTEGER NOT NULL DEFAULT 6,
                period            INTEGER NOT NULL DEFAULT 30,
                is_active         INTEGER NOT NULL DEFAULT 0,
                created_at        TEXT NOT NULL,
                last_used_at      TEXT
            );
            CREATE UNIQUE INDEX IF NOT EXISTS idx_cloud_totp_owner
                ON cloud_totp_secrets(owner_id);

            CREATE TABLE IF NOT EXISTS cloud_recovery_codes (
                id          TEXT PRIMARY KEY,
                owner_id    TEXT NOT NULL,
                code_hash   TEXT NOT NULL,
                is_used     INTEGER NOT NULL DEFAULT 0,
                created_at  TEXT NOT NULL,
                used_at     TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_cloud_recovery_codes_owner
                ON cloud_recovery_codes(owner_id);

            CREATE TABLE IF NOT EXISTS cloud_onboarding (
                owner_id          TEXT PRIMARY KEY,
                current_step      INTEGER NOT NULL DEFAULT 1,
                passphrase_set    INTEGER NOT NULL DEFAULT 0,
                totp_set          INTEGER NOT NULL DEFAULT 0,
                storage_verified  INTEGER NOT NULL DEFAULT 0,
                completed         INTEGER NOT NULL DEFAULT 0,
                completed_at      TEXT
            );

            CREATE TABLE IF NOT EXISTS cloud_content_blobs (
                hash        TEXT PRIMARY KEY NOT NULL,
                data        BLOB NOT NULL,
                size        INTEGER NOT NULL,
                compressed  INTEGER NOT NULL DEFAULT 0,
                ref_count   INTEGER NOT NULL DEFAULT 1,
                created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
            );
            CREATE INDEX IF NOT EXISTS idx_content_blobs_hash ON cloud_content_blobs(hash);

            CREATE TABLE IF NOT EXISTS cloud_file_blobs (
                file_id   TEXT NOT NULL REFERENCES cloud_files(id) ON DELETE CASCADE,
                blob_hash TEXT NOT NULL REFERENCES cloud_content_blobs(hash),
                PRIMARY KEY (file_id, blob_hash)
            );
            CREATE INDEX IF NOT EXISTS idx_file_blobs_file ON cloud_file_blobs(file_id);
            ",
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Files
    // -----------------------------------------------------------------------

    /// Liste les fichiers d'un dossier pour un proprietaire.
    pub fn file_list(
        &self,
        parent_id: Option<&str>,
        owner_id: &str,
    ) -> Result<Vec<FileEntry>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = if parent_id.is_some() {
            conn.prepare(
                "SELECT id, parent_id, owner_id, name, mime_type, size_bytes, checksum_sha256,
                        encryption_iv, chunk_count, is_trashed, trashed_at, created_at, updated_at
                 FROM cloud_files WHERE parent_id = ?1 AND owner_id = ?2 AND is_trashed = 0
                 ORDER BY name",
            )?
        } else {
            conn.prepare(
                "SELECT id, parent_id, owner_id, name, mime_type, size_bytes, checksum_sha256,
                        encryption_iv, chunk_count, is_trashed, trashed_at, created_at, updated_at
                 FROM cloud_files WHERE parent_id IS NULL AND owner_id = ?2 AND is_trashed = 0
                 ORDER BY name",
            )?
        };
        let pid = parent_id.unwrap_or("");
        let rows = if parent_id.is_some() {
            stmt.query_map(params![pid, owner_id], row_to_file)?
        } else {
            stmt.query_map(params![pid, owner_id], row_to_file)?
        };
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| MiyucloudError::Db(e.to_string()))
    }

    /// Recupere un fichier par son ID.
    pub fn file_by_id(&self, id: &str) -> Result<Option<FileEntry>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, parent_id, owner_id, name, mime_type, size_bytes, checksum_sha256,
                    encryption_iv, chunk_count, is_trashed, trashed_at, created_at, updated_at
             FROM cloud_files WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map(params![id], row_to_file)?;
        match rows.next() {
            Some(r) => Ok(Some(r.map_err(|e| MiyucloudError::Db(e.to_string()))?)),
            None => Ok(None),
        }
    }

    /// Cree un fichier.
    pub fn file_create(&self, data: &FileEntry) -> Result<FileEntry, MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT INTO cloud_files (id, parent_id, owner_id, name, mime_type, size_bytes,
             checksum_sha256, encryption_iv, chunk_count, is_trashed, trashed_at, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                data.id,
                data.parent_id,
                data.owner_id,
                data.name,
                data.mime_type,
                data.size_bytes as i64,
                data.checksum_sha256,
                data.encryption_iv,
                data.chunk_count,
                data.is_trashed as i32,
                data.trashed_at,
                data.created_at,
                data.updated_at,
            ],
        )?;
        Ok(data.clone())
    }

    /// Met a jour un fichier.
    pub fn file_update(&self, data: &FileEntry) -> Result<FileEntry, MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_files SET parent_id = ?1, name = ?2, mime_type = ?3,
             size_bytes = ?4, checksum_sha256 = ?5, encryption_iv = ?6,
             chunk_count = ?7, updated_at = ?8 WHERE id = ?9",
            params![
                data.parent_id,
                data.name,
                data.mime_type,
                data.size_bytes as i64,
                data.checksum_sha256,
                data.encryption_iv,
                data.chunk_count,
                now,
                data.id,
            ],
        )?;
        let mut updated = data.clone();
        updated.updated_at = now;
        Ok(updated)
    }

    /// Supprime definitivement un fichier de la base.
    pub fn file_delete(&self, id: &str) -> Result<(), MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute("DELETE FROM cloud_files WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Met un fichier en corbeille.
    pub fn file_trash(&self, id: &str) -> Result<(), MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_files SET is_trashed = 1, trashed_at = ?1, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    /// Restaure un fichier depuis la corbeille.
    pub fn file_restore(&self, id: &str) -> Result<(), MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_files SET is_trashed = 0, trashed_at = NULL, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    /// Recherche des fichiers par nom.
    pub fn file_search(
        &self,
        owner_id: &str,
        query: &str,
    ) -> Result<Vec<FileEntry>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let pattern = format!("%{query}%");
        let mut stmt = conn.prepare(
            "SELECT id, parent_id, owner_id, name, mime_type, size_bytes, checksum_sha256,
                    encryption_iv, chunk_count, is_trashed, trashed_at, created_at, updated_at
             FROM cloud_files WHERE owner_id = ?1 AND name LIKE ?2 AND is_trashed = 0
             ORDER BY name",
        )?;
        let rows = stmt.query_map(params![owner_id, pattern], row_to_file)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| MiyucloudError::Db(e.to_string()))
    }

    /// Liste les fichiers en corbeille pour un proprietaire.
    pub fn file_list_trashed(&self, owner_id: &str) -> Result<Vec<FileEntry>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, parent_id, owner_id, name, mime_type, size_bytes, checksum_sha256,
                    encryption_iv, chunk_count, is_trashed, trashed_at, created_at, updated_at
             FROM cloud_files WHERE owner_id = ?1 AND is_trashed = 1
             ORDER BY trashed_at DESC",
        )?;
        let rows = stmt.query_map(params![owner_id], row_to_file)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| MiyucloudError::Db(e.to_string()))
    }

    // -----------------------------------------------------------------------
    // Folders
    // -----------------------------------------------------------------------

    /// Liste les dossiers d'un parent pour un proprietaire.
    pub fn folder_list(
        &self,
        parent_id: Option<&str>,
        owner_id: &str,
    ) -> Result<Vec<FolderEntry>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = if parent_id.is_some() {
            conn.prepare(
                "SELECT id, parent_id, owner_id, name, is_trashed, trashed_at, created_at, updated_at
                 FROM cloud_folders WHERE parent_id = ?1 AND owner_id = ?2 AND is_trashed = 0
                 ORDER BY name",
            )?
        } else {
            conn.prepare(
                "SELECT id, parent_id, owner_id, name, is_trashed, trashed_at, created_at, updated_at
                 FROM cloud_folders WHERE parent_id IS NULL AND owner_id = ?2 AND is_trashed = 0
                 ORDER BY name",
            )?
        };
        let pid = parent_id.unwrap_or("");
        let rows = if parent_id.is_some() {
            stmt.query_map(params![pid, owner_id], row_to_folder)?
        } else {
            stmt.query_map(params![pid, owner_id], row_to_folder)?
        };
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| MiyucloudError::Db(e.to_string()))
    }

    /// Recupere un dossier par son ID.
    pub fn folder_by_id(&self, id: &str) -> Result<Option<FolderEntry>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, parent_id, owner_id, name, is_trashed, trashed_at, created_at, updated_at
             FROM cloud_folders WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map(params![id], row_to_folder)?;
        match rows.next() {
            Some(r) => Ok(Some(r.map_err(|e| MiyucloudError::Db(e.to_string()))?)),
            None => Ok(None),
        }
    }

    /// Cree un dossier.
    pub fn folder_create(&self, data: &FolderEntry) -> Result<FolderEntry, MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT INTO cloud_folders (id, parent_id, owner_id, name, is_trashed, trashed_at, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                data.id,
                data.parent_id,
                data.owner_id,
                data.name,
                data.is_trashed as i32,
                data.trashed_at,
                data.created_at,
                data.updated_at,
            ],
        )?;
        Ok(data.clone())
    }

    /// Met a jour un dossier.
    pub fn folder_update(&self, data: &FolderEntry) -> Result<FolderEntry, MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_folders SET parent_id = ?1, name = ?2, updated_at = ?3 WHERE id = ?4",
            params![data.parent_id, data.name, now, data.id],
        )?;
        let mut updated = data.clone();
        updated.updated_at = now;
        Ok(updated)
    }

    /// Supprime definitivement un dossier de la base.
    pub fn folder_delete(&self, id: &str) -> Result<(), MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute("DELETE FROM cloud_folders WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Met un dossier en corbeille.
    pub fn folder_trash(&self, id: &str) -> Result<(), MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_folders SET is_trashed = 1, trashed_at = ?1, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    /// Restaure un dossier depuis la corbeille.
    pub fn folder_restore(&self, id: &str) -> Result<(), MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_folders SET is_trashed = 0, trashed_at = NULL, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    /// Liste les dossiers en corbeille pour un proprietaire.
    pub fn folder_list_trashed(&self, owner_id: &str) -> Result<Vec<FolderEntry>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, parent_id, owner_id, name, is_trashed, trashed_at, created_at, updated_at
             FROM cloud_folders WHERE owner_id = ?1 AND is_trashed = 1
             ORDER BY trashed_at DESC",
        )?;
        let rows = stmt.query_map(params![owner_id], row_to_folder)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| MiyucloudError::Db(e.to_string()))
    }

    /// Retourne le contenu d'un dossier (fichiers + sous-dossiers).
    pub fn folder_contents(
        &self,
        folder_id: Option<&str>,
        owner_id: &str,
    ) -> Result<FolderContents, MiyucloudError> {
        let folder = match folder_id {
            Some(id) => self.folder_by_id(id)?,
            None => None,
        };
        let subfolders = self.folder_list(folder_id, owner_id)?;
        let files = self.file_list(folder_id, owner_id)?;
        Ok(FolderContents {
            folder,
            subfolders,
            files,
        })
    }

    // -----------------------------------------------------------------------
    // Share Links
    // -----------------------------------------------------------------------

    /// Cree un lien de partage.
    pub fn share_link_create(
        &self,
        data: &ShareLink,
        password_hash: Option<&str>,
    ) -> Result<ShareLink, MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT INTO cloud_share_links (id, file_id, folder_id, creator_id, token, password_hash,
             expires_at, max_downloads, download_count, is_revoked, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                data.id,
                data.file_id,
                data.folder_id,
                data.creator_id,
                data.token,
                password_hash,
                data.expires_at,
                data.max_downloads.map(|v| v as i64),
                data.download_count as i64,
                data.is_revoked as i32,
                data.created_at,
            ],
        )?;
        Ok(data.clone())
    }

    /// Recupere un lien de partage par son token.
    pub fn share_link_by_token(&self, token: &str) -> Result<Option<ShareLink>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, file_id, folder_id, creator_id, token, password_hash,
                    expires_at, max_downloads, download_count, is_revoked, created_at
             FROM cloud_share_links WHERE token = ?1",
        )?;
        let mut rows = stmt.query_map(params![token], row_to_share_link)?;
        match rows.next() {
            Some(r) => Ok(Some(r.map_err(|e| MiyucloudError::Db(e.to_string()))?)),
            None => Ok(None),
        }
    }

    /// Revoque un lien de partage.
    pub fn share_link_revoke(&self, id: &str) -> Result<(), MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_share_links SET is_revoked = 1 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    /// Liste les liens de partage d'un createur.
    pub fn share_link_list(&self, creator_id: &str) -> Result<Vec<ShareLink>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, file_id, folder_id, creator_id, token, password_hash,
                    expires_at, max_downloads, download_count, is_revoked, created_at
             FROM cloud_share_links WHERE creator_id = ?1 ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(params![creator_id], row_to_share_link)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| MiyucloudError::Db(e.to_string()))
    }

    /// Incremente le compteur de telechargements d'un lien.
    pub fn share_link_increment_downloads(&self, id: &str) -> Result<(), MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_share_links SET download_count = download_count + 1 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    /// Recupere le hash du mot de passe d'un lien de partage.
    pub fn share_link_password_hash(
        &self,
        link_id: &str,
    ) -> Result<Option<String>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare("SELECT password_hash FROM cloud_share_links WHERE id = ?1")?;
        let mut rows = stmt.query_map(params![link_id], |row| row.get::<_, Option<String>>(0))?;
        match rows.next() {
            Some(r) => Ok(r.map_err(|e| MiyucloudError::Db(e.to_string()))?),
            None => Ok(None),
        }
    }

    /// Recupere un lien de partage par son ID.
    pub fn share_link_by_id(&self, id: &str) -> Result<Option<ShareLink>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, file_id, folder_id, creator_id, token, password_hash,
                    expires_at, max_downloads, download_count, is_revoked, created_at
             FROM cloud_share_links WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map(params![id], row_to_share_link)?;
        match rows.next() {
            Some(r) => Ok(Some(r.map_err(|e| MiyucloudError::Db(e.to_string()))?)),
            None => Ok(None),
        }
    }

    /// Met a jour le quota maximum pour un proprietaire.
    pub fn quota_set_max(&self, owner_id: &str, max_bytes: u64) -> Result<(), MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT INTO cloud_quotas (owner_id, max_bytes, used_bytes, updated_at) VALUES (?1, ?2, 0, ?3)
             ON CONFLICT(owner_id) DO UPDATE SET max_bytes = ?2, updated_at = ?3",
            params![owner_id, max_bytes as i64, now],
        )?;
        Ok(())
    }

    /// Calcule les statistiques de stockage.
    pub fn storage_stats(&self) -> Result<crate::data::types::StorageStats, MiyucloudError> {
        let conn = self.lock_conn()?;
        let total_files: u64 = conn
            .query_row(
                "SELECT COUNT(*) FROM cloud_files WHERE is_trashed = 0",
                [],
                |row| row.get::<_, i64>(0),
            )
            .unwrap_or(0) as u64;
        let total_folders: u64 = conn
            .query_row(
                "SELECT COUNT(*) FROM cloud_folders WHERE is_trashed = 0",
                [],
                |row| row.get::<_, i64>(0),
            )
            .unwrap_or(0) as u64;
        let total_size_bytes: u64 = conn
            .query_row(
                "SELECT COALESCE(SUM(size_bytes), 0) FROM cloud_files WHERE is_trashed = 0",
                [],
                |row| row.get::<_, i64>(0),
            )
            .unwrap_or(0) as u64;
        let trashed_files: u64 = conn
            .query_row(
                "SELECT COUNT(*) FROM cloud_files WHERE is_trashed = 1",
                [],
                |row| row.get::<_, i64>(0),
            )
            .unwrap_or(0) as u64;
        let trashed_size_bytes: u64 = conn
            .query_row(
                "SELECT COALESCE(SUM(size_bytes), 0) FROM cloud_files WHERE is_trashed = 1",
                [],
                |row| row.get::<_, i64>(0),
            )
            .unwrap_or(0) as u64;
        let active_shares: u64 = conn
            .query_row(
                "SELECT COUNT(*) FROM cloud_share_links WHERE is_revoked = 0",
                [],
                |row| row.get::<_, i64>(0),
            )
            .unwrap_or(0) as u64;
        let sync_peers: u64 = conn
            .query_row("SELECT COUNT(*) FROM cloud_sync_peers", [], |row| {
                row.get::<_, i64>(0)
            })
            .unwrap_or(0) as u64;

        Ok(crate::data::types::StorageStats {
            total_files,
            total_folders,
            total_size_bytes,
            trashed_files,
            trashed_size_bytes,
            active_shares,
            sync_peers,
        })
    }

    // -----------------------------------------------------------------------
    // Share Permissions
    // -----------------------------------------------------------------------

    /// Cree une permission de partage interne.
    pub fn share_permission_create(
        &self,
        data: &SharePermission,
    ) -> Result<SharePermission, MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT INTO cloud_share_permissions (id, file_id, folder_id, grantee_id, grantee_type, permission, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                data.id,
                data.file_id,
                data.folder_id,
                data.grantee_id,
                data.grantee_type.as_str(),
                data.permission.as_str(),
                data.created_at,
            ],
        )?;
        Ok(data.clone())
    }

    /// Liste les permissions pour un fichier ou dossier.
    pub fn share_permission_list(
        &self,
        file_or_folder_id: &str,
    ) -> Result<Vec<SharePermission>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, file_id, folder_id, grantee_id, grantee_type, permission, created_at
             FROM cloud_share_permissions WHERE file_id = ?1 OR folder_id = ?1
             ORDER BY created_at",
        )?;
        let rows = stmt.query_map(params![file_or_folder_id], |row| {
            Ok(SharePermission {
                id: row.get(0)?,
                file_id: row.get(1)?,
                folder_id: row.get(2)?,
                grantee_id: row.get(3)?,
                grantee_type: GranteeType::from_str_value(
                    &row.get::<_, String>(4).unwrap_or_default(),
                ),
                permission: PermissionLevel::from_str_value(
                    &row.get::<_, String>(5).unwrap_or_default(),
                ),
                created_at: row.get(6)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| MiyucloudError::Db(e.to_string()))
    }

    /// Supprime une permission de partage.
    pub fn share_permission_delete(&self, id: &str) -> Result<(), MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "DELETE FROM cloud_share_permissions WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Sync Peers
    // -----------------------------------------------------------------------

    /// Upsert un pair de sync.
    pub fn sync_peer_upsert(&self, data: &SyncPeer) -> Result<SyncPeer, MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT INTO cloud_sync_peers (id, cog_id, display_name, public_key, last_seen_at, last_sync_at, is_trusted, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(cog_id) DO UPDATE SET
                display_name = excluded.display_name,
                public_key = excluded.public_key,
                last_seen_at = excluded.last_seen_at,
                last_sync_at = excluded.last_sync_at,
                is_trusted = excluded.is_trusted",
            params![
                data.id,
                data.cog_id,
                data.display_name,
                data.public_key,
                data.last_seen_at,
                data.last_sync_at,
                data.is_trusted as i32,
                data.created_at,
            ],
        )?;
        Ok(data.clone())
    }

    /// Liste tous les pairs de sync.
    pub fn sync_peer_list(&self) -> Result<Vec<SyncPeer>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, cog_id, display_name, public_key, last_seen_at, last_sync_at, is_trusted, created_at
             FROM cloud_sync_peers ORDER BY created_at",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(SyncPeer {
                id: row.get(0)?,
                cog_id: row.get(1)?,
                display_name: row.get(2)?,
                public_key: row.get(3)?,
                last_seen_at: row.get(4)?,
                last_sync_at: row.get(5)?,
                is_trusted: row.get::<_, i32>(6).unwrap_or(0) != 0,
                created_at: row.get(7)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| MiyucloudError::Db(e.to_string()))
    }

    /// Recupere un pair par son COG ID.
    pub fn sync_peer_by_cog_id(&self, cog_id: &str) -> Result<Option<SyncPeer>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, cog_id, display_name, public_key, last_seen_at, last_sync_at, is_trusted, created_at
             FROM cloud_sync_peers WHERE cog_id = ?1",
        )?;
        let mut rows = stmt.query_map(params![cog_id], |row| {
            Ok(SyncPeer {
                id: row.get(0)?,
                cog_id: row.get(1)?,
                display_name: row.get(2)?,
                public_key: row.get(3)?,
                last_seen_at: row.get(4)?,
                last_sync_at: row.get(5)?,
                is_trusted: row.get::<_, i32>(6).unwrap_or(0) != 0,
                created_at: row.get(7)?,
            })
        })?;
        match rows.next() {
            Some(r) => Ok(Some(r.map_err(|e| MiyucloudError::Db(e.to_string()))?)),
            None => Ok(None),
        }
    }

    // -----------------------------------------------------------------------
    // Vector Clocks
    // -----------------------------------------------------------------------

    /// Recupere les entrees d'horloge vectorielle pour un fichier.
    pub fn vector_clock_get(&self, file_id: &str) -> Result<Vec<VectorClockEntry>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT file_id, node_id, counter, updated_at FROM cloud_vector_clocks WHERE file_id = ?1",
        )?;
        let rows = stmt.query_map(params![file_id], |row| {
            Ok(VectorClockEntry {
                file_id: row.get(0)?,
                node_id: row.get(1)?,
                counter: row.get::<_, i64>(2).unwrap_or(0) as u64,
                updated_at: row.get(3)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| MiyucloudError::Db(e.to_string()))
    }

    /// Met a jour (upsert) une entree d'horloge vectorielle.
    pub fn vector_clock_update(
        &self,
        file_id: &str,
        node_id: &str,
        counter: u64,
    ) -> Result<(), MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT INTO cloud_vector_clocks (file_id, node_id, counter, updated_at)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(file_id, node_id) DO UPDATE SET counter = excluded.counter, updated_at = excluded.updated_at",
            params![file_id, node_id, counter as i64, now],
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Sync Conflicts
    // -----------------------------------------------------------------------

    /// Cree un conflit de sync.
    pub fn sync_conflict_create(
        &self,
        data: &SyncConflict,
    ) -> Result<SyncConflict, MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT INTO cloud_sync_conflicts (id, file_id, local_version, remote_version, remote_node_id, status, created_at, resolved_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                data.id,
                data.file_id,
                data.local_version,
                data.remote_version,
                data.remote_node_id,
                data.status.as_str(),
                data.created_at,
                data.resolved_at,
            ],
        )?;
        Ok(data.clone())
    }

    /// Liste les conflits en attente.
    pub fn sync_conflict_list_pending(&self) -> Result<Vec<SyncConflict>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, file_id, local_version, remote_version, remote_node_id, status, created_at, resolved_at
             FROM cloud_sync_conflicts WHERE status = 'pending' ORDER BY created_at",
        )?;
        let rows = stmt.query_map([], row_to_sync_conflict)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| MiyucloudError::Db(e.to_string()))
    }

    /// Resout un conflit.
    pub fn sync_conflict_resolve(
        &self,
        id: &str,
        status: ConflictStatus,
    ) -> Result<(), MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_sync_conflicts SET status = ?1, resolved_at = ?2 WHERE id = ?3",
            params![status.as_str(), now, id],
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Access Log
    // -----------------------------------------------------------------------

    /// Cree une entree dans le journal d'acces.
    pub fn access_log_create(&self, data: &AccessLogEntry) -> Result<(), MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT INTO cloud_access_log (id, share_link_id, accessor_ip, accessor_ua, action, file_id, success, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                data.id,
                data.share_link_id,
                data.accessor_ip,
                data.accessor_ua,
                data.action.as_str(),
                data.file_id,
                data.success as i32,
                data.created_at,
            ],
        )?;
        Ok(())
    }

    /// Liste le journal d'acces pour un lien de partage.
    pub fn access_log_list(
        &self,
        share_link_id: Option<&str>,
        limit: u32,
    ) -> Result<Vec<AccessLogEntry>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = if share_link_id.is_some() {
            conn.prepare(
                "SELECT id, share_link_id, accessor_ip, accessor_ua, action, file_id, success, created_at
                 FROM cloud_access_log WHERE share_link_id = ?1 ORDER BY created_at DESC LIMIT ?2",
            )?
        } else {
            conn.prepare(
                "SELECT id, share_link_id, accessor_ip, accessor_ua, action, file_id, success, created_at
                 FROM cloud_access_log ORDER BY created_at DESC LIMIT ?2",
            )?
        };
        let sid = share_link_id.unwrap_or("");
        let rows = if share_link_id.is_some() {
            stmt.query_map(params![sid, limit], row_to_access_log)?
        } else {
            stmt.query_map(params![sid, limit], row_to_access_log)?
        };
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| MiyucloudError::Db(e.to_string()))
    }

    // -----------------------------------------------------------------------
    // Quotas
    // -----------------------------------------------------------------------

    /// Recupere le quota d'un proprietaire (cree un enregistrement par defaut si absent).
    pub fn quota_get(&self, owner_id: &str) -> Result<UserQuota, MiyucloudError> {
        let conn = self.lock_conn()?;
        let now = now_iso();
        conn.execute(
            "INSERT OR IGNORE INTO cloud_quotas (owner_id, max_bytes, used_bytes, updated_at) VALUES (?1, 0, 0, ?2)",
            params![owner_id, now],
        )?;
        let quota = conn.query_row(
            "SELECT owner_id, max_bytes, used_bytes, updated_at FROM cloud_quotas WHERE owner_id = ?1",
            params![owner_id],
            |row| {
                Ok(UserQuota {
                    owner_id: row.get(0)?,
                    max_bytes: row.get::<_, i64>(1).unwrap_or(0) as u64,
                    used_bytes: row.get::<_, i64>(2).unwrap_or(0) as u64,
                    updated_at: row.get(3)?,
                })
            },
        )?;
        Ok(quota)
    }

    /// Met a jour l'espace utilise (delta peut etre negatif pour une suppression).
    pub fn quota_update_used(&self, owner_id: &str, delta: i64) -> Result<(), MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        // Ensure row exists
        conn.execute(
            "INSERT OR IGNORE INTO cloud_quotas (owner_id, max_bytes, used_bytes, updated_at) VALUES (?1, 0, 0, ?2)",
            params![owner_id, now],
        )?;
        conn.execute(
            "UPDATE cloud_quotas SET used_bytes = MAX(0, used_bytes + ?1), updated_at = ?2 WHERE owner_id = ?3",
            params![delta, now, owner_id],
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Crypto Config
    // -----------------------------------------------------------------------

    /// Recupere la configuration cryptographique pour un proprietaire.
    pub fn crypto_config_get(
        &self,
        owner_id: &str,
    ) -> Result<Option<CryptoConfig>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT owner_id, argon2_salt, canary_ciphertext, canary_nonce, key_version, created_at, updated_at
             FROM cloud_crypto_config WHERE owner_id = ?1",
        )?;
        let mut rows = stmt.query_map(params![owner_id], |row| {
            Ok(CryptoConfig {
                owner_id: row.get(0)?,
                argon2_salt: row.get(1)?,
                canary_ciphertext: row.get(2)?,
                canary_nonce: row.get(3)?,
                key_version: row.get::<_, i32>(4).unwrap_or(1) as u32,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;
        match rows.next() {
            Some(r) => Ok(Some(r.map_err(|e| MiyucloudError::Db(e.to_string()))?)),
            None => Ok(None),
        }
    }

    /// Cree ou met a jour la configuration cryptographique.
    pub fn crypto_config_upsert(&self, data: &CryptoConfig) -> Result<(), MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT INTO cloud_crypto_config (owner_id, argon2_salt, canary_ciphertext, canary_nonce, key_version, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(owner_id) DO UPDATE SET
                argon2_salt = excluded.argon2_salt,
                canary_ciphertext = excluded.canary_ciphertext,
                canary_nonce = excluded.canary_nonce,
                key_version = excluded.key_version,
                updated_at = excluded.updated_at",
            params![
                data.owner_id,
                data.argon2_salt,
                data.canary_ciphertext,
                data.canary_nonce,
                data.key_version as i32,
                data.created_at,
                data.updated_at,
            ],
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Sessions
    // -----------------------------------------------------------------------

    /// Insere une nouvelle session.
    pub fn session_insert(
        &self,
        id: &str,
        owner_id: &str,
        token: &str,
        ip_hash: Option<&str>,
        ua: Option<&str>,
        expires_at: &str,
    ) -> Result<(), MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT INTO cloud_sessions (id, owner_id, token, created_ip_hash, created_ua, created_at, expires_at, last_activity_at, is_revoked, totp_verified)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?6, 0, 0)",
            params![id, owner_id, token, ip_hash, ua, now, expires_at],
        )?;
        Ok(())
    }

    /// Recupere une session par son token.
    pub fn session_by_token(&self, token: &str) -> Result<Option<SessionRow>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, owner_id, token, created_ip_hash, created_ua, created_at, expires_at,
                    last_activity_at, is_revoked, totp_verified
             FROM cloud_sessions WHERE token = ?1",
        )?;
        let mut rows = stmt.query_map(params![token], row_to_session)?;
        match rows.next() {
            Some(r) => Ok(Some(r.map_err(|e| MiyucloudError::Db(e.to_string()))?)),
            None => Ok(None),
        }
    }

    /// Liste les sessions d'un proprietaire.
    pub fn sessions_by_owner(&self, owner_id: &str) -> Result<Vec<SessionRow>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, owner_id, token, created_ip_hash, created_ua, created_at, expires_at,
                    last_activity_at, is_revoked, totp_verified
             FROM cloud_sessions WHERE owner_id = ?1 ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(params![owner_id], row_to_session)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| MiyucloudError::Db(e.to_string()))
    }

    /// Met a jour la date de derniere activite d'une session.
    pub fn session_update_activity(&self, id: &str) -> Result<(), MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_sessions SET last_activity_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    /// Revoque une session (soft delete).
    pub fn session_revoke(&self, id: &str) -> Result<(), MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_sessions SET is_revoked = 1 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    /// Revoque toutes les sessions d'un proprietaire. Retourne le nombre de sessions revoquees.
    pub fn session_revoke_all(&self, owner_id: &str) -> Result<u64, MiyucloudError> {
        let conn = self.lock_conn()?;
        let count = conn.execute(
            "UPDATE cloud_sessions SET is_revoked = 1 WHERE owner_id = ?1 AND is_revoked = 0",
            params![owner_id],
        )?;
        Ok(count as u64)
    }

    /// Marque une session comme TOTP verifie.
    pub fn session_mark_totp_verified(&self, id: &str) -> Result<(), MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_sessions SET totp_verified = 1 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    /// Purge les sessions expirees. Retourne le nombre de sessions supprimees.
    pub fn session_purge_expired(&self) -> Result<u64, MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        let count = conn.execute(
            "DELETE FROM cloud_sessions WHERE expires_at < ?1",
            params![now],
        )?;
        Ok(count as u64)
    }

    /// Compte les sessions actives (non-revoquees, non-expirees) d'un proprietaire.
    pub fn session_count_by_owner(&self, owner_id: &str) -> Result<u64, MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM cloud_sessions WHERE owner_id = ?1 AND is_revoked = 0 AND expires_at >= ?2",
            params![owner_id, now],
            |row| row.get(0),
        )?;
        Ok(count as u64)
    }

    /// Retourne l'ID de la session active la plus ancienne d'un proprietaire.
    pub fn session_oldest_by_owner(
        &self,
        owner_id: &str,
    ) -> Result<Option<String>, MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id FROM cloud_sessions WHERE owner_id = ?1 AND is_revoked = 0 AND expires_at >= ?2
             ORDER BY created_at ASC LIMIT 1",
        )?;
        let mut rows = stmt.query_map(params![owner_id, now], |row| row.get::<_, String>(0))?;
        match rows.next() {
            Some(r) => Ok(Some(r.map_err(|e| MiyucloudError::Db(e.to_string()))?)),
            None => Ok(None),
        }
    }

    // -----------------------------------------------------------------------
    // TOTP
    // -----------------------------------------------------------------------

    /// Insere un secret TOTP pour un proprietaire.
    pub fn totp_insert(
        &self,
        id: &str,
        owner_id: &str,
        encrypted_secret: &str,
        nonce: &str,
        account_name: &str,
    ) -> Result<(), MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT INTO cloud_totp_secrets (id, owner_id, encrypted_secret, encryption_nonce, issuer, account_name, algorithm, digits, period, is_active, created_at, last_used_at)
             VALUES (?1, ?2, ?3, ?4, 'MiyuCloud', ?5, 'SHA1', 6, 30, 0, ?6, NULL)",
            params![id, owner_id, encrypted_secret, nonce, account_name, now],
        )?;
        Ok(())
    }

    /// Recupere le secret TOTP d'un proprietaire.
    pub fn totp_get_by_owner(&self, owner_id: &str) -> Result<Option<TotpRow>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, owner_id, encrypted_secret, encryption_nonce, issuer, account_name,
                    algorithm, digits, period, is_active, created_at, last_used_at
             FROM cloud_totp_secrets WHERE owner_id = ?1",
        )?;
        let mut rows = stmt.query_map(params![owner_id], row_to_totp)?;
        match rows.next() {
            Some(r) => Ok(Some(r.map_err(|e| MiyucloudError::Db(e.to_string()))?)),
            None => Ok(None),
        }
    }

    /// Active le TOTP d'un proprietaire.
    pub fn totp_activate(&self, owner_id: &str) -> Result<(), MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_totp_secrets SET is_active = 1 WHERE owner_id = ?1",
            params![owner_id],
        )?;
        Ok(())
    }

    /// Supprime le secret TOTP d'un proprietaire.
    pub fn totp_delete(&self, owner_id: &str) -> Result<(), MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "DELETE FROM cloud_totp_secrets WHERE owner_id = ?1",
            params![owner_id],
        )?;
        Ok(())
    }

    /// Met a jour la date de derniere utilisation du TOTP.
    pub fn totp_update_last_used(&self, owner_id: &str) -> Result<(), MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_totp_secrets SET last_used_at = ?1 WHERE owner_id = ?2 AND is_active = 1",
            params![now, owner_id],
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Recovery Codes
    // -----------------------------------------------------------------------

    /// Insere un code de recuperation.
    pub fn recovery_code_insert(
        &self,
        id: &str,
        owner_id: &str,
        code_hash: &str,
    ) -> Result<(), MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT INTO cloud_recovery_codes (id, owner_id, code_hash, is_used, created_at, used_at)
             VALUES (?1, ?2, ?3, 0, ?4, NULL)",
            params![id, owner_id, code_hash, now],
        )?;
        Ok(())
    }

    /// Liste les codes de recuperation d'un proprietaire.
    pub fn recovery_codes_by_owner(
        &self,
        owner_id: &str,
    ) -> Result<Vec<RecoveryCodeRow>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, owner_id, code_hash, is_used, created_at, used_at
             FROM cloud_recovery_codes WHERE owner_id = ?1 ORDER BY created_at",
        )?;
        let rows = stmt.query_map(params![owner_id], row_to_recovery_code)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| MiyucloudError::Db(e.to_string()))
    }

    /// Marque un code de recuperation comme utilise.
    pub fn recovery_code_mark_used(&self, id: &str) -> Result<(), MiyucloudError> {
        let now = now_iso();
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_recovery_codes SET is_used = 1, used_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    /// Supprime tous les codes de recuperation d'un proprietaire.
    pub fn recovery_codes_delete_all(&self, owner_id: &str) -> Result<(), MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "DELETE FROM cloud_recovery_codes WHERE owner_id = ?1",
            params![owner_id],
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Onboarding
    // -----------------------------------------------------------------------

    /// Recupere l'etat d'onboarding d'un proprietaire.
    pub fn onboarding_get(&self, owner_id: &str) -> Result<Option<OnboardingRow>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT owner_id, current_step, passphrase_set, totp_set, storage_verified, completed, completed_at
             FROM cloud_onboarding WHERE owner_id = ?1",
        )?;
        let mut rows = stmt.query_map(params![owner_id], row_to_onboarding)?;
        match rows.next() {
            Some(r) => Ok(Some(r.map_err(|e| MiyucloudError::Db(e.to_string()))?)),
            None => Ok(None),
        }
    }

    /// Cree ou met a jour l'etat d'onboarding d'un proprietaire.
    ///
    /// Utilise les champs de `data` pour l'upsert. Si `data.completed` est vrai
    /// et `data.completed_at` est `None`, la date courante est utilisee.
    pub fn onboarding_upsert(&self, data: &OnboardingRow) -> Result<(), MiyucloudError> {
        let completed_at = if data.completed && data.completed_at.is_none() {
            Some(now_iso())
        } else {
            data.completed_at.clone()
        };
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT INTO cloud_onboarding (owner_id, current_step, passphrase_set, totp_set, storage_verified, completed, completed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(owner_id) DO UPDATE SET
                current_step = excluded.current_step,
                passphrase_set = excluded.passphrase_set,
                totp_set = excluded.totp_set,
                storage_verified = excluded.storage_verified,
                completed = excluded.completed,
                completed_at = excluded.completed_at",
            params![
                data.owner_id,
                data.current_step as i32,
                data.passphrase_set as i32,
                data.totp_set as i32,
                data.storage_verified as i32,
                data.completed as i32,
                completed_at,
            ],
        )?;
        Ok(())
    }

    /// Supprime l'etat d'onboarding d'un proprietaire.
    pub fn onboarding_delete(&self, owner_id: &str) -> Result<(), MiyucloudError> {
        let conn = self.lock_conn()?;
        conn.execute(
            "DELETE FROM cloud_onboarding WHERE owner_id = ?1",
            params![owner_id],
        )?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Row mapping helpers
// ---------------------------------------------------------------------------

fn row_to_file(row: &rusqlite::Row<'_>) -> rusqlite::Result<FileEntry> {
    Ok(FileEntry {
        id: row.get(0)?,
        parent_id: row.get(1)?,
        owner_id: row.get(2)?,
        name: row.get(3)?,
        mime_type: row.get(4)?,
        size_bytes: row.get::<_, i64>(5).unwrap_or(0) as u64,
        checksum_sha256: row.get(6)?,
        encryption_iv: row.get(7)?,
        chunk_count: row.get::<_, i32>(8).unwrap_or(1) as u32,
        is_trashed: row.get::<_, i32>(9).unwrap_or(0) != 0,
        trashed_at: row.get(10)?,
        created_at: row.get(11)?,
        updated_at: row.get(12)?,
    })
}

fn row_to_folder(row: &rusqlite::Row<'_>) -> rusqlite::Result<FolderEntry> {
    Ok(FolderEntry {
        id: row.get(0)?,
        parent_id: row.get(1)?,
        owner_id: row.get(2)?,
        name: row.get(3)?,
        is_trashed: row.get::<_, i32>(4).unwrap_or(0) != 0,
        trashed_at: row.get(5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}

fn row_to_share_link(row: &rusqlite::Row<'_>) -> rusqlite::Result<ShareLink> {
    Ok(ShareLink {
        id: row.get(0)?,
        file_id: row.get(1)?,
        folder_id: row.get(2)?,
        creator_id: row.get(3)?,
        token: row.get(4)?,
        has_password: row.get::<_, Option<String>>(5)?.is_some(),
        expires_at: row.get(6)?,
        max_downloads: row.get::<_, Option<i64>>(7)?.map(|v| v as u32),
        download_count: row.get::<_, i64>(8).unwrap_or(0) as u32,
        is_revoked: row.get::<_, i32>(9).unwrap_or(0) != 0,
        created_at: row.get(10)?,
    })
}

fn row_to_sync_conflict(row: &rusqlite::Row<'_>) -> rusqlite::Result<SyncConflict> {
    Ok(SyncConflict {
        id: row.get(0)?,
        file_id: row.get(1)?,
        local_version: row.get(2)?,
        remote_version: row.get(3)?,
        remote_node_id: row.get(4)?,
        status: ConflictStatus::from_str_value(&row.get::<_, String>(5).unwrap_or_default()),
        created_at: row.get(6)?,
        resolved_at: row.get(7)?,
    })
}

fn row_to_access_log(row: &rusqlite::Row<'_>) -> rusqlite::Result<AccessLogEntry> {
    Ok(AccessLogEntry {
        id: row.get(0)?,
        share_link_id: row.get(1)?,
        accessor_ip: row.get(2)?,
        accessor_ua: row.get(3)?,
        action: AccessAction::from_str_value(&row.get::<_, String>(4).unwrap_or_default()),
        file_id: row.get(5)?,
        success: row.get::<_, i32>(6).unwrap_or(1) != 0,
        created_at: row.get(7)?,
    })
}

fn row_to_session(row: &rusqlite::Row<'_>) -> rusqlite::Result<SessionRow> {
    Ok(SessionRow {
        id: row.get(0)?,
        owner_id: row.get(1)?,
        token: row.get(2)?,
        created_ip_hash: row.get(3)?,
        created_ua: row.get(4)?,
        created_at: row.get(5)?,
        expires_at: row.get(6)?,
        last_activity_at: row.get(7)?,
        is_revoked: row.get::<_, i32>(8).unwrap_or(0) != 0,
        totp_verified: row.get::<_, i32>(9).unwrap_or(0) != 0,
    })
}

fn row_to_totp(row: &rusqlite::Row<'_>) -> rusqlite::Result<TotpRow> {
    Ok(TotpRow {
        id: row.get(0)?,
        owner_id: row.get(1)?,
        encrypted_secret: row.get(2)?,
        encryption_nonce: row.get(3)?,
        issuer: row.get(4)?,
        account_name: row.get(5)?,
        algorithm: row.get(6)?,
        digits: row.get::<_, i32>(7).unwrap_or(6) as u32,
        period: row.get::<_, i32>(8).unwrap_or(30) as u32,
        is_active: row.get::<_, i32>(9).unwrap_or(0) != 0,
        created_at: row.get(10)?,
        last_used_at: row.get(11)?,
    })
}

fn row_to_recovery_code(row: &rusqlite::Row<'_>) -> rusqlite::Result<RecoveryCodeRow> {
    Ok(RecoveryCodeRow {
        id: row.get(0)?,
        owner_id: row.get(1)?,
        code_hash: row.get(2)?,
        is_used: row.get::<_, i32>(3).unwrap_or(0) != 0,
        created_at: row.get(4)?,
        used_at: row.get(5)?,
    })
}

fn row_to_onboarding(row: &rusqlite::Row<'_>) -> rusqlite::Result<OnboardingRow> {
    Ok(OnboardingRow {
        owner_id: row.get(0)?,
        current_step: row.get::<_, i32>(1).unwrap_or(1) as u32,
        passphrase_set: row.get::<_, i32>(2).unwrap_or(0) != 0,
        totp_set: row.get::<_, i32>(3).unwrap_or(0) != 0,
        storage_verified: row.get::<_, i32>(4).unwrap_or(0) != 0,
        completed: row.get::<_, i32>(5).unwrap_or(0) != 0,
        completed_at: row.get(6)?,
    })
}

// -----------------------------------------------------------------------
// ContentAddressableStorage
// -----------------------------------------------------------------------

impl ContentAddressableStorage for MiyucloudDb {
    /// Store raw `data` as a deduplicated blob.
    ///
    /// If the blob already exists (same SHA-256 hash), increments its
    /// ref_count and returns the existing hash.  Otherwise compresses
    /// (if above threshold), inserts a new row, and returns the hash.
    fn store_blob(&self, data: &[u8]) -> Result<ContentHash, MiyucloudError> {
        let hash = ContentHash::from_data(data);
        let hex = hash.to_hex();

        if self.blob_exists(&hash)? {
            self.increment_refcount(&hash)?;
            return Ok(hash);
        }

        let (stored, compressed) = maybe_compress(data)?;
        let size = data.len() as i64;
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT OR IGNORE INTO cloud_content_blobs (hash, data, size, compressed, ref_count)
             VALUES (?1, ?2, ?3, ?4, 1)",
            params![hex, stored, size, compressed as i32],
        )?;
        Ok(hash)
    }

    /// Read and decompress (if needed) a stored blob by its hash.
    fn read_blob(&self, hash: &ContentHash) -> Result<Vec<u8>, MiyucloudError> {
        let hex = hash.to_hex();
        let conn = self.lock_conn()?;
        let (data, compressed): (Vec<u8>, bool) = conn
            .query_row(
                "SELECT data, compressed FROM cloud_content_blobs WHERE hash = ?1",
                params![hex],
                |row| Ok((row.get::<_, Vec<u8>>(0)?, row.get::<_, i32>(1)? != 0)),
            )
            .map_err(|e| MiyucloudError::Db(format!("blob not found ({hex}): {e}")))?;

        if compressed {
            decompress(&data)
        } else {
            Ok(data)
        }
    }

    /// Return `true` if a blob with this hash already exists in the store.
    fn blob_exists(&self, hash: &ContentHash) -> Result<bool, MiyucloudError> {
        let hex = hash.to_hex();
        let conn = self.lock_conn()?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM cloud_content_blobs WHERE hash = ?1",
            params![hex],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    /// Increment the reference count of a blob (called on dedup hit).
    fn increment_refcount(&self, hash: &ContentHash) -> Result<(), MiyucloudError> {
        let hex = hash.to_hex();
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_content_blobs SET ref_count = ref_count + 1 WHERE hash = ?1",
            params![hex],
        )?;
        Ok(())
    }

    /// Decrement the reference count; delete the blob when it reaches zero.
    fn decrement_refcount(&self, hash: &ContentHash) -> Result<(), MiyucloudError> {
        let hex = hash.to_hex();
        let conn = self.lock_conn()?;
        conn.execute(
            "UPDATE cloud_content_blobs SET ref_count = ref_count - 1 WHERE hash = ?1",
            params![hex],
        )?;
        conn.execute(
            "DELETE FROM cloud_content_blobs WHERE hash = ?1 AND ref_count <= 0",
            params![hex],
        )?;
        Ok(())
    }
}

impl MiyucloudDb {
    /// Associate a file ID with its content blob hash in `cloud_file_blobs`.
    pub fn record_file_blob(
        &self,
        file_id: &str,
        hash: &ContentHash,
    ) -> Result<(), MiyucloudError> {
        let hex = hash.to_hex();
        let conn = self.lock_conn()?;
        conn.execute(
            "INSERT OR IGNORE INTO cloud_file_blobs (file_id, blob_hash) VALUES (?1, ?2)",
            params![file_id, hex],
        )?;
        Ok(())
    }

    /// Remove the file→blob association and decrement the blob's ref_count.
    pub fn remove_file_blob(
        &self,
        file_id: &str,
        hash: &ContentHash,
    ) -> Result<(), MiyucloudError> {
        let hex = hash.to_hex();
        let conn = self.lock_conn()?;
        conn.execute(
            "DELETE FROM cloud_file_blobs WHERE file_id = ?1 AND blob_hash = ?2",
            params![file_id, hex],
        )?;
        drop(conn);
        self.decrement_refcount(hash)
    }

    /// Return the blob hash associated with a file, if any.
    pub fn file_blob_hash(&self, file_id: &str) -> Result<Option<ContentHash>, MiyucloudError> {
        let conn = self.lock_conn()?;
        let mut stmt =
            conn.prepare("SELECT blob_hash FROM cloud_file_blobs WHERE file_id = ?1")?;
        let mut rows = stmt.query_map(params![file_id], |row| row.get::<_, String>(0))?;
        match rows.next() {
            Some(r) => {
                let hex = r.map_err(|e| MiyucloudError::Db(e.to_string()))?;
                Ok(Some(ContentHash::from_hex(&hex)?))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mem_db() -> MiyucloudDb {
        MiyucloudDb::open(":memory:").unwrap()
    }

    fn make_file(owner_id: &str, parent_id: Option<&str>) -> FileEntry {
        let now = now_iso();
        FileEntry {
            id: uuid::Uuid::new_v4().to_string(),
            parent_id: parent_id.map(String::from),
            owner_id: owner_id.to_string(),
            name: "test.txt".to_string(),
            mime_type: "text/plain".to_string(),
            size_bytes: 100,
            checksum_sha256: "abc123".to_string(),
            encryption_iv: Some("iv".to_string()),
            chunk_count: 1,
            is_trashed: false,
            trashed_at: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    fn make_folder(owner_id: &str, parent_id: Option<&str>) -> FolderEntry {
        let now = now_iso();
        FolderEntry {
            id: uuid::Uuid::new_v4().to_string(),
            parent_id: parent_id.map(String::from),
            owner_id: owner_id.to_string(),
            name: "TestFolder".to_string(),
            is_trashed: false,
            trashed_at: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    #[test]
    fn test_file_create_and_get() {
        let db = mem_db();
        let file = make_file("owner1", None);
        db.file_create(&file).unwrap();
        let result = db.file_by_id(&file.id).unwrap();
        assert!(result.is_some());
        let f = result.unwrap();
        assert_eq!(f.name, "test.txt");
        assert_eq!(f.owner_id, "owner1");
    }

    #[test]
    fn test_file_list_by_parent() {
        let db = mem_db();
        let folder = make_folder("owner1", None);
        db.folder_create(&folder).unwrap();
        let file = make_file("owner1", Some(&folder.id));
        db.file_create(&file).unwrap();
        let orphan = make_file("owner1", None);
        db.file_create(&orphan).unwrap();

        let root_files = db.file_list(None, "owner1").unwrap();
        assert_eq!(root_files.len(), 1);
        let folder_files = db.file_list(Some(&folder.id), "owner1").unwrap();
        assert_eq!(folder_files.len(), 1);
    }

    #[test]
    fn test_file_trash_and_restore() {
        let db = mem_db();
        let file = make_file("owner1", None);
        db.file_create(&file).unwrap();

        db.file_trash(&file.id).unwrap();
        let f = db.file_by_id(&file.id).unwrap().unwrap();
        assert!(f.is_trashed);
        assert!(f.trashed_at.is_some());

        // Trashed files should not appear in normal listing
        let files = db.file_list(None, "owner1").unwrap();
        assert_eq!(files.len(), 0);

        // But should appear in trashed listing
        let trashed = db.file_list_trashed("owner1").unwrap();
        assert_eq!(trashed.len(), 1);

        db.file_restore(&file.id).unwrap();
        let f = db.file_by_id(&file.id).unwrap().unwrap();
        assert!(!f.is_trashed);
        assert!(f.trashed_at.is_none());

        let files = db.file_list(None, "owner1").unwrap();
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_folder_create_and_contents() {
        let db = mem_db();
        let folder = make_folder("owner1", None);
        db.folder_create(&folder).unwrap();
        let file = make_file("owner1", Some(&folder.id));
        db.file_create(&file).unwrap();

        let contents = db.folder_contents(Some(&folder.id), "owner1").unwrap();
        assert!(contents.folder.is_some());
        assert_eq!(contents.files.len(), 1);
        assert_eq!(contents.subfolders.len(), 0);
    }

    #[test]
    fn test_share_link_create_and_revoke() {
        let db = mem_db();
        let file = make_file("owner1", None);
        db.file_create(&file).unwrap();
        let now = now_iso();
        let link = ShareLink {
            id: uuid::Uuid::new_v4().to_string(),
            file_id: Some(file.id.clone()),
            folder_id: None,
            creator_id: "owner1".to_string(),
            token: "test-token-12345".to_string(),
            has_password: false,
            expires_at: "2026-04-01T00:00:00Z".to_string(),
            max_downloads: Some(5),
            download_count: 0,
            is_revoked: false,
            created_at: now,
        };
        db.share_link_create(&link, None).unwrap();

        let found = db.share_link_by_token("test-token-12345").unwrap();
        assert!(found.is_some());
        let l = found.unwrap();
        assert_eq!(l.token, "test-token-12345");
        assert!(!l.is_revoked);

        db.share_link_revoke(&l.id).unwrap();
        let revoked = db.share_link_by_token("test-token-12345").unwrap().unwrap();
        assert!(revoked.is_revoked);
    }

    #[test]
    fn test_quota_update() {
        let db = mem_db();
        let q = db.quota_get("owner1").unwrap();
        assert_eq!(q.used_bytes, 0);

        db.quota_update_used("owner1", 1024).unwrap();
        let q = db.quota_get("owner1").unwrap();
        assert_eq!(q.used_bytes, 1024);

        db.quota_update_used("owner1", -512).unwrap();
        let q = db.quota_get("owner1").unwrap();
        assert_eq!(q.used_bytes, 512);

        // Negative delta should not go below zero
        db.quota_update_used("owner1", -1000).unwrap();
        let q = db.quota_get("owner1").unwrap();
        assert_eq!(q.used_bytes, 0);
    }

    #[test]
    fn test_crypto_config_upsert_and_get() {
        let db = mem_db();
        let now = now_iso();
        let cfg = CryptoConfig {
            owner_id: "owner1".to_string(),
            argon2_salt: "c2FsdA==".to_string(),
            canary_ciphertext: Some("enc".to_string()),
            canary_nonce: Some("nonce".to_string()),
            key_version: 1,
            created_at: now.clone(),
            updated_at: now,
        };
        db.crypto_config_upsert(&cfg).unwrap();
        let found = db.crypto_config_get("owner1").unwrap();
        assert!(found.is_some());
        let c = found.unwrap();
        assert_eq!(c.argon2_salt, "c2FsdA==");
        assert_eq!(c.canary_ciphertext.as_deref(), Some("enc"));
    }

    #[test]
    fn test_file_search() {
        let db = mem_db();
        let mut f1 = make_file("owner1", None);
        f1.name = "photo_vacances.jpg".to_string();
        db.file_create(&f1).unwrap();
        let mut f2 = make_file("owner1", None);
        f2.name = "document.pdf".to_string();
        db.file_create(&f2).unwrap();

        let results = db.file_search("owner1", "photo").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "photo_vacances.jpg");
    }

    // -------------------------------------------------------------------
    // Dedup / ContentAddressableStorage tests (E2-06, E2-07, E2-08)
    // -------------------------------------------------------------------

    #[test]
    fn test_dedup_store_and_read_blob() {
        // E2-06: store a blob and retrieve the same bytes back
        use crate::storage::dedup::ContentAddressableStorage;
        let db = mem_db();
        let data = b"hello dedup world";
        let hash = db.store_blob(data).unwrap();
        assert!(db.blob_exists(&hash).unwrap());
        let retrieved = db.read_blob(&hash).unwrap();
        assert_eq!(retrieved, data);
    }

    #[test]
    fn test_dedup_upload_same_file_twice_one_blob() {
        // E2-07: uploading the same content twice should produce one blob
        // with ref_count = 2 and two distinct file→blob associations.
        use crate::domain::dedup_ops;
        let db = mem_db();
        let data = b"deduplicated content";

        // Create two distinct file entries so the FK constraint is satisfied
        let fa = make_file("owner1", None);
        let fb = make_file("owner1", None);
        db.file_create(&fa).unwrap();
        db.file_create(&fb).unwrap();

        let hash1 = dedup_ops::dedup_upload(&db, data).unwrap();
        db.record_file_blob(&fa.id, &hash1).unwrap();

        let hash2 = dedup_ops::dedup_upload(&db, data).unwrap();
        db.record_file_blob(&fb.id, &hash2).unwrap();

        assert_eq!(hash1, hash2);

        // Only one blob should exist
        let conn = db.lock_conn().unwrap();
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM cloud_content_blobs WHERE hash = ?1",
                rusqlite::params![hash1.to_hex()],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);

        let ref_count: i64 = conn
            .query_row(
                "SELECT ref_count FROM cloud_content_blobs WHERE hash = ?1",
                rusqlite::params![hash1.to_hex()],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(ref_count, 2);
    }

    #[test]
    fn test_dedup_delete_decrements_refcount_and_removes() {
        // E2-08: deleting the last reference to a blob removes it entirely.
        use crate::domain::dedup_ops;
        use crate::storage::dedup::ContentAddressableStorage;
        let db = mem_db();
        let data = b"ephemeral content";

        // Create a real file entry to satisfy the FK constraint
        let fx = make_file("owner1", None);
        db.file_create(&fx).unwrap();

        let hash = dedup_ops::dedup_upload(&db, data).unwrap();
        db.record_file_blob(&fx.id, &hash).unwrap();
        assert!(db.blob_exists(&hash).unwrap());

        // Remove the file→blob link and decrement ref
        db.remove_file_blob(&fx.id, &hash).unwrap();

        // Blob should be gone (ref_count was 1, now 0 → deleted)
        assert!(!db.blob_exists(&hash).unwrap());
    }
}
