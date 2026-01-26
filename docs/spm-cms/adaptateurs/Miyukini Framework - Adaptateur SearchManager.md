# Miyukini Framework - Adaptateur SearchManager

> Guide d'implémentation de l'adaptateur SearchManager pour le Module Recherche SPM CMS.

---

## Contexte

Le Module Recherche permet d'indexer et de rechercher des entités (contenus, médias, etc.) selon des critères multiples. Il ne gère pas la recherche full-text avancée, mais fournit une base pour la recherche structurée.

**Caractéristiques :**
- Indexation d'entités avec champs
- Recherche par critères multiples
- Opérateurs logiques (ET, OU)
- Pagination

---

## 1. Trait à implémenter

### SearchManager

Le trait `SearchManager` expose **7 méthodes** à implémenter :

1. `index_entity()` - Indexer une entité
2. `unindex_entity()` - Désindexer une entité
3. `search()` - Rechercher selon une requête
4. `list_indexed_entities()` - Lister les entités indexées
5. `is_indexed()` - Vérifier si une entité est indexée
6. `get_indexed_fields()` - Obtenir les champs indexés
7. `clear_index()` - Vider l'index

**Module :** `miyukini-spm-cms-search`

**Import :**
```rust
use miyukini_spm_cms_search::{
    SearchManager, SearchError, EntityId, EntityType,
    IndexedField, IndexedEntity, SearchQuery, SearchResult
};
```

---

## 2. Structure de l'adaptateur

### Structure de base

```rust
use miyukini_spm_cms_search::{
    SearchManager, SearchError, EntityId, EntityType,
    IndexedField, IndexedEntity, SearchQuery, SearchResult
};
use miyukini_kernel::{Id, Logger};
use std::sync::Arc;

pub struct SearchAdapter {
    // Dépendances kernel
    logger: Option<Arc<dyn Logger>>,
    
    // Stack technique du produit
    // Exemples :
    // index: SearchIndex, // Elasticsearch, Algolia, DB, etc.
    // repository: SearchRepository,
}

impl SearchAdapter {
    pub fn new(
        logger: Option<Arc<dyn Logger>>,
        // ... autres dépendances produit
    ) -> Self {
        Self {
            logger,
            // ... initialisation
        }
    }
}
```

### Types de données

**Types SPM :**
- `EntityId` : Identifiant d'une entité (alias `Id`)
- `EntityType` : Type d'entité (String)
- `IndexedField` : Champ indexé avec nom, valeur(s), type de recherche
- `IndexedEntity` : Entité indexée avec champs
- `SearchQuery` : Requête de recherche avec critères
- `SearchResult` : Résultat avec entités et pagination
- `SearchError` : Erreurs

---

## 3. Implémentation des méthodes

### 3.1. index_entity()

**Responsabilité :** Indexer une entité (ou mettre à jour si existe).

**Exemple :**
```rust
impl SearchManager for SearchAdapter {
    fn index_entity(
        &self,
        entity_id: EntityId,
        entity_type: EntityType,
        fields: Vec<IndexedField>,
    ) -> Result<(), SearchError> {
        let indexed_entity = IndexedEntity::new(entity_id, entity_type, fields);
        
        // Indexer (remplace si existe)
        self.index.save(indexed_entity)?;
        
        Ok(())
    }
}
```

---

### 3.2. unindex_entity()

**Exemple :**
```rust
fn unindex_entity(
    &self,
    entity_id: EntityId,
    entity_type: EntityType,
) -> Result<(), SearchError> {
    self.index.delete(entity_id, &entity_type)?;
    Ok(())
}
```

---

### 3.3. search()

**Responsabilité :** Rechercher selon une requête avec critères.

