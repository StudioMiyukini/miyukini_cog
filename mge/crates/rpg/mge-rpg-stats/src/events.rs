//! @id mge.rpg.stats.v1.events
//! @role event
//! @layer plugin
//! @domain rpg

use mge_ecs::EntityId;
use mge_event::Event;

use crate::components::StatId;

/// @id mge.rpg.stats.v1.event.statchanged
/// @fields entity:EntityId,stat_id:StatId,old_value:f64,new_value:f64
/// Emis quand une stat de base change.
#[derive(Debug, Clone)]
pub struct StatChangedEvent {
    pub entity: EntityId,
    pub stat_id: StatId,
    pub old_value: f64,
    pub new_value: f64,
}

impl Event for StatChangedEvent {}

/// @id mge.rpg.stats.v1.event.entitydeath
/// @fields entity:EntityId
/// Emis quand une entite meurt (Health.current <= 0).
#[derive(Debug, Clone)]
pub struct EntityDeathEvent {
    pub entity: EntityId,
}

impl Event for EntityDeathEvent {}
