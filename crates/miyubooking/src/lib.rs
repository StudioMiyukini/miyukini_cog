//! # MiyuBooking — toolkit.booking.reservations
//!
//! Kit d'outils réservations (créneaux, réservations, ressources, prix, participants).
//! Toute écriture = WriteIntent KindMother ; décision (création, annulation) = StrongFather.
//! Alignement MIP : domaine `booking`, layer tool/toolkit.

pub mod admin_cell;
pub mod booking;
pub mod context;
pub mod errors;
pub mod participants;
pub mod price;
pub mod resource;
pub mod slots;

pub use admin_cell::{
    miyubooking_admin_cell, MiyubookingAdminCell, MiyubookingIdentification, MiyubookingIntegrity,
    MiyubookingTestManifest, TOOLKIT_ID,
};
pub use booking::{cancel as booking_cancel, create as booking_create, update as booking_update};
pub use context::GovernedContext;
pub use errors::MiyubookingError;
pub use participants::compute as participants_compute;
pub use price::compute as price_compute;
pub use resource::{availability as resource_availability, resolve as resource_resolve};
pub use slots::{list as slots_list, resolve as slots_resolve};
