# Miyukini Framework - Adaptateur ContentManager

> Guide d'implémentation de l'adaptateur ContentManager pour le Module Contenu SPM CMS.

---

## Contexte

Le Module Contenu est le **module fondation** du SPM CMS. Il gère les entités de contenu (pages, articles, blocs) avec CRUD complet, statuts (brouillon/publié/archivé), relations entre contenus, versioning et métadonnées.

**Référence :** Voir le contrat complet dans `docs/spm-cms/modules/content/contrat.md`

---

## 1. Trait à implémenter

### ContentManager

Le trait `ContentManager` expose **12 méthodes** à implémenter :

1. `create_content()` - Créer un contenu
2. `get_content()` - Lire un contenu
3. `update_content()` - Modifier un contenu
4. `delete_content()` - Supprimer un contenu
5. `list_contents()` - Lister des contenus avec filtres
6. `add_relation()` - Ajouter une relation
7. `remove_relation()` - Supprimer une relation
8. `list_relations()` - Lister les relations
9. `create_version()` - Créer une version
10. `get_version()` - Lire une version
11. `list_versions()` - Lister les versions
12. `restore_version()` - Restaurer une version

**Module :** `miyukini-spm-cms-content`

**Import :**
```rust
use miyukini_spm_cms_content::{
    ContentManager, ContentInput, Content, ContentUpdates, ContentFilters,
    ContentListResult, ContentRelation, ContentVersion, ContentError, ContentStatus
};
```

---

## 2. Structure de l'adaptateur

### Structure de base

