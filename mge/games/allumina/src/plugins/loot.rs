//! Wires loot generation into the schedule.

use crate::app::{App, PhaseId, Plugin};
use crate::systems::loot::loot_system;

pub struct LootPlugin;

impl Plugin for LootPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(PhaseId::Loot, loot_system);
    }
}
