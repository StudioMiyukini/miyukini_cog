//! @id mge.plugin.basic-physics.v1.events
//! @domain physics

use mge_ecs::EntityId;
use mge_event::Event;

/// @id mge.plugin.basic-physics.v1.event.collision
/// @fields entity_a:EntityId,entity_b:EntityId
#[derive(Debug, Clone)]
pub struct CollisionEvent {
    pub entity_a: EntityId,
    pub entity_b: EntityId,
}

impl Event for CollisionEvent {}
