//! @id     games_allumina_systems_summon
//! @do     definir_max_summons
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « systems summon ».

//! Summon behaviour: spawning invocations from a profile, their allied AI
//! (mirror of the mob FSM but targeting enemies), and their lifecycle (timed
//! expiry, death, dismissal when the owner dies). Summons reuse the shared
//! [`crate::systems::combat`] resolution; their AI only sets targets and paths.

use crate::components::player::{ClassType, Progression, Stats};
use crate::components::skill::{Buff, Resist, SkillEffect, Slow};
use crate::components::summon::{SummonTag, SummonType};
use crate::components::{Combat, Faction, Health, MoveTarget, Position2D};
use crate::data::summon::SummonProfile;
use crate::ecs::{Entity, RenderEvent, SkillFx, VisualKind, World};
use crate::systems::combat::{effective_damage, effective_defense, resolve_attack};
use crate::systems::movement::find_path;
use crate::systems::spatial::MobSnapshot;

/// Maximum simultaneous *fixed-count* summons per owner. Level-scaled Soldat
/// hosts (peasants/militia/warriors) are exempt and use a per-type cap instead,
/// so a Soldat can field a large mixed army.
pub const MAX_SUMMONS: usize = 3;
/// Per-point fraction of a Soldat cross-synergy bonus: each point invested in
/// one host skill grants this much of the relevant stat to the *other* hosts
/// (e.g. +10% HP per point — see [`cross_synergy`]).
pub const CROSS_SYNERGY_PER_POINT: f32 = 0.10;
/// How far (tiles) a summon will look for a foe to engage.
const ENGAGE_RANGE: f32 = 12.0;
/// How close a roaming summon tries to stay to its owner.
const REGROUP_RANGE: f32 = 6.0;
/// Re-path cadence while chasing (ticks).
const REPATH_TICKS: u64 = 12;

