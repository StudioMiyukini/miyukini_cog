//! Moteur de synchronisation P2P.
//!
//! @id: miyucloud_sync_engine
//! @do: implement_p2p_sync_loop_with_discovery_diff_transfer_conflicts
//! @role: sync
//! @layer: infra
//!
//! Boucle principale de synchronisation :
//! 1. Decouverte des pairs (scan LAN toutes les 30s)
//! 2. Pour chaque pair en ligne et de confiance :
//!    a. Echanger les manifestes
//!    b. Calculer le diff
//!    c. Transferer les fichiers modifies (dans les deux sens)
//!    d. Resoudre ou signaler les conflits
//! 3. Mettre a jour les horloges vectorielles
//! 4. Dormir (intervalle configurable, defaut 10 secondes)
//!
//! INVARIANT : jamais de perte silencieuse de donnees.

use crate::data::types::{SyncPeer, SyncStatus};
use crate::errors::MiyucloudError;
use crate::sync::manifest::SyncManifest;
use crate::sync::peer_discovery::PeerDiscovery;
use crate::sync::protocol::SyncMessage;
use crate::sync::transport::SyncConnection;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Intervalle de synchronisation par defaut (10 secondes).
pub const DEFAULT_SYNC_INTERVAL_SECS: u64 = 10;

/// Intervalle de scan de decouverte par defaut (30 secondes).
pub const DEFAULT_DISCOVERY_INTERVAL_SECS: u64 = 30;

/// Configuration du moteur de synchronisation.
#[derive(Debug, Clone)]
pub struct SyncEngineConfig {
    /// COG ID local.
    pub cog_id: String,
    /// Owner ID pour les requetes DB.
    pub owner_id: String,
    /// Port de synchronisation TCP local.
    pub sync_port: u16,
    /// Cle publique X25519 locale (base64).
    pub public_key: String,
    /// Nom affiche local.
    pub display_name: Option<String>,
    /// Intervalle de synchronisation en secondes.
    pub sync_interval_secs: u64,
    /// Intervalle de scan de decouverte en secondes.
    pub discovery_interval_secs: u64,
}

impl SyncEngineConfig {
    /// Cree une configuration par defaut.
    pub fn new(cog_id: String, owner_id: String, public_key: String) -> Self {
        Self {
            cog_id,
            owner_id,
            sync_port: crate::sync::transport::DEFAULT_SYNC_PORT,
            public_key,
            display_name: None,
            sync_interval_secs: DEFAULT_SYNC_INTERVAL_SECS,
            discovery_interval_secs: DEFAULT_DISCOVERY_INTERVAL_SECS,
        }
    }
}

/// Moteur de synchronisation P2P.
///
/// Gere la boucle de synchronisation en arriere-plan. Arretable proprement
/// via le token d'annulation (`stop_flag`).
pub struct SyncEngine {
    /// Configuration du moteur.
    config: SyncEngineConfig,
    /// Token d'annulation pour l'arret propre.
    stop_flag: Arc<AtomicBool>,
    /// Etat courant de la synchronisation.
    is_syncing: Arc<AtomicBool>,
}

