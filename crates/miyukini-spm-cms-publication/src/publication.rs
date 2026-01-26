//! Types et trait PublicationManager.

use crate::error::PublicationError;
use miyukini_kernel::Id;
use std::time::SystemTime;

/// Identifiant de publication (alias vers Id du kernel).
pub type PublicationId = Id;

/// Identifiant de contenu (alias vers Id du kernel).
pub type ContentId = Id;

/// Statut d'une publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationStatus {
    /// Brouillon (état initial).
    Draft,
    /// Programmée pour publication future.
    Scheduled,
    /// Publiée.
    Published,
    /// Archivée (état final).
    Archived,
}

/// Publication (entité principale).
#[derive(Debug, Clone)]
pub struct Publication {
    /// Identifiant unique (fourni par le kernel).
    pub id: Id,
    /// Identifiant du contenu associé.
    pub content_id: ContentId,
    /// Statut actuel.
    pub status: PublicationStatus,
    /// Date de publication programmée (si Scheduled).
    pub publish_at: Option<SystemTime>,
    /// Date de création (fournie par le kernel).
    pub created_at: SystemTime,
    /// Date de modification (fournie par le kernel).
    pub updated_at: SystemTime,
}

impl Publication {
    /// Crée une nouvelle publication (utilisé en interne par les implémentations).
    pub fn new(
        id: Id,
        content_id: ContentId,
        status: PublicationStatus,
        publish_at: Option<SystemTime>,
        created_at: SystemTime,
        updated_at: SystemTime,
    ) -> Self {
        Self {
            id,
            content_id,
            status,
            publish_at,
            created_at,
            updated_at,
        }
    }
}

/// Trait principal : gestion des publications.
///
/// Le produit implémente ce trait pour adapter le contrat fonctionnel
/// vers sa stack technique (DB, sérialisation, etc.).
pub trait PublicationManager {
    /// Crée une publication pour un contenu.
    ///
    /// L'identifiant est généré par le kernel (IdGenerator).
    /// Les dates sont fournies par le kernel (Clock).
    /// La publication commence toujours en état Draft.
    fn create_publication(&self, content_id: ContentId) -> Result<PublicationId, PublicationError>;

    /// Lit le statut actuel d'une publication.
    fn status(&self, publication_id: PublicationId) -> Result<PublicationStatus, PublicationError>;

    /// Programme une publication pour une date future.
    ///
    /// Transition : Draft → Scheduled
    /// La date doit être dans le futur, sinon InvalidSchedule.
    fn schedule(
        &self,
        publication_id: PublicationId,
        publish_at: SystemTime,
    ) -> Result<(), PublicationError>;

    /// Publie immédiatement une publication.
    ///
    /// Transitions autorisées :
    /// - Draft → Published
    /// - Scheduled → Published (si maintenant >= publish_at)
    fn publish_now(&self, publication_id: PublicationId) -> Result<(), PublicationError>;

    /// Archive une publication.
    ///
    /// Transition : Published → Archived
    /// Une publication archivée ne peut plus changer d'état.
    fn archive(&self, publication_id: PublicationId) -> Result<(), PublicationError>;

    /// Retourne le statut effectif d'une publication.
    ///
    /// Pour une publication Scheduled :
    /// - Si publish_at > now : retourne Scheduled
    /// - Si publish_at <= now : retourne Published
    /// Pour les autres statuts : retourne le statut actuel.
    fn effective_status(
        &self,
        publication_id: PublicationId,
    ) -> Result<PublicationStatus, PublicationError>;
}
