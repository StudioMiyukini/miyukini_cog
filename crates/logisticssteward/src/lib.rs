//! # LogisticsSteward
//!
//! Gestionnaire de ressources et optimisation du Miyukini Core System.
//!
//! LogisticsSteward gère les ressources système, optimise les allocations, et surveille l'utilisation.

pub mod resource;
pub mod optimization;
pub mod allocation;

pub use allocation::{Allocation, AllocationManager};
pub use optimization::{Optimization, Optimizer};
pub use resource::{Resource, ResourceManager};
