//! Versioning des contenus.

use miyukini_kernel::Id;
use std::time::SystemTime;

/// Version d'un contenu (snapshot à un instant donné).
#[derive(Debug, Clone)]
pub struct ContentVersion {
    /// Identifiant de la version.
    pub version_id: Id,
    /// Identifiant du contenu versionné.
    pub content_id: Id,
    /// Snapshot du contenu à cette version (structure définie par le produit).
    /// Type opaque : le produit définit la structure.
    pub snapshot: Vec<u8>, // Représentation sérialisée, format défini par le produit
    /// Date de création de la version.
    pub created_at: SystemTime,
}

impl ContentVersion {
    /// Crée une nouvelle version.
    pub fn new(version_id: Id, content_id: Id, snapshot: Vec<u8>, created_at: SystemTime) -> Self {
        Self {
            version_id,
            content_id,
            snapshot,
            created_at,
        }
    }
}
