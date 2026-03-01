// @id: MGE-Studio @do: studio-core @role: back-end @layer: 6 @human: miyuk
//! MGE Studio data types: atlas editor, animation clips, map editor, data authoring.
#![deny(unsafe_code)]

pub mod anim_editor;
pub mod atlas_editor;
pub mod error;
pub mod map_editor;
pub mod project;

pub use anim_editor::{AnimClip, AnimFrame, AnimLoop};
pub use atlas_editor::{AtlasEditorState, FrameEntry};
pub use error::StudioError;
pub use map_editor::{LayerKind, MapEditorState, PlacedTile};
pub use project::{ProjectMeta, StudioProject};
