//! Module données JayKoa : types domaine et base fille SQLite via KindMother.
//!
//! @id: jaykoa_data_module
//! @do: expose_domain_types_and_persistence
//! @layer: domain
//! @human: KindMother Daughter (SQLite locale), modèle métier calendrier.

mod kindmother_db;
/// Types domaine JayKoa (Agenda, TemporalEntry, TemporalConflict, etc.).
pub mod types;

pub use kindmother_db::{DbError, JayKoaDb};
pub use types::{
    Agenda, CalendarType, EntryType, EventSource, TemporalConflict, TemporalEntry,
    TemporalStatus, UserSettings,
};
