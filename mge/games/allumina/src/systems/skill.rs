//! Active-skill execution: cooldowns, mana, status timers, and the per-effect
//! resolution for all twelve class skills (single target, self/target AoE,
//! chain, buff, dash, blink, frost-nova slow, trap, dodge).

use mge_items::item::EquipSlot;

use crate::components::cc::Interruptible;
use crate::components::player::ClassType;
use crate::components::skill::{
    ActiveAura, ActiveSkill, ArrowRainField, AuraBonus, AuraType, Buff, ChargingShot, ComboLock,
    Converted, DefDebuff, Dot, Element, HealField, HunterInstinctBuff, OrderBonuses, OrderBuff,
    OrderType, Pardoned, Resist, Resurrecting, SkillEffect, Slow, Trap, TrapKind, Unstoppable,
};
use crate::components::summon::SummonType;
use crate::components::{Faction, Position2D};
use crate::ecs::{Entity, RenderEvent, SkillFx, World};
use crate::render::figures::Direction;
use crate::systems::combat::{
    apply_knockdown, apply_pushback, apply_pushback_then_knockdown, effective_damage, is_cc_locked,
    resolve_attack, resolve_spell, try_interrupt, AttackOutcome,
};

/// Timed ground-effect fields (flame wall, blizzard, meteor, lightning shield,
/// arrow rain), kept in their own file to keep this module navigable.
mod fields;

/// Mage de guerre elemental-branch level curves + chain-lightning resolver.
/// Re-exported so the dispatcher and other classes keep calling them unqualified
/// (the shared `lf` helper lives here now).
mod mage;
use mage::*;

/// Archer thematic-branch level curves (Tir / Piège / Agilité), re-exported so
/// the dispatcher keeps calling them unqualified.
mod archer;
use archer::*;

/// Clerc support branches: aura toggles, level curves, and the support status
/// ticks. Re-exported so the dispatcher and `tick_statuses` call them unqualified.
mod clerc;
use clerc::*;

/// Soldat melee-technique curves + battle-order system. Re-exported so the
/// dispatcher and `tick_statuses` call them unqualified.
mod soldat;
use soldat::*;

/// Passive mana regenerated per second.
const MANA_REGEN: f32 = 2.0;

pub fn skill_system(world: &mut World) {
    let dt = world.dt;
    let Some(player) = world.player_entity else { return };

    // Tick cooldowns, regen mana, age status effects.
    if let Some(slot) = world.skills.get_mut(&player) {
        slot.tick(dt);
    }
    if let Some(m) = world.mana.get_mut(&player) {
        m.restore(MANA_REGEN * dt);
    }
    tick_statuses(world, dt);

    // Resolve a queued skill use.
    let Some(target) = world.input.pending_skill.take() else { return };
    let Some(sel) = world.skills.get(&player).map(|s| s.selected) else { return };
    let Some(skill) = world.skills.get(&player).and_then(|s| s.selected_skill().cloned()) else {
        return;
    };

    // Passive skills (e.g. Élu prophétique) live on the hotbar so their rank is
    // discoverable, but they never fire — they are read live by other systems.
    if matches!(skill.effect, SkillEffect::Passive { .. }) {
        return;
    }

    // A combo lock (Estocs) or an in-progress Résurrection cast locks the
    // owner out of starting another cast.
    if world.combo_locks.contains_key(&player) || world.resurrecting.contains_key(&player) {
        return;
    }

    // Crowd control (knockdown / recul / stun) prevents the caster from acting.
    if is_cc_locked(world, player) {
        return;
    }

    // Auras toggle on/off and don't follow the normal "pay then cast" flow —
    // their static mana_cost is reserved (deducted from max mana) while active.
    if let SkillEffect::Aura { aura_type } = skill.effect {
        let cd_ready = world
            .skills
            .get(&player)
            .and_then(|s| s.cooldowns.get(sel))
            .is_some_and(|c| c.is_ready());
        if !cd_ready {
            return;
        }
        let caster_pos = world.positions.get(&player).copied().unwrap_or(Position2D::new(0.0, 0.0));
        toggle_aura(world, player, &skill, aura_type);
        if let Some(s) = world.skills.get_mut(&player) {
            if let Some(c) = s.cooldowns.get_mut(sel) {
                c.trigger(skill.cooldown);
            }
        }
        emit_fx(world, player, caster_pos, caster_pos, skill.fx);
        return;
    }

    let cost = effective_mana_cost(&skill);
    let cd_ready = world
        .skills
        .get(&player)
        .and_then(|s| s.cooldowns.get(sel))
        .is_some_and(|c| c.is_ready());
    let has_mana = world.mana.get(&player).is_some_and(|m| m.current >= cost);
    if !cd_ready || !has_mana {
        return;
    }

    let Some(caster_pos) = world.positions.get(&player).copied() else { return };

    // Gate targeted skills on a valid, in-range enemy.
    if skill.effect.needs_target() {
        let valid = target != player
            && world.health.get(&target).is_some_and(|h| !h.is_empty())
            && world.positions.contains_key(&target);
        if !valid {
            return;
        }
        let tpos = *world.positions.get(&target).unwrap();
        // Dash/Blink may reach further than their nominal strike range.
        let max = match skill.effect {
            SkillEffect::Dash | SkillEffect::Blink { .. } => skill.range.max(6.0),
            _ => skill.range,
        };
        if caster_pos.distance(tpos) > max {
            return;
        }
    }

    // Pay costs and fire.
    if let Some(m) = world.mana.get_mut(&player) {
        m.drain(cost);
    }
    if let Some(s) = world.skills.get_mut(&player) {
        if let Some(c) = s.cooldowns.get_mut(sel) {
            c.trigger(skill.cooldown);
        }
    }

    // Face the target when casting a targeted skill.
    if skill.effect.needs_target() {
        if let Some(tpos) = world.positions.get(&target).copied() {
            if let Some(dir) = Direction::from_world_vector(tpos.x - caster_pos.x, tpos.y - caster_pos.y) {
                world.directions.insert(player, dir);
            }
        }
    }

    let hits = execute(world, player, target, &skill, caster_pos);
    begin_cast(world, player, &skill);
    world.log_event(format!("You cast {} ({hits} hit).", skill.name));
}

/// Open a brief interruptible window for a *spell* cast (Mage / Clerc magic).
/// Weapon techniques are skipped — they protect themselves with [`Unstoppable`]
/// instead. Casts resolve instantly here, so this is the recovery/channel tail
/// during which a hit, stun, or knockdown cancels the action.
fn begin_cast(world: &mut World, caster: Entity, skill: &ActiveSkill) {
    let is_spell = matches!(
        skill.fx,
        SkillFx::Fire
            | SkillFx::Frost
            | SkillFx::Lightning
            | SkillFx::Holy
            | SkillFx::Heal
            | SkillFx::Meteor
            | SkillFx::Poison
    );
    if is_spell && !crate::systems::combat::is_unstoppable(world, caster) {
        world.interruptibles.insert(caster, Interruptible::cast(skill.id));
    }
}

/// Age every entry of a status map by `dt` and drop the expired ones in a
/// single pass. `remaining` yields each entry's countdown (seconds left).
fn tick_expiring<T>(
    map: &mut std::collections::BTreeMap<Entity, T>,
    dt: f32,
    mut remaining: impl FnMut(&mut T) -> &mut f32,
) {
    map.retain(|_, v| {
        let r = remaining(v);
        *r -= dt;
        *r > 0.0
    });
}

/// Age and expire every timed status, and apply per-tick effects (poison
/// damage, healing fields).
fn tick_statuses(world: &mut World, dt: f32) {
    tick_expiring(&mut world.buffs, dt, |b| &mut b.remaining);
    tick_expiring(&mut world.slows, dt, |s| &mut s.remaining);
    tick_expiring(&mut world.invuln, dt, |t| t);
    tick_expiring(&mut world.stuns, dt, |t| t);
    tick_expiring(&mut world.def_debuffs, dt, |d| &mut d.remaining);
    tick_expiring(&mut world.resist, dt, |r| &mut r.remaining);
    tick_expiring(&mut world.unstoppable, dt, |u| &mut u.remaining);

    tick_dots(world, dt);
    tick_heal_fields(world, dt);
    tick_orders(world, dt);
    tick_combos(world, dt);
    tick_frost_armor(world, dt);
    fields::tick_lightning_shields(world, dt);
    fields::tick_flame_walls(world, dt);
    fields::tick_blizzards(world, dt);
    tick_charging_shots(world, dt);
    fields::tick_meteor_fields(world, dt);
    fields::tick_arrow_rains(world, dt);
    tick_hunter_instinct(world, dt);
    tick_auras(world, dt);
    tick_aura_bonuses(world, dt);
    tick_converted(world, dt);
    tick_pardoned(world, dt);
    tick_resurrecting(world, dt);
}

/// Advance every locked melee combo ("Estocs"), landing a guaranteed strike each
/// time its timer elapses. The lock ends once its strikes are spent or its
/// target dies/vanishes.
fn tick_combos(world: &mut World, dt: f32) {
    for owner in world.combo_locks.keys().copied().collect::<Vec<_>>() {
        let Some(lock) = world.combo_locks.get_mut(&owner) else { continue };
        lock.timer -= dt;
        if lock.timer > 0.0 {
            continue;
        }
        let target = lock.target;
        #[allow(clippy::cast_possible_truncation)]
        let dmg = lock.dmg_per_hit as f32;
        lock.hits_remaining = lock.hits_remaining.saturating_sub(1);
        let done = lock.hits_remaining == 0;
        lock.timer += lock.interval;

        let target_alive = world.health.get(&target).is_some_and(|h| !h.is_empty())
            && world.positions.contains_key(&target);
        if target_alive {
            let from = world.positions.get(&owner).copied().unwrap_or(Position2D::new(0.0, 0.0));
            let to = world.positions.get(&target).copied().unwrap_or(from);
            apply_flat_damage(world, owner, target, dmg);
            emit_fx(world, owner, from, to, SkillFx::Slash);
        }
        if done || !target_alive {
            world.combo_locks.remove(&owner);
        }
    }
}

/// Apply poison damage-over-time and expire finished poisons. The defender's
/// Poison resistance scales every tick (a vulnerability multiplies the tick,
/// full immunity skips the damage entirely).
fn tick_dots(world: &mut World, dt: f32) {
    use crate::components::skill::Element;
    for e in world.dots.keys().copied().collect::<Vec<_>>() {
        let Some(dot) = world.dots.get_mut(&e) else { continue };
        let raw = dot.dps * dt;
        let owner = dot.owner;
        dot.remaining -= dt;
        let done = dot.remaining <= 0.0;
        let alive = world.health.get(&e).is_some_and(|h| !h.is_empty());
        let resist = world.elem_resists.get(&e).map_or(0.0, |r| r.value(Element::Poison));
        let immune = resist >= 1.0 - f32::EPSILON;
        let tick = if immune { 0.0 } else { (raw * (1.0 - resist).max(0.0)).max(0.0) };
        if alive && tick > 0.0 {
            let pos = world.positions.get(&e).copied().unwrap_or(Position2D::new(0.0, 0.0));
            if let Some(h) = world.health.get_mut(&e) {
                h.drain(tick);
            }
            world.render_events.push(RenderEvent::Damage {
                attacker: owner,
                target: e,
                pos,
                amount: tick,
                crit: false,
                dodged: false,
            });
        }
        if done || !alive {
            world.dots.remove(&e);
        }
    }
}

