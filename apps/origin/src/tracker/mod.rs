//! Serveur Tracker MWS Origin.
//!
//! Gère les pools de COGs, le catalogue et les connexions.
//!
//! # Composants
//!
//! - `server` — Serveur TCP principal
//! - `protocol` — Protocole binaire de communication
//! - `pool` — Gestion des pools de COGs par version
//! - `catalog` — Catalogue des services et lobbys
//! - `metrics` — Statistiques et métriques

pub mod catalog;
pub mod metrics;
pub mod pool;
pub mod protocol;
pub mod server;

pub use server::TrackerServer;
