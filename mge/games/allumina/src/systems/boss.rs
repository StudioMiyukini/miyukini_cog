//! @id     games_allumina_systems_boss
//! @do     definir_slam_cd
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « systems boss ».

//! Boss behaviour: a three-part fight on top of the boss's normal melee.
//!
//! * **Slam** — a periodic ground shockwave that hits anything within
//!   [`SLAM_RADIUS`] of the boss.
//! * **Charge** — when the player is too far to slam but still close enough to
//!   matter, the boss dashes adjacent and strikes.
//! * **Enrage** — a one-way latch at [`ENRAGE_AT`] of max health that speeds up
//!   both specials for the rest of the fight.

use crate::components::mob::BossState;
use crate::components::Position2D;
use crate::ecs::{Entity, RenderEvent, SkillFx, World};
use crate::systems::combat::{apply_knockdown, resolve_attack};

/// Seconds between slams.
const SLAM_CD: f32 = 3.5;
/// Slam radius in tiles.
const SLAM_RADIUS: f32 = 3.2;
/// Slam damage multiplier.
const SLAM_MULT: f32 = 1.8;

/// Seconds between charges.
const CHARGE_CD: f32 = 6.0;
/// Charge fires only when the player sits in this distance band (tiles): beyond
/// slam reach, but not so far the dash is pointless.
const CHARGE_MIN: f32 = 3.4;
const CHARGE_MAX: f32 = 12.0;
/// Charge impact damage multiplier.
const CHARGE_MULT: f32 = 1.4;

/// Health fraction at/below which the boss enrages.
const ENRAGE_AT: f32 = 0.30;
/// Cooldown multiplier once enraged (specials come back twice as fast).
const ENRAGE_RUSH: f32 = 0.5;

