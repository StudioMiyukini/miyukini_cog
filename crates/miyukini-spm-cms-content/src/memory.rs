//! Implémentation en mémoire du ContentManager (pour tests et démo uniquement).
//!
//! Cette implémentation n'est pas destinée à la production. Elle sert uniquement
//! à valider le contrat et à fournir une base pour les tests et démos.
//!
//! Le produit fournira sa propre implémentation adaptée à sa stack technique.

use crate::content::{
    Content, ContentFilters, ContentInput, ContentListResult, ContentManager, ContentUpdates,
};
use crate::error::ContentError;
use crate::relation::ContentRelation;
use crate::status::ContentStatus;
use crate::version::ContentVersion;
use miyukini_kernel::{Clock, DefaultClock, IdGenerator, UuidIdGenerator};
use std::collections::HashMap;
use std::sync::Mutex;

/// Implémentation en mémoire du ContentManager.
///
/// Stocke les contenus dans une HashMap. Utilise UuidIdGenerator et DefaultClock du kernel.
/// Pas de persistance : données perdues à l'arrêt.
pub struct MemoryContentManager {
    contents: Mutex<HashMap<miyukini_kernel::Id, Content>>,
    relations: Mutex<HashMap<miyukini_kernel::Id, Vec<ContentRelation>>>,
    versions: Mutex<HashMap<miyukini_kernel::Id, Vec<ContentVersion>>>,
    id_generator: UuidIdGenerator,
    clock: DefaultClock,
}

impl MemoryContentManager {
    /// Crée un nouveau gestionnaire en mémoire.
    pub fn new() -> Self {
        Self {
            contents: Mutex::new(HashMap::new()),
            relations: Mutex::new(HashMap::new()),
            versions: Mutex::new(HashMap::new()),
            id_generator: UuidIdGenerator::new(),
            clock: DefaultClock::new(),
        }
    }
}

impl Default for MemoryContentManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ContentManager for MemoryContentManager {
    fn create_content(&self, input: ContentInput) -> Result<miyukini_kernel::Id, ContentError> {
        let id = self.id_generator.generate();
        let now = self.clock.now();
        let status = input.status.unwrap_or(ContentStatus::Draft);

        let content = Content::new(
            id,
            input.content_type,
            status,
            now,
            now,
            input.metadata,
        );

        let mut contents = self.contents.lock().unwrap();
        contents.insert(id, content);

        Ok(id)
    }

    fn get_content(&self, id: miyukini_kernel::Id) -> Result<Content, ContentError> {
        let contents = self.contents.lock().unwrap();
        contents.get(&id).cloned().ok_or(ContentError::NotFound)
    }

    fn update_content(
        &self,
        id: miyukini_kernel::Id,
        updates: ContentUpdates,
        create_version: bool,
    ) -> Result<(), ContentError> {
        let mut contents = self.contents.lock().unwrap();
        let content = contents.get_mut(&id).ok_or(ContentError::NotFound)?;

        // Créer une version si demandé
        if create_version {
            let version_id = self.id_generator.generate();
            let snapshot = self.serialize_content(content)?;
            let version = ContentVersion::new(version_id, id, snapshot, self.clock.now());

            let mut versions = self.versions.lock().unwrap();
            versions.entry(id).or_insert_with(Vec::new).push(version);
        }

        // Appliquer les mises à jour
        if let Some(content_type) = updates.content_type {
            content.content_type = content_type;
        }
        if let Some(status) = updates.status {
            if !status.is_valid() {
                return Err(ContentError::InvalidStatus);
            }
            content.status = status;
        }
        if let Some(metadata) = updates.metadata {
            content.metadata = metadata;
        }

        // Mettre à jour la date de modification
        content.updated_at = self.clock.now();

        Ok(())
    }

    fn delete_content(
        &self,
        id: miyukini_kernel::Id,
        soft: bool,
    ) -> Result<(), ContentError> {
        let mut contents = self.contents.lock().unwrap();

        if soft {
            // Archivage : changer le statut
            if let Some(content) = contents.get_mut(&id) {
                content.status = ContentStatus::Archived;
                content.updated_at = self.clock.now();
                return Ok(());
            }
        } else {
            // Suppression définitive
            contents.remove(&id);
            let mut relations = self.relations.lock().unwrap();
            relations.remove(&id);
            let mut versions = self.versions.lock().unwrap();
            versions.remove(&id);
            return Ok(());
        }

        Err(ContentError::NotFound)
    }

