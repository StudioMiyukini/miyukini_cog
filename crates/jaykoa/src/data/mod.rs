//! Persistance JayKoa via KindMother Service.
//!
//! @id: jaykoa_data_module
//! @do: expose_domain_types_and_persistence
//! @layer: domain
//!
//! Full KM : délégation exclusive au service KindMother (kindmother-client).

/// Types domaine JayKoa (Agenda, TemporalEntry, TemporalConflict, etc.).
pub mod types;

mod kindmother_client_db;

pub use types::{
    Agenda, CalendarType, EntryType, EventSource, TemporalConflict, TemporalEntry,
    TemporalStatus, UserSettings,
};

pub use kindmother_client_db::{DbError, JayKoaDb};
