//! Decouverte de pairs P2P via broadcast UDP LAN.
//!
//! @id: miyucloud_sync_peer_discovery
//! @do: discover_and_register_p2p_peers_via_lan_broadcast
//! @role: sync
//! @layer: infra
//!
//! Implementation simplifiee de la decouverte de pairs via broadcast UDP
//! sur le LAN (port 11443). Compatible Windows, Linux et macOS.
//!
//! Un pair non approuve manuellement n'est JAMAIS synchronise
//! (`is_trusted = false` par defaut).

use crate::data::types::SyncPeer;
use crate::errors::MiyucloudError;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Port par defaut pour la decouverte de pairs.
pub const DISCOVERY_PORT: u16 = 11443;

/// Intervalle de scan par defaut en secondes.
pub const SCAN_INTERVAL_SECS: u64 = 30;

/// Pair decouvert sur le reseau LAN.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredPeer {
    /// COG ID du pair.
    pub cog_id: String,
    /// Cle publique X25519 (base64).
    pub public_key: String,
    /// Nom affiche.
    pub display_name: Option<String>,
    /// Port de synchronisation.
    pub sync_port: u16,
    /// Version du protocole.
    pub version: String,
    /// Adresse IP du pair (deduite du paquet UDP).
    pub address: String,
}

/// Annonce diffusee par un pair sur le reseau.
///
/// Ce message est envoye en broadcast UDP sur le LAN.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerAnnounce {
    /// COG ID du pair.
    pub cog_id: String,
    /// Cle publique X25519 (base64).
    pub public_key: String,
    /// Nom affiche.
    pub display_name: Option<String>,
    /// Port de synchronisation TCP.
    pub sync_port: u16,
    /// Version du protocole.
    pub version: String,
}

/// Moteur de decouverte de pairs P2P.
///
/// Utilise le broadcast UDP sur le LAN pour annoncer la presence
/// et decouvrir d'autres instances MiyuCloud.
pub struct PeerDiscovery {
    /// COG ID local.
    cog_id: String,
    /// Cle publique X25519 locale (base64).
    public_key: String,
    /// Nom affiche local.
    display_name: Option<String>,
    /// Port de synchronisation TCP local.
    sync_port: u16,
    /// Port de decouverte UDP.
    discovery_port: u16,
}

impl PeerDiscovery {
    /// Cree un nouveau moteur de decouverte.
    pub fn new(
        cog_id: String,
        public_key: String,
        display_name: Option<String>,
        sync_port: u16,
    ) -> Self {
        Self {
            cog_id,
            public_key,
            display_name,
            sync_port,
            discovery_port: DISCOVERY_PORT,
        }
    }

    /// Cree un nouveau moteur avec un port de decouverte personnalise.
    pub fn with_discovery_port(mut self, port: u16) -> Self {
        self.discovery_port = port;
        self
    }

