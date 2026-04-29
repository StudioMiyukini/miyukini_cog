//! # KindMother Service
//!
//! Serveur de persistance SQLite isolé.
//!
//! Ce service est le **seul** interlocuteur autorisé pour accéder aux bases de données.
//! Il expose une API JSON/TCP que les opérateurs utilisent via `kindmother-client`.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────┐    JSON/TCP     ┌───────────────────────┐
//! │  Opérateur      │◄───────────────►│  KindMother Service   │
//! │  (jaykoa...)    │   localhost     │  ┌─────────────────┐  │
//! │                 │                 │  │ Arbitration     │  │
//! │  kindmother-    │                 │  │ Engine          │  │
//! │  client         │                 │  └────────┬────────┘  │
//! └─────────────────┘                 │           │           │
//!                                     │  ┌────────▼────────┐  │
//!                                     │  │ SQLite          │  │
//!                                     │  │ (isolé)         │  │
//!                                     │  └─────────────────┘  │
//!                                     └───────────────────────┘
//! ```
//!
//! ## Sécurité
//!
//! - Processus isolé - aucun accès direct possible aux fichiers DB
//! - Contrôle d'accès par opérateur (arbitrage)
//! - Audit complet des opérations
//! - Écoute uniquement sur localhost

mod arbitration;
mod database;
mod errors;
mod protocol;
mod server;

pub use arbitration::{ArbitrationEngine, Permission};
pub use database::EncryptedDatabase;
pub use errors::ServiceError;
#[cfg(feature = "db-encryption")]
pub use kindmother_db_key::KeyDerivation;
pub use protocol::{Request, Response};
pub use server::KindMotherServer;
