//! @id mge.plugin.basic-physics.v1.components
//! @domain physics

use mge_ecs::Component;

/// @id mge.plugin.basic-physics.v1.component.collider
/// @fields width:f32,height:f32,is_trigger:bool
#[derive(Debug, Clone)]
pub struct Collider {
    pub width: f32,
    pub height: f32,
    pub is_trigger: bool,
}

/// @id mge.plugin.basic-physics.v1.component.rigid_body
/// @fields mass:f32,restitution:f32
#[derive(Debug, Clone)]
pub struct RigidBody {
    pub mass: f32,
    pub restitution: f32,
}

impl Component for Collider {}
impl Component for RigidBody {}
