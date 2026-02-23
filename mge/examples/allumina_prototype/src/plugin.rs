//! @id allumina.prototype.plugin
//! @role plugin
//! @layer application
//! @domain allumina
//! @do register_systems_and_components
//!
//! AlluminaPlugin — enregistre composants et systèmes.
//! Ordre des phases : AI(40) → Input(50) → Pathfinding(60) → Combat(80) → Movement(100)

use mge_core::{Engine, PhaseId};
use mge_plugin_spatial::{Position2D, Velocity2D};

use crate::ai::{MonsterAI, ai_system};
use crate::combat::combat_apply_system;
use crate::components::{AlluminaInput, AlluminaMap, EntitySprite, MoveTarget, PlayerMarker};
use crate::input_handler::input_processing_system;
use crate::movement::movement_system;
use crate::pathfinding::{PathFollow, PathRequest, pathfinding_system};
use crate::stats::{CombatStats, Dead, PendingDamage};

pub const PHASE_AI: PhaseId = PhaseId(40);
pub const PHASE_INPUT: PhaseId = PhaseId(50);
pub const PHASE_PATHFINDING: PhaseId = PhaseId(60);
pub const PHASE_COMBAT: PhaseId = PhaseId(80);
pub const PHASE_MOVEMENT: PhaseId = PhaseId(100);

pub struct AlluminaPlugin;

impl mge_core::Plugin for AlluminaPlugin {
    fn name(&self) -> &str {
        "allumina"
    }

    fn build(&self, engine: &mut Engine) {
        // ── Composants ────────────────────────────────────────────────────────
        engine.register_component::<Position2D>();
        engine.register_component::<Velocity2D>();
        engine.register_component::<AlluminaMap>();
        engine.register_component::<AlluminaInput>();
        engine.register_component::<PlayerMarker>();
        engine.register_component::<EntitySprite>();
        engine.register_component::<MoveTarget>();

        // Stats & combat
        engine.register_component::<CombatStats>();
        engine.register_component::<Dead>();
        engine.register_component::<PendingDamage>();

        // IA
        engine.register_component::<MonsterAI>();

        // Pathfinding
        engine.register_component::<PathRequest>();
        engine.register_component::<PathFollow>();

        // ── Systèmes (ordre déterministe par PhaseId) ─────────────────────────
        engine.add_named_system(PHASE_AI, "ai", ai_system);
        engine.add_named_system(PHASE_INPUT, "input_processing", input_processing_system);
        engine.add_named_system(PHASE_PATHFINDING, "pathfinding", pathfinding_system);
        engine.add_named_system(PHASE_COMBAT, "combat_apply", combat_apply_system);
        engine.add_named_system(PHASE_MOVEMENT, "movement", movement_system);
    }

    fn dependencies(&self) -> &[&str] {
        &["mge-plugin-spatial"]
    }
}
