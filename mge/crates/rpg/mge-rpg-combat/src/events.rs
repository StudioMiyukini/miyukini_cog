//! @id mge.rpg.combat.v1.events
//! @role event
//! @layer plugin
//! @domain rpg

use mge_ecs::EntityId;
use mge_event::Event;

/// @id mge.rpg.combat.v1.event.attackrequest
/// @fields attacker:EntityId,target:EntityId
/// Demande d'attaque d'une entité contre une cible.
#[derive(Debug, Clone)]
pub struct AttackRequestEvent {
    pub attacker: EntityId,
    pub target: EntityId,
}

impl Event for AttackRequestEvent {}

/// @id mge.rpg.combat.v1.event.damage
/// @fields target:EntityId,amount:f64,crit:bool
/// Dégâts infligés à une cible (après résolution toucher/parade).
#[derive(Debug, Clone)]
pub struct DamageEvent {
    pub target: EntityId,
    pub amount: f64,
    pub crit: bool,
}

impl Event for DamageEvent {}

/// @id mge.rpg.combat.v1.event.attackmiss
/// @fields attacker:EntityId,target:EntityId
/// Attaque ratée (Phase 1 toucher échoué).
#[derive(Debug, Clone)]
pub struct AttackMissEvent {
    pub attacker: EntityId,
    pub target: EntityId,
}

impl Event for AttackMissEvent {}
