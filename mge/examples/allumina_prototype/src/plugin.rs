//! @id allumina.prototype.plugin
//! @role plugin
//! @layer application
//! @domain allumina
//! @do register_systems_and_components
//!
//! AlluminaPlugin — enregistre composants et systèmes.
//! Ordre des phases : AI(40) → Input(50) → Pathfinding(60) → Combat(80) → PlayerAttack(85) → Movement(100) → Collision(105) → Animation(110)

use mge_core::{Engine, PhaseId};
use mge_plugin_spatial::{Position2D, Velocity2D};

use crate::ai::{MonsterAI, ai_system};
use crate::animation::{AnimSprite, animation_system};
use crate::collision::collision_system;
use crate::combat::{combat_apply_system, player_attack_system};
use crate::components::{AlluminaInput, AlluminaMap, EntitySprite, MoveTarget, PlayerMarker};
use crate::input_handler::input_processing_system;
use crate::movement::movement_system;
use crate::pathfinding::{PathFollow, PathRequest, pathfinding_system};
use crate::stats::{CharacterStats, CombatStats, Dead, PendingDamage, PlayerAttack, PlayerGameStats, XpReward};

pub const PHASE_AI: PhaseId = PhaseId(40);
pub const PHASE_INPUT: PhaseId = PhaseId(50);
pub const PHASE_PATHFINDING: PhaseId = PhaseId(60);
pub const PHASE_COMBAT: PhaseId = PhaseId(80);
pub const PHASE_PLAYER_ATTACK: PhaseId = PhaseId(85);
pub const PHASE_MOVEMENT: PhaseId = PhaseId(100);
pub const PHASE_COLLISION: PhaseId = PhaseId(105);
pub const PHASE_ANIMATION: PhaseId = PhaseId(110);

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
        engine.register_component::<CharacterStats>();
        engine.register_component::<CombatStats>();
        engine.register_component::<Dead>();
        engine.register_component::<PendingDamage>();
        engine.register_component::<PlayerAttack>();
        engine.register_component::<PlayerGameStats>();

        // IA
        engine.register_component::<MonsterAI>();

        // Récompenses
        engine.register_component::<XpReward>();

        // Pathfinding
        engine.register_component::<PathRequest>();
        engine.register_component::<PathFollow>();

        // Animation
        engine.register_component::<AnimSprite>();

        // ── Systèmes (ordre déterministe par PhaseId) ─────────────────────────
        engine.add_named_system(PHASE_AI, "ai", ai_system);
        engine.add_named_system(PHASE_INPUT, "input_processing", input_processing_system);
        engine.add_named_system(PHASE_PATHFINDING, "pathfinding", pathfinding_system);
        engine.add_named_system(PHASE_COMBAT, "combat_apply", combat_apply_system);
        engine.add_named_system(PHASE_PLAYER_ATTACK, "player_attack", player_attack_system);
        engine.add_named_system(PHASE_MOVEMENT, "movement", movement_system);
        engine.add_named_system(PHASE_COLLISION, "collision", collision_system);
        engine.add_named_system(PHASE_ANIMATION, "animation", animation_system);
    }

    fn dependencies(&self) -> &[&str] {
        &["mge-plugin-spatial"]
    }
}
