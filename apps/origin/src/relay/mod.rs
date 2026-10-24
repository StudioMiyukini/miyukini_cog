//! @id     origin_relay_mod
//! @do     declarer_module_metrics
//! @role   orchestration
//! @layer  app
//! @human  origin — module « relay mod ».

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

pub mod identites;
pub mod metrics;
pub mod permis_registry;
pub mod rate_limiter;
pub mod server;
pub mod session;
pub mod tunnel;
pub mod verification;

pub use identites::RegistreIdentites;
pub use permis_registry::PermisRegistry;
pub use server::RelayServer;
