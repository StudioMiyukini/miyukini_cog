use crate::common::dav_error::DavError;
use miyucloud::crypto::KeyManager;
use miyucloud::data::types::{FileEntry, FolderEntry};
use miyucloud::data::MiyucloudDb;

use std::sync::Arc;

/// A resource in the DAV tree (file or collection)
#[derive(Debug, Clone)]
pub enum DavResource {
    File(FileEntry),
    Collection(FolderEntry),
    Root { owner_id: String },
}

impl DavResource {
    pub fn display_name(&self) -> &str {
        match self {
            Self::File(f) => &f.name,
            Self::Collection(d) => &d.name,
            Self::Root { .. } => "/",
        }
    }

    pub fn is_collection(&self) -> bool {
        matches!(self, Self::Collection(_) | Self::Root { .. })
    }

    pub fn content_type(&self) -> &str {
        match self {
            Self::File(f) => &f.mime_type,
            Self::Collection(_) | Self::Root { .. } => "httpd/unix-directory",
        }
    }

    pub fn content_length(&self) -> u64 {
        match self {
            Self::File(f) => f.size_bytes,
            _ => 0,
        }
    }

    pub fn etag(&self) -> String {
        match self {
            Self::File(f) => f.checksum_sha256.clone(),
            Self::Collection(d) => d.updated_at.clone(),
            Self::Root { .. } => "root".to_string(),
        }
    }

    pub fn created_at(&self) -> &str {
        match self {
            Self::File(f) => &f.created_at,
            Self::Collection(d) => &d.created_at,
            Self::Root { .. } => "1970-01-01T00:00:00Z",
        }
    }

    pub fn last_modified(&self) -> &str {
        match self {
            Self::File(f) => &f.updated_at,
            Self::Collection(d) => &d.updated_at,
            Self::Root { .. } => "1970-01-01T00:00:00Z",
        }
    }

    pub fn owner_id(&self) -> &str {
        match self {
            Self::File(f) => &f.owner_id,
            Self::Collection(d) => &d.owner_id,
            Self::Root { owner_id } => owner_id,
        }
    }
}

/// Storage adapter bridging WebDAV to miyucloud DB + storage backend.
///
/// Thread-safe: wraps the DB in an Arc, intended for use in axum State.
#[derive(Clone)]
pub struct DavStore {
    db: Arc<MiyucloudDb>,
    storage: Arc<dyn miyucloud::storage::StorageBackend + Send + Sync>,
    key_manager: Arc<KeyManager>,
}

impl DavStore {
    pub fn new(
        db: Arc<MiyucloudDb>,
        storage: Arc<dyn miyucloud::storage::StorageBackend + Send + Sync>,
        key_manager: Arc<KeyManager>,
    ) -> Self {
        Self {
            db,
            storage,
            key_manager,
        }
    }

    /// Resolve a DAV path like `/Documents/report.pdf` into a DavResource.
    /// The path is relative to the user's root.
    pub fn resolve_path(&self, owner_id: &str, path: &str) -> Result<DavResource, DavError> {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if segments.is_empty() {
            return Ok(DavResource::Root {
                owner_id: owner_id.to_string(),
            });
        }

        let mut current_parent: Option<String> = None;

        for (i, segment) in segments.iter().enumerate() {
            let is_last = i == segments.len() - 1;

            // Try folder first
            let folder = self.find_folder(owner_id, current_parent.as_deref(), segment)?;
            if let Some(f) = folder {
                if is_last {
                    return Ok(DavResource::Collection(f));
                }
                current_parent = Some(f.id);
                continue;
            }

            // If last segment, try file
            if is_last {
                let file = self.find_file(owner_id, current_parent.as_deref(), segment)?;
                if let Some(f) = file {
                    return Ok(DavResource::File(f));
                }
            }

            return Err(DavError::NotFound(format!("Resource not found: {path}")));
        }

        Err(DavError::NotFound(format!("Resource not found: {path}")))
    }

