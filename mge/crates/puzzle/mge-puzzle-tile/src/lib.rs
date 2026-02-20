//! @id mge.puzzle.tile.v1
//! @role plugin
//! @layer plugin
//! @domain puzzle
//! @do scaffold_tile_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
