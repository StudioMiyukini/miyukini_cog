//! Mercenary behaviour: recruitment at the village, an AI that follows the
//! player and fights at their side (with per-archetype quirks), and a lifecycle
//! pass that shares the player's XP, levels the merc up, and handles death +
//! respawn. Mercenaries reuse the shared combat resolution; their stats are
//! recomputed from their level and the same ten equipment slots the player uses.

use mge_gameplay::stats::{Modifier, StatKind};
use mge_items::affixes::ItemAffixes;
use mge_items::item::{EquipSlot, Item};
use mge_items::rarity::Rarity;

use crate::components::item::ItemInstance;
use crate::components::mercenary::{MercInventory, MercState, MercType, MercenaryTag};
use crate::components::player::{ClassType, Stats};
use crate::components::{Combat, Faction, Health, MoveTarget, Position2D};
use crate::data::items::base_item;
use crate::data::mercenary::merc_profile;
use crate::ecs::{Entity, RenderEvent, SkillFx, VisualKind, World};
use crate::systems::combat::resolve_attack;
use crate::systems::movement::find_path;
use crate::systems::spatial::MobSnapshot;

/// Seconds before a slain mercenary returns.
pub const RESPAWN_TIME: f32 = 30.0;
/// Radius (tiles) around the player within which the merc engages foes.
const ENGAGE_RADIUS: f32 = 4.0;
/// Distance from the player past which the merc breaks off and returns.
const FOLLOW_MAX: f32 = 5.0;
/// Stop trailing the player once this close.
const FOLLOW_KEEP: f32 = 2.5;
/// Re-path cadence while moving (ticks).
const REPATH_TICKS: u64 = 12;

const HEAL_CD: f32 = 5.0;
const HEAL_THRESHOLD: f32 = 0.5;
const HEAL_FRAC: f32 = 0.18;

const CAST_CD: f32 = 3.0;
const CAST_RADIUS: f32 = 2.5;
const CAST_MULT: f32 = 1.4;

/// XP the merc must accumulate to advance from `level` to the next.
fn xp_to_next(level: u32) -> u64 {
    60 + u64::from(level) * 40
}

/// Map a mercenary archetype onto an Allumina class for base-stat seeding (the
/// resolved numbers are then overwritten by [`recompute_merc_stats`]).
fn merc_class(merc_type: MercType) -> ClassType {
    match merc_type {
        MercType::Guardian => ClassType::Soldat,
        MercType::Archer => ClassType::Archer,
        MercType::Healer => ClassType::Clerc,
        MercType::Mage => ClassType::MageDeGuerre,
    }
}

