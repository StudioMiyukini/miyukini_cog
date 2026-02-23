//! @id mge.plugin.spatial.v1.components
//! @role data
//! @layer plugin
//! @domain spatial

use mge_ecs::Component;

/// @id mge.plugin.spatial.v1.component.position2d
/// @fields x:f32,y:f32
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
}

/// @id mge.plugin.spatial.v1.component.velocity2d
/// @fields x:f32,y:f32
#[derive(Debug, Clone)]
pub struct Velocity2D {
    pub x: f32,
    pub y: f32,
}

/// @id mge.plugin.spatial.v1.component.rotation
/// @fields angle:f32
#[derive(Debug, Clone)]
pub struct Rotation {
    pub angle: f32,
}

/// @id mge.plugin.spatial.v1.component.spatial_hash
/// @fields cell_id:u64
#[derive(Debug, Clone)]
pub struct SpatialHash {
    pub cell_id: u64,
}

impl Component for Position2D {}
impl Component for Velocity2D {}
impl Component for Rotation {}
impl Component for SpatialHash {}
