//! @id     games_allumina_systems_cc
//! @do     definir_cast_window
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « systems cc ».

//! Crowd-control ticking: renversement (knockdown), recul (pushback), and the
//! interruption check. The application helpers (`apply_knockdown`,
//! `apply_pushback*`, `try_interrupt`) live in [`crate::systems::combat`]; these
//! systems only advance and resolve effects already in flight.

use crate::components::Position2D;
use crate::ecs::{Entity, RenderEvent, World};
use crate::systems::combat::{apply_knockdown, try_interrupt};

/// Seconds an interruptible cast window stays open before it resolves and the
/// entity stops being interruptible.
const CAST_WINDOW: f32 = 0.5;

/// Age every knockdown, removing the expired ones. While down, the entity holds
/// no path (movement/AI already treat it as frozen).
pub fn knockdown_system(world: &mut World) {
    let dt = world.dt;
    for e in world.knockdowns.keys().copied().collect::<Vec<_>>() {
        let Some(k) = world.knockdowns.get_mut(&e) else { continue };
        k.remaining -= dt;
        if k.remaining <= 0.0 {
            world.knockdowns.remove(&e);
        } else {
            world.move_targets.remove(&e);
        }
    }
}

/// Slide every pushed entity along its recul vector. A wall stops it early (with
/// bonus damage); reaching the full distance ends it. Either way an optional
/// `knockdown_after` chains a renversement on arrival.
pub fn pushback_system(world: &mut World) {
    let dt = world.dt;
    if dt <= 0.0 {
        return;
    }
    for e in world.pushbacks.keys().copied().collect::<Vec<_>>() {
        let Some(pb) = world.pushbacks.get(&e).copied() else { continue };
        let Some(pos) = world.positions.get(&e).copied() else {
            world.pushbacks.remove(&e);
            continue;
        };

        let remaining = (pb.distance - pb.traveled).max(0.0);
        let step = (pb.speed * dt).min(remaining);
        let cand = Position2D::new(pos.x + pb.dx * step, pos.y + pb.dy * step);
        let (tx, ty) = cand.tile();

        // Slam into a wall: stop dead, deal the bonus damage, then knock down.
        if !world.map.is_walkable(tx, ty) {
            world.pushbacks.remove(&e);
            wall_slam(world, e, pos, pb.wall_bonus_dmg);
            if let Some(dur) = pb.knockdown_after {
                apply_knockdown(world, e, dur);
            }
            continue;
        }

        world.positions.insert(e, cand);
        let traveled = pb.traveled + step;
        if traveled >= pb.distance - 1e-4 {
            world.pushbacks.remove(&e);
            if let Some(dur) = pb.knockdown_after {
                apply_knockdown(world, e, dur);
            }
        } else if let Some(p) = world.pushbacks.get_mut(&e) {
            p.traveled = traveled;
        }
    }
}

/// Deal the flat wall-collision bonus to `e` (self-attributed in the damage feed).
fn wall_slam(world: &mut World, e: Entity, pos: Position2D, bonus: f64) {
    if bonus <= 0.0 || world.health.get(&e).map_or(true, |h| h.is_empty()) {
        return;
    }
    #[allow(clippy::cast_possible_truncation)]
    let amount = bonus as f32;
    if let Some(h) = world.health.get_mut(&e) {
        h.drain(amount);
    }
    world.render_events.push(RenderEvent::Damage {
        attacker: e,
        target: e,
        pos,
        amount,
        crit: false,
        dodged: false,
    });
    world.log_event("Slammed into a wall!".to_string());
}

