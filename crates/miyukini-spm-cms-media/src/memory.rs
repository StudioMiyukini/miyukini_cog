//! Implémentation en mémoire du MediaManager (pour tests et démo uniquement).
//!
//! Cette implémentation n'est pas destinée à la production. Elle sert uniquement
//! à valider le contrat et à fournir une base pour les tests et démos.
//!
//! Le produit fournira sa propre implémentation adaptée à sa stack technique.

use crate::error::MediaError;
use crate::media::{EntityId, Media, MediaId, MediaInput, MediaManager};
use miyukini_kernel::{Clock, DefaultClock, IdGenerator, UuidIdGenerator};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

/// Implémentation en mémoire du MediaManager.
///
/// Stocke les médias dans une HashMap. Utilise UuidIdGenerator et DefaultClock du kernel.
/// Gère les liens média ↔ entité dans une structure séparée.
/// Pas de persistance : données perdues à l'arrêt.
pub struct MemoryMediaManager {
    media: Mutex<HashMap<MediaId, Media>>,
    // Mapping: entity_id -> set of media_ids
    entity_to_media: Mutex<HashMap<EntityId, HashSet<MediaId>>>,
    // Mapping: media_id -> set of entity_ids
    media_to_entities: Mutex<HashMap<MediaId, HashSet<EntityId>>>,
    id_generator: UuidIdGenerator,
    clock: DefaultClock,
}

impl MemoryMediaManager {
    /// Crée un nouveau gestionnaire en mémoire.
    pub fn new() -> Self {
        Self {
            media: Mutex::new(HashMap::new()),
            entity_to_media: Mutex::new(HashMap::new()),
            media_to_entities: Mutex::new(HashMap::new()),
            id_generator: UuidIdGenerator::new(),
            clock: DefaultClock::new(),
        }
    }
}

impl Default for MemoryMediaManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaManager for MemoryMediaManager {
    fn create_media(&self, input: MediaInput) -> Result<MediaId, MediaError> {
        let id = self.id_generator.generate();
        let now = self.clock.now();

        let media = Media::new(id, input.mime_type, input.metadata, now, now);

        let mut media_map = self.media.lock().unwrap();
        media_map.insert(id, media);

        Ok(id)
    }

    fn get_media(&self, id: MediaId) -> Result<Media, MediaError> {
        let media_map = self.media.lock().unwrap();
        media_map.get(&id).cloned().ok_or(MediaError::NotFound)
    }

    fn delete_media(&self, id: MediaId) -> Result<(), MediaError> {
        let mut media_map = self.media.lock().unwrap();
        
        // Vérifier que le média existe
        if !media_map.contains_key(&id) {
            return Err(MediaError::NotFound);
        }

        // Supprimer le média
        media_map.remove(&id);

        // Supprimer tous les liens avec les entités
        let mut entity_to_media = self.entity_to_media.lock().unwrap();
        let mut media_to_entities = self.media_to_entities.lock().unwrap();

        if let Some(entity_ids) = media_to_entities.remove(&id) {
            // Retirer ce média de toutes les entités
            for entity_id in entity_ids {
                if let Some(media_ids) = entity_to_media.get_mut(&entity_id) {
                    media_ids.remove(&id);
                    // Nettoyer les entrées vides
                    if media_ids.is_empty() {
                        entity_to_media.remove(&entity_id);
                    }
                }
            }
        }

        Ok(())
    }

    fn attach_media_to_entity(
        &self,
        media_id: MediaId,
        entity_id: EntityId,
    ) -> Result<(), MediaError> {
        // Vérifier que le média existe
        let media_map = self.media.lock().unwrap();
        if !media_map.contains_key(&media_id) {
            return Err(MediaError::NotFound);
        }
        drop(media_map);

        // Ajouter le lien dans les deux sens
        let mut entity_to_media = self.entity_to_media.lock().unwrap();
        let mut media_to_entities = self.media_to_entities.lock().unwrap();

        entity_to_media
            .entry(entity_id)
            .or_insert_with(HashSet::new)
            .insert(media_id);

        media_to_entities
            .entry(media_id)
            .or_insert_with(HashSet::new)
            .insert(entity_id);

        Ok(())
    }

    fn detach_media_from_entity(
        &self,
        media_id: MediaId,
        entity_id: EntityId,
    ) -> Result<(), MediaError> {
        let mut entity_to_media = self.entity_to_media.lock().unwrap();
        let mut media_to_entities = self.media_to_entities.lock().unwrap();

        // Retirer le lien dans les deux sens
        if let Some(media_ids) = entity_to_media.get_mut(&entity_id) {
            media_ids.remove(&media_id);
            // Nettoyer les entrées vides
            if media_ids.is_empty() {
                entity_to_media.remove(&entity_id);
            }
        } else {
            return Err(MediaError::EntityNotFound);
        }

        if let Some(entity_ids) = media_to_entities.get_mut(&media_id) {
            entity_ids.remove(&entity_id);
            // Nettoyer les entrées vides
            if entity_ids.is_empty() {
                media_to_entities.remove(&media_id);
            }
        }

        Ok(())
    }

    fn list_media_for_entity(&self, entity_id: EntityId) -> Result<Vec<Media>, MediaError> {
        let entity_to_media = self.entity_to_media.lock().unwrap();
        let media_map = self.media.lock().unwrap();

        if let Some(media_ids) = entity_to_media.get(&entity_id) {
            // Pré-allouer avec la capacité exacte pour éviter les réallocations
            let mut result = Vec::with_capacity(media_ids.len());
            for media_id in media_ids {
                if let Some(media) = media_map.get(media_id) {
                    result.push(media.clone());
                }
            }
            Ok(result)
        } else {
            // Entité sans médias : retourner une liste vide
            Ok(Vec::new())
        }
    }
}
