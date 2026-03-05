//! Dechiffrement streaming pour le partage web.
//!
//! @id: miyucloud_crypto_streaming
//! @do: decrypt_file_chunks_streaming_constant_memory
//! @role: crypto
//! @layer: infra
//!
//! Lit les chunks un par un depuis le storage, dechiffre chaque chunk
//! individuellement et retourne un iterateur de donnees claires.
//!
//! INVARIANT : Memoire constante — un seul chunk en RAM a la fois.
//! INVARIANT : Si un chunk est corrompu, l'erreur est signalee immediatement
//! (pas de continuation silencieuse).

use crate::crypto::{self, EncryptedChunk};
use crate::errors::MiyucloudError;
use crate::storage::StorageBackend;

/// Decrypteur streaming : dechiffre un fichier chunk par chunk.
///
/// Utilise un iterateur pour garantir que seul un chunk est en memoire a la fois.
pub struct StreamingDecryptor<'a> {
    storage: &'a dyn StorageBackend,
    file_id: String,
    file_key: [u8; 32],
    chunk_count: u32,
    current_chunk: u32,
}

impl<'a> StreamingDecryptor<'a> {
    /// Cree un nouveau decrypteur streaming.
    ///
    /// - `storage` : backend de stockage pour lire les chunks.
    /// - `file_id` : identifiant du fichier.
    /// - `file_key` : cle de chiffrement du fichier (derivee via HKDF).
    /// - `chunk_count` : nombre total de chunks.
    pub fn new(
        storage: &'a dyn StorageBackend,
        file_id: &str,
        file_key: [u8; 32],
        chunk_count: u32,
    ) -> Self {
        Self {
            storage,
            file_id: file_id.to_string(),
            file_key,
            chunk_count,
            current_chunk: 0,
        }
    }

    /// Dechiffre le chunk suivant.
    ///
    /// Retourne `Ok(Some(data))` si un chunk a ete dechiffre,
    /// `Ok(None)` si tous les chunks ont ete traites,
    /// ou `Err(...)` si le chunk est corrompu ou illisible.
    pub fn next_chunk(&mut self) -> Result<Option<Vec<u8>>, MiyucloudError> {
        if self.current_chunk >= self.chunk_count {
            return Ok(None);
        }

        let stored = self.storage.read_chunk(&self.file_id, self.current_chunk)?;
        if stored.len() < 12 {
            return Err(MiyucloudError::Integrity(format!(
                "Chunk {} of file {} is too short ({} bytes, expected >= 12)",
                self.current_chunk,
                self.file_id,
                stored.len()
            )));
        }

        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(&stored[..12]);
        let ciphertext = stored[12..].to_vec();
        let encrypted = EncryptedChunk { nonce, ciphertext };

        let plaintext = crypto::decrypt_chunk(&self.file_key, &encrypted)?;

        self.current_chunk += 1;
        Ok(Some(plaintext))
    }

    /// Retourne le nombre total de chunks.
    pub fn chunk_count(&self) -> u32 {
        self.chunk_count
    }

    /// Retourne l'index du chunk courant.
    pub fn current_index(&self) -> u32 {
        self.current_chunk
    }

    /// Retourne `true` si tous les chunks ont ete traites.
    pub fn is_done(&self) -> bool {
        self.current_chunk >= self.chunk_count
    }

    /// Collecte tous les chunks dechiffres en un seul Vec (utile pour les tests).
    /// ATTENTION : charge tout le fichier en memoire. Ne pas utiliser en production
    /// pour de gros fichiers.
    pub fn collect_all(&mut self) -> Result<Vec<u8>, MiyucloudError> {
        let mut result = Vec::new();
        while let Some(chunk) = self.next_chunk()? {
            result.extend_from_slice(&chunk);
        }
        Ok(result)
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

    #[test]
    fn test_streaming_decrypt_matches_bulk_decrypt() {
        let (db, storage, km) = setup();
        let original_data = b"Hello, streaming decryption! This is test data.";
        let entry = FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner1",
            None,
            "test.txt",
            "text/plain",
            original_data,
        )
        .unwrap();

        // Bulk decrypt (standard download)
        let bulk_result = FileOps::download_file(&db, &storage, &km, &entry.id).unwrap();

        // Streaming decrypt
        let file_key = km.file_key(&entry.id).unwrap();
        let mut decryptor =
            StreamingDecryptor::new(&storage, &entry.id, file_key, entry.chunk_count);
        let streaming_result = decryptor.collect_all().unwrap();

        assert_eq!(bulk_result, streaming_result);
        assert_eq!(streaming_result, original_data);
    }

    #[test]
    fn test_streaming_decrypt_chunk_by_chunk() {
        let (db, storage, km) = setup();
        // Create data larger than one chunk (256 KB) to have multiple chunks
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

        assert!(entry.chunk_count > 1, "Should have multiple chunks");

        let file_key = km.file_key(&entry.id).unwrap();
        let mut decryptor =
            StreamingDecryptor::new(&storage, &entry.id, file_key, entry.chunk_count);

        let mut chunks_read = 0u32;
        let mut total_bytes = 0usize;
        while let Some(chunk) = decryptor.next_chunk().unwrap() {
            chunks_read += 1;
            total_bytes += chunk.len();
        }

        assert_eq!(chunks_read, entry.chunk_count);
        assert_eq!(total_bytes, original_data.len());
        assert!(decryptor.is_done());
    }

    #[test]
    fn test_streaming_decrypt_corrupted_chunk_fails() {
        let (db, storage, km) = setup();
        let entry = FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner1",
            None,
            "test.txt",
            "text/plain",
            b"secret data",
        )
        .unwrap();

        // Corrupt the stored chunk
        let stored = storage.read_chunk(&entry.id, 0).unwrap();
        let mut corrupted = stored;
        // Corrupt some bytes in the ciphertext (after the 12-byte nonce)
        if corrupted.len() > 14 {
            corrupted[14] ^= 0xFF;
        }
        storage.write_chunk(&entry.id, 0, &corrupted).unwrap();

        let file_key = km.file_key(&entry.id).unwrap();
        let mut decryptor =
            StreamingDecryptor::new(&storage, &entry.id, file_key, entry.chunk_count);

        let result = decryptor.next_chunk();
        assert!(result.is_err(), "Should fail on corrupted chunk");
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Decryption failed"));
    }

    #[test]
    fn test_streaming_empty_file() {
        let (db, storage, km) = setup();
        let entry = FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner1",
            None,
            "empty.txt",
            "text/plain",
            b"",
        )
        .unwrap();

        let file_key = km.file_key(&entry.id).unwrap();
        let mut decryptor =
            StreamingDecryptor::new(&storage, &entry.id, file_key, entry.chunk_count);
        let result = decryptor.collect_all().unwrap();
        assert!(result.is_empty());
    }
}
