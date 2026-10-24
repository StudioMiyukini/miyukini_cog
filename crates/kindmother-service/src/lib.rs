//! @id     kindmother_service_lib
//! @do     declarer_module_arbitration
//! @role   orchestration
//! @layer  core
//! @human  kindmother-service — module « lib ».

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
//! - Authentification mutuelle Ed25519 (K-04, `kindmother-auth-v2`) : le serveur prouve
//!   son identité épinglée (`server.pub`), l'opérateur signe une transcription liée aux
//!   deux nonces et à la clé du serveur ; l'identité est liée à la connexion, jamais lue
//!   dans la requête (C-20)
//! - Contrôle d'accès par opérateur (arbitrage) et politique SQL appliquée par
//!   l'autorisateur SQLite (K-05)
//! - Audit des écritures
//! - Écoute uniquement sur la boucle locale (sauf configuration explicite), trames
//!   bornées, poignées de main non authentifiées bornées à part (éviction des plus
//!   anciennes), emplacement principal pris après authentification, délai d'inactivité

mod arbitration;
mod credentials;
mod database;
mod errors;
mod protocol;
mod server;
mod sql_guard;

pub use arbitration::{AccessMode, ArbitrationEngine, OperatorConfig, Permission};
pub use credentials::{
    client_transcript, server_transcript, CredentialStore, ServerIdentity, AUTH_PROTOCOL,
    NONCE_LEN, PUBLIC_KEY_LEN, SERVER_KEY_NAME,
};
pub use sql_guard::SqlPolicy;
pub use database::EncryptedDatabase;
pub use errors::ServiceError;
#[cfg(feature = "db-encryption")]
pub use kindmother_db_key::KeyDerivation;
pub use protocol::{Request, Response};
pub use server::{KindMotherServer, ServerLimits};