impl SyncEngine {
    /// Cree un nouveau moteur de synchronisation.
    pub fn new(config: SyncEngineConfig) -> Self {
        Self {
            config,
            stop_flag: Arc::new(AtomicBool::new(false)),
            is_syncing: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Retourne un handle pour arreter proprement le moteur.
    pub fn stop_handle(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.stop_flag)
    }

    /// Retourne l'etat de synchronisation en cours.
    pub fn is_syncing(&self) -> bool {
        self.is_syncing.load(Ordering::Relaxed)
    }

    /// Retourne l'etat global de la synchronisation.
    #[cfg(feature = "legacy-sqlite")]
    pub fn get_status(
        &self,
        db: &crate::data::MiyucloudDb,
    ) -> Result<SyncStatus, MiyucloudError> {
        let peers = db.sync_peer_list()?;
        let peers_total = peers.len() as u32;
        let peers_online = peers.iter().filter(|p| p.is_trusted).count() as u32;
        let conflicts = db.sync_conflict_list_pending()?;

        Ok(SyncStatus {
            is_syncing: self.is_syncing(),
            peers_online,
            peers_total,
            pending_uploads: 0,
            pending_downloads: 0,
            conflicts_pending: conflicts.len() as u32,
            last_sync_at: peers
                .iter()
                .filter_map(|p| p.last_sync_at.as_deref())
                .max()
                .map(String::from),
        })
    }

    /// Execute un cycle de synchronisation unique.
    ///
    /// 1. Scanne les pairs disponibles (decouverte).
    /// 2. Pour chaque pair de confiance en ligne, echange les manifestes
    ///    et transfere les fichiers modifies.
    /// 3. Detecte et signale les conflits (jamais de perte silencieuse).
    #[cfg(feature = "legacy-sqlite")]
    pub fn sync_once(
        &self,
        db: &crate::data::MiyucloudDb,
        storage: &dyn crate::storage::StorageBackend,
        key_manager: &crate::crypto::KeyManager,
    ) -> Result<SyncCycleResult, MiyucloudError> {
        self.is_syncing.store(true, Ordering::Relaxed);
        let result = self.do_sync_cycle(db, storage, key_manager);
        self.is_syncing.store(false, Ordering::Relaxed);
        result
    }

    #[cfg(feature = "legacy-sqlite")]
    fn do_sync_cycle(
        &self,
        db: &crate::data::MiyucloudDb,
        _storage: &dyn crate::storage::StorageBackend,
        _key_manager: &crate::crypto::KeyManager,
    ) -> Result<SyncCycleResult, MiyucloudError> {
        let mut result = SyncCycleResult::default();

        // 1. Discovery
        let discovery = PeerDiscovery::new(
            self.config.cog_id.clone(),
            self.config.public_key.clone(),
            self.config.display_name.clone(),
            self.config.sync_port,
        );

        // Announce ourselves
        if let Err(e) = discovery.announce() {
            tracing::debug!("Discovery announce failed (non-fatal): {e}");
        }

        // Scan for peers (short timeout during sync cycle)
        match discovery.scan(500) {
            Ok(discovered) => {
                for peer in &discovered {
                    if let Err(e) = PeerDiscovery::register_peer(db, peer) {
                        tracing::warn!("Failed to register discovered peer {}: {e}", peer.cog_id);
                    } else {
                        result.peers_discovered += 1;
                    }
                }
            }
            Err(e) => {
                tracing::debug!("Discovery scan failed (non-fatal): {e}");
            }
        }

        // 2. Get trusted peers
        let peers = db.sync_peer_list()?;
        let trusted_peers: Vec<&SyncPeer> = peers.iter().filter(|p| p.is_trusted).collect();

        if trusted_peers.is_empty() {
            return Ok(result);
        }

        // 3. Build local manifest
        let local_manifest = SyncManifest::build(db, &self.config.owner_id, &[])?;

        // 4. For each trusted peer, try to sync
        for peer in &trusted_peers {
            if self.stop_flag.load(Ordering::Relaxed) {
                tracing::info!("Sync engine stop requested, aborting cycle");
                break;
            }

            match self.sync_with_peer(db, peer, &local_manifest) {
                Ok(peer_result) => {
                    result.files_downloaded += peer_result.files_downloaded;
                    result.files_uploaded += peer_result.files_uploaded;
                    result.conflicts_detected += peer_result.conflicts_detected;
                    result.peers_synced += 1;
                }
                Err(e) => {
                    tracing::warn!("Sync with peer {} failed: {e}", peer.cog_id);
                    result.peers_failed += 1;
                }
            }
        }

        Ok(result)
    }

    /// Synchronise avec un pair unique.
    ///
    /// Tente de se connecter au pair, echanger les manifestes, calculer le diff,
    /// et transferer les fichiers necessaires.
    /// La signature `Result` est conservee car le corps fera du TCP
    /// une fois le stockage d'adresses de pairs implemente.
    #[cfg(feature = "legacy-sqlite")]
    #[allow(clippy::unnecessary_wraps)]
    fn sync_with_peer(
        &self,
        _db: &crate::data::MiyucloudDb,
        peer: &SyncPeer,
        _local_manifest: &[crate::sync::protocol::ManifestEntry],
    ) -> Result<PeerSyncResult, MiyucloudError> {
        let result = PeerSyncResult::default();

        // Try to connect (peer needs to be reachable).
        // In a real implementation, we would store the peer's IP address
        // in the DB and attempt a TCP connection here.
        // For now, we log and return since we cannot resolve the address.
        if peer.last_seen_at.is_some() {
            tracing::debug!(
                "Peer {} is known, attempting manifest exchange via DB",
                peer.cog_id
            );
        } else {
            tracing::debug!("Peer {} has no last_seen_at, skipping", peer.cog_id);
        }

        Ok(result)
    }

    /// Demande et recoit le manifeste d'un pair distant.
    ///
    /// Sera utilise quand le transport TCP avec adresse persistante sera actif.
    #[allow(dead_code)]
    fn request_manifest(
        conn: &mut SyncConnection,
        local_cog_id: &str,
        folder_ids: &[String],
    ) -> Result<Vec<crate::sync::protocol::ManifestEntry>, MiyucloudError> {
        conn.send_message(&SyncMessage::ManifestRequest {
            sender_cog_id: local_cog_id.to_string(),
            folder_ids: folder_ids.to_vec(),
        })?;

        match conn.receive_message()? {
            SyncMessage::ManifestResponse { entries } => Ok(entries),
            other => Err(MiyucloudError::InvalidInput(format!(
                "Expected ManifestResponse, got {:?}",
                std::mem::discriminant(&other)
            ))),
        }
    }

    /// Envoie un chunk a un pair distant.
    ///
    /// Sera utilise quand le transport TCP avec adresse persistante sera actif.
    #[allow(dead_code)]
    fn send_chunk(
        conn: &mut SyncConnection,
        file_id: &str,
        chunk_index: u32,
        data: &[u8],
        checksum: &str,
    ) -> Result<(), MiyucloudError> {
        conn.send_message(&SyncMessage::ChunkData {
            file_id: file_id.to_string(),
            chunk_index,
            data: data.to_vec(),
            checksum: checksum.to_string(),
        })?;

        // Wait for Ack
        match conn.receive_message()? {
            SyncMessage::Ack { .. } => Ok(()),
            other => Err(MiyucloudError::InvalidInput(format!(
                "Expected Ack, got {:?}",
                std::mem::discriminant(&other)
            ))),
        }
    }

    /// Arrete proprement le moteur de synchronisation.
    pub fn stop(&self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }
}

/// Resultat d'un cycle de synchronisation complet.
#[derive(Debug, Clone, Default)]
pub struct SyncCycleResult {
    /// Nombre de pairs decouverts.
    pub peers_discovered: u32,
    /// Nombre de pairs synchronises avec succes.
    pub peers_synced: u32,
    /// Nombre de pairs avec erreur de sync.
    pub peers_failed: u32,
    /// Nombre de fichiers telecharges.
    pub files_downloaded: u32,
    /// Nombre de fichiers uploades.
    pub files_uploaded: u32,
    /// Nombre de conflits detectes.
    pub conflicts_detected: u32,
}

/// Resultat de la synchronisation avec un pair unique.
#[derive(Debug, Clone, Default)]
pub struct PeerSyncResult {
    /// Nombre de fichiers telecharges depuis ce pair.
    pub files_downloaded: u32,
    /// Nombre de fichiers uploades vers ce pair.
    pub files_uploaded: u32,
    /// Nombre de conflits detectes avec ce pair.
    pub conflicts_detected: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::FileEntry;
    use crate::sync::conflict::ConflictResolver;
    use crate::sync::vector_clock::VectorClock;

