//! Test d'integration : flux complet storage + crypto.
//!
//! @id: miyucloud_integration_storage_crypto
//! @do: test_full_upload_download_flow
//! @role: test
//! @layer: integration
//!
//! Ce test couvre le flux complet :
//! donnees -> chunk -> chiffrer -> ecrire -> lire -> dechiffrer -> reassembler -> verifier SHA-256

#[cfg(feature = "legacy-sqlite")]
mod integration {
    use miyucloud::crypto::{self, KeyManager};
    use miyucloud::storage::local_fs::LocalFsStorage;
    use miyucloud::storage::{compute_sha256, verify_sha256, Chunker, StorageBackend};

    #[test]
    fn test_full_write_flow() {
        // --- Setup ---
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.path().to_path_buf()).unwrap();
        let key_manager = KeyManager::from_master_key([42u8; 32]);
        let file_id = "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d";
        let chunker = Chunker::new();

        // --- Original data ---
        // Create data that spans multiple chunks (>256 KB)
        let mut original_data = Vec::with_capacity(500 * 1024);
        for i in 0u32..(500 * 1024 / 4) {
            original_data.extend_from_slice(&i.to_le_bytes());
        }
        let original_sha256 = compute_sha256(&original_data);

        // --- WRITE: chunk -> encrypt -> store ---
        let file_key = key_manager.file_key(file_id).unwrap();
        let chunks = chunker.split(&original_data);
        assert!(chunks.len() > 1, "Expected multiple chunks for 500 KB data");

        for (i, chunk_data) in chunks.iter().enumerate() {
            let encrypted = crypto::encrypt_chunk(&file_key, chunk_data).unwrap();
            // Serialize: nonce (12 bytes) + ciphertext
            let mut stored = Vec::with_capacity(12 + encrypted.ciphertext.len());
            stored.extend_from_slice(&encrypted.nonce);
            stored.extend_from_slice(&encrypted.ciphertext);
            storage.write_chunk(file_id, i as u32, &stored).unwrap();
        }

        // Verify chunks exist
        let stored_chunks = storage.list_chunks(file_id).unwrap();
        assert_eq!(stored_chunks.len(), chunks.len());

        // --- READ: load -> decrypt -> reassemble -> verify ---
        let mut decrypted_chunks = Vec::new();
        for i in 0..chunks.len() {
            let stored = storage.read_chunk(file_id, i as u32).unwrap();
            assert!(stored.len() >= 12, "Stored chunk too short");
            let mut nonce = [0u8; 12];
            nonce.copy_from_slice(&stored[..12]);
            let ciphertext = stored[12..].to_vec();
            let encrypted = crypto::EncryptedChunk { nonce, ciphertext };
            let plaintext = crypto::decrypt_chunk(&file_key, &encrypted).unwrap();
            decrypted_chunks.push(plaintext);
        }

        let reassembled = chunker.reassemble(&decrypted_chunks);

        // Verify SHA-256 integrity
        verify_sha256(&reassembled, &original_sha256).unwrap();

        // Verify bit-for-bit identical
        assert_eq!(reassembled, original_data);
    }

    #[test]
    fn test_full_flow_with_db() {
        // Test using the full FileOps API which integrates DB + storage + crypto
        use miyucloud::data::MiyucloudDb;
        use miyucloud::domain::FileOps;

        let db = MiyucloudDb::open(":memory:").unwrap();
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.path().to_path_buf()).unwrap();
        let km = KeyManager::from_master_key([99u8; 32]);

        // Upload
        let original = b"Integration test data -- full DB + storage + crypto flow.";
        let entry = FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner-integration",
            None,
            "integration_test.txt",
            "text/plain",
            original,
        )
        .unwrap();

        // Verify DB entry
        let db_entry = db.file_by_id(&entry.id).unwrap().unwrap();
        assert_eq!(db_entry.name, "integration_test.txt");
        assert_eq!(db_entry.size_bytes, original.len() as u64);

        // Download and verify
        let downloaded = FileOps::download_file(&db, &storage, &km, &entry.id).unwrap();
        assert_eq!(downloaded, original);

        // Verify encrypted storage is not plaintext
        let stored = storage.read_chunk(&entry.id, 0).unwrap();
        assert_ne!(&stored[12..], original, "Stored data should be encrypted, not plaintext");
    }
}
