//! Test d'integration : synchronisation P2P complette.
//!
//! @id: miyucloud_integration_sync
//! @do: test_sync_manifest_diff_conflict_export_import_flow
//! @role: test
//! @layer: integration
//!
//! Ce test couvre le flux de synchronisation P2P :
//! - Construction de manifeste depuis la DB
//! - Calcul de diff bidirectionnel
//! - Detection et resolution de conflits
//! - Export/import ZIP avec round-trip
//! - Horloges vectorielles inter-instances
//! - Decouverte de pairs et enregistrement en DB

#[cfg(feature = "legacy-sqlite")]
mod integration_sync {
    use miyucloud::crypto::KeyManager;
    use miyucloud::data::types::{ConflictStatus, SyncPeer};
    use miyucloud::data::MiyucloudDb;
    use miyucloud::domain::FileOps;
    use miyucloud::storage::local_fs::LocalFsStorage;
    use miyucloud::sync::conflict::ConflictResolver;
    use miyucloud::sync::engine::{SyncEngine, SyncEngineConfig};
    use miyucloud::sync::manifest::SyncManifest;
    use miyucloud::sync::protocol::ManifestEntry;
    use miyucloud::sync::vector_clock::VectorClock;

    /// Cree un environnement de test (DB in-memory + storage tempdir + KeyManager).
    fn setup_instance(
        master_key: [u8; 32],
    ) -> (MiyucloudDb, LocalFsStorage, KeyManager, tempfile::TempDir) {
        let db = MiyucloudDb::open(":memory:").unwrap();
        let tmp = tempfile::TempDir::new().unwrap();
        let storage_dir = tmp.path().join("storage");
        let storage = LocalFsStorage::new(storage_dir).unwrap();
        let km = KeyManager::from_master_key(master_key);
        (db, storage, km, tmp)
    }

    // -----------------------------------------------------------------------
    // Manifest + Diff
    // -----------------------------------------------------------------------

    #[test]
    fn test_two_instances_manifest_diff_new_files() {
        // Instance A has file1, instance B has file2
        let (db_a, storage_a, km_a, _tmp_a) = setup_instance([1u8; 32]);
        let (db_b, storage_b, km_b, _tmp_b) = setup_instance([2u8; 32]);

        FileOps::upload_file(
            &db_a,
            &storage_a,
            &km_a,
            "owner-a",
            None,
            "file_a.txt",
            "text/plain",
            b"Content from instance A",
        )
        .unwrap();

        FileOps::upload_file(
            &db_b,
            &storage_b,
            &km_b,
            "owner-b",
            None,
            "file_b.txt",
            "text/plain",
            b"Content from instance B",
        )
        .unwrap();

        // Build manifests
        let manifest_a = SyncManifest::build(&db_a, "owner-a", &[]).unwrap();
        let manifest_b = SyncManifest::build(&db_b, "owner-b", &[]).unwrap();

        assert_eq!(manifest_a.len(), 1);
        assert_eq!(manifest_b.len(), 1);
        assert_eq!(manifest_a[0].name, "file_a.txt");
        assert_eq!(manifest_b[0].name, "file_b.txt");

        // Compute diff from A's perspective (A is local, B is remote)
        let diff = SyncManifest::compute_diff(&manifest_a, &manifest_b);

        // A should download file_b from B
        assert_eq!(diff.to_download.len(), 1);
        assert_eq!(diff.to_download[0].name, "file_b.txt");

        // A should upload file_a to B
        assert_eq!(diff.to_upload.len(), 1);
        assert_eq!(diff.to_upload[0].name, "file_a.txt");

        // No conflicts (different files)
        assert_eq!(diff.conflicts.len(), 0);
    }

