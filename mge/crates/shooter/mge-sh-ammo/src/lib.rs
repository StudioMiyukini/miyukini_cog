//! @id mge.sh.ammo.v1
//! @role plugin
//! @layer plugin
//! @domain shooter
//! @do scaffold_ammo_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
