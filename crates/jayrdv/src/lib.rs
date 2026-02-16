//! JayRDV — Service rendez-vous et réservation en ligne du COG.
//!
//! @id: jayrdv_lib
//! @do: expose_jayrdv_public_api_and_modules
//! @role: service
//! @layer: infra
//! @human: Point d'entrée du service JayRDV (rendez-vous, créneaux, ressources, rappels).
//!
//! Domaine : créneaux, ressources, rendez-vous, statuts, rappels.
//! Intégration : JayXpose (vitrine pro), JayKoa (agenda unifié), MiyuBooking, MiyuNotify.
//! Exclusion : le domaine médical est couvert par JayBobo.

pub mod data;
pub mod domain;

pub use data::{
    Appointment, AppointmentStatus, CancelledBy, Client, DbError, Exception, JayRdvStore,
    Practitioner, PractitionerRole, Professional, ProfessionalSettings, Reminder, ReminderChannel,
    Resource, Schedule, ScheduleOwner, Service, Slot, SlotStatus,
};
pub use domain::{
    appointment_cancel, appointment_create, appointment_set_status, client_create, exception_create,
    practitioner_create, professional_create, reminder_create, resource_create, schedule_create,
    service_create, slot_create, slot_hold, slot_release, slot_release_expired,
};
