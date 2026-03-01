//! Structs serde pour les definitions de donnees de jeu.
//!
//! <!-- @id: sd-data-types @do: define @role: arpg @layer: 3 @human: miyuk -->
//!
//! Chaque sous-module correspond a un domaine de donnees :
//! - [`common`] : types partages (StatValue, enums)
//! - [`classes`] : definitions de classes de personnages
//! - [`skills`] : definitions de competences
//! - [`items`] : items de base, affixes, uniques, sets, runewords, runes, gemmes
//! - [`monsters`] : monstres, super-uniques, champion affixes, bosses
//! - [`zones`] : zones de jeu
//! - [`quests`] : quetes et objectifs
//! - [`config`] : difficulte, breakpoints, experience, shrines, recettes cube

pub mod classes;
pub mod common;
pub mod config;
pub mod items;
pub mod monsters;
pub mod quests;
pub mod skills;
pub mod zones;

pub use classes::*;
pub use common::*;
pub use config::*;
pub use items::*;
pub use monsters::*;
pub use quests::*;
pub use skills::*;
pub use zones::*;