/// Active cap (and per-cast count) for a profile at a given casting skill level.
/// Level-scaled kinds grow by `1 + floor((level - 1) * max_per_level)`; fixed
/// kinds simply spawn their static [`SummonProfile::count`].
#[must_use]
pub fn host_size(profile: &SummonProfile, skill_level: u32) -> u32 {
    if profile.max_per_level <= 0.0 {
        return profile.count;
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let extra = ((skill_level.saturating_sub(1)) as f32 * profile.max_per_level).floor() as u32;
    1 + extra
}

/// Points invested in the Soldat's three summon skills, read from the owner's
/// hotbar (each summon skill's [`level`](crate::components::skill::ActiveSkill)).
/// Zero for skills the owner doesn't have, which disables the cross-synergies
/// for non-Soldat owners.
#[derive(Debug, Clone, Copy, Default)]
pub struct SoldatHostLevels {
    pub peasant: u32,
    pub militiaman: u32,
    pub warrior: u32,
}

impl SoldatHostLevels {
    /// Points in the skill that raises `kind` (0 if `kind` is not a Soldat host).
    fn for_kind(self, kind: SummonType) -> u32 {
        match kind {
            SummonType::Peasant => self.peasant,
            SummonType::Militiaman => self.militiaman,
            SummonType::EliteWarrior => self.warrior,
            _ => 0,
        }
    }
}

/// Read the points invested in `owner`'s three Soldat summon skills.
#[must_use]
pub fn soldat_host_levels(world: &World, owner: Entity) -> SoldatHostLevels {
    let mut lv = SoldatHostLevels::default();
    let Some(slot) = world.skills.get(&owner) else { return lv };
    for skill in &slot.bar {
        if let SkillEffect::Summon { profile } = skill.effect {
            match profile.kind {
                SummonType::Peasant => lv.peasant = skill.level,
                SummonType::Militiaman => lv.militiaman = skill.level,
                SummonType::EliteWarrior => lv.warrior = skill.level,
                _ => {}
            }
        }
    }
    lv
}

/// Stat multipliers a Soldat host receives from points invested in the *other*
/// two host skills (the cross-synergies). Each is `1.0` for non-Soldat kinds.
///
/// * Peasants: +HP per militia point, +damage per warrior point.
/// * Militia:  +move speed per peasant point, +HP per warrior point.
/// * Warriors: +attack speed per peasant point, +HP per militia point.
#[derive(Debug, Clone, Copy)]
struct CrossSynergy {
    hp_mult: f32,
    dmg_mult: f32,
    move_mult: f32,
    atk_speed_mult: f32,
}

impl CrossSynergy {
    const NONE: Self = Self { hp_mult: 1.0, dmg_mult: 1.0, move_mult: 1.0, atk_speed_mult: 1.0 };
}

/// Rank invested in each of the Clerc's three angelic-summon skills plus the
/// "Élu prophétique" passive, read off the owner's hotbar by id.
#[derive(Debug, Clone, Copy, Default)]
pub struct ClercAngelicLevels {
    pub cherubin: u32,
    pub guardian: u32,
    pub archangel: u32,
    pub prophet: u32,
}

impl ClercAngelicLevels {
    fn for_kind(self, kind: SummonType) -> u32 {
        match kind {
            SummonType::Cherubin => self.cherubin,
            SummonType::GuardianAngel => self.guardian,
            SummonType::Archangel => self.archangel,
            _ => 0,
        }
    }
}

/// Read the owner's invested ranks in the four Clerc DIVIN skills (the three
/// angelic summons plus the prophet passive).
#[must_use]
pub fn clerc_angelic_levels(world: &World, owner: Entity) -> ClercAngelicLevels {
    let mut lv = ClercAngelicLevels::default();
    let Some(slot) = world.skills.get(&owner) else { return lv };
    for skill in &slot.bar {
        match skill.id {
            5012 => lv.prophet = skill.level,
            5013 => lv.cherubin = skill.level,
            5014 => lv.guardian = skill.level,
            5015 => lv.archangel = skill.level,
            _ => {}
        }
    }
    lv
}

/// Spawn-time stat multipliers for an angelic summon. Non-angelic kinds receive
/// the identity multipliers (no effect).
#[derive(Debug, Clone, Copy)]
struct AngelicScaling {
    hp_mult: f32,
    dmg_mult: f32,
    def_mult: f32,
    atk_speed_mult: f32,
}

impl AngelicScaling {
    const NONE: Self = Self { hp_mult: 1.0, dmg_mult: 1.0, def_mult: 1.0, atk_speed_mult: 1.0 };
}

/// Compute the spawn-time scaling for an angelic summon: per-rank growth on
/// its own skill, plus the prophet passive bonus (and, for the Archange, a
/// cross-synergy with Chérubin and Ange-gardien ranks).
#[allow(clippy::cast_precision_loss)]
fn angelic_scaling(kind: SummonType, lv: ClercAngelicLevels) -> AngelicScaling {
    let prophet = 1.0 + 0.10 * lv.prophet as f32;
    match kind {
        SummonType::Cherubin => AngelicScaling {
            hp_mult: prophet * (1.0 + 0.05 * lv.cherubin as f32),
            dmg_mult: 1.0,
            def_mult: prophet * (1.0 + 0.05 * lv.cherubin as f32),
            atk_speed_mult: prophet,
        },
        SummonType::GuardianAngel => {
            let rank = 1.0 + 0.10 * lv.guardian as f32;
            AngelicScaling {
                hp_mult: prophet * rank,
                dmg_mult: prophet * rank,
                def_mult: prophet * rank,
                atk_speed_mult: prophet,
            }
        }
        SummonType::Archangel => {
            let cross = 1.0 + 0.05 * lv.cherubin as f32 + 0.05 * lv.guardian as f32;
            AngelicScaling {
                hp_mult: prophet * cross,
                dmg_mult: prophet * cross,
                def_mult: prophet * cross,
                atk_speed_mult: prophet,
            }
        }
        _ => AngelicScaling::NONE,
    }
}

/// Per-spawn Cherubin heal aura fraction (per second), scaled by its own rank
/// and the prophet passive.
#[allow(clippy::cast_precision_loss)]
fn cherubin_heal_pct(lv: ClercAngelicLevels) -> f32 {
    let base = 0.01 + 0.005 * lv.cherubin as f32;
    base * (1.0 + 0.10 * lv.prophet as f32)
}

fn cross_synergy(kind: SummonType, lv: SoldatHostLevels) -> CrossSynergy {
    let bonus = |points: u32| 1.0 + CROSS_SYNERGY_PER_POINT * points as f32;
    match kind {
        SummonType::Peasant => {
            CrossSynergy { hp_mult: bonus(lv.militiaman), dmg_mult: bonus(lv.warrior), ..CrossSynergy::NONE }
        }
        SummonType::Militiaman => {
            CrossSynergy { hp_mult: bonus(lv.warrior), move_mult: bonus(lv.peasant), ..CrossSynergy::NONE }
        }
        SummonType::EliteWarrior => {
            CrossSynergy { hp_mult: bonus(lv.militiaman), atk_speed_mult: bonus(lv.peasant), ..CrossSynergy::NONE }
        }
        _ => CrossSynergy::NONE,
    }
}

/// Spawn the summons described by `profile`, owned by `owner`. The host size and
/// the Soldat cross-synergy bonuses are both derived from the points invested in
/// the owner's three Soldat summon skills (their hotbar levels). Enforces the
/// appropriate cap (a per-type cap for level-scaled Soldat hosts, the global cap
/// otherwise) by evicting the oldest summons first, and returns the new entities.
pub fn spawn_summon(
    world: &mut World,
    owner: Entity,
    profile: &'static SummonProfile,
) -> Vec<Entity> {
    let Some(&owner_pos) = world.positions.get(&owner) else { return Vec::new() };
    let owner_class = world.classes.get(&owner).copied().unwrap_or(ClassType::Soldat);
    let owner_level = world.progression.get(&owner).map_or(1, Progression::level);
    let owner_hp_max = world.health.get(&owner).map_or(50.0, |h| h.max);
    let (eff_min, eff_max) = effective_damage(world, owner);
    let eff_def = effective_defense(world, owner);

    // Points in the owner's three Soldat summon skills drive both the host size
    // (the skill matching this profile) and the cross-synergy stat bonuses (from
    // the other two skills). Non-Soldat owners report zero, so no synergy.
    let levels = soldat_host_levels(world, owner);
    let own_level = levels.for_kind(profile.kind).max(1);
    let syn = cross_synergy(profile.kind, levels);

    // Angelic summons (Clerc) scale by their own rank + Élu prophétique passive
    // and (for Archange) by the Chérubin and Ange-gardien ranks.
    let angelic = clerc_angelic_levels(world, owner);
    let ang = angelic_scaling(profile.kind, angelic);
    let own_clerc_lvl = angelic.for_kind(profile.kind).max(1);
    let duration = if matches!(profile.kind, SummonType::Archangel) {
        15.0 + 2.0 * own_clerc_lvl as f32
    } else {
        profile.duration
    };

    let count = host_size(profile, own_level);
    if profile.max_per_level > 0.0 {
        // A re-cast refreshes this host: evict the owner's existing summons of
        // this kind so the new (full-strength) group fits its per-type cap.
        enforce_type_cap(world, owner, profile.kind, count as usize);
    } else {
        enforce_global_cap(world, owner, count as usize);
    }

    let mut spawned = Vec::new();
    for i in 0..count {
        let e = world.spawn();
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let pos = free_tile_near(world, owner_pos, i as i32 + 1).unwrap_or(owner_pos);
        world.positions.insert(e, pos);
        world.factions.insert(e, Faction::Player);

        // Inherit a fraction of the owner's power, then fold in the cross-synergy
        // multipliers. We deliberately do NOT insert a `classes` entry so the
        // combat formula reads zero primary stat and the damage we bake into
        // `resolved` is used as-is.
        let mut stats = Stats::new(owner_class, owner_level);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        {
            stats.inner.resolved.life_max =
                (owner_hp_max * profile.hp_pct * syn.hp_mult * ang.hp_mult).max(1.0);
            stats.inner.resolved.damage_min =
                (eff_min * profile.dmg_pct * syn.dmg_mult * ang.dmg_mult).round().max(0.0) as i32;
            stats.inner.resolved.damage_max =
                (eff_max * profile.dmg_pct * syn.dmg_mult * ang.dmg_mult).round().max(1.0) as i32;
            stats.inner.resolved.defense =
                (eff_def * profile.def_pct * ang.def_mult).round().max(0.0) as i32;
        }
        let hp = stats.inner.resolved.life_max;
        world.stats.insert(e, stats);
        world.health.insert(e, Health::full(hp));

        if profile.attacks {
            // Higher attack speed → shorter interval between swings.
            let interval = profile.attack_interval / (syn.atk_speed_mult * ang.atk_speed_mult);
            world.combat.insert(e, Combat::new(profile.attack_range, interval));
        }
        let mut tag = SummonTag::new(owner, duration, world.tick, profile);
        tag.move_speed = profile.move_speed * syn.move_mult;
        // Cherubin's heal aura grows with its own rank (and the prophet passive).
        if matches!(profile.kind, SummonType::Cherubin) {
            tag.aura_heal_pct = cherubin_heal_pct(angelic);
        }
        world.summon_tags.insert(e, tag);
        spawned.push(e);
    }

    let suffix = if count > 1 { format!(" x{count}") } else { String::new() };
    world.log_event(format!("Summoned {}{suffix}.", profile.name));
    spawned
}

/// Evict the oldest *fixed-count* summons of `owner` until `incoming` more will
/// fit under the global cap. Level-scaled hosts are ignored here — they neither
/// count toward nor are limited by the global cap.
fn enforce_global_cap(world: &mut World, owner: Entity, incoming: usize) {
    loop {
        let mut owned: Vec<(Entity, u64)> = world
            .summon_tags
            .iter()
            .filter(|(_, t)| t.owner == owner && t.profile.max_per_level <= 0.0)
            .map(|(e, t)| (*e, t.created_tick))
            .collect();
        if owned.is_empty() || owned.len() + incoming <= MAX_SUMMONS {
            break;
        }
        owned.sort_by_key(|(e, tick)| (*tick, e.0));
        world.despawn(owned[0].0);
    }
}

/// Evict the oldest summons of `owner` of one `kind` until `incoming` more fit
/// under that kind's `cap`.
fn enforce_type_cap(world: &mut World, owner: Entity, kind: SummonType, incoming: usize) {
    let cap = incoming.max(1);
    loop {
        let mut owned: Vec<(Entity, u64)> = world
            .summon_tags
            .iter()
            .filter(|(_, t)| t.owner == owner && t.summon_type == kind)
            .map(|(e, t)| (*e, t.created_tick))
            .collect();
        if owned.is_empty() || owned.len() + incoming <= cap {
            break;
        }
        owned.sort_by_key(|(e, tick)| (*tick, e.0));
        world.despawn(owned[0].0);
    }
}

/// First walkable tile at ring distance `ring` around `center`.
fn free_tile_near(world: &World, center: Position2D, ring: i32) -> Option<Position2D> {
    let (cx, cy) = center.tile();
    let offsets = [
        (ring, 0),
        (-ring, 0),
        (0, ring),
        (0, -ring),
        (ring, ring),
        (-ring, -ring),
        (ring, -ring),
        (-ring, ring),
    ];
    offsets
        .into_iter()
        .find(|(dx, dy)| world.map.is_walkable(cx + dx, cy + dy))
        .map(|(dx, dy)| Position2D::from_tile(cx + dx, cy + dy))
}

/// Allied AI: pick enemy targets, chase or hold, and project buff auras.
pub fn summon_ai_system(world: &mut World) {
    // Capture the living-mob list once for every summon's proximity query this
    // frame instead of re-scanning (and re-allocating) per summon.
    let mobs = MobSnapshot::build(world);
    for e in world.summon_entities() {
        let Some(tag) = world.summon_tags.get(&e).copied() else { continue };
        let Some(&pos) = world.positions.get(&e) else { continue };
        let profile = tag.profile;
        // "En avant !" and other orders fold a move-speed surge into this summon.
        let speed = tag.move_speed * (1.0 + world.order_bonuses(e).move_speed_pct);

        // Totem-style summons project one or more auras onto nearby allies (or a
        // slow onto nearby enemies).
        if profile.aura_atk_pct > 0.0 {
            apply_atk_aura(world, tag.owner, pos, profile.aura_atk_pct, profile.aura_radius);
        }
        if profile.aura_def_pct > 0.0 {
            apply_def_aura(world, tag.owner, pos, profile.aura_def_pct, profile.aura_radius);
        }
        if tag.aura_heal_pct > 0.0 {
            apply_heal_aura(world, tag.owner, pos, tag.aura_heal_pct, profile.aura_radius);
        }
        if profile.enemy_slow > 0.0 {
            apply_enemy_slow(world, &mobs, pos, profile.enemy_slow, profile.aura_radius);
        }
        if !profile.attacks {
            continue;
        }

        let target = mobs.nearest_alive(world, pos, ENGAGE_RANGE);
        if let Some(c) = world.combat.get_mut(&e) {
            c.target = target;
        }

        let Some(target) = target else {
            if !profile.stationary {
                regroup(world, e, pos, tag.owner, speed);
            }
            continue;
        };

        if profile.stationary {
            world.move_targets.remove(&e);
            continue;
        }

        let Some(&tpos) = world.positions.get(&target) else { continue };
        if pos.distance(tpos) > profile.attack_range {
            let need_path =
                world.move_targets.get(&e).is_none() || world.tick % REPATH_TICKS == 0;
            if need_path {
                if let Some(path) = find_path(&world.map, pos.tile(), tpos.tile()) {
                    if !path.is_empty() {
                        world.move_targets.insert(e, MoveTarget::new(tpos, path, speed));
                    }
                }
            }
        } else {
            world.move_targets.remove(&e);
        }
    }
}

/// Lifecycle: dismiss summons whose owner died, despawn the slain, and age out
/// timed summons. Runs late in the Combat phase so it sees this tick's deaths
/// (including the player's, before the death system respawns them).
pub fn summon_system(world: &mut World) {
    let dt = world.dt;
    let player = world.player_entity;
    let player_dead = player.map_or(true, |p| world.health.get(&p).is_some_and(|h| h.is_empty()));
    // Shared by every elite cleave this frame (see `cleave`).
    let mobs = MobSnapshot::build(world);

    for e in world.summon_entities() {
        let Some(mut tag) = world.summon_tags.get(&e).copied() else { continue };
        let profile = tag.profile;

        let owner_gone = !world.positions.contains_key(&tag.owner)
            || world.health.get(&tag.owner).is_some_and(|h| h.is_empty());
        let owner_is_dead_player = Some(tag.owner) == player && player_dead;
        if owner_gone || owner_is_dead_player {
            world.despawn(e);
            continue;
        }

        if world.health.get(&e).is_some_and(|h| h.is_empty()) {
            emit_summon_death(world, e);
            world.despawn(e);
            continue;
        }

        // Elite warrior "Frappe de taille": a periodic cleave around itself. The
        // timer stays armed (does not reset) until a foe is actually in reach.
        if profile.cleave_interval > 0.0 {
            tag.special_cd -= dt;
            if tag.special_cd <= 0.0 {
                let hit = cleave(world, &mobs, e, profile.cleave_mult, profile.cleave_radius);
                tag.special_cd = if hit { profile.cleave_interval } else { 0.0 };
            }
        }

        if tag.duration > 0.0 {
            tag.duration -= dt;
            if tag.duration <= 0.0 {
                emit_summon_death(world, e);
                world.despawn(e);
                continue;
            }
        }
        world.summon_tags.insert(e, tag);
    }
}

/// One "Frappe de taille" swing: the warrior `e` strikes every living enemy
/// within `radius`. Returns whether it landed on at least one foe.
fn cleave(world: &mut World, mobs: &MobSnapshot, e: Entity, mult: f32, radius: f32) -> bool {
    let Some(&pos) = world.positions.get(&e) else { return false };
    let foes = mobs.within_alive(world, pos, radius);
    if foes.is_empty() {
        return false;
    }
    for f in &foes {
        resolve_attack(world, e, *f, mult);
    }
    world.render_events.push(RenderEvent::Skill { caster: e, from: pos, to: pos, fx: SkillFx::Whirlwind });
    true
}

fn emit_summon_death(world: &mut World, e: Entity) {
    if let Some(&pos) = world.positions.get(&e) {
        world.render_events.push(RenderEvent::Death { pos, kind: VisualKind::Caster });
    }
}

/// Refresh an attack buff on every ally within `radius` of `center`, never
/// downgrading a stronger buff already in place (e.g. War Cry).
fn apply_atk_aura(world: &mut World, owner: Entity, center: Position2D, atk_pct: f32, radius: f32) {
    for ally in ally_entities(world, owner) {
        let Some(&apos) = world.positions.get(&ally) else { continue };
        if center.distance(apos) > radius {
            continue;
        }
        let keep = match world.buffs.get(&ally).copied() {
            Some(b) if b.atk_pct > atk_pct => b,
            _ => Buff { atk_pct, remaining: 0.6 },
        };
        world.buffs.insert(ally, keep);
    }
}

/// Refresh a damage-reduction blessing on allies within `radius` (war banner).
fn apply_def_aura(world: &mut World, owner: Entity, center: Position2D, def_pct: f32, radius: f32) {
    for ally in ally_entities(world, owner) {
        let Some(&apos) = world.positions.get(&ally) else { continue };
        if center.distance(apos) > radius {
            continue;
        }
        let keep = match world.resist.get(&ally).copied() {
            Some(r) if r.pct > def_pct => r,
            _ => Resist { pct: def_pct, remaining: 0.6 },
        };
        world.resist.insert(ally, keep);
    }
}

/// Heal allies standing within `radius` of `center` (sacred circle).
fn apply_heal_aura(world: &mut World, owner: Entity, center: Position2D, hp_pct: f32, radius: f32) {
    let dt = world.dt;
    for ally in ally_entities(world, owner) {
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
}

/// Slow every enemy within `radius` of `center` (jaw trap immobiliser).
fn apply_enemy_slow(world: &mut World, mobs: &MobSnapshot, center: Position2D, factor: f32, radius: f32) {
    for e in mobs.within_alive(world, center, radius) {
        world.slows.insert(e, Slow { factor: 1.0 - factor.clamp(0.0, 0.95), remaining: 0.6 });
    }
}

/// The owner plus every ally entity it controls (its summons and mercenaries).
fn ally_entities(world: &World, owner: Entity) -> Vec<Entity> {
    let mut out = vec![owner];
    out.extend(
        world.summon_tags.iter().filter(|(_, t)| t.owner == owner).map(|(e, _)| *e),
    );
    out.extend(
        world.mercenary_tags.iter().filter(|(_, t)| t.owner == owner).map(|(e, _)| *e),
    );
    out
}

/// Path a roaming summon back toward its owner when it strays too far.
fn regroup(world: &mut World, e: Entity, pos: Position2D, owner: Entity, speed: f32) {
    let Some(&opos) = world.positions.get(&owner) else { return };
    if pos.distance(opos) > REGROUP_RANGE {
        if world.move_targets.get(&e).is_none() {
            if let Some(path) = find_path(&world.map, pos.tile(), opos.tile()) {
                if !path.is_empty() {
                    world.move_targets.insert(e, MoveTarget::new(opos, path, speed));
                }
            }
        }
    } else {
        world.move_targets.remove(&e);
    }
}
