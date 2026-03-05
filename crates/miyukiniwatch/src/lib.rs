//! MiyukiniWatch — Service silencieux de mesure des habitudes.
//!
//! Collecte passivement des métriques d'usage (sessions, services, amis, clics)
//! pour alimenter Miou (avatar/mascotte). Ne lit jamais le contenu des messages.
//! Données strictement locales au COG.

pub mod aggregates;
pub mod aggregator;
pub mod collector;
pub mod data;
pub mod errors;
pub mod presenter;

pub use aggregates::{compute_aggregates, Aggregate, AggregateId};
pub use aggregator::MiyukiniWatchAggregator;
pub use collector::MiyukiniWatchCollector;
pub use data::{MetricRecord, MiyukiniWatchDb, Prefs};
pub use errors::MiyukiniWatchError;
pub use presenter::MiyukiniWatchPresenter;
