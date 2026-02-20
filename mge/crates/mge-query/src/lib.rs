//! Query — helpers d'itération ECS, Query2Mut.
//!
//! @id mge.kernel.query
//! @role simulation
//! @layer kernel
//! @human Helpers de requête et itération
//! @do provide_query_iteration_helpers

// Re-export des types de requête depuis mge-ecs.
// Les méthodes iter1, iter2, iter3, for_each_mut, for_each1_mut, iter2_mut
// sont sur mge_ecs::World.
pub use mge_ecs::{Component, EntityId, Query2Mut, World};
