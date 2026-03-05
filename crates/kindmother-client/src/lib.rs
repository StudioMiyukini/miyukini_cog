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
//! │  Opérateur (ex: JayXpose)           │
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
//! let client = KindMotherClient::connect("127.0.0.1:50051", "jayxpose", "jayxpose").await?;
//!
//! // Lecture
//! let products = client.query("SELECT * FROM products WHERE active = ?", vec!["1"]).await?;
//!
//! // Écriture
//! client.execute(
//!     "INSERT INTO products (name, price) VALUES (?, ?)",
//!     vec!["Widget", "29.99"],
//!     "create_product"
//! ).await?;
//! ```

mod client;
mod errors;
mod protocol;

pub use client::KindMotherClient;
pub use errors::ClientError;
pub use protocol::{Operation, Request, Response};
