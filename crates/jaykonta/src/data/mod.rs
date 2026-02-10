//! Persistance JayKonta via KindMother Service.
//!
//! En mode `legacy-sqlite`, accès direct SQLite (migration progressive).
//! En mode `kindmother-only`, délégation exclusive au service KindMother.

mod types;

// Mode legacy SQLite (migration progressive)
#[cfg(feature = "legacy-sqlite")]
mod kindmother_db;

// Mode KindMother client (production)
#[cfg(feature = "kindmother-only")]
mod kindmother_client_db;

pub use types::{AuditRecord, InvoiceRecord, MovementRecord, PaymentRecord, QuoteRecord, ReminderRecord};

// Export de l'implémentation selon le feature flag
#[cfg(feature = "legacy-sqlite")]
pub use kindmother_db::{AccountStats, DbError, JayKontaDb, PurseStats};

#[cfg(feature = "kindmother-only")]
pub use kindmother_client_db::{AccountStats, DbError, JayKontaDb, PurseStats};
