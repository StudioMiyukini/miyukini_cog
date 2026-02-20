//! @id mge.factory.recipe.v1
//! @role plugin
//! @layer plugin
//! @domain factory
//! @do scaffold_recipe_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
