//! @id mge.rpg.ai.v1.events
//! @role event
//! @layer plugin
//! @domain rpg

use mge_ecs::EntityId;
use mge_event::Event;

/// @id mge.rpg.ai.v1.event.aiattackrequest
/// @fields attacker:EntityId,target:EntityId
/// @role event
/// @layer plugin
/// @do demande d'attaque emise par l'IA (traitee par mge-rpg-combat)
#[derive(Debug, Clone)]
pub struct AiAttackRequestEvent {
    pub attacker: EntityId,
    pub target: EntityId,
}

impl Event for AiAttackRequestEvent {}