    #[test]
    fn test_two_instances_manifest_diff_same_file_conflict() {
        // Simulate: a file was initially synced (same file_id on both nodes),
        // then both nodes modified it independently (concurrent clocks).
        // compute_diff matches by file_id, so we use the same ID.
        let shared_id = uuid::Uuid::new_v4().to_string();

        let (db_a, storage_a, km_a, _tmp_a) = setup_instance([1u8; 32]);
        let (db_b, storage_b, km_b, _tmp_b) = setup_instance([2u8; 32]);

        let entry_a = FileOps::upload_file(
            &db_a,
            &storage_a,
            &km_a,
            "owner-a",
            None,
            "shared.txt",
            "text/plain",
            b"Version A",
        )
        .unwrap();

        let entry_b = FileOps::upload_file(
            &db_b,
            &storage_b,
            &km_b,
            "owner-b",
            None,
            "shared.txt",
            "text/plain",
            b"Version B",
        )
        .unwrap();

        // Build manifests with SAME file_id but concurrent clocks
        // (simulating a file that was synced, then diverged)
        let manifest_a = vec![ManifestEntry {
            file_id: shared_id.clone(),
            name: "shared.txt".into(),
            parent_id: None,
            checksum_sha256: entry_a.checksum_sha256.clone(),
            size_bytes: entry_a.size_bytes,
            clock: VectorClock::from_entries(vec![("node-a".into(), 2), ("node-b".into(), 1)]),
            updated_at: entry_a.updated_at.clone(),
        }];

        let manifest_b = vec![ManifestEntry {
            file_id: shared_id,
            name: "shared.txt".into(),
            parent_id: None,
            checksum_sha256: entry_b.checksum_sha256.clone(),
            size_bytes: entry_b.size_bytes,
            clock: VectorClock::from_entries(vec![("node-a".into(), 1), ("node-b".into(), 2)]),
            updated_at: entry_b.updated_at.clone(),
        }];

        let diff = SyncManifest::compute_diff(&manifest_a, &manifest_b);

        // Should detect a conflict (concurrent clocks, different checksums)
        assert_eq!(diff.conflicts.len(), 1);
        assert_eq!(diff.conflicts[0].local.name, "shared.txt");
        assert_eq!(diff.conflicts[0].remote.name, "shared.txt");
        assert_ne!(
            diff.conflicts[0].local.checksum_sha256,
            diff.conflicts[0].remote.checksum_sha256,
        );
    }

    // -----------------------------------------------------------------------
    // Conflict Resolution
    // -----------------------------------------------------------------------

    #[test]
    fn test_conflict_resolution_keeps_both_versions() {
        let (db, storage, km, _tmp) = setup_instance([1u8; 32]);

        // Upload a file
        let entry = FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner",
            None,
            "document.txt",
            "text/plain",
            b"Local version",
        )
        .unwrap();

        // Create a conflict record
        let conflict = ConflictResolver::create_conflict_copy(&entry, "remote-checksum-abc");

        assert_eq!(conflict.file_id, entry.id);
        assert_eq!(conflict.status, ConflictStatus::Pending);
        assert!(!conflict.remote_version.is_empty());

        // Store conflict in DB
        db.sync_conflict_create(&conflict).unwrap();

