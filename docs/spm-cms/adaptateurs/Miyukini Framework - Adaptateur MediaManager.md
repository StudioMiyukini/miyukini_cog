# Miyukini Framework - Adaptateur MediaManager

> Guide d'implémentation de l'adaptateur MediaManager pour le Module Médias SPM CMS.

---

## Contexte

Le Module Médias gère les assets (images, vidéos, fichiers) et leurs liens avec des entités. Il ne gère pas l'upload HTTP, le rendu, ni les permissions - uniquement les métadonnées et les liens.

**Caractéristiques :**
- Gestion des métadonnées de médias
- Liens avec des entités externes (many-to-many)
- Pas de stockage de fichiers (responsabilité du produit)

---

## 1. Trait à implémenter

### MediaManager

Le trait `MediaManager` expose **6 méthodes** à implémenter :

1. `create_media()` - Créer un média
2. `get_media()` - Lire un média
3. `delete_media()` - Supprimer un média
4. `attach_media_to_entity()` - Attacher un média à une entité
5. `detach_media_from_entity()` - Détacher un média d'une entité
6. `list_media_for_entity()` - Lister les médias d'une entité

**Module :** `miyukini-spm-cms-media`

**Import :**
```rust
use miyukini_spm_cms_media::{
    MediaManager, MediaError, MediaId, EntityId, Media, MediaInput
};
```

---

## 2. Structure de l'adaptateur

### Structure de base

```rust
use miyukini_spm_cms_media::{
    MediaManager, MediaError, MediaId, EntityId, Media, MediaInput
};
use miyukini_kernel::{Id, IdGenerator, Clock, Logger};
use std::sync::Arc;

pub struct MediaAdapter {
    // Dépendances kernel
    id_generator: Arc<dyn IdGenerator>,
    clock: Arc<dyn Clock>,
    logger: Option<Arc<dyn Logger>>,
    
    // Stack technique du produit
    // db: Arc<Database>,
    // file_storage: FileStorage,
    // repository: MediaRepository,
}

impl MediaAdapter {
    pub fn new(
        id_generator: Arc<dyn IdGenerator>,
        clock: Arc<dyn Clock>,
        logger: Option<Arc<dyn Logger>>,
        // ... autres dépendances produit
    ) -> Self {
        Self {
            id_generator,
            clock,
            logger,
            // ... initialisation
        }
    }
}
```

### Types de données

**Types SPM :**
- `MediaId` : Identifiant d'un média (alias `Id`)
- `EntityId` : Identifiant d'une entité externe (alias `Id`)
- `Media` : Entité principale avec id, mime_type, metadata, dates
- `MediaInput` : Données d'entrée pour création
- `MediaError` : Erreurs

---

## 3. Implémentation des méthodes

### 3.1. create_media()

**Exemple :**
```rust
impl MediaManager for MediaAdapter {
    fn create_media(&self, input: MediaInput) -> Result<MediaId, MediaError> {
        let id = self.id_generator.generate();
        let now = self.clock.now();
        
        let media = Media::new(
            id,
            input.mime_type,
            input.metadata,
            now,
            now,
        );
        
        self.repository.save(media)?;
        
        Ok(id)
    }
}
```

---

### 3.2. get_media()

**Exemple :**
```rust
fn get_media(&self, id: MediaId) -> Result<Media, MediaError> {
    self.repository.find(id)
        .map_err(|e| self.translate_error(e))?
        .ok_or(MediaError::NotFound)
}
```

---

### 3.3. delete_media()

**Responsabilité :** Supprimer un média et tous ses liens.

**Exemple :**
```rust
fn delete_media(&self, id: MediaId) -> Result<(), MediaError> {
    // Supprimer tous les liens
    self.repository.delete_all_links(id)?;
    
    // Supprimer le média
    self.repository.delete(id)?;
    
    // Optionnel : supprimer le fichier du stockage
    // self.file_storage.delete(id)?;
    
    Ok(())
}
```

---

### 3.4. attach_media_to_entity()

**Exemple :**
```rust
fn attach_media_to_entity(
    &self,
    media_id: MediaId,
    entity_id: EntityId,
) -> Result<(), MediaError> {
    // Vérifier média existe
    self.get_media(media_id)?;
    
    // Attacher (idempotent)
    self.repository.save_link(media_id, entity_id)?;
    
    Ok(())
}
```

---

### 3.5. detach_media_from_entity()

**Exemple :**
```rust
fn detach_media_from_entity(
    &self,
    media_id: MediaId,
    entity_id: EntityId,
) -> Result<(), MediaError> {
    self.repository.delete_link(media_id, entity_id)?;
    Ok(())
}
```

---

### 3.6. list_media_for_entity()

