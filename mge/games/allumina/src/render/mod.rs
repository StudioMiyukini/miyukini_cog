//! @id     games_allumina_render_mod
//! @do     declarer_module_animation
//! @role   ui
//! @layer  core
//! @human  games allumina — module « render mod ».

//! The Allumina renderer: an isometric 2D presenter on top of the engine's
//! `mge-render` wgpu sprite pipeline (instanced, tinted unit quads) driven by a
//! `winit` event loop. There are no art assets — everything (tiles, entities,
//! VFX, HUD, full UI) is drawn as coloured quads and a 5x7 bitmap font, which
//! is exactly the "placeholder coloured squares" target for the MVP graphics.

pub mod animation;
pub mod camera;
pub mod feedback;
pub mod figures;
pub mod font;
pub mod item_icon;
pub mod particles;
pub mod pixel_sprite;
pub mod renderer;
pub mod shapes;
pub mod sprites_data;
pub mod skill_fx;
pub mod skill_icon;
pub mod sprite;
pub mod tilemap_renderer;
pub mod vertex;

use mge_render::atlas::{AtlasHandle, MaterialHandle, SpriteRect};

/// Single placeholder atlas handle (the pipeline binds a 1x1 white texture).
pub const ATLAS: AtlasHandle = AtlasHandle::new(0);
/// Single placeholder material handle.
pub const MAT: MaterialHandle = MaterialHandle::new(0);
/// Unit source rect — the whole 1x1 sentinel texture.
pub const UNIT: SpriteRect = SpriteRect { x: 0, y: 0, w: 1, h: 1 };

pub use renderer::run_game;
