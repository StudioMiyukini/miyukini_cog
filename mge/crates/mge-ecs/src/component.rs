//! Trait Component — marqueur pour les composants.
//!
//! @id mge.kernel.ecs.component
//! @role simulation
//! @layer kernel
//! @human Marqueur des composants ECS
//! @do define_component_marker_trait

/// Marqueur requis pour tout composant. Données pures : Send + Sync.
pub trait Component: Send + Sync + 'static {}
