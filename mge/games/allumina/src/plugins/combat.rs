//! @id     games_allumina_plugins_combat
//! @do     definir_combat_plugin
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « plugins combat ».

//! Wires combat resolution and death handling into the schedule.

use crate::app::{App, PhaseId, Plugin};
use crate::systems::combat::combat_system;
use crate::systems::death::death_system;
use crate::systems::projectile::projectile_system;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(PhaseId::Combat, combat_system);
        // In-flight projectiles tick after fresh ones may have been spawned by
        // this frame's combat resolution.
        app.add_system(PhaseId::Combat, projectile_system);
        app.add_system(PhaseId::Death, death_system);
    }
}