**Exemple :**
```rust
fn list_media_for_entity(&self, entity_id: EntityId) -> Result<Vec<Media>, MediaError> {
    let media_ids = self.repository.find_media_by_entity(entity_id)?;
    
    let mut media = Vec::new();
    for media_id in media_ids {
        if let Ok(m) = self.get_media(media_id) {
            media.push(m);
        }
    }
    
    Ok(media)
}
```

---

## 4. Points d'attention spécifiques

### Stockage de fichiers

- **Responsabilité produit :** Le module ne gère pas le stockage de fichiers
- **Métadonnées :** Stocker les métadonnées (chemin, taille, etc.) dans `metadata`
- **Synchronisation :** Gérer la synchronisation fichier ↔ métadonnées

### Liens many-to-many

- **Performance :** Utiliser des index sur `media_id` et `entity_id`
- **Cohérence :** Supprimer les liens lors de la suppression d'un média

---

## 5. Cas d'usage courants

### Créer un média

```rust
let input = MediaInput::new(
    "image/jpeg".to_string(),
    serde_json::to_vec(&media_metadata).unwrap(),
);
let media_id = adapter.create_media(input)?;
```

### Attacher un média à un contenu

```rust
adapter.attach_media_to_entity(media_id, content_id)?;
```

### Lister les médias d'un contenu

```rust
let media_list = adapter.list_media_for_entity(content_id)?;
for media in media_list {
    println!("Media: {:?}, Type: {}", media.id, media.mime_type);
}
```

### Détacher un média

```rust
adapter.detach_media_from_entity(media_id, content_id)?;
```

### Supprimer un média

```rust
// Supprime le média et tous ses liens
adapter.delete_media(media_id)?;
```

---

## 6. Tests recommandés

### Tests unitaires

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_media() {
        let adapter = create_test_adapter();
        let input = MediaInput::new(
            "image/jpeg".to_string(),
            b"{}".to_vec(),
        );
        
        let media_id = adapter.create_media(input).unwrap();
        assert!(!media_id.is_nil());
        
        let media = adapter.get_media(media_id).unwrap();
        assert_eq!(media.mime_type, "image/jpeg");
    }
    
    #[test]
    fn test_attach_media() {
        let adapter = create_test_adapter();
        let input = MediaInput::new("image/jpeg".to_string(), b"{}".to_vec());
        let media_id = adapter.create_media(input).unwrap();
        let entity_id = generate_test_id();
        
        adapter.attach_media_to_entity(media_id, entity_id).unwrap();
        
        let media_list = adapter.list_media_for_entity(entity_id).unwrap();
        assert_eq!(media_list.len(), 1);
        assert_eq!(media_list[0].id, media_id);
    }
    
    #[test]
    fn test_detach_media() {
        let adapter = create_test_adapter();
        let input = MediaInput::new("image/jpeg".to_string(), b"{}".to_vec());
        let media_id = adapter.create_media(input).unwrap();
        let entity_id = generate_test_id();
        
        adapter.attach_media_to_entity(media_id, entity_id).unwrap();
        adapter.detach_media_from_entity(media_id, entity_id).unwrap();
        
        let media_list = adapter.list_media_for_entity(entity_id).unwrap();
        assert_eq!(media_list.len(), 0);
    }
    
    #[test]
    fn test_delete_media() {
        let adapter = create_test_adapter();
        let input = MediaInput::new("image/jpeg".to_string(), b"{}".to_vec());
        let media_id = adapter.create_media(input).unwrap();
        let entity_id = generate_test_id();
        
        adapter.attach_media_to_entity(media_id, entity_id).unwrap();
        adapter.delete_media(media_id).unwrap();
        
        // Le média ne doit plus exister
        assert!(adapter.get_media(media_id).is_err());
        
        // Les liens doivent être supprimés
        let media_list = adapter.list_media_for_entity(entity_id).unwrap();
        assert_eq!(media_list.len(), 0);
    }
}
```

### Tests d'intégration

```rust
#[test]
fn test_integration_with_database() {
    let db = setup_test_database();
    let adapter = MediaAdapter::new(/* ... */);
    
    // Test création média
    let input = MediaInput::new("image/jpeg".to_string(), b"{}".to_vec());
    let media_id = adapter.create_media(input).unwrap();
    
    // Vérifier en DB
    let db_media = db.get_media(media_id).unwrap();
    assert_eq!(db_media.mime_type, "image/jpeg");
}
```

---

## 7. Références

- **Implémentation mémoire :** `crates/miyukini-spm-cms-media/src/memory.rs`
- **Guide général :** `docs/spm-cms/Miyukini Framework - Guide Adaptateurs Produits.md`
