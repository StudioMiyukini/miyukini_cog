//! Module données JayFestival : types domaine et base fille SQLite via KindMother.
//!
//! KindMother instance Daughter : persistance locale SQLite, autorité données.

mod kindmother_db;
mod types;

pub use kindmother_db::{DbError, JayFestivalDb};
pub use types::{Edition, EditionExposant, Exposant, Organisateur, Profile, UserType};
