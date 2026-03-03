//! Operations CRUD sur les fichiers et dossiers MiyuCloud.
//!
//! @id: miyucloud_domain_file_ops
//! @do: implement_file_upload_download_crud
//! @role: domain
//! @layer: domain
//!
//! Upload : donnees -> chunking -> chiffrement at-rest -> ecriture storage -> insertion DB
//! Download : lecture DB -> lecture storage -> dechiffrement -> reassemblage -> verification SHA-256

use crate::crypto::{self, KeyManager};
use crate::data::types::{FileEntry, FolderContents, FolderEntry};
use crate::errors::MiyucloudError;
use crate::storage::{self, Chunker, StorageBackend};

#[cfg(feature = "legacy-sqlite")]
use crate::data::MiyucloudDb;

/// Operations sur les fichiers et dossiers.
pub struct FileOps;

impl FileOps {
    /// Upload un fichier : chunking -> chiffrement -> stockage -> DB.
    ///
    /// Retourne l'entree fichier creee en base.
    #[cfg(feature = "legacy-sqlite")]
    pub fn upload_file(
        db: &MiyucloudDb,
        storage: &dyn StorageBackend,
        key_manager: &KeyManager,
        owner_id: &str,
        parent_id: Option<&str>,
        name: &str,
        mime_type: &str,
        data: &[u8],
    ) -> Result<FileEntry, MiyucloudError> {
        let file_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        // 1. Compute checksum of original data
        let checksum = storage::compute_sha256(data);

        // 2. Chunk the data
        let chunker = Chunker::new();
        let chunks = chunker.split(data);
        let chunk_count = chunks.len() as u32;

        // 3. Derive file key
        let file_key = key_manager.file_key(&file_id)?;

        // 4. Encrypt and store each chunk
        let mut first_nonce_b64 = None;
        for (i, chunk_data) in chunks.iter().enumerate() {
            let encrypted = crypto::encrypt_chunk(&file_key, chunk_data)?;
            if i == 0 {
                first_nonce_b64 =
                    Some(base64_encode(&encrypted.nonce));
            }
            // Serialize encrypted chunk: nonce (12) + ciphertext
            let mut stored = Vec::with_capacity(12 + encrypted.ciphertext.len());
            stored.extend_from_slice(&encrypted.nonce);
            stored.extend_from_slice(&encrypted.ciphertext);
            storage.write_chunk(&file_id, i as u32, &stored)?;
        }

        // 5. Create DB entry
        let file_entry = FileEntry {
            id: file_id,
            parent_id: parent_id.map(String::from),
            owner_id: owner_id.to_string(),
            name: name.to_string(),
            mime_type: mime_type.to_string(),
            size_bytes: data.len() as u64,
            checksum_sha256: checksum,
            encryption_iv: first_nonce_b64,
            chunk_count,
            is_trashed: false,
            trashed_at: None,
            created_at: now.clone(),
            updated_at: now,
        };
        db.file_create(&file_entry)?;

        // 6. Update quota
        db.quota_update_used(owner_id, data.len() as i64)?;

        Ok(file_entry)
    }

    /// Download un fichier : DB -> storage -> dechiffrement -> reassemblage -> verification.
    ///
    /// Retourne les donnees originales en clair.
    #[cfg(feature = "legacy-sqlite")]
    pub fn download_file(
        db: &MiyucloudDb,
        storage: &dyn StorageBackend,
        key_manager: &KeyManager,
        file_id: &str,
    ) -> Result<Vec<u8>, MiyucloudError> {
        // 1. Read file entry from DB
        let file_entry = db
            .file_by_id(file_id)?
            .ok_or_else(|| MiyucloudError::NotFound(format!("File {file_id} not found")))?;

        if file_entry.is_trashed {
            return Err(MiyucloudError::NotFound(format!(
                "File {file_id} is in trash"
            )));
        }

        // 2. Derive file key
        let file_key = key_manager.file_key(file_id)?;

        // 3. Read, decrypt, and reassemble chunks
        let chunker = Chunker::new();
        let mut decrypted_chunks = Vec::with_capacity(file_entry.chunk_count as usize);
        for i in 0..file_entry.chunk_count {
            let stored = storage.read_chunk(file_id, i)?;
            if stored.len() < 12 {
                return Err(MiyucloudError::Integrity(format!(
                    "Chunk {i} of file {file_id} is too short"
                )));
            }
            let mut nonce = [0u8; 12];
            nonce.copy_from_slice(&stored[..12]);
            let ciphertext = stored[12..].to_vec();
            let encrypted = crypto::EncryptedChunk { nonce, ciphertext };
            let plaintext = crypto::decrypt_chunk(&file_key, &encrypted)?;
            decrypted_chunks.push(plaintext);
        }

        let data = chunker.reassemble(&decrypted_chunks);

        // 4. Verify SHA-256
        storage::verify_sha256(&data, &file_entry.checksum_sha256)?;

        Ok(data)
    }

