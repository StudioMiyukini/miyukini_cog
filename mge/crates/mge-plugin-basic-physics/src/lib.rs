//! @id mge.plugin.basic-physics.v1
//! @role plugin
//! @layer plugin
//! @domain physics
//! @do provide_basic_collision_components

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
