//! EntityId — identifiant opaque (index + generation).
//!
//! @id mge.kernel.ecs.entity
//! @role simulation
//! @layer kernel
//! @human Identifiant opaque d'entité
//! @do represent_opaque_entity_identifier

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId {
    index: u32,
    generation: u32,
}

impl EntityId {
    /// Crée un EntityId (index, generation). Utilisé par World::spawn et tests.
    pub fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    /// Bits pour dérivation de seed par entité : global_seed ^ to_bits()
    #[must_use]
    pub fn to_bits(&self) -> u64 {
        (u64::from(self.index) << 32) | u64::from(self.generation)
    }

    /// Index dans le World.
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Génération pour invalider les EntityId recyclés.
    pub fn generation(&self) -> u32 {
        self.generation
    }
}
