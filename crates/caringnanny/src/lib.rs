//! # CaringNanny
//!
//! Moteur d'observation et monitoring du Miyukini Core System.
//!
//! CaringNanny observe, détecte, classe, propage, et historise les états du système
//! sans jamais modifier, décider, ou exécuter.

pub mod observer;
pub mod metrics;
pub mod health;

pub use health::{DefaultHealthChecker, HealthChecker, HealthStatus};
pub use metrics::{Metrics, MetricsCollector};
pub use observer::{DefaultObserver, Observer, SystemEvent};