    /// Cree un nouveau dossier.
    #[cfg(feature = "legacy-sqlite")]
    pub fn create_folder(
        db: &MiyucloudDb,
        owner_id: &str,
        parent_id: Option<&str>,
        name: &str,
    ) -> Result<FolderEntry, MiyucloudError> {
        let now = chrono::Utc::now().to_rfc3339();
        let folder = FolderEntry {
            id: uuid::Uuid::new_v4().to_string(),
            parent_id: parent_id.map(String::from),
            owner_id: owner_id.to_string(),
            name: name.to_string(),
            is_trashed: false,
            trashed_at: None,
            created_at: now.clone(),
            updated_at: now,
        };
        db.folder_create(&folder)
    }

    /// Renomme un fichier ou dossier.
    #[cfg(feature = "legacy-sqlite")]
    pub fn rename_file(
        db: &MiyucloudDb,
        file_id: &str,
        new_name: &str,
    ) -> Result<(), MiyucloudError> {
        if new_name.is_empty() {
            return Err(MiyucloudError::InvalidInput(
                "Name cannot be empty".to_string(),
            ));
        }
        let mut file = db
            .file_by_id(file_id)?
            .ok_or_else(|| MiyucloudError::NotFound(format!("File {file_id} not found")))?;
        file.name = new_name.to_string();
        db.file_update(&file)?;
        Ok(())
    }

    /// Renomme un dossier.
    #[cfg(feature = "legacy-sqlite")]
    pub fn rename_folder(
        db: &MiyucloudDb,
        folder_id: &str,
        new_name: &str,
    ) -> Result<(), MiyucloudError> {
        if new_name.is_empty() {
            return Err(MiyucloudError::InvalidInput(
                "Name cannot be empty".to_string(),
            ));
        }
        let mut folder = db
            .folder_by_id(folder_id)?
            .ok_or_else(|| MiyucloudError::NotFound(format!("Folder {folder_id} not found")))?;
        folder.name = new_name.to_string();
        db.folder_update(&folder)?;
        Ok(())
    }

    /// Deplace un fichier vers un autre dossier.
    #[cfg(feature = "legacy-sqlite")]
    pub fn move_file(
        db: &MiyucloudDb,
        file_id: &str,
        new_parent_id: Option<&str>,
    ) -> Result<(), MiyucloudError> {
        let mut file = db
            .file_by_id(file_id)?
            .ok_or_else(|| MiyucloudError::NotFound(format!("File {file_id} not found")))?;
        file.parent_id = new_parent_id.map(String::from);
        db.file_update(&file)?;
        Ok(())
    }

    /// Supprime un fichier (soft delete -> corbeille).
    #[cfg(feature = "legacy-sqlite")]
    pub fn delete_file(db: &MiyucloudDb, file_id: &str) -> Result<(), MiyucloudError> {
        db.file_trash(file_id)
    }

    /// Supprime un dossier (soft delete -> corbeille).
    #[cfg(feature = "legacy-sqlite")]
    pub fn delete_folder(db: &MiyucloudDb, folder_id: &str) -> Result<(), MiyucloudError> {
        db.folder_trash(folder_id)
    }

    /// Liste le contenu d'un dossier.
    #[cfg(feature = "legacy-sqlite")]
    pub fn list_contents(
        db: &MiyucloudDb,
        folder_id: Option<&str>,
        owner_id: &str,
    ) -> Result<FolderContents, MiyucloudError> {
        db.folder_contents(folder_id, owner_id)
    }
}

/// Encode des bytes en base64 standard.
fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

#[cfg(all(test, feature = "legacy-sqlite"))]
mod tests {
    use super::*;
    use crate::data::MiyucloudDb;
    use crate::storage::local_fs::LocalFsStorage;

