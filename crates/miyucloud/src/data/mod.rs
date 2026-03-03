//! Persistance MiyuCloud via KindMother (Instance Daughter).
//!
//! @id: miyucloud_data_module
//! @do: expose_domain_types_and_persistence
//! @role: data
//! @layer: domain
//!
//! `legacy-sqlite` : acces SQLite direct.
//! `kindmother-only` : delegation au service KindMother.

#[cfg(all(feature = "legacy-sqlite", feature = "kindmother-only"))]
compile_error!("Choisir une seule feature de persistance: legacy-sqlite OU kindmother-only.");

pub mod types;

#[cfg(feature = "legacy-sqlite")]
pub mod kindmother_db;

pub use types::*;

#[cfg(feature = "legacy-sqlite")]
pub use kindmother_db::MiyucloudDb;
