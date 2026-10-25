//! @id     miyucloud_infrastructure_services_dedup_service
//! @do     definir_stream_chunk_size
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « infrastructure services dedup service ».

//! Content-Addressable Storage with Deduplication (PostgreSQL-backed)
//!
//! Implements hash-based deduplication to eliminate redundant file storage.
//! Files are stored by their BLAKE3 hash, and multiple references can point
//! to the same physical blob.
//!
//! Architecture:
//! ```text
//! ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
//! │ storage.files   │────▶│ storage.blobs   │────▶│ Blob Store      │
//! │ (references)    │     │ (PG dedup index)│     │ (.blobs/ on FS) │
//! └─────────────────┘     └─────────────────┘     └─────────────────┘
//! ```
//!
//! The dedup index lives in PostgreSQL (`storage.blobs`) — no in-memory
//! HashMap, no JSON file, no WAL.
//!
//! **Write-first strategy** (store_bytes / store_from_file):
//!   1. Write/move (or encrypt) the blob file to disk *before* touching
//!      PostgreSQL (`preparer_blob`) — no PG connection held during I/O.
//!   2. `INSERT … ON CONFLICT … RETURNING ref_count` upsert in the
//!      caller's transaction (`referencer_blob`), **the same one** that
//!      writes the `storage.files` row, then a presence re-check of the blob
//!      under row lock.
//!
//! **Reference counting** — single rule: every change of
//! `storage.files.blob_hash` adjusts `storage.blobs.ref_count` in the same
//! transaction (`referencer_blob`, `incrementer_reference`,
//! `decrementer_reference`, the delete trigger, `copy_folder_tree`).
//! Unreferenced blobs are removed only by `garbage_collect` (row-locked);
//! `reconcilier_lot` repairs counts left wrong by a crash.
//!
//! **Encryption at rest** (`MIYUCLOUD_ENCRYPTION_ENABLED`): new blobs are
//! written in the `MIYUCENC` v1 format (see `blob_cipher`), legacy
//! plaintext blobs stay readable. The BLAKE3 hash is always the plaintext's.
//!
//! Benefits:
//! - ACID durability — crash-safe, zero orphaned index entries
//! - PG connections never blocked by disk I/O (write-first)
//! - 30-50% storage reduction typical
//! - Faster uploads for existing content (instant dedup)

use bytes::Bytes;
use futures::stream::{self, StreamExt};
use futures::{Stream, TryStreamExt};

use sqlx::PgPool;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use tokio::fs;

use crate::application::ports::dedup_ports::{
    BlobMetadataDto, DedupPort, DedupResultDto, DedupStatsDto,
};
use crate::application::services::reconciliation_references_service::{
    DepotReferences, LotReconcilie,
};
use crate::domain::errors::{DomainError, ErrorKind};
use crate::infrastructure::services::blob_cipher::{self, BlobCipher};

/// Chunk size for streaming file reads (256 KB)
const STREAM_CHUNK_SIZE: usize = 256 * 1024;

/// Content-Addressable Storage Service (PostgreSQL-backed)
pub struct DedupService {
    /// Root directory for blob storage on the filesystem
    blob_root: PathBuf,
    /// Root directory for temporary files during upload
    temp_root: PathBuf,
    /// PostgreSQL connection pool (dedup index in `storage.blobs`) — primary,
    /// used by request-path operations (store_bytes, store_from_file, etc.).
    pool: Arc<PgPool>,
    /// Isolated maintenance pool for long-running operations
    /// (verify_integrity, garbage_collect) that must never starve the primary.
    maintenance_pool: Arc<PgPool>,
    /// Chiffrement au repos des blobs (`None` : désactivé, blobs écrits en
    /// clair ; les blobs chiffrés existants sont alors refusés à la lecture).
    cipher: Option<Arc<BlobCipher>>,
}

/// Compile-time lookup table for the 256 two-digit lowercase hex prefixes ("00"…"ff").
/// Avoids a `format!("{:02x}", i)` allocation on every iteration of `initialize()`.
static HEX_PREFIXES: [&str; 256] = [
    "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "0a", "0b", "0c", "0d", "0e", "0f",
    "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "1a", "1b", "1c", "1d", "1e", "1f",
    "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "2a", "2b", "2c", "2d", "2e", "2f",
    "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "3a", "3b", "3c", "3d", "3e", "3f",
    "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "4a", "4b", "4c", "4d", "4e", "4f",
    "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "5a", "5b", "5c", "5d", "5e", "5f",
    "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "6a", "6b", "6c", "6d", "6e", "6f",
    "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "7a", "7b", "7c", "7d", "7e", "7f",
    "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "8a", "8b", "8c", "8d", "8e", "8f",
    "90", "91", "92", "93", "94", "95", "96", "97", "98", "99", "9a", "9b", "9c", "9d", "9e", "9f",
    "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "aa", "ab", "ac", "ad", "ae", "af",
    "b0", "b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8", "b9", "ba", "bb", "bc", "bd", "be", "bf",
    "c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9", "ca", "cb", "cc", "cd", "ce", "cf",
    "d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "da", "db", "dc", "dd", "de", "df",
    "e0", "e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8", "e9", "ea", "eb", "ec", "ed", "ee", "ef",
    "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "fa", "fb", "fc", "fd", "fe", "ff",
];

impl DedupService {
    /// Create a new dedup service backed by PostgreSQL.
    ///
    /// * `pool` — primary pool for request-path operations.
    /// * `maintenance_pool` — isolated pool for verify_integrity / garbage_collect.
    pub fn new(storage_root: &Path, pool: Arc<PgPool>, maintenance_pool: Arc<PgPool>) -> Self {
        let blob_root = storage_root.join(".blobs");
        let temp_root = storage_root.join(".dedup_temp");

        Self {
            blob_root,
            temp_root,
            pool,
            maintenance_pool,
            cipher: None,
        }
    }

