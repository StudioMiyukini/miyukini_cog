//! @id     miyucloud_application_services_file_upload_service
//! @do     extract_username_from_path
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « application services file upload service ».

use std::path::Path;
use std::sync::Arc;

use crate::application::dtos::file_dto::FileDto;
use crate::application::ports::file_ports::FileUploadUseCase;
use crate::application::ports::storage_ports::FileReadPort;
use crate::application::services::desarmement_service::{DesarmementService, IssueDesarmement};
use crate::application::services::storage_usage_service::StorageUsageService;
use crate::common::errors::DomainError;
use crate::infrastructure::repositories::pg::FileBlobReadRepository;
use crate::infrastructure::repositories::pg::FileBlobWriteRepository;
use tracing::{debug, info, warn};

/// Helper function to extract username from folder path string.
/// e.g. "My Folder - user1/subfolder/file.txt" → "user1"
fn extract_username_from_path(path: &str) -> Option<String> {
    if !path.contains("My Folder - ") {
        return None;
    }
    let parts: Vec<&str> = path.split("My Folder - ").collect();
    if parts.len() <= 1 {
        return None;
    }
    let remainder = parts[1].trim();
    let username = remainder.split('/').next().unwrap_or(remainder);
    let username = username.trim();
    if username.is_empty() {
        return None;
    }
    Some(username.to_string())
}

/// Service for file upload operations.
///
/// **Every upload path converges on streaming-to-disk** — there is no
/// `Vec<u8>` buffer path.
///
/// - **Normal uploads**: handler spools multipart to temp file → `upload_file_streaming`
/// - **Chunked uploads**: chunks already on disk → `upload_file_from_path`
/// - **WebDAV PUT (large)**: handler streams body to temp file → `update_file_streaming`
/// - **WebDAV PUT (small / compat)**: `create_file` / `update_file` spool `&[u8]`
///   to a temp file internally, then call the streaming path.
///
/// Peak RAM usage during any upload: ~256 KB (streaming hash) regardless of file size,
/// plus the CDR working set (bounded by `miyucdr::Limites`) for disarmable types.
///
/// **Entonnoir CDR (EX-45, décision A-004)** : chaque méthode passe le fichier
/// temporaire dans [`DesarmementService`] *avant* tout hachage, rangement ou
/// écriture en base. Le blob, la taille, le hachage et le type MIME stockés
/// sont ceux de la reconstruction ; les types non pris en charge sont rangés
/// en quarantaine ; les contenus discordants ou malformés sont refusés.
pub struct FileUploadService {
    /// Write port — handles save, streaming, deferred registration
    file_write: Arc<FileBlobWriteRepository>,
    /// Read port — needed for WebDAV create_file / update_file
    file_read: Option<Arc<FileBlobReadRepository>>,
    /// Optional storage usage tracking
    storage_usage_service: Option<Arc<StorageUsageService>>,
    /// Entonnoir CDR — obligatoire, jamais désactivable.
    desarmement: Arc<DesarmementService>,
}

impl FileUploadService {
    /// Constructor with write port only (minimal).
    pub fn new(file_repository: Arc<FileBlobWriteRepository>) -> Self {
        Self {
            file_write: file_repository,
            file_read: None,
            storage_usage_service: None,
            desarmement: Arc::new(DesarmementService::default()),
        }
    }

    /// Constructor for blob-storage model: write + read ports.
    pub fn new_with_read(
        file_write: Arc<FileBlobWriteRepository>,
        file_read: Arc<FileBlobReadRepository>,
    ) -> Self {
        Self {
            file_write,
            file_read: Some(file_read),
            storage_usage_service: None,
            desarmement: Arc::new(DesarmementService::default()),
        }
    }

    /// Configures the storage usage service
    pub fn with_storage_usage_service(
        mut self,
        storage_usage_service: Arc<StorageUsageService>,
    ) -> Self {
        self.storage_usage_service = Some(storage_usage_service);
        self
    }

    /// Partage l'entonnoir CDR de l'application (mêmes limites, même borne
    /// de concurrence).
    pub fn with_desarmement(mut self, desarmement: Arc<DesarmementService>) -> Self {
        self.desarmement = desarmement;
        self
    }

    // ── private helpers ──────────────────────────────────────────

    /// Entonnoir CDR : désarme `temp_path` (réécrit en place si désarmé).
    /// En cas de refus, le fichier temporaire est supprimé : rien n'est stocké.
    async fn desarmer(
        &self,
        temp_path: &Path,
        content_type: &str,
        pre_computed_hash: Option<String>,
    ) -> Result<IssueDesarmement, DomainError> {
        match self
            .desarmement
            .desarmer_fichier(temp_path, content_type, pre_computed_hash)
            .await
        {
            Ok(issue) => Ok(issue),
            Err(e) => {
                let _ = tokio::fs::remove_file(temp_path).await;
                Err(e)
            }
        }
    }

