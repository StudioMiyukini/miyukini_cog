//! @id     games_allumina_systems_projectile
//! @do     definir_arrow_speed
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « systems projectile ».

//! Homing projectiles spawned by ranged auto-attacks.
//!
//! On each tick every projectile steers toward its target's current position,
//! advancing by `speed * dt` tiles. A projectile resolves the moment it
//! crosses [`HIT_RADIUS`] of its target, fizzles if the target dies before
//! impact, and times out after [`MAX_RANGE_TILES`] of accumulated flight.

use crate::components::projectile::{Projectile, HIT_RADIUS, MAX_RANGE_TILES};
use crate::components::skill::Element;
use crate::components::Position2D;
use crate::ecs::{Entity, RenderEvent, World};
use crate::systems::combat::{attack_rating, block_chance, effective_damage, effective_defense, entity_name, mitigate, try_interrupt};

/// Tiles/s a physical (arrow/bolt) projectile travels.
pub const ARROW_SPEED: f32 = 15.0;
/// Tiles/s a magic projectile travels (slightly slower than an arrow).
pub const MAGIC_SPEED: f32 = 12.0;

/// Roll a damage payload for `owner` and push a fresh [`Projectile`] toward
/// `target`. Returns the rolled damage (`0.0` if positions are missing —
/// caller should treat that as a no-op).
///
/// The roll mirrors [`crate::systems::combat::resolve_attack`]'s offensive
/// pipeline (base + weapon, primary scaling, buffs/order/aura, optional crit)
/// but defers all defender-side mitigation to impact resolution.
#[allow(clippy::cast_precision_loss)]
pub fn spawn_projectile(world: &mut World, owner: Entity, target: Entity, is_magic: bool) -> f32 {
    let (Some(&opos), Some(_tpos)) = (world.positions.get(&owner), world.positions.get(&target))
    else {
        return 0.0;
    };

    let (dmg_min, dmg_max) = effective_damage(world, owner);
    let roll = world.rng.next_f32();
    let base = dmg_min + (dmg_max - dmg_min).max(0.0) * roll;

    let buff_mult = 1.0 + world.buffs.get(&owner).map_or(0.0, |b| b.atk_pct);
    let order_dmg = 1.0 + world.order_bonuses(owner).dmg_pct;
    let aura_dmg = 1.0 + world.aura_bonuses.get(&owner).map_or(0.0, |a| a.dmg_pct);
    let raw = base * buff_mult * order_dmg * aura_dmg;

    let crit_pct = crit_chance_pct(world, owner);
    let crit = world.rng.chance_pct(crit_pct);
    let damage = if crit { raw * 1.5 } else { raw };

    world.projectiles.push(Projectile {
        owner,
        target,
        x: opos.x,
        y: opos.y,
        speed: if is_magic { MAGIC_SPEED } else { ARROW_SPEED },
        damage,
        is_magic,
        crit,
        travelled: 0.0,
    });
    damage
}

/// Crit % for `e`, mirroring `gather_attacker`'s fold (base 5 + resolved +
/// hunter-instinct buff).
fn crit_chance_pct(world: &World, e: Entity) -> u32 {
    if let Some(stats) = world.stats.get(&e) {
        let base = (5 + stats.inner.resolved.crit_chance).max(0) as u32;
        return base + world.hunter_instinct.get(&e).map_or(0, |b| b.crit_pct);
    }
    5 + world.hunter_instinct.get(&e).map_or(0, |b| b.crit_pct)
}

/// Step every in-flight projectile by `dt`, applying damage on impact and
/// removing finished/expired projectiles in-place.
pub fn projectile_system(world: &mut World) {
    let dt = world.dt;
    if dt <= 0.0 || world.projectiles.is_empty() {
        return;
    }

    let mut surviving: Vec<Projectile> = Vec::with_capacity(world.projectiles.len());
    for mut proj in std::mem::take(&mut world.projectiles) {
        // The target may have died or de-spawned mid-flight — fizzle silently.
        let Some(&tpos) = world.positions.get(&proj.target) else {
            continue;
        };
        if world.health.get(&proj.target).map_or(true, |h| h.is_empty()) {
            continue;
        }

        let dx = tpos.x - proj.x;
        let dy = tpos.y - proj.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist <= HIT_RADIUS {
            resolve_impact(world, &proj);
            continue;
        }

        let step = proj.speed * dt;
        if step >= dist {
            // This tick takes us past the target — snap to it and resolve.
            proj.x = tpos.x;
            proj.y = tpos.y;
            proj.travelled += dist;
            resolve_impact(world, &proj);
            continue;
        }
        let inv = 1.0 / dist;
        proj.x += dx * inv * step;
        proj.y += dy * inv * step;
        proj.travelled += step;

        if proj.travelled >= MAX_RANGE_TILES {
            // Out of range / "miss".
            continue;
        }

        surviving.push(proj);
    }
    world.projectiles = surviving;
}

