//! @id     games_allumina_plugins_summon
//! @do     definir_summon_plugin
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « plugins summon ».

//! Wires summon AI and lifecycle into the schedule.

use crate::app::{App, PhaseId, Plugin};
use crate::systems::summon::{summon_ai_system, summon_system};

pub struct SummonPlugin;

impl Plugin for SummonPlugin {
    fn build(&self, app: &mut App) {
        // Allied targeting/movement runs alongside mob AI.
        app.add_system(PhaseId::Ai, summon_ai_system);
        // Lifecycle runs late in the Combat phase so it observes this tick's
        // deaths (including the player's, before the death system respawns them).
        app.add_system(PhaseId::Combat, summon_system);
    }
}
