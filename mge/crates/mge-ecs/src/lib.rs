//! ECS — World, EntityId, Component, SoA storage.
//!
//! @id mge.kernel.ecs
//! @role simulation
//! @layer kernel
//! @human Stockage entités et composants SoA
//! @do store_entities_and_components

mod component;
mod entity;
mod storage;
mod world;

pub use component::Component;
pub use entity::EntityId;
pub use world::{Query2Mut, World};
