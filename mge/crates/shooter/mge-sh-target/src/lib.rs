//! @id mge.sh.target.v1
//! @role plugin
//! @layer plugin
//! @domain shooter
//! @do scaffold_target_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
