//! Wires mercenary AI and lifecycle into the schedule.

use crate::app::{App, PhaseId, Plugin};
use crate::systems::mercenary::{mercenary_ai_system, mercenary_update_system};

pub struct MercenaryPlugin;

impl Plugin for MercenaryPlugin {
    fn build(&self, app: &mut App) {
        // Follow/engage/abilities run alongside the other AI.
        app.add_system(PhaseId::Ai, mercenary_ai_system);
        // XP sharing, levelling, and death/respawn run after combat & rewards.
        app.add_system(PhaseId::Progression, mercenary_update_system);
    }
}
