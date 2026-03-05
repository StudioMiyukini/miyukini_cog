//! Couche données JayFaim — types et store.
//!
//! Par défaut : store mémoire. Avec feature `kindmother-only` : à venir.

mod memory_store;
mod types;

pub use memory_store::{DbError, JayFaimStore};
pub use types::{Guest, Reservation, ReservationSlot, ReservationStatus, Table};
