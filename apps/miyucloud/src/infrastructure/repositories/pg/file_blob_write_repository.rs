//! @id     miyucloud_infrastructure_repositories_pg_file_blob_write_repository
//! @do     definir_file_blob_write_repository
//! @role   donnee
//! @layer  app
//! @human  miyucloud — module « infrastructure repositories pg file blob write repository ».

//! PostgreSQL + Blob-backed file write repository.
//!
//! Implements `FileWritePort` using:
//! - `storage.files` table for metadata
//! - `DedupPort` for content-addressable blob storage on the filesystem
//!
//! File paths are resolved by querying the materialized `storage.folders.path`
//! column (O(1) per lookup), so no recursive CTEs are needed.

use sqlx::PgPool;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::ports::storage_ports::{CopyFolderTreeResult, FileWritePort};
use crate::common::errors::DomainError;
use crate::domain::entities::etat_cdr::MarqueCdr;
use crate::domain::entities::file::File;
use crate::domain::services::path_service::StoragePath;

use super::folder_db_repository::FolderDbRepository;
use crate::infrastructure::services::dedup_service::{BlobPrepare, DedupService};

/// File write repository backed by PostgreSQL metadata + blob storage.
pub struct FileBlobWriteRepository {
    pool: Arc<PgPool>,
    dedup: Arc<DedupService>,
    folder_repo: Arc<FolderDbRepository>,
}

impl FileBlobWriteRepository {
    pub fn new(
        pool: Arc<PgPool>,
        dedup: Arc<DedupService>,
        folder_repo: Arc<FolderDbRepository>,
    ) -> Self {
        Self {
            pool,
            dedup,
            folder_repo,
        }
    }

    /// Creates a stub instance for testing — never hits PG.
    #[cfg(test)]
    pub fn new_stub() -> Self {
        use crate::infrastructure::services::dedup_service::DedupService;
        Self {
            pool: Arc::new(
                sqlx::pool::PoolOptions::<sqlx::Postgres>::new()
                    .max_connections(1)
                    .connect_lazy("postgres://invalid:5432/none")
                    .unwrap(),
            ),
            dedup: Arc::new(DedupService::new_stub()),
            folder_repo: Arc::new(super::folder_db_repository::FolderDbRepository::new_stub()),
        }
    }

    /// Build a `StoragePath` from the materialized folder path + file name.
    fn make_file_path(folder_path: Option<&str>, file_name: &str) -> StoragePath {
        match folder_path {
            Some(fp) if !fp.is_empty() => StoragePath::from_string(&format!("{fp}/{file_name}")),
            _ => StoragePath::from_string(file_name),
        }
    }

    /// Look up the materialized folder path. O(1) — no recursive CTE.
    async fn lookup_folder_path(
        &self,
        folder_id: Option<&str>,
    ) -> Result<Option<String>, DomainError> {
        match folder_id {
            Some(fid) => {
                let path: String =
                    sqlx::query_scalar("SELECT path FROM storage.folders WHERE id = $1::uuid")
                        .bind(fid)
                        .fetch_optional(self.pool.as_ref())
                        .await
                        .map_err(|e| {
                            DomainError::internal_error(
                                "FileBlobWrite",
                                format!("folder path: {e}"),
                            )
                        })?
                        .ok_or_else(|| DomainError::not_found("Folder", fid))?;
                Ok(Some(path))
            }
            None => Ok(None),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn row_to_file(
        id: String,
        name: String,
        folder_id: Option<String>,
        folder_path: Option<String>,
        size: i64,
        mime_type: String,
        created_at: i64,
        modified_at: i64,
        owner_id: Option<Uuid>,
        etag: String,
    ) -> Result<File, DomainError> {
        let storage_path = Self::make_file_path(folder_path.as_deref(), &name);
        File::with_timestamps_and_etag(
            id,
            name,
            storage_path,
            size as u64,
            mime_type,
            folder_id,
            created_at as u64,
            modified_at as u64,
            owner_id,
            etag,
        )
        .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("entity: {e}")))
    }

