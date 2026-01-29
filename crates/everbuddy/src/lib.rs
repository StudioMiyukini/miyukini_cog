//! # EverBuddy
//!
//! Gestionnaire de compatibilité et migration du Miyukini Core System.
//!
//! EverBuddy gouverne l'évolution des structures, des contrats, et des entités dans le temps.

pub mod compatibility;
pub mod migration;
pub mod version;

pub use compatibility::{Compatibility, CompatibilityChecker};
pub use migration::{Migration, MigrationExecutor};
pub use version::{Version, VersionManager};