**Exemple :**
```rust
fn search(
    &self,
    query: SearchQuery,
    offset: usize,
    limit: usize,
) -> Result<SearchResult, SearchError> {
    // Construire requête selon le moteur de recherche
    let search_query = self.build_search_query(&query)?;
    
    // Exécuter recherche
    let results = self.index.search(&search_query, offset, limit)?;
    
    // Compter total
    let total = self.index.count(&search_query)?;
    
    // Retourner résultat
    Ok(SearchResult::new(
        results.entity_ids,
        total,
        offset,
        limit,
    ))
}
```

**Points d'attention :**
- Traduire les critères SPM vers le format du moteur
- Gérer les opérateurs logiques (ET, OU)
- Appliquer la pagination

---

### 3.4. list_indexed_entities()

**Exemple :**
```rust
fn list_indexed_entities(
    &self,
    entity_type: Option<EntityType>,
    offset: usize,
    limit: usize,
) -> Result<SearchResult, SearchError> {
    let query = if let Some(et) = entity_type {
        SearchQuery::with_entity_type(et)
    } else {
        SearchQuery::new()
    };
    
    self.search(query, offset, limit)
}
```

---

### 3.5. is_indexed()

**Exemple :**
```rust
fn is_indexed(&self, entity_id: EntityId, entity_type: EntityType) -> bool {
    self.index.exists(entity_id, &entity_type).unwrap_or(false)
}
```

---

### 3.6. get_indexed_fields()

**Exemple :**
```rust
fn get_indexed_fields(
    &self,
    entity_id: EntityId,
    entity_type: EntityType,
) -> Result<Vec<IndexedField>, SearchError> {
    let entity = self.index.get(entity_id, &entity_type)
        .map_err(|e| self.translate_error(e))?
        .ok_or(SearchError::NotFound)?;
    
    Ok(entity.fields)
}
```

---

### 3.7. clear_index()

**Exemple :**
```rust
fn clear_index(&self, entity_type: Option<EntityType>) -> Result<(), SearchError> {
    if let Some(et) = entity_type {
        self.index.clear_type(&et)?;
    } else {
        self.index.clear_all()?;
    }
    
    Ok(())
}
```

---

## 4. Points d'attention spécifiques

### Moteur de recherche

- **Choix :** Elasticsearch, Algolia, DB native, etc.
- **Traduction :** Adapter les critères SPM au format du moteur
- **Performance :** Optimiser les requêtes complexes

### Indexation

- **Synchronisation :** Synchroniser l'index avec les données
- **Mise à jour :** Mettre à jour l'index lors des modifications
- **Désindexation :** Désindexer lors des suppressions

---

## 5. Cas d'usage courants

### Indexer une entité

```rust
let fields = vec![
    IndexedField::new("title".to_string(), vec!["Mon article".to_string()], FieldType::Text),
    IndexedField::new("status".to_string(), vec!["published".to_string()], FieldType::Exact),
];
adapter.index_entity(content_id, "content".to_string(), fields)?;
```

### Rechercher des entités

```rust
let query = SearchQuery::new()
    .with_field("title".to_string(), "article".to_string(), FieldType::Text)
    .with_field("status".to_string(), "published".to_string(), FieldType::Exact);
    
let result = adapter.search(query, 0, 20)?;
for entity_id in result.entity_ids {
    println!("Found entity: {:?}", entity_id);
}
```

### Lister les entités indexées

```rust
let result = adapter.list_indexed_entities(
    Some("content".to_string()),
    0,
    50
)?;
```

### Vérifier si une entité est indexée

```rust
if adapter.is_indexed(content_id, "content".to_string()) {
    println!("Content is indexed");
}
```

### Désindexer une entité

```rust
adapter.unindex_entity(content_id, "content".to_string())?;
```

### Vider l'index

```rust
// Vider tout l'index
adapter.clear_index(None)?;

// Vider un type spécifique
adapter.clear_index(Some("content".to_string()))?;
```

---

## 6. Tests recommandés