    /// Active le chiffrement au repos des nouveaux blobs (format
    /// [`blob_cipher`] v1) et la lecture des blobs chiffrés.
    pub fn with_cipher(mut self, cipher: Option<Arc<BlobCipher>>) -> Self {
        self.cipher = cipher;
        self
    }

    /// Creates a stub instance for testing — never hits PG or the filesystem.
    #[cfg(any(test, feature = "integration_tests"))]
    pub fn new_stub() -> Self {
        let stub_pool = Arc::new(
            sqlx::pool::PoolOptions::<sqlx::Postgres>::new()
                .max_connections(1)
                .connect_lazy("postgres://invalid:5432/none")
                .unwrap(),
        );
        Self {
            blob_root: std::path::PathBuf::from("/tmp/miyucloud_stub_blobs"),
            temp_root: std::path::PathBuf::from("/tmp/miyucloud_stub_temp"),
            pool: stub_pool.clone(),
            maintenance_pool: stub_pool,
            cipher: None,
        }
    }

    /// Initialize the service (create blob directories on the filesystem).
    pub async fn initialize(&self) -> Result<(), DomainError> {
        // Create directories
        fs::create_dir_all(&self.blob_root)
            .await
            .map_err(DomainError::from)?;
        fs::create_dir_all(&self.temp_root)
            .await
            .map_err(DomainError::from)?;

        // Create hash prefix directories (00-ff)
        for prefix in &HEX_PREFIXES {
            fs::create_dir_all(self.blob_root.join(prefix))
                .await
                .map_err(DomainError::from)?;
        }

        // Log existing blob stats from PG
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM storage.blobs")
            .fetch_one(self.pool.as_ref())
            .await
            .unwrap_or(0);

        let total_bytes: i64 =
            sqlx::query_scalar("SELECT COALESCE(SUM(size), 0) FROM storage.blobs")
                .fetch_one(self.pool.as_ref())
                .await
                .unwrap_or(0);

        tracing::info!(
            "Dedup service initialized (PostgreSQL-backed): {} blobs, {} bytes stored",
            count,
            total_bytes
        );

        Ok(())
    }

    // ── Path helpers ─────────────────────────────────────────────

    /// Get the blob path for a given hash.
    pub fn blob_path(&self, hash: &str) -> PathBuf {
        let prefix = hash.get(0..2).unwrap_or("00");
        self.blob_root.join(prefix).join(format!("{}.blob", hash))
    }

    // ── Hash helpers ─────────────────────────────────────────────

    /// Calculate BLAKE3 hash of in-memory content (~5× faster than SHA-256).
    ///
    /// For buffers larger than 128 KB the computation is parallelised across
    /// all available cores via `update_rayon()`.
    pub fn hash_bytes(content: &[u8]) -> String {
        if content.len() > 128 * 1024 {
            let mut hasher = blake3::Hasher::new();
            hasher.update_rayon(content);
            hasher.finalize().to_hex().to_string()
        } else {
            blake3::hash(content).to_hex().to_string()
        }
    }

    /// Calculate BLAKE3 hash of a file (~5× faster than SHA-256).
    ///
    /// Runs entirely on `spawn_blocking` with synchronous I/O so the Tokio
    /// worker threads are never blocked by CPU-bound hashing.
    ///
    /// Uses memory-mapped I/O (`update_mmap_rayon`) which avoids loading the
    /// entire file into the heap.  The OS pages in data on demand and BLAKE3
    /// parallelises the computation across all available cores via rayon.
    /// Peak RAM for a 500 MB file is only a few MB of active pages instead
    /// of the full 500 MB.
    pub async fn hash_file(path: &Path) -> std::io::Result<String> {
        let path = path.to_path_buf();
        tokio::task::spawn_blocking(move || {
            let mut hasher = blake3::Hasher::new();
            hasher.update_mmap_rayon(&path)?;
            Ok(hasher.finalize().to_hex().to_string())
        })
        .await
        .map_err(|e| std::io::Error::other(format!("hash_file: blocking task failed: {e}")))?
    }

    // ── Core store operations ────────────────────────────────────

    /// Maximum payload accepted by `store_bytes`.  Anything larger
    /// should use `store_from_file` (streaming — constant RAM).
    const MAX_STORE_BYTES: usize = 10 * 1024 * 1024; // 10 MB

