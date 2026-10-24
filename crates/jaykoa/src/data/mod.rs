//! Persistance JayKoa via KindMother Service.
//!
//! @id     jaykoa_data_module
//! @do     expose_domain_types_and_persistence
//@human ! @layer  domain
//!
//! Full KM : délégation exclusive au service KindMother (kindmother-client).

/// Types domaine JayKoa (Agenda, TemporalEntry, TemporalConflict, etc.).
pub mod types;

mod kindmother_client_db;
mod schema_kindmother;

/// Import de la base SQLite héritée (K-13), feature `import-heritage`.
#[cfg(feature = "import-heritage")]
pub mod import_heritage;

pub use types::{
    Agenda, CalendarType, EntryType, EventSource, TemporalConflict, TemporalEntry, TemporalStatus,
    UserSettings,
};

pub use kindmother_client_db::{DbError, JayKoaDb};
