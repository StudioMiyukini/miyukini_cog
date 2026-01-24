//! Miyukini Core System — noyau technique minimal.
//! Contrat : traits et types publics uniquement. Pas de métier, pas de backend imposé.

#[macro_use]
extern crate log_facade;

pub mod config;
pub mod id;
pub mod lifecycle;
pub mod log;
pub mod time;

// Ré-exports du contrat : traits, types, implémentations par défaut.
pub use config::{Config, EnvConfig};
pub use id::{Id, IdGenerator, IdParseError, UuidIdGenerator};
pub use lifecycle::{Lifecycle, DefaultLifecycle};
pub use log::{Level, Logger, DefaultLogger};
pub use time::{Clock, DefaultClock};
