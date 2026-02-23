//! @id allumina.prototype.combat
//! @role system
//! @layer application
//! @domain allumina
//! @do combat_damage_application
//!
//! Système d'application des dégâts — Phase 80.
//! Lit les PendingDamage, applique à CombatStats, marque Dead si HP ≤ 0.
//! Le joueur (PlayerMarker) ne peut pas mourir (prototype).

use crate::components::PlayerMarker;
use crate::stats::{CombatStats, Dead, PendingDamage};

/// Phase 80 : applique tous les PendingDamage accumulés dans le tick.
pub fn combat_apply_system(world: &mut mge_ecs::World, _ctx: &mut mge_core::Context) {
    // Collecter les entités avec des dégâts en attente
    let damaged: Vec<(mge_ecs::EntityId, i32)> = world
        .iter1::<PendingDamage>()
        .filter(|(_, pd)| pd.amount > 0)
        .map(|(id, pd)| (id, pd.amount))
        .collect();

    for (id, raw_dmg) in damaged {
        // Réinitialiser le pending (pour le prochain tick)
        if let Some(pd) = world.get_mut::<PendingDamage>(id) {
            pd.amount = 0;
        }

        // is_player : shared borrow libéré avant la mut borrow sur stats
        let is_player = world.has_component::<PlayerMarker>(id);

        // Appliquer à CombatStats (formule D2 : dégâts réels = max(1, raw - défense))
        let mut should_die = false;
        if let Some(stats) = world.get_mut::<CombatStats>(id) {
            let actual = (raw_dmg - stats.defense).max(1);
            stats.hp = (stats.hp - actual).max(0);
            should_die = stats.hp <= 0;
        }

        // Marquer mort après libération du borrow stats
        if should_die && !is_player {
            world.insert(id, Dead);
        }
    }
}
