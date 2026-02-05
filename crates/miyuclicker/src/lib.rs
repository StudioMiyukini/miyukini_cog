//! Lord of the Click — Premier jeu officiel Miyukini.
//!
//! Idle/Clicker (gestion) + Carte stratégique (conquête).
//!
//! @id: miyuclicker_lib_module
//! @role: application
//! @layer: application
//! @do: aggregate_miyuclicker_operators_and_toolkits
//! @human: Service jeu : UI, IdleSim, Save, Combat, Carte.

pub mod app;
pub mod carte;
pub mod combat;
pub mod idlesim;
pub mod save;
pub mod state;
pub mod ui_assets;

pub use app::{MiyuClickerApp, Screen};
pub use state::{Allocation, Cite, ClickTarget, GameState, Proprietaire};
