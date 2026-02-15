//! Données MiyukiniWatch : types et persistance KindMother.

mod types;
mod db;

pub use types::{MetricRecord, Prefs, TimeSlot, AuditEvent};
pub use db::MiyukiniWatchDb;
