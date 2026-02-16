//! Couche données JayRDV — types et store.
//!
//! @id: jayrdv_data
//! @do: reexport_data_types_and_store
//! @role: data
//! @layer: domain
//!
//! Par défaut : store mémoire. Avec feature `kindmother-only` : store KindMother (stub = délégation mémoire).

mod memory_store;
mod types;

#[cfg(feature = "kindmother-only")]
mod kindmother_client_db;

pub use memory_store::DbError;
#[cfg(not(feature = "kindmother-only"))]
pub use memory_store::JayRdvStore;
#[cfg(feature = "kindmother-only")]
pub use kindmother_client_db::JayRdvStore;

pub use types::{
    Appointment, AppointmentStatus, CancelledBy, Client, Exception, Practitioner, PractitionerRole,
    Professional, ProfessionalSettings, Reminder, ReminderChannel, Resource, Schedule,
    ScheduleOwner, Service, Slot, SlotStatus,
};