/// Resolve interruptions: an entity that picks up a stun or knockdown this tick
/// has its cast cancelled, and any cast window that runs its course closes (after
/// which the entity is no longer interruptible).
pub fn interrupt_system(world: &mut World) {
    let dt = world.dt;
    for e in world.interruptibles.keys().copied().collect::<Vec<_>>() {
        let stunned = world.stuns.get(&e).is_some_and(|&t| t > 0.0);
        let knocked = world.knockdowns.contains_key(&e);
        if stunned || knocked {
            try_interrupt(world, e);
            continue;
        }
        if let Some(i) = world.interruptibles.get_mut(&e) {
            i.progress += dt / CAST_WINDOW;
            if i.progress >= 1.0 {
                world.interruptibles.remove(&e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::cc::{Interruptible, Knockdown};
    use crate::components::mob::{MobTag, MobType};
    use crate::components::skill::{ActiveSkill, Cooldown, SkillEffect, SkillKind, SkillSlot, Unstoppable};
    use crate::components::Health;
    use crate::data::maps::{dungeon_map, village_map};
    use crate::data::GameData;
    use crate::ecs::SkillFx;
    use crate::systems::combat::{apply_knockdown, apply_pushback, apply_pushback_then_knockdown};

    fn test_world() -> World {
        let mut w = World::new(village_map(), GameData::load(), 7);
        w.dt = 0.1;
        w
    }

    fn spawn_mob(world: &mut World, pos: Position2D) -> Entity {
        let e = world.spawn();
        world.mob_tags.insert(e, MobTag);
        world.positions.insert(e, pos);
        world.health.insert(e, Health::full(100.0));
        e
    }

    #[test]
    fn knockdown_expires_after_its_duration() {
        let mut world = test_world();
        let e = world.spawn();
        world.knockdowns.insert(e, Knockdown::new(0.25));
        knockdown_system(&mut world); // 0.15 left
        assert!(world.knockdowns.contains_key(&e));
        knockdown_system(&mut world); // 0.05 left
        knockdown_system(&mut world); // expired
        assert!(!world.knockdowns.contains_key(&e));
    }

    #[test]
    fn apply_knockdown_ignores_unstoppable() {
        let mut world = test_world();
        let e = world.spawn();
        world.unstoppable.insert(e, Unstoppable { remaining: 1.0 });
        apply_knockdown(&mut world, e, 2.0);
        assert!(!world.knockdowns.contains_key(&e));
    }

    #[test]
    fn bosses_ignore_pushback_but_still_take_knockdown() {
        // Bosses are flagged via MobDef::is_boss; the only one in Act I lives in
        // the Dungeon map. Find its def_id from the data table.
        let data = GameData::load();
        let boss_def_id =
            (1..1000).find(|id| data.mob(*id).is_some_and(|d| d.is_boss)).expect("a boss def");

        let mut world = World::new(dungeon_map(), data, 7);
        world.dt = 0.1;
        let e = world.spawn();
        world.mob_tags.insert(e, MobTag);
        world.mob_types.insert(e, MobType { def_id: boss_def_id });
        world.positions.insert(e, Position2D::new(10.0, 10.0));
        world.health.insert(e, Health::full(1000.0));

        let start = world.positions.get(&e).unwrap().x;
        apply_pushback(&mut world, e, 1.0, 0.0, 5.0, 5.0);
        assert!(world.pushbacks.get(&e).is_none(), "boss should never enter pushback state");
        for _ in 0..10 {
            pushback_system(&mut world);
        }
        let end = world.positions.get(&e).unwrap().x;
        assert!((end - start).abs() < 1e-4, "boss should not have moved ({start} -> {end})");

        // Knockdown still applies — only the recul is blocked.
        apply_knockdown(&mut world, e, 1.0);
        assert!(world.knockdowns.contains_key(&e), "boss should still be knocked down");
    }

    #[test]
    fn pushback_travels_then_knocks_down() {
        let mut world = test_world();
        let e = spawn_mob(&mut world, Position2D::new(10.0, 10.0));
        // 2 tiles east at 5 tiles/s, knockdown 1s on arrival. Open ground.
        apply_pushback_then_knockdown(&mut world, e, 1.0, 0.0, 2.0, 5.0, 1.0, 0.0);
        let start = world.positions.get(&e).unwrap().x;
        // Run enough ticks (5 t/s, 2 tiles → 0.4s → 4 ticks of 0.1s).
        for _ in 0..6 {
            pushback_system(&mut world);
        }
        let end = world.positions.get(&e).unwrap().x;
        assert!(end > start, "entity should have moved east ({start} -> {end})");
        assert!(world.pushbacks.get(&e).is_none(), "recul should have ended");
        assert!(world.knockdowns.contains_key(&e), "knockdown should chain after");
    }

    #[test]
    fn interrupt_refunds_half_cooldown_and_clears_cast() {
        let mut world = test_world();
        let e = world.spawn();
        world.positions.insert(e, Position2D::new(0.0, 0.0));
        let skill = ActiveSkill {
            id: 4242,
            name: "Bolt".into(),
            level: 1,
            kind: SkillKind::Spell,
            mana_cost: 1.0,
            cooldown: 4.0,
            damage_mult: 1.0,
            range: 6.0,
            effect: SkillEffect::Fireball,
            fx: SkillFx::Fire,
        };
        let mut slot = SkillSlot::from_skills(vec![skill]);
        slot.cooldowns[0] = Cooldown { remaining: 4.0 };
        world.skills.insert(e, slot);
        world.interruptibles.insert(e, Interruptible::cast(4242));

        // A stun landing this tick interrupts the cast.
        world.stuns.insert(e, 1.0);
        interrupt_system(&mut world);
        assert!(world.interruptibles.get(&e).is_none(), "cast cancelled");
        let cd = world.skills.get(&e).unwrap().cooldowns[0].remaining;
        assert!((cd - 2.0).abs() < 1e-4, "cooldown refunded to 50% (got {cd})");
    }

    #[test]
    fn cast_window_closes_on_its_own() {
        let mut world = test_world();
        let e = world.spawn();
        world.interruptibles.insert(e, Interruptible::cast(1));
        // CAST_WINDOW is 0.5s; at 0.1s/tick it takes 5 ticks to close.
        for _ in 0..5 {
            interrupt_system(&mut world);
        }
        assert!(world.interruptibles.get(&e).is_none());
    }
}