```rust
use miyukini_spm_cms_content::{
    ContentManager, ContentInput, Content, ContentUpdates, ContentFilters,
    ContentListResult, ContentRelation, ContentVersion, ContentError, ContentStatus
};
use miyukini_kernel::{Id, IdGenerator, Clock, Logger};
use std::sync::Arc;

pub struct ContentAdapter {
    // Dépendances kernel
    id_generator: Arc<dyn IdGenerator>,
    clock: Arc<dyn Clock>,
    logger: Option<Arc<dyn Logger>>,
    
    // Stack technique du produit
    // Exemples selon la stack :
    // db: Arc<Database>,
    // repository: ContentRepository,
    // version_storage: VersionStorage,
}

impl ContentAdapter {
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
- `Content` : Entité principale avec id, content_type, status, dates, metadata
- `ContentInput` : Données d'entrée pour création
- `ContentUpdates` : Mises à jour partielles
- `ContentFilters` : Filtres pour la liste
- `ContentRelation` : Relation entre contenus
- `ContentVersion` : Version (snapshot) d'un contenu
- `ContentStatus` : Statut (Draft, Published, Archived)

**Types kernel :**
- `Id` : Identifiant unique (alias `ContentId`)
- `SystemTime` : Horodatage

---

## 3. Implémentation des méthodes

### 3.1. create_content()

**Responsabilité :** Créer un nouveau contenu.

**Étapes :**
1. Générer l'ID via `id_generator.generate()`
2. Obtenir l'horodatage via `clock.now()`
3. Valider les données d'entrée
4. Créer l'entité `Content`
5. Persister dans la DB
6. Retourner l'ID

**Exemple :**
```rust
impl ContentManager for ContentAdapter {
    fn create_content(&self, input: ContentInput) -> Result<Id, ContentError> {
        // 1. Générer ID
        let id = self.id_generator.generate();
        
        // 2. Obtenir horodatage
        let now = self.clock.now();
        
        // 3. Valider
        self.validate_content_input(&input)?;
        
        // 4. Créer l'entité
        let status = input.status.unwrap_or(ContentStatus::Draft);
        let content = Content::new(
            id,
            input.content_type,
            status,
            now,
            now,
            input.metadata,
        );
        
        // 5. Persister
        self.repository.save(content)?;
        
        // 6. Logger
        self.log_operation("create_content", &format!("Created content {:?}", id));
        
        Ok(id)
    }
}
```

**Points d'attention :**
- Le statut par défaut est `Draft` si non spécifié
- Les dates `created_at` et `updated_at` sont identiques à la création
- Valider le `content_type` selon les règles du produit

---

### 3.2. get_content()

**Responsabilité :** Lire un contenu par son identifiant.

**Étapes :**
1. Récupérer depuis la DB
2. Vérifier l'existence
3. Traduire depuis le format DB vers `Content`
4. Retourner

**Exemple :**
```rust
fn get_content(&self, id: Id) -> Result<Content, ContentError> {
    // 1. Récupérer depuis DB
    let db_content = self.repository.find_by_id(id)
        .map_err(|e| self.translate_error(e))?;
    
    // 2. Vérifier existence
    let db_content = db_content.ok_or(ContentError::NotFound)?;
    
    // 3. Traduire vers Content
    let content = self.translate_from_db_format(db_content)?;
    
    Ok(content)
}
```

**Points d'attention :**
- Gérer le cas où le contenu n'existe pas (`NotFound`)
- Traduire correctement les dates depuis la DB
- Désérialiser les métadonnées si nécessaire

---

### 3.3. update_content()

**Responsabilité :** Modifier un contenu (mise à jour partielle).

**Étapes :**
1. Récupérer le contenu existant
2. Créer une version si `create_version = true`
3. Appliquer les mises à jour
4. Mettre à jour `updated_at` via `clock.now()`
5. Persister

**Exemple :**
```rust
fn update_content(
    &self,
    id: Id,
    updates: ContentUpdates,
    create_version: bool,
) -> Result<(), ContentError> {
    // 1. Récupérer le contenu
    let mut content = self.get_content(id)?;
    
    // 2. Créer version si demandé
    if create_version {
        self.create_version_internal(id, &content)?;
    }
    
    // 3. Appliquer les mises à jour
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
    
    // 4. Mettre à jour updated_at
    content.updated_at = self.clock.now();
    
    // 5. Persister
    self.repository.update(content)?;
    
    Ok(())
}
```

**Points d'attention :**
- Valider les transitions de statut selon les règles du produit
- Créer la version **avant** d'appliquer les mises à jour
- Toujours mettre à jour `updated_at`

---

### 3.4. delete_content()

**Responsabilité :** Supprimer un contenu (soft ou hard delete).

**Étapes :**
1. Récupérer le contenu
2. Si `soft = true` : changer le statut à `Archived`
3. Si `soft = false` : supprimer définitivement
4. Gérer les relations et versions selon les règles du produit

**Exemple :**
```rust
fn delete_content(&self, id: Id, soft: bool) -> Result<(), ContentError> {
    if soft {
        // Archivage
        let mut content = self.get_content(id)?;
        content.status = ContentStatus::Archived;
        content.updated_at = self.clock.now();
        self.repository.update(content)?;
    } else {
        // Suppression définitive
        // Vérifier les contraintes (ex. contenus enfants)
        self.check_deletion_constraints(id)?;
        
        // Supprimer les relations
        self.repository.delete_relations(id)?;
        
        // Supprimer les versions
        self.repository.delete_versions(id)?;
        
        // Supprimer le contenu
        self.repository.delete(id)?;
    }
    
    Ok(())
}
```

**Points d'attention :**
- Vérifier les contraintes avant suppression définitive
- Gérer la cascade des relations selon les règles du produit
- Supprimer les versions si nécessaire

---

### 3.5. list_contents()

**Responsabilité :** Lister des contenus avec filtres et pagination.

**Étapes :**
1. Construire la requête avec filtres
2. Compter le total (sans pagination)
3. Appliquer pagination (offset, limit)
4. Récupérer les résultats
5. Traduire vers `Content`
6. Retourner `ContentListResult`

**Exemple :**
```rust
fn list_contents(
    &self,
    filters: ContentFilters,
    offset: usize,
    limit: usize,
) -> Result<ContentListResult, ContentError> {
    // 1. Construire requête
    let query = self.build_query(filters)?;
    
    // 2. Compter total
    let total = self.repository.count(&query)?;
    
    // 3. Appliquer pagination et récupérer
    let db_contents = self.repository.find_with_pagination(&query, offset, limit)?;
    
    // 4. Traduire
    let contents: Vec<Content> = db_contents
        .into_iter()
        .map(|db| self.translate_from_db_format(db))
        .collect::<Result<_, _>>()?;
    
    // 5. Retourner résultat
    Ok(ContentListResult::new(contents, total, offset, limit))
}
```

**Points d'attention :**
- Le `total` doit être calculé **sans** pagination
- Gérer efficacement les grandes listes (index DB)
- Traduire tous les filtres vers la requête DB

---

### 3.6. add_relation()

**Responsabilité :** Ajouter une relation entre deux contenus.

**Étapes :**
1. Vérifier que les deux contenus existent
2. Vérifier qu'il n'y a pas de cycle (si hiérarchique)
3. Créer la relation
4. Persister

**Exemple :**
```rust
fn add_relation(
    &self,
    source_id: Id,
    relation_type: String,
    target_id: Id,
) -> Result<(), ContentError> {
    // 1. Vérifier existence
    self.get_content(source_id)?;
    self.get_content(target_id)?;
    
    // 2. Vérifier cycle (si hiérarchique)
    if relation_type == "parent" {
        self.check_no_cycle(source_id, target_id)?;
    }
    
    // 3. Créer relation
    let relation = ContentRelation::new(source_id, relation_type, target_id);
    
    // 4. Persister
    self.repository.save_relation(relation)?;
    
    Ok(())
}
```

**Points d'attention :**
- Vérifier l'existence des contenus
- Détecter les cycles pour les relations hiérarchiques
- Gérer les relations bidirectionnelles si nécessaire

---

### 3.7. remove_relation()

**Responsabilité :** Supprimer une relation entre deux contenus.

**Étapes :**
1. Vérifier que la relation existe
2. Supprimer de la DB

**Exemple :**
```rust
fn remove_relation(
    &self,
    source_id: Id,
    relation_type: String,
    target_id: Id,
) -> Result<(), ContentError> {
    // Vérifier existence
    let relation = self.repository.find_relation(source_id, &relation_type, target_id)
        .map_err(|e| self.translate_error(e))?;
    
    if relation.is_none() {
        return Err(ContentError::InvalidRelation);
    }
    
    // Supprimer
    self.repository.delete_relation(source_id, &relation_type, target_id)?;
    
    Ok(())
}
```

---

### 3.8. list_relations()

**Responsabilité :** Lister toutes les relations d'un contenu.

**Exemple :**
```rust
fn list_relations(&self, content_id: Id) -> Result<Vec<ContentRelation>, ContentError> {
    // Vérifier existence du contenu
    self.get_content(content_id)?;
    
    // Récupérer relations
    let relations = self.repository.find_relations_by_source(content_id)?;
    
    Ok(relations)
}
```

---

### 3.9. create_version()

**Responsabilité :** Créer une version (snapshot) d'un contenu.

**Étapes :**
1. Récupérer le contenu
2. Sérialiser le contenu (snapshot)
3. Générer un ID pour la version
4. Créer `ContentVersion`
5. Persister

**Exemple :**
```rust
fn create_version(&self, content_id: Id) -> Result<Id, ContentError> {
    // 1. Récupérer contenu
    let content = self.get_content(content_id)?;
    
    // 2. Sérialiser (format défini par le produit)
    let snapshot = self.serialize_content(&content)?;
    
    // 3. Générer ID version
    let version_id = self.id_generator.generate();
    
    // 4. Créer version
    let version = ContentVersion::new(
        version_id,
        content_id,
        snapshot,
        self.clock.now(),
    );
    
    // 5. Persister
    self.repository.save_version(version)?;
    
    Ok(version_id)
}
```

**Points d'attention :**
- Le format de sérialisation est défini par le produit
- Le snapshot doit contenir toutes les données nécessaires pour restaurer
- Utiliser un format stable (JSON, protobuf, etc.)

---

### 3.10. get_version()

**Responsabilité :** Lire une version spécifique.

**Exemple :**
```rust
fn get_version(&self, content_id: Id, version_id: Id) -> Result<ContentVersion, ContentError> {
    let version = self.repository.find_version(content_id, version_id)
        .map_err(|e| self.translate_error(e))?
        .ok_or(ContentError::VersionNotFound)?;
    
    Ok(version)
}
```

---

### 3.11. list_versions()

**Responsabilité :** Lister toutes les versions d'un contenu.

**Exemple :**
```rust
fn list_versions(&self, content_id: Id) -> Result<Vec<ContentVersion>, ContentError> {
    // Vérifier existence du contenu
    self.get_content(content_id)?;
    
    // Récupérer versions
    let versions = self.repository.find_versions_by_content(content_id)?;
    
    Ok(versions)
}
```

---

### 3.12. restore_version()

**Responsabilité :** Restaurer une version (crée une nouvelle version avec le snapshot restauré).

**Étapes :**
1. Récupérer la version
2. Désérialiser le snapshot
3. Restaurer l'ID et les dates du contenu original
4. Créer une nouvelle version (pour l'historique)
5. Mettre à jour le contenu

**Exemple :**
```rust
fn restore_version(&self, content_id: Id, version_id: Id) -> Result<(), ContentError> {
    // 1. Récupérer version
    let version = self.get_version(content_id, version_id)?;
    
    // 2. Désérialiser
    let mut restored_content = self.deserialize_content(&version.snapshot)?;
    
    // 3. Restaurer ID et dates
    let original_content = self.get_content(content_id)?;
    restored_content.id = content_id;
    restored_content.created_at = original_content.created_at;
    restored_content.updated_at = self.clock.now();
    
    // 4. Créer nouvelle version (pour historique)
    self.create_version(content_id)?;
    
    // 5. Mettre à jour contenu
    self.repository.update(restored_content)?;
    
    Ok(())
}
```

**Points d'attention :**
- Ne pas modifier l'historique existant
- Créer une nouvelle version après restauration
- Préserver `created_at` du contenu original

---

## 4. Méthodes auxiliaires

### Traduction SPM ↔ Produit

```rust
impl ContentAdapter {
    // SPM → DB
    fn translate_to_db_format(
        &self,
        id: Id,
        input: ContentInput,
        now: SystemTime,
    ) -> Result<DbContent, ContentError> {
        // Traduire vers format DB
        Ok(DbContent {
            id: id.to_string(),
            content_type: input.content_type,
            status: self.status_to_db(input.status.unwrap_or(ContentStatus::Draft)),
            created_at: now,
            updated_at: now,
            metadata: input.metadata, // ou sérialiser selon format DB
        })
    }
    