    /// Range un nouveau fichier déjà passé par l'entonnoir.
    async fn ranger_nouveau(
        &self,
        name: String,
        folder_id: Option<String>,
        temp_path: &Path,
        issue: &IssueDesarmement,
    ) -> Result<FileDto, DomainError> {
        let file = self
            .file_write
            .save_file_from_temp_cdr(
                name,
                folder_id,
                issue.type_mime.clone(),
                temp_path,
                issue.taille,
                issue.hachage.clone(),
                &issue.marque(),
            )
            .await?;
        let mut dto = FileDto::from(file);
        dto.etat_cdr = Some(issue.etat.as_str().to_string());
        Ok(dto)
    }

    /// Remplace le contenu d'un fichier existant, déjà passé par l'entonnoir.
    async fn remplacer_contenu(
        &self,
        file_id: &str,
        temp_path: &Path,
        issue: &IssueDesarmement,
        modified_at: Option<i64>,
    ) -> Result<(), DomainError> {
        self.file_write
            .update_file_content_from_temp_cdr(
                file_id,
                temp_path,
                issue.taille,
                Some(issue.type_mime.clone()),
                issue.hachage.clone(),
                modified_at,
                &issue.marque(),
            )
            .await
            .map(|_| ())
    }

    /// Optionally update storage usage after a successful upload.
    fn maybe_update_storage_usage(&self, file: &FileDto) {
        if let Some(storage_service) = &self.storage_usage_service {
            let file_path = file.path.clone();
            if let Some(username) = extract_username_from_path(&file_path) {
                let service_clone = Arc::clone(storage_service);
                tokio::spawn(async move {
                    match service_clone
                        .update_user_storage_usage_by_username(&username)
                        .await
                    {
                        Ok(usage) => debug!(
                            "Updated storage usage for user {} to {} bytes",
                            username, usage
                        ),
                        Err(e) => warn!("Failed to update storage usage for {}: {}", username, e),
                    }
                });
            }
        }
    }
}

impl FileUploadUseCase for FileUploadService {
    /// Streaming upload from a temp file on disk.
    ///
    /// The temp file goes through the CDR funnel first; it is then consumed
    /// (moved/deleted) by the blob store. `size` and `pre_computed_hash`
    /// describe the received bytes and are superseded by the CDR outcome.
    async fn upload_file_streaming(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        temp_path: &Path,
        size: u64,
        pre_computed_hash: Option<String>,
    ) -> Result<FileDto, DomainError> {
        let issue = self
            .desarmer(temp_path, &content_type, pre_computed_hash)
            .await?;
        let dto = self
            .ranger_nouveau(name.clone(), folder_id, temp_path, &issue)
            .await?;
        info!(
            "📡 STREAMING UPLOAD: {} ({} → {} bytes, CDR: {}, ID: {})",
            name, size, issue.taille, issue.etat, dto.id
        );
        self.maybe_update_storage_usage(&dto);
        Ok(dto)
    }

