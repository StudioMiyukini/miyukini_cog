//! Implementation filesystem local pour le stockage de chunks.
//!
//! @id: miyucloud_storage_local_fs
//! @do: implement_local_filesystem_storage_backend
//! @role: storage
//! @layer: infra
//!
//! Structure de stockage :
//! `{storage_root}/chunks/{file_id_prefix_2chars}/{file_id}/chunk_NNNN.enc`

use crate::errors::MiyucloudError;
use crate::storage::StorageBackend;
use std::fs;
use std::path::PathBuf;

/// Backend de stockage filesystem local.
pub struct LocalFsStorage {
    /// Racine du stockage (ex: `~/.miyucloud/storage/`).
    root: PathBuf,
}

impl LocalFsStorage {
    /// Cree un nouveau backend de stockage local.
    /// Cree le repertoire racine et le sous-dossier `chunks/` si necessaires.
    pub fn new(root: PathBuf) -> Result<Self, MiyucloudError> {
        let chunks_dir = root.join("chunks");
        fs::create_dir_all(&chunks_dir)?;
        Ok(Self { root })
    }

    /// Retourne le chemin du repertoire d'un fichier.
    ///
    /// Valide que `file_id` est un UUID pour prevenir les path traversal.
    fn file_dir(&self, file_id: &str) -> Result<PathBuf, MiyucloudError> {
        if !crate::utils::sanitize::validate_uuid(file_id) {
            return Err(MiyucloudError::InvalidInput(format!(
                "Invalid file_id (must be UUID): {file_id}"
            )));
        }
        let prefix = &file_id[..2];
        Ok(self.root.join("chunks").join(prefix).join(file_id))
    }

    /// Retourne le chemin d'un chunk.
    fn chunk_path(&self, file_id: &str, chunk_index: u32) -> Result<PathBuf, MiyucloudError> {
        Ok(self
            .file_dir(file_id)?
            .join(format!("chunk_{chunk_index:04}.enc")))
    }
}

impl StorageBackend for LocalFsStorage {
    fn write_chunk(
        &self,
        file_id: &str,
        chunk_index: u32,
        data: &[u8],
    ) -> Result<(), MiyucloudError> {
        let dir = self.file_dir(file_id)?;
        fs::create_dir_all(&dir)?;
        let path = self.chunk_path(file_id, chunk_index)?;
        fs::write(&path, data)?;
        Ok(())
    }

    fn read_chunk(
        &self,
        file_id: &str,
        chunk_index: u32,
    ) -> Result<Vec<u8>, MiyucloudError> {
        let path = self.chunk_path(file_id, chunk_index)?;
        if !path.exists() {
            return Err(MiyucloudError::NotFound(format!(
                "Chunk {chunk_index} of file {file_id} not found"
            )));
        }
        let data = fs::read(&path)?;
        Ok(data)
    }

    fn delete_file(&self, file_id: &str) -> Result<(), MiyucloudError> {
        let dir = self.file_dir(file_id)?;
        if dir.exists() {
            fs::remove_dir_all(&dir)?;
        }
        Ok(())
    }

    fn exists(&self, file_id: &str) -> Result<bool, MiyucloudError> {
        let dir = self.file_dir(file_id)?;
        Ok(dir.exists() && dir.is_dir())
    }

    fn list_chunks(&self, file_id: &str) -> Result<Vec<u32>, MiyucloudError> {
        let dir = self.file_dir(file_id)?;
        if !dir.exists() {
            return Ok(vec![]);
        }
        let mut indices = Vec::new();
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            // Parse chunk_NNNN.enc
            if let Some(stripped) = name_str.strip_prefix("chunk_") {
                if let Some(num_str) = stripped.strip_suffix(".enc") {
                    if let Ok(idx) = num_str.parse::<u32>() {
                        indices.push(idx);
                    }
                }
            }
        }
        indices.sort_unstable();
        Ok(indices)
    }

    fn overwrite_chunk_with_zeros(
        &self,
        file_id: &str,
        chunk_index: u32,
    ) -> Result<(), MiyucloudError> {
        let path = self.chunk_path(file_id, chunk_index)?;
        if path.exists() {
            let size = fs::metadata(&path)?.len() as usize;
            let zeros = vec![0u8; size];
            fs::write(&path, &zeros)?;
            fs::remove_file(&path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_UUID_1: &str = "550e8400-e29b-41d4-a716-446655440000";
    const TEST_UUID_2: &str = "6ba7b810-9dad-11d1-80b4-00c04fd430c8";
    const TEST_UUID_3: &str = "6ba7b811-9dad-11d1-80b4-00c04fd430c8";
    const TEST_UUID_4: &str = "6ba7b812-9dad-11d1-80b4-00c04fd430c8";
    const TEST_UUID_5: &str = "6ba7b813-9dad-11d1-80b4-00c04fd430c8";

    #[test]
    fn test_write_and_read_file() {
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.path().to_path_buf()).unwrap();
        let data = b"hello world";
        storage.write_chunk(TEST_UUID_1, 0, data).unwrap();
        let read = storage.read_chunk(TEST_UUID_1, 0).unwrap();
        assert_eq!(read, data);
    }

    #[test]
    fn test_delete_file() {
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.path().to_path_buf()).unwrap();
        storage.write_chunk(TEST_UUID_2, 0, b"data").unwrap();
        assert!(storage.exists(TEST_UUID_2).unwrap());
        storage.delete_file(TEST_UUID_2).unwrap();
        assert!(!storage.exists(TEST_UUID_2).unwrap());
        let result = storage.read_chunk(TEST_UUID_2, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_file_exists() {
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.path().to_path_buf()).unwrap();
        assert!(!storage.exists(TEST_UUID_3).unwrap());
        storage.write_chunk(TEST_UUID_3, 0, b"data").unwrap();
        assert!(storage.exists(TEST_UUID_3).unwrap());
    }

    #[test]
    fn test_list_chunks() {
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.path().to_path_buf()).unwrap();
        storage.write_chunk(TEST_UUID_4, 0, b"chunk0").unwrap();
        storage.write_chunk(TEST_UUID_4, 1, b"chunk1").unwrap();
        storage.write_chunk(TEST_UUID_4, 2, b"chunk2").unwrap();
        let chunks = storage.list_chunks(TEST_UUID_4).unwrap();
        assert_eq!(chunks, vec![0, 1, 2]);
    }

    #[test]
    fn test_overwrite_with_zeros() {
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.path().to_path_buf()).unwrap();
        storage.write_chunk(TEST_UUID_5, 0, b"secret data").unwrap();
        storage.overwrite_chunk_with_zeros(TEST_UUID_5, 0).unwrap();
        let result = storage.read_chunk(TEST_UUID_5, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_file_dir_rejects_path_traversal() {
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.path().to_path_buf()).unwrap();
        let result = storage.write_chunk("../../../etc/passwd", 0, b"evil");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid file_id"));
    }

    #[test]
    fn test_file_dir_rejects_non_uuid() {
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.path().to_path_buf()).unwrap();
        assert!(storage.write_chunk("not-a-uuid", 0, b"data").is_err());
        assert!(storage.read_chunk("file-abc", 0).is_err());
        assert!(storage.exists("bad/path").is_err());
    }
}