    /// Construit le message d'annonce.
    pub fn build_announce(&self) -> PeerAnnounce {
        PeerAnnounce {
            cog_id: self.cog_id.clone(),
            public_key: self.public_key.clone(),
            display_name: self.display_name.clone(),
            sync_port: self.sync_port,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Envoie une annonce en broadcast UDP sur le LAN.
    ///
    /// Le paquet est envoye a l'adresse broadcast `255.255.255.255:{port}`.
    pub fn announce(&self) -> Result<(), MiyucloudError> {
        let socket = std::net::UdpSocket::bind("0.0.0.0:0").map_err(MiyucloudError::Io)?;
        socket.set_broadcast(true).map_err(MiyucloudError::Io)?;

        let announce = self.build_announce();
        let data = serde_json::to_vec(&announce)
            .map_err(|e| MiyucloudError::Serialization(e.to_string()))?;

        let broadcast_addr: SocketAddr = format!("255.255.255.255:{}", self.discovery_port)
            .parse()
            .map_err(|e: std::net::AddrParseError| MiyucloudError::InvalidInput(e.to_string()))?;

        socket
            .send_to(&data, broadcast_addr)
            .map_err(MiyucloudError::Io)?;

        Ok(())
    }

    /// Scanne le reseau LAN pour decouvrir des pairs.
    ///
    /// Ecoute pendant `timeout_ms` millisecondes sur le port de decouverte UDP
    /// et retourne les pairs decouverts (en excluant soi-meme).
    pub fn scan(&self, timeout_ms: u64) -> Result<Vec<DiscoveredPeer>, MiyucloudError> {
        let bind_addr = format!("0.0.0.0:{}", self.discovery_port);
        let socket = match std::net::UdpSocket::bind(&bind_addr) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!("Could not bind discovery socket on {bind_addr}: {e}");
                return Ok(Vec::new());
            }
        };

        socket
            .set_read_timeout(Some(std::time::Duration::from_millis(timeout_ms)))
            .map_err(MiyucloudError::Io)?;

        let mut peers = Vec::new();
        let mut buf = vec![0u8; 4096];

        loop {
            match socket.recv_from(&mut buf) {
                Ok((n, addr)) => {
                    if let Ok(announce) = serde_json::from_slice::<PeerAnnounce>(&buf[..n]) {
                        // Skip self
                        if announce.cog_id == self.cog_id {
                            continue;
                        }
                        peers.push(DiscoveredPeer {
                            cog_id: announce.cog_id,
                            public_key: announce.public_key,
                            display_name: announce.display_name,
                            sync_port: announce.sync_port,
                            version: announce.version,
                            address: addr.ip().to_string(),
                        });
                    }
                }
                Err(ref e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::TimedOut =>
                {
                    break;
                }
                Err(e) => {
                    tracing::warn!("Discovery scan recv error: {e}");
                    break;
                }
            }
        }

        Ok(peers)
    }

    /// Enregistre un pair decouvert dans la base de donnees.
    ///
    /// Un nouveau pair est TOUJOURS cree avec `is_trusted = false`.
    /// L'utilisateur doit approuver manuellement le pair via `trust_peer`.
    #[cfg(feature = "legacy-sqlite")]
    pub fn register_peer(
        db: &crate::data::MiyucloudDb,
        peer: &DiscoveredPeer,
    ) -> Result<SyncPeer, MiyucloudError> {
        let now = chrono::Utc::now().to_rfc3339();

        // Check if peer already exists
        if let Some(existing) = db.sync_peer_by_cog_id(&peer.cog_id)? {
            // Update last_seen_at
            let updated = SyncPeer {
                last_seen_at: Some(now),
                public_key: peer.public_key.clone(),
                display_name: peer.display_name.clone(),
                ..existing
            };
            return db.sync_peer_upsert(&updated);
        }

        // Create new untrusted peer
        let sync_peer = SyncPeer {
            id: uuid::Uuid::new_v4().to_string(),
            cog_id: peer.cog_id.clone(),
            display_name: peer.display_name.clone(),
            public_key: peer.public_key.clone(),
            last_seen_at: Some(now.clone()),
            last_sync_at: None,
            is_trusted: false,
            created_at: now,
        };
        db.sync_peer_upsert(&sync_peer)
    }

    /// Marque un pair comme de confiance.
    ///
    /// C'est une action manuelle de l'utilisateur : il doit verifier
    /// le fingerprint du pair avant de l'approuver.
    #[cfg(feature = "legacy-sqlite")]
    pub fn trust_peer(db: &crate::data::MiyucloudDb, peer_id: &str) -> Result<(), MiyucloudError> {
        let peer = db
            .sync_peer_by_cog_id(peer_id)?
            .or_else(|| {
                // Try by ID
                let list = db.sync_peer_list().ok()?;
                list.into_iter().find(|p| p.id == peer_id)
            })
            .ok_or_else(|| MiyucloudError::NotFound(format!("Peer not found: {peer_id}")))?;

        let trusted = SyncPeer {
            is_trusted: true,
            ..peer
        };
        db.sync_peer_upsert(&trusted)?;
        Ok(())
    }
}

/// Parse un enregistrement TXT mDNS en `PeerAnnounce`.
///
/// Format attendu des champs TXT :
/// - `cog_id=<UUID>`
/// - `version=<semver>`
/// - `port=<port_sync>`
/// - `pubkey=<X25519_base64>`
/// - `name=<display_name>` (optionnel)
pub fn parse_mdns_txt(txt_records: &[(String, String)]) -> Option<PeerAnnounce> {
    let mut cog_id = None;
    let mut version = None;
    let mut port = None;
    let mut pubkey = None;
    let mut name = None;

    for (key, value) in txt_records {
        match key.as_str() {
            "cog_id" => cog_id = Some(value.clone()),
            "version" => version = Some(value.clone()),
            "port" => port = value.parse().ok(),
            "pubkey" => pubkey = Some(value.clone()),
            "name" => name = Some(value.clone()),
            _ => {}
        }
    }

    Some(PeerAnnounce {
        cog_id: cog_id?,
        public_key: pubkey?,
        display_name: name,
        sync_port: port?,
        version: version.unwrap_or_else(|| "0.0.0".into()),
    })
}

/// Construit les champs TXT pour une annonce mDNS.
pub fn build_mdns_txt(announce: &PeerAnnounce) -> Vec<(String, String)> {
    let mut records = vec![
        ("cog_id".into(), announce.cog_id.clone()),
        ("version".into(), announce.version.clone()),
        ("port".into(), announce.sync_port.to_string()),
        ("pubkey".into(), announce.public_key.clone()),
    ];
    if let Some(name) = &announce.display_name {
        records.push(("name".into(), name.clone()));
    }
    records
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mdns_txt_record() {
        let records = vec![
            ("cog_id".into(), "abc-123".into()),
            ("version".into(), "0.1.0".into()),
            ("port".into(), "11441".into()),
            ("pubkey".into(), "AAAA==".into()),
            ("name".into(), "My COG".into()),
        ];
        let result = parse_mdns_txt(&records);
        assert!(result.is_some());
        let announce = result.unwrap();
        assert_eq!(announce.cog_id, "abc-123");
        assert_eq!(announce.sync_port, 11441);
        assert_eq!(announce.public_key, "AAAA==");
        assert_eq!(announce.display_name.as_deref(), Some("My COG"));
    }

    #[test]
    fn test_build_mdns_txt_record() {
        let announce = PeerAnnounce {
            cog_id: "abc-123".into(),
            public_key: "BBBB==".into(),
            display_name: Some("Test".into()),
            sync_port: 11441,
            version: "0.1.0".into(),
        };
        let records = build_mdns_txt(&announce);
        assert_eq!(records.len(), 5);
        assert!(records.contains(&("cog_id".into(), "abc-123".into())));
        assert!(records.contains(&("port".into(), "11441".into())));
    }

    #[test]
    fn test_build_mdns_txt_record_no_name() {
        let announce = PeerAnnounce {
            cog_id: "abc".into(),
            public_key: "pk".into(),
            display_name: None,
            sync_port: 11441,
            version: "0.1.0".into(),
        };
        let records = build_mdns_txt(&announce);
        assert_eq!(records.len(), 4);
    }

    #[test]
    fn test_parse_mdns_txt_missing_required() {
        let records = vec![
            ("cog_id".into(), "abc".into()),
            // Missing pubkey and port
        ];
        let result = parse_mdns_txt(&records);
        assert!(result.is_none());
    }

    #[test]
    fn test_peer_already_known_updates_last_seen() {
        let db = crate::data::MiyucloudDb::open(":memory:").unwrap();
        let discovered = DiscoveredPeer {
            cog_id: "cog-remote".into(),
            public_key: "pubkey-base64".into(),
            display_name: Some("Remote COG".into()),
            sync_port: 11441,
            version: "0.1.0".into(),
            address: "192.168.1.10".into(),
        };

        // First registration
        let peer1 = PeerDiscovery::register_peer(&db, &discovered).unwrap();
        assert!(!peer1.is_trusted);
        let first_seen = peer1.last_seen_at.clone();

        // Small delay to ensure different timestamp
        std::thread::sleep(std::time::Duration::from_millis(10));

        // Second registration (update)
        let peer2 = PeerDiscovery::register_peer(&db, &discovered).unwrap();
        assert_eq!(peer2.cog_id, "cog-remote");
        // last_seen_at should be updated
        assert_ne!(peer2.last_seen_at, first_seen);
    }

    #[test]
    fn test_new_peer_is_not_trusted() {
        let db = crate::data::MiyucloudDb::open(":memory:").unwrap();
        let discovered = DiscoveredPeer {
            cog_id: "new-cog".into(),
            public_key: "pk".into(),
            display_name: None,
            sync_port: 11441,
            version: "0.1.0".into(),
            address: "10.0.0.1".into(),
        };
        let peer = PeerDiscovery::register_peer(&db, &discovered).unwrap();
        assert!(!peer.is_trusted);
    }

    #[test]
    fn test_trust_peer() {
        let db = crate::data::MiyucloudDb::open(":memory:").unwrap();
        let discovered = DiscoveredPeer {
            cog_id: "trust-test".into(),
            public_key: "pk".into(),
            display_name: None,
            sync_port: 11441,
            version: "0.1.0".into(),
            address: "10.0.0.2".into(),
        };
        let peer = PeerDiscovery::register_peer(&db, &discovered).unwrap();
        assert!(!peer.is_trusted);

        PeerDiscovery::trust_peer(&db, "trust-test").unwrap();
        let trusted = db.sync_peer_by_cog_id("trust-test").unwrap().unwrap();
        assert!(trusted.is_trusted);
    }

    #[test]
    fn test_build_announce() {
        let discovery = PeerDiscovery::new(
            "my-cog".into(),
            "my-pubkey".into(),
            Some("My COG".into()),
            11441,
        );
        let announce = discovery.build_announce();
        assert_eq!(announce.cog_id, "my-cog");
        assert_eq!(announce.public_key, "my-pubkey");
        assert_eq!(announce.sync_port, 11441);
    }

    #[test]
    fn test_announce_serialize_roundtrip() {
        let announce = PeerAnnounce {
            cog_id: "cog-1".into(),
            public_key: "pk-1".into(),
            display_name: Some("COG 1".into()),
            sync_port: 11441,
            version: "0.1.0".into(),
        };
        let json = serde_json::to_string(&announce).unwrap();
        let restored: PeerAnnounce = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.cog_id, "cog-1");
    }
}
