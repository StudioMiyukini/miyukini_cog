//! Types et trait MediaManager.

use crate::error::MediaError;
use miyukini_kernel::Id;
use std::time::SystemTime;

/// Identifiant de média (alias vers Id du kernel).
pub type MediaId = Id;

/// Identifiant d'entité externe (alias vers Id du kernel).
pub type EntityId = Id;

/// Média (entité principale).
#[derive(Debug, Clone)]
pub struct Media {
    /// Identifiant unique (fourni par le kernel).
    pub id: Id,
    /// Type MIME (ex. "image/jpeg", "video/mp4", "application/pdf").
    pub mime_type: String,
    /// Métadonnées (structure définie par le produit, format opaque).
    /// Représentation sérialisée, format défini par le produit.
    pub metadata: Vec<u8>,
    /// Date de création (fournie par le kernel).
    pub created_at: SystemTime,
    /// Date de modification (fournie par le kernel).
    pub updated_at: SystemTime,
}

impl Media {
    /// Crée un nouveau média (utilisé en interne par les implémentations).
    pub fn new(
        id: Id,
        mime_type: String,
        metadata: Vec<u8>,
        created_at: SystemTime,
        updated_at: SystemTime,
    ) -> Self {
        Self {
            id,
            mime_type,
            metadata,
            created_at,
            updated_at,
        }
    }
}

/// Données d'entrée pour créer un média.
#[derive(Debug, Clone)]
pub struct MediaInput {
    /// Type MIME (défini par le produit).
    pub mime_type: String,
    /// Métadonnées (structure définie par le produit, format opaque).
    pub metadata: Vec<u8>,
}

impl MediaInput {
    /// Crée une nouvelle entrée de média.
    pub fn new(mime_type: String, metadata: Vec<u8>) -> Self {
        Self {
            mime_type,
            metadata,
        }
    }
}

/// Trait principal : gestion des médias.
///
/// Le produit implémente ce trait pour adapter le contrat fonctionnel
/// vers sa stack technique (DB, sérialisation, etc.).
pub trait MediaManager {
    /// Crée un média.
    ///
    /// L'identifiant est généré par le kernel (IdGenerator).
    /// Les dates sont fournies par le kernel (Clock).
    fn create_media(&self, input: MediaInput) -> Result<MediaId, MediaError>;

    /// Lit un média par son identifiant.
    fn get_media(&self, id: MediaId) -> Result<Media, MediaError>;

    /// Supprime un média.
    ///
    /// Supprime également tous les liens avec les entités.
    fn delete_media(&self, id: MediaId) -> Result<(), MediaError>;

    /// Attache un média à une entité externe.
    ///
    /// Un média peut être attaché à plusieurs entités.
    /// L'entité n'a pas besoin d'exister dans ce module (référence externe).
    fn attach_media_to_entity(
        &self,
        media_id: MediaId,
        entity_id: EntityId,
    ) -> Result<(), MediaError>;

    /// Détache un média d'une entité externe.
    fn detach_media_from_entity(
        &self,
        media_id: MediaId,
        entity_id: EntityId,
    ) -> Result<(), MediaError>;

    /// Liste tous les médias attachés à une entité.
    fn list_media_for_entity(&self, entity_id: EntityId) -> Result<Vec<Media>, MediaError>;
}
