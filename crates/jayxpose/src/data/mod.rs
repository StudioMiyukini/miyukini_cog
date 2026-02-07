//! Module données JayXpose — types domaine et persistance SQLite (KindMother Daughter).
//!
//! @id: jayxpose_data_module
//! @do: expose_data_types_and_db
//! @layer: infra

mod types;
mod kindmother_db;

pub use types::*;
pub use kindmother_db::{JayXposeDb, DbError};
