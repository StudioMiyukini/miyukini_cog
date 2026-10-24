//! @id     caringnanny_lib
//! @do     declarer_module_health
//! @role   orchestration
//! @layer  core
//! @human  caringnanny — module « lib ».

//! # CaringNanny
//!
//! Moteur d'observation et monitoring du Miyukini Core System.
//!
//! CaringNanny a pour rôle d'observer, détecter, classer, propager et historiser
//! les états du système sans jamais modifier, décider, ou exécuter.
//!
//! ## État réel (K-17)
//!
//! Ce crate est un **modèle** : types et registres en mémoire (événements
//! observés, statuts de santé déclarés). Ce n'est **pas** une gate active ni un
//! moniteur : il n'observe rien de lui-même, il conserve ce qu'on lui transmet.
//! Un composant sans statut enregistré est `HealthStatus::Inconnu`, jamais
//! « sain » par défaut (C-12).
//!
//! Prévu, non implémenté : collecte réelle (signaux des Vigies, lot 3),
//! propagation vers WorrySentinel et ESCALATION (constat K-17, à mettre au plan).

pub mod health;
pub mod metrics;
pub mod observer;

pub use health::{DefaultHealthChecker, HealthChecker, HealthStatus};
pub use metrics::{Metrics, MetricsCollector};
pub use observer::{DefaultObserver, Observer, SystemEvent};