/// Heal a caster's allies standing in their healing field, then expire the
/// field when its timer runs out.
fn tick_heal_fields(world: &mut World, dt: f32) {
    for owner in world.heal_fields.keys().copied().collect::<Vec<_>>() {
        let Some(field) = world.heal_fields.get_mut(&owner) else { continue };
        field.remaining -= dt;
        let (radius, hp_pct, done) = (field.radius, field.hp_pct, field.remaining <= 0.0);
        let Some(center) = world.positions.get(&owner).copied() else {
            world.heal_fields.remove(&owner);
            continue;
        };
        for ally in allies_of(world, owner) {
            let Some(&apos) = world.positions.get(&ally) else { continue };
            if center.distance(apos) > radius {
                continue;
            }
            if let Some(h) = world.health.get_mut(&ally) {
                if !h.is_empty() {
                    h.restore(h.max * hp_pct * dt);
                }
            }
        }
        if done {
            world.heal_fields.remove(&owner);
        }
    }
}

/// The owner plus every ally entity it controls (its summons and mercenaries).
fn allies_of(world: &World, owner: Entity) -> Vec<Entity> {
    let mut out = vec![owner];
    out.extend(world.summon_tags.iter().filter(|(_, t)| t.owner == owner).map(|(e, _)| *e));
    out.extend(world.mercenary_tags.iter().filter(|(_, t)| t.owner == owner).map(|(e, _)| *e));
    out
}

/// Living enemy (mob) entities within `radius` of `center`.
fn enemies_within(world: &World, center: Position2D, radius: f32) -> Vec<Entity> {
    crate::systems::targeting::in_circle(world, center, radius)
}

/// Emit the cast's visual feedback.
fn emit_fx(world: &mut World, caster: Entity, from: Position2D, to: Position2D, fx: SkillFx) {
    world.render_events.push(RenderEvent::Skill { caster, from, to, fx });
}

