//! MiyukiniWatch — Service silencieux de mesure des habitudes.
//!
//! Collecte passivement des métriques d'usage (sessions, services, amis, clics)
//! pour alimenter Miou (avatar/mascotte). Ne lit jamais le contenu des messages.
//! Données strictement locales au COG.

pub mod data;
pub mod errors;
pub mod aggregates;
pub mod collector;
pub mod aggregator;
pub mod presenter;

pub use data::{MiyukiniWatchDb, MetricRecord, Prefs};
pub use errors::MiyukiniWatchError;
pub use aggregates::{Aggregate, AggregateId, compute_aggregates};
pub use collector::MiyukiniWatchCollector;
pub use aggregator::MiyukiniWatchAggregator;
pub use presenter::MiyukiniWatchPresenter;
