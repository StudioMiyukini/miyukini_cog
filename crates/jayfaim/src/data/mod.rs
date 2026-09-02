//! @id     jayfaim_data_mod
//! @do     declarer_module_memory_store
//! @role   donnee
//! @layer  core
//! @human  jayfaim — module « data mod ».

//! Couche données JayFaim — types et store.
//!
//! Par défaut : store mémoire. Avec feature `kindmother-only` : à venir.

mod memory_store;
mod types;

pub use memory_store::{DbError, JayFaimStore};
pub use types::{Guest, Reservation, ReservationSlot, ReservationStatus, Table};
