//! Persistance JayKoa via KindMother Service.
//!
//! @id: jaykoa_data_module
//! @do: expose_domain_types_and_persistence
//! @layer: domain
//!
//! En mode `legacy-sqlite`, accès direct SQLite (migration progressive).
//! En mode `kindmother-only`, délégation exclusive au service KindMother.

/// Types domaine JayKoa (Agenda, TemporalEntry, TemporalConflict, etc.).
pub mod types;

// Mode legacy SQLite (migration progressive)
#[cfg(feature = "legacy-sqlite")]
mod kindmother_db;

// Mode KindMother client (production)
#[cfg(feature = "kindmother-only")]
mod kindmother_client_db;

pub use types::{
    Agenda, CalendarType, EntryType, EventSource, TemporalConflict, TemporalEntry,
    TemporalStatus, UserSettings,
};

// Export de l'implémentation selon le feature flag
#[cfg(feature = "legacy-sqlite")]
pub use kindmother_db::{DbError, JayKoaDb};

#[cfg(feature = "kindmother-only")]
pub use kindmother_client_db::{DbError, JayKoaDb};