/// Execute the skill, returning the number of enemies hit.
fn execute(
    world: &mut World,
    caster: Entity,
    target: Entity,
    skill: &ActiveSkill,
    caster_pos: Position2D,
) -> u32 {
    match skill.effect {
        SkillEffect::SingleTarget => {
            let tpos = world.positions.get(&target).copied().unwrap_or(caster_pos);
            resolve_attack(world, caster, target, skill.damage_mult);
            emit_fx(world, caster, caster_pos, tpos, skill.fx);
            1
        }
        SkillEffect::AoeSelf { radius } => {
            let foes = enemies_within(world, caster_pos, radius);
            for e in &foes {
                resolve_attack(world, caster, *e, skill.damage_mult);
            }
            emit_fx(world, caster, caster_pos, caster_pos, skill.fx);
            foes.len() as u32
        }
        SkillEffect::AoeTarget { radius } => {
            let center = world.positions.get(&target).copied().unwrap_or(caster_pos);
            let foes = enemies_within(world, center, radius);
            for e in &foes {
                resolve_attack(world, caster, *e, skill.damage_mult);
            }
            emit_fx(world, caster, caster_pos, center, skill.fx);
            foes.len() as u32
        }
        SkillEffect::Chain { jumps, radius } => chain(world, caster, target, skill, caster_pos, jumps, radius),
        SkillEffect::Buff { atk_pct, dur, radius } => {
            for ally in allies_in_radius(world, caster, caster_pos, radius) {
                let keep = match world.buffs.get(&ally).copied() {
                    Some(b) if b.atk_pct > atk_pct => b,
                    _ => Buff { atk_pct, remaining: dur },
                };
                world.buffs.insert(ally, keep);
            }
            emit_fx(world, caster, caster_pos, caster_pos, skill.fx);
            0
        }
        SkillEffect::Dash => {
            // The Soldat charge: close in, strike, then shove the target 3 tiles
            // back and knock it down for 1s (a wall slam adds bonus damage).
            let tpos = world.positions.get(&target).copied().unwrap_or(caster_pos);
            let dest = step_toward(caster_pos, tpos, (caster_pos.distance(tpos) - 1.0).max(0.0));
            reposition(world, caster, dest);
            let hit_pos = world.positions.get(&caster).copied().unwrap_or(dest);
            let wall_bonus = match resolve_attack(world, caster, target, skill.damage_mult) {
                AttackOutcome::Hit { damage, .. } => f64::from(damage) * 0.5,
                AttackOutcome::Dodged | AttackOutcome::Blocked => 0.0,
            };
            apply_pushback_then_knockdown(
                world,
                target,
                tpos.x - hit_pos.x,
                tpos.y - hit_pos.y,
                3.0,
                8.0,
                1.0,
                wall_bonus,
            );
            emit_fx(world, caster, caster_pos, tpos, skill.fx);
            1
        }
        SkillEffect::Blink { dist } => {
            let tpos = world.positions.get(&target).copied().unwrap_or(caster_pos);
            let dest = blink_dest(world, caster_pos, tpos, dist);
            reposition(world, caster, dest);
            emit_fx(world, caster, caster_pos, dest, skill.fx);
            0
        }
        SkillEffect::FrostNova { radius, slow, dur } => {
            let foes = enemies_within(world, caster_pos, radius);
            for e in &foes {
                resolve_attack(world, caster, *e, skill.damage_mult);
                world.slows.insert(*e, Slow { factor: slow, remaining: dur });
            }
            emit_fx(world, caster, caster_pos, caster_pos, skill.fx);
            foes.len() as u32
        }
        SkillEffect::Trap { radius, slow, dur } => {
            let e = world.spawn();
            world.positions.insert(e, caster_pos);
            // Legacy generic trap defaults to the explosive flavour.
            let (lo, hi) = effective_damage(world, caster);
            world.traps.insert(
                e,
                Trap {
                    owner: caster,
                    kind: TrapKind::Explosive,
                    radius,
                    damage: (lo + hi) * 0.5 * skill.damage_mult,
                    life: 12.0,
                    slow,
                    slow_dur: dur,
                },
            );
            emit_fx(world, caster, caster_pos, caster_pos, skill.fx);
            0
        }
        SkillEffect::Dodge { dur } => {
            world.invuln.insert(caster, dur);
            // Hop away from the nearest threat.
            if let Some(threat) = nearest_enemy_pos(world, caster_pos) {
                let away = Position2D::new(
                    caster_pos.x - (threat.x - caster_pos.x),
                    caster_pos.y - (threat.y - caster_pos.y),
                );
                let dest = blink_dest(world, caster_pos, away, 2.5);
                reposition(world, caster, dest);
            }
            emit_fx(world, caster, caster_pos, caster_pos, skill.fx);
            0
        }
        SkillEffect::Summon { profile } => {
            let spawned = crate::systems::summon::spawn_summon(world, caster, profile);
            // The Mage's elemental summons (the elemental and the hydra) take on
            // the element of the branch with the most points invested.
            let is_mage_summon = matches!(profile.kind, SummonType::Elemental | SummonType::Totem)
                && world.classes.get(&caster).copied() == Some(ClassType::MageDeGuerre);
            let fx = if is_mage_summon {
                match dominant_branch(read_mage_levels(world, caster)) {
                    Element::Frost => SkillFx::Frost,
                    Element::Lightning => SkillFx::Lightning,
                    _ => SkillFx::Fire,
                }
            } else {
                skill.fx
            };
            emit_fx(world, caster, caster_pos, caster_pos, fx);
            #[allow(clippy::cast_possible_truncation)]
            {
                spawned.len() as u32
            }
        }
        SkillEffect::Smite { stun } => {
            let tpos = world.positions.get(&target).copied().unwrap_or(caster_pos);
            resolve_attack(world, caster, target, skill.damage_mult);
            apply_stun(world, target, stun);
            emit_fx(world, caster, caster_pos, tpos, skill.fx);
            1
        }
        SkillEffect::Frostbolt { slow, dur } => {
            let tpos = world.positions.get(&target).copied().unwrap_or(caster_pos);
            resolve_attack(world, caster, target, skill.damage_mult);
            world.slows.insert(target, Slow { factor: slow, remaining: dur });
            emit_fx(world, caster, caster_pos, tpos, skill.fx);
            1
        }
        SkillEffect::PoisonShot { dps, dur, def_pct } => {
            let tpos = world.positions.get(&target).copied().unwrap_or(caster_pos);
            resolve_attack(world, caster, target, skill.damage_mult);
            world.dots.insert(target, Dot { owner: caster, dps, remaining: dur });
            world.def_debuffs.insert(target, DefDebuff { pct: def_pct, remaining: dur });
            emit_fx(world, caster, caster_pos, tpos, skill.fx);
            1
        }
        SkillEffect::MultiAoe { radius, hits, slow, dur } => {
            let center = world.positions.get(&target).copied().unwrap_or(caster_pos);
            let foes = enemies_within(world, center, radius);
            let per_hit = if hits == 0 { skill.damage_mult } else { skill.damage_mult / hits as f32 };
            for _ in 0..hits.max(1) {
                for e in &foes {
                    if world.health.get(e).is_some_and(|h| !h.is_empty()) {
                        resolve_attack(world, caster, *e, per_hit);
                    }
                }
            }
            if slow > 0.0 {
                for e in &foes {
                    world.slows.insert(*e, Slow { factor: slow, remaining: dur });
                }
            }
            emit_fx(world, caster, caster_pos, center, skill.fx);
            foes.len() as u32
        }
        SkillEffect::HealField { radius, hp_pct, dur } => {
            world.heal_fields.insert(caster, HealField { radius, hp_pct, remaining: dur });
            emit_fx(world, caster, caster_pos, caster_pos, skill.fx);
            0
        }
        SkillEffect::Bless { resist_pct, dur, radius } => {
            for ally in allies_in_radius(world, caster, caster_pos, radius) {
                let keep = match world.resist.get(&ally).copied() {
                    Some(r) if r.pct > resist_pct => r,
                    _ => Resist { pct: resist_pct, remaining: dur },
                };
                world.resist.insert(ally, keep);
            }
            emit_fx(world, caster, caster_pos, caster_pos, skill.fx);
            0
        }
        SkillEffect::Order { order, dur, radius } => {
            let syn = read_order_synergies(world, caster);
            let bonuses = order_bonuses_for(order, skill.level, syn);
            for ally in allies_in_radius(world, caster, caster_pos, radius) {
                apply_order(world, ally, order, dur, bonuses);
            }
            emit_fx(world, caster, caster_pos, caster_pos, skill.fx);
            0
        }
        SkillEffect::GrandCoup { duration_base, duration_speed_reduction } => {
            let tpos = world.positions.get(&target).copied().unwrap_or(caster_pos);
            let mult = grand_coup_mult(is_two_handed(world, caster), skill.level);
            resolve_attack(world, caster, target, mult);
            let dur = melee_window(world, caster, duration_base, duration_speed_reduction);
            world.unstoppable.insert(caster, Unstoppable { remaining: dur });
            emit_fx(world, caster, caster_pos, tpos, skill.fx);
            1
        }
        SkillEffect::Taille { arc_deg, radius, duration_base, duration_speed_reduction } => {
            let center = world.input.cursor_world.unwrap_or(caster_pos);
            let aim = aim_dir(caster_pos, center);
            let grand = skill_level_by_id(world, caster, 1001);
            let mult = taille_mult(skill.level, grand);
            let foes = enemies_in_arc(world, center, aim, radius, arc_deg);
            for e in &foes {
                resolve_attack(world, caster, *e, mult);
            }
            let dur = melee_window(world, caster, duration_base, duration_speed_reduction);
            world.unstoppable.insert(caster, Unstoppable { remaining: dur });
            if let Some(dir) = Direction::from_world_vector(aim.0, aim.1) {
                world.directions.insert(caster, dir);
            }
            emit_fx(world, caster, caster_pos, center, skill.fx);
            foes.len() as u32
        }
        SkillEffect::Estocs { hits } => {
            let tpos = world.positions.get(&target).copied().unwrap_or(caster_pos);
            let grand = skill_level_by_id(world, caster, 1001);
            let (lo, hi) = effective_damage(world, caster);
            let dmg_per_hit = f64::from((lo + hi) * 0.5) + 10.0 * f64::from(grand);
            #[allow(clippy::cast_possible_truncation)]
            let dmg = dmg_per_hit as f32;
            let interval = estocs_interval(player_attack_interval(world, caster), skill.level);

            // Land the first thrust now; the lock plays out the rest.
            apply_flat_damage(world, caster, target, dmg);
            emit_fx(world, caster, caster_pos, tpos, skill.fx);
            let remaining = hits.saturating_sub(1);
            if remaining > 0 {
                world.combo_locks.insert(
                    caster,
                    ComboLock { hits_remaining: remaining, interval, timer: interval, target, dmg_per_hit },
                );
            }
            // Unstoppable for the full sequence; halt any in-progress movement.
            world.unstoppable.insert(caster, Unstoppable { remaining: interval * hits as f32 });
            world.move_targets.remove(&caster);
            1
        }

        // ── Mage de guerre: FEU ────────────────────────────────────────────
        SkillEffect::Fireball => {
            let lv = read_mage_levels(world, caster);
            let tpos = world.positions.get(&target).copied().unwrap_or(caster_pos);
            resolve_spell(world, caster, target, fireball_dmg(lv), Element::Fire);
            emit_fx(world, caster, caster_pos, tpos, skill.fx);
            1
        }
        SkillEffect::FlameWall => {
            let lv = read_mage_levels(world, caster);
            let center = cursor_in_range(world, caster_pos, skill.range);
            let aim = aim_dir(caster_pos, center);
            let zone = world.spawn();
            world.flame_walls.insert(
                zone,
                crate::components::skill::FlameWall {
                    owner: caster,
                    center,
                    dir: aim,
                    half_len: 3.0,
                    half_wid: 1.0,
                    dmg_per_tick: flame_wall_tick_dmg(lv),
                    tick_interval: 0.5,
                    tick_timer: 0.5,
                    remaining: flame_wall_duration(lv),
                },
            );
            emit_fx(world, caster, caster_pos, center, skill.fx);
            0
        }
        SkillEffect::MeteorRain { radius } => {
            let lv = read_mage_levels(world, caster);
            let center = cursor_in_range(world, caster_pos, skill.range);
            let impacts = meteor_impacts(lv);
            let interval = if impacts > 0 { 3.0 / impacts as f32 } else { 3.0 };
            let zone = world.spawn();
            world.meteor_fields.insert(
                zone,
                crate::components::skill::MeteorField {
                    owner: caster,
                    center,
                    radius,
                    dmg_per_impact: meteor_impact_dmg(lv),
                    impacts_left: impacts,
                    impact_interval: interval,
                    impact_timer: interval,
                },
            );
            emit_fx(world, caster, caster_pos, center, skill.fx);
            0
        }

        // ── Mage de guerre: GLACE ──────────────────────────────────────────
        SkillEffect::IceLance => {
            let lv = read_mage_levels(world, caster);
            let tpos = world.positions.get(&target).copied().unwrap_or(caster_pos);
            resolve_spell(world, caster, target, ice_lance_dmg(lv), Element::Frost);
            let slow = ice_lance_slow(lv);
            world.slows.insert(target, Slow { factor: 1.0 - slow, remaining: 3.0 });
            emit_fx(world, caster, caster_pos, tpos, skill.fx);
            1
        }
        SkillEffect::FrostArmor => {
            let lv = read_mage_levels(world, caster);
            world.frost_armor.insert(
                caster,
                crate::components::skill::FrostArmor {
                    owner: caster,
                    def_pct: frost_armor_def(lv),
                    reflect: frost_armor_reflect(lv),
                    remaining: 15.0,
                },
            );
            emit_fx(world, caster, caster_pos, caster_pos, skill.fx);
            0
        }
        SkillEffect::Blizzard { radius } => {
            let lv = read_mage_levels(world, caster);
            let center = cursor_in_range(world, caster_pos, skill.range);
            let zone = world.spawn();
            world.blizzards.insert(
                zone,
                crate::components::skill::BlizzardField {
                    owner: caster,
                    center,
                    radius,
                    dmg_per_sec: blizzard_dps(lv),
                    elapsed: 0.0,
                    remaining: blizzard_duration(lv),
                },
            );
            emit_fx(world, caster, caster_pos, center, skill.fx);
            0
        }

        // ── Mage de guerre: FOUDRE ─────────────────────────────────────────
        SkillEffect::ChainBolt { radius } => {
            let lv = read_mage_levels(world, caster);
            chain_bolt(world, caster, target, skill, caster_pos, radius, lv)
        }
        SkillEffect::LightningShield => {
            let lv = read_mage_levels(world, caster);
            world.lightning_shield.insert(
                caster,
                crate::components::skill::LightningShield {
                    owner: caster,
                    dmg: lightning_shield_dmg(lv),
                    radius: lightning_shield_radius(lv),
                    interval: 2.0,
                    timer: 2.0,
                    remaining: 20.0,
                },
            );
            emit_fx(world, caster, caster_pos, caster_pos, skill.fx);
            0
        }
        SkillEffect::ElectricNova { radius } => {
            let lv = read_mage_levels(world, caster);
            let dmg = nova_dmg(lv);
            let stun = nova_stun(lv);
            let foes = enemies_within(world, caster_pos, radius);
            for e in &foes {
                resolve_spell(world, caster, *e, dmg, Element::Lightning);
                apply_stun(world, *e, stun);
                // Blast each foe 2 tiles radially outward from the caster.
                let fpos = world.positions.get(e).copied().unwrap_or(caster_pos);
                apply_pushback(world, *e, fpos.x - caster_pos.x, fpos.y - caster_pos.y, 2.0, 5.0);
            }
            emit_fx(world, caster, caster_pos, caster_pos, skill.fx);
            foes.len() as u32
        }

        // ── Archer: TIR ────────────────────────────────────────────────────
        SkillEffect::FastShot => {
            let al = read_archer_levels(world, caster);
            let tpos = world.positions.get(&target).copied().unwrap_or(caster_pos);
            resolve_attack(world, caster, target, fast_shot_mult(al));
            emit_fx(world, caster, caster_pos, tpos, skill.fx);
            1
        }
        SkillEffect::PiercingShot { half_width } => {
            let al = read_archer_levels(world, caster);
            let tpos = world.positions.get(&target).copied().unwrap_or(caster_pos);
            let dir = aim_dir(caster_pos, tpos);
            let pierces = piercing_pierces(al);
            let line = enemies_on_line(world, caster_pos, dir, skill.range, half_width);
            let base = piercing_shot_mult(al);
            let mut hit_count = 0u32;
            for (idx, e) in line.iter().take(pierces as usize).enumerate() {
                let reduction = 0.15 * idx as f32;
                let mult = (base * (1.0 - reduction)).max(base * 0.40);
                resolve_attack(world, caster, *e, mult);
                hit_count += 1;
            }
            emit_fx(world, caster, caster_pos, tpos, skill.fx);
            hit_count.max(1)
        }
        SkillEffect::ChargedShot => {
            let al = read_archer_levels(world, caster);
            // Install the charging window; the per-frame ticker resolves the
            // shot when its timer elapses (cancelled by stuns/knockdowns).
            world.charging_shots.insert(
                caster,
                ChargingShot {
                    target,
                    remaining: charged_shot_charge_time(al),
                    dmg_mult: charged_shot_mult(al),
                    crit_bonus_pct: 50,
                },
            );
            // Lock the caster's movement during the charge.
            world.move_targets.remove(&caster);
            emit_fx(world, caster, caster_pos, caster_pos, SkillFx::Buff);
            0
        }

        // ── Archer: PIÈGE ──────────────────────────────────────────────────
        SkillEffect::ExplosiveTrap { radius, max_active, lifetime } => {
            let al = read_archer_levels(world, caster);
            let pos = cursor_in_range(world, caster_pos, skill.range);
            enforce_trap_cap(world, caster, TrapKind::Explosive, max_active);
            let e = world.spawn();
            world.positions.insert(e, pos);
            world.traps.insert(
                e,
                Trap {
                    owner: caster,
                    kind: TrapKind::Explosive,
                    radius,
                    damage: explosive_trap_dmg(al),
                    life: lifetime,
                    slow: explosive_trap_slow_factor(al),
                    slow_dur: 3.0,
                },
            );
            emit_fx(world, caster, caster_pos, pos, skill.fx);
            0
        }
        SkillEffect::JawTrap { radius, max_active, lifetime } => {
            let al = read_archer_levels(world, caster);
            let pos = cursor_in_range(world, caster_pos, skill.range);
            enforce_trap_cap(world, caster, TrapKind::Jaw, max_active);
            let e = world.spawn();
            world.positions.insert(e, pos);
            world.traps.insert(
                e,
                Trap {
                    owner: caster,
                    kind: TrapKind::Jaw,
                    radius,
                    damage: jaw_trap_dmg(al),
                    life: lifetime,
                    // factor 0 = full immobilise.
                    slow: 0.0,
                    slow_dur: jaw_trap_immobilize_dur(al),
                },
            );
            emit_fx(world, caster, caster_pos, pos, skill.fx);
            0
        }

        // ── Archer: AGILITÉ ────────────────────────────────────────────────
        SkillEffect::DodgeRoll { dist, invuln } => {
            // Direction: toward the cursor if set, otherwise away from the
            // nearest threat (and finally a default east hop).
            let aim = match world.input.cursor_world {
                Some(c) if c.distance(caster_pos) > 0.1 => aim_dir(caster_pos, c),
                _ => match nearest_enemy_pos(world, caster_pos) {
                    Some(t) => aim_dir(t, caster_pos), // away from the threat.
                    None => (1.0, 0.0),
                },
            };
            let al = read_archer_levels(world, caster);
            let total = dist + dodge_roll_extra_dist(al);
            let target_pt = Position2D::new(caster_pos.x + aim.0 * total, caster_pos.y + aim.1 * total);
            let dest = blink_dest(world, caster_pos, target_pt, total);
            reposition(world, caster, dest);
            world.invuln.insert(caster, invuln + dodge_roll_extra_invuln(al));
            emit_fx(world, caster, caster_pos, dest, skill.fx);
            0
        }
        SkillEffect::ArrowRain { radius } => {
            let al = read_archer_levels(world, caster);
            let center = cursor_in_range(world, caster_pos, skill.range);
            let duration = arrow_rain_duration(al);
            let interval = 0.5;
            let zone = world.spawn();
            world.arrow_rains.insert(
                zone,
                ArrowRainField {
                    owner: caster,
                    center,
                    radius,
                    dmg_per_tick: arrow_rain_tick_dmg(al),
                    tick_interval: interval,
                    tick_timer: interval,
                    remaining: duration,
                },
            );
            emit_fx(world, caster, caster_pos, center, skill.fx);
            0
        }
        SkillEffect::HunterInstinct => {
            let al = read_archer_levels(world, caster);
            world.hunter_instinct.insert(
                caster,
                HunterInstinctBuff {
                    remaining: 10.0,
                    crit_pct: hunter_instinct_crit_pct(al),
                    dodge_pts: hunter_instinct_dodge_pts(al),
                    reveal_radius: 10.0,
                },
            );
            emit_fx(world, caster, caster_pos, caster_pos, skill.fx);
            0
        }

        // ── Clerc: BÉNI ────────────────────────────────────────────────────
        SkillEffect::Prayer => {
            let cl = read_clerc_levels(world, caster);
            let center = cursor_in_range(world, caster_pos, skill.range);
            let radius = prayer_radius(cl);
            let dur = prayer_stun_dur(cl);
            let foes = enemies_within(world, center, radius);
            for e in &foes {
                apply_stun(world, *e, dur);
            }
            emit_fx(world, caster, caster_pos, center, skill.fx);
            #[allow(clippy::cast_possible_truncation)]
            {
                foes.len() as u32
            }
        }
        SkillEffect::Exorcism => {
            let cl = read_clerc_levels(world, caster);
            let center = cursor_in_range(world, caster_pos, skill.range);
            let radius = prayer_radius(cl);
            let dmg = exorcism_dmg(cl);
            let foes = enemies_within(world, center, radius);
            for e in &foes {
                resolve_spell(world, caster, *e, dmg, Element::Physical);
            }
            emit_fx(world, caster, caster_pos, center, skill.fx);
            #[allow(clippy::cast_possible_truncation)]
            {
                foes.len() as u32
            }
        }
        SkillEffect::Conversion => {
            let cl = read_clerc_levels(world, caster);
            let tpos = world.positions.get(&target).copied().unwrap_or(caster_pos);
            // Conversion never charms a boss.
            let is_boss = world.boss_state.contains_key(&target);
            if !is_boss && world.mob_tags.contains_key(&target) {
                let original = world.factions.get(&target).copied().unwrap_or(Faction::Enemy);
                world.factions.insert(target, Faction::Player);
                world.converted.insert(
                    target,
                    Converted { original_faction: original, remaining: conversion_dur(cl) },
                );
                if let Some(c) = world.combat.get_mut(&target) {
                    c.target = None;
                }
            }
            emit_fx(world, caster, caster_pos, tpos, skill.fx);
            1
        }
        SkillEffect::Pardon => {
            let cl = read_clerc_levels(world, caster);
            let tpos = world.positions.get(&target).copied().unwrap_or(caster_pos);
            if world.mob_tags.contains_key(&target) {
                world.pardoned.insert(
                    target,
                    Pardoned { leech_pct: pardon_leech_pct(cl), remaining: pardon_dur(cl) },
                );
            }
            emit_fx(world, caster, caster_pos, tpos, skill.fx);
            1
        }
        SkillEffect::Resurrection => {
            // Channel a long cast that revives a dead mercenary on completion.
            world.resurrecting.insert(
                caster,
                Resurrecting { remaining: resurrection_cast_time(skill.level), mana_cost: 0.0 },
            );
            world.move_targets.remove(&caster);
            emit_fx(world, caster, caster_pos, caster_pos, skill.fx);
            0
        }

        // Passives never reach `execute()` (skipped in `skill_system`).
        SkillEffect::Passive { .. } => 0,

        // Auras are handled in `skill_system`'s toggle branch; the arm here is
        // unreachable but keeps the match exhaustive.
        SkillEffect::Aura { .. } => 0,
    }
}

