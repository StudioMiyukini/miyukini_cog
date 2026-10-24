//! Persistance Jay1Tribu via KindMother (Instance Daughter).
//!
//! @id     jay1tribu_data_module
//! @do     expose_domain_types_and_persistence
//@human ! @role   data
//! @layer  domain
//!
//! kindmother-only (défaut, K-13) : délégation au service KindMother (intentions typées).
//! legacy-sqlite (HÉRITÉ, transitoire) : accès SQLite direct.
//!
//! Règle de priorité : si `legacy-sqlite` est activée explicitement par un dépendant
//! (ex. jayclub, qui garde aussi les features par défaut), le chemin SQLite HÉRITÉ
//! l'emporte jusqu'à la migration de ce dépendant ; le chemin KindMother n'est alors
//! pas compilé.

#[cfg(not(any(feature = "kindmother-only", feature = "legacy-sqlite")))]
compile_error!("Activer une feature de persistance : kindmother-only (défaut) ou legacy-sqlite.");

pub mod types;

#[cfg(feature = "legacy-sqlite")]
mod kindmother_db;

#[cfg(all(feature = "kindmother-only", not(feature = "legacy-sqlite")))]
mod kindmother_client_db;

#[cfg(any(all(feature = "kindmother-only", not(feature = "legacy-sqlite")), feature = "import-heritage"))]
mod schema_kindmother;

#[cfg(feature = "import-heritage")]
pub mod import_heritage;

pub use types::{
    Friend, FriendWithPresence, Message, Salon, SalonMember, SalonType, Tribe, TribeMember,
    TribeRole,
};

#[cfg(feature = "legacy-sqlite")]
pub use kindmother_db::{DbError, Jay1TribuDb};

#[cfg(all(feature = "kindmother-only", not(feature = "legacy-sqlite")))]
pub use kindmother_client_db::{DbError, Jay1TribuDb};