    // DB → SPM
    fn translate_from_db_format(&self, db: DbContent) -> Result<Content, ContentError> {
        Ok(Content::new(
            self.id_from_string(&db.id)?,
            db.content_type,
            self.status_from_db(db.status)?,
            db.created_at,
            db.updated_at,
            db.metadata, // ou désérialiser
        ))
    }
    
    // Sérialisation pour versioning
    fn serialize_content(&self, content: &Content) -> Result<Vec<u8>, ContentError> {
        // Format défini par le produit (JSON, protobuf, etc.)
        serde_json::to_vec(content)
            .map_err(|e| ContentError::Other(format!("Serialization error: {}", e)))
    }
    
    fn deserialize_content(&self, snapshot: &[u8]) -> Result<Content, ContentError> {
        serde_json::from_slice(snapshot)
            .map_err(|e| ContentError::Other(format!("Deserialization error: {}", e)))
    }
}
```

### Gestion des erreurs

```rust
impl ContentAdapter {
    fn translate_error(&self, error: DbError) -> ContentError {
        match error {
            DbError::NotFound => ContentError::NotFound,
            DbError::ConstraintViolation(msg) => ContentError::ConstraintViolation(msg),
            DbError::InvalidData => ContentError::InvalidInput,
            _ => ContentError::Other(format!("Database error: {}", error)),
        }
    }
    
