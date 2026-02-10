//! Types d'erreur Lord of the Castle (Miyukini Survivor).
//!
//! @id: lord_of_the_castle_errors
//! @do: declare_save_and_db_errors
//! @role: bootstrap
//! @layer: domain

/// Erreur de persistance (sauvegarde / chargement).
#[derive(Debug, Clone)]
pub enum SaveError {
    /// Base de données ou fichier introuvable ou inaccessible.
    Db(String),
    /// Sérialisation ou désérialisation (JSON).
    Serialization(String),
    /// Slot invalide (ex. hors 1..=3).
    InvalidSlot(u8),
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Db(s) => write!(f, "DB: {}", s),
            Self::Serialization(s) => write!(f, "Serialization: {}", s),
            Self::InvalidSlot(id) => write!(f, "Invalid slot: {}", id),
        }
    }
}

impl std::error::Error for SaveError {}
