//! @id     jayfaim_lib
//! @do     declarer_module_data
//! @role   orchestration
//! @layer  core
//! @human  jayfaim — module « lib ».

//! JayFaim — Service réservation tables et restauration du COG.
//!
//! Domaine : tables, créneaux, réservations, convives.
//! Intégration prévue : MiyuBooking, JayKonta (paiement).

pub mod data;
pub mod domain;

pub use data::{
    DbError, Guest, JayFaimStore, Reservation, ReservationSlot, ReservationStatus, Table,
};
pub use domain::{
    guest_add, reservation_cancel, reservation_confirm, reservation_create, slot_create,
    table_create,
};
