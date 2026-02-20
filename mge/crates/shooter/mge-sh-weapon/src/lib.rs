//! @id mge.sh.weapon.v1
//! @role plugin
//! @layer plugin
//! @domain shooter
//! @do scaffold_weapon_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
