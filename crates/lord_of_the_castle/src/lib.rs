//! Lord of the Castle — Miyukini Survivor (Survivor + Tower Defense).
//!
//! Titre du service Miyukini Survivor. Le joueur protège le Château au centre,
//! construit des tours en phase Préparation, affronte des vagues d'ennemis en phase Bataille.
//!
//! Référence : docs/services/MiyukiniSurvivor/
//!
//! @id: lord_of_the_castle_lib
//! @do: declare_modules_and_reexports
//! @role: bootstrap
//! @layer: domain
//! @human: Point d'entrée bibliothèque du jeu Lord of the Castle (modules, réexports publics).

pub mod app;
mod app_ui;
pub mod castle;
pub mod character_creation;
pub mod constants;
pub mod enemies;
pub mod game_loop;
pub mod game_state;
pub mod loot;
pub mod player;
pub mod spritesheet;
pub mod towers;
pub mod troops;
pub mod warrior_skills;

/// Onglet de l'arbre de compétences (fenêtre Compétences).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkillsTab {
    #[default]
    Guerrier,
    Sorcier,
    Saint,
    Regent,
    Commandant,
}

pub use app::{LordOfTheCastleApp, Screen};
pub use game_state::{GamePhase, GameState};
