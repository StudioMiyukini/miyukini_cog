//! Couche données JayRDV — types et store.
//!
//! @id     jayrdv_data
//! @do     reexport_data_types_and_store
//@human ! @role   data
//! @layer  domain
//!
//! Par défaut (feature `kindmother-only`) : store KindMother (persistance réelle, intentions typées).
//! Feature `memory` (prioritaire si activée explicitement) ou aucune feature : store mémoire.
//! Le store mémoire reste toujours accessible sous le nom `MemoryJayRdvStore`.

#![allow(clippy::doc_markdown)] // En-têtes MSCM.

mod memory_store;
mod types;

#[cfg(all(feature = "kindmother-only", not(feature = "memory")))]
mod kindmother_client_db;

#[cfg(all(test, feature = "kindmother-only", not(feature = "memory")))]
pub(crate) use kindmother_client_db::test_support as kindmother_test_support;
#[cfg(all(feature = "kindmother-only", not(feature = "memory")))]
pub use kindmother_client_db::JayRdvStore;
pub use memory_store::DbError;
#[cfg(any(feature = "memory", not(feature = "kindmother-only")))]
pub use memory_store::JayRdvStore;
/// Store mémoire (toujours disponible : tests, démos, mode hors-ligne).
pub use memory_store::JayRdvStore as MemoryJayRdvStore;

pub use types::{
    Appointment, AppointmentStatus, CancelledBy, Client, Exception, Practitioner, PractitionerRole,
    Professional, ProfessionalSettings, Reminder, ReminderChannel, Resource, Schedule,
    ScheduleOwner, Service, Slot, SlotStatus,
};
