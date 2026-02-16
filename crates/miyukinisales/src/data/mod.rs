//! Couche données MiyukiniSales — types et store.
//!
//! Par défaut : store mémoire. Avec feature `kindmother-only` : délégation KindMother (à venir).

mod memory_store;
mod types;

pub use memory_store::{DbError, MiyukiniSalesStore};
pub use types::{Order, OrderLine, OrderStatus, Quote, QuoteStatus};
