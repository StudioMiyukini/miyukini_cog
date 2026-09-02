//! @id     games_allumina_plugins_content
//! @do     definir_content_plugin
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « plugins content ».

//! Act I content: merchant spawning, zone transitions, the boss slam, and
//! quest tracking.

use crate::app::{App, PhaseId, Plugin};
use crate::systems::boss::boss_system;
use crate::systems::quest::quest_system;
use crate::systems::vendor::spawn_merchant_system;
use crate::systems::zone::zone_transition_system;

pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn build(&self, app: &mut App) {
        // Spawn the merchant for the starting zone.
        app.add_startup_system(spawn_merchant_system);
        // Boss special attack runs in the combat phase.
        app.add_system(PhaseId::Combat, boss_system);
        // Zone transitions + quest rewards run late, after movement/combat/loot.
        app.add_system(PhaseId::Progression, zone_transition_system);
        app.add_system(PhaseId::Progression, quest_system);
    }
}
