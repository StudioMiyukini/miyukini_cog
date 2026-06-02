//! Mage de guerre: the three elemental branches (Fire / Frost / Lightning).
//! Per-skill level curves and the chain-lightning resolver, split out of the
//! parent `skill` module. Shared helpers (`lf`, `emit_fx`, `resolve_spell`) are
//! reached via `super`.

use super::*;

// ── Mage de guerre: elemental branches ──────────────────────────────────────

/// Rank invested in each of the Mage's nine spell skills, read off the caster's
/// hotbar by id (0 for any not slotted). Drives the cross-branch synergies.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct MageLevels {
    pub(super) fireball: u32,         // "Boule de feu"      (3001)
    pub(super) flame_wall: u32,       // "Mur de flammes"    (3002)
    pub(super) meteor: u32,           // "Pluie de météores" (3003)
    pub(super) ice_lance: u32,        // "Lance de glace"    (3004)
    pub(super) frost_armor: u32,      // "Armure de givre"   (3005)
    pub(super) blizzard: u32,         // "Blizzard"          (3006)
    pub(super) lightning: u32,        // "Éclair"            (3007)
    pub(super) lightning_shield: u32, // "Bouclier de foudre" (3008)
    pub(super) nova: u32,             // "Nova électrique"   (3009)
}

/// Read the Mage's spell ranks from `caster`'s hotbar.
pub(super) fn read_mage_levels(world: &World, caster: Entity) -> MageLevels {
    MageLevels {
        fireball: skill_level_by_id(world, caster, 3001),
        flame_wall: skill_level_by_id(world, caster, 3002),
        meteor: skill_level_by_id(world, caster, 3003),
        ice_lance: skill_level_by_id(world, caster, 3004),
        frost_armor: skill_level_by_id(world, caster, 3005),
        blizzard: skill_level_by_id(world, caster, 3006),
        lightning: skill_level_by_id(world, caster, 3007),
        lightning_shield: skill_level_by_id(world, caster, 3008),
        nova: skill_level_by_id(world, caster, 3009),
    }
}

/// `u32 → f32` without per-call lint noise.
#[allow(clippy::cast_precision_loss)]
pub(super) fn lf(x: u32) -> f32 {
    x as f32
}

/// The branch (Fire/Frost/Lightning) with the most total invested points. Ties
/// favour Fire, then Frost over Lightning; used to flavour the Mage's summons.
pub(super) fn dominant_branch(l: MageLevels) -> Element {
    let fire = l.fireball + l.flame_wall + l.meteor;
    let frost = l.ice_lance + l.frost_armor + l.blizzard;
    let light = l.lightning + l.lightning_shield + l.nova;
    if fire >= frost && fire >= light {
        Element::Fire
    } else if frost >= light {
        Element::Frost
    } else {
        Element::Lightning
    }
}

// FEU ───────────────────────────────────────────────────────────────────────

/// "Boule de feu": 30 +15/lvl, plus 5% of the other fire skills' representative
/// damage, scaled by the meteor (+3%/lvl) and ice-lance (+2%/lvl) synergies.
pub(super) fn fireball_dmg(l: MageLevels) -> f32 {
    let base = 30.0 + 15.0 * lf(l.fireball);
    let fire_pool = flame_wall_tick_dmg(l) + meteor_impact_dmg(l);
    (base + 0.05 * fire_pool) * (1.0 + 0.03 * lf(l.meteor) + 0.02 * lf(l.ice_lance))
}

/// "Mur de flammes" per-tick damage: 10 +8/lvl, +3%/lvl from Nova électrique.
pub(super) fn flame_wall_tick_dmg(l: MageLevels) -> f32 {
    (10.0 + 8.0 * lf(l.flame_wall)) * (1.0 + 0.03 * lf(l.nova))
}

/// "Mur de flammes" duration: 4s, +5%/lvl from Boule de feu.
pub(super) fn flame_wall_duration(l: MageLevels) -> f32 {
    4.0 * (1.0 + 0.05 * lf(l.fireball))
}

/// "Pluie de météores" per-impact damage: 25 +12/lvl, +5%/lvl from Mur de flammes.
pub(super) fn meteor_impact_dmg(l: MageLevels) -> f32 {
    (25.0 + 12.0 * lf(l.meteor)) * (1.0 + 0.05 * lf(l.flame_wall))
}

/// "Pluie de météores" impact count: 6, +1 every 3 Boule-de-feu levels.
pub(super) fn meteor_impacts(l: MageLevels) -> u32 {
    6 + l.fireball / 3
}

// GLACE ───────────────────────────────────────────────────────────────────────

/// "Lance de glace" damage: 25 +12/lvl, +2%/lvl from Boule de feu.
pub(super) fn ice_lance_dmg(l: MageLevels) -> f32 {
    (25.0 + 12.0 * lf(l.ice_lance)) * (1.0 + 0.02 * lf(l.fireball))
}

