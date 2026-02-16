//! Persistance JayFestival via KindMother Service.
//! Par défaut : legacy-sqlite. Full KM (kindmother-only) en cours de parité API.

mod types;

#[cfg(feature = "legacy-sqlite")]
mod kindmother_db;

#[cfg(feature = "kindmother-only")]
mod kindmother_client_db;

pub use types::{
    Animation, BudgetEntry, BudgetSummary, Edition, EditionExposant, Exposant, Organisateur,
    Profile, UserType,
};

#[cfg(feature = "legacy-sqlite")]
pub use kindmother_db::{DbError, JayFestivalDb};

#[cfg(feature = "kindmother-only")]
pub use kindmother_client_db::{DbError, JayFestivalDb};
