//! Corbeille MiyuCloud : suppression douce, restauration, purge securisee.
//!
//! @id: miyucloud_domain_trash
//! @do: implement_trash_restore_purge_operations
//! @role: domain
//! @layer: domain
//!
//! La purge ecrase les chunks avec des zeros avant suppression (securite RGPD).
//! Les elements en corbeille sont masques des listings normaux.

use crate::data::types::{FolderContents};
use crate::errors::MiyucloudError;
use crate::storage::StorageBackend;

#[cfg(feature = "legacy-sqlite")]
use crate::data::MiyucloudDb;

/// Operations sur la corbeille.
pub struct TrashOps;

impl TrashOps {
    /// Met un fichier en corbeille (soft delete).
    #[cfg(feature = "legacy-sqlite")]
    pub fn trash_file(db: &MiyucloudDb, file_id: &str) -> Result<(), MiyucloudError> {
        db.file_trash(file_id)
    }

    /// Met un dossier en corbeille (soft delete).
    #[cfg(feature = "legacy-sqlite")]
    pub fn trash_folder(db: &MiyucloudDb, folder_id: &str) -> Result<(), MiyucloudError> {
        db.folder_trash(folder_id)
    }

    /// Restaure un fichier depuis la corbeille.
    #[cfg(feature = "legacy-sqlite")]
    pub fn restore_file(db: &MiyucloudDb, file_id: &str) -> Result<(), MiyucloudError> {
        db.file_restore(file_id)
    }

    /// Restaure un dossier depuis la corbeille.
    #[cfg(feature = "legacy-sqlite")]
    pub fn restore_folder(db: &MiyucloudDb, folder_id: &str) -> Result<(), MiyucloudError> {
        db.folder_restore(folder_id)
    }

    /// Purge definitivement un fichier : ecrase les chunks avec des zeros puis supprime.
    #[cfg(feature = "legacy-sqlite")]
    pub fn purge_file(
        db: &MiyucloudDb,
        storage: &dyn StorageBackend,
        file_id: &str,
    ) -> Result<(), MiyucloudError> {
        // Get file entry to know the chunk count and size
        let file_entry = db.file_by_id(file_id)?;
        let size_bytes = file_entry.as_ref().map_or(0, |f| f.size_bytes);
        let chunk_count = file_entry.as_ref().map_or(0, |f| f.chunk_count);
        let owner_id = file_entry.as_ref().map(|f| f.owner_id.clone());

        // 1. Overwrite each chunk with zeros
        for i in 0..chunk_count {
            storage.overwrite_chunk_with_zeros(file_id, i)?;
        }

        // 2. Delete the file directory
        storage.delete_file(file_id)?;

        // 3. Update quota (subtract file size)
        if let Some(oid) = &owner_id {
            db.quota_update_used(oid, -(size_bytes as i64))?;
        }

        // 4. Delete from DB
        db.file_delete(file_id)?;

        Ok(())
    }

    /// Purge definitivement un dossier et tout son contenu recursivement.
    #[cfg(feature = "legacy-sqlite")]
    pub fn purge_folder(
        db: &MiyucloudDb,
        storage: &dyn StorageBackend,
        folder_id: &str,
    ) -> Result<(), MiyucloudError> {
        // Delete all files in this folder from the trashed listing
        let folder = db.folder_by_id(folder_id)?;
        if let Some(f) = &folder {
            // Purge files inside the folder (including non-trashed ones since we're purging the whole folder)
            let files = db.file_list(Some(folder_id), &f.owner_id)?;
            for file in &files {
                Self::purge_file(db, storage, &file.id)?;
            }
            // Also purge trashed files that were in this folder
            let trashed_files = db.file_list_trashed(&f.owner_id)?;
            for file in &trashed_files {
                if file.parent_id.as_deref() == Some(folder_id) {
                    Self::purge_file(db, storage, &file.id)?;
                }
            }
        }
        db.folder_delete(folder_id)?;
        Ok(())
    }

    /// Liste les elements en corbeille pour un proprietaire.
    #[cfg(feature = "legacy-sqlite")]
    pub fn list_trashed(
        db: &MiyucloudDb,
        owner_id: &str,
    ) -> Result<FolderContents, MiyucloudError> {
        let files = db.file_list_trashed(owner_id)?;
        let folders = db.folder_list_trashed(owner_id)?;
        Ok(FolderContents {
            folder: None,
            subfolders: folders,
            files,
        })
    }

