//! @id mge.vn.save.v1
//! @role plugin
//! @layer plugin
//! @domain visual_novel
//! @do scaffold_save_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
