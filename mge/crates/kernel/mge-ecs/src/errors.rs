// @id: mge-ecs-errors @do: define @role: kernel @layer: 1 @human: miyuk
//
//! Error types for the ECS module.

/// Types d'erreur pour le module ECS.
#[derive(Debug, thiserror::Error)]
pub enum EcsError {
    /// Entity not found in the world.
    #[error("Entity not found")]
    EntityNotFound,

    /// Entity reference is stale (generation mismatch).
    #[error("Entity reference is stale (generation mismatch)")]
    EntityStale,

    /// Archetype not found in the world.
    #[error("Archetype not found")]
    ArchetypeNotFound,

    /// Component type mismatch during downcast.
    #[error("Component type mismatch during downcast")]
    TypeMismatch,

    /// Index out of bounds in a column.
    #[error("Index {index} out of bounds")]
    IndexOutOfBounds {
        /// The invalid index.
        index: usize,
    },

    /// Global resource not found.
    #[error("Resource not found")]
    ResourceNotFound,
}
