//! Intégration Miyubooking (créneaux, réservations, billets, pass).
//!
//! JayFestival consomme Miyubooking pour écrans visiteur (réservations, billets, pass)
//! et organisateur (créneaux ateliers, etc.).
//!
//! @id: jayfestival_svc_miyubooking_mod @do: export_miyubooking_adapter
//! @role: api @layer: service
//! @human: Module intégration Miyubooking — réservations, billets et créneaux ateliers JayFestival.

pub mod adapter;

pub use adapter::{miyubooking_create_booking, miyubooking_list_slots, MiyubookingAdapterError};