### Tests unitaires

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_index_entity() {
        let adapter = create_test_adapter();
        let entity_id = generate_test_id();
        let fields = vec![
            IndexedField::new("title".to_string(), vec!["Test".to_string()], FieldType::Text),
        ];
        
        adapter.index_entity(entity_id, "content".to_string(), fields).unwrap();
        
        assert!(adapter.is_indexed(entity_id, "content".to_string()));
    }
    
    #[test]
    fn test_search() {
        let adapter = create_test_adapter();
        let entity_id = generate_test_id();
        let fields = vec![
            IndexedField::new("title".to_string(), vec!["Article".to_string()], FieldType::Text),
        ];
        
        adapter.index_entity(entity_id, "content".to_string(), fields).unwrap();
        
        let query = SearchQuery::new()
            .with_field("title".to_string(), "Article".to_string(), FieldType::Text);
        let result = adapter.search(query, 0, 10).unwrap();
        
        assert!(result.entity_ids.contains(&entity_id));
    }
    
    #[test]
    fn test_unindex_entity() {
        let adapter = create_test_adapter();
        let entity_id = generate_test_id();
        let fields = vec![
            IndexedField::new("title".to_string(), vec!["Test".to_string()], FieldType::Text),
        ];
        
        adapter.index_entity(entity_id, "content".to_string(), fields).unwrap();
        adapter.unindex_entity(entity_id, "content".to_string()).unwrap();
        
        assert!(!adapter.is_indexed(entity_id, "content".to_string()));
    }
    
    #[test]
    fn test_get_indexed_fields() {
        let adapter = create_test_adapter();
        let entity_id = generate_test_id();
        let fields = vec![
            IndexedField::new("title".to_string(), vec!["Test".to_string()], FieldType::Text),
            IndexedField::new("status".to_string(), vec!["published".to_string()], FieldType::Exact),
        ];
        
        adapter.index_entity(entity_id, "content".to_string(), fields.clone()).unwrap();
        
        let retrieved_fields = adapter.get_indexed_fields(entity_id, "content".to_string()).unwrap();
        assert_eq!(retrieved_fields.len(), 2);
    }
    
    #[test]
    fn test_list_indexed_entities() {
        let adapter = create_test_adapter();
        let entity_id1 = generate_test_id();
        let entity_id2 = generate_test_id();
        
        let fields = vec![
            IndexedField::new("title".to_string(), vec!["Test".to_string()], FieldType::Text),
        ];
        
        adapter.index_entity(entity_id1, "content".to_string(), fields.clone()).unwrap();
        adapter.index_entity(entity_id2, "content".to_string(), fields).unwrap();
        
        let result = adapter.list_indexed_entities(Some("content".to_string()), 0, 10).unwrap();
        assert_eq!(result.entity_ids.len(), 2);
    }
    
    #[test]
    fn test_clear_index() {
        let adapter = create_test_adapter();
        let entity_id = generate_test_id();
        let fields = vec![
            IndexedField::new("title".to_string(), vec!["Test".to_string()], FieldType::Text),
        ];
        
        adapter.index_entity(entity_id, "content".to_string(), fields).unwrap();
        adapter.clear_index(Some("content".to_string())).unwrap();
        
        assert!(!adapter.is_indexed(entity_id, "content".to_string()));
    }
}
```

### Tests d'intégration

```rust
#[test]
fn test_integration_with_search_engine() {
    let search_engine = setup_test_search_engine();
    let adapter = SearchAdapter::new(/* ... */);
    
    // Test indexation
    let entity_id = generate_test_id();
    let fields = vec![
        IndexedField::new("title".to_string(), vec!["Test".to_string()], FieldType::Text),
    ];
    adapter.index_entity(entity_id, "content".to_string(), fields).unwrap();
    
    // Vérifier dans le moteur de recherche
    let indexed = search_engine.is_indexed(entity_id, "content").unwrap();
    assert!(indexed);
}
```

---

## 7. Références

- **Contrat module :** `docs/spm-cms/modules/search/contrat.md`
- **Implémentation mémoire :** `crates/miyukini-spm-cms-search/src/memory.rs`
- **Guide général :** `docs/spm-cms/Miyukini Framework - Guide Adaptateurs Produits.md`