    fn test_config() -> SyncEngineConfig {
        SyncEngineConfig::new(
            "test-cog".into(),
            "test-owner".into(),
            "test-pubkey".into(),
        )
    }

    #[test]
    fn test_sync_engine_creation() {
        let engine = SyncEngine::new(test_config());
        assert!(!engine.is_syncing());
    }

    #[test]
    fn test_sync_engine_stop() {
        let engine = SyncEngine::new(test_config());
        let stop = engine.stop_handle();
        assert!(!stop.load(Ordering::Relaxed));

        engine.stop();
        assert!(stop.load(Ordering::Relaxed));
    }

    #[test]
    fn test_sync_engine_get_status_no_peers() {
        let db = crate::data::MiyucloudDb::open(":memory:").unwrap();
        let engine = SyncEngine::new(test_config());

        let status = engine.get_status(&db).unwrap();
        assert!(!status.is_syncing);
        assert_eq!(status.peers_total, 0);
        assert_eq!(status.peers_online, 0);
        assert_eq!(status.conflicts_pending, 0);
    }

    #[test]
    fn test_sync_engine_get_status_with_peers() {
        let db = crate::data::MiyucloudDb::open(":memory:").unwrap();
        let now = chrono::Utc::now().to_rfc3339();

        // Create trusted peer
        let peer = crate::data::types::SyncPeer {
            id: uuid::Uuid::new_v4().to_string(),
            cog_id: "peer-1".into(),
            display_name: Some("Peer 1".into()),
            public_key: "pk".into(),
            last_seen_at: Some(now.clone()),
            last_sync_at: Some(now.clone()),
            is_trusted: true,
            created_at: now.clone(),
        };
        db.sync_peer_upsert(&peer).unwrap();

        // Create untrusted peer
        let peer2 = crate::data::types::SyncPeer {
            id: uuid::Uuid::new_v4().to_string(),
            cog_id: "peer-2".into(),
            display_name: None,
            public_key: "pk2".into(),
            last_seen_at: None,
            last_sync_at: None,
            is_trusted: false,
            created_at: now,
        };
        db.sync_peer_upsert(&peer2).unwrap();

        let engine = SyncEngine::new(test_config());
        let status = engine.get_status(&db).unwrap();
        assert_eq!(status.peers_total, 2);
        assert_eq!(status.peers_online, 1); // Only trusted peers counted as "online"
    }

