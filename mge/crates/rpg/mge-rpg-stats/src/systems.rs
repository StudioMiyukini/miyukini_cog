//! @id mge.rpg.stats.v1.systems
//! @role system
//! @layer plugin
//! @domain rpg
//! @do health_regen_per_tick

use mge_core::Context;
use mge_ecs::{EntityId, World};
use std::collections::HashSet;

use crate::components::{DeadTag, Health};

/// Régénération HP par tick pour toutes les entités vivantes.
/// Phase recommandée : 300+ (après combat).
pub fn health_regen_system(world: &mut World, ctx: &mut Context) {
    let dt = ctx.delta_secs() as f64;

    let dead_set: HashSet<EntityId> = world
        .iter1::<DeadTag>()
        .map(|(id, _)| id)
        .collect();

    world.for_each1_mut::<Health, _>(|id, health| {
        if dead_set.contains(&id) {
            return;
        }
        if health.current < health.max && health.regen_rate > 0.0 {
            health.current = (health.current + health.regen_rate * dt).min(health.max);
        }
    });
}
