//! Lord of the Castle — Miyukini Survivor (Survivor + Tower Defense).
//!
//! Titre du service Miyukini Survivor. Le joueur protège le Château au centre,
//! construit des tours en phase Préparation, affronte des vagues d'ennemis en phase Bataille.
//!
//! Référence : docs/services/MiyukiniSurvivor/

pub mod app;
pub mod castle;
pub mod character_creation;
pub mod constants;
pub mod enemies;
pub mod game_loop;
pub mod game_state;
pub mod loot;
pub mod player;
pub mod towers;

pub use app::{LordOfTheCastleApp, Screen};
pub use game_state::{GamePhase, GameState};