pub fn boss_system(world: &mut World) {
    let dt = world.dt;
    let Some(player) = world.player_entity else { return };
    let player_alive = world.health.get(&player).is_some_and(|h| !h.is_empty());
    let ppos = world.positions.get(&player).copied();

    // Decide actions without holding a borrow across the combat calls.
    let mut slams: Vec<(Entity, Position2D)> = Vec::new();
    let mut charges: Vec<(Entity, Position2D)> = Vec::new();
    let mut enraged_now: Vec<Entity> = Vec::new();

    for e in world.mob_entities() {
        let is_boss = world
            .mob_types
            .get(&e)
            .and_then(|m| world.data.mob(m.def_id))
            .is_some_and(|d| d.is_boss);
        if !is_boss {
            continue;
        }
        let Some(&bpos) = world.positions.get(&e) else { continue };
        let hp_frac = world.health.get(&e).map_or(1.0, |h| h.fraction());
        let dist = ppos.map_or(f32::INFINITY, |p| bpos.distance(p));

        let st = world.boss_state.entry(e).or_insert_with(|| BossState::new(SLAM_CD, CHARGE_CD));

        // Enrage is a one-way latch.
        if !st.enraged && hp_frac <= ENRAGE_AT {
            st.enraged = true;
            enraged_now.push(e);
        }
        let rush = if st.enraged { ENRAGE_RUSH } else { 1.0 };

        st.slam_cd -= dt;
        st.charge_cd -= dt;

        if st.slam_cd <= 0.0 {
            st.slam_cd = SLAM_CD * rush;
            if player_alive && dist <= SLAM_RADIUS {
                slams.push((e, bpos));
            }
        }
        if st.charge_cd <= 0.0 && player_alive && dist > CHARGE_MIN && dist <= CHARGE_MAX {
            st.charge_cd = CHARGE_CD * rush;
            if let Some(p) = ppos {
                charges.push((e, p));
            }
        }
    }

    // Charges: dash adjacent to the player, then strike.
    for (boss, target) in charges {
        let Some(&bpos) = world.positions.get(&boss) else { continue };
        let dest = step_toward(bpos, target, (bpos.distance(target) - 1.0).max(0.0));
        let (tx, ty) = dest.tile();
        if world.map.is_walkable(tx, ty) {
            world.positions.insert(boss, dest);
        }
        world.move_targets.remove(&boss);
        resolve_attack(world, boss, player, CHARGE_MULT);
        // Boss attacks never *recul* the player (poise rule cuts both ways),
        // but the charge still knocks the player down briefly on impact.
        apply_knockdown(world, player, 1.0);
        world.render_events.push(RenderEvent::Skill {
            caster: boss,
            from: bpos,
            to: target,
            fx: SkillFx::Charge,
        });
        world.log_event("The Maw charges!".to_string());
    }

    // Slams: the boss's signature special — a shockwave that flattens the player.
    for (boss, bpos) in slams {
        resolve_attack(world, boss, player, SLAM_MULT);
        apply_knockdown(world, player, 2.0);
        world.render_events.push(RenderEvent::Skill {
            caster: boss,
            from: bpos,
            to: bpos,
            fx: SkillFx::Whirlwind,
        });
        world.log_event("The Maw slams the ground — shockwave!".to_string());
    }

    for _ in enraged_now {
        world.log_event("The Maw ENRAGES — its blows come faster!".to_string());
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::player::ClassType;
    use crate::components::world::ZoneId;

    fn dungeon_boss() -> (crate::app::App, Entity) {
        let app = crate::build_app_in(ClassType::Soldat, 2, ZoneId::WerewolfDen);
        let boss = app
            .world
            .mob_entities()
            .into_iter()
            .find(|e| {
                app.world
                    .mob_types
                    .get(e)
                    .and_then(|m| app.world.data.mob(m.def_id))
                    .is_some_and(|d| d.is_boss)
            })
            .expect("dungeon has a boss");
        (app, boss)
    }

    #[test]
    fn step_toward_clamps_to_endpoint() {
        let a = Position2D::new(0.0, 0.0);
        let b = Position2D::new(10.0, 0.0);
        assert!((step_toward(a, b, 4.0).x - 4.0).abs() < 1e-3);
        assert!((step_toward(a, b, 100.0).x - 10.0).abs() < 1e-3);
    }

    #[test]
    fn boss_enrages_below_threshold() {
        let (mut app, boss) = dungeon_boss();
        // Drop the boss to 20% health and stand the player next to it.
        {
            let h = app.world.health.get_mut(&boss).unwrap();
            h.current = h.max * 0.20;
        }
        let bpos = *app.world.positions.get(&boss).unwrap();
        let player = app.world.player_entity.unwrap();
        app.world.positions.insert(player, Position2D::new(bpos.x + 1.0, bpos.y));

        app.world.dt = 1.0 / 30.0;
        for _ in 0..5 {
            boss_system(&mut app.world);
        }
        assert!(app.world.boss_state.get(&boss).is_some_and(|s| s.enraged));
    }

    #[test]
    fn boss_charges_at_a_distant_player() {
        let (mut app, boss) = dungeon_boss();
        // Place the player out of slam range but inside charge range. Driving
        // only `boss_system` (not the full tick) isolates the charge from the
        // normal chase AI.
        let bpos = *app.world.positions.get(&boss).unwrap();
        let player = app.world.player_entity.unwrap();
        let target = Position2D::new(bpos.x - 8.0, bpos.y);
        app.world.positions.insert(player, target);

        app.world.dt = 1.0 / 30.0;
        let before = app.world.positions.get(&boss).unwrap().distance(target);
        // CHARGE_CD is 6 s → ~180 ticks at 30 Hz; run a little longer.
        for _ in 0..210 {
            boss_system(&mut app.world);
        }
        let after = app.world.positions.get(&boss).unwrap().distance(target);
        assert!(after < before, "charge should close distance ({before} -> {after})");
        assert!(app.world.log.iter().any(|l| l.contains("charges")), "charge should be logged");
    }

    #[test]
    fn boss_charge_knocks_down_player_without_pushing() {
        let (mut app, boss) = dungeon_boss();
        let bpos = *app.world.positions.get(&boss).unwrap();
        let player = app.world.player_entity.unwrap();
        // Seat the player on the boss's east-side so charge fires straight along x.
        let target = Position2D::new(bpos.x - 8.0, bpos.y);
        app.world.positions.insert(player, target);

        app.world.dt = 1.0 / 30.0;
        for _ in 0..210 {
            boss_system(&mut app.world);
            // The charge must never seed a recul on the player — boss attacks
            // don't pushback (poise rule applies both ways).
            assert!(
                app.world.pushbacks.get(&player).is_none(),
                "boss charge must not pushback the player"
            );
        }
        assert!(
            app.world.knockdowns.contains_key(&player),
            "boss charge should still knock the player down on impact"
        );
    }
}
