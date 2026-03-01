//! mge-asset -- MGE asset: TOML data loading, hot-reload, asset registry.
//!
//! <!-- @id: MGE-Asset-Lib @do: re-exports @role: back-end @layer: 1 @human: miyuk -->
//!
//! Crate de chargement des donnees TOML pour Sodomight. Fournit :
//! - [`DataLoadError`] -- types d'erreur explicites pour IO, parsing, validation
//! - [`loader`] -- chargement generique de fichiers TOML
//! - [`registry`] -- registre central de toutes les definitions de jeu
//! - [`types`] -- structs serde pour items, monstres, skills, zones, quetes, config
//! - [`hot_reload`] -- watcher de fichiers (feature `dev-hotreload`)
#![deny(unsafe_code)]

pub mod loader;
pub mod registry;
pub mod types;
pub mod validation;

#[cfg(feature = "dev-hotreload")]
pub mod hot_reload;

// =========================================================================
// Re-exports
// =========================================================================

pub use loader::{load_toml_directory, load_toml_file};
pub use registry::GameDataRegistry;
pub use validation::validate_registry;

// =========================================================================
// Errors -- @id: sd-data-errors @do: define @role: arpg @layer: 3
// =========================================================================

/// Erreur explicite pour le chargement de donnees TOML.
#[derive(Debug, thiserror::Error)]
pub enum DataLoadError {
    /// Erreur d'entree/sortie lors de la lecture d'un fichier.
    #[error("IO error reading '{path}': {message}")]
    IoError {
        /// Chemin du fichier en erreur.
        path: String,
        /// Message d'erreur IO.
        message: String,
    },

    /// Erreur de parsing TOML.
    #[error("TOML parse error in '{path}': {message}")]
    ParseError {
        /// Chemin du fichier en erreur.
        path: String,
        /// Message d'erreur de parsing.
        message: String,
    },

    /// Repertoire introuvable.
    #[error("Directory not found: '{path}'")]
    DirectoryNotFound {
        /// Chemin du repertoire manquant.
        path: String,
    },

    /// ID duplique dans les donnees.
    #[error("Duplicate ID '{id}' found in '{path}'")]
    DuplicateId {
        /// L'ID duplique.
        id: String,
        /// Chemin du fichier contenant le doublon.
        path: String,
    },

    /// Erreur du watcher de fichiers.
    #[error("Watcher error: {message}")]
    WatcherError {
        /// Message d'erreur du watcher.
        message: String,
    },

    /// Erreurs de validation croisee.
    #[error("Validation errors:\n{}", errors.join("\n"))]
    ValidationErrors {
        /// Liste des erreurs de validation.
        errors: Vec<String>,
    },

    /// Valeur hors limites.
    #[error("Value out of range: {field} = {value} (expected {min}..{max})")]
    ValueOutOfRange {
        /// Nom du champ.
        field: String,
        /// Valeur trouvee.
        value: String,
        /// Borne minimale.
        min: String,
        /// Borne maximale.
        max: String,
    },
}

/// Alias de Result pour les operations de chargement.
pub type AssetResult<T> = Result<T, DataLoadError>;

#[cfg(test)]
mod tests;