    fn validate_content_input(&self, input: &ContentInput) -> Result<(), ContentError> {
        if input.content_type.is_empty() {
            return Err(ContentError::Other("content_type cannot be empty".to_string()));
        }
        
        // Validation selon règles produit
        if !self.is_valid_content_type(&input.content_type) {
            return Err(ContentError::Other("invalid content_type".to_string()));
        }
        
        Ok(())
    }
}
```

---

## 5. Points d'attention spécifiques

### Versioning

- **Format de sérialisation :** Choisir un format stable (JSON, protobuf)
- **Taille des snapshots :** Gérer la taille pour les contenus volumineux
- **Rétention :** Définir une politique de rétention des versions

### Relations

- **Cycles :** Détecter et prévenir les cycles dans les relations hiérarchiques
- **Bidirectionnelles :** Gérer les relations bidirectionnelles si nécessaire
- **Cascade :** Définir le comportement de suppression en cascade

### Performance

- **Index DB :** Créer des index sur `content_type`, `status`, `created_at`
- **Pagination :** Utiliser des requêtes efficaces avec LIMIT/OFFSET
- **Cache :** Mettre en cache les contenus fréquemment accédés

### Transactions

- **Cohérence :** Utiliser des transactions pour les opérations multi-étapes
- **Isolation :** Gérer l'isolation selon les besoins (ex. création + relation)

---

## 6. Cas d'usage courants

### Créer un article

```rust
let input = ContentInput::new(
    "article".to_string(),
    serde_json::to_vec(&article_metadata).unwrap(),
);
let id = adapter.create_content(input)?;
```

### Publier un contenu

```rust
let updates = ContentUpdates::new()
    .with_status(ContentStatus::Published);