    /// Derive user_id from the parent folder, or error if folder_id is None.
    async fn resolve_user_id(&self, folder_id: Option<&str>) -> Result<Uuid, DomainError> {
        match folder_id {
            Some(fid) => self.folder_repo.get_folder_user_id(fid).await,
            None => Err(DomainError::internal_error(
                "FileBlobWrite",
                "folder_id is required to determine file owner",
            )),
        }
    }

    /// Remplace le blob d'un fichier, **en une transaction** : ligne mise à
    /// jour, référence du nouveau blob prise, celle de l'ancien retirée.
    /// Aucune compensation hors transaction : un échec annule tout (le blob
    /// placé sur disque sans référence est repris par le ramasse-miettes).
    ///
    /// `modified_at`: if `Some`, sets `updated_at` to that Unix timestamp;
    /// if `None`, uses `NOW()` (server time). Returns the new hash on success.
    #[allow(clippy::too_many_arguments)]
    async fn swap_blob_hash(
        &self,
        file_id: &str,
        prepare: &BlobPrepare,
        content_type: Option<String>,
        new_size: i64,
        modified_at: Option<i64>,
        marque: &MarqueCdr,
        mime_type: Option<&str>,
    ) -> Result<String, DomainError> {
        let resultat = self
            .swap_blob_hash_tx(
                file_id,
                prepare,
                content_type,
                new_size,
                modified_at,
                marque,
                mime_type,
            )
            .await;
        // Source libérée quelle que soit l'issue.
        self.dedup.liberer_source(prepare).await;
        resultat
    }

    #[allow(clippy::too_many_arguments)]
    async fn swap_blob_hash_tx(
        &self,
        file_id: &str,
        prepare: &BlobPrepare,
        content_type: Option<String>,
        new_size: i64,
        modified_at: Option<i64>,
        marque: &MarqueCdr,
        mime_type: Option<&str>,
    ) -> Result<String, DomainError> {
        let erreur = |contexte: &str, e: sqlx::Error| {
            DomainError::internal_error("FileBlobWrite", format!("{contexte}: {e}"))
        };
        let mut tx = self.pool.begin().await.map_err(|e| erreur("begin", e))?;
        // Atomic CTE: capture old hash then update in one round-trip, no TOCTOU.
        let old_hash = sqlx::query_scalar::<_, String>(
            r#"
            WITH old AS (
                SELECT id, blob_hash FROM storage.files WHERE id = $3::uuid FOR UPDATE
            )
            UPDATE storage.files f
               SET blob_hash = $1, size = $2,
                   updated_at = COALESCE(to_timestamp($4), NOW()),
                   etat_cdr = $5, cdr_rapport = $6,
                   mime_type = COALESCE($7, f.mime_type)
              FROM old
             WHERE f.id = old.id
            RETURNING old.blob_hash
            "#,
        )
        .bind(&prepare.hash)
        .bind(new_size)
        .bind(file_id)
        .bind(modified_at.map(|t| t as f64))
        .bind(marque.etat.as_str())
        .bind(&marque.rapport)
        .bind(mime_type)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| erreur("update", e))?
        .ok_or_else(|| DomainError::not_found("File", file_id))?;

