//! @id     games_allumina_systems_death
//! @do     scale_xp_for_level_diff
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « systems death ».

//! Death handling: mobs drop loot and grant XP; the player respawns.

use crate::components::mob::{AiBehavior, AiState};
use crate::components::Position2D;
use crate::ecs::{DeathDrop, RenderEvent, VisualKind, World};

/// Scale `base_xp` based on the player's level relative to the mob's:
/// * `player_level > mob_level + 5` → −10% per extra level above the grace
///   band, floored at 10% of base (so a player can never *gain nothing* from
///   a kill — they just gain a sliver).
/// * `player_level < mob_level` → +5% per level under, capped at +50%.
/// * Otherwise → 1.0 (full XP).
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
pub fn scale_xp_for_level_diff(base_xp: u64, player_level: u32, mob_level: u32) -> u64 {
    if base_xp == 0 {
        return 0;
    }
    let diff = player_level as i32 - mob_level as i32;
    let factor: f64 = if diff > 5 {
        (1.0 - 0.10 * f64::from(diff - 5)).max(0.10)
    } else if diff < 0 {
        (1.0 + 0.05 * f64::from(-diff)).min(1.50)
    } else {
        1.0
    };
    // Round instead of floor — avoids 0.10 * 1 = 0.1000000…06 → 89 from a 100 base.
    let scaled = (base_xp as f64 * factor).round();
    (scaled as u64).max(1)
}

pub fn death_system(world: &mut World) {
    // ── Dead mobs ───────────────────────────────────────────────────────
    for e in world.mob_entities() {
        let dead = world.health.get(&e).is_some_and(|h| h.is_empty());
        if !dead {
            continue;
        }
        if world.aggro.get(&e).is_some_and(|a| a.state == AiState::Dead) {
            world.despawn(e);
            continue;
        }

        let pos = world.positions.get(&e).copied();
        let def_id = world.mob_types.get(&e).map(|m| m.def_id);
        let (Some(pos), Some(def_id)) = (pos, def_id) else {
            world.despawn(e);
            continue;
        };

        let (base_xp, mob_level, name, is_boss) = world
            .data
            .mob(def_id)
            .map_or((0, 1, String::new(), false), |d| (d.xp_reward, d.level, d.name.clone(), d.is_boss));
        let elite = world.elite_mobs.contains(&e);
        let raw = if elite { base_xp.saturating_mul(2) } else { base_xp };
        let player_level = world
            .player_entity
            .and_then(|p| world.progression.get(&p).map(|pr| pr.level()))
            .unwrap_or(1);
        let xp = scale_xp_for_level_diff(raw, player_level, mob_level);

        let kind = if is_boss {
            VisualKind::Boss
        } else {
            match world.aggro.get(&e).map(|a| a.behavior) {
                Some(AiBehavior::Ranged) => VisualKind::Ranged,
                Some(AiBehavior::Caster) => VisualKind::Caster,
                _ => VisualKind::Melee,
            }
        };
        world.render_events.push(RenderEvent::Death { pos, kind });
        world.quests.record_kill(is_boss);
        world.loot_queue.push(DeathDrop { pos, mob_def_id: def_id, mob_level, elite });

        // Remember this kill so the spawn stays empty next time the player
        // re-enters this zone.
        if let Some(&sid) = world.spawn_ids.get(&e) {
            let key = world.zone.key().to_string();
            let entry = world.killed_mobs.entry(key).or_default();
            if !entry.contains(&sid) {
                entry.push(sid);
            }
        }

        if let Some(player) = world.player_entity {
            if let Some(prog) = world.progression.get_mut(&player) {
                prog.inner.grant_xp(xp);
            }
        }
        world.log_event(format!("{name} is slain!  (+{xp} XP)"));
        world.despawn(e);
    }

    // ── Player death → respawn ──────────────────────────────────────────
    let Some(player) = world.player_entity else { return };
    if world.health.get(&player).is_some_and(|h| h.is_empty()) {
        if let Some(&ppos) = world.positions.get(&player) {
            world.render_events.push(RenderEvent::Death { pos: ppos, kind: VisualKind::Player });
        }
        let spawn = world.map.player_spawn;
        world.positions.insert(player, Position2D::from_tile(spawn.0, spawn.1));
        if let Some(h) = world.health.get_mut(&player) {
            h.current = h.max;
        }
        if let Some(m) = world.mana.get_mut(&player) {
            m.current = m.max;
        }
        world.move_targets.remove(&player);
        if let Some(c) = world.combat.get_mut(&player) {
            c.target = None;
        }
        world.log_event("You have fallen... and awaken at the entrance.");
    }
}

#[cfg(test)]
mod tests {
    use super::scale_xp_for_level_diff;

    #[test]
    fn same_level_keeps_base_xp() {
        assert_eq!(scale_xp_for_level_diff(100, 10, 10), 100);
        // 5-level grace band still grants full XP.
        assert_eq!(scale_xp_for_level_diff(100, 15, 10), 100);
    }

    #[test]
    fn higher_player_loses_ten_percent_per_extra_level() {
        // diff = 6 → −10% → 90
        assert_eq!(scale_xp_for_level_diff(100, 16, 10), 90);
        // diff = 7 → −20% → 80
        assert_eq!(scale_xp_for_level_diff(100, 17, 10), 80);
    }

    #[test]
    fn way_higher_player_floored_at_ten_percent() {
        // diff = 20 → would be -150% → clamped to 10% → 10
        assert_eq!(scale_xp_for_level_diff(100, 30, 10), 10);
    }

    #[test]
    fn lower_player_gets_bonus_capped_at_fifty_percent() {
        // diff = -5 → +25% → 125
        assert_eq!(scale_xp_for_level_diff(100, 10, 15), 125);
        // diff = -20 → would be +100% → capped at +50% → 150
        assert_eq!(scale_xp_for_level_diff(100, 10, 30), 150);
    }

    #[test]
    fn never_returns_zero_for_nonzero_base() {
        // Even at 10% of 1 XP, we still credit 1.
        assert_eq!(scale_xp_for_level_diff(1, 100, 1), 1);
    }
}