    fn setup() -> (MiyucloudDb, LocalFsStorage, KeyManager) {
        let db = MiyucloudDb::open(":memory:").unwrap();
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.into_path()).unwrap();
        let km = KeyManager::from_master_key([1u8; 32]);
        (db, storage, km)
    }

    #[test]
    fn test_upload_file_creates_entry_and_stores_chunks() {
        let (db, storage, km) = setup();
        let data = b"Hello, MiyuCloud!";
        let entry = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "hello.txt", "text/plain", data,
        )
        .unwrap();
        assert_eq!(entry.name, "hello.txt");
        assert_eq!(entry.size_bytes, data.len() as u64);
        assert!(entry.chunk_count >= 1);
        assert!(storage.exists(&entry.id).unwrap());
        // Verify DB entry
        let found = db.file_by_id(&entry.id).unwrap().unwrap();
        assert_eq!(found.name, "hello.txt");
    }

    #[test]
    fn test_download_file_returns_original_data() {
        let (db, storage, km) = setup();
        let original_data = b"Secret document content for testing download.";
        let entry = FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner1",
            None,
            "doc.txt",
            "text/plain",
            original_data,
        )
        .unwrap();

        let downloaded = FileOps::download_file(&db, &storage, &km, &entry.id).unwrap();
        assert_eq!(downloaded, original_data);
    }

    #[test]
    fn test_create_folder() {
        let (db, _storage, _km) = setup();
        let folder = FileOps::create_folder(&db, "owner1", None, "My Folder").unwrap();
        assert_eq!(folder.name, "My Folder");
        assert!(folder.parent_id.is_none());
        let found = db.folder_by_id(&folder.id).unwrap();
        assert!(found.is_some());
    }

    #[test]
    fn test_rename_file() {
        let (db, storage, km) = setup();
        let entry = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "old.txt", "text/plain", b"data",
        )
        .unwrap();
        FileOps::rename_file(&db, &entry.id, "new.txt").unwrap();
        let updated = db.file_by_id(&entry.id).unwrap().unwrap();
        assert_eq!(updated.name, "new.txt");
    }

    #[test]
    fn test_move_file_to_folder() {
        let (db, storage, km) = setup();
        let folder = FileOps::create_folder(&db, "owner1", None, "Target").unwrap();
        let entry = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "file.txt", "text/plain", b"data",
        )
        .unwrap();
        assert!(entry.parent_id.is_none());

        FileOps::move_file(&db, &entry.id, Some(&folder.id)).unwrap();
        let updated = db.file_by_id(&entry.id).unwrap().unwrap();
        assert_eq!(updated.parent_id.as_deref(), Some(folder.id.as_str()));
    }

    #[test]
    fn test_delete_file_trashes_it() {
        let (db, storage, km) = setup();
        let entry = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "delete_me.txt", "text/plain", b"data",
        )
        .unwrap();
        FileOps::delete_file(&db, &entry.id).unwrap();
        let found = db.file_by_id(&entry.id).unwrap().unwrap();
        assert!(found.is_trashed);
    }

    #[test]
    fn test_list_folder_contents() {
        let (db, storage, km) = setup();
        let folder = FileOps::create_folder(&db, "owner1", None, "Docs").unwrap();
        FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner1",
            Some(&folder.id),
            "a.txt",
            "text/plain",
            b"aaa",
        )
        .unwrap();
        FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner1",
            Some(&folder.id),
            "b.txt",
            "text/plain",
            b"bbb",
        )
        .unwrap();
        let contents = FileOps::list_contents(&db, Some(&folder.id), "owner1").unwrap();
        assert_eq!(contents.files.len(), 2);
        assert!(contents.folder.is_some());
    }

    #[test]
    fn test_large_file_upload_download() {
        let (db, storage, km) = setup();
        // Create data larger than one chunk (256 KB)
        let original_data = vec![0xABu8; 300 * 1024];
        let entry = FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner1",
            None,
            "big.bin",
            "application/octet-stream",
            &original_data,
        )
        .unwrap();
        assert!(entry.chunk_count > 1);

        let downloaded = FileOps::download_file(&db, &storage, &km, &entry.id).unwrap();
        assert_eq!(downloaded, original_data);
    }
}