        if old_hash != prepare.hash {
            self.dedup
                .referencer_blob(&mut tx, prepare, content_type)
                .await?;
            DedupService::decrementer_reference(&mut tx, &old_hash).await?;
        }
        tx.commit().await.map_err(|e| erreur("commit", e))?;
        Ok(prepare.hash.clone())
    }

    /// Ligne `storage.files` + référence du blob préparé, en une transaction.
    #[allow(clippy::too_many_arguments)]
    async fn inserer_fichier_tx(
        &self,
        prepare: &BlobPrepare,
        name: &str,
        folder_id: &Option<String>,
        user_id: Uuid,
        size: u64,
        content_type: &str,
        marque: &MarqueCdr,
    ) -> Result<(String, i64, i64), DomainError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("begin: {e}")))?;

        let row = match sqlx::query_as::<_, (String, i64, i64)>(
            r#"
            INSERT INTO storage.files (name, folder_id, user_id, blob_hash, size, mime_type,
                                       etat_cdr, cdr_rapport)
            VALUES ($1, $2::uuid, $3, $4, $5, $6, $7, $8)
            RETURNING id::text,
                      EXTRACT(EPOCH FROM created_at)::bigint,
                      EXTRACT(EPOCH FROM updated_at)::bigint
            "#,
        )
        .bind(name)
        .bind(folder_id)
        .bind(user_id)
        .bind(&prepare.hash)
        .bind(size as i64)
        .bind(content_type)
        .bind(marque.etat.as_str())
        .bind(&marque.rapport)
        .fetch_one(&mut *tx)
        .await
        {
            Ok(row) => row,
            Err(e) => {
                // Annulation implicite (tx abandonnée) : aucune référence prise.
                if let sqlx::Error::Database(ref db_err) = e
                    && db_err.code().as_deref() == Some("23505")
                {
                    return Err(DomainError::already_exists(
                        "File",
                        format!("'{name}' already exists in this folder"),
                    ));
                }
                return Err(DomainError::internal_error(
                    "FileBlobWrite",
                    format!("insert: {e}"),
                ));
            }
        };

        self.dedup
            .referencer_blob(&mut tx, prepare, Some(content_type.to_string()))
            .await?;
        tx.commit()
            .await
            .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("commit: {e}")))?;
        Ok(row)
    }

    /// Range un fichier temporaire (déjà passé par l'entonnoir CDR) et crée sa
    /// ligne `storage.files` avec son état CDR, en une seule insertion.
    #[allow(clippy::too_many_arguments)]
    pub async fn save_file_from_temp_cdr(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        temp_path: &std::path::Path,
        size: u64,
        pre_computed_hash: Option<String>,
        marque: &MarqueCdr,
    ) -> Result<File, DomainError> {
        let user_id = self.resolve_user_id(folder_id.as_deref()).await?;

        // Blob placé sur disque hors transaction (pre-computed hash : aucune
        // relecture), puis ligne + référence dans **une** transaction : pas
        // de compensation hors transaction, pas de référence orpheline.
        let prepare = self
            .dedup
            .preparer_blob(temp_path, pre_computed_hash)
            .await?;
        let blob_hash = prepare.hash.clone();
        // La source est libérée quelle que soit l'issue (elle était
        // consommée par le magasin de blobs avant cette révision).
        let insertion = self
            .inserer_fichier_tx(
                &prepare,
                &name,
                &folder_id,
                user_id,
                size,
                &content_type,
                marque,
            )
            .await;
        self.dedup.liberer_source(&prepare).await;
        let row = insertion?;

        tracing::info!(
            "📡 STREAMING WRITE: {} ({} bytes, hash: {})",
            name,
            size,
            blob_hash.get(..12).unwrap_or(&blob_hash)
        );

        let folder_path = self.lookup_folder_path(folder_id.as_deref()).await?;
        Self::row_to_file(
            row.0,
            name,
            folder_id,
            folder_path,
            size as i64,
            content_type,
            row.1,
            row.2,
            Some(user_id),
            blob_hash.clone(),
        )
    }

    /// Remplace le contenu du fichier `file_id` par un fichier temporaire
    /// (déjà passé par l'entonnoir CDR) et met à jour son état CDR dans la
    /// même instruction que l'échange de blob.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_file_content_from_temp_cdr(
        &self,
        file_id: &str,
        temp_path: &std::path::Path,
        size: u64,
        content_type: Option<String>,
        pre_computed_hash: Option<String>,
        modified_at: Option<i64>,
        marque: &MarqueCdr,
    ) -> Result<String, DomainError> {
        // Streaming: pass pre-computed hash so dedup skips re-reading the file.
        let prepare = self
            .dedup
            .preparer_blob(temp_path, pre_computed_hash)
            .await?;

        // Le type MIME suit le contenu stocké (reconstruction GIF → PNG…).
        self.swap_blob_hash(
            file_id,
            &prepare,
            content_type.clone(),
            size as i64,
            modified_at,
            marque,
            content_type.as_deref(),
        )
        .await
    }
}

