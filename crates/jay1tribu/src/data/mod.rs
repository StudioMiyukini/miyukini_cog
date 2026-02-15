//! Persistance Jay1Tribu via KindMother (Instance Daughter).
//! En mode legacy-sqlite : accès SQLite direct. Archives locales uniquement.

pub mod types;

#[cfg(feature = "legacy-sqlite")]
mod kindmother_db;

pub use types::{
    Friend, FriendWithPresence, Message, Salon, SalonMember, SalonType, Tribe, TribeMember,
    TribeRole,
};

#[cfg(feature = "legacy-sqlite")]
pub use kindmother_db::{DbError, Jay1TribuDb};