    /// Store content with deduplication (from bytes).
    ///
    /// The content is spooled to a temporary file under the blob store, then
    /// stored through [`Self::store_from_file`] (same placement, encryption
    /// and reference semantics).
    ///
    /// **Guard**: rejects payloads >10 MB.  Large content must go through
    /// `store_from_file` which streams from disk with constant RAM.
    pub async fn store_bytes(
        &self,
        content: &[u8],
        content_type: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        if content.len() > Self::MAX_STORE_BYTES {
            return Err(DomainError::internal_error(
                "Dedup",
                format!(
                    "store_bytes called with {} bytes (max {}). Use store_from_file for large content.",
                    content.len(),
                    Self::MAX_STORE_BYTES
                ),
            ));
        }
        let hash = Self::hash_bytes(content);
        let temp_path = self.temp_root.join(format!("{}.tmp", uuid::Uuid::new_v4()));
        fs::write(&temp_path, content).await.map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to write temp blob: {}", e))
        })?;
        let result = self
            .store_from_file(&temp_path, content_type, Some(hash))
            .await;
        let _ = fs::remove_file(&temp_path).await;
        result
    }

    /// Store content with deduplication (streaming from file).
    ///
    /// Two phases: the blob is placed on disk first ([`Self::preparer_blob`],
    /// no PG connection held), then the reference is taken in a short
    /// transaction ([`Self::referencer_blob`]) that also re-checks the blob
    /// is still on disk. The source file is consumed.
    ///
    /// If `pre_computed_hash` is `Some`, the file will NOT be re-read for
    /// BLAKE3 — saving one full sequential read (the biggest I/O win).
    pub async fn store_from_file(
        &self,
        source_path: &Path,
        content_type: Option<String>,
        pre_computed_hash: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        let prepare = self.preparer_blob(source_path, pre_computed_hash).await?;
        let result = self.referencer_seul(&prepare, content_type).await;
        // Source consommée quelle que soit l'issue.
        self.liberer_source(&prepare).await;
        result
    }

    /// Référence `prepare` dans sa propre transaction (sans ligne de fichier).
    async fn referencer_seul(
        &self,
        prepare: &BlobPrepare,
        content_type: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to begin transaction: {}", e))
        })?;
        let result = self.referencer_blob(&mut tx, prepare, content_type).await?;
        tx.commit().await.map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to commit blob reference: {}", e))
        })?;
        Ok(result)
    }

    /// Phase 1 (hors PostgreSQL) : hache `source` si besoin et place le blob
    /// sur disque s'il n'y est pas encore — chiffré (format
    /// [`blob_cipher`]) si le chiffrement au repos est actif.
    ///
    /// En clair, la source peut être déplacée (renommage, zéro copie) ; en
    /// chiffré elle est conservée. Dans les deux cas, appeler
    /// [`Self::liberer_source`] après validation de la transaction.
    pub async fn preparer_blob(
        &self,
        source_path: &Path,
        pre_computed_hash: Option<String>,
    ) -> Result<BlobPrepare, DomainError> {
        let size = fs::metadata(source_path)
            .await
            .map_err(|e| {
                DomainError::internal_error("Dedup", format!("Failed to get file metadata: {}", e))
            })?
            .len();
        let hash = match pre_computed_hash {
            Some(h) => h,
            None => Self::hash_file(source_path)
                .await
                .map_err(DomainError::from)?,
        };
        if !est_hachage_valide(&hash) {
            return Err(DomainError::internal_error(
                "Dedup",
                "invalid blob hash (64 hex chars expected)",
            ));
        }
        let prepare = BlobPrepare {
            blob_path: self.blob_path(&hash),
            hash,
            size,
            source: source_path.to_path_buf(),
        };
        if !fs::try_exists(&prepare.blob_path).await.unwrap_or(false) {
            self.placer(&prepare).await?;
        }
        Ok(prepare)
    }

    /// Écrit le blob de `prepare` à sa place (temporaire puis renommage
    /// atomique ; contenu identique par construction en cas de course).
    async fn placer(&self, prepare: &BlobPrepare) -> Result<(), DomainError> {
        if let Some(chiffreur) = self.cipher.clone() {
            let temp_path = self.temp_root.join(format!("{}.enc", uuid::Uuid::new_v4()));
            let (source, temp) = (prepare.source.clone(), temp_path.clone());
            let chiffre =
                tokio::task::spawn_blocking(move || chiffreur.chiffrer_fichier(&source, &temp))
                    .await
                    .map_err(|e| {
                        DomainError::internal_error("Dedup", format!("encryption task: {e}"))
                    })?;
            if let Err(e) = chiffre {
                let _ = fs::remove_file(&temp_path).await;
                return Err(DomainError::internal_error(
                    "Dedup",
                    format!("Failed to encrypt blob: {}", e),
                ));
            }
            if let Err(e) = fs::rename(&temp_path, &prepare.blob_path).await {
                let _ = fs::remove_file(&temp_path).await;
                return Err(DomainError::internal_error(
                    "Dedup",
                    format!("Failed to place encrypted blob: {}", e),
                ));
            }
            return Ok(());
        }

        // Clair : renommage (atomique sur le même système de fichiers).
        if let Err(e) = fs::rename(&prepare.source, &prepare.blob_path).await {
            if e.raw_os_error() == Some(18) {
                // EXDEV : copie vers un temporaire du magasin, puis renommage.
                let temp_path = self.temp_root.join(format!("{}.tmp", uuid::Uuid::new_v4()));
                fs::copy(&prepare.source, &temp_path).await.map_err(|ce| {
                    DomainError::internal_error(
                        "Dedup",
                        format!("Failed to copy file to blob store: {}", ce),
                    )
                })?;
                if let Err(re) = fs::rename(&temp_path, &prepare.blob_path).await {
                    let _ = fs::remove_file(&temp_path).await;
                    return Err(DomainError::internal_error(
                        "Dedup",
                        format!("Failed to place blob: {}", re),
                    ));
                }
            } else if !fs::try_exists(&prepare.blob_path).await.unwrap_or(false) {
                return Err(DomainError::internal_error(
                    "Dedup",
                    format!("Failed to move file to blob store: {}", e),
                ));
            }
        }
        Ok(())
    }

    /// Phase 2 (dans la transaction `conn` de l'appelant) : prend une
    /// référence sur le blob préparé (`INSERT … ON CONFLICT` : +1), puis
    /// vérifie, ligne verrouillée, que le blob est bien sur disque — un
    /// ramasse-miettes concurrent peut l'avoir supprimé entre la préparation
    /// et la référence ; il est alors replacé depuis la source, ou l'écriture
    /// échoue (jamais de référence vers un blob absent).
    ///
    /// La référence et la ligne `storage.files` qui la porte doivent être
    /// écrites dans la **même** transaction (réconciliation exacte, voir
    /// [`Self::reconcilier_lot`]).
    pub async fn referencer_blob(
        &self,
        conn: &mut sqlx::PgConnection,
        prepare: &BlobPrepare,
        content_type: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        let ref_count: i32 = sqlx::query_scalar(
            "INSERT INTO storage.blobs (hash, size, ref_count, content_type)
             VALUES ($1, $2, 1, $3)
             ON CONFLICT (hash) DO UPDATE SET ref_count = storage.blobs.ref_count + 1
             RETURNING ref_count",
        )
        .bind(&prepare.hash)
        .bind(i64::try_from(prepare.size).unwrap_or(i64::MAX))
        .bind(&content_type)
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to upsert blob: {}", e))
        })?;

        if !fs::try_exists(&prepare.blob_path).await.unwrap_or(false) {
            if fs::try_exists(&prepare.source).await.unwrap_or(false) {
                tracing::warn!(
                    "Blob {} absent après référence (ramasse-miettes concurrent) : replacé",
                    court(&prepare.hash)
                );
                self.placer(prepare).await?;
            } else {
                return Err(DomainError::internal_error(
                    "Dedup",
                    format!(
                        "blob {} vanished before its reference was committed",
                        court(&prepare.hash)
                    ),
                ));
            }
        }

        if ref_count > 1 {
            tracing::info!(
                "DEDUP HIT: {} ({} bytes saved)",
                court(&prepare.hash),
                prepare.size
            );
            Ok(DedupResultDto::ExistingBlob {
                hash: prepare.hash.clone(),
                size: prepare.size,
                blob_path: prepare.blob_path.clone(),
                saved_bytes: prepare.size,
            })
        } else {
            tracing::info!(
                "NEW BLOB: {} ({} bytes)",
                court(&prepare.hash),
                prepare.size
            );
            Ok(DedupResultDto::NewBlob {
                hash: prepare.hash.clone(),
                size: prepare.size,
                blob_path: prepare.blob_path.clone(),
            })
        }
    }

    /// Supprime la source d'un blob préparé (après validation de la
    /// transaction ; sans effet si elle a été déplacée).
    pub async fn liberer_source(&self, prepare: &BlobPrepare) {
        if prepare.source != prepare.blob_path {
            let _ = fs::remove_file(&prepare.source).await;
        }
    }

    // ── Reference counting ───────────────────────────────────────

    /// Check if a blob with the given hash exists in the PG index.
    pub async fn blob_exists(&self, hash: &str) -> bool {
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM storage.blobs WHERE hash = $1)")
            .bind(hash)
            .fetch_one(self.pool.as_ref())
            .await
            .unwrap_or(false)
    }

    /// Returns `true` if `user_id` owns at least one (non-trashed) file that
    /// references the blob identified by `hash`.
    ///
    /// Used by the dedup API handlers to enforce per-user access control on
    /// the content-addressed blob store.
    pub async fn user_owns_blob_reference(&self, hash: &str, user_id: &str) -> bool {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM storage.files WHERE blob_hash = $1 AND user_id = $2 AND NOT is_trashed)",
        )
        .bind(hash)
        .bind(user_id)
        .fetch_one(self.pool.as_ref())
        .await
        .unwrap_or(false)
    }

    /// Returns `true` if `user_id` owns at least one (non-trashed) file that
    /// references `hash` **and is disarmed** (EX-45). Used before serving blob
    /// bytes: a blob reachable only through quarantined or `inconnu` files
    /// (pre-CDR, not yet reclassified) is never served by hash.
    pub async fn user_owns_desarmed_blob_reference(&self, hash: &str, user_id: &str) -> bool {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM storage.files WHERE blob_hash = $1 AND user_id = $2 \
               AND NOT is_trashed AND etat_cdr = 'desarme')",
        )
        .bind(hash)
        .bind(user_id)
        .fetch_one(self.pool.as_ref())
        .await
        .unwrap_or(false)
    }

    /// Get metadata for a blob from PostgreSQL.
    pub async fn get_blob_metadata(&self, hash: &str) -> Option<BlobMetadataDto> {
        let row = sqlx::query_as::<_, (String, i64, i32, Option<String>)>(
            "SELECT hash, size, ref_count, content_type FROM storage.blobs WHERE hash = $1",
        )
        .bind(hash)
        .fetch_optional(self.pool.as_ref())
        .await
        .ok()
        .flatten()?;

        Some(BlobMetadataDto {
            hash: row.0,
            size: row.1 as u64,
            ref_count: row.2 as u32,
            content_type: row.3,
        })
    }

    /// Add a reference to a blob (increment ref_count).
    pub async fn add_reference(&self, hash: &str) -> Result<(), DomainError> {
        let mut conn = self.pool.acquire().await.map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to acquire connection: {}", e))
        })?;
        Self::incrementer_reference(&mut conn, hash).await
    }

    /// Ajoute une référence au blob **existant** `hash`, dans la transaction
    /// de l'appelant (celle qui écrit la ligne `storage.files`). Erreur
    /// `NotFound` si le blob n'est plus indexé (libéré entre temps) : la
    /// transaction doit alors être annulée, jamais validée sans référence.
    pub async fn incrementer_reference(
        conn: &mut sqlx::PgConnection,
        hash: &str,
    ) -> Result<(), DomainError> {
        let rows_affected =
            sqlx::query("UPDATE storage.blobs SET ref_count = ref_count + 1 WHERE hash = $1")
                .bind(hash)
                .execute(&mut *conn)
                .await
                .map_err(|e| {
                    DomainError::internal_error(
                        "Dedup",
                        format!("Failed to increment ref_count: {}", e),
                    )
                })?
                .rows_affected();

        if rows_affected == 0 {
            return Err(DomainError::new(
                ErrorKind::NotFound,
                "Blob",
                format!("Blob not found: {}", hash),
            ));
        }
        Ok(())
    }

    /// Retire une référence au blob `hash` dans la transaction de
    /// l'appelant — même sémantique que le déclencheur
    /// `trg_files_decrement_blob_ref` (plancher à 0). Le blob devenu sans
    /// référence est supprimé par [`Self::garbage_collect`], jamais ici.
    /// Rend le nouveau compte (`None` si le blob n'est pas indexé).
    pub async fn decrementer_reference(
        conn: &mut sqlx::PgConnection,
        hash: &str,
    ) -> Result<Option<i32>, DomainError> {
        sqlx::query_scalar(
            "UPDATE storage.blobs SET ref_count = GREATEST(ref_count - 1, 0) \
              WHERE hash = $1 RETURNING ref_count",
        )
        .bind(hash)
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to decrement ref_count: {}", e))
        })
    }

    /// Remove a reference from a blob (outside any file-row transaction).
    ///
    /// Returns `true` if the blob has no reference left. The blob file is
    /// **not** deleted here: [`Self::garbage_collect`] removes unreferenced
    /// blobs under row lock, so a concurrent store of the same content can
    /// never lose its blob.
    pub async fn remove_reference(&self, hash: &str) -> Result<bool, DomainError> {
        let mut conn = self.pool.acquire().await.map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to acquire connection: {}", e))
        })?;
        Ok(Self::decrementer_reference(&mut conn, hash).await? == Some(0))
    }

    /// Réconcilie `ref_count` avec le nombre réel de lignes `storage.files`
    /// pour au plus `limite` blobs de hachage > `apres` (ordre croissant).
    ///
    /// Exact : les lignes du lot sont verrouillées (`FOR UPDATE SKIP LOCKED`,
    /// les blobs en cours d'écriture sont repris au passage suivant) puis
    /// comptées dans une **nouvelle** instruction, qui voit toute écriture
    /// validée ; toute modification de `storage.files.blob_hash` prend sa
    /// référence dans la même transaction (déclencheur de suppression,
    /// [`Self::referencer_blob`], [`Self::incrementer_reference`],
    /// [`Self::decrementer_reference`], `storage.copy_folder_tree`).
    ///
    /// Les références sans ligne de fichier (dépôt direct `/api/dedup`) sont
    /// ainsi ramenées à 0 et le blob libéré par le ramasse-miettes.
    pub async fn reconcilier_lot(
        &self,
        apres: Option<&str>,
        limite: i64,
    ) -> Result<LotReconcilie, DomainError> {
        let erreur = |contexte: &str, e: sqlx::Error| {
            DomainError::internal_error("Dedup", format!("réconciliation ({contexte}) : {e}"))
        };
        let mut tx = self
            .maintenance_pool
            .begin()
            .await
            .map_err(|e| erreur("transaction", e))?;
        let lot: Vec<String> = sqlx::query_scalar(
            "SELECT hash FROM storage.blobs \
              WHERE ($1::text IS NULL OR hash > $1) \
              ORDER BY hash LIMIT $2 FOR UPDATE SKIP LOCKED",
        )
        .bind(apres)
        .bind(limite)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| erreur("verrouillage du lot", e))?;
        let Some(dernier) = lot.last().cloned() else {
            tx.commit().await.map_err(|e| erreur("validation", e))?;
            return Ok(LotReconcilie::default());
        };
        let corriges: Vec<(String, i32, i32)> = sqlx::query_as(
            "WITH reel AS ( \
                 SELECT b.hash, b.ref_count AS ancien, \
                        (SELECT COUNT(*) FROM storage.files f WHERE f.blob_hash = b.hash)::int AS n \
                   FROM storage.blobs b WHERE b.hash = ANY($1) \
             ) \
             UPDATE storage.blobs b SET ref_count = reel.n \
               FROM reel \
              WHERE b.hash = reel.hash AND b.ref_count <> reel.n \
             RETURNING b.hash, reel.ancien, reel.n",
        )
        .bind(&lot)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| erreur("comptage", e))?;
        tx.commit().await.map_err(|e| erreur("validation", e))?;
        for (hash, ancien, n) in &corriges {
            tracing::warn!(
                "Réconciliation des références : blob {} ref_count {} → {}",
                court(hash),
                ancien,
                n
            );
        }
        Ok(LotReconcilie {
            examines: lot.len() as u64,
            corriges: corriges.len() as u64,
            dernier: Some(dernier),
        })
    }

    // ── Read operations ──────────────────────────────────────────

    /// Erreur de lecture d'un blob : absent ⇒ `NotFound` ; blob chiffré
    /// illisible (sans clé, autre clé, altéré) ⇒ erreur interne, jamais
    /// d'octets bruts servis.
    fn erreur_lecture(hash: &str, e: std::io::Error) -> DomainError {
        if e.kind() == std::io::ErrorKind::NotFound {
            DomainError::new(
                ErrorKind::NotFound,
                "Blob",
                format!("Failed to open blob {}: {}", hash, e),
            )
        } else {
            DomainError::internal_error("Blob", format!("Failed to read blob {}: {}", hash, e))
        }
    }

    /// Stream blob content in chunks — constant memory (one chunk / one
    /// 64 KiB encrypted segment per stream). Encrypted blobs are decrypted
    /// on the fly; legacy plaintext blobs are read as-is.
    pub async fn read_blob_stream(
        &self,
        hash: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>, DomainError>
    {
        self.read_blob_range_stream(hash, 0, None).await
    }

    /// Stream a byte range `[start, end)` of a blob's plaintext — only the
    /// requested portion (and, when encrypted, the segments covering it) is
    /// read.
    pub async fn read_blob_range_stream(
        &self,
        hash: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>, DomainError>
    {
        blob_cipher::ouvrir_flux_clair(
            &self.blob_path(hash),
            self.cipher.clone(),
            start,
            end,
            STREAM_CHUNK_SIZE,
        )
        .await
        .map_err(|e| Self::erreur_lecture(hash, e))
    }

    /// Get the plaintext size of a blob without reading its content.
    pub async fn blob_size(&self, hash: &str) -> Result<u64, DomainError> {
        blob_cipher::taille_claire_fichier(&self.blob_path(hash))
            .await
            .map_err(|e| Self::erreur_lecture(hash, e))
    }

    /// Copie le contenu **clair** du blob `hash` dans `dest` (déchiffré si
    /// besoin, mémoire bornée) ; rend la taille copiée.
    pub async fn copier_blob_clair(&self, hash: &str, dest: &Path) -> Result<u64, DomainError> {
        if !est_hachage_valide(hash) {
            return Err(DomainError::internal_error("Blob", "invalid blob hash"));
        }
        let (source, dest, chiffreur) = (
            self.blob_path(hash),
            dest.to_path_buf(),
            self.cipher.clone(),
        );
        tokio::task::spawn_blocking(move || {
            blob_cipher::copier_clair_sync(&source, chiffreur.as_deref(), &dest)
        })
        .await
        .map_err(|e| DomainError::internal_error("Blob", format!("copy task: {e}")))?
        .map_err(|e| Self::erreur_lecture(hash, e))
    }

    /// Chiffreur des blobs (`None` : chiffrement au repos désactivé).
    pub fn chiffreur(&self) -> Option<Arc<BlobCipher>> {
        self.cipher.clone()
    }

    // ── Statistics (computed from PG) ────────────────────────────

    /// Get deduplication statistics by querying PostgreSQL.
    pub async fn get_stats(&self) -> DedupStatsDto {
        let row = sqlx::query_as::<_, (i64, i64, i64)>(
            "SELECT
                 COUNT(*)                                    AS total_blobs,
                 COALESCE(SUM(size), 0)                     AS total_bytes_stored,
                 COALESCE(SUM(size::BIGINT * ref_count), 0) AS total_bytes_referenced
             FROM storage.blobs",
        )
        .fetch_one(self.pool.as_ref())
        .await
        .unwrap_or((0, 0, 0));

        let total_blobs = row.0 as u64;
        let total_bytes_stored = row.1 as u64;
        let total_bytes_referenced = row.2 as u64;
        let bytes_saved = total_bytes_referenced.saturating_sub(total_bytes_stored);
        let dedup_ratio = if total_bytes_stored > 0 {
            total_bytes_referenced as f64 / total_bytes_stored as f64
        } else {
            1.0
        };

        DedupStatsDto {
            total_blobs,
            total_bytes_stored,
            total_bytes_referenced,
            bytes_saved,
            dedup_hits: 0, // Not tracked per-session — derive from SUM(ref_count - 1)
            dedup_ratio,
        }
    }

    // ── Maintenance ──────────────────────────────────────────────

    /// Verify integrity of all blobs (PG index vs filesystem).
    ///
    /// Uses a **streaming cursor** (`fetch()`) so memory stays O(batch)
    /// instead of O(total_blobs).  Blobs are verified in micro-batches
    /// of `VERIFY_CONCURRENCY` using `buffer_unordered`.
    pub async fn verify_integrity(&self) -> Result<Vec<String>, DomainError> {
        /// Max blobs verified concurrently.  Each spawns a blocking
        /// thread for BLAKE3 so this also caps blocking-pool pressure.
        const VERIFY_CONCURRENCY: usize = 16;

        let mut row_stream = sqlx::query_as::<_, (String, i64)>(
            "SELECT hash, size FROM storage.blobs ORDER BY hash",
        )
        .fetch(self.maintenance_pool.as_ref());

        let mut total = 0usize;
        let mut corrupted = Vec::<String>::new();
        let mut batch = Vec::with_capacity(VERIFY_CONCURRENCY);

        loop {
            let maybe_row = row_stream.try_next().await.map_err(|e| {
                DomainError::internal_error("Dedup", format!("Failed to list blobs: {}", e))
            })?;

            let is_done = maybe_row.is_none();

            if let Some(row) = maybe_row {
                total += 1;
                batch.push(row);
            }

            // Flush when batch is full or we've exhausted the cursor
            if batch.len() >= VERIFY_CONCURRENCY || (is_done && !batch.is_empty()) {
                let current_batch =
                    std::mem::replace(&mut batch, Vec::with_capacity(VERIFY_CONCURRENCY));

                let issues: Vec<String> = stream::iter(current_batch)
                    .map(|(hash, expected_size)| {
                        let blob_path = self.blob_path(&hash);
                        let chiffreur = self.cipher.clone();
                        async move {
                            let mut issues = Vec::new();
                            if !est_hachage_valide(&hash) {
                                issues.push(format!("{}: invalid hash", hash));
                                return issues;
                            }
                            // Hachage et taille du **clair** (blob chiffré
                            // déchiffré et authentifié, blob hérité lu tel quel).
                            let verification = tokio::task::spawn_blocking(move || {
                                blob_cipher::hacher_clair_sync(&blob_path, chiffreur.as_deref())
                            })
                            .await;
                            match verification {
                                Ok(Ok((actual_hash, actual_size))) => {
                                    if actual_size != expected_size as u64 {
                                        issues.push(format!(
                                            "{}: size mismatch (expected: {}, actual: {})",
                                            hash, expected_size, actual_size,
                                        ));
                                    }
                                    if actual_hash != hash {
                                        issues.push(format!(
                                            "{}: hash mismatch (actual: {})",
                                            hash, actual_hash,
                                        ));
                                    }
                                }
                                Ok(Err(e)) if e.kind() == std::io::ErrorKind::NotFound => {
                                    issues.push(format!("{}: file missing on disk", hash));
                                }
                                Ok(Err(e)) => {
                                    issues.push(format!("{}: read error ({})", hash, e));
                                }
                                Err(e) => {
                                    issues.push(format!(
                                        "{}: verification task failed ({})",
                                        hash, e
                                    ));
                                }
                            }
                            issues
                        }
                    })
                    .buffer_unordered(VERIFY_CONCURRENCY)
                    .flat_map(stream::iter)
                    .collect()
                    .await;

                corrupted.extend(issues);
            }

            if is_done {
                break;
            }
        }

        if corrupted.is_empty() {
            tracing::info!("Integrity check passed for {} blobs", total);
        } else {
            tracing::warn!("Integrity check found {} issues", corrupted.len());
        }

        Ok(corrupted)
    }

    /// Garbage collect orphaned blobs (ref_count = 0).
    ///
    /// Par lots courts (`BATCH_SIZE` lignes par transaction) : les lignes
    /// sans référence sont verrouillées (`FOR UPDATE SKIP LOCKED`), leurs
    /// fichiers supprimés **pendant** que le verrou est tenu, puis les lignes
    /// supprimées. Un rangement concurrent du même contenu attend donc la fin
    /// du lot, recrée la ligne et replace le blob (voir
    /// [`Self::referencer_blob`]) : aucune référence ne peut viser un blob
    /// supprimé. Un fichier non supprimable garde sa ligne (repris plus tard).
    ///
    /// Défense en profondeur : un blob encore visé par une ligne
    /// `storage.files` n'est jamais supprimé, même si son compteur est
    /// faussement à 0 (compteur hérité pas encore réconcilié).
    pub async fn garbage_collect(&self) -> Result<(u64, u64), DomainError> {
        /// Max rows deleted per mini-transaction.
        const BATCH_SIZE: i64 = 500;

        let erreur =
            |e: sqlx::Error| DomainError::internal_error("Dedup", format!("GC batch failed: {e}"));
        let mut total_deleted = 0u64;
        let mut total_bytes = 0u64;
        let mut apres: Option<String> = None;

        loop {
            let mut tx = self.maintenance_pool.begin().await.map_err(erreur)?;
            let batch: Vec<(String, i64)> = sqlx::query_as(
                "SELECT b.hash, b.size FROM storage.blobs b \
                  WHERE b.ref_count = 0 AND ($1::text IS NULL OR b.hash > $1) \
                    AND NOT EXISTS (SELECT 1 FROM storage.files f WHERE f.blob_hash = b.hash) \
                  ORDER BY b.hash LIMIT $2 FOR UPDATE OF b SKIP LOCKED",
            )
            .bind(apres.as_deref())
            .bind(BATCH_SIZE)
            .fetch_all(&mut *tx)
            .await
            .map_err(erreur)?;

            let Some((dernier, _)) = batch.last() else {
                tx.commit().await.map_err(erreur)?;
                break;
            };
            apres = Some(dernier.clone());

            let mut supprimes: Vec<String> = Vec::with_capacity(batch.len());
            for (hash, size) in &batch {
                match fs::remove_file(self.blob_path(hash)).await {
                    Ok(()) => {}
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                    Err(e) => {
                        tracing::warn!("Failed to delete orphan blob file {hash}: {e}");
                        continue;
                    }
                }
                total_bytes += u64::try_from(*size).unwrap_or(0);
                supprimes.push(hash.clone());
            }
            sqlx::query("DELETE FROM storage.blobs WHERE hash = ANY($1) AND ref_count = 0")
                .bind(&supprimes)
                .execute(&mut *tx)
                .await
                .map_err(erreur)?;
            tx.commit().await.map_err(erreur)?;
            total_deleted += supprimes.len() as u64;

            // Yield so uploads / other tasks are not starved.
            tokio::task::yield_now().await;
        }

        if total_deleted > 0 {
            tracing::info!("GC: removed {total_deleted} blobs ({total_bytes} bytes)");
        }

        Ok((total_deleted, total_bytes))
    }
}

/// Blob placé sur disque, en attente de sa référence (voir
/// [`DedupService::preparer_blob`]).
#[derive(Debug, Clone)]
pub struct BlobPrepare {
    /// Hachage BLAKE3 (hex) du contenu clair.
    pub hash: String,
    /// Taille claire.
    pub size: u64,
    /// Fichier source.
    pub source: PathBuf,
    /// Emplacement du blob.
    pub blob_path: PathBuf,
}

/// Hachage BLAKE3 hexadécimal de 64 caractères.
pub fn est_hachage_valide(hash: &str) -> bool {
    hash.len() == 64 && hash.bytes().all(|b| b.is_ascii_hexdigit())
}

/// 12 premiers caractères d'un hachage (journaux), sans risque de panique.
fn court(hash: &str) -> &str {
    hash.get(..12).unwrap_or(hash)
}

// ─── Réconciliation des références ───────────────────────────────────────────

impl DepotReferences for DedupService {
    async fn reconcilier_lot(
        &self,
        apres: Option<&str>,
        limite: i64,
    ) -> Result<LotReconcilie, DomainError> {
        DedupService::reconcilier_lot(self, apres, limite).await
    }

    async fn ramasser_orphelins(&self) -> Result<(u64, u64), DomainError> {
        self.garbage_collect().await
    }
}

// ─── Port implementation ─────────────────────────────────────────────────────

impl DedupPort for DedupService {
    async fn store_bytes(
        &self,
        content: &[u8],
        content_type: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        self.store_bytes(content, content_type).await
    }

    async fn store_from_file(
        &self,
        source_path: &Path,
        content_type: Option<String>,
        pre_computed_hash: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        self.store_from_file(source_path, content_type, pre_computed_hash)
            .await
    }

    async fn blob_exists(&self, hash: &str) -> bool {
        self.blob_exists(hash).await
    }

    async fn get_blob_metadata(&self, hash: &str) -> Option<BlobMetadataDto> {
        self.get_blob_metadata(hash).await
    }

    async fn read_blob_stream(
        &self,
        hash: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>, DomainError>
    {
        self.read_blob_stream(hash).await
    }

    async fn read_blob_range_stream(
        &self,
        hash: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>, DomainError>
    {
        self.read_blob_range_stream(hash, start, end).await
    }

    async fn blob_size(&self, hash: &str) -> Result<u64, DomainError> {
        self.blob_size(hash).await
    }

    async fn add_reference(&self, hash: &str) -> Result<(), DomainError> {
        self.add_reference(hash).await
    }

    async fn remove_reference(&self, hash: &str) -> Result<bool, DomainError> {
        self.remove_reference(hash).await
    }

    async fn hash_file(&self, path: &Path) -> Result<String, DomainError> {
        DedupService::hash_file(path)
            .await
            .map_err(DomainError::from)
    }

    fn blob_path(&self, hash: &str) -> PathBuf {
        self.blob_path(hash)
    }

    async fn get_stats(&self) -> DedupStatsDto {
        self.get_stats().await
    }

    async fn flush(&self) -> Result<(), DomainError> {
        // No-op: PostgreSQL handles persistence automatically via WAL/commit
        Ok(())
    }

    async fn verify_integrity(&self) -> Result<Vec<String>, DomainError> {
        self.verify_integrity().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::TryStreamExt;

    /// Service sur `racine` sans base (pools paresseux jamais connectés) :
    /// seules les opérations disque sont exercées.
    fn service(racine: &Path, cle: Option<u8>) -> DedupService {
        let pool = Arc::new(
            sqlx::pool::PoolOptions::<sqlx::Postgres>::new()
                .max_connections(1)
                .connect_lazy("postgres://invalid:5432/none")
                .expect("pool paresseux"),
        );
        let svc = DedupService::new(racine, pool.clone(), pool).with_cipher(
            cle.map(|c| Arc::new(BlobCipher::from_key(&[c; 32]).expect("clé de test"))),
        );
        std::fs::create_dir_all(&svc.temp_root).expect("temp");
        for p in HEX_PREFIXES {
            std::fs::create_dir_all(svc.blob_root.join(p)).expect("préfixe");
        }
        svc
    }

    async fn lire(
        svc: &DedupService,
        hash: &str,
        debut: u64,
        fin: Option<u64>,
    ) -> Result<Vec<u8>, String> {
        let flux = svc
            .read_blob_range_stream(hash, debut, fin)
            .await
            .map_err(|e| e.to_string())?;
        let morceaux: Vec<Bytes> = flux.try_collect().await.map_err(|e| e.to_string())?;
        Ok(morceaux.concat())
    }

    fn contenu() -> Vec<u8> {
        (0..200_000u32).map(|i| (i % 253) as u8).collect()
    }

    #[tokio::test]
    async fn ecriture_chiffree_relecture_et_cle_fausse_refusee() {
        let dir = tempfile::tempdir().expect("tempdir");
        let svc = service(dir.path(), Some(0x21));
        let clair = contenu();
        let source = dir.path().join("source");
        std::fs::write(&source, &clair).expect("source");

        let prepare = svc.preparer_blob(&source, None).await.expect("préparation");
        assert_eq!(
            prepare.hash,
            DedupService::hash_bytes(&clair),
            "hachage du clair"
        );
        assert_eq!(prepare.size, clair.len() as u64);
        let sur_disque = std::fs::read(&prepare.blob_path).expect("blob");
        assert!(
            blob_cipher::est_chiffre(&sur_disque),
            "blob chiffré au repos"
        );
        assert!(
            !sur_disque.windows(64).any(|w| w == &clair[..64]),
            "aucun clair"
        );
        assert!(source.exists(), "source conservée jusqu'à la validation");
        svc.liberer_source(&prepare).await;
        assert!(!source.exists());

        // Relecture transparente : entier, plage, taille claire, copie claire.
        assert_eq!(
            lire(&svc, &prepare.hash, 0, None).await.expect("lecture"),
            clair
        );
        assert_eq!(
            lire(&svc, &prepare.hash, 70_000, Some(140_001))
                .await
                .expect("plage"),
            clair[70_000..140_001]
        );
        assert_eq!(
            svc.blob_size(&prepare.hash).await.expect("taille"),
            clair.len() as u64
        );
        let copie = dir.path().join("copie");
        svc.copier_blob_clair(&prepare.hash, &copie)
            .await
            .expect("copie");
        assert_eq!(std::fs::read(&copie).expect("copie"), clair);

        // Même magasin, autre clé ou aucune clé : refus, jamais d'octets bruts.
        for autre in [service(dir.path(), Some(0x22)), service(dir.path(), None)] {
            assert!(lire(&autre, &prepare.hash, 0, None).await.is_err());
            assert!(
                autre
                    .copier_blob_clair(&prepare.hash, &copie)
                    .await
                    .is_err()
            );
        }
    }

    #[tokio::test]
    async fn blob_clair_herite_relu_apres_activation() {
        let dir = tempfile::tempdir().expect("tempdir");
        let clair = contenu();
        let source = dir.path().join("source");
        std::fs::write(&source, &clair).expect("source");

        // Écrit avant l'activation du chiffrement.
        let avant = service(dir.path(), None);
        let prepare = avant
            .preparer_blob(&source, None)
            .await
            .expect("préparation");
        assert_eq!(
            std::fs::read(&prepare.blob_path).expect("blob"),
            clair,
            "clair"
        );

        // Relu tel quel une fois le chiffrement activé.
        let apres = service(dir.path(), Some(0x31));
        assert_eq!(
            lire(&apres, &prepare.hash, 0, None).await.expect("lecture"),
            clair
        );
        assert_eq!(
            lire(&apres, &prepare.hash, 5, Some(9))
                .await
                .expect("plage"),
            clair[5..9]
        );
        assert_eq!(
            apres.blob_size(&prepare.hash).await.expect("taille"),
            clair.len() as u64
        );

        // Même contenu rangé de nouveau : le blob existant est réutilisé.
        std::fs::write(&source, &clair).expect("source");
        let encore = apres.preparer_blob(&source, None).await.expect("dédup");
        assert_eq!(encore.blob_path, prepare.blob_path);
        assert!(!blob_cipher::est_chiffre(
            &std::fs::read(&encore.blob_path).expect("blob")
        ));
    }

    #[tokio::test]
    async fn hachage_invalide_refuse_sans_panique() {
        let dir = tempfile::tempdir().expect("tempdir");
        let svc = service(dir.path(), None);
        let source = dir.path().join("source");
        std::fs::write(&source, b"x").expect("source");
        assert!(
            svc.preparer_blob(&source, Some("../../x".into()))
                .await
                .is_err()
        );
        assert!(svc.copier_blob_clair("z", &source).await.is_err());
        assert!(svc.blob_path("").ends_with("00/.blob"));
        assert!(est_hachage_valide(&DedupService::hash_bytes(b"x")));
        assert!(!est_hachage_valide("abc"));
    }
}