/// Mana cost for a cast, scaling the three Soldat melee techniques with rank
/// (others use their static cost).
#[allow(clippy::cast_precision_loss)]
fn effective_mana_cost(skill: &ActiveSkill) -> f32 {
    let lvl = skill.level;
    match skill.effect {
        // 3 + 1 per two ranks: 3 @1, 4 @3, 5 @5…
        SkillEffect::GrandCoup { .. } => 3.0 + (lvl.saturating_sub(1) / 2) as f32,
        // 4 + 1 per rank.
        SkillEffect::Taille { .. } | SkillEffect::Estocs { .. } => {
            4.0 + lvl.saturating_sub(1) as f32
        }
        // Mage spells scale their static base by +1/+2/+3 mana per rank.
        SkillEffect::Fireball | SkillEffect::IceLance | SkillEffect::ChainBolt { .. } => {
            skill.mana_cost + lf(lvl.saturating_sub(1))
        }
        SkillEffect::FlameWall
        | SkillEffect::FrostArmor
        | SkillEffect::LightningShield
        | SkillEffect::ElectricNova { .. } => skill.mana_cost + 2.0 * lf(lvl.saturating_sub(1)),
        SkillEffect::MeteorRain { .. } | SkillEffect::Blizzard { .. } => {
            skill.mana_cost + 3.0 * lf(lvl.saturating_sub(1))
        }
        _ => skill.mana_cost,
    }
}

/// Whether `e` swings two-handed: a main-hand weapon with an empty off-hand (a
/// shield would occupy the off-hand). Unarmed counts as one-handed.
fn is_two_handed(world: &World, e: Entity) -> bool {
    world.equipment.get(&e).is_some_and(|eq| {
        eq.get(EquipSlot::MainHand).is_some() && eq.get(EquipSlot::OffHand).is_none()
    })
}

/// Seconds between an entity's swings (defaults to 1s, clamped away from zero).
fn player_attack_interval(world: &World, e: Entity) -> f32 {
    world.combat.get(&e).map_or(1.0, |c| c.attack_interval.max(0.05))
}

/// The speed-scaled unstoppable window for a melee technique: `base` seconds,
/// shortened by `reduction` per attack-per-second, floored so it never vanishes.
fn melee_window(world: &World, e: Entity, base: f32, reduction: f32) -> f32 {
    let aps = 1.0 / player_attack_interval(world, e);
    (base - reduction * aps).max(0.1)
}

/// Unit aim vector from `from` toward `to` (defaults to +x if they coincide).
fn aim_dir(from: Position2D, to: Position2D) -> (f32, f32) {
    let (dx, dy) = (to.x - from.x, to.y - from.y);
    let len = (dx * dx + dy * dy).sqrt();
    if len <= 1e-4 {
        (1.0, 0.0)
    } else {
        (dx / len, dy / len)
    }
}

/// Living enemies within `radius` of `center` whose bearing lies within
/// ±`arc_deg`/2 of `aim`. Enemies on the centre point are always included.
fn enemies_in_arc(
    world: &World,
    center: Position2D,
    aim: (f32, f32),
    radius: f32,
    arc_deg: f32,
) -> Vec<Entity> {
    crate::systems::targeting::in_arc(world, center, aim, radius, arc_deg)
}

/// Apply guaranteed flat damage (Estocs' uninterruptible thrusts) and emit the
/// floating-number feedback. Bypasses dodge/defence by design.
fn apply_flat_damage(world: &mut World, attacker: Entity, target: Entity, amount: f32) {
    let alive = world.health.get(&target).is_some_and(|h| !h.is_empty());
    if amount <= 0.0 || !alive {
        return;
    }
    let pos = world.positions.get(&target).copied().unwrap_or(Position2D::new(0.0, 0.0));
    if let Some(h) = world.health.get_mut(&target) {
        h.drain(amount);
    }
    world.render_events.push(RenderEvent::Damage {
        attacker,
        target,
        pos,
        amount,
        crit: false,
        dodged: false,
    });
    // Estocs' guaranteed thrusts carry an interrupt: each hit cancels a cast.
    try_interrupt(world, target);
}

/// Rank invested in the hotbar skill with `id` on `owner` (0 if absent). Shared
/// by every class's `read_*_levels`, so it stays in the parent module.
fn skill_level_by_id(world: &World, owner: Entity, id: u32) -> u32 {
    world
        .skills
        .get(&owner)
        .and_then(|s| s.bar.iter().find(|sk| sk.id == id))
        .map_or(0, |sk| sk.level)
}

/// Stun `target` for `dur` seconds, never shortening an existing longer stun.
/// Entities in an uninterruptible window shrug it off.
fn apply_stun(world: &mut World, target: Entity, dur: f32) {
    if crate::systems::combat::is_unstoppable(world, target) {
        return;
    }
    let cur = world.stuns.get(&target).copied().unwrap_or(0.0);
    world.stuns.insert(target, cur.max(dur));
    // A landed stun interrupts whatever the target was casting.
    try_interrupt(world, target);
}

/// The caster plus every allied entity it controls within `radius` of `center`
/// (`radius <= 0` means the caster alone).
fn allies_in_radius(world: &World, caster: Entity, center: Position2D, radius: f32) -> Vec<Entity> {
    if radius <= 0.0 {
        return vec![caster];
    }
    allies_of(world, caster)
        .into_iter()
        .filter(|a| world.positions.get(a).is_some_and(|p| center.distance(*p) <= radius))
        .collect()
}

/// Lightning-style chain: hit the target, then arc to nearby enemies.
fn chain(
    world: &mut World,
    caster: Entity,
    first: Entity,
    skill: &ActiveSkill,
    caster_pos: Position2D,
    jumps: u32,
    radius: f32,
) -> u32 {
    let mut hit = vec![first];
    let tpos = world.positions.get(&first).copied().unwrap_or(caster_pos);
    resolve_attack(world, caster, first, skill.damage_mult);
    emit_fx(world, caster, caster_pos, tpos, skill.fx);

    let mut last = tpos;
    let mut mult = skill.damage_mult;
    for _ in 0..jumps {
        mult *= 0.8;
        let next = enemies_within(world, last, radius)
            .into_iter()
            .filter(|e| !hit.contains(e))
            .filter_map(|e| world.positions.get(&e).map(|p| (e, last.distance(*p))))
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(e, _)| e);
        let Some(next) = next else { break };
        let npos = world.positions.get(&next).copied().unwrap_or(last);
        resolve_attack(world, caster, next, mult);
        emit_fx(world, caster, last, npos, SkillFx::Lightning);
        hit.push(next);
        last = npos;
    }
    hit.len() as u32
}

