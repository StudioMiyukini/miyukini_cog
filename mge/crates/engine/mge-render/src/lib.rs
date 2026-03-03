// @id: sd-impl-render-lib @do: implement @role: back-end @layer: 2 @human: miyuk

//! `mge-render` -- MGE renderer: wgpu sprite batching, iso tilemap, dual-res.
//!
//! This crate provides the core 2D isometric rendering primitives for MGE:
//!
//! - **Animation** -- FSM animation system with 7 states, 8 directions, TOML-driven clips.
//! - **Camera** -- dimetric 2:1 world/screen conversion with zoom and smooth follow.
//! - **Atlas** -- TOML-described texture atlas parsing and frame lookup.
//! - **Sprite** -- per-layer sprite batching and depth sorting.
//! - **Tile** -- isometric tile grid rendering.
//! - **Overhead** -- world-space overlay UI: floating text, emotes, progress bars.
//! - **Renderer** -- top-level GPU renderer stub (full init requires `mge-platform`).
//! - **Text** -- TTF/OTF text rendering with glyph caching (replaces `BitmapFont`).
//! - **Errors** -- explicit error types via `thiserror`.

pub mod animation;
pub mod atlas;
pub mod camera;
pub mod errors;
pub mod overhead;
pub mod pipeline;
pub mod renderer;
pub mod sprite;
pub mod tile;

#[cfg(feature = "instanced")]
pub mod culling;
#[cfg(feature = "instanced")]
pub mod instanced;
pub mod text;

#[cfg(test)]
mod tests;

// -- Re-exports for convenience --

pub use animation::{
    AnimationBank, AnimationClip, AnimationController, AnimationLod, AnimationState, Direction,
    FrameEvent, FrameEventKind, SpriteSize, compute_lod,
};
pub use atlas::{AtlasDescriptor, AtlasEntry, AtlasFrame, AtlasRegistry, TextureAtlas};
pub use camera::Camera2D;
pub use errors::{RenderError, RenderResult};
pub use pipeline::{GpuTexture, SpriteBatcher, SpritePipeline, SpriteVertex};
pub use renderer::{RenderConfig, Renderer, MAX_SPRITES_U16, MAX_SPRITES_U32};
#[allow(deprecated)]
pub use sprite::{RenderLayer, SpriteBatch, SpriteInstance, SpriteRenderer};
pub use tile::{TilePos, TileRenderArgs, TileRenderer};

// -- Overhead UI re-exports --

pub use overhead::{
    EmoteKind, EmoteManager, FloatingText, FloatingTextKind, FloatingTextManager,
    FloatingTextPreset, OverheadProgressBar, ProgressBarManager, SpriteEmote,
    CASTING_COLOR, CRAFTING_COLOR, GATHERING_COLOR, LOADING_COLOR,
    DAMAGE_COLOR_PHYSICAL, DAMAGE_COLOR_FIRE, DAMAGE_COLOR_COLD,
    DAMAGE_COLOR_LIGHTNING, DAMAGE_COLOR_POISON, DAMAGE_COLOR_MAGIC,
};

// -- Instanced pipeline re-exports (behind feature flag) --

#[cfg(feature = "instanced")]
pub use culling::{FrustumCuller, RenderEntity, RenderSpatialGrid};
#[cfg(feature = "instanced")]
pub use instanced::{
    InstanceData, InstancedSpriteBatcher, InstancedSpritePipeline, QuadVertex,
    TextureArray,
};

// -- Text rendering re-exports --

pub use text::{
    CachedGlyph, FontId, GlyphCache, GlyphKey, GlyphMetrics, TextRenderer, TtfFont,
};
