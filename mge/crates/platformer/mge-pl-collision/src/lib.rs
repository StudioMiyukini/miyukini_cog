//! @id mge.pl.collision.v1
//! @role plugin
//! @layer plugin
//! @domain platformer
//! @do scaffold_collision_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