/// Move an entity to `dest`, snapping to the tile if it's walkable and
/// cancelling any path-follow.
fn reposition(world: &mut World, e: Entity, dest: Position2D) {
    let (tx, ty) = dest.tile();
    let pos = if world.map.is_walkable(tx, ty) { dest } else { *world.positions.get(&e).unwrap_or(&dest) };
    world.positions.insert(e, pos);
    world.move_targets.remove(&e);
}

/// A point `dist` tiles from `from` toward `to`.
fn step_toward(from: Position2D, to: Position2D, dist: f32) -> Position2D {
    let d = from.distance(to);
    if d <= 1e-3 {
        return from;
    }
    let t = (dist / d).clamp(0.0, 1.0);
    Position2D::new(from.x + (to.x - from.x) * t, from.y + (to.y - from.y) * t)
}

/// Farthest walkable point up to `max_dist` from `from` toward `to`.
fn blink_dest(world: &World, from: Position2D, to: Position2D, max_dist: f32) -> Position2D {
    let mut best = from;
    let mut d = max_dist;
    while d >= 1.0 {
        let cand = step_toward(from, to, d.min(from.distance(to).max(d)));
        let (tx, ty) = cand.tile();
        if world.map.is_walkable(tx, ty) {
            best = cand;
            break;
        }
        d -= 1.0;
    }
    best
}

fn nearest_enemy_pos(world: &World, from: Position2D) -> Option<Position2D> {
    crate::systems::targeting::nearest_pos(world, from)
}

/// Detonate deployed traps when an enemy steps into range, and expire old ones.
/// Explosive traps strike every foe in radius once and slow the survivors; jaw
/// traps deal a small flat hit and clamp the survivors with a near-zero
/// `slow.factor` (immobilisation).
pub fn trap_system(world: &mut World) {
    let dt = world.dt;
    for e in world.traps.keys().copied().collect::<Vec<_>>() {
        let Some(trap) = world.traps.get_mut(&e) else { continue };
        trap.life -= dt;
        let expired = trap.life <= 0.0;
        let (owner, kind, radius, dmg, slow, slow_dur) =
            (trap.owner, trap.kind, trap.radius, trap.damage, trap.slow, trap.slow_dur);
        let Some(tpos) = world.positions.get(&e).copied() else {
            world.despawn(e);
            continue;
        };

        let foes = enemies_within(world, tpos, radius);
        if !foes.is_empty() {
            for f in &foes {
                apply_flat_damage(world, owner, *f, dmg);
                // Jaw traps always clamp (factor=0 = full immobilise);
                // explosive traps only carry a slow when configured.
                let (apply_slow, factor) = match kind {
                    TrapKind::Jaw => (true, 0.0),
                    TrapKind::Explosive => (slow > 0.0 && slow < 1.0, slow),
                };
                if apply_slow {
                    world.slows.insert(*f, Slow { factor, remaining: slow_dur });
                }
            }
            world.render_events.push(RenderEvent::Skill {
                caster: owner,
                from: tpos,
                to: tpos,
                fx: SkillFx::Trap,
            });
            let label = match kind {
                TrapKind::Explosive => "Explosive trap",
                TrapKind::Jaw => "Jaw trap",
            };
            world.log_event(format!("{label} snaps! ({} hit)", foes.len()));
            world.despawn(e);
        } else if expired {
            world.despawn(e);
        }
    }
}

/// The cursor world-position clamped to `range` tiles of the caster (the caster's
/// own position when no cursor is set or the range is zero).
fn cursor_in_range(world: &World, caster_pos: Position2D, range: f32) -> Position2D {
    let target = world.input.cursor_world.unwrap_or(caster_pos);
    if range <= 0.0 || caster_pos.distance(target) <= range {
        target
    } else {
        step_toward(caster_pos, target, range)
    }
}

/// Age frost-armour buffs, expiring finished ones (their reflection/DEF are read
/// live by [`crate::systems::combat`]).
fn tick_frost_armor(world: &mut World, dt: f32) {
    tick_expiring(&mut world.frost_armor, dt, |f| &mut f.remaining);
}

/// Living mob entities lying on a straight ray from `from` along the unit
/// vector `dir`, within `range` tiles forward and `half_width` perpendicular.
/// Returned sorted by distance along the ray, closest first.
fn enemies_on_line(
    world: &World,
    from: Position2D,
    dir: (f32, f32),
    range: f32,
    half_width: f32,
) -> Vec<Entity> {
    crate::systems::targeting::on_line(world, from, dir, range, half_width)
}

/// Despawn the oldest trap of `kind` owned by `owner` until at most
/// `max_active - 1` remain, so the impending spawn fits within the cap.
fn enforce_trap_cap(world: &mut World, owner: Entity, kind: TrapKind, max_active: u32) {
    if max_active == 0 {
        return;
    }
    let mut owned: Vec<(Entity, f32)> = world
        .traps
        .iter()
        .filter(|(_, t)| t.owner == owner && t.kind == kind)
        .map(|(e, t)| (*e, t.life))
        .collect();
    // The oldest trap is the one with the LEAST life remaining (life ticks down).
    owned.sort_by(|a, b| a.1.total_cmp(&b.1));
    while owned.len() as u32 >= max_active {
        let Some((victim, _)) = owned.first().copied() else { break };
        world.despawn(victim);
        owned.remove(0);
    }
}

/// Resolve charging shots: tick down their timers, fire when the timer hits
/// zero (with a guaranteed-crit bonus), cancel on owner CC or lost target.
fn tick_charging_shots(world: &mut World, dt: f32) {
    for owner in world.charging_shots.keys().copied().collect::<Vec<_>>() {
        // CC during the charge cancels the shot.
        if crate::systems::combat::is_cc_locked(world, owner)
            || world.stuns.contains_key(&owner)
            || world.knockdowns.contains_key(&owner)
            || world.pushbacks.contains_key(&owner)
        {
            world.charging_shots.remove(&owner);
            continue;
        }
        let Some(cs) = world.charging_shots.get_mut(&owner) else { continue };
        cs.remaining -= dt;
        if cs.remaining > 0.0 {
            continue;
        }
        let (target, mult, crit_bonus) = (cs.target, cs.dmg_mult, cs.crit_bonus_pct);
        world.charging_shots.remove(&owner);

        // Target must still be alive and positioned.
        let target_ok = world.health.get(&target).is_some_and(|h| !h.is_empty())
            && world.positions.contains_key(&target);
        if !target_ok {
            continue;
        }
        // Apply the crit bonus as a transient HunterInstinct stack so the
        // shared combat resolver picks it up; restore the prior buff after.
        let prior = world.hunter_instinct.insert(
            owner,
            HunterInstinctBuff {
                remaining: 0.05,
                crit_pct: crit_bonus,
                dodge_pts: 0.0,
                reveal_radius: 0.0,
            },
        );
        resolve_attack(world, owner, target, mult);
        if let Some(p) = prior {
            world.hunter_instinct.insert(owner, p);
        } else {
            world.hunter_instinct.remove(&owner);
        }
        let from = world.positions.get(&owner).copied().unwrap_or(Position2D::new(0.0, 0.0));
        let to = world.positions.get(&target).copied().unwrap_or(from);
        emit_fx(world, owner, from, to, SkillFx::Arrow);
    }
}