/// Apply `proj`'s pre-rolled damage to its target through the standard
/// mitigation pipeline. Magic projectiles bypass physical defence reduction.
fn resolve_impact(world: &mut World, proj: &Projectile) {
    let target_pos = world
        .positions
        .get(&proj.target)
        .copied()
        .unwrap_or(Position2D::new(proj.x, proj.y));

    // Invulnerable defenders eat the projectile with no damage.
    if world.invuln.get(&proj.target).is_some_and(|&t| t > 0.0) {
        world.render_events.push(RenderEvent::Damage {
            attacker: proj.owner,
            target: proj.target,
            pos: target_pos,
            amount: 0.0,
            crit: false,
            dodged: true,
        });
        return;
    }

    // A shield (or dual-wield parry) can knock a physical projectile aside.
    // Magic projectiles can't be physically blocked.
    if !proj.is_magic {
        let block = block_chance(world, proj.target);
        if block > 0.0 && world.rng.next_f32() < block {
            world.render_events.push(RenderEvent::Block { pos: target_pos });
            return;
        }
    }

    let attacker_ar = attack_rating(world, proj.owner);
    let blessing = world.resist.get(&proj.target).map_or(0.0, |r| r.pct);
    let aura_resist = world
        .aura_bonuses
        .get(&proj.target)
        .map_or(0.0, |a| a.resist_pct);
    let resist = (blessing + aura_resist).clamp(0.0, 0.9);
    let knock_amp = world.knockdowns.get(&proj.target).map_or(1.0, |k| k.dmg_amp as f32);

    let mitigated = if proj.is_magic {
        // Magic projectile: physical defence is ignored, blessings/auras still
        // apply, no elemental flavour (Element::Physical → 0 elem resist).
        (proj.damage * (1.0 - resist)).max(1.0)
    } else {
        let def = effective_defense(world, proj.target);
        mitigate(proj.damage, def, attacker_ar, resist, 0.0)
    };
    let damage = mitigated * knock_amp;

    if let Some(h) = world.health.get_mut(&proj.target) {
        h.drain(damage);
    }
    world.render_events.push(RenderEvent::Damage {
        attacker: proj.owner,
        target: proj.target,
        pos: target_pos,
        amount: damage,
        crit: proj.crit,
        dodged: false,
    });
    // Any landed projectile damage interrupts an in-progress cast.
    try_interrupt(world, proj.target);

    let att = entity_name(world, proj.owner);
    let def = entity_name(world, proj.target);
    let tag = if proj.crit { " (CRIT)" } else { "" };
    world.log_event(format!("{att} hits {def} for {damage:.0}{tag}"));

    // Silence unused-import warning when no other elemental path runs.
    let _ = Element::Physical;
}

#[cfg(test)]
mod tests {
    use mge_items::affixes::ItemAffixes;
    use mge_items::item::{Item, ItemKind};
    use mge_items::rarity::Rarity;

    use super::*;
    use crate::components::equipment::EquipmentSlots;
    use crate::components::item::ItemInstance;
    use crate::components::mob::MobTag;
    use crate::components::player::{ClassType, PlayerTag, Stats};
    use crate::components::{Combat, Faction, Health, Position2D};
    use crate::data::items::base_item;
    use crate::data::maps::village_map;
    use crate::data::GameData;
    use crate::systems::combat::combat_system;

    fn make_world() -> World {
        let mut w = World::new(village_map(), GameData::load(), 7);
        w.dt = 0.1;
        w
    }

    fn spawn_player_with_weapon(world: &mut World, kind: ItemKind) -> Entity {
        let e = world.spawn();
        world.player_entity = Some(e);
        world.positions.insert(e, Position2D::new(10.0, 10.0));
        world.factions.insert(e, Faction::Player);
        world.player_tags.insert(e, PlayerTag);
        world.classes.insert(e, ClassType::Archer);
        let stats = Stats::new(ClassType::Archer, 1);
        let life = stats.inner.resolved.life_max;
        world.health.insert(e, Health::full(life));
        world.stats.insert(e, stats);
        let mut eq = EquipmentSlots::new();
        let base = base_item(kind, 1);
        let item = Item::new(e.0 * 1_000, base, Rarity::Normal, 1);
        let weapon = ItemInstance::new(item, ItemAffixes::new(), Vec::new(), "Test Weapon".into());
        let _ = eq.equip(weapon);
        world.equipment.insert(e, eq);
        // Auto-attack reach matches the weapon (bow = 8 tiles, sword = 1.5).
        world.combat.insert(e, Combat::new(crate::components::item::attack_range(kind), 0.8));
        e
    }

    fn spawn_target_mob(world: &mut World, pos: Position2D, hp: f32) -> Entity {
        let m = world.spawn();
        world.mob_tags.insert(m, MobTag);
        world.positions.insert(m, pos);
        world.health.insert(m, Health::full(hp));
        m
    }

