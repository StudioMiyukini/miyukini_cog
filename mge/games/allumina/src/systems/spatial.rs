//! Per-system proximity snapshots. Many allied-AI and AoE queries used to call
//! [`World::mob_entities`] (which allocates a `Vec`) once *per ally, per frame*,
//! re-scanning every mob each time. [`MobSnapshot`] captures the living-mob
//! entity list **once** (in deterministic `BTreeMap` order) so the per-ally
//! queries reuse it without re-allocating.
//!
//! Positions and health are read live at query time, so results are identical
//! to scanning the world directly — only the repeated allocation is removed.
//! The entity set is stable within a single phase (mobs despawn only in the
//! Death phase), so a snapshot built at the top of a system stays valid for
//! that system's whole pass.

use crate::components::Position2D;
use crate::ecs::{Entity, World};

/// The living-mob entity list captured once for a batch of proximity queries.
pub struct MobSnapshot {
    entities: Vec<Entity>,
}

impl MobSnapshot {
    /// Capture every mob entity, in deterministic `BTreeMap` (entity-id) order.
    #[must_use]
    pub fn build(world: &World) -> Self {
        Self { entities: world.mob_tags.keys().copied().collect() }
    }

    /// The captured mob entities, in deterministic order. For callers needing a
    /// custom predicate beyond [`nearest_alive`]/[`within_alive`].
    #[must_use]
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    /// Nearest mob with non-empty health to `from`, within `range` tiles. Ties
    /// resolve to the lowest entity id (matching `BTreeMap` iteration order).
    #[must_use]
    pub fn nearest_alive(&self, world: &World, from: Position2D, range: f32) -> Option<Entity> {
        self.entities
            .iter()
            .copied()
            .filter(|e| world.health.get(e).is_some_and(|h| !h.is_empty()))
            .filter_map(|e| world.positions.get(&e).map(|p| (e, from.distance(*p))))
            .filter(|(_, d)| *d <= range)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(e, _)| e)
    }

    /// Every living mob within `radius` of `center`, in entity-id order. The
    /// returned `Vec` lets the caller mutate the world while iterating it.
    #[must_use]
    pub fn within_alive(&self, world: &World, center: Position2D, radius: f32) -> Vec<Entity> {
        self.entities
            .iter()
            .copied()
            .filter(|e| world.health.get(e).is_some_and(|h| !h.is_empty()))
            .filter(|e| world.positions.get(e).is_some_and(|p| center.distance(*p) <= radius))
            .collect()
    }
}