    fn list_contents(
        &self,
        filters: ContentFilters,
        offset: usize,
        limit: usize,
    ) -> Result<ContentListResult, ContentError> {
        let contents = self.contents.lock().unwrap();

        // Filtrer et compter sans cloner d'abord
        let mut count = 0;
        let mut items = Vec::with_capacity(limit.min(contents.len()));
        
        for content in contents.values() {
            // Vérifier les filtres
            if let Some(ref content_type) = filters.content_type {
                if content.content_type != *content_type {
                    continue;
                }
            }
            if let Some(status) = filters.status {
                if content.status != status {
                    continue;
                }
            }
            if let Some(created_after) = filters.created_after {
                if content.created_at < created_after {
                    continue;
                }
            }
            if let Some(created_before) = filters.created_before {
                if content.created_at > created_before {
                    continue;
                }
            }
            
            // Compter tous les éléments filtrés
            count += 1;
            
            // Cloner uniquement les éléments dans la fenêtre de pagination
            if count > offset && items.len() < limit {
                items.push(content.clone());
            }
        }

        Ok(ContentListResult::new(items, count, offset, limit))
    }

    fn add_relation(
        &self,
        source_id: miyukini_kernel::Id,
        relation_type: String,
        target_id: miyukini_kernel::Id,
    ) -> Result<(), ContentError> {
        // Vérifier que les contenus existent
        let contents = self.contents.lock().unwrap();
        if !contents.contains_key(&source_id) || !contents.contains_key(&target_id) {
            return Err(ContentError::InvalidRelation);
        }
        drop(contents);

        let relation = ContentRelation::new(source_id, relation_type, target_id);
        let mut relations = self.relations.lock().unwrap();
        relations.entry(source_id).or_insert_with(Vec::new).push(relation);

        Ok(())
    }

    fn remove_relation(
        &self,
        source_id: miyukini_kernel::Id,
        relation_type: String,
        target_id: miyukini_kernel::Id,
    ) -> Result<(), ContentError> {
        let mut relations = self.relations.lock().unwrap();
        if let Some(rels) = relations.get_mut(&source_id) {
            rels.retain(|r| {
                !(r.relation_type == relation_type && r.target_id == target_id)
            });
            Ok(())
        } else {
            Err(ContentError::InvalidRelation)
        }
    }

    fn list_relations(
        &self,
        content_id: miyukini_kernel::Id,
    ) -> Result<Vec<ContentRelation>, ContentError> {
        let relations = self.relations.lock().unwrap();
        Ok(relations.get(&content_id).cloned().unwrap_or_default())
    }

    fn create_version(&self, content_id: miyukini_kernel::Id) -> Result<miyukini_kernel::Id, ContentError> {
        let content = {
            let contents = self.contents.lock().unwrap();
            contents.get(&content_id).cloned().ok_or(ContentError::NotFound)?
        };

        let version_id = self.id_generator.generate();
        let snapshot = self.serialize_content(&content)?;
        let version = ContentVersion::new(version_id, content_id, snapshot, self.clock.now());

        let mut versions = self.versions.lock().unwrap();
        versions.entry(content_id).or_insert_with(Vec::new).push(version);

        Ok(version_id)
    }

    fn get_version(
        &self,
        content_id: miyukini_kernel::Id,
        version_id: miyukini_kernel::Id,
    ) -> Result<ContentVersion, ContentError> {
        let versions = self.versions.lock().unwrap();
        versions
            .get(&content_id)
            .and_then(|vers| vers.iter().find(|v| v.version_id == version_id))
            .cloned()
            .ok_or(ContentError::VersionNotFound)
    }

    fn list_versions(
        &self,
        content_id: miyukini_kernel::Id,
    ) -> Result<Vec<ContentVersion>, ContentError> {
        let versions = self.versions.lock().unwrap();
        Ok(versions.get(&content_id).cloned().unwrap_or_default())
    }

    fn restore_version(
        &self,
        content_id: miyukini_kernel::Id,
        version_id: miyukini_kernel::Id,
    ) -> Result<(), ContentError> {
        // Récupérer la version
        let version = self.get_version(content_id, version_id)?;

        // Désérialiser le snapshot
        let mut restored_content = self.deserialize_content(&version.snapshot)?;
        
        // Restaurer l'ID et les dates du contenu original
        let mut contents = self.contents.lock().unwrap();
        if let Some(original_content) = contents.get(&content_id) {
            restored_content.id = content_id;
            restored_content.created_at = original_content.created_at;
            restored_content.updated_at = self.clock.now();
            
            // Remplacer le contenu
            contents.insert(content_id, restored_content);
            Ok(())
        } else {
            Err(ContentError::NotFound)
        }
    }
}

