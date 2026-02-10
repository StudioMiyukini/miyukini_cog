//! Module données JayXpose — types domaine et persistance.
//!
//! Ce module fournit deux implémentations de la couche de persistance :
//! 
//! - **legacy-sqlite** (par défaut) : Accès SQLite direct via `rusqlite`.
//!   À utiliser uniquement pendant la migration.
//! 
//! - **kindmother-only** : Accès exclusif via `kindmother-client`.
//!   KindMother est le seul interlocuteur de la base de données.
//!   C'est le mode production recommandé.
//!
//! @id: jayxpose_data_module
//! @do: expose_data_types_and_db
//! @layer: infra

mod types;

// Mode legacy SQLite (migration progressive)
#[cfg(feature = "legacy-sqlite")]
mod kindmother_db;

// Mode KindMother client (production)
#[cfg(feature = "kindmother-only")]
mod kindmother_client_db;

pub use types::*;

// Export de l'implémentation selon le feature flag
#[cfg(feature = "legacy-sqlite")]
pub use kindmother_db::{JayXposeDb, DbError};

#[cfg(feature = "kindmother-only")]
pub use kindmother_client_db::{JayXposeDb, DbError};