    /// List children of a collection (files + folders in the given parent).
    pub fn list_children(
        &self,
        owner_id: &str,
        parent_id: Option<&str>,
    ) -> Result<Vec<DavResource>, DavError> {
        let folders = self
            .db
            .folder_list(parent_id, owner_id)
            .map_err(|e| DavError::Storage(e.to_string()))?;

        let files = self
            .db
            .file_list(parent_id, owner_id)
            .map_err(|e| DavError::Storage(e.to_string()))?;

        let mut children: Vec<DavResource> = folders
            .into_iter()
            .map(DavResource::Collection)
            .collect();

        children.extend(files.into_iter().map(DavResource::File));

        Ok(children)
    }

    /// Upload a file via WebDAV PUT.
    pub fn put_file(
        &self,
        owner_id: &str,
        parent_id: Option<&str>,
        name: &str,
        data: &[u8],
    ) -> Result<FileEntry, DavError> {
        let mime = mime_guess::from_path(name)
            .first_or_octet_stream()
            .to_string();

        miyucloud::domain::FileOps::upload_file(
            &self.db,
            self.storage.as_ref(),
            &self.key_manager,
            owner_id,
            parent_id,
            name,
            &mime,
            data,
        )
        .map_err(|e| DavError::Storage(e.to_string()))
    }

    /// Download a file via WebDAV GET.
    pub fn get_file(&self, file: &FileEntry) -> Result<Vec<u8>, DavError> {
        miyucloud::domain::FileOps::download_file(
            &self.db,
            self.storage.as_ref(),
            &self.key_manager,
            &file.id,
        )
        .map_err(|e| DavError::Storage(e.to_string()))
    }

    /// Delete a file (moves to trash).
    pub fn delete_file(&self, file_id: &str) -> Result<(), DavError> {
        miyucloud::domain::FileOps::delete_file(&self.db, file_id)
            .map_err(|e| DavError::Storage(e.to_string()))
    }

    /// Create a collection (folder) via MKCOL.
    pub fn create_collection(
        &self,
        owner_id: &str,
        parent_id: Option<&str>,
        name: &str,
    ) -> Result<FolderEntry, DavError> {
        miyucloud::domain::FileOps::create_folder(&self.db, owner_id, parent_id, name)
            .map_err(|e| DavError::Storage(e.to_string()))
    }

    /// Delete a collection (folder).
    pub fn delete_collection(&self, folder_id: &str) -> Result<(), DavError> {
        self.db
            .folder_delete(folder_id)
            .map_err(|e| DavError::Storage(e.to_string()))
    }

    /// Move (rename) a file.
    pub fn move_file(
        &self,
        file_id: &str,
        new_parent_id: Option<&str>,
        new_name: &str,
    ) -> Result<(), DavError> {
        miyucloud::domain::FileOps::move_file(&self.db, file_id, new_parent_id)
            .map_err(|e| DavError::Storage(e.to_string()))?;
        miyucloud::domain::FileOps::rename_file(&self.db, file_id, new_name)
            .map_err(|e| DavError::Storage(e.to_string()))
    }

    // -- Private helpers --

    fn find_folder(
        &self,
        owner_id: &str,
        parent_id: Option<&str>,
        name: &str,
    ) -> Result<Option<FolderEntry>, DavError> {
        let folders = self
            .db
            .folder_list(parent_id, owner_id)
            .map_err(|e| DavError::Storage(e.to_string()))?;

        Ok(folders.into_iter().find(|f| f.name == name))
    }

    fn find_file(
        &self,
        owner_id: &str,
        parent_id: Option<&str>,
        name: &str,
    ) -> Result<Option<FileEntry>, DavError> {
        let files = self
            .db
            .file_list(parent_id, owner_id)
            .map_err(|e| DavError::Storage(e.to_string()))?;

        Ok(files.into_iter().find(|f| f.name == name))
    }
}
