//! Couche persistance JayManga.
//!
//! Feature flags :
//! - `legacy-sqlite` (défaut) : accès SQLite direct via rusqlite.
//! - `kindmother-only` : accès via KindMother Client (IPC/gRPC).

pub mod types;
pub mod types_payment;
pub mod types_reader;
pub mod types_aggregator;
pub mod types_federation;

#[cfg(feature = "legacy-sqlite")]
mod kindmother_db;

#[cfg(feature = "kindmother-only")]
mod kindmother_client_db;

pub use types::*;
pub use types_payment::*;
pub use types_reader::*;
pub use types_aggregator::*;
pub use types_federation::*;

#[cfg(feature = "legacy-sqlite")]
pub use kindmother_db::{DbError, JayMangaDb};

#[cfg(feature = "kindmother-only")]
pub use kindmother_client_db::{DbError, JayMangaDb};
