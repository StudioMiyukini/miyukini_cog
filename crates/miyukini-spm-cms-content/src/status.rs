//! Statuts de contenu (brouillon, publié, archivé).

use std::fmt;

/// Statut fonctionnel d'un contenu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentStatus {
    /// Brouillon (non publié).
    Draft,
    /// Publié (visible).
    Published,
    /// Archivé (supprimé de manière douce).
    Archived,
}

impl ContentStatus {
    /// Retourne tous les statuts possibles.
    pub fn all() -> &'static [ContentStatus] {
        &[ContentStatus::Draft, ContentStatus::Published, ContentStatus::Archived]
    }

    /// Vérifie si le statut est valide pour une transition depuis un autre statut.
    /// Les transitions sont définies par le produit, mais cette méthode permet
    /// de valider qu'un statut est dans la liste des statuts autorisés.
    pub fn is_valid(&self) -> bool {
        matches!(
            self,
            ContentStatus::Draft | ContentStatus::Published | ContentStatus::Archived
        )
    }
}

impl fmt::Display for ContentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentStatus::Draft => write!(f, "draft"),
            ContentStatus::Published => write!(f, "published"),
            ContentStatus::Archived => write!(f, "archived"),
        }
    }
}

impl Default for ContentStatus {
    fn default() -> Self {
        ContentStatus::Draft
    }
}
