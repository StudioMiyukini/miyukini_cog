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
//! │  (jayxpose...)  │   localhost     │  ┌─────────────────┐  │
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

mod encryption;
mod database;
mod arbitration;
mod server;
mod protocol;
mod errors;

pub use encryption::KeyDerivation;
pub use database::EncryptedDatabase;
pub use arbitration::{ArbitrationEngine, Permission};
pub use server::KindMotherServer;
pub use protocol::{Request, Response};
pub use errors::ServiceError;
