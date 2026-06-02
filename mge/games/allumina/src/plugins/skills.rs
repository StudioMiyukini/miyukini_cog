//! Wires active-skill execution into the schedule.

use crate::app::{App, PhaseId, Plugin};
use crate::systems::skill::{skill_system, trap_system};

pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(PhaseId::Skills, skill_system);
        // Detonate traps after movement/skills resolve, alongside combat.
        app.add_system(PhaseId::Combat, trap_system);
    }
}
