// @id: sd-impl-tile @do: implement @role: back-end @layer: 2 @human: miyuk

//! Tile position helpers and isometric tile renderer.

use crate::atlas::AtlasRegistry;
use crate::camera::Camera2D;
use crate::sprite::{RenderLayer, SpriteInstance, SpriteRenderer};

// ---------------------------------------------------------------------------
// TilePos
// ---------------------------------------------------------------------------

/// Integer tile coordinate on the isometric grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TilePos {
    /// Tile column.
    pub tx: i32,
    /// Tile row.
    pub ty: i32,
}

impl TilePos {
    /// Create a new tile position.
    pub fn new(tx: i32, ty: i32) -> Self {
        Self { tx, ty }
    }

    /// Convert this tile position to screen pixel coordinates using
    /// the given camera (which includes zoom).
    pub fn to_screen(&self, camera: &Camera2D) -> (f32, f32) {
        camera.world_to_screen(self.tx as f32, self.ty as f32)
    }
}

// ---------------------------------------------------------------------------
// TileRenderArgs
// ---------------------------------------------------------------------------

/// Arguments for rendering a single tile, bundled to satisfy the 7-arg limit.
pub struct TileRenderArgs<'a> {
    /// Tile position on the grid.
    pub pos: &'a TilePos,
    /// Atlas identifier.
    pub atlas_id: &'a str,
    /// Frame name inside the atlas.
    pub frame_name: &'a str,
    /// Render layer for the tile.
    pub layer: RenderLayer,
    /// Camera used for coordinate conversion.
    pub camera: &'a Camera2D,
    /// Atlas registry for frame lookup.
    pub registry: &'a AtlasRegistry,
}

// ---------------------------------------------------------------------------
// TileRenderer
// ---------------------------------------------------------------------------

/// Renders a grid of isometric tiles by pushing sprites into a
/// [`SpriteRenderer`].
#[derive(Debug, Clone, Default)]
pub struct TileRenderer {
    /// Number of tiles along the X axis.
    pub map_width: u32,
    /// Number of tiles along the Y axis.
    pub map_height: u32,
}

impl TileRenderer {
    /// Create a tile renderer for the given map dimensions.
    pub fn new(map_width: u32, map_height: u32) -> Self {
        Self {
            map_width,
            map_height,
        }
    }

    /// Push a single tile sprite into the sprite renderer.
    ///
    /// The tile is placed at `args.pos` using the UV coordinates looked up
    /// from the atlas. If the atlas or frame is not found the tile is
    /// silently skipped (a warning is logged).
    pub fn render_tile(&self, args: &TileRenderArgs<'_>, renderer: &mut SpriteRenderer) {
        let Some(atlas) = args.registry.get(args.atlas_id) else {
            tracing::warn!("TileRenderer: atlas '{}' not in registry", args.atlas_id);
            return;
        };

        let Some(uv) = atlas.get_uv(args.frame_name) else {
            tracing::warn!(
                "TileRenderer: frame '{}' not in atlas '{}'",
                args.frame_name,
                args.atlas_id
            );
            return;
        };

        let _ = args.camera; // zoom is applied during the camera world_to_screen step

        renderer.push(SpriteInstance {
            x: args.pos.tx as f32,
            y: args.pos.ty as f32,
            z: 0.0,
            uv,
            tint: [1.0, 1.0, 1.0, 1.0],
            layer: args.layer,
        });
    }
}
