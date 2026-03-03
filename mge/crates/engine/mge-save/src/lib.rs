// @id: MGE-Save-Lib @do: persistence-root @role: back-end @layer: 3 @human: denis
//! # mge-save
//!
//! Couche de persistance SQLite pour Sodomight (MGE).
//!
//! Fournit un Data Access Layer (DAL) pour les comptes, personnages, items,
//! competences, waypoints, quest flags et sessions de jeu. Le stockage repose
//! sur SQLite via `rusqlite`, avec migrations versionnees et transactions
//! atomiques pour la coherence des sauvegardes.

use thiserror::Error;

/// Erreurs de la couche persistance.
#[derive(Debug, Error)]
pub enum PersistenceError {
    /// Erreur SQLite sous-jacente.
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    /// Erreur de serialisation/deserialisation JSON.
    #[error("Serialization error: {0}")]
    Json(#[from] serde_json::Error),
    /// Entite introuvable en base.
    #[error("Not found: {entity} with id {id}")]
    NotFound {
        /// Type d'entite recherchee.
        entity: &'static str,
        /// Identifiant recherche.
        id: String,
    },
    /// Violation de contrainte UNIQUE.
    #[error("Duplicate entry: {0}")]
    Duplicate(String),
    /// Erreur lors de l'application d'une migration.
    #[error("Migration error: {0}")]
    Migration(String),
    /// Erreur de hashing ou verification de mot de passe.
    #[error("Password hash error: {0}")]
    PasswordHash(String),
}

/// Alias de resultat pour la couche persistance.
pub type PersistResult<T> = Result<T, PersistenceError>;

pub use db::DbPool;
pub use accounts::{Account, AccountDal, CreateAccountParams, hash_password, verify_password};
pub use characters::{
    CharacterDal, CharacterRow, CharacterSave, CharacterSummary,
    validate_character, save_character, load_character,
};
pub use items::{
    ItemAffix, ItemDal, ItemData, ItemRow,
    InventorySave, save_inventory, load_inventory,
};
pub use skills::SkillDal;
pub use waypoints::WaypointDal;
pub use sessions::SessionDal;
pub use quest_flags::{QuestFlagDal, QuestState};
pub use transactions::{save_character_full, CharacterSnapshot};

mod db;
mod schema;
mod accounts;
mod characters;
mod items;
mod skills;
mod waypoints;
mod sessions;
mod quest_flags;
mod transactions;

#[cfg(test)]
mod tests;
