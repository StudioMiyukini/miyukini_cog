//! @id mge.rpg.progression.v1.events
//! @role event
//! @layer plugin
//! @domain rpg

use mge_ecs::EntityId;
use mge_event::Event;

/// @id mge.rpg.progression.v1.event.skillcheck
/// @fields entity:EntityId,skill_id:u32,difficulty:f64,success:bool
/// Résultat d'un test de compétence (combat, harvest, craft, etc.).
/// Consommé par skill_gain_system pour calculer un gain potentiel.
#[derive(Debug, Clone)]
pub struct SkillCheckEvent {
    pub entity: EntityId,
    pub skill_id: u32,
    pub difficulty: f64,
    pub success: bool,
}

impl Event for SkillCheckEvent {}

/// @id mge.rpg.progression.v1.event.skillgain
/// @fields entity:EntityId,skill_id:u32,old_value:f64,new_value:f64
/// Émis quand une compétence gagne effectivement en niveau.
#[derive(Debug, Clone)]
pub struct SkillGainEvent {
    pub entity: EntityId,
    pub skill_id: u32,
    pub old_value: f64,
    pub new_value: f64,
}

impl Event for SkillGainEvent {}