    /// Upload from a file already on disk (chunked uploads).
    async fn upload_file_from_path(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        file_path: &Path,
        pre_computed_hash: Option<String>,
    ) -> Result<FileDto, DomainError> {
        let size = tokio::fs::metadata(file_path)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FileUpload",
                    format!("Failed to read file metadata: {}", e),
                )
            })?
            .len();

        self.upload_file_streaming(
            name,
            folder_id,
            content_type,
            file_path,
            size,
            pre_computed_hash,
        )
        .await
    }

    /// Creates a file at a specific path (for WebDAV PUT on new resource).
    ///
    /// Spools the in-memory `&[u8]` to a temp file, passes it through the CDR
    /// funnel, then delegates to the streaming store.
    async fn create_file(
        &self,
        parent_path: &str,
        filename: &str,
        content: &[u8],
        content_type: &str,
    ) -> Result<FileDto, DomainError> {
        // Look up the folder ID by folder path
        let parent_id = if !parent_path.is_empty() {
            if let Some(file_read) = &self.file_read {
                // Use get_folder_id_by_path to look up the folder directly
                file_read.get_folder_id_by_path(parent_path).await.ok()
            } else {
                None
            }
        } else {
            None
        };

        // Spool to temp file + hash
        let temp = tempfile::NamedTempFile::new()
            .map_err(|e| DomainError::internal_error("FileUpload", format!("temp file: {e}")))?;
        tokio::fs::write(temp.path(), content)
            .await
            .map_err(|e| DomainError::internal_error("FileUpload", format!("write temp: {e}")))?;
        let hash = blake3::hash(content).to_hex().to_string();

        let issue = self.desarmer(temp.path(), content_type, Some(hash)).await?;
        let dto = self
            .ranger_nouveau(filename.to_string(), parent_id, temp.path(), &issue)
            .await?;
        self.maybe_update_storage_usage(&dto);
        Ok(dto)
    }

    /// Updates an existing file's content, or creates it if not found (for WebDAV PUT).
    ///
    /// Spools the in-memory `&[u8]` to a temp file with hash-on-write,
    /// then delegates to the streaming update/create path (CDR funnel).
    async fn update_file(
        &self,
        path: &str,
        content: &[u8],
        content_type: &str,
        modified_at: Option<i64>,
    ) -> Result<FileDto, DomainError> {
        // Spool to temp file + hash
        let temp = tempfile::NamedTempFile::new()
            .map_err(|e| DomainError::internal_error("FileUpload", format!("temp file: {e}")))?;
        tokio::fs::write(temp.path(), content)
            .await
            .map_err(|e| DomainError::internal_error("FileUpload", format!("write temp: {e}")))?;
        let hash = blake3::hash(content).to_hex().to_string();

        self.update_file_streaming(
            path,
            temp.path(),
            content.len() as u64,
            content_type,
            Some(hash),
            modified_at,
        )
        .await
    }

    /// Streaming update — replaces file content from a temp file on disk.
    ///
    /// The temp file goes through the CDR funnel before any lookup or store.
    /// For new files (not found at `path`), creates the file.
    ///
    /// Peak RAM: ~256 KB regardless of file size (plus the CDR working set).
    async fn update_file_streaming(
        &self,
        path: &str,
        temp_path: &Path,
        _size: u64,
        content_type: &str,
        pre_computed_hash: Option<String>,
        modified_at: Option<i64>,
    ) -> Result<FileDto, DomainError> {
        let issue = self
            .desarmer(temp_path, content_type, pre_computed_hash)
            .await?;

        // Try to find the existing file first
        if let Some(file_read) = &self.file_read
            && let Some(file) = file_read.find_file_by_path(path).await?
        {
            let file_id = file.id().to_string();
            self.remplacer_contenu(&file_id, temp_path, &issue, modified_at)
                .await?;
            // Re-read to get fresh DTO with updated etag and timestamps.
            let updated = file_read.get_file(&file_id).await?;
            let mut dto = FileDto::from(updated);
            dto.etat_cdr = Some(issue.etat.as_str().to_string());
            return Ok(dto);
        }

        // File doesn't exist — create it via streaming upload
        let path_normalized = path.trim_start_matches('/').trim_end_matches('/');
        let (_, filename) = if let Some(idx) = path_normalized.rfind('/') {
            (&path_normalized[..idx], &path_normalized[idx + 1..])
        } else {
            ("", path_normalized)
        };

        // get_parent_folder_id expects the full file path — it strips the
        // last segment (filename) internally to find the parent folder.
        let parent_id = if path_normalized.contains('/') {
            if let Some(file_read) = &self.file_read {
                file_read.get_parent_folder_id(path_normalized).await.ok()
            } else {
                None
            }
        } else {
            None
        };

        self.ranger_nouveau(filename.to_string(), parent_id, temp_path, &issue)
            .await
    }

    /// Replaces the content of an existing file identified by its id.
    async fn update_file_content_by_id(
        &self,
        file_id: &str,
        temp_path: &Path,
        _size: u64,
        content_type: &str,
        pre_computed_hash: Option<String>,
        modified_at: Option<i64>,
    ) -> Result<FileDto, DomainError> {
        let issue = self
            .desarmer(temp_path, content_type, pre_computed_hash)
            .await?;
        let file_read = self.file_read.as_ref().ok_or_else(|| {
            DomainError::internal_error("FileUpload", "file read port unavailable")
        })?;
        self.remplacer_contenu(file_id, temp_path, &issue, modified_at)
            .await?;
        let mut updated = FileDto::from(file_read.get_file(file_id).await?);
        updated.etat_cdr = Some(issue.etat.as_str().to_string());
        self.maybe_update_storage_usage(&updated);
        Ok(updated)
    }
}

#[cfg(test)]
mod tests {
    //! EX-45 : chaque chemin d'écriture passe par l'entonnoir CDR **avant**
    //! tout accès au dépôt. Le dépôt de test n'a pas de base : l'appel échoue
    //! après l'entonnoir, ce qui permet d'observer son effet sur le fichier
    //! temporaire (réécrit si désarmé, supprimé si refusé).
    use super::*;
    use crate::application::services::desarmement_service::tests::{
        PDF_MINIMAL, png_avec_metadonnees,
    };
    use crate::common::errors::ErrorKind;

