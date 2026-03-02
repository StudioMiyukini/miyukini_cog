// @id: MGE-AnimPack @do: tool @role: back-end @layer: 6 @human: francois
//! Animation atlas packer for MGE.
//!
//! Transforms Aseprite (.ase) files or individual PNG frames into
//! texture atlases with TOML descriptors. Supports automatic mirror
//! generation (4 rendered directions -> 8 directions).

pub mod errors;
pub mod naming;
pub mod aseprite;
pub mod mirror;
pub mod packer;
pub mod toml_output;
pub mod png_import;
pub mod aseprite_export;