impl FileWritePort for FileBlobWriteRepository {
    /// Écriture hors entonnoir CDR : état `inconnu` (traité comme non désarmé).
    /// Les chemins d'écriture de MiyuCloud passent par `FileUploadService`,
    /// qui appelle [`FileBlobWriteRepository::save_file_from_temp_cdr`].
    async fn save_file_from_temp(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        temp_path: &std::path::Path,
        size: u64,
        pre_computed_hash: Option<String>,
    ) -> Result<File, DomainError> {
        self.save_file_from_temp_cdr(
            name,
            folder_id,
            content_type,
            temp_path,
            size,
            pre_computed_hash,
            &MarqueCdr::inconnue(),
        )
        .await
    }

    async fn move_file(
        &self,
        file_id: &str,
        target_folder_id: Option<String>,
    ) -> Result<File, DomainError> {
        // If moving to a different folder, get the new user_id (must be same user)
        let row = sqlx::query_as::<_, (String, String, Option<String>, i64, String, i64, i64)>(
            r#"
            UPDATE storage.files
               SET folder_id = $1::uuid, updated_at = NOW()
             WHERE id = $2::uuid AND NOT is_trashed
            RETURNING id::text, name, folder_id::text, size, mime_type,
                      EXTRACT(EPOCH FROM created_at)::bigint,
                      EXTRACT(EPOCH FROM updated_at)::bigint
            "#,
        )
        .bind(&target_folder_id)
        .bind(file_id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("move: {e}")))?
        .ok_or_else(|| DomainError::not_found("File", file_id))?;

