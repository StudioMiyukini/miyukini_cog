//! @id     games_allumina_systems_skill_clerc
//! @do     definir_clerc_levels
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « systems skill clerc ».

//! Clerc: the SOUTIEN / BÉNI / DIVIN branches — aura toggles, level curves,
//! and the support status ticks (auras, conversion, pardon, resurrection).
//! Shared helpers are reached via `super`.

use super::*;

// ── Clerc: branches SOUTIEN / BÉNI / DIVIN ──────────────────────────────────

/// Rank invested in each of the Clerc's 15 hotbar entries, read off the caster's
/// hotbar by id (0 for any not slotted). Drives the cross-branch synergies.
/// Several fields are reserved for future synergies but not yet read.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ClercLevels {
    pub heal_aura: u32,       // 5001
    pub def_aura: u32,        // 5002
    pub resist_aura: u32,     // 5003
    pub war_aura: u32,        // 5004
    pub wind_aura: u32,       // 5005
    pub vengeance_aura: u32,  // 5006
    pub prayer: u32,          // 5007
    pub exorcism: u32,        // 5008
    pub conversion: u32,      // 5009
    pub pardon: u32,          // 5010
    pub resurrection: u32,    // 5011
    pub prophet: u32,         // 5012 (passive)
    pub cherubin: u32,        // 5013
    pub guardian: u32,        // 5014
    pub archangel: u32,       // 5015
}

pub(crate) fn read_clerc_levels(world: &World, caster: Entity) -> ClercLevels {
    ClercLevels {
        heal_aura: skill_level_by_id(world, caster, 5001),
        def_aura: skill_level_by_id(world, caster, 5002),
        resist_aura: skill_level_by_id(world, caster, 5003),
        war_aura: skill_level_by_id(world, caster, 5004),
        wind_aura: skill_level_by_id(world, caster, 5005),
        vengeance_aura: skill_level_by_id(world, caster, 5006),
        prayer: skill_level_by_id(world, caster, 5007),
        exorcism: skill_level_by_id(world, caster, 5008),
        conversion: skill_level_by_id(world, caster, 5009),
        pardon: skill_level_by_id(world, caster, 5010),
        resurrection: skill_level_by_id(world, caster, 5011),
        prophet: skill_level_by_id(world, caster, 5012),
        cherubin: skill_level_by_id(world, caster, 5013),
        guardian: skill_level_by_id(world, caster, 5014),
        archangel: skill_level_by_id(world, caster, 5015),
    }
}

/// Aura effective radius: 5 tiles +0.5/lvl on the toggled aura.
pub(super) fn aura_radius(level: u32) -> f32 {
    5.0 + 0.5 * lf(level)
}

/// Per-level Clerc aura bonuses, packed into [`AuraBonus`] fields. Only the
/// fields relevant to `aura_type` are non-zero so the same struct can back any
/// aura. Wind grants both move and attack-speed; Vengeance the reflect.
pub(super) fn aura_bonus_for(aura_type: AuraType, level: u32) -> AuraBonus {
    let l = lf(level);
    let mut b = AuraBonus::default();
    match aura_type {
        // Heal is applied directly to HP, not via AuraBonus.
        AuraType::Heal => {}
        AuraType::Defense => b.def_pct = 0.05 * l,
        AuraType::Resist => b.resist_pct = 0.05 * l,
        AuraType::War => b.dmg_pct = 0.05 * l,
        AuraType::Wind => {
            b.move_pct = 0.05 * l;
            b.atk_speed_pct = 0.05 * l;
        }
        AuraType::Vengeance => b.retaliate_pct = 0.10 * l,
    }
    // Each refresh extends a short window so the bonus survives between ticks.
    b.remaining = 1.5;
    b
}

/// Heal-aura fraction of max HP restored per second: 1% +0.5%/lvl.
pub(super) fn heal_aura_pct_per_sec(level: u32) -> f32 {
    0.01 + 0.005 * lf(level)
}

/// Toggle the Clerc's currently active aura. Pressing the same aura skill again
/// deactivates it (refunds the mana reservation). Pressing a different one
/// swaps. Reservation is the skill's static mana cost.
pub(super) fn toggle_aura(world: &mut World, caster: Entity, skill: &ActiveSkill, aura_type: AuraType) {
    if let Some(active) = world.active_auras.get(&caster).copied() {
        // Refund any previous reservation.
        if let Some(amount) = world.aura_reserved_mana.remove(&caster) {
            if let Some(m) = world.mana.get_mut(&caster) {
                m.max += amount;
            }
        }
        world.active_auras.remove(&caster);
        if active.aura_type == aura_type {
            world.log_event(format!("Aura {:?} désactivée.", aura_type));
            return;
        }
    }
    // Install the new aura. Reserve mana_cost from the caster's max mana.
    let reserved = skill.mana_cost;
    if let Some(m) = world.mana.get_mut(&caster) {
        m.max = (m.max - reserved).max(1.0);
        m.current = m.current.min(m.max);
    }
    world.aura_reserved_mana.insert(caster, reserved);
    world.active_auras.insert(
        caster,
        ActiveAura {
            skill_id: skill.id,
            aura_type,
            level: skill.level,
            reserved_mana: reserved,
            radius: aura_radius(skill.level),
            tick_timer: 0.0,
        },
    );
    world.log_event(format!("Aura {:?} activée.", aura_type));
}

