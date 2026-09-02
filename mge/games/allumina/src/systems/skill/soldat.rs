//! @id     games_allumina_systems_skill_soldat
//! @do     grand_coup_mult
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « systems skill soldat ».

//! Soldat: melee-technique damage curves (Grand coup / Taille / Estocs) and
//! the battle-order system (synergies, bonuses, application + per-tick upkeep).
//! Shared helpers (`lf`, `skill_level_by_id`, `emit_fx`) are reached via `super`.

use super::*;

/// Grand-coup damage multiplier on a normal swing: heavier two-handed, scaling
/// with rank (1H 1.5 +0.05/rank, 2H 1.7 +0.07/rank).
#[allow(clippy::cast_precision_loss)]
pub(super) fn grand_coup_mult(two_handed: bool, lvl: u32) -> f32 {
    if two_handed {
        1.7 + 0.07 * lvl as f32
    } else {
        1.5 + 0.05 * lvl as f32
    }
}

/// Taille damage multiplier: −30% base, +10%/Taille rank, +5%/Grand-coup rank.
#[allow(clippy::cast_precision_loss)]
pub(super) fn taille_mult(taille_lvl: u32, grand_coup_lvl: u32) -> f32 {
    0.7 + 0.10 * taille_lvl as f32 + 0.05 * grand_coup_lvl as f32
}

/// Per-strike interval for Estocs: the base swing accelerated by +200% +10%/rank.
#[allow(clippy::cast_precision_loss)]
pub(super) fn estocs_interval(base_interval: f32, lvl: u32) -> f32 {
    base_interval / (3.0 + 0.1 * lvl as f32)
}

/// Skill levels that feed the Soldat order synergies, read off a caster's
/// hotbar by skill id (0 for any the caster doesn't field).
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct OrderSynergyLevels {
    pub(super) peasant: u32,    // "Paysans" (1005)
    pub(super) militiaman: u32, // "Miliciens" (1006)
    pub(super) warrior: u32,    // "Guerriers" (1007)
    pub(super) hold_fast: u32,  // "Tenez bon !" (1009)
    pub(super) wargh: u32,      // "Wargh !" (1010)
}

pub(super) fn read_order_synergies(world: &World, caster: Entity) -> OrderSynergyLevels {
    OrderSynergyLevels {
        peasant: skill_level_by_id(world, caster, 1005),
        militiaman: skill_level_by_id(world, caster, 1006),
        warrior: skill_level_by_id(world, caster, 1007),
        hold_fast: skill_level_by_id(world, caster, 1009),
        wargh: skill_level_by_id(world, caster, 1010),
    }
}

/// Compute an order's army-wide bonuses from its own rank `lvl` and the caster's
/// synergistic skill levels. All values are additive fractions (0.05 = +5%);
/// `def_pct` is signed (Wargh sacrifices defence).
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub(super) fn order_bonuses_for(order: OrderType, lvl: u32, s: OrderSynergyLevels) -> OrderBonuses {
    let l = lvl as f32;
    let (peasant, militiaman, warrior, hold_fast, wargh) = (
        s.peasant as f32,
        s.militiaman as f32,
        s.warrior as f32,
        s.hold_fast as f32,
        s.wargh as f32,
    );
    match order {
        // +3% move/lvl, +1%/lvl Paysans, +1%/lvl Wargh.
        OrderType::Advance => OrderBonuses {
            move_speed_pct: (3.0 * l + peasant + wargh) / 100.0,
            ..OrderBonuses::default()
        },
        // +5% DEF/lvl (+5%/lvl Miliciens, +3%/lvl Guerriers) and +5% HP/lvl.
        OrderType::HoldFast => OrderBonuses {
            def_pct: (5.0 * l + 5.0 * militiaman + 3.0 * warrior) / 100.0,
            hp_pct: (5.0 * l) / 100.0,
            ..OrderBonuses::default()
        },
        // +5% atk speed/lvl, +5% dmg/lvl (+3%/lvl Guerriers). DEF is slashed to
        // -100%, recovered +5%/lvl (+3%/lvl Miliciens, +3%/lvl Tenez bon).
        OrderType::Wargh => {
            let def_malus =
                (100.0 - 5.0 * l - 3.0 * militiaman - 3.0 * hold_fast).max(0.0);
            OrderBonuses {
                atk_speed_pct: (5.0 * l) / 100.0,
                dmg_pct: (5.0 * l + 3.0 * warrior) / 100.0,
                def_pct: -def_malus / 100.0,
                ..OrderBonuses::default()
            }
        }
    }
}

/// Apply (or refresh) one battle order on `ally`. A re-cast of the same order
/// replaces the previous instance, reverting its HP bump first so the +HP never
/// compounds across refreshes.
pub(super) fn apply_order(
    world: &mut World,
    ally: Entity,
    order_type: OrderType,
    dur: f32,
    bonuses: OrderBonuses,
) {
    let stale_hp = world
        .order_buffs
        .get(&ally)
        .and_then(|v| v.iter().find(|b| b.order_type == order_type))
        .map(|b| b.hp_added);
    if let Some(hp) = stale_hp {
        revert_order_hp(world, ally, hp);
        if let Some(v) = world.order_buffs.get_mut(&ally) {
            v.retain(|b| b.order_type != order_type);
        }
    }

    let hp_added = if bonuses.hp_pct > 0.0 {
        world.health.get_mut(&ally).map_or(0.0, |h| {
            let add = h.max * bonuses.hp_pct;
            h.set_max(h.max + add);
            add
        })
    } else {
        0.0
    };

    world.order_buffs.entry(ally).or_default().push(OrderBuff {
        order_type,
        remaining_secs: dur,
        bonuses,
        hp_added,
    });
}

/// Remove a previously granted max-life bump (clamped so life never drops below 1).
pub(super) fn revert_order_hp(world: &mut World, e: Entity, hp_added: f32) {
    if hp_added <= 0.0 {
        return;
    }
    if let Some(h) = world.health.get_mut(&e) {
        h.set_max((h.max - hp_added).max(1.0));
    }
}

/// Age every active battle order, reverting the HP bump of any that expire.
pub(super) fn tick_orders(world: &mut World, dt: f32) {
    let mut reverts: Vec<(Entity, f32)> = Vec::new();
    for (e, list) in world.order_buffs.iter_mut() {
        for b in list.iter_mut() {
            b.remaining_secs -= dt;
        }
        for b in list.iter().filter(|b| b.remaining_secs <= 0.0) {
            if b.hp_added > 0.0 {
                reverts.push((*e, b.hp_added));
            }
        }
        list.retain(|b| b.remaining_secs > 0.0);
    }
    world.order_buffs.retain(|_, v| !v.is_empty());
    for (e, hp) in reverts {
        revert_order_hp(world, e, hp);
    }
}