adapter.update_content(id, updates, true)?; // Créer version
```

### Lister les articles publiés

```rust
let filters = ContentFilters::new()
    .with_content_type("article".to_string())
    .with_status(ContentStatus::Published);
let result = adapter.list_contents(filters, 0, 20)?;
```

### Créer une hiérarchie

```rust
let parent_id = adapter.create_content(/* ... */)?;
let child_id = adapter.create_content(/* ... */)?;
adapter.add_relation(parent_id, "parent".to_string(), child_id)?;
```

---

## 7. Tests recommandés

### Tests unitaires

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_content() {
        let adapter = create_test_adapter();
        let input = ContentInput::new("article".to_string(), b"{}".to_vec());
        
        let id = adapter.create_content(input).unwrap();
        assert!(!id.is_nil());
        
        let content = adapter.get_content(id).unwrap();
        assert_eq!(content.content_type, "article");
        assert_eq!(content.status, ContentStatus::Draft);
    }
    
    #[test]
    fn test_update_content() {
        let adapter = create_test_adapter();
        let id = adapter.create_content(/* ... */).unwrap();
        
        let updates = ContentUpdates::new()
            .with_status(ContentStatus::Published);
        adapter.update_content(id, updates, true).unwrap();
        
        let content = adapter.get_content(id).unwrap();
        assert_eq!(content.status, ContentStatus::Published);
    }
    
    #[test]
    fn test_versioning() {
        let adapter = create_test_adapter();
        let id = adapter.create_content(/* ... */).unwrap();
        
        // Créer version
        let version_id = adapter.create_version(id).unwrap();
        
        // Modifier contenu
        adapter.update_content(id, /* ... */, false).unwrap();
        
        // Restaurer version
        adapter.restore_version(id, version_id).unwrap();
    }
}
```

### Tests d'intégration

```rust
#[test]
fn test_integration_with_database() {
    let db = setup_test_database();
    let adapter = ContentAdapter::new(/* ... */);
    
    // Test création
    let id = adapter.create_content(/* ... */).unwrap();
    
    // Vérifier en DB
    let db_content = db.get_content(id).unwrap();
    assert_eq!(db_content.content_type, "article");
}
```

---

## 8. Références

- **Contrat module :** `docs/spm-cms/modules/content/contrat.md`
- **Implémentation mémoire :** `crates/miyukini-spm-cms-content/src/memory.rs`
- **Guide général :** `docs/spm-cms/Miyukini Framework - Guide Adaptateurs Produits.md`
