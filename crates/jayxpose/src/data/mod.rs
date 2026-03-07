//! Module données JayXpose — types domaine et persistance.
//!
//! @id: jayxpose_data_module
//! @do: expose_data_types_and_db
//! @layer: infra
//! Par défaut : legacy-sqlite. Full KM (kindmother-only) en cours de parité API.

mod types;
pub mod upload_validation;

#[cfg(feature = "legacy-sqlite")]
mod kindmother_db;

#[cfg(feature = "kindmother-only")]
mod kindmother_client_db;

pub use types::*;

#[cfg(feature = "legacy-sqlite")]
pub use kindmother_db::{DbError, JayXposeDb};

#[cfg(feature = "kindmother-only")]
pub use kindmother_client_db::{DbError, JayXposeDb};
