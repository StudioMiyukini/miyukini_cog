//! Serveur Relay MWS Origin.
//!
//! Gère les connexions des COGs, la vérification de conformité,
//! et la délivrance des Permis de circulation.
//!
//! # Composants
//!
//! - `server` — Serveur TCP+TLS principal
//! - `session` — Gestion des sessions de connexion
//! - `verification` — Vérification de conformité en 3 phases
//! - `rate_limiter` — Protection contre les abus
//! - `tunnel` — Tunnels de données entre COGs
//! - `metrics` — Métriques et statistiques

pub mod metrics;
pub mod rate_limiter;
pub mod server;
pub mod session;
pub mod tunnel;
pub mod verification;

pub use server::RelayServer;