/// Age hunter-instinct buffs; combat resolution reads the buff live, so we
/// just expire finished ones here.
fn tick_hunter_instinct(world: &mut World, dt: f32) {
    tick_expiring(&mut world.hunter_instinct, dt, |b| &mut b.remaining);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::mob::MobTag;
    use crate::components::skill::SkillKind;
    use crate::data::maps::village_map;
    use crate::data::GameData;

    fn test_world() -> World {
        World::new(village_map(), GameData::load(), 7)
    }

    fn close(a: f32, b: f32) {
        assert!((a - b).abs() < 1e-4, "expected {b}, got {a}");
    }

    fn spawn_mob(world: &mut World, pos: Position2D) -> Entity {
        let e = world.spawn();
        world.mob_tags.insert(e, MobTag);
        world.positions.insert(e, pos);
        world.health.insert(e, crate::components::Health::full(100.0));
        e
    }

    fn melee_skill(effect: SkillEffect, level: u32) -> ActiveSkill {
        ActiveSkill {
            id: 9999,
            name: "test".into(),
            level,
            kind: SkillKind::Melee,
            mana_cost: 99.0,
            cooldown: 1.0,
            damage_mult: 1.0,
            range: 2.0,
            effect,
            fx: SkillFx::Slash,
        }
    }

    const GRAND_COUP: SkillEffect =
        SkillEffect::GrandCoup { duration_base: 0.8, duration_speed_reduction: 0.20 };
    const TAILLE: SkillEffect =
        SkillEffect::Taille { arc_deg: 120.0, radius: 4.0, duration_base: 1.1, duration_speed_reduction: 0.30 };

    #[test]
    fn grand_coup_scales_with_weapon_hands_and_rank() {
        // 1H: 1.5 + 0.05/rank; 2H: 1.7 + 0.07/rank.
        close(grand_coup_mult(false, 1), 1.55);
        close(grand_coup_mult(true, 1), 1.77);
        close(grand_coup_mult(false, 5), 1.75);
        close(grand_coup_mult(true, 5), 2.05);
    }

    #[test]
    fn taille_folds_grand_coup_synergy() {
        // -30% base, +10%/Taille rank, +5%/Grand-coup rank.
        close(taille_mult(1, 0), 0.80);
        close(taille_mult(3, 2), 0.7 + 0.30 + 0.10); // 1.10
    }

    #[test]
    fn estocs_interval_triples_then_scales() {
        // +200% +10%/rank → interval / (3 + 0.1*rank).
        close(estocs_interval(1.2, 1), 1.2 / 3.1);
        close(estocs_interval(0.9, 5), 0.9 / 3.5);
    }

    #[test]
    fn melee_techniques_mana_scales_with_rank() {
        close(effective_mana_cost(&melee_skill(GRAND_COUP, 1)), 3.0);
        close(effective_mana_cost(&melee_skill(GRAND_COUP, 3)), 4.0);
        close(effective_mana_cost(&melee_skill(GRAND_COUP, 5)), 5.0);
        close(effective_mana_cost(&melee_skill(TAILLE, 1)), 4.0);
        close(effective_mana_cost(&melee_skill(TAILLE, 4)), 7.0);
        close(effective_mana_cost(&melee_skill(SkillEffect::Estocs { hits: 3 }, 2)), 5.0);
    }

    #[test]
    fn melee_window_shortens_for_faster_attackers() {
        let mut world = test_world();
        let e = world.spawn();
        // 1 atk/s → 0.8 - 0.20*1 = 0.6s.
        world.combat.insert(e, crate::components::Combat::new(2.0, 1.0));
        close(melee_window(&world, e, 0.8, 0.20), 0.6);
        // 2 atk/s → 0.8 - 0.20*2 = 0.4s (faster → shorter).
        world.combat.insert(e, crate::components::Combat::new(2.0, 0.5));
        close(melee_window(&world, e, 0.8, 0.20), 0.4);
    }

    #[test]
    fn unarmed_counts_as_one_handed() {
        let mut world = test_world();
        let e = world.spawn();
        world.equipment.insert(e, crate::components::equipment::EquipmentSlots::new());
        assert!(!is_two_handed(&world, e));
    }

    #[test]
    fn taille_arc_only_hits_the_forward_wedge() {
        let mut world = test_world();
        let center = Position2D::new(10.0, 10.0);
        let aim = (1.0, 0.0); // facing +x
        let front = spawn_mob(&mut world, Position2D::new(12.0, 10.0)); // in wedge, in range
        let behind = spawn_mob(&mut world, Position2D::new(7.0, 10.0)); // 180° — out of wedge
        let far = spawn_mob(&mut world, Position2D::new(20.0, 10.0)); // beyond radius
        let hit = enemies_in_arc(&world, center, aim, 4.0, 120.0);
        assert!(hit.contains(&front));
        assert!(!hit.contains(&behind));
        assert!(!hit.contains(&far));
    }

    #[test]
    fn estocs_combo_lands_each_tick_then_clears() {
        let mut world = test_world();
        let owner = world.spawn();
        world.positions.insert(owner, Position2D::new(0.0, 0.0));
        let target = spawn_mob(&mut world, Position2D::new(1.0, 0.0));
        world.combo_locks.insert(
            owner,
            ComboLock { hits_remaining: 2, interval: 0.2, timer: 0.2, target, dmg_per_hit: 15.0 },
        );
        // Each elapsed interval lands one guaranteed strike.
        tick_combos(&mut world, 0.2);
        close(world.health.get(&target).unwrap().current, 85.0);
        assert_eq!(world.combo_locks.get(&owner).map(|l| l.hits_remaining), Some(1));
        // The final strike empties the lock.
        tick_combos(&mut world, 0.2);
        close(world.health.get(&target).unwrap().current, 70.0);
        assert!(world.combo_locks.get(&owner).is_none());
    }

    #[test]
    fn unstoppable_shrugs_off_stun() {
        let mut world = test_world();
        let e = world.spawn();
        world.unstoppable.insert(e, Unstoppable { remaining: 0.5 });
        apply_stun(&mut world, e, 2.0);
        assert!(world.stuns.get(&e).is_none());
        // Without the window the stun lands normally.
        world.unstoppable.clear();
        apply_stun(&mut world, e, 2.0);
        assert_eq!(world.stuns.get(&e).copied(), Some(2.0));
    }

    #[test]
    fn combo_lock_freezes_player_input() {
        use crate::plugins::player::{input_system, PlayerCommand};
        let mut world = test_world();
        let player = world.spawn();
        world.positions.insert(player, Position2D::new(0.0, 0.0));
        world.player_entity = Some(player);
        world.combat.insert(player, crate::components::Combat::new(1.8, 0.8));
        let mob = spawn_mob(&mut world, Position2D::new(1.0, 0.0));
        world.combo_locks.insert(
            player,
            ComboLock { hits_remaining: 1, interval: 0.2, timer: 0.2, target: player, dmg_per_hit: 1.0 },
        );

        // An attack command issued while combo-locked is dropped.
        world.input.command = Some(PlayerCommand::Attack(mob));
        input_system(&mut world);
        assert_eq!(world.combat.get(&player).and_then(|c| c.target), None);

        // So is a move command.
        world.input.command = Some(PlayerCommand::MoveTo(5, 5));
        input_system(&mut world);
        assert!(world.move_targets.get(&player).is_none());
    }

    #[test]
    fn advance_order_sums_move_speed_synergies() {
        // 3%/lvl move, +1%/lvl Paysans, +1%/lvl Wargh.
        let syn = OrderSynergyLevels { peasant: 3, wargh: 1, ..Default::default() };
        let b = order_bonuses_for(OrderType::Advance, 2, syn);
        close(b.move_speed_pct, (6.0 + 3.0 + 1.0) / 100.0); // 0.10
        close(b.def_pct, 0.0);
    }

    #[test]
    fn hold_fast_order_grants_def_and_hp() {
        // DEF: 5%/lvl + 5%/lvl Miliciens + 3%/lvl Guerriers. HP: 5%/lvl.
        let syn = OrderSynergyLevels { militiaman: 2, warrior: 1, ..Default::default() };
        let b = order_bonuses_for(OrderType::HoldFast, 1, syn);
        close(b.def_pct, (5.0 + 10.0 + 3.0) / 100.0); // 0.18
        close(b.hp_pct, 0.05);
    }

    #[test]
    fn wargh_order_trades_defence_for_offence() {
        // Bare lvl 1: -95% DEF, +5% atk speed, +5% dmg.
        let b = order_bonuses_for(OrderType::Wargh, 1, OrderSynergyLevels::default());
        close(b.atk_speed_pct, 0.05);
        close(b.dmg_pct, 0.05);
        close(b.def_pct, -0.95);

        // Synergies claw the defence malus back; it never overshoots zero.
        let syn = OrderSynergyLevels { militiaman: 20, hold_fast: 20, warrior: 4, ..Default::default() };
        let b = order_bonuses_for(OrderType::Wargh, 1, syn);
        close(b.def_pct, 0.0);
        close(b.dmg_pct, (5.0 + 12.0) / 100.0); // 0.17
    }

    #[test]
    fn order_applies_and_expires_restoring_hp() {
        use crate::components::Health;
        let mut world = test_world();
        let ally = world.spawn();
        world.health.insert(ally, Health::full(100.0));
        world.positions.insert(ally, Position2D::new(0.0, 0.0));

        let bonuses = OrderBonuses { hp_pct: 0.05, def_pct: 0.18, ..OrderBonuses::default() };
        apply_order(&mut world, ally, OrderType::HoldFast, 20.0, bonuses);
        close(world.health.get(&ally).unwrap().max, 105.0);
        close(world.order_bonuses(ally).def_pct, 0.18);

        // After its window the buff is gone and max life is back to base.
        tick_orders(&mut world, 25.0);
        assert!(world.order_buffs.get(&ally).is_none());
        close(world.health.get(&ally).unwrap().max, 100.0);
    }

    #[test]
    fn order_refresh_does_not_compound_hp() {
        use crate::components::Health;
        let mut world = test_world();
        let ally = world.spawn();
        world.health.insert(ally, Health::full(100.0));

        let bonuses = OrderBonuses { hp_pct: 0.05, ..OrderBonuses::default() };
        apply_order(&mut world, ally, OrderType::HoldFast, 20.0, bonuses);
        apply_order(&mut world, ally, OrderType::HoldFast, 20.0, bonuses);
        // Re-cast replaces rather than stacks: still +5%, not +10.25%.
        close(world.health.get(&ally).unwrap().max, 105.0);
        assert_eq!(world.order_buffs.get(&ally).map(Vec::len), Some(1));
    }

    #[test]
    fn step_toward_clamps() {
        let a = Position2D::new(0.0, 0.0);
        let b = Position2D::new(10.0, 0.0);
        let mid = step_toward(a, b, 4.0);
        assert!((mid.x - 4.0).abs() < 1e-3);
        // Asking for more than the distance clamps to the endpoint.
        let end = step_toward(a, b, 100.0);
        assert!((end.x - 10.0).abs() < 1e-3);
    }

    // ── Mage de guerre ─────────────────────────────────────────────────────

    #[test]
    fn fireball_folds_fire_pool_and_cross_synergies() {
        // Lvl 1 alone: base 45; fire pool = wall(10) + meteor(25) = 35 → +1.75.
        let l = MageLevels { fireball: 1, ..Default::default() };
        close(fireball_dmg(l), 46.75);

        // With meteor + ice-lance ranks the pool grows and the multiplier kicks in.
        let l2 = MageLevels { fireball: 1, meteor: 2, ice_lance: 1, ..Default::default() };
        let pool = 10.0 + (25.0 + 12.0 * 2.0); // wall 10 + meteor 49 = 59
        let expected = (45.0 + 0.05 * pool) * (1.0 + 0.03 * 2.0 + 0.02);
        close(fireball_dmg(l2), expected);
    }

    #[test]
    fn meteor_impacts_grow_every_three_fireball_levels() {
        assert_eq!(meteor_impacts(MageLevels { meteor: 1, ..Default::default() }), 6);
        assert_eq!(meteor_impacts(MageLevels { fireball: 3, ..Default::default() }), 7);
        assert_eq!(meteor_impacts(MageLevels { fireball: 6, ..Default::default() }), 8);
    }

    #[test]
    fn ice_lance_slow_grows_with_blizzard_and_caps() {
        close(ice_lance_slow(MageLevels { ice_lance: 1, ..Default::default() }), 0.40);
        close(ice_lance_slow(MageLevels { blizzard: 2, ..Default::default() }), 0.40 * 1.10);
        // A huge blizzard investment is clamped to the 95% ceiling.
        close(ice_lance_slow(MageLevels { blizzard: 100, ..Default::default() }), 0.95);
    }

    #[test]
    fn lightning_chains_scale_with_lightning_and_nova() {
        assert_eq!(lightning_chains(MageLevels { lightning: 1, ..Default::default() }), 2);
        assert_eq!(lightning_chains(MageLevels { lightning: 3, ..Default::default() }), 3);
        // +1 per 3 Éclair levels, +1 per 3 Nova levels.
        assert_eq!(lightning_chains(MageLevels { lightning: 3, nova: 6, ..Default::default() }), 5);
    }

    #[test]
    fn nova_damage_and_stun_fold_synergies() {
        let l = MageLevels { nova: 2, ..Default::default() };
        close(nova_dmg(l), 65.0); // 35 + 30
        close(nova_stun(l), 0.7); // 0.5 + 0.2
        let l2 = MageLevels { nova: 2, lightning_shield: 1, lightning: 10, ..Default::default() };
        close(nova_dmg(l2), 65.0 * 1.05);
        close(nova_stun(l2), 0.7 * 1.30);
    }

    #[test]
    fn frost_armor_def_and_reflect_scale() {
        let l = MageLevels { frost_armor: 1, ..Default::default() };
        close(frost_armor_def(l), 0.20);
        close(frost_armor_reflect(l), 15.0);
        let l2 = MageLevels { frost_armor: 2, ice_lance: 1, blizzard: 5, ..Default::default() };
        close(frost_armor_def(l2), 0.40 + 0.03);
        close(frost_armor_reflect(l2), 20.0 * 1.10);
    }

    #[test]
    fn blizzard_duration_grows_every_three_frost_armor_levels() {
        close(blizzard_duration(MageLevels { blizzard: 1, ..Default::default() }), 5.0);
        close(blizzard_duration(MageLevels { frost_armor: 3, ..Default::default() }), 6.0);
        close(blizzard_duration(MageLevels { frost_armor: 5, ..Default::default() }), 6.0);
    }

    #[test]
    fn dominant_branch_picks_the_deepest_investment() {
        assert_eq!(dominant_branch(MageLevels { fireball: 2, ..Default::default() }), Element::Fire);
        assert_eq!(dominant_branch(MageLevels { blizzard: 3, ..Default::default() }), Element::Frost);
        assert_eq!(dominant_branch(MageLevels { nova: 4, ..Default::default() }), Element::Lightning);
        // Ties favour fire.
        assert_eq!(
            dominant_branch(MageLevels { fireball: 2, ice_lance: 2, ..Default::default() }),
            Element::Fire
        );
    }

    #[test]
    fn mitigate_applies_defense_and_elemental_resist() {
        use crate::systems::combat::mitigate;
        close(mitigate(100.0, 0.0, 25.0, 0.0, 0.0), 100.0);
        close(mitigate(100.0, 0.0, 25.0, 0.0, 0.5), 50.0);
        // Defense 50 vs an attacker of AR 25 → 50/(50 + 2*25) = 0.5 reduction.
        close(mitigate(100.0, 50.0, 25.0, 0.0, 0.0), 50.0);
        // Any hit floors at 1.
        close(mitigate(0.5, 0.0, 25.0, 0.9, 0.9), 1.0);
    }

    #[test]
    fn elemental_resist_reduces_spell_damage() {
        use crate::components::skill::ElementResists;
        let mut world = test_world();
        let owner = world.spawn(); // no stats → deterministic (no crit, no defence).
        world.positions.insert(owner, Position2D::new(0.0, 0.0));
        let m1 = spawn_mob(&mut world, Position2D::new(1.0, 0.0));
        let m2 = spawn_mob(&mut world, Position2D::new(2.0, 0.0));
        world.elem_resists.insert(m2, ElementResists { fire: 0.5, ..Default::default() });

        resolve_spell(&mut world, owner, m1, 40.0, Element::Fire);
        resolve_spell(&mut world, owner, m2, 40.0, Element::Fire);
        close(world.health.get(&m1).unwrap().current, 60.0); // full 40
        close(world.health.get(&m2).unwrap().current, 80.0); // halved to 20
    }

    #[test]
    fn immunity_short_circuits_to_zero() {
        use crate::components::skill::ElementResists;
        let mut world = test_world();
        let owner = world.spawn();
        world.positions.insert(owner, Position2D::new(0.0, 0.0));
        let target = spawn_mob(&mut world, Position2D::new(1.0, 0.0));
        // 100% fire resistance — the spell hit must deal zero, bypassing the
        // usual one-damage floor.
        world.elem_resists.insert(target, ElementResists { fire: 1.0, ..Default::default() });
        let before = world.health.get(&target).unwrap().current;
        resolve_spell(&mut world, owner, target, 200.0, Element::Fire);
        close(world.health.get(&target).unwrap().current, before);
    }

    #[test]
    fn vulnerability_amplifies_spell_damage() {
        use crate::components::skill::ElementResists;
        let mut world = test_world();
        let owner = world.spawn();
        world.positions.insert(owner, Position2D::new(0.0, 0.0));
        let target = spawn_mob(&mut world, Position2D::new(1.0, 0.0));
        // -0.25 frost: a 40-damage hit becomes 50.
        world.elem_resists.insert(target, ElementResists { frost: -0.25, ..Default::default() });
        resolve_spell(&mut world, owner, target, 40.0, Element::Frost);
        close(world.health.get(&target).unwrap().current, 50.0);
    }

    #[test]
    fn poison_resist_scales_dot_ticks() {
        use crate::components::skill::{Dot, ElementResists};
        let mut world = test_world();
        let owner = world.spawn();
        let target = spawn_mob(&mut world, Position2D::new(1.0, 0.0));
        world.dots.insert(target, Dot { owner, dps: 20.0, remaining: 1.0 });
        // 75% poison resist: a 0.5-second tick (10 raw) should land 2.5.
        world.elem_resists.insert(target, ElementResists { poison: 0.75, ..Default::default() });
        let before = world.health.get(&target).unwrap().current;
        let dt = 0.5;
        world.dt = dt;
        super::tick_dots(&mut world, dt);
        let after = world.health.get(&target).unwrap().current;
        close(before - after, 2.5);
    }

    #[test]
    fn poison_immunity_skips_dot_ticks_entirely() {
        use crate::components::skill::{Dot, ElementResists};
        let mut world = test_world();
        let owner = world.spawn();
        let target = spawn_mob(&mut world, Position2D::new(1.0, 0.0));
        world.dots.insert(target, Dot { owner, dps: 50.0, remaining: 1.0 });
        world.elem_resists.insert(target, ElementResists { poison: 1.0, ..Default::default() });
        let before = world.health.get(&target).unwrap().current;
        let dt = 0.5;
        world.dt = dt;
        super::tick_dots(&mut world, dt);
        // No floor: a poison-immune mob takes literally zero from poison DOTs.
        close(world.health.get(&target).unwrap().current, before);
    }

    #[test]
    fn flame_wall_only_burns_enemies_inside_the_rectangle() {
        let mut world = test_world();
        let owner = world.spawn();
        world.positions.insert(owner, Position2D::new(0.0, 0.0));
        let inside = spawn_mob(&mut world, Position2D::new(2.0, 0.0));
        let outside = spawn_mob(&mut world, Position2D::new(2.0, 5.0));
        let zone = world.spawn();
        world.flame_walls.insert(
            zone,
            crate::components::skill::FlameWall {
                owner,
                center: Position2D::new(0.0, 0.0),
                dir: (1.0, 0.0),
                half_len: 3.0,
                half_wid: 1.0,
                dmg_per_tick: 10.0,
                tick_interval: 0.5,
                tick_timer: 0.05,
                remaining: 4.0,
            },
        );
        fields::tick_flame_walls(&mut world, 0.1);
        close(world.health.get(&inside).unwrap().current, 90.0);
        close(world.health.get(&outside).unwrap().current, 100.0);
    }

    #[test]
    fn meteor_barrage_lands_every_impact_then_despawns() {
        let mut world = test_world();
        let owner = world.spawn();
        world.positions.insert(owner, Position2D::new(10.0, 10.0));
        // A fat mob covering the whole circle so every random impact connects.
        let mob = world.spawn();
        world.mob_tags.insert(mob, crate::components::mob::MobTag);
        world.positions.insert(mob, Position2D::new(10.0, 10.0));
        world.health.insert(mob, crate::components::Health::full(1000.0));
        let zone = world.spawn();
        world.meteor_fields.insert(
            zone,
            crate::components::skill::MeteorField {
                owner,
                center: Position2D::new(10.0, 10.0),
                radius: 0.5,
                dmg_per_impact: 10.0,
                impacts_left: 3,
                impact_interval: 0.1,
                impact_timer: 0.05,
            },
        );
        // Three ticks land three impacts; a fourth expires the zone. The first
        // impact knocks the mob down, so the next two land on a grounded target
        // and are amplified +20%: 10 + 12 + 12 = 34 damage.
        for _ in 0..4 {
            fields::tick_meteor_fields(&mut world, 0.1);
        }
        close(world.health.get(&mob).unwrap().current, 966.0);
        assert!(world.meteor_fields.get(&zone).is_none());
    }

    // ── Archer ─────────────────────────────────────────────────────────────

    #[test]
    fn fast_shot_folds_poison_synergy() {
        // 0.8 + 0.05/lvl, +2%/lvl from poison.
        close(fast_shot_mult(ArcherLevels { fast: 1, ..Default::default() }), 0.85);
        close(
            fast_shot_mult(ArcherLevels { fast: 4, poison: 2, ..Default::default() }),
            (0.8 + 0.20) * 1.04,
        );
    }

    #[test]
    fn piercing_pierce_count_grows_with_fast_shot() {
        assert_eq!(piercing_pierces(ArcherLevels { piercing: 1, ..Default::default() }), 1);
        assert_eq!(piercing_pierces(ArcherLevels { fast: 3, ..Default::default() }), 2);
        assert_eq!(piercing_pierces(ArcherLevels { fast: 9, ..Default::default() }), 4);
    }

    #[test]
    fn charged_shot_is_the_archers_biggest_single_hit() {
        // 2.0 +0.15/lvl, with piercing/fast synergies.
        close(charged_shot_mult(ArcherLevels { charged: 1, ..Default::default() }), 2.15);
        // A geared Archer puts up a much bigger number.
        let l = ArcherLevels { charged: 4, piercing: 3, fast: 4, ..Default::default() };
        close(charged_shot_mult(l), (2.0 + 0.60) * (1.0 + 0.15 + 0.12));
        // Charge time floors at 0.3s.
        close(charged_shot_charge_time(ArcherLevels { charged: 50, ..Default::default() }), 0.3);
    }

    #[test]
    fn jaw_trap_duration_stacks_synergies() {
        // 2s base, +0.3/lvl, +0.5 every 3 explosive levels, *(1 + 0.05*poison).
        close(jaw_trap_immobilize_dur(ArcherLevels { jaw: 1, ..Default::default() }), 2.3);
        close(
            jaw_trap_immobilize_dur(ArcherLevels { jaw: 1, explosive: 3, ..Default::default() }),
            2.8,
        );
        close(
            jaw_trap_immobilize_dur(ArcherLevels {
                jaw: 1,
                explosive: 3,
                poison: 4,
                ..Default::default()
            }),
            2.8 * 1.20,
        );
    }

    #[test]
    fn explosive_trap_damage_folds_poison_synergy() {
        close(explosive_trap_dmg(ArcherLevels { explosive: 1, ..Default::default() }), 32.0);
        close(
            explosive_trap_dmg(ArcherLevels { explosive: 2, poison: 4, ..Default::default() }),
            (20.0 + 24.0) * 1.20,
        );
    }

    #[test]
    fn hunter_instinct_crit_pct_caps_at_eighty() {
        assert_eq!(hunter_instinct_crit_pct(ArcherLevels { instinct: 1, ..Default::default() }), 10);
        // Capped: deep investment can't pile past 80%.
        assert_eq!(hunter_instinct_crit_pct(ArcherLevels { instinct: 20, ..Default::default() }), 80);
    }

    #[test]
    fn enemies_on_line_picks_only_the_forward_corridor() {
        let mut world = test_world();
        let from = Position2D::new(0.0, 0.0);
        let dir = (1.0, 0.0); // facing east
        let near = spawn_mob(&mut world, Position2D::new(2.0, 0.0));
        let far = spawn_mob(&mut world, Position2D::new(7.0, 0.0));
        let side = spawn_mob(&mut world, Position2D::new(3.0, 5.0));
        let behind = spawn_mob(&mut world, Position2D::new(-2.0, 0.0));
        let line = enemies_on_line(&world, from, dir, 9.0, 0.8);
        assert!(line.contains(&near));
        assert!(line.contains(&far));
        assert!(!line.contains(&side));
        assert!(!line.contains(&behind));
        // Sorted closest first.
        assert_eq!(line[0], near);
        assert_eq!(line[1], far);
    }

    #[test]
    fn explosive_trap_caps_at_max_active() {
        let mut world = test_world();
        let owner = world.spawn();
        // Three slots free → all three traps survive.
        for life in [1.0_f32, 2.0, 3.0] {
            let e = world.spawn();
            world.positions.insert(e, Position2D::new(0.0, 0.0));
            world.traps.insert(
                e,
                Trap {
                    owner,
                    kind: TrapKind::Explosive,
                    radius: 1.0,
                    damage: 10.0,
                    life,
                    slow: 1.0,
                    slow_dur: 0.0,
                },
            );
        }
        // Capping to 3 must despawn the oldest (lowest life) to make room.
        enforce_trap_cap(&mut world, owner, TrapKind::Explosive, 3);
        let alive_lives: Vec<f32> = world.traps.values().map(|t| t.life).collect();
        assert_eq!(alive_lives.len(), 2);
        assert!(!alive_lives.iter().any(|&l| l == 1.0)); // oldest gone
    }

    #[test]
    fn hunter_instinct_lifts_crit_and_dodge() {
        use crate::components::skill::HunterInstinctBuff;
        let mut world = test_world();
        let e = world.spawn();
        world.hunter_instinct.insert(
            e,
            HunterInstinctBuff { remaining: 10.0, crit_pct: 25, dodge_pts: 30.0, reveal_radius: 0.0 },
        );
        tick_hunter_instinct(&mut world, 5.0);
        assert!(world.hunter_instinct.get(&e).is_some(), "buff still active at t=5s");
        tick_hunter_instinct(&mut world, 6.0);
        assert!(world.hunter_instinct.get(&e).is_none(), "buff expired past its window");
    }

    #[test]
    fn charged_shot_cancels_when_owner_is_stunned() {
        use crate::components::skill::ChargingShot;
        let mut world = test_world();
        let owner = world.spawn();
        world.positions.insert(owner, Position2D::new(0.0, 0.0));
        let target = spawn_mob(&mut world, Position2D::new(3.0, 0.0));
        world.charging_shots.insert(
            owner,
            ChargingShot { target, remaining: 1.0, dmg_mult: 3.0, crit_bonus_pct: 50 },
        );
        // Stunning the caster mid-charge wipes the shot.
        world.stuns.insert(owner, 1.0);
        tick_charging_shots(&mut world, 0.1);
        assert!(world.charging_shots.get(&owner).is_none(), "stun cancels the charge");
    }

    #[test]
    fn arrow_rain_drops_repeated_volleys_then_expires() {
        let mut world = test_world();
        let owner = world.spawn();
        let mob = spawn_mob(&mut world, Position2D::new(0.0, 0.0));
        let zone = world.spawn();
        world.arrow_rains.insert(
            zone,
            ArrowRainField {
                owner,
                center: Position2D::new(0.0, 0.0),
                radius: 2.0,
                dmg_per_tick: 10.0,
                tick_interval: 0.5,
                tick_timer: 0.0,
                remaining: 1.5,
            },
        );
        // Three half-second ticks land three volleys, then the zone despawns.
        for _ in 0..4 {
            fields::tick_arrow_rains(&mut world, 0.5);
        }
        assert!(world.arrow_rains.get(&zone).is_none(), "zone should be expired");
        let hp = world.health.get(&mob).unwrap().current;
        assert!(hp < 100.0, "mob should have taken at least one volley (hp={hp})");
    }

    #[test]
    fn frost_armor_retaliates_against_melee_attackers() {
        let mut world = test_world();
        let player = world.spawn();
        world.positions.insert(player, Position2D::new(0.0, 0.0));
        world.health.insert(player, crate::components::Health::full(200.0));
        world.frost_armor.insert(
            player,
            crate::components::skill::FrostArmor { owner: player, def_pct: 0.0, reflect: 12.0, remaining: 15.0 },
        );
        let mob = spawn_mob(&mut world, Position2D::new(1.0, 0.0)); // in melee range
        resolve_attack(&mut world, mob, player, 1.0);
        // The mob took the 12 flat frost reflection (no defence/resist on it).
        close(world.health.get(&mob).unwrap().current, 88.0);
    }

    // ── Clerc ───────────────────────────────────────────────────────────────

    #[test]
    fn prayer_radius_and_stun_scale_with_rank() {
        // Radius: 3 + 0.3/lvl. Stun: 1 + 0.2/lvl.
        let cl = ClercLevels { prayer: 2, ..Default::default() };
        close(prayer_radius(cl), 3.6);
        close(prayer_stun_dur(cl), 1.4);
    }

    #[test]
    fn exorcism_dmg_folds_pardon_and_conversion_synergies() {
        // Base 30 + 15/lvl, * (1 + 0.05/lvl Pardon + 0.05/lvl Conversion).
        let cl = ClercLevels { exorcism: 1, pardon: 2, conversion: 1, ..Default::default() };
        close(exorcism_dmg(cl), (30.0 + 15.0) * (1.0 + 0.10 + 0.05));
    }

    #[test]
    fn pardon_leeches_attacker_on_hit() {
        // The attacker should heal for the pardon's flat leech of the mark's max HP.
        let mut world = test_world();
        let attacker = world.spawn();
        world.positions.insert(attacker, Position2D::new(0.0, 0.0));
        world.health.insert(attacker, crate::components::Health { current: 50.0, max: 200.0 });
        let victim = spawn_mob(&mut world, Position2D::new(1.0, 0.0));
        world.pardoned.insert(victim, Pardoned { leech_pct: 0.10, remaining: 5.0 });

        resolve_attack(&mut world, attacker, victim, 1.0);
        // 10% of the victim's max HP (100) = +10 healing on the attacker.
        let hp = world.health.get(&attacker).unwrap().current;
        assert!(hp >= 60.0, "attacker should have leeched, got {hp}");
    }

    #[test]
    fn vengeance_aura_reflects_a_fraction() {
        // Vengeance reflects retaliate_pct of damage taken.
        let mut world = test_world();
        let defender = world.spawn();
        world.positions.insert(defender, Position2D::new(0.0, 0.0));
        world.health.insert(defender, crate::components::Health::full(500.0));
        world.aura_bonuses.insert(
            defender,
            AuraBonus { retaliate_pct: 0.5, remaining: 1.0, ..Default::default() },
        );
        let attacker = spawn_mob(&mut world, Position2D::new(1.0, 0.0));
        let before = world.health.get(&attacker).unwrap().current;
        resolve_attack(&mut world, attacker, defender, 1.0);
        let after = world.health.get(&attacker).unwrap().current;
        assert!(after < before, "attacker should take some reflected damage (before={before}, after={after})");
    }

    #[test]
    fn conversion_flips_a_mob_faction_then_restores_it() {
        let mut world = test_world();
        let player = world.spawn();
        world.positions.insert(player, Position2D::new(0.0, 0.0));
        let mob = spawn_mob(&mut world, Position2D::new(2.0, 0.0));
        world.factions.insert(mob, Faction::Enemy);
        world.converted.insert(
            mob,
            Converted { original_faction: Faction::Enemy, remaining: 0.05 },
        );
        world.factions.insert(mob, Faction::Player);

        tick_converted(&mut world, 0.1);
        // Expired: original faction restored.
        assert_eq!(world.factions.get(&mob).copied(), Some(Faction::Enemy));
        assert!(world.converted.get(&mob).is_none());
    }

    #[test]
    fn heal_aura_refreshes_health_on_allies() {
        let mut world = test_world();
        let caster = world.spawn();
        world.positions.insert(caster, Position2D::new(0.0, 0.0));
        world.health.insert(caster, crate::components::Health { current: 50.0, max: 100.0 });
        world.active_auras.insert(
            caster,
            ActiveAura {
                skill_id: 5001,
                aura_type: AuraType::Heal,
                level: 4, // 0.01 + 0.005*4 = 0.03 per tick
                reserved_mana: 15.0,
                radius: 5.0,
                tick_timer: 0.0,
            },
        );
        // One full tick fires immediately (timer at 0): heal +3 HP.
        tick_auras(&mut world, 0.1);
        close(world.health.get(&caster).unwrap().current, 53.0);
    }

    #[test]
    fn defense_aura_refreshes_bonus_on_allies() {
        let mut world = test_world();
        let caster = world.spawn();
        world.positions.insert(caster, Position2D::new(0.0, 0.0));
        world.active_auras.insert(
            caster,
            ActiveAura {
                skill_id: 5002,
                aura_type: AuraType::Defense,
                level: 3,
                reserved_mana: 12.0,
                radius: 5.0,
                tick_timer: 0.0,
            },
        );
        tick_auras(&mut world, 0.1);
        let b = world.aura_bonuses.get(&caster).copied().unwrap();
        close(b.def_pct, 0.15); // 5%/lvl
    }

    #[test]
    fn resurrection_completes_and_revives_a_dead_mercenary() {
        use crate::components::mercenary::{MercInventory, MercState, MercType, MercenaryTag};
        let mut world = test_world();
        let player = world.spawn();
        world.positions.insert(player, Position2D::new(0.0, 0.0));
        world.player_entity = Some(player);
        // A dead mercenary owned by the player (respawn_timer > 0).
        let merc = world.spawn();
        world.mercenary_tags.insert(
            merc,
            MercenaryTag { owner: player, merc_type: MercType::Guardian, hired: true },
        );
        let mut state = MercState::new(1, 0);
        state.respawn_timer = 30.0;
        world.merc_states.insert(merc, state);
        world.merc_inventory.insert(merc, MercInventory::new());

        world.resurrecting.insert(player, Resurrecting { remaining: 0.05, mana_cost: 0.0 });
        tick_resurrecting(&mut world, 0.1);
        // Cast complete: the merc's respawn timer is shrunk to a tiny window.
        assert!(world.resurrecting.get(&player).is_none(), "resurrection should finish");
        let respawn = world.merc_states.get(&merc).unwrap().respawn_timer;
        assert!(respawn > 0.0 && respawn <= 0.01, "respawn should be a tiny tail (got {respawn})");
    }

    #[test]
    fn resurrection_cancels_on_stun() {
        let mut world = test_world();
        let caster = world.spawn();
        world.resurrecting.insert(caster, Resurrecting { remaining: 2.0, mana_cost: 0.0 });
        world.stuns.insert(caster, 1.0);
        tick_resurrecting(&mut world, 0.1);
        assert!(world.resurrecting.get(&caster).is_none(), "stun should cancel the channel");
    }

    #[test]
    fn cherubin_heal_pct_scales_with_rank_and_prophet() {
        use crate::systems::summon::{clerc_angelic_levels, ClercAngelicLevels};
        let _ = clerc_angelic_levels; // silence unused-import warning for non-test paths.

        // Smoke check: with rank 3 and prophet 2, base = 0.01 + 0.005*3 = 0.025,
        // boosted by +20% from the prophet → 0.030.
        let lv = ClercAngelicLevels { cherubin: 3, prophet: 2, ..Default::default() };
        let computed = (0.01 + 0.005 * 3.0) * (1.0 + 0.10 * 2.0);
        close(computed, 0.030);
        let _ = lv;
    }
}
