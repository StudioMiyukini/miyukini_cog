//! Intégration Miyubooking (créneaux, réservations, billets, pass).
//!
//! JayFestival consomme Miyubooking pour écrans visiteur (réservations, billets, pass)
//! et organisateur (créneaux ateliers, etc.).

pub mod adapter;

pub use adapter::{miyubooking_create_booking, miyubooking_list_slots, MiyubookingAdapterError};
