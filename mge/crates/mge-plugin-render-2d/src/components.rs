//! @id mge.plugin.render-2d.v1.components
//! @domain render

use mge_ecs::Component;

/// @id mge.plugin.render-2d.v1.component.sprite
/// @fields texture_id:u32,width:f32,height:f32
#[derive(Debug, Clone)]
pub struct Sprite {
    pub texture_id: u32,
    pub width: f32,
    pub height: f32,
}

/// @id mge.plugin.render-2d.v1.component.camera2d
/// @fields zoom:f32,offset_x:f32,offset_y:f32
#[derive(Debug, Clone)]
pub struct Camera2D {
    pub zoom: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

/// @id mge.plugin.render-2d.v1.component.render_layer
/// @fields layer:u32
#[derive(Debug, Clone)]
pub struct RenderLayer {
    pub layer: u32,
}

impl Component for Sprite {}
impl Component for Camera2D {}
impl Component for RenderLayer {}