        let folder_path = self.lookup_folder_path(row.2.as_deref()).await?;
        Self::row_to_file(
            row.0,
            row.1,
            row.2,
            folder_path,
            row.3,
            row.4,
            row.5,
            row.6,
            None,
            String::new(),
        )
    }

    async fn copy_file(
        &self,
        file_id: &str,
        target_folder_id: Option<String>,
    ) -> Result<File, DomainError> {
        // Une transaction : ligne source verrouillée en partage (un échange
        // de blob concurrent attend), nouvelle ligne insérée, référence du
        // blob incrémentée. Si le blob a été libéré entre temps, la copie
        // échoue et rien n'est écrit (jamais de ligne vers un blob supprimé).
        // Zero-copy : le contenu n'est pas dupliqué (dédup).
        let target_fid = target_folder_id.clone();
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("begin: {e}")))?;

        let row = sqlx::query_as::<
            _,
            (
                String,
                String,
                Option<String>,
                i64,
                String,
                i64,
                i64,
                String,
            ),
        >(
            r#"
            WITH src AS (
                SELECT name, folder_id, user_id, blob_hash, size, mime_type,
                       etat_cdr, cdr_rapport
                  FROM storage.files
                 WHERE id = $1::uuid AND NOT is_trashed
                   FOR SHARE
            ),
            new_file AS (
                -- EX-45 : la copie (même blob) conserve l'état CDR de la source.
                INSERT INTO storage.files (name, folder_id, user_id, blob_hash, size, mime_type,
                                           etat_cdr, cdr_rapport)
                SELECT name,
                       COALESCE($2::uuid, folder_id),
                       user_id,
                       blob_hash,
                       size,
                       mime_type,
                       etat_cdr,
                       cdr_rapport
                  FROM src
                RETURNING id::text, name, folder_id::text, size, mime_type,
                          EXTRACT(EPOCH FROM created_at)::bigint,
                          EXTRACT(EPOCH FROM updated_at)::bigint,
                          blob_hash
            )
            SELECT * FROM new_file
            "#,
        )
        .bind(file_id)
        .bind(&target_fid)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e
                && db_err.code().as_deref() == Some("23505")
            {
                return DomainError::already_exists(
                    "File",
                    "a file with this name already exists in the target folder",
                );
            }
            DomainError::internal_error("FileBlobWrite", format!("copy: {e}"))
        })?
        .ok_or_else(|| DomainError::not_found("File", file_id))?;

        let blob_hash = &row.7;

        // Échec fatal (annulation de la transaction, donc de la ligne).
        DedupService::incrementer_reference(&mut tx, blob_hash).await?;
        tx.commit()
            .await
            .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("commit: {e}")))?;

        tracing::info!(
            "📋 BLOB COPY: {} (hash: {}, zero-copy via dedup)",
            row.1,
            blob_hash.get(..12).unwrap_or(blob_hash)
        );

        let folder_path = self.lookup_folder_path(row.2.as_deref()).await?;
        Self::row_to_file(
            row.0,
            row.1,
            row.2,
            folder_path,
            row.3,
            row.4,
            row.5,
            row.6,
            None,
            row.7,
        )
    }

    async fn rename_file(&self, file_id: &str, new_name: &str) -> Result<File, DomainError> {
        let row = sqlx::query_as::<_, (String, String, Option<String>, i64, String, i64, i64)>(
            r#"
            UPDATE storage.files
               SET name = $1, updated_at = NOW()
             WHERE id = $2::uuid AND NOT is_trashed
            RETURNING id::text, name, folder_id::text, size, mime_type,
                      EXTRACT(EPOCH FROM created_at)::bigint,
                      EXTRACT(EPOCH FROM updated_at)::bigint
            "#,
        )
        .bind(new_name)
        .bind(file_id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e
                && db_err.code().as_deref() == Some("23505")
            {
                return DomainError::already_exists("File", format!("'{new_name}' already exists"));
            }
            DomainError::internal_error("FileBlobWrite", format!("rename: {e}"))
        })?
        .ok_or_else(|| DomainError::not_found("File", file_id))?;

        let folder_path = self.lookup_folder_path(row.2.as_deref()).await?;
        Self::row_to_file(
            row.0,
            row.1,
            row.2,
            folder_path,
            row.3,
            row.4,
            row.5,
            row.6,
            None,
            String::new(),
        )
    }

    async fn delete_file(&self, id: &str) -> Result<(), DomainError> {
        // The PG trigger `trg_files_decrement_blob_ref` automatically
        // decrements storage.blobs.ref_count for the deleted row's blob_hash.
        // Disk cleanup of orphaned blobs (ref_count = 0) is handled by
        // garbage_collect().
        let result = sqlx::query("DELETE FROM storage.files WHERE id = $1::uuid")
            .bind(id)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("delete: {e}")))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::not_found("File", id));
        }

        Ok(())
    }

    async fn update_file_content_from_temp(
        &self,
        file_id: &str,
        temp_path: &std::path::Path,
        size: u64,
        content_type: Option<String>,
        pre_computed_hash: Option<String>,
        modified_at: Option<i64>,
    ) -> Result<String, DomainError> {
        // Hors entonnoir CDR : état `inconnu` (traité comme non désarmé).
        self.update_file_content_from_temp_cdr(
            file_id,
            temp_path,
            size,
            content_type,
            pre_computed_hash,
            modified_at,
            &MarqueCdr::inconnue(),
        )
        .await
    }

    async fn register_file_deferred(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        size: u64,
    ) -> Result<(File, PathBuf), DomainError> {
        let user_id = self.resolve_user_id(folder_id.as_deref()).await?;

        // For deferred registration we use a placeholder hash.
        // The write-behind cache will call update_file_content later.
        let placeholder_hash = "0000000000000000000000000000000000000000000000000000000000000000";

        let row = sqlx::query_as::<_, (String, i64, i64)>(
            r#"
            INSERT INTO storage.files (name, folder_id, user_id, blob_hash, size, mime_type)
            VALUES ($1, $2::uuid, $3, $4, $5, $6)
            RETURNING id::text,
                      EXTRACT(EPOCH FROM created_at)::bigint,
                      EXTRACT(EPOCH FROM updated_at)::bigint
            "#,
        )
        .bind(&name)
        .bind(&folder_id)
        .bind(user_id)
        .bind(placeholder_hash)
        .bind(size as i64)
        .bind(&content_type)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("deferred: {e}")))?;

        let folder_path = self.lookup_folder_path(folder_id.as_deref()).await?;
        let file = Self::row_to_file(
            row.0.clone(),
            name,
            folder_id,
            folder_path,
            size as i64,
            content_type,
            row.1,
            row.2,
            Some(user_id),
            String::new(),
        )?;

        // The target_path is not meaningful for blob storage (content goes to .blobs/)
        // but the WriteBehindCache API requires it. We return a synthetic path.
        let target_path = PathBuf::from(format!(".pending/{}", row.0));

        Ok((file, target_path))
    }

    // ── Trash operations ──

    async fn move_to_trash(&self, file_id: &str) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            UPDATE storage.files
               SET is_trashed = TRUE,
                   trashed_at = NOW(),
                   original_folder_id = folder_id,
                   updated_at = NOW()
             WHERE id = $1::uuid AND NOT is_trashed
            "#,
        )
        .bind(file_id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("trash: {e}")))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::not_found("File", file_id));
        }
        Ok(())
    }

    async fn restore_from_trash(
        &self,
        file_id: &str,
        _original_path: &str,
    ) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            UPDATE storage.files
               SET is_trashed = FALSE,
                   trashed_at = NULL,
                   folder_id = COALESCE(original_folder_id, folder_id),
                   original_folder_id = NULL,
                   updated_at = NOW()
             WHERE id = $1::uuid AND is_trashed
            "#,
        )
        .bind(file_id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("restore: {e}")))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::not_found("File", file_id));
        }
        Ok(())
    }

    async fn delete_file_permanently(&self, file_id: &str) -> Result<(), DomainError> {
        // Same as delete_file — removes from DB and decrements blob ref
        self.delete_file(file_id).await
    }

    async fn copy_folder_tree(
        &self,
        source_folder_id: &str,
        target_parent_id: Option<String>,
        dest_name: Option<String>,
    ) -> Result<CopyFolderTreeResult, DomainError> {
        let row = sqlx::query_as::<_, (String, i64, i64)>(
            "SELECT new_root_id, folders_copied, files_copied \
               FROM storage.copy_folder_tree($1::uuid, $2::uuid, $3)",
        )
        .bind(source_folder_id)
        .bind(&target_parent_id)
        .bind(&dest_name)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| {
            // Map PG P0002 (no_data_found) to NotFound
            if let sqlx::Error::Database(ref db_err) = e {
                if db_err.code().as_deref() == Some("P0002") {
                    return DomainError::not_found("Folder", source_folder_id);
                }
                if db_err.code().as_deref() == Some("23505") {
                    return DomainError::already_exists(
                        "Folder",
                        "a folder with this name already exists in the target location",
                    );
                }
            }
            DomainError::internal_error("FileBlobWrite", format!("copy_folder_tree: {e}"))
        })?;

        tracing::info!(
            "📂 TREE COPY: {} folders + {} files (root: {}, zero-copy via dedup)",
            row.1,
            row.2,
            row.0.get(..8).unwrap_or(row.0.as_str())
        );

        Ok(CopyFolderTreeResult {
            new_root_folder_id: row.0,
            folders_copied: row.1,
            files_copied: row.2,
        })
    }
}