/// "Lance de glace" slow fraction: 40%, +5%/lvl from Blizzard, capped at 95%.
pub(super) fn ice_lance_slow(l: MageLevels) -> f32 {
    (0.40 * (1.0 + 0.05 * lf(l.blizzard))).min(0.95)
}

/// "Armure de givre" defence bonus: +20%/lvl, +3%/lvl from Lance de glace.
pub(super) fn frost_armor_def(l: MageLevels) -> f32 {
    0.20 * lf(l.frost_armor) + 0.03 * lf(l.ice_lance)
}

/// "Armure de givre" melee reflection: 10 +5/lvl, +2%/lvl from Blizzard.
pub(super) fn frost_armor_reflect(l: MageLevels) -> f32 {
    (10.0 + 5.0 * lf(l.frost_armor)) * (1.0 + 0.02 * lf(l.blizzard))
}

/// "Blizzard" damage per second: 15 +10/lvl, +5%/lvl from Lance de glace.
pub(super) fn blizzard_dps(l: MageLevels) -> f32 {
    (15.0 + 10.0 * lf(l.blizzard)) * (1.0 + 0.05 * lf(l.ice_lance))
}

/// "Blizzard" duration: 5s, +1s every 3 Armure-de-givre levels.
pub(super) fn blizzard_duration(l: MageLevels) -> f32 {
    5.0 + lf(l.frost_armor / 3)
}

// FOUDRE ──────────────────────────────────────────────────────────────────────

/// "Éclair" damage: 20 +10/lvl, +3%/lvl from Boule de feu.
pub(super) fn lightning_dmg(l: MageLevels) -> f32 {
    (20.0 + 10.0 * lf(l.lightning)) * (1.0 + 0.03 * lf(l.fireball))
}

/// "Éclair" chain count: 2, +1 every 3 Éclair levels and +1 every 3 Nova levels.
pub(super) fn lightning_chains(l: MageLevels) -> u32 {
    2 + l.lightning / 3 + l.nova / 3
}

/// "Bouclier de foudre" riposte damage: 15 +8/lvl, +5%/lvl from Éclair.
pub(super) fn lightning_shield_dmg(l: MageLevels) -> f32 {
    (15.0 + 8.0 * lf(l.lightning_shield)) * (1.0 + 0.05 * lf(l.lightning))
}

/// "Bouclier de foudre" radius: 3m, +2%/lvl from Nova électrique.
pub(super) fn lightning_shield_radius(l: MageLevels) -> f32 {
    3.0 * (1.0 + 0.02 * lf(l.nova))
}

/// "Nova électrique" damage: 35 +15/lvl, +5%/lvl from Bouclier de foudre.
pub(super) fn nova_dmg(l: MageLevels) -> f32 {
    (35.0 + 15.0 * lf(l.nova)) * (1.0 + 0.05 * lf(l.lightning_shield))
}

/// "Nova électrique" stun duration: 0.5 +0.1/lvl, +3%/lvl from Éclair.
pub(super) fn nova_stun(l: MageLevels) -> f32 {
    (0.5 + 0.1 * lf(l.nova)) * (1.0 + 0.03 * lf(l.lightning))
}

/// Chain lightning ("Éclair"): a full-power hit on the primary target, then arcs
/// to the nearest unhit enemies (each at 70% damage).
pub(super) fn chain_bolt(
    world: &mut World,
    caster: Entity,
    first: Entity,
    skill: &ActiveSkill,
    caster_pos: Position2D,
    radius: f32,
    lv: MageLevels,
) -> u32 {
    let dmg = lightning_dmg(lv);
    let chains = lightning_chains(lv);
    let mut hit = vec![first];
    let tpos = world.positions.get(&first).copied().unwrap_or(caster_pos);
    resolve_spell(world, caster, first, dmg, Element::Lightning);
    emit_fx(world, caster, caster_pos, tpos, skill.fx);

    let chain_dmg = dmg * 0.70;
    let mut last = tpos;
    for _ in 0..chains {
        let next = enemies_within(world, last, radius)
            .into_iter()
            .filter(|e| !hit.contains(e))
            .filter_map(|e| world.positions.get(&e).map(|p| (e, last.distance(*p))))
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(e, _)| e);
        let Some(next) = next else { break };
        let npos = world.positions.get(&next).copied().unwrap_or(last);
        resolve_spell(world, caster, next, chain_dmg, Element::Lightning);
        emit_fx(world, caster, last, npos, SkillFx::Lightning);
        hit.push(next);
        last = npos;
    }
    #[allow(clippy::cast_possible_truncation)]
    {
        hit.len() as u32
    }
}
