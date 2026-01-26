# Miyukini Framework - Adaptateur PublicationManager

> Guide d'implémentation de l'adaptateur PublicationManager pour le Module Publication SPM CMS.

---

## Contexte

Le Module Publication gère le cycle de vie éditorial des contenus : brouillon, programmation, publication, archivage. Il permet de contrôler quand et comment un contenu est publié.

**Caractéristiques :**
- Statuts : Draft, Scheduled, Published, Archived
- Programmation de publication (date future)
- Statut effectif (Scheduled → Published automatique)

---

## 1. Trait à implémenter

### PublicationManager

Le trait `PublicationManager` expose **6 méthodes** à implémenter :

1. `create_publication()` - Créer une publication
2. `status()` - Obtenir le statut actuel
3. `schedule()` - Programmer une publication
4. `publish_now()` - Publier immédiatement
5. `archive()` - Archiver une publication
6. `effective_status()` - Obtenir le statut effectif

**Module :** `miyukini-spm-cms-publication`

**Import :**
```rust
use miyukini_spm_cms_publication::{
    PublicationManager, PublicationError, PublicationId, ContentId,
    PublicationStatus, Publication
};
```

---

## 2. Structure de l'adaptateur

### Structure de base

```rust
use miyukini_spm_cms_publication::{
    PublicationManager, PublicationError, PublicationId, ContentId,
    PublicationStatus, Publication
};
use miyukini_kernel::{Id, IdGenerator, Clock, Logger};
use std::sync::Arc;

pub struct PublicationAdapter {
    // Dépendances kernel
    id_generator: Arc<dyn IdGenerator>,
    clock: Arc<dyn Clock>,
    logger: Option<Arc<dyn Logger>>,
    
    // Stack technique du produit
    // db: Arc<Database>,
    // repository: PublicationRepository,
}

impl PublicationAdapter {
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
- `PublicationId` : Identifiant d'une publication (alias `Id`)
- `ContentId` : Identifiant du contenu associé (alias `Id`)
- `PublicationStatus` : Statut (Draft, Scheduled, Published, Archived)
- `Publication` : Entité principale
- `PublicationError` : Erreurs

---

## 3. Implémentation des méthodes

### 3.1. create_publication()

**Responsabilité :** Créer une publication pour un contenu (état Draft).

**Exemple :**
```rust
impl PublicationManager for PublicationAdapter {
    fn create_publication(&self, content_id: ContentId) -> Result<PublicationId, PublicationError> {
        let id = self.id_generator.generate();
        let now = self.clock.now();
        
        let publication = Publication::new(
            id,
            content_id,
            PublicationStatus::Draft,
            None, // Pas de date programmée
            now,
            now,
        );
        
        self.repository.save(publication)?;
        
        Ok(id)
    }
}
```

---

### 3.2. status()

**Exemple :**
```rust
fn status(&self, publication_id: PublicationId) -> Result<PublicationStatus, PublicationError> {
    let publication = self.repository.find(publication_id)
        .map_err(|e| self.translate_error(e))?
        .ok_or(PublicationError::NotFound)?;
    
    Ok(publication.status)
}
```

---

### 3.3. schedule()

**Responsabilité :** Programmer une publication (Draft → Scheduled).

**Exemple :**
```rust
fn schedule(
    &self,
    publication_id: PublicationId,
    publish_at: SystemTime,
) -> Result<(), PublicationError> {
    let mut publication = self.repository.find(publication_id)
        .map_err(|e| self.translate_error(e))?
        .ok_or(PublicationError::NotFound)?;
    
    // Vérifier date dans le futur
    let now = self.clock.now();
    if publish_at <= now {
        return Err(PublicationError::InvalidSchedule);
    }
    
    // Transition Draft → Scheduled
    if publication.status != PublicationStatus::Draft {
        return Err(PublicationError::InvalidTransition);
    }
    
    publication.status = PublicationStatus::Scheduled;
    publication.publish_at = Some(publish_at);
    publication.updated_at = now;
    
    self.repository.update(publication)?;
    
    Ok(())
}
```

---

### 3.4. publish_now()

**Responsabilité :** Publier immédiatement (Draft → Published ou Scheduled → Published).

**Exemple :**
```rust
fn publish_now(&self, publication_id: PublicationId) -> Result<(), PublicationError> {
    let mut publication = self.repository.find(publication_id)
        .map_err(|e| self.translate_error(e))?
        .ok_or(PublicationError::NotFound)?;
    
    let now = self.clock.now();
    
    // Transitions autorisées
    match publication.status {
        PublicationStatus::Draft => {
            publication.status = PublicationStatus::Published;
        }
        PublicationStatus::Scheduled => {
            // Vérifier que la date est passée
            if let Some(publish_at) = publication.publish_at {
                if publish_at > now {
                    return Err(PublicationError::InvalidTransition);
                }
            }
            publication.status = PublicationStatus::Published;
        }
        _ => return Err(PublicationError::InvalidTransition),
    }
    
    publication.updated_at = now;
    self.repository.update(publication)?;
    
    Ok(())
}
```

---

### 3.5. archive()

**Responsabilité :** Archiver une publication (Published → Archived).

**Exemple :**
```rust
fn archive(&self, publication_id: PublicationId) -> Result<(), PublicationError> {
    let mut publication = self.repository.find(publication_id)
        .map_err(|e| self.translate_error(e))?
        .ok_or(PublicationError::NotFound)?;
    
    // Transition Published → Archived
    if publication.status != PublicationStatus::Published {
        return Err(PublicationError::InvalidTransition);
    }
    
    publication.status = PublicationStatus::Archived;
    publication.updated_at = self.clock.now();
    
    self.repository.update(publication)?;
    
    Ok(())
}
```

---

### 3.6. effective_status()

**Responsabilité :** Retourner le statut effectif (Scheduled → Published si date passée).

**Exemple :**
```rust
fn effective_status(
    &self,
    publication_id: PublicationId,
) -> Result<PublicationStatus, PublicationError> {
    let publication = self.repository.find(publication_id)
        .map_err(|e| self.translate_error(e))?
        .ok_or(PublicationError::NotFound)?;
    
    // Pour Scheduled, vérifier si la date est passée
    if publication.status == PublicationStatus::Scheduled {
        if let Some(publish_at) = publication.publish_at {
            let now = self.clock.now();
            if publish_at <= now {
                return Ok(PublicationStatus::Published);
            }
        }
    }
    
    Ok(publication.status)
}
```

---

## 4. Points d'attention spécifiques

### Transitions de statut

- **Draft → Scheduled :** Date doit être dans le futur
- **Draft → Published :** Direct
- **Scheduled → Published :** Si date passée
- **Published → Archived :** Final (pas de retour)

### Programmation automatique

- **Worker :** Créer un worker pour publier automatiquement les Scheduled
- **Vérification :** Vérifier périodiquement les publications programmées

---

## 5. Cas d'usage courants

### Créer une publication

```rust
let publication_id = adapter.create_publication(content_id)?;
```

### Programmer une publication

```rust
let publish_at = SystemTime::now() + Duration::from_secs(86400); // Dans 24h
adapter.schedule(publication_id, publish_at)?;
```

### Publier immédiatement

```rust
adapter.publish_now(publication_id)?;
```

### Vérifier le statut effectif

```rust
let status = adapter.effective_status(publication_id)?;
// Pour Scheduled : retourne Published si la date est passée
```

### Archiver une publication

```rust
adapter.archive(publication_id)?;
```

---

## 6. Tests recommandés

### Tests unitaires

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_publication() {
        let adapter = create_test_adapter();
        let content_id = generate_test_id();
        
        let publication_id = adapter.create_publication(content_id).unwrap();
        assert!(!publication_id.is_nil());
        
        let status = adapter.status(publication_id).unwrap();
        assert_eq!(status, PublicationStatus::Draft);
    }
    
    #[test]
    fn test_schedule_publication() {
        let adapter = create_test_adapter();
        let content_id = generate_test_id();
        let publication_id = adapter.create_publication(content_id).unwrap();
        
        let publish_at = SystemTime::now() + Duration::from_secs(3600);
        adapter.schedule(publication_id, publish_at).unwrap();
        
        let status = adapter.status(publication_id).unwrap();
        assert_eq!(status, PublicationStatus::Scheduled);
    }
    
    #[test]
    fn test_publish_now() {
        let adapter = create_test_adapter();
        let content_id = generate_test_id();
        let publication_id = adapter.create_publication(content_id).unwrap();
        
        adapter.publish_now(publication_id).unwrap();
        
        let status = adapter.status(publication_id).unwrap();
        assert_eq!(status, PublicationStatus::Published);
    }
    
    #[test]
    fn test_effective_status_scheduled() {
        let adapter = create_test_adapter();
        let content_id = generate_test_id();
        let publication_id = adapter.create_publication(content_id).unwrap();
        
        // Programmer dans le passé
        let publish_at = SystemTime::now() - Duration::from_secs(3600);
        adapter.schedule(publication_id, publish_at).unwrap();
        
        // Le statut effectif doit être Published
        let effective = adapter.effective_status(publication_id).unwrap();
        assert_eq!(effective, PublicationStatus::Published);
    }
    
    #[test]
    fn test_archive() {
        let adapter = create_test_adapter();
        let content_id = generate_test_id();
        let publication_id = adapter.create_publication(content_id).unwrap();
        
        adapter.publish_now(publication_id).unwrap();
        adapter.archive(publication_id).unwrap();
        
        let status = adapter.status(publication_id).unwrap();
        assert_eq!(status, PublicationStatus::Archived);
    }
    
    #[test]
    fn test_invalid_transitions() {
        let adapter = create_test_adapter();
        let content_id = generate_test_id();
        let publication_id = adapter.create_publication(content_id).unwrap();
        
        // Ne peut pas archiver un Draft
        assert!(adapter.archive(publication_id).is_err());
        
        // Publier puis archiver
        adapter.publish_now(publication_id).unwrap();
        adapter.archive(publication_id).unwrap();
        
        // Ne peut plus changer d'état après archivage
        assert!(adapter.publish_now(publication_id).is_err());
    }
}
```

### Tests d'intégration

```rust
#[test]
fn test_integration_with_database() {
    let db = setup_test_database();
    let adapter = PublicationAdapter::new(/* ... */);
    
    // Test création publication
    let content_id = generate_test_id();
    let publication_id = adapter.create_publication(content_id).unwrap();
    
    // Vérifier en DB
    let db_publication = db.get_publication(publication_id).unwrap();
    assert_eq!(db_publication.status, PublicationStatus::Draft);
}
```

---

## 7. Références

- **Implémentation mémoire :** `crates/miyukini-spm-cms-publication/src/memory.rs`
- **Guide général :** `docs/spm-cms/Miyukini Framework - Guide Adaptateurs Produits.md`
