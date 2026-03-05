//! Gestion des liens de partage web externes.
//!
//! @id: miyucloud_domain_external_share
//! @do: create_revoke_access_list_external_share_links
//! @role: domain
//! @layer: domain
//!
//! Liens de partage avec expiration obligatoire (min 1h, max 30 jours),
//! mot de passe optionnel, compteur de telechargements et revocation instantanee.
//!
//! INVARIANT : L'expiration est TOUJOURS presente. Pas de lien permanent.
//! INVARIANT : La revocation est un flag, jamais une suppression.

use crate::auth::web_tokens;
use crate::data::types::{FileEntry, ShareLink};
use crate::errors::MiyucloudError;

#[cfg(feature = "legacy-sqlite")]
use crate::data::MiyucloudDb;

/// Nombre minimum d'heures d'expiration.
const MIN_EXPIRY_HOURS: u32 = 1;
/// Nombre maximum d'heures d'expiration (30 jours).
const MAX_EXPIRY_HOURS: u32 = 30 * 24;

/// Operations sur les liens de partage externe.
pub struct ExternalShareOps;

impl ExternalShareOps {
    /// Cree un lien de partage externe.
    ///
    /// - `file_id` ou `folder_id` doit etre fourni (exclusif).
    /// - `expires_in_hours` doit etre entre 1 et 720 (30 jours).
    /// - `password` est optionnel : s'il est fourni, il sera hashe.
    /// - Retourne le `ShareLink` avec le token en clair (unique occasion de le voir).
    #[cfg(feature = "legacy-sqlite")]
    pub fn create_link(
        db: &MiyucloudDb,
        file_id: Option<&str>,
        folder_id: Option<&str>,
        creator_id: &str,
        password: Option<&str>,
        expires_in_hours: u32,
        max_downloads: Option<u32>,
    ) -> Result<ShareLink, MiyucloudError> {
        // Validate exactly one of file_id or folder_id
        match (file_id, folder_id) {
            (Some(_), None) | (None, Some(_)) => {}
            _ => {
                return Err(MiyucloudError::InvalidInput(
                    "Exactly one of file_id or folder_id must be provided".to_string(),
                ));
            }
        }

        // Validate expiration range
        if !(MIN_EXPIRY_HOURS..=MAX_EXPIRY_HOURS).contains(&expires_in_hours) {
            return Err(MiyucloudError::InvalidInput(format!(
                "Expiration must be between {MIN_EXPIRY_HOURS} and {MAX_EXPIRY_HOURS} hours"
            )));
        }

        // Verify the target resource exists
        if let Some(fid) = file_id {
            let file = db.file_by_id(fid)?;
            if file.is_none() {
                return Err(MiyucloudError::NotFound(format!("File {fid} not found")));
            }
        }
        if let Some(foid) = folder_id {
            let folder = db.folder_by_id(foid)?;
            if folder.is_none() {
                return Err(MiyucloudError::NotFound(format!("Folder {foid} not found")));
            }
        }

        let token = web_tokens::generate_share_token();
        let password_hash = match password {
            Some(pwd) => Some(web_tokens::hash_password(pwd)?),
            None => None,
        };

        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::hours(i64::from(expires_in_hours));

        let link = ShareLink {
            id: uuid::Uuid::new_v4().to_string(),
            file_id: file_id.map(String::from),
            folder_id: folder_id.map(String::from),
            creator_id: creator_id.to_string(),
            token,
            has_password: password.is_some(),
            expires_at: expires_at.to_rfc3339(),
            max_downloads,
            download_count: 0,
            is_revoked: false,
            created_at: now.to_rfc3339(),
        };

        db.share_link_create(&link, password_hash.as_deref())?;

        Ok(link)
    }

    /// Revoque un lien de partage (instantane).
    #[cfg(feature = "legacy-sqlite")]
    pub fn revoke_link(db: &MiyucloudDb, link_id: &str) -> Result<(), MiyucloudError> {
        db.share_link_revoke(link_id)
    }

    /// Accede a un lien de partage par token.
    ///
    /// Effectue la validation complete (expiration, revocation, max downloads, mot de passe).
    /// Incremente le compteur de telechargements si la validation reussit.
    /// Retourne le `ShareLink` et le `FileEntry` associe.
    #[cfg(feature = "legacy-sqlite")]
    pub fn access_link(
        db: &MiyucloudDb,
        token: &str,
        password: Option<&str>,
    ) -> Result<(ShareLink, FileEntry), MiyucloudError> {
        let link = db
            .share_link_by_token(token)?
            .ok_or_else(|| MiyucloudError::NotFound("Share link not found".to_string()))?;

        // Get password hash from DB for verification
        let password_hash = db.share_link_password_hash(&link.id)?;

        web_tokens::validate_share_link(&link, password, password_hash.as_deref())?;

        // Get the associated file
        let file_id = link.file_id.as_deref().ok_or_else(|| {
            MiyucloudError::InvalidInput("Folder share download not yet supported".to_string())
        })?;

        let file_entry = db
            .file_by_id(file_id)?
            .ok_or_else(|| MiyucloudError::NotFound("Shared file not found".to_string()))?;

        // Increment download count
        db.share_link_increment_downloads(&link.id)?;

        Ok((link, file_entry))
    }

    /// Liste tous les liens de partage d'un createur.
    #[cfg(feature = "legacy-sqlite")]
    pub fn list_links(
        db: &MiyucloudDb,
        creator_id: &str,
    ) -> Result<Vec<ShareLink>, MiyucloudError> {
        db.share_link_list(creator_id)
    }
}

