//! Persistance Jay1Tribu via KindMother (Instance Daughter).
//!
//! @id: jay1tribu_data_module
//! @do: expose_domain_types_and_persistence
//! @role: data
//! @layer: domain
//!
//! legacy-sqlite : accès SQLite direct. kindmother-only : délégation au service KindMother.

#[cfg(all(feature = "legacy-sqlite", feature = "kindmother-only"))]
compile_error!("Choisir une seule feature de persistance: legacy-sqlite OU kindmother-only.");

pub mod types;

#[cfg(feature = "legacy-sqlite")]
mod kindmother_db;

#[cfg(feature = "kindmother-only")]
mod kindmother_client_db;

pub use types::{
    Friend, FriendWithPresence, Message, Salon, SalonMember, SalonType, Tribe, TribeMember,
    TribeRole,
};

#[cfg(feature = "legacy-sqlite")]
pub use kindmother_db::{DbError, Jay1TribuDb};

#[cfg(feature = "kindmother-only")]
pub use kindmother_client_db::{DbError, Jay1TribuDb};
