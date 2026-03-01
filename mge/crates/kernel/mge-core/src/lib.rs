//! mge-core -- MGE kernel: game loop, time, events, system scheduler.
//!
//! <!-- @id: MGE-Core-Lib @do: re-exports @role: back-end @layer: 1 @human: miyuk -->
//!
//! Crate racine du moteur MGE. Fournit :
//! - [`GameLoop`] — boucle de jeu a tick rate fixe avec accumulation
//! - [`GameTime`] / [`TickRate`] — gestion du temps, delta, tick count
//! - [`EventBus`] / [`Event`] — bus d'evenements synchrone type-erased
//! - [`SystemScheduler`] — ordonnanceur de systemes par priorite
#![deny(unsafe_code)]


pub mod event;
pub mod game_loop;
pub mod scheduler;
pub mod time;

pub use event::{Event, EventBus};
pub use game_loop::{GameLoop, LoopConfig};
pub use scheduler::{SystemId, SystemPriority, SystemScheduler};
pub use time::{GameTime, TickRate};

#[cfg(test)]
mod tests;
