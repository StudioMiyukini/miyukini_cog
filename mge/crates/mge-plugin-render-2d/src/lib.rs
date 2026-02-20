//! @id mge.plugin.render-2d.v1
//! @role plugin
//! @layer plugin
//! @domain render
//! @do provide_2d_rendering_components

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