    #[test]
    fn ranged_weapon_spawns_a_projectile_instead_of_resolving_immediately() {
        let mut world = make_world();
        let player = spawn_player_with_weapon(&mut world, ItemKind::Bow);
        // Mob 5 tiles away — inside bow range (8) but outside melee.
        let mob = spawn_target_mob(&mut world, Position2D::new(15.0, 10.0), 50.0);

        let hp_before = world.health.get(&mob).unwrap().current;
        world.combat.get_mut(&player).unwrap().target = Some(mob);

        assert!(world.projectiles.is_empty(), "no projectile before tick");
        combat_system(&mut world);

        // Combat resolution should *defer* damage to the projectile.
        assert_eq!(world.projectiles.len(), 1, "ranged auto-attack should spawn one projectile");
        let proj = world.projectiles[0];
        assert_eq!(proj.owner, player);
        assert_eq!(proj.target, mob);
        assert!(!proj.is_magic, "bow projectile must be physical");
        assert!(proj.damage > 0.0, "projectile must carry a rolled damage payload");
        let hp_after = world.health.get(&mob).unwrap().current;
        assert!(
            (hp_after - hp_before).abs() < 1e-3,
            "target HP must not change at spawn time (damage is applied at impact)",
        );
    }

    #[test]
    fn magic_weapon_spawns_a_magic_projectile() {
        let mut world = make_world();
        let player = spawn_player_with_weapon(&mut world, ItemKind::Staff);
        let mob = spawn_target_mob(&mut world, Position2D::new(14.0, 10.0), 60.0);
        world.combat.get_mut(&player).unwrap().target = Some(mob);

        combat_system(&mut world);

        assert_eq!(world.projectiles.len(), 1);
        assert!(world.projectiles[0].is_magic, "staff must fire a magic projectile");
    }

    #[test]
    fn melee_weapon_does_not_spawn_a_projectile() {
        let mut world = make_world();
        let player = spawn_player_with_weapon(&mut world, ItemKind::Sword);
        // Inside the 1.5-tile melee reach.
        let mob = spawn_target_mob(&mut world, Position2D::new(11.0, 10.0), 50.0);

        let hp_before = world.health.get(&mob).unwrap().current;
        world.combat.get_mut(&player).unwrap().target = Some(mob);
        combat_system(&mut world);

        assert!(world.projectiles.is_empty(), "melee swing must not spawn a projectile");
        let hp_after = world.health.get(&mob).unwrap().current;
        assert!(hp_after < hp_before, "melee swing resolves damage instantly");
    }

    #[test]
    fn projectile_disappears_if_target_dies_mid_flight() {
        let mut world = make_world();
        let player = spawn_player_with_weapon(&mut world, ItemKind::Bow);
        let mob = spawn_target_mob(&mut world, Position2D::new(18.0, 10.0), 30.0);
        world.combat.get_mut(&player).unwrap().target = Some(mob);
        combat_system(&mut world);
        assert_eq!(world.projectiles.len(), 1, "one projectile in flight");

        // Kill the mob mid-flight.
        world.health.get_mut(&mob).unwrap().drain(9_999.0);

        // Tick the projectile system: should fizzle silently.
        world.dt = 0.05;
        projectile_system(&mut world);
        assert!(
            world.projectiles.is_empty(),
            "projectile must despawn when its target dies before impact",
        );
    }

    #[test]
    fn projectile_homes_in_and_damages_target_on_impact() {
        let mut world = make_world();
        let player = spawn_player_with_weapon(&mut world, ItemKind::Bow);
        let mob = spawn_target_mob(&mut world, Position2D::new(12.0, 10.0), 200.0);
        world.combat.get_mut(&player).unwrap().target = Some(mob);
        combat_system(&mut world);
        let hp_at_spawn = world.health.get(&mob).unwrap().current;

        // Step projectiles forward generously — at 15 tiles/s and ~2 tiles of
        // separation, a handful of 50ms ticks is plenty for impact.
        world.dt = 0.05;
        for _ in 0..40 {
            projectile_system(&mut world);
            if world.projectiles.is_empty() {
                break;
            }
        }
        assert!(world.projectiles.is_empty(), "projectile must impact within the test window");
        let hp_after = world.health.get(&mob).unwrap().current;
        assert!(
            hp_after < hp_at_spawn,
            "projectile impact must apply damage (hp_at_spawn {hp_at_spawn}, hp_after {hp_after})",
        );
    }

    #[test]
    fn projectile_expires_after_max_range() {
        let mut world = make_world();
        let player = spawn_player_with_weapon(&mut world, ItemKind::Bow);
        // Target is well inside attack_range but we will teleport it far away
        // *after* the projectile spawns so the arrow's homing never catches up
        // within MAX_RANGE_TILES.
        let mob = spawn_target_mob(&mut world, Position2D::new(11.0, 10.0), 200.0);
        world.combat.get_mut(&player).unwrap().target = Some(mob);
        combat_system(&mut world);
        assert_eq!(world.projectiles.len(), 1);

        // Yank the target across the map and tick until the projectile gives up.
        world.positions.insert(mob, Position2D::new(200.0, 10.0));
        world.dt = 0.05;
        for _ in 0..400 {
            projectile_system(&mut world);
            if world.projectiles.is_empty() {
                break;
            }
        }
        assert!(world.projectiles.is_empty(), "projectile must expire past MAX_RANGE_TILES");
    }
}