impl MemoryContentManager {
    /// Sérialise un contenu en bytes (format simple pour la démo).
    ///
    /// Format simplifié pour la démo : on stocke une copie du contenu complet.
    /// En production, le produit utilisera son propre format (JSON, protobuf, etc.).
    ///
    /// Note : Pour une vraie implémentation, on utiliserait serde_json ou un autre format,
    /// mais on évite les dépendances externes. Le produit implémentera sa propre sérialisation.
    fn serialize_content(&self, content: &Content) -> Result<Vec<u8>, ContentError> {
        // Pour la démo, on stocke le contenu complet dans un format simple.
        // Format : on encode les champs essentiels de manière basique.
        // En production, le produit sérialisera selon son format (JSON, protobuf, etc.).
        
        // Format simplifié pour la démo : on encode les champs dans un format simple
        // (content_type length + content_type + status as u8 + metadata length + metadata)
        let mut bytes = Vec::new();
        
        // Content type
        let content_type_bytes = content.content_type.as_bytes();
        bytes.extend_from_slice(&(content_type_bytes.len() as u32).to_le_bytes());
        bytes.extend_from_slice(content_type_bytes);
        
        // Status (u8: 0=Draft, 1=Published, 2=Archived)
        let status_byte = match content.status {
            ContentStatus::Draft => 0u8,
            ContentStatus::Published => 1u8,
            ContentStatus::Archived => 2u8,
        };
        bytes.push(status_byte);
        
        // Metadata
        bytes.extend_from_slice(&(content.metadata.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&content.metadata);
        
        Ok(bytes)
    }

    /// Désérialise un contenu depuis bytes.
    ///
    /// Format simplifié pour la démo : correspond à serialize_content.
    /// En production, le produit désérialisera selon son format.
    fn deserialize_content(&self, snapshot: &[u8]) -> Result<Content, ContentError> {
        let mut offset = 0;
        
        // Lire content_type
        if offset + 4 > snapshot.len() {
            return Err(ContentError::Other("invalid snapshot format".to_string()));
        }
        let content_type_len = u32::from_le_bytes([
            snapshot[offset],
            snapshot[offset + 1],
            snapshot[offset + 2],
            snapshot[offset + 3],
        ]) as usize;
        offset += 4;
        
        if offset + content_type_len > snapshot.len() {
            return Err(ContentError::Other("invalid snapshot format".to_string()));
        }
        let content_type = String::from_utf8(snapshot[offset..offset + content_type_len].to_vec())
            .map_err(|_| ContentError::Other("invalid content type encoding".to_string()))?;
        offset += content_type_len;
        
        // Lire status
        if offset >= snapshot.len() {
            return Err(ContentError::Other("invalid snapshot format".to_string()));
        }
        let status = match snapshot[offset] {
            0 => ContentStatus::Draft,
            1 => ContentStatus::Published,
            2 => ContentStatus::Archived,
            _ => return Err(ContentError::Other("invalid status".to_string())),
        };
        offset += 1;
        
        // Lire metadata
        if offset + 4 > snapshot.len() {
            return Err(ContentError::Other("invalid snapshot format".to_string()));
        }
        let metadata_len = u32::from_le_bytes([
            snapshot[offset],
            snapshot[offset + 1],
            snapshot[offset + 2],
            snapshot[offset + 3],
        ]) as usize;
        offset += 4;
        
        if offset + metadata_len > snapshot.len() {
            return Err(ContentError::Other("invalid snapshot format".to_string()));
        }
        let metadata = snapshot[offset..offset + metadata_len].to_vec();
        
        // Reconstruire le contenu (sans ID ni dates, qui seront mis à jour lors de la restauration)
        // Note : pour la démo, on utilise des valeurs par défaut pour id et dates
        // En production, le produit gérera cela correctement
        let dummy_id = self.id_generator.generate();
        let now = self.clock.now();
        
        Ok(Content::new(
            dummy_id, // Sera remplacé par le content_id lors de la restauration
            content_type,
            status,
            now,
            now,
            metadata,
        ))
    }
}
