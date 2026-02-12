//! Module données JayShop — types domaine et persistance.
//!
//! Délégation exclusive au service KindMother.
//! KindMother est le seul interlocuteur de la base de données.
//!
//! @id: jayshop_data_module
//! @do: expose_data_types_and_db
//! @layer: infra

mod types;
mod kindmother_client_db;

pub use types::*;

pub use kindmother_client_db::{DbError, JayShopDb};