/// Recompute a mercenary's resolved combat numbers from its level and equipped
/// gear, preserving its current health fraction.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn recompute_merc_stats(world: &mut World, merc: Entity) {
    let Some(tag) = world.mercenary_tags.get(&merc).copied() else { return };
    let Some(state) = world.merc_states.get(&merc).copied() else { return };
    let profile = merc_profile(tag.merc_type);
    let lvl = (state.level.max(1) - 1) as f32;

    let mut dmg_min = profile.base_dmg_min + profile.dmg_per_level * lvl;
    let mut dmg_max = profile.base_dmg_max + profile.dmg_per_level * lvl;
    let mut defense = profile.base_def + profile.def_per_level * lvl;
    let mut life = profile.base_hp + profile.hp_per_level * lvl;
    let mut dodge = profile.dodge;

    if let Some(inv) = world.merc_inventory.get(&merc) {
        for item in inv.items() {
            for m in item.stat_modifiers() {
                if let Modifier::Flat(v) = m.modifier {
                    let v = v as f32;
                    match m.stat {
                        StatKind::DamageMin => dmg_min += v,
                        StatKind::DamageMax => dmg_max += v,
                        StatKind::Defense => defense += v,
                        StatKind::Life => life += v,
                        StatKind::Dexterity => dodge += v,
                        _ => {}
                    }
                }
            }
        }
        // NB: the weapon's ilvl base damage is *not* baked in here — combat's
        // `gather_attacker` adds the swinging weapon's term per hand (so a
        // dual-wield merc alternates each weapon's damage like the player).
    }

    // Block chance from the merc's off-hand configuration (shield or two
    // one-handed melee weapons), sharpened by its dodge/Dexterity — same rules
    // as the player.
    let (main_kind, off_kind) = world.merc_inventory.get(&merc).map_or((None, None), |inv| {
        (
            inv.get(EquipSlot::MainHand).map(|i| i.item.base.kind),
            inv.get(EquipSlot::OffHand).map(|i| i.item.base.kind),
        )
    });
    let block_base = crate::systems::stats::block_base_from_kinds(main_kind, off_kind);
    let block = crate::systems::stats::block_chance_from_base(block_base, dodge.round() as i32);

    if let Some(s) = world.stats.get_mut(&merc) {
        s.inner.level = state.level;
        s.inner.resolved.life_max = life.max(1.0);
        s.inner.resolved.damage_min = dmg_min.round().max(0.0) as i32;
        s.inner.resolved.damage_max = dmg_max.round().max(1.0) as i32;
        s.inner.resolved.defense = defense.round().max(0.0) as i32;
        s.inner.resolved.block_chance = block;
        s.inner.attributes.dexterity = dodge.round().max(0.0) as i32;
    }
    if let Some(h) = world.health.get_mut(&merc) {
        h.set_max(life.max(1.0));
    }
}

/// Spawn a mercenary of `merc_type` at `level`, owned by `owner`, near the
/// owner. Equips the archetype's starting weapon. Returns the new entity.
pub fn spawn_mercenary(
    world: &mut World,
    owner: Entity,
    merc_type: MercType,
    level: u32,
) -> Option<Entity> {
    let owner_pos = world.positions.get(&owner).copied()?;
    let profile = merc_profile(merc_type);
    let e = world.spawn();

    let pos = free_tile_near(world, owner_pos).unwrap_or(owner_pos);
    world.positions.insert(e, pos);
    world.factions.insert(e, Faction::Player);
    world.stats.insert(e, Stats::new(merc_class(merc_type), level));
    world.health.insert(e, Health::full(profile.base_hp));
    world.combat.insert(e, Combat::new(profile.attack_range, profile.attack_interval));

    // Starting weapon in the mercenary's equipment.
    let mut inv = MercInventory::new();
    let base = base_item(profile.weapon, level.max(1));
    let weapon_name = format!("{} {}", merc_type.display_name(), base.name);
    let item = Item::new(e.0 * 7919, base, Rarity::Normal, level.max(1));
    let _ = inv.equip(ItemInstance::new(item, ItemAffixes::new(), Vec::new(), weapon_name));
    // Off-hand weapon for dual-wield archetypes (e.g. the Guardian): forced into
    // the off-hand slot since its natural slot would be MainHand.
    if let Some(off_kind) = profile.offhand {
        let off_base = base_item(off_kind, level.max(1));
        let off_name = format!("{} {}", merc_type.display_name(), off_base.name);
        let off_item = Item::new(e.0 * 7919 + 1, off_base, Rarity::Normal, level.max(1));
        inv.equip_to(EquipSlot::OffHand, ItemInstance::new(off_item, ItemAffixes::new(), Vec::new(), off_name));
    }
    world.merc_inventory.insert(e, inv);

    let player_xp = world.progression.get(&owner).map_or(0, |p| p.experience());
    world.merc_states.insert(e, MercState::new(level, player_xp));
    world.mercenary_tags.insert(e, MercenaryTag::new(merc_type, owner, true));

    recompute_merc_stats(world, e);
    if let Some(h) = world.health.get_mut(&e) {
        h.current = h.max;
    }
    world.log_event(format!("{} joins your party (Lv.{level}).", merc_type.display_name()));
    Some(e)
}

