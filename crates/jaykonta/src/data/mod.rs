//! @id     jaykonta_data_mod
//! @do     declarer_module_types
//! @role   donnee
//! @layer  core
//! @human  jaykonta — module « data mod ».

//! Persistance JayKonta via KindMother Service.
//!
//! - `kindmother-only` (défaut, K-13) : KindMother est le seul écrivain ; schéma
//!   `schema.sql` appliqué par migrations versionnées côté service.
//! - `legacy-sqlite` (HÉRITÉ, transitoire) : accès SQLite direct, retiré au lot C+.
//!   Un opt-in explicite à `legacy-sqlite` prend le pas sur `kindmother-only` (même
//!   si ce dernier est actif par défaut) : le chemin transitoire reste ainsi
//!   sélectionnable sans désactiver les features par défaut des crates dépendantes.

mod types;

#[cfg(feature = "legacy-sqlite")]
mod kindmother_db;

#[cfg(feature = "legacy-sqlite")]
pub mod migrations;

#[cfg(all(feature = "kindmother-only", not(feature = "legacy-sqlite")))]
mod kindmother_client_db;

#[cfg(any(all(feature = "kindmother-only", not(feature = "legacy-sqlite")), feature = "import-heritage"))]
mod schema_kindmother;

#[cfg(feature = "import-heritage")]
pub mod import_heritage;

pub use types::{
    AuditRecord, InvoiceRecord, MovementRecord, PaymentRecord, QuoteRecord, ReminderRecord,
};

#[cfg(feature = "legacy-sqlite")]
pub use kindmother_db::{AccountStats, DbError, JayKontaDb, PurseStats};

#[cfg(all(feature = "kindmother-only", not(feature = "legacy-sqlite")))]
pub use kindmother_client_db::{AccountStats, DbError, JayKontaDb, PurseStats};

#[cfg(any(all(feature = "kindmother-only", not(feature = "legacy-sqlite")), feature = "import-heritage"))]
pub use schema_kindmother::schema_migrations;
