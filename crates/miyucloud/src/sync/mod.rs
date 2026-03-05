//! Synchronisation P2P chiffree E2E pour MiyuCloud.
//!
//! @id: miyucloud_sync_module
//! @do: expose_p2p_sync_engine_clocks_discovery_transport
//! @role: sync
//! @layer: infra
//!
//! Fournit les horloges vectorielles, la detection/resolution de conflits,
//! le protocole de messages, la construction de manifeste, la decouverte
//! de pairs (mDNS/broadcast UDP), le transport TCP chiffre E2E et le moteur
//! de synchronisation.

pub mod conflict;
pub mod engine;
pub mod manifest;
pub mod peer_discovery;
pub mod protocol;
pub mod transport;
pub mod vector_clock;

pub use conflict::ConflictResolver;
pub use engine::SyncEngine;
pub use manifest::{SyncDiff, SyncManifest};
pub use peer_discovery::PeerDiscovery;
pub use protocol::{ManifestEntry, SyncMessage};
pub use transport::SyncTransport;
pub use vector_clock::{ClockOrder, VectorClock};
