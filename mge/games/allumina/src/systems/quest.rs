//! Quest tracking. Kill/pickup hooks live in the death/pickup paths; this
//! system polls gold and zone each tick, then grants rewards for any quest that
//! has just completed.

use crate::ecs::World;

pub fn quest_system(world: &mut World) {
    let Some(player) = world.player_entity else { return };

    let gold = world.inventory.get(&player).map_or(0, |i| i.gold);
    world.quests.set_gold(gold);
    world.quests.set_zone(world.zone);

    let completed = world.quests.take_completed();
    for (title, reward_gold, reward_xp) in completed {
        if reward_gold > 0 {
            if let Some(inv) = world.inventory.get_mut(&player) {
                inv.add_gold(reward_gold);
            }
        }
        if reward_xp > 0 {
            if let Some(p) = world.progression.get_mut(&player) {
                p.inner.grant_xp(reward_xp);
            }
        }
        world.log_event(format!(
            "Quest complete: {title} (+{reward_gold} gold, +{reward_xp} XP)"
        ));
    }
}
