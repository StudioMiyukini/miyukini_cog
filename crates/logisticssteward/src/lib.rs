//! @id     logisticssteward_lib
//! @do     declarer_module_allocation
//! @role   orchestration
//! @layer  core
//! @human  logisticssteward — module « lib ».

//! # LogisticsSteward
//!
//! Gestionnaire de ressources et optimisation du Miyukini Core System.
//!
//! LogisticsSteward a pour rôle de gérer les ressources système, d'optimiser les
//! allocations et de surveiller l'utilisation.
//!
//! ## État réel (K-17)
//!
//! Ce crate est un **modèle** : types et registres en mémoire (ressources
//! déclarées, comptabilité d'allocations). Ce n'est **pas** une gate active :
//! aucune ressource système réelle n'est mesurée, réservée ni limitée, et
//! `Optimizer` n'a pas d'implémentation.
//!
//! Prévu, non implémenté : application réelle des quotas et mécanismes
//! d'APPROVAL / OVERRIDE / ESCALATION (constat K-17, à mettre au plan).

pub mod allocation;
pub mod optimization;
pub mod resource;

pub use allocation::{Allocation, AllocationError, AllocationManager, DefaultAllocationManager};
pub use optimization::{Optimization, Optimizer};
pub use resource::{DefaultResourceManager, Resource, ResourceManager};