    fn service() -> FileUploadService {
        FileUploadService::new(Arc::new(FileBlobWriteRepository::new_stub()))
    }

    fn contient(foin: &[u8], aiguille: &[u8]) -> bool {
        foin.windows(aiguille.len()).any(|w| w == aiguille)
    }

    async fn temporaire(octets: &[u8]) -> tempfile::NamedTempFile {
        let t = tempfile::NamedTempFile::new().unwrap();
        tokio::fs::write(t.path(), octets).await.unwrap();
        t
    }

    /// Polyglotte texte/PDF : refusé (400) sur chaque chemin, fichier supprimé.
    #[tokio::test]
    async fn tous_les_chemins_refusent_un_polyglotte() {
        let s = service();

        // Téléversement multipart.
        let t = temporaire(PDF_MINIMAL).await;
        let err = s
            .upload_file_streaming(
                "a.txt".into(),
                None,
                "text/plain".into(),
                t.path(),
                PDF_MINIMAL.len() as u64,
                None,
            )
            .await
            .unwrap_err();
        assert_eq!(err.kind, ErrorKind::InvalidInput);
        assert!(!t.path().exists(), "octets refusés supprimés");

        // Téléversement par morceaux (fichier assemblé).
        let t = temporaire(PDF_MINIMAL).await;
        let err = s
            .upload_file_from_path("a.txt".into(), None, "text/plain".into(), t.path(), None)
            .await
            .unwrap_err();
        assert_eq!(err.kind, ErrorKind::InvalidInput);

        // WebDAV PUT (chemin) et Nextcloud PUT / assemblage (identifiant).
        let t = temporaire(PDF_MINIMAL).await;
        let err = s
            .update_file_streaming("/a.txt", t.path(), 0, "text/plain", None, None)
            .await
            .unwrap_err();
        assert_eq!(err.kind, ErrorKind::InvalidInput);

        let t = temporaire(PDF_MINIMAL).await;
        let err = s
            .update_file_content_by_id("id", t.path(), 0, "text/plain", None, None)
            .await
            .unwrap_err();
        assert_eq!(err.kind, ErrorKind::InvalidInput);

        // WebDAV en mémoire.
        let err = s
            .create_file("", "a.txt", PDF_MINIMAL, "text/plain")
            .await
            .unwrap_err();
        assert_eq!(err.kind, ErrorKind::InvalidInput);
        let err = s
            .update_file("/a.txt", PDF_MINIMAL, "text/plain", None)
            .await
            .unwrap_err();
        assert_eq!(err.kind, ErrorKind::InvalidInput);
    }

    /// Image propre : le fichier temporaire est réécrit avec les octets
    /// reconstruits avant tout rangement (chemins par morceaux et DAV).
    #[tokio::test]
    async fn chemins_morceaux_et_dav_desarment_avant_rangement() {
        let s = service();
        let brut = png_avec_metadonnees();

        let t = temporaire(&brut).await;
        let err = s
            .upload_file_from_path("p.png".into(), None, "image/png".into(), t.path(), None)
            .await
            .unwrap_err();
        // Échec attendu au rangement (pas de dossier / pas de base), pas au CDR.
        assert_ne!(err.kind, ErrorKind::InvalidInput);
        let stocke = tokio::fs::read(t.path()).await.unwrap();
        assert!(
            !contient(&stocke, b"SECRET-METADONNEE"),
            "chunked : désarmé"
        );

        let t = temporaire(&brut).await;
        let _ = s
            .update_file_streaming("/p.png", t.path(), 0, "image/png", None, None)
            .await
            .unwrap_err();
        let stocke = tokio::fs::read(t.path()).await.unwrap();
        assert!(!contient(&stocke, b"SECRET-METADONNEE"), "WebDAV : désarmé");

        let t = temporaire(&brut).await;
        let _ = s
            .update_file_content_by_id("id", t.path(), 0, "image/png", None, None)
            .await
            .unwrap_err();
        let stocke = tokio::fs::read(t.path()).await.unwrap();
        assert!(
            !contient(&stocke, b"SECRET-METADONNEE"),
            "Nextcloud : désarmé"
        );
    }

    /// PDF : quarantaine, octets d'origine conservés (aucun refus).
    #[tokio::test]
    async fn pdf_passe_en_quarantaine_sans_refus() {
        let s = service();
        let t = temporaire(PDF_MINIMAL).await;
        let err = s
            .upload_file_from_path(
                "d.pdf".into(),
                None,
                "application/pdf".into(),
                t.path(),
                None,
            )
            .await
            .unwrap_err();
        assert_ne!(err.kind, ErrorKind::InvalidInput, "quarantaine, pas refus");
        assert_eq!(tokio::fs::read(t.path()).await.unwrap(), PDF_MINIMAL);
    }
}
