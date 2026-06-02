//! Synchronise stats with progression on level up. Allumina hands the player
//! 10 unspent attribute points per level (see `STAT_POINTS_PER_LEVEL`) which
//! they distribute manually via the character sheet — no auto-allocation.
//! Skill points still flow straight into the tree.

use crate::ecs::World;
use crate::systems::stats::recompute_player_stats;

pub fn level_up_system(world: &mut World) {
    let Some(player) = world.player_entity else { return };
    let Some(prog_level) = world.progression.get(&player).map(|p| p.level()) else { return };
    let Some(stat_level) = world.stats.get(&player).map(|s| s.inner.level) else { return };
    if prog_level <= stat_level {
        return;
    }
    let gained = prog_level - stat_level;

    let skill_points = world.progression.get(&player).map_or(0, |p| p.inner.skill_points);

    // The stat sheet only tracks `level` (so the +2/level life roll stays in
    // sync) — attributes themselves stay put and wait for the player.
    if let Some(s) = world.stats.get_mut(&player) {
        s.inner.level = prog_level;
    }

    // Re-derive every resolved stat (pools, damage, defence) from the new
    // level + worn gear, then refresh Health/Mana max.
    recompute_player_stats(world, player);

    // Skill points still auto-route to the tree; stat points stay unspent
    // until the player clicks them.
    if let Some(prog) = world.progression.get_mut(&player) {
        prog.inner.skill_points = 0;
    }
    if let Some(tree) = world.skill_trees.get_mut(&player) {
        tree.add_points(skill_points);
    }

    // Full heal on level up (overrides the fraction preserved by recompute).
    if let Some((life, mana)) =
        world.stats.get(&player).map(|s| (s.inner.resolved.life_max, s.inner.resolved.mana_max))
    {
        if let Some(h) = world.health.get_mut(&player) {
            h.max = life;
            h.current = life;
        }
        if let Some(m) = world.mana.get_mut(&player) {
            m.max = mana;
            m.current = mana;
        }
    }

    world.log_event(format!("Level up!  You are now level {prog_level}  (+{gained})"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::player::ClassType;
    use crate::data::maps::village_map;
    use crate::data::GameData;
    use crate::plugins::player::spawn_player_system;

    fn world_with_player() -> World {
        let mut world = World::new(village_map(), GameData::load(), 1);
        world.chosen_class = ClassType::Soldat;
        spawn_player_system(&mut world);
        world
    }

    #[test]
    fn each_level_grants_one_skill_point() {
        let mut world = world_with_player();
        let p = world.player_entity.unwrap();
        let start_pts = world.skill_trees.get(&p).unwrap().available_points;

        // Grant enough XP for exactly three level-ups, then drain it into the
        // tree via level_up_system.
        let needed: u64 = {
            let prog = &world.progression.get(&p).unwrap().inner;
            (prog.level..prog.level + 3)
                .map(mge_gameplay::progression::xp_for_level)
                .sum()
        };
        world.progression.get_mut(&p).unwrap().inner.grant_xp(needed);
        level_up_system(&mut world);

        let after = world.skill_trees.get(&p).unwrap().available_points;
        assert_eq!(after - start_pts, 3, "3 level-ups -> +3 skill points");
        // The intermediate prog.skill_points pool must be drained.
        assert_eq!(world.progression.get(&p).unwrap().inner.skill_points, 0);
    }

    #[test]
    fn skill_point_cap_prevents_investing_past_max_level() {
        use crate::ui::skill_tree_ui::invest;
        use mge_gameplay::skills::class_skill_stubs;

        let mut world = world_with_player();
        let p = world.player_entity.unwrap();

        // Raise the character to a level high enough that prerequisites don't
        // gate the test, and stuff the tree with enough points to overshoot
        // the per-skill max (20).
        world.progression.get_mut(&p).unwrap().inner.level = 120;
        world.stats.get_mut(&p).unwrap().inner.level = 120;
        world.skill_trees.get_mut(&p).unwrap().add_points(50);
        let before = world.skill_trees.get(&p).unwrap().available_points;

        // First skill in the Soldat tree.
        let id = class_skill_stubs(world.skill_trees.get(&p).unwrap().class)[0].id;
        for _ in 0..25 {
            invest(&mut world, id);
        }
        let tree = world.skill_trees.get(&p).unwrap();
        assert_eq!(tree.level_of(id), 20, "must cap at max_level=20");
        // Exactly 20 points spent regardless of starting pool size; the cap
        // refuses the 5 extra clicks.
        assert_eq!(before - tree.available_points, 20);
    }
}
