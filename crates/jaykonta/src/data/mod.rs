//! Persistance JayKonta via KindMother Service.
//! Par défaut : legacy-sqlite. Full KM (kindmother-only) en cours de parité API.

mod types;

#[cfg(feature = "legacy-sqlite")]
mod kindmother_db;

#[cfg(feature = "kindmother-only")]
mod kindmother_client_db;

pub use types::{
    AuditRecord, InvoiceRecord, MovementRecord, PaymentRecord, QuoteRecord, ReminderRecord,
};

#[cfg(feature = "legacy-sqlite")]
pub use kindmother_db::{AccountStats, DbError, JayKontaDb, PurseStats};

#[cfg(feature = "kindmother-only")]
pub use kindmother_client_db::{AccountStats, DbError, JayKontaDb, PurseStats};