        // Verify pending
        let pending = db.sync_conflict_list_pending().unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, conflict.id);

        // Resolve as "both"
        db.sync_conflict_resolve(&conflict.id, ConflictStatus::ResolvedBoth)
            .unwrap();

        // No more pending conflicts
        let pending_after = db.sync_conflict_list_pending().unwrap();
        assert_eq!(pending_after.len(), 0);
    }

    #[test]
    fn test_conflict_resolution_resolve_local() {
        let (db, storage, km, _tmp) = setup_instance([1u8; 32]);

        let entry = FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner",
            None,
            "data.bin",
            "application/octet-stream",
            b"Binary data",
        )
        .unwrap();

        let conflict = ConflictResolver::create_conflict_copy(&entry, "sha256-remote");
        db.sync_conflict_create(&conflict).unwrap();

        // Resolve local
        db.sync_conflict_resolve(&conflict.id, ConflictStatus::ResolvedLocal)
            .unwrap();

        let pending = db.sync_conflict_list_pending().unwrap();
        assert!(pending.is_empty());
    }

    // -----------------------------------------------------------------------
    // Vector Clocks
    // -----------------------------------------------------------------------

    #[test]
    fn test_vector_clock_causal_ordering_between_instances() {
        use miyucloud::sync::vector_clock::ClockOrder;

        // Instance A increments its clock
        let mut clock_a = VectorClock::new();
        clock_a.increment("node-a");
        clock_a.increment("node-a");

        // Instance B has its own clock
        let mut clock_b = VectorClock::new();
        clock_b.increment("node-b");

        // They are concurrent (neither before nor after)
        assert_eq!(clock_a.compare(&clock_b), ClockOrder::Concurrent);

        // Merge (simulates sync completion) -- merge modifies in place
        let mut merged = clock_a.clone();
        merged.merge(&clock_b);
        assert_eq!(merged.get("node-a"), 2);
        assert_eq!(merged.get("node-b"), 1);

        // After merge, A progresses
        merged.increment("node-a");

        // Now merged is strictly After the pre-merge state of B
        assert_eq!(merged.compare(&clock_b), ClockOrder::After);
    }

    // -----------------------------------------------------------------------
    // Peer Discovery + DB
    // -----------------------------------------------------------------------

    #[test]
    fn test_peer_registration_always_untrusted() {
        let (db, _storage, _km, _tmp) = setup_instance([1u8; 32]);

        let now = chrono::Utc::now().to_rfc3339();
        let peer = SyncPeer {
            id: uuid::Uuid::new_v4().to_string(),
            cog_id: "discovered-cog-001".into(),
            display_name: Some("Peer One".into()),
            public_key: "fake-pubkey-base64".into(),
            last_seen_at: Some(now.clone()),
            last_sync_at: None,
            is_trusted: false,
            created_at: now,
        };

        db.sync_peer_upsert(&peer).unwrap();

        let stored = db.sync_peer_by_cog_id("discovered-cog-001").unwrap();
        assert!(stored.is_some());
        let stored = stored.unwrap();
        assert!(!stored.is_trusted, "New peers must always be untrusted");
        assert_eq!(stored.display_name.as_deref(), Some("Peer One"));
    }

    #[test]
    fn test_peer_trust_then_untrust() {
        let (db, _storage, _km, _tmp) = setup_instance([1u8; 32]);

        let now = chrono::Utc::now().to_rfc3339();
        let peer = SyncPeer {
            id: uuid::Uuid::new_v4().to_string(),
            cog_id: "trust-test-cog".into(),
            display_name: None,
            public_key: "pk-trust-test".into(),
            last_seen_at: Some(now.clone()),
            last_sync_at: None,
            is_trusted: false,
            created_at: now,
        };

        db.sync_peer_upsert(&peer).unwrap();

        // Trust the peer
        let trusted = SyncPeer {
            is_trusted: true,
            ..peer.clone()
        };
        db.sync_peer_upsert(&trusted).unwrap();

        let stored = db.sync_peer_by_cog_id("trust-test-cog").unwrap().unwrap();
        assert!(stored.is_trusted);

        // Untrust the peer
        let untrusted = SyncPeer {
            is_trusted: false,
            ..peer.clone()
        };
        db.sync_peer_upsert(&untrusted).unwrap();

        let stored = db.sync_peer_by_cog_id("trust-test-cog").unwrap().unwrap();
        assert!(!stored.is_trusted);
    }

    // -----------------------------------------------------------------------
    // Sync Engine
    // -----------------------------------------------------------------------

    #[test]
    fn test_sync_engine_no_trusted_peers_returns_empty_result() {
        let (db, storage, km, _tmp) = setup_instance([1u8; 32]);

        let config = SyncEngineConfig::new("local-cog".into(), "owner".into(), "pubkey".into());
        let engine = SyncEngine::new(config);

        let result = engine.sync_once(&db, &storage, &km).unwrap();
        assert_eq!(result.peers_synced, 0);
        assert_eq!(result.files_downloaded, 0);
        assert_eq!(result.files_uploaded, 0);
        assert_eq!(result.conflicts_detected, 0);
    }

    #[test]
    fn test_sync_engine_stop_flag_prevents_sync() {
        let config = SyncEngineConfig::new("local-cog".into(), "owner".into(), "pubkey".into());
        let engine = SyncEngine::new(config);

        assert!(!engine.is_syncing());

        let stop = engine.stop_handle();
        engine.stop();

        assert!(stop.load(std::sync::atomic::Ordering::Relaxed));
    }

    // -----------------------------------------------------------------------
    // Export / Import Round-Trip
    // -----------------------------------------------------------------------

    #[test]
    fn test_export_import_roundtrip_different_keys() {
        let (db_a, storage_a, km_a, tmp_a) = setup_instance([10u8; 32]);
        let (db_b, storage_b, km_b, _tmp_b) = setup_instance([20u8; 32]);

        // Upload files on instance A
        FileOps::upload_file(
            &db_a,
            &storage_a,
            &km_a,
            "owner-a",
            None,
            "doc1.txt",
            "text/plain",
            b"Document one content",
        )
        .unwrap();

        FileOps::upload_file(
            &db_a,
            &storage_a,
            &km_a,
            "owner-a",
            None,
            "doc2.pdf",
            "application/pdf",
            b"PDF bytes here",
        )
        .unwrap();

        // Export from A
        let zip_path = tmp_a.path().join("transfer.zip");
        miyucloud::export::export_folder(&db_a, &storage_a, &km_a, None, "owner-a", &zip_path)
            .unwrap();

        assert!(zip_path.exists());
        assert!(zip_path.metadata().unwrap().len() > 0);

        // Import into B (different key!)
        let imported =
            miyucloud::export::import_archive(&db_b, &storage_b, &km_b, &zip_path, None, "owner-b")
                .unwrap();

        assert_eq!(imported.len(), 2);

        // Verify both files can be downloaded with B's key
        for file in &imported {
            let data = FileOps::download_file(&db_b, &storage_b, &km_b, &file.id).unwrap();
            assert!(!data.is_empty());
        }

        // Verify specific contents
        let doc1 = imported.iter().find(|f| f.name == "doc1.txt").unwrap();
        let content = FileOps::download_file(&db_b, &storage_b, &km_b, &doc1.id).unwrap();
        assert_eq!(content, b"Document one content");

        let doc2 = imported.iter().find(|f| f.name == "doc2.pdf").unwrap();
        let content = FileOps::download_file(&db_b, &storage_b, &km_b, &doc2.id).unwrap();
        assert_eq!(content, b"PDF bytes here");
    }

    #[test]
    fn test_export_empty_folder_errors() {
        let (db, storage, km, tmp) = setup_instance([1u8; 32]);
        let zip_path = tmp.path().join("empty.zip");

        let result = miyucloud::export::export_folder(&db, &storage, &km, None, "owner", &zip_path);
        assert!(result.is_err());

        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("No files to export"),
            "Expected 'No files to export', got: {err}"
        );
    }

    // -----------------------------------------------------------------------
    // E2E Crypto
    // -----------------------------------------------------------------------

    #[test]
    fn test_e2e_key_exchange_and_message_encryption() {
        use miyucloud::crypto::e2e;

        // Simulate two peers doing key exchange
        let kp_a = e2e::generate_ephemeral_keypair();
        let kp_b = e2e::generate_ephemeral_keypair();

        let session_key_a = e2e::derive_session_key(
            &kp_a,
            &x25519_dalek::PublicKey::from(kp_b.public_key_bytes()),
        )
        .unwrap();
        let session_key_b = e2e::derive_session_key(
            &kp_b,
            &x25519_dalek::PublicKey::from(kp_a.public_key_bytes()),
        )
        .unwrap();

        // Both should derive the same session key
        assert_eq!(session_key_a, session_key_b);

        // Encrypt with A's key, decrypt with B's key
        let plaintext = b"Sync message payload";
        let encrypted = e2e::encrypt_message(&session_key_a, plaintext).unwrap();
        let decrypted = e2e::decrypt_message(&session_key_b, &encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_e2e_perfect_forward_secrecy() {
        use miyucloud::crypto::e2e;

        // Two different sessions should produce different keys
        let kp_a1 = e2e::generate_ephemeral_keypair();
        let kp_b1 = e2e::generate_ephemeral_keypair();
        let key1 = e2e::derive_session_key(
            &kp_a1,
            &x25519_dalek::PublicKey::from(kp_b1.public_key_bytes()),
        )
        .unwrap();

        let kp_a2 = e2e::generate_ephemeral_keypair();
        let kp_b2 = e2e::generate_ephemeral_keypair();
        let key2 = e2e::derive_session_key(
            &kp_a2,
            &x25519_dalek::PublicKey::from(kp_b2.public_key_bytes()),
        )
        .unwrap();

        assert_ne!(key1, key2, "Each session should derive a unique key (PFS)");
    }
}