    /// Vide la corbeille pour un proprietaire : purge tous les elements.
    #[cfg(feature = "legacy-sqlite")]
    pub fn empty_trash(
        db: &MiyucloudDb,
        storage: &dyn StorageBackend,
        owner_id: &str,
    ) -> Result<(), MiyucloudError> {
        // Purge all trashed files
        let trashed_files = db.file_list_trashed(owner_id)?;
        for file in &trashed_files {
            Self::purge_file(db, storage, &file.id)?;
        }
        // Purge all trashed folders
        let trashed_folders = db.folder_list_trashed(owner_id)?;
        for folder in &trashed_folders {
            Self::purge_folder(db, storage, &folder.id)?;
        }
        Ok(())
    }
}

#[cfg(all(test, feature = "legacy-sqlite"))]
mod tests {
    use super::*;
    use crate::crypto::KeyManager;
    use crate::data::MiyucloudDb;
    use crate::domain::file_ops::FileOps;
    use crate::storage::local_fs::LocalFsStorage;

    fn setup() -> (MiyucloudDb, LocalFsStorage, KeyManager) {
        let db = MiyucloudDb::open(":memory:").unwrap();
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.into_path()).unwrap();
        let km = KeyManager::from_master_key([1u8; 32]);
        (db, storage, km)
    }

    #[test]
    fn test_trash_file_sets_is_trashed() {
        let (db, storage, km) = setup();
        let entry = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "test.txt", "text/plain", b"data",
        )
        .unwrap();
        TrashOps::trash_file(&db, &entry.id).unwrap();
        let found = db.file_by_id(&entry.id).unwrap().unwrap();
        assert!(found.is_trashed);
        assert!(found.trashed_at.is_some());
    }

    #[test]
    fn test_restore_file_clears_is_trashed() {
        let (db, storage, km) = setup();
        let entry = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "test.txt", "text/plain", b"data",
        )
        .unwrap();
        TrashOps::trash_file(&db, &entry.id).unwrap();
        TrashOps::restore_file(&db, &entry.id).unwrap();
        let found = db.file_by_id(&entry.id).unwrap().unwrap();
        assert!(!found.is_trashed);
        assert!(found.trashed_at.is_none());
    }

    #[test]
    fn test_purge_file_deletes_permanently() {
        let (db, storage, km) = setup();
        let entry = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "purge.txt", "text/plain", b"secret",
        )
        .unwrap();
        let file_id = entry.id.clone();
        TrashOps::purge_file(&db, &storage, &file_id).unwrap();
        assert!(db.file_by_id(&file_id).unwrap().is_none());
        assert!(!storage.exists(&file_id).unwrap());
    }

    #[test]
    fn test_purge_deletes_storage_chunks() {
        let (db, storage, km) = setup();
        let entry = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "chunks.txt", "text/plain", b"some data",
        )
        .unwrap();
        assert!(storage.exists(&entry.id).unwrap());
        TrashOps::purge_file(&db, &storage, &entry.id).unwrap();
        assert!(!storage.exists(&entry.id).unwrap());
    }

    #[test]
    fn test_list_trashed_files() {
        let (db, storage, km) = setup();
        let entry = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "test.txt", "text/plain", b"data",
        )
        .unwrap();
        TrashOps::trash_file(&db, &entry.id).unwrap();

        let trashed = TrashOps::list_trashed(&db, "owner1").unwrap();
        assert_eq!(trashed.files.len(), 1);
        assert_eq!(trashed.files[0].id, entry.id);
    }

    #[test]
    fn test_empty_trash() {
        let (db, storage, km) = setup();
        let e1 = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "a.txt", "text/plain", b"aaa",
        )
        .unwrap();
        let e2 = FileOps::upload_file(
            &db, &storage, &km, "owner1", None, "b.txt", "text/plain", b"bbb",
        )
        .unwrap();
        TrashOps::trash_file(&db, &e1.id).unwrap();
        TrashOps::trash_file(&db, &e2.id).unwrap();

        let trashed_before = TrashOps::list_trashed(&db, "owner1").unwrap();
        assert_eq!(trashed_before.files.len(), 2);

        TrashOps::empty_trash(&db, &storage, "owner1").unwrap();

        let trashed_after = TrashOps::list_trashed(&db, "owner1").unwrap();
        assert_eq!(trashed_after.files.len(), 0);
    }
}