/// Recruit a mercenary at the village. Costs gold, releases any current merc,
/// and only works in a safe zone. Returns whether the hire succeeded.
pub fn recruit_mercenary(world: &mut World, player: Entity, merc_type: MercType) -> bool {
    if !world.zone.is_safe() {
        world.log_event("Mercenaries can only be hired at the village.".to_string());
        return false;
    }
    // One companion per archetype, up to the party cap.
    let party = companions(world, player);
    if party.iter().any(|&e| world.mercenary_tags.get(&e).is_some_and(|t| t.merc_type == merc_type)) {
        world.log_event(format!("A {} already serves you.", merc_type.display_name()));
        return false;
    }
    if party.len() >= MAX_COMPANIONS {
        world.log_event(format!("Your party is full (max {MAX_COMPANIONS})."));
        return false;
    }
    let profile = merc_profile(merc_type);
    let gold = world.inventory.get(&player).map_or(0, |i| i.gold);
    if gold < profile.recruit_cost {
        world.log_event("Not enough gold to hire.".to_string());
        return false;
    }

    if let Some(inv) = world.inventory.get_mut(&player) {
        inv.gold -= profile.recruit_cost;
    }
    let hired = spawn_mercenary(world, player, merc_type, 1).is_some();
    if hired {
        world.log_event(format!(
            "Hired a {} for {} gold.",
            merc_type.display_name(),
            profile.recruit_cost
        ));
    }
    hired
}

/// Maximum simultaneous companions a player may field.
pub const MAX_COMPANIONS: usize = 3;

/// Every mercenary owned by `player`, in a stable order (by entity id) so the
/// party roster / selector tabs don't shuffle between frames.
#[must_use]
pub fn companions(world: &World, player: Entity) -> Vec<Entity> {
    let mut v: Vec<Entity> = world
        .mercenary_entities()
        .into_iter()
        .filter(|e| world.mercenary_tags.get(e).is_some_and(|t| t.owner == player))
        .collect();
    v.sort_unstable();
    v
}

/// The player's first (lowest-id) mercenary, if any.
#[must_use]
pub fn active_mercenary(world: &World, player: Entity) -> Option<Entity> {
    companions(world, player).into_iter().next()
}

// ── AI ───────────────────────────────────────────────────────────────────────

/// Follow the player, engage nearby foes, and run per-archetype abilities.
pub fn mercenary_ai_system(world: &mut World) {
    let dt = world.dt;
    let Some(player) = world.player_entity else { return };
    let Some(&ppos) = world.positions.get(&player) else { return };
    // One living-mob capture for every merc's targeting/cast query this frame.
    let mobs = MobSnapshot::build(world);

    for e in world.mercenary_entities() {
        let Some(tag) = world.mercenary_tags.get(&e).copied() else { continue };
        let Some(mut state) = world.merc_states.get(&e).copied() else { continue };
        if state.is_dead() {
            continue;
        }
        let Some(&pos) = world.positions.get(&e) else { continue };

        // Age ability cooldowns.
        state.heal_cd = (state.heal_cd - dt).max(0.0);
        state.cast_cd = (state.cast_cd - dt).max(0.0);

        // Pick a foe within the engage bubble around the player.
        let guardian = tag.merc_type == MercType::Guardian;
        let target = engage_target(world, &mobs, player, ppos, guardian);

        match tag.merc_type {
            MercType::Healer => {
                let acted = try_heal(world, e, player, ppos, &mut state);
                if !acted {
                    set_target(world, e, target);
                    chase_or_follow(world, e, pos, ppos, target);
                }
            }
            MercType::Mage => {
                let cast = try_cast(world, &mobs, e, target, &mut state);
                set_target(world, e, target);
                if !cast {
                    chase_or_follow(world, e, pos, ppos, target);
                } else {
                    world.move_targets.remove(&e);
                }
            }
            MercType::Guardian | MercType::Archer => {
                set_target(world, e, target);
                chase_or_follow(world, e, pos, ppos, target);
            }
        }

        world.merc_states.insert(e, state);
    }
}

