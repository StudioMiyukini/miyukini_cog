//! # LogisticsSteward
//!
//! Gestionnaire de ressources et optimisation du Miyukini Core System.
//!
//! LogisticsSteward gère les ressources système, optimise les allocations, et surveille l'utilisation.

pub mod allocation;
pub mod optimization;
pub mod resource;

pub use allocation::{Allocation, AllocationError, AllocationManager, DefaultAllocationManager};
pub use optimization::{Optimization, Optimizer};
pub use resource::{DefaultResourceManager, Resource, ResourceManager};
