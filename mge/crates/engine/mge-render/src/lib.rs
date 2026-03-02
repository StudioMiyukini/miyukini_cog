// @id: sd-impl-render-lib @do: implement @role: back-end @layer: 2 @human: miyuk

//! `mge-render` -- MGE renderer: wgpu sprite batching, iso tilemap, dual-res.
//!
//! This crate provides the core 2D isometric rendering primitives for MGE:
//!
//! - **Camera** -- dimetric 2:1 world/screen conversion with zoom and smooth follow.
//! - **Atlas** -- TOML-described texture atlas parsing and frame lookup.
//! - **Sprite** -- per-layer sprite batching and depth sorting.
//! - **Tile** -- isometric tile grid rendering.
//! - **Renderer** -- top-level GPU renderer stub (full init requires `mge-platform`).
//! - **Errors** -- explicit error types via `thiserror`.

pub mod atlas;
pub mod camera;
pub mod errors;
pub mod pipeline;
pub mod renderer;
pub mod sprite;
pub mod tile;

#[cfg(feature = "instanced")]
pub mod instanced;

#[cfg(test)]
mod tests;

// -- Re-exports for convenience --

pub use atlas::{AtlasDescriptor, AtlasEntry, AtlasFrame, AtlasRegistry, TextureAtlas};
pub use camera::Camera2D;
pub use errors::{RenderError, RenderResult};
pub use renderer::{RenderConfig, Renderer};
pub use pipeline::{GpuTexture, SpriteBatcher, SpritePipeline, SpriteVertex};
pub use sprite::{RenderLayer, SpriteBatch, SpriteInstance, SpriteRenderer};
pub use tile::{TilePos, TileRenderArgs, TileRenderer};
