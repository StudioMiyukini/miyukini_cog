//! # KindMother Client
//!
//! Client de délégation vers le service KindMother.
//!
//! Ce crate fournit une interface simple pour les opérateurs (services COG)
//! afin d'accéder à leurs données via KindMother, le seul interlocuteur autorisé.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │  Opérateur (ex: JayKoa)             │
//! │  ┌─────────────────────────────────┐│
//! │  │  KindMotherClient               ││
//! │  │  - Connexion TCP                ││
//! │  │  - Sérialisation JSON           ││
//! │  │  - Gestion d'erreurs            ││
//! │  └─────────────────────────────────┘│
//! └──────────────┬──────────────────────┘
//!                │ JSON/TCP
//!                ▼
//! ┌─────────────────────────────────────┐
//! │  KindMother Service                 │
//! │  (seul accès à la DB)               │
//! └─────────────────────────────────────┘
//! ```
//!
//! ## Usage
//!
//! ```ignore
//! use kindmother_client::KindMotherClient;
//!
//! let client = KindMotherClient::connect("127.0.0.1:50051", "jaykoa", "jaykoa").await?;
//!
//! // Lecture
//! let entries = client.query("SELECT * FROM entries WHERE active = ?", vec!["1"]).await?;
//!
//! // Écriture
//! client.execute(
//!     "INSERT INTO entries (title, start_at) VALUES (?, ?)",
//!     vec!["Reunion", "2026-05-01T09:00:00"],
//!     "create_entry"
//! ).await?;
//! ```

mod client;
mod errors;
mod protocol;

pub use client::KindMotherClient;
pub use errors::ClientError;
pub use protocol::{Operation, Request, Response};