/// The foe a merc should engage: nearest mob within [`ENGAGE_RADIUS`] of the
/// player. A guardian prefers mobs currently attacking the player.
fn engage_target(
    world: &World,
    mobs: &MobSnapshot,
    player: Entity,
    ppos: Position2D,
    guardian: bool,
) -> Option<Entity> {
    let in_range = mobs.within_alive(world, ppos, ENGAGE_RADIUS);
    // A guardian prefers mobs currently attacking the player, when any do.
    let attacks_player = |e: &Entity| world.combat.get(e).is_some_and(|c| c.target == Some(player));
    let prefer_attackers = guardian && in_range.iter().any(attacks_player);
    in_range
        .into_iter()
        .filter(|e| !prefer_attackers || attacks_player(e))
        .filter_map(|e| world.positions.get(&e).map(|p| (e, ppos.distance(*p))))
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, _)| e)
}

fn set_target(world: &mut World, merc: Entity, target: Option<Entity>) {
    if let Some(c) = world.combat.get_mut(&merc) {
        c.target = target;
    }
}

/// Chase the current target if out of reach, otherwise trail the player.
fn chase_or_follow(
    world: &mut World,
    merc: Entity,
    pos: Position2D,
    ppos: Position2D,
    target: Option<Entity>,
) {
    let range = world.combat.get(&merc).map_or(1.6, |c| c.range);
    let speed = merc_profile(world.mercenary_tags[&merc].merc_type).move_speed;

    if let Some(target) = target {
        if let Some(&tpos) = world.positions.get(&target) {
            if pos.distance(tpos) > range {
                repath(world, merc, pos, tpos, speed);
            } else {
                world.move_targets.remove(&merc);
            }
            return;
        }
    }

    // No foe: keep close to the player.
    if pos.distance(ppos) > FOLLOW_MAX {
        repath(world, merc, pos, ppos, speed);
    } else if pos.distance(ppos) <= FOLLOW_KEEP {
        world.move_targets.remove(&merc);
    } else if world.move_targets.get(&merc).is_none() {
        repath(world, merc, pos, ppos, speed);
    }
}

fn repath(world: &mut World, e: Entity, from: Position2D, to: Position2D, speed: f32) {
    let need = world.move_targets.get(&e).is_none() || world.tick % REPATH_TICKS == 0;
    if !need {
        return;
    }
    if let Some(path) = find_path(&world.map, from.tile(), to.tile()) {
        if !path.is_empty() {
            world.move_targets.insert(e, MoveTarget::new(to, path, speed));
        }
    }
}

/// Healer: restore the player when they drop below half health. Returns whether
/// a heal fired this tick.
fn try_heal(
    world: &mut World,
    merc: Entity,
    player: Entity,
    ppos: Position2D,
    state: &mut MercState,
) -> bool {
    if state.heal_cd > 0.0 {
        return false;
    }
    let (cur, max) = match world.health.get(&player) {
        Some(h) => (h.current, h.max),
        None => return false,
    };
    if max <= 0.0 || cur / max >= HEAL_THRESHOLD {
        return false;
    }
    #[allow(clippy::cast_precision_loss)]
    let amount = max * HEAL_FRAC + state.level as f32;
    if let Some(h) = world.health.get_mut(&player) {
        h.restore(amount);
    }
    state.heal_cd = HEAL_CD;
    world.render_events.push(RenderEvent::Skill {
        caster: merc,
        from: world.positions.get(&merc).copied().unwrap_or(ppos),
        to: ppos,
        fx: SkillFx::Buff,
    });
    world.log_event(format!("Healer mends you for {amount:.0}."));
    true
}

