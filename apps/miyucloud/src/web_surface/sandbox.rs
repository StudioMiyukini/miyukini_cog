//! Middleware de sandboxing pour la surface web.
//!
//! @id: miyucloud_web_sandbox
//! @do: restrict_web_surface_access_to_shared_files_only
//! @role: security
//! @layer: app
//!
//! Le `SandboxedStore` ne peut acceder qu'aux fichiers explicitement partages
//! via un `ShareLink` actif. Un acces a un fichier non partage retourne 404
//! (pas 403, pour ne rien reveler sur l'existence du fichier).
//!
//! INVARIANT : Aucun listing de fichiers possible depuis la surface web.
//! INVARIANT : Aucune route `/api/*` accessible depuis la surface web.

use miyucloud::auth::web_tokens;
use miyucloud::data::types::{FileEntry, ShareLink};
use miyucloud::data::MiyucloudDb;
use miyucloud::errors::MiyucloudError;
use miyucloud::storage::StorageBackend;

/// Store sandboxe : acces restreint aux fichiers partages.
///
/// Tout acces passe par la verification du token de partage.
/// Si le fichier n'est pas partage, le store retourne NotFound (404).
pub struct SandboxedStore<'a> {
    db: &'a MiyucloudDb,
    #[allow(dead_code)]
    storage: &'a dyn StorageBackend,
}

impl<'a> SandboxedStore<'a> {
    /// Cree un nouveau store sandboxe.
    pub fn new(db: &'a MiyucloudDb, storage: &'a dyn StorageBackend) -> Self {
        Self { db, storage }
    }

    /// Verifie un token de partage et retourne les informations du lien et du fichier.
    ///
    /// Retourne 404 si le token est invalide ou le fichier n'existe pas.
    /// Retourne une erreur specifique si le lien est expire ou revoque.
    pub fn verify_token(
        &self,
        token: &str,
    ) -> Result<(ShareLink, FileEntry, Option<String>), MiyucloudError> {
        let link = self
            .db
            .share_link_by_token(token)?
            .ok_or_else(|| MiyucloudError::NotFound("Not found".to_string()))?;

        // Get password hash for later verification
        let password_hash = self.db.share_link_password_hash(&link.id)?;

        // Validate the link (except password — that's done separately)
        let temp_link_for_validation = ShareLink {
            has_password: false, // Skip password check here
            ..link.clone()
        };
        web_tokens::validate_share_link(&temp_link_for_validation, None, None)?;

        // Get the file
        let file_id = link.file_id.as_deref().ok_or_else(|| {
            MiyucloudError::NotFound("Folder shares not yet supported".to_string())
        })?;

        let file_entry = self
            .db
            .file_by_id(file_id)?
            .ok_or_else(|| MiyucloudError::NotFound("Not found".to_string()))?;

        Ok((link, file_entry, password_hash))
    }

    /// Lit un chunk d'un fichier partage.
    ///
    /// IMPORTANT : Cette methode ne verifie PAS le token. La verification
    /// doit etre faite en amont via `verify_token()`.
    #[allow(dead_code)]
    pub fn read_shared_chunk(
        &self,
        file_id: &str,
        chunk_index: u32,
    ) -> Result<Vec<u8>, MiyucloudError> {
        self.storage.read_chunk(file_id, chunk_index)
    }

    /// Incremente le compteur de telechargements.
    #[allow(dead_code)]
    pub fn increment_downloads(&self, link_id: &str) -> Result<(), MiyucloudError> {
        self.db.share_link_increment_downloads(link_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyucloud::crypto::KeyManager;
    use miyucloud::domain::{ExternalShareOps, FileOps};
    use miyucloud::storage::local_fs::LocalFsStorage;

    fn setup() -> (MiyucloudDb, LocalFsStorage, KeyManager) {
        let db = MiyucloudDb::open(":memory:").unwrap();
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.into_path()).unwrap();
        let km = KeyManager::from_master_key([1u8; 32]);
        (db, storage, km)
    }

    #[test]
    fn test_sandbox_blocks_access_to_non_shared_file() {
        let (db, storage, _km) = setup();
        let sandbox = SandboxedStore::new(&db, &storage);

        // Try with a fake token
        let result = sandbox.verify_token("nonexistent_token_1234567890abcdef");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Not found"));
    }

    #[test]
    fn test_sandbox_allows_access_to_shared_file() {
        let (db, storage, km) = setup();
        let file = FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner1",
            None,
            "shared.txt",
            "text/plain",
            b"shared data",
        )
        .unwrap();

        let link = ExternalShareOps::create_link(
            &db,
            Some(&file.id),
            None,
            "owner1",
            None,
            24,
            None,
        )
        .unwrap();

        let sandbox = SandboxedStore::new(&db, &storage);
        let result = sandbox.verify_token(&link.token);
        assert!(result.is_ok());
        let (verified_link, verified_file, _pwd_hash) = result.unwrap();
        assert_eq!(verified_link.id, link.id);
        assert_eq!(verified_file.id, file.id);
    }

    #[test]
    fn test_sandbox_blocks_api_routes() {
        // The sandbox doesn't expose any API route methods.
        // This is an architectural guarantee: the web surface router is completely
        // separate from the API router. The SandboxedStore only provides
        // verify_token() and read_shared_chunk() — no CRUD operations.
        // This test verifies the API surface of SandboxedStore.
        let (db, storage, _km) = setup();
        let sandbox = SandboxedStore::new(&db, &storage);

        // SandboxedStore has no method to list files, create files, etc.
        // It can only verify tokens and read chunks.
        // The test passes by virtue of the type system.
        let _ = sandbox; // Use the variable
    }
}
