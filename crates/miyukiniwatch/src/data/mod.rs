//! @id     miyukiniwatch_data_mod
//! @do     declarer_module_db
//! @role   donnee
//! @layer  core
//! @human  miyukiniwatch — module « data mod ».

//! Données MiyukiniWatch : types et persistance KindMother.

mod db;
mod types;

pub use db::MiyukiniWatchDb;
pub use types::{AuditEvent, MetricRecord, Prefs, TimeSlot};