/// Mage: lob an elemental AoE on the target's cluster. Returns whether it fired.
fn try_cast(
    world: &mut World,
    mobs: &MobSnapshot,
    merc: Entity,
    target: Option<Entity>,
    state: &mut MercState,
) -> bool {
    if state.cast_cd > 0.0 {
        return false;
    }
    let Some(target) = target else { return false };
    let Some(center) = world.positions.get(&target).copied() else { return false };

    let foes = mobs.within_alive(world, center, CAST_RADIUS);
    if foes.is_empty() {
        return false;
    }
    for f in &foes {
        resolve_attack(world, merc, *f, CAST_MULT);
    }
    state.cast_cd = CAST_CD;
    let from = world.positions.get(&merc).copied().unwrap_or(center);
    world.render_events.push(RenderEvent::Skill { caster: merc, from, to: center, fx: SkillFx::Fire });
    world.log_event(format!("Mage scorches {} foes.", foes.len()));
    true
}

// ── Lifecycle: XP, levelling, death + respawn ─────────────────────────────────

/// Share the player's XP, level the merc up, and run the death/respawn clock.
pub fn mercenary_update_system(world: &mut World) {
    let dt = world.dt;
    let Some(player) = world.player_entity else { return };
    let player_pos = world.positions.get(&player).copied();
    let player_xp = world.progression.get(&player).map_or(0, |p| p.experience());

    for e in world.mercenary_entities() {
        let Some(mut state) = world.merc_states.get(&e).copied() else { continue };

        // Respawning.
        if state.is_dead() {
            state.respawn_timer -= dt;
            if state.respawn_timer <= 0.0 {
                state.respawn_timer = 0.0;
                if let Some(spot) = player_pos.and_then(|p| free_tile_near(world, p)) {
                    world.positions.insert(e, spot);
                }
                if let Some(h) = world.health.get_mut(&e) {
                    h.current = h.max;
                }
                let name = world
                    .mercenary_tags
                    .get(&e)
                    .map_or("Mercenary", |t| t.merc_type.display_name());
                world.log_event(format!("{name} returns to your side."));
            }
            world.merc_states.insert(e, state);
            continue;
        }

        // Death.
        if world.health.get(&e).is_some_and(|h| h.is_empty()) {
            state.respawn_timer = RESPAWN_TIME;
            world.move_targets.remove(&e);
            if let Some(c) = world.combat.get_mut(&e) {
                c.target = None;
            }
            if let Some(&pos) = world.positions.get(&e) {
                world.render_events.push(RenderEvent::Death { pos, kind: VisualKind::Player });
            }
            let name = world
                .mercenary_tags
                .get(&e)
                .map_or("Mercenary", |t| t.merc_type.display_name());
            world.log_event(format!("{name} has fallen! Respawning in {RESPAWN_TIME:.0}s."));
            world.merc_states.insert(e, state);
            continue;
        }

        // XP share + level up.
        let gained = player_xp.saturating_sub(state.player_xp_mark);
        if gained > 0 {
            state.xp += gained / 2;
            state.player_xp_mark = player_xp;
            let mut levelled = false;
            while state.xp >= xp_to_next(state.level) {
                state.xp -= xp_to_next(state.level);
                state.level += 1;
                levelled = true;
            }
            world.merc_states.insert(e, state);
            if levelled {
                recompute_merc_stats(world, e);
                let name = world
                    .mercenary_tags
                    .get(&e)
                    .map_or("Mercenary", |t| t.merc_type.display_name());
                world.log_event(format!("{name} reaches Lv.{}.", state.level));
            }
        } else {
            world.merc_states.insert(e, state);
        }
    }
}

/// First walkable tile in a ring around `center`.
fn free_tile_near(world: &World, center: Position2D) -> Option<Position2D> {
    let (cx, cy) = center.tile();
    let offsets = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1), (2, 0), (0, 2)];
    offsets
        .into_iter()
        .find(|(dx, dy)| world.map.is_walkable(cx + dx, cy + dy))
        .map(|(dx, dy)| Position2D::from_tile(cx + dx, cy + dy))
}