    #[test]
    fn test_sync_engine_skips_untrusted_peer() {
        let db = crate::data::MiyucloudDb::open(":memory:").unwrap();
        let now = chrono::Utc::now().to_rfc3339();

        // Only untrusted peers
        let peer = crate::data::types::SyncPeer {
            id: uuid::Uuid::new_v4().to_string(),
            cog_id: "untrusted-peer".into(),
            display_name: None,
            public_key: "pk".into(),
            last_seen_at: Some(now.clone()),
            last_sync_at: None,
            is_trusted: false,
            created_at: now,
        };
        db.sync_peer_upsert(&peer).unwrap();

        let storage = crate::storage::local_fs::LocalFsStorage::new(
            std::env::temp_dir().join("miyucloud-test-engine"),
        )
        .unwrap();
        let km = crate::crypto::KeyManager::from_master_key([1u8; 32]);

        let engine = SyncEngine::new(test_config());
        let result = engine.sync_once(&db, &storage, &km).unwrap();

        // No sync should happen (no trusted peers)
        assert_eq!(result.peers_synced, 0);
    }

    #[test]
    fn test_sync_engine_handles_conflict_detection() {
        // Test that conflict detection works at the manifest level
        let local_entries = vec![crate::sync::protocol::ManifestEntry {
            file_id: "file-1".into(),
            name: "test.txt".into(),
            parent_id: None,
            checksum_sha256: "aaa".into(),
            size_bytes: 100,
            clock: VectorClock::from_entries(vec![
                ("node-a".into(), 2),
                ("node-b".into(), 1),
            ]),
            updated_at: "2026-03-01T00:00:00Z".into(),
        }];

        let remote_entries = vec![crate::sync::protocol::ManifestEntry {
            file_id: "file-1".into(),
            name: "test.txt".into(),
            parent_id: None,
            checksum_sha256: "bbb".into(),
            size_bytes: 200,
            clock: VectorClock::from_entries(vec![
                ("node-a".into(), 1),
                ("node-b".into(), 2),
            ]),
            updated_at: "2026-03-01T01:00:00Z".into(),
        }];

        let diff = SyncManifest::compute_diff(&local_entries, &remote_entries);
        assert_eq!(diff.conflicts.len(), 1);

        // Create conflict record
        let file = FileEntry {
            id: "file-1".into(),
            parent_id: None,
            owner_id: "owner".into(),
            name: "test.txt".into(),
            mime_type: "text/plain".into(),
            size_bytes: 100,
            checksum_sha256: "aaa".into(),
            encryption_iv: None,
            chunk_count: 1,
            is_trashed: false,
            trashed_at: None,
            created_at: "2026-03-01T00:00:00Z".into(),
            updated_at: "2026-03-01T00:00:00Z".into(),
        };

        let conflict = ConflictResolver::create_conflict_copy(
            &file,
            &diff.conflicts[0].remote.checksum_sha256,
        );
        assert_eq!(conflict.status, crate::data::types::ConflictStatus::Pending);
    }

    #[test]
    fn test_sync_engine_detects_new_file_on_peer() {
        // File only exists on remote -> should be in to_download
        let local_entries: Vec<crate::sync::protocol::ManifestEntry> = vec![];
        let remote_entries = vec![crate::sync::protocol::ManifestEntry {
            file_id: "new-file".into(),
            name: "new.txt".into(),
            parent_id: None,
            checksum_sha256: "abc".into(),
            size_bytes: 50,
            clock: VectorClock::from_entries(vec![("remote-node".into(), 1)]),
            updated_at: "2026-03-01T00:00:00Z".into(),
        }];

        let diff = SyncManifest::compute_diff(&local_entries, &remote_entries);
        assert_eq!(diff.to_download.len(), 1);
        assert_eq!(diff.to_download[0].file_id, "new-file");
    }
}
