//! Persistance JayKonta (SQLite, instance KindMother Daughter).

mod kindmother_db;
mod types;

pub use kindmother_db::{AccountStats, DbError, JayKontaDb, PurseStats};
pub use types::{AuditRecord, InvoiceRecord, MovementRecord, PaymentRecord, QuoteRecord, ReminderRecord};
