//! @id     games_allumina_plugins_cc
//! @do     definir_cc_plugin
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « plugins cc ».

//! Wires the crowd-control ticking (knockdown, recul, interruption) into the
//! schedule.

use crate::app::{App, PhaseId, Plugin};
use crate::systems::cc::{interrupt_system, knockdown_system, pushback_system};

pub struct CcPlugin;

impl Plugin for CcPlugin {
    fn build(&self, app: &mut App) {
        // Advance reculs and knockdowns in the Movement phase, after the normal
        // path-following (which already skips controlled entities).
        app.add_system(PhaseId::Movement, pushback_system);
        app.add_system(PhaseId::Movement, knockdown_system);
        // Resolve interruptions after combat damage has been applied for the tick.
        app.add_system(PhaseId::Combat, interrupt_system);
    }
}
