//! Implémentation en mémoire du PublicationManager (pour tests et démo uniquement).
//!
//! Cette implémentation n'est pas destinée à la production. Elle sert uniquement
//! à valider le contrat et à fournir une base pour les tests et démos.
//!
//! Le produit fournira sa propre implémentation adaptée à sa stack technique.

use crate::error::PublicationError;
use crate::publication::{
    ContentId, Publication, PublicationId, PublicationManager, PublicationStatus,
};
use miyukini_kernel::{Clock, DefaultClock, IdGenerator, UuidIdGenerator};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::SystemTime;

/// Implémentation en mémoire du PublicationManager.
///
/// Stocke les publications dans une HashMap. Utilise UuidIdGenerator et DefaultClock du kernel.
/// Pas de persistance : données perdues à l'arrêt.
pub struct MemoryPublicationManager {
    publications: Mutex<HashMap<PublicationId, Publication>>,
    id_generator: UuidIdGenerator,
    clock: DefaultClock,
}

impl MemoryPublicationManager {
    /// Crée un nouveau gestionnaire en mémoire.
    pub fn new() -> Self {
        Self {
            publications: Mutex::new(HashMap::new()),
            id_generator: UuidIdGenerator::new(),
            clock: DefaultClock::new(),
        }
    }
}

impl Default for MemoryPublicationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicationManager for MemoryPublicationManager {
    fn create_publication(&self, content_id: ContentId) -> Result<PublicationId, PublicationError> {
        let id = self.id_generator.generate();
        let now = self.clock.now();

        let publication = Publication::new(
            id,
            content_id,
            PublicationStatus::Draft,
            None,
            now,
            now,
        );

        let mut publications = self.publications.lock().unwrap();
        publications.insert(id, publication);

        Ok(id)
    }

    fn status(&self, publication_id: PublicationId) -> Result<PublicationStatus, PublicationError> {
        let publications = self.publications.lock().unwrap();
        publications
            .get(&publication_id)
            .map(|p| p.status)
            .ok_or(PublicationError::NotFound)
    }

    fn schedule(
        &self,
        publication_id: PublicationId,
        publish_at: SystemTime,
    ) -> Result<(), PublicationError> {
        let now = self.clock.now();

        // Vérifier que la date est dans le futur
        if publish_at <= now {
            return Err(PublicationError::InvalidSchedule);
        }

        let mut publications = self.publications.lock().unwrap();
        let publication = publications
            .get_mut(&publication_id)
            .ok_or(PublicationError::NotFound)?;

        // Vérifier que la transition est valide : Draft → Scheduled
        if publication.status != PublicationStatus::Draft {
            return Err(PublicationError::InvalidTransition);
        }

        // Effectuer la transition
        publication.status = PublicationStatus::Scheduled;
        publication.publish_at = Some(publish_at);
        publication.updated_at = now;

        Ok(())
    }

    fn publish_now(&self, publication_id: PublicationId) -> Result<(), PublicationError> {
        let now = self.clock.now();

        let mut publications = self.publications.lock().unwrap();
        let publication = publications
            .get_mut(&publication_id)
            .ok_or(PublicationError::NotFound)?;

        // Vérifier que la transition est valide
        match publication.status {
            PublicationStatus::Draft => {
                // Draft → Published : OK
                publication.status = PublicationStatus::Published;
                publication.publish_at = Some(now);
                publication.updated_at = now;
                Ok(())
            }
            PublicationStatus::Scheduled => {
                // Scheduled → Published : OK seulement si maintenant >= publish_at
                if let Some(publish_at) = publication.publish_at {
                    if now >= publish_at {
                        publication.status = PublicationStatus::Published;
                        publication.updated_at = now;
                        Ok(())
                    } else {
                        Err(PublicationError::InvalidTransition)
                    }
                } else {
                    // Cas théorique : Scheduled sans publish_at
                    Err(PublicationError::InvalidTransition)
                }
            }
            PublicationStatus::Published => {
                // Déjà publié : pas de changement
                publication.updated_at = now;
                Ok(())
            }
            PublicationStatus::Archived => {
                // Archivé : ne peut plus changer
                Err(PublicationError::InvalidTransition)
            }
        }
    }

    fn archive(&self, publication_id: PublicationId) -> Result<(), PublicationError> {
        let now = self.clock.now();

        let mut publications = self.publications.lock().unwrap();
        let publication = publications
            .get_mut(&publication_id)
            .ok_or(PublicationError::NotFound)?;

        // Vérifier que la transition est valide : Published → Archived
        if publication.status != PublicationStatus::Published {
            return Err(PublicationError::InvalidTransition);
        }

        // Effectuer la transition
        publication.status = PublicationStatus::Archived;
        publication.updated_at = now;

        Ok(())
    }

    fn effective_status(
        &self,
        publication_id: PublicationId,
    ) -> Result<PublicationStatus, PublicationError> {
        let now = self.clock.now();

        let publications = self.publications.lock().unwrap();
        let publication = publications
            .get(&publication_id)
            .ok_or(PublicationError::NotFound)?;

        // Pour une publication Scheduled, vérifier si elle doit être considérée comme Published
        if publication.status == PublicationStatus::Scheduled {
            if let Some(publish_at) = publication.publish_at {
                if now >= publish_at {
                    return Ok(PublicationStatus::Published);
                }
            }
            return Ok(PublicationStatus::Scheduled);
        }

        // Pour les autres statuts, retourner le statut actuel
        Ok(publication.status)
    }
}