#[cfg(all(test, feature = "legacy-sqlite"))]
mod tests {
    use super::*;
    use crate::crypto::KeyManager;
    use crate::data::MiyucloudDb;
    use crate::domain::FileOps;
    use crate::storage::local_fs::LocalFsStorage;

    fn setup() -> (MiyucloudDb, LocalFsStorage, KeyManager) {
        let db = MiyucloudDb::open(":memory:").unwrap();
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.into_path()).unwrap();
        let km = KeyManager::from_master_key([1u8; 32]);
        (db, storage, km)
    }

    fn create_test_file(db: &MiyucloudDb, storage: &LocalFsStorage, km: &KeyManager) -> FileEntry {
        FileOps::upload_file(
            db,
            storage,
            km,
            "owner1",
            None,
            "test.txt",
            "text/plain",
            b"test data for sharing",
        )
        .unwrap()
    }

    #[test]
    fn test_create_share_link_with_expiration() {
        let (db, storage, km) = setup();
        let file = create_test_file(&db, &storage, &km);
        let link =
            ExternalShareOps::create_link(&db, Some(&file.id), None, "owner1", None, 24, None)
                .unwrap();

        assert_eq!(link.file_id.as_deref(), Some(file.id.as_str()));
        assert!(!link.has_password);
        assert_eq!(link.download_count, 0);
        assert!(!link.is_revoked);
        assert_eq!(link.token.len(), 64); // 32 bytes hex
    }

    #[test]
    fn test_create_share_link_with_password() {
        let (db, storage, km) = setup();
        let file = create_test_file(&db, &storage, &km);
        let link = ExternalShareOps::create_link(
            &db,
            Some(&file.id),
            None,
            "owner1",
            Some("my-secret"),
            24,
            None,
        )
        .unwrap();

        assert!(link.has_password);
    }

    #[test]
    fn test_revoke_share_link() {
        let (db, storage, km) = setup();
        let file = create_test_file(&db, &storage, &km);
        let link =
            ExternalShareOps::create_link(&db, Some(&file.id), None, "owner1", None, 24, None)
                .unwrap();

        ExternalShareOps::revoke_link(&db, &link.id).unwrap();

        // Access should fail after revocation
        let result = ExternalShareOps::access_link(&db, &link.token, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("revoked"));
    }

    #[test]
    fn test_access_share_link_increments_counter() {
        let (db, storage, km) = setup();
        let file = create_test_file(&db, &storage, &km);
        let link =
            ExternalShareOps::create_link(&db, Some(&file.id), None, "owner1", None, 24, None)
                .unwrap();

        // First access
        let (accessed, _file_entry) =
            ExternalShareOps::access_link(&db, &link.token, None).unwrap();
        // download_count is read before increment, so it's 0 in the returned link
        // but incremented in the DB
        assert_eq!(accessed.download_count, 0);

        // Check DB directly for incremented count
        let db_link = db.share_link_by_token(&link.token).unwrap().unwrap();
        assert_eq!(db_link.download_count, 1);
    }

    #[test]
    fn test_access_revoked_link_fails() {
        let (db, storage, km) = setup();
        let file = create_test_file(&db, &storage, &km);
        let link =
            ExternalShareOps::create_link(&db, Some(&file.id), None, "owner1", None, 24, None)
                .unwrap();

        ExternalShareOps::revoke_link(&db, &link.id).unwrap();
        let result = ExternalShareOps::access_link(&db, &link.token, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_access_expired_link_fails() {
        let (db, storage, km) = setup();
        let file = create_test_file(&db, &storage, &km);

        // Create a link manually with past expiration
        let token = web_tokens::generate_share_token();
        let link = ShareLink {
            id: uuid::Uuid::new_v4().to_string(),
            file_id: Some(file.id.clone()),
            folder_id: None,
            creator_id: "owner1".to_string(),
            token: token.clone(),
            has_password: false,
            expires_at: "2020-01-01T00:00:00+00:00".to_string(),
            max_downloads: None,
            download_count: 0,
            is_revoked: false,
            created_at: "2020-01-01T00:00:00+00:00".to_string(),
        };
        db.share_link_create(&link, None).unwrap();

        let result = ExternalShareOps::access_link(&db, &token, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expired"));
    }

    #[test]
    fn test_list_links() {
        let (db, storage, km) = setup();
        let file = create_test_file(&db, &storage, &km);

        ExternalShareOps::create_link(&db, Some(&file.id), None, "owner1", None, 24, None).unwrap();
        ExternalShareOps::create_link(&db, Some(&file.id), None, "owner1", None, 48, None).unwrap();

        let links = ExternalShareOps::list_links(&db, "owner1").unwrap();
        assert_eq!(links.len(), 2);
    }

    #[test]
    fn test_create_link_invalid_expiration() {
        let (db, storage, km) = setup();
        let file = create_test_file(&db, &storage, &km);

        // Too short
        let result =
            ExternalShareOps::create_link(&db, Some(&file.id), None, "owner1", None, 0, None);
        assert!(result.is_err());

        // Too long (>30 days)
        let result =
            ExternalShareOps::create_link(&db, Some(&file.id), None, "owner1", None, 721, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_link_no_target() {
        let (db, _storage, _km) = setup();
        let result = ExternalShareOps::create_link(&db, None, None, "owner1", None, 24, None);
        assert!(result.is_err());
    }
}