/// Advance every active aura by `dt`. When the per-second tick fires, refresh
/// the aura bonuses on every ally currently in radius (and heal them if the
/// aura is the Heal aura).
pub(super) fn tick_auras(world: &mut World, dt: f32) {
    for caster in world.active_auras.keys().copied().collect::<Vec<_>>() {
        let Some(mut aura) = world.active_auras.get(&caster).copied() else { continue };
        aura.tick_timer -= dt;
        if aura.tick_timer > 0.0 {
            world.active_auras.insert(caster, aura);
            continue;
        }
        aura.tick_timer += 1.0;
        world.active_auras.insert(caster, aura);

        let Some(center) = world.positions.get(&caster).copied() else { continue };
        let allies = allies_in_radius(world, caster, center, aura.radius);
        let bonus_template = aura_bonus_for(aura.aura_type, aura.level);
        let heal_pct = if matches!(aura.aura_type, AuraType::Heal) {
            heal_aura_pct_per_sec(aura.level)
        } else {
            0.0
        };
        for ally in allies {
            if heal_pct > 0.0 {
                if let Some(h) = world.health.get_mut(&ally) {
                    if !h.is_empty() {
                        h.restore(h.max * heal_pct);
                    }
                }
            }
            if !matches!(aura.aura_type, AuraType::Heal) {
                // Refresh the recipient's aura bonus (replace, not stack).
                world.aura_bonuses.insert(ally, bonus_template);
            }
        }
    }
}

/// Age aura bonuses; expired entries fall off when no aura refreshes them.
pub(super) fn tick_aura_bonuses(world: &mut World, dt: f32) {
    for b in world.aura_bonuses.values_mut() {
        b.remaining -= dt;
    }
    world.aura_bonuses.retain(|_, b| b.remaining > 0.0);
}

/// Tick the Clerc's [`Converted`] mobs; restore their original faction when the
/// charm expires.
pub(super) fn tick_converted(world: &mut World, dt: f32) {
    for e in world.converted.keys().copied().collect::<Vec<_>>() {
        let Some(mut c) = world.converted.get(&e).copied() else { continue };
        c.remaining -= dt;
        if c.remaining <= 0.0 {
            world.factions.insert(e, c.original_faction);
            world.converted.remove(&e);
            if let Some(combat) = world.combat.get_mut(&e) {
                combat.target = None;
            }
        } else {
            world.converted.insert(e, c);
        }
    }
}

/// Tick the Clerc's [`Pardoned`] mobs; expire the mark when its window closes.
pub(super) fn tick_pardoned(world: &mut World, dt: f32) {
    for p in world.pardoned.values_mut() {
        p.remaining -= dt;
    }
    world.pardoned.retain(|_, p| p.remaining > 0.0);
}

/// Advance any in-progress Résurrection. CC during the cast cancels it; on
/// completion the caster's dead mercenary is queued for an instant respawn.
pub(super) fn tick_resurrecting(world: &mut World, dt: f32) {
    for caster in world.resurrecting.keys().copied().collect::<Vec<_>>() {
        // CC during the channel cancels the cast.
        if crate::systems::combat::is_cc_locked(world, caster)
            || world.stuns.contains_key(&caster)
            || world.knockdowns.contains_key(&caster)
            || world.pushbacks.contains_key(&caster)
        {
            world.resurrecting.remove(&caster);
            world.log_event("Résurrection interrompue.".to_string());
            continue;
        }
        let Some(mut r) = world.resurrecting.get(&caster).copied() else { continue };
        r.remaining -= dt;
        if r.remaining > 0.0 {
            world.resurrecting.insert(caster, r);
            continue;
        }
        world.resurrecting.remove(&caster);
        // Find a dead mercenary owned by the caster and queue its respawn.
        let dead_merc = world
            .mercenary_entities()
            .into_iter()
            .find(|e| {
                world.mercenary_tags.get(e).is_some_and(|t| t.owner == caster)
                    && world.merc_states.get(e).is_some_and(|s| s.is_dead())
            });
        if let Some(merc) = dead_merc {
            if let Some(state) = world.merc_states.get_mut(&merc) {
                // A tiny remaining time lets the mercenary update system pick
                // up the revive on its next pass.
                state.respawn_timer = 0.001;
            }
            world.log_event("Résurrection: votre mercenaire revient!".to_string());
        } else {
            world.log_event("Résurrection: aucun allié à ramener.".to_string());
        }
    }
}

/// Prière / Exorcism shared radius: 3 +0.3/lvl on the Prière rank.
pub(super) fn prayer_radius(cl: ClercLevels) -> f32 {
    3.0 + 0.3 * lf(cl.prayer)
}

/// Prière stun duration: 1s +0.2/lvl.
pub(super) fn prayer_stun_dur(cl: ClercLevels) -> f32 {
    1.0 + 0.2 * lf(cl.prayer)
}

/// Exorcism flat holy damage: 30 +15/lvl, +5%/lvl from Pardon and Conversion.
pub(super) fn exorcism_dmg(cl: ClercLevels) -> f32 {
    (30.0 + 15.0 * lf(cl.exorcism)) * (1.0 + 0.05 * lf(cl.pardon) + 0.05 * lf(cl.conversion))
}

/// Conversion duration: 8s +1s/lvl.
pub(super) fn conversion_dur(cl: ClercLevels) -> f32 {
    8.0 + lf(cl.conversion)
}

/// Pardon leech fraction (of the mob's max HP per hit): 5% +1%/lvl.
pub(super) fn pardon_leech_pct(cl: ClercLevels) -> f32 {
    0.05 + 0.01 * lf(cl.pardon)
}

/// Pardon duration: 10s +1s/lvl.
pub(super) fn pardon_dur(cl: ClercLevels) -> f32 {
    10.0 + lf(cl.pardon)
}

/// Résurrection cast time: 3s -0.2s/lvl, floored at 1s.
pub(super) fn resurrection_cast_time(level: u32) -> f32 {
    (3.0 - 0.2 * lf(level)).max(1.0)
}
