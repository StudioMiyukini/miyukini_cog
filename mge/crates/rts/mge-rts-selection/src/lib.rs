//! @id mge.rts.selection.v1
//! @role plugin
//! @layer plugin
//! @domain rts
//! @do scaffold_selection_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
