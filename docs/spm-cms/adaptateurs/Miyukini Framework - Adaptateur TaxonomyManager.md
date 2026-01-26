# Miyukini Framework - Adaptateur TaxonomyManager

> Guide d'implémentation de l'adaptateur TaxonomyManager pour le Module Taxonomies SPM CMS.

---

## Contexte

Le Module Taxonomies permet de classifier des entités (contenus, pages, etc.) en utilisant des taxonomies et des termes. Une taxonomie contient plusieurs termes, et chaque terme peut être assigné à plusieurs entités.

**Caractéristiques :**
- Taxonomies (ex. "Catégories", "Tags", "Auteurs")
- Termes (ex. "Technologie", "Design", "Marketing")
- Assignations bidirectionnelles (terme ↔ entité)

---

## 1. Trait à implémenter

### TaxonomyManager

Le trait `TaxonomyManager` expose **6 méthodes** à implémenter :

1. `create_taxonomy()` - Créer une taxonomie
2. `add_term()` - Ajouter un terme à une taxonomie
3. `assign_term()` - Assigner un terme à une entité
4. `unassign_term()` - Désassigner un terme d'une entité
5. `terms_for_entity()` - Obtenir les termes d'une entité
6. `entities_for_term()` - Obtenir les entités d'un terme

**Note :** Le trait utilise `&mut self` pour certaines opérations.

**Module :** `miyukini-spm-cms-taxonomies`

**Import :**
```rust
use miyukini_spm_cms_taxonomies::{
    TaxonomyManager, TaxonomyError, TaxonomyId, TermId, EntityId
};
```

---

## 2. Structure de l'adaptateur

### Structure de base

```rust
use miyukini_spm_cms_taxonomies::{
    TaxonomyManager, TaxonomyError, TaxonomyId, TermId, EntityId
};
use miyukini_kernel::{Id, IdGenerator, Logger};
use std::sync::Arc;

pub struct TaxonomyAdapter {
    // Dépendances kernel
    id_generator: Arc<dyn IdGenerator>,
    logger: Option<Arc<dyn Logger>>,
    
    // Stack technique du produit
    // db: Arc<Database>,
    // repository: TaxonomyRepository,
}

impl TaxonomyAdapter {
    pub fn new(
        id_generator: Arc<dyn IdGenerator>,
        logger: Option<Arc<dyn Logger>>,
        // ... autres dépendances produit
    ) -> Self {
        Self {
            id_generator,
            logger,
            // ... initialisation
        }
    }
}
```

### Types de données

**Types SPM :**
- `TaxonomyId` : Identifiant d'une taxonomie (alias `Id`)
- `TermId` : Identifiant d'un terme (alias `Id`)
- `EntityId` : Identifiant d'une entité externe (alias `Id`)
- `TaxonomyError` : Erreurs (TaxonomyNotFound, TermNotFound, InvalidOperation)

**Structure interne (exemple) :**
```rust
struct Taxonomy {
    taxonomy_id: TaxonomyId,
    name: String,
    terms: Vec<TermId>,
}

struct Term {
    term_id: TermId,
    taxonomy_id: TaxonomyId,
    label: String,
}
```

---

## 3. Implémentation des méthodes

### 3.1. create_taxonomy()

**Responsabilité :** Créer une nouvelle taxonomie.

**Exemple :**
```rust
impl TaxonomyManager for TaxonomyAdapter {
    fn create_taxonomy(&mut self, name: String) -> TaxonomyId {
        let taxonomy_id = self.id_generator.generate();
        
        let taxonomy = Taxonomy {
            taxonomy_id,
            name,
            terms: Vec::new(),
        };
        
        self.repository.save_taxonomy(taxonomy)?;
        
        taxonomy_id
    }
}
```

---

### 3.2. add_term()

**Responsabilité :** Ajouter un terme à une taxonomie.

**Exemple :**
```rust
fn add_term(&mut self, taxonomy: TaxonomyId, label: String) -> Result<TermId, TaxonomyError> {
    // Vérifier taxonomie existe
    let mut taxonomy_data = self.repository.find_taxonomy(taxonomy)
        .map_err(|_| TaxonomyError::TaxonomyNotFound)?
        .ok_or(TaxonomyError::TaxonomyNotFound)?;
    
    // Créer terme
    let term_id = self.id_generator.generate();
    let term = Term {
        term_id,
        taxonomy_id: taxonomy,
        label,
    };
    
    // Ajouter à la taxonomie
    taxonomy_data.terms.push(term_id);
    self.repository.update_taxonomy(taxonomy_data)?;
    self.repository.save_term(term)?;
    
    Ok(term_id)
}
```

---

### 3.3. assign_term()

**Responsabilité :** Assigner un terme à une entité.

**Exemple :**
```rust
fn assign_term(&mut self, term: TermId, entity: EntityId) -> Result<(), TaxonomyError> {
    // Vérifier terme existe
    self.repository.find_term(term)
        .map_err(|_| TaxonomyError::TermNotFound)?
        .ok_or(TaxonomyError::TermNotFound)?;
    
    // Assigner (idempotent)
    self.repository.save_assignment(term, entity)?;
    
    Ok(())
}
```

**Points d'attention :**
- Opération idempotente (pas d'erreur si déjà assigné)
- Maintenir la cohérence bidirectionnelle

---

### 3.4. unassign_term()

**Responsabilité :** Désassigner un terme d'une entité.

**Exemple :**
```rust
fn unassign_term(&mut self, term: TermId, entity: EntityId) -> Result<(), TaxonomyError> {
    // Vérifier terme existe
    self.repository.find_term(term)
        .map_err(|_| TaxonomyError::TermNotFound)?
        .ok_or(TaxonomyError::TermNotFound)?;
    
    // Désassigner (idempotent)
    self.repository.delete_assignment(term, entity)?;
    
    Ok(())
}
```

---

### 3.5. terms_for_entity()

**Responsabilité :** Retourner les termes assignés à une entité.

**Exemple :**
```rust
fn terms_for_entity(&self, entity: EntityId) -> Vec<TermId> {
    self.repository.find_terms_by_entity(entity)
        .unwrap_or_default()
}
```

---

### 3.6. entities_for_term()

**Responsabilité :** Retourner les entités assignées à un terme.

**Exemple :**
```rust
fn entities_for_term(&self, term: TermId) -> Vec<EntityId> {
    self.repository.find_entities_by_term(term)
        .unwrap_or_default()
}
```

---

## 4. Points d'attention spécifiques

### Assignations bidirectionnelles

- **Cohérence :** Maintenir la cohérence terme ↔ entité
- **Performance :** Utiliser des index sur les deux directions
- **Transactions :** Utiliser des transactions pour garantir la cohérence

### Suppression en cascade

- **Taxonomie :** Supprimer tous les termes et assignations
- **Terme :** Supprimer toutes les assignations
- **Entité :** Gérer selon les règles du produit

---

## 5. Cas d'usage courants

### Créer une taxonomie de catégories

```rust
let categories_id = adapter.create_taxonomy("Catégories".to_string());
let tech_id = adapter.add_term(categories_id, "Technologie".to_string())?;
let design_id = adapter.add_term(categories_id, "Design".to_string())?;
```

### Classifier un contenu

```rust
adapter.assign_term(tech_id, content_id)?;
adapter.assign_term(design_id, content_id)?;
```

### Rechercher par terme

```rust
let contents = adapter.entities_for_term(tech_id);
```

---

## 6. Tests recommandés

### Tests unitaires

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_taxonomy() {
        let mut adapter = create_test_adapter();
        let taxonomy_id = adapter.create_taxonomy("Catégories".to_string());
        assert!(!taxonomy_id.is_nil());
    }
    
    #[test]
    fn test_add_term() {
        let mut adapter = create_test_adapter();
        let taxonomy_id = adapter.create_taxonomy("Catégories".to_string());
        let term_id = adapter.add_term(taxonomy_id, "Technologie".to_string()).unwrap();
        assert!(!term_id.is_nil());
    }
    
    #[test]
    fn test_assign_term() {
        let mut adapter = create_test_adapter();
        let taxonomy_id = adapter.create_taxonomy("Catégories".to_string());
        let term_id = adapter.add_term(taxonomy_id, "Technologie".to_string()).unwrap();
        let entity_id = generate_test_id();
        
        adapter.assign_term(term_id, entity_id).unwrap();
        
        let terms = adapter.terms_for_entity(entity_id);
        assert!(terms.contains(&term_id));
    }
    
    #[test]
    fn test_entities_for_term() {
        let mut adapter = create_test_adapter();
        let taxonomy_id = adapter.create_taxonomy("Catégories".to_string());
        let term_id = adapter.add_term(taxonomy_id, "Technologie".to_string()).unwrap();
        let entity_id1 = generate_test_id();
        let entity_id2 = generate_test_id();
        
        adapter.assign_term(term_id, entity_id1).unwrap();
        adapter.assign_term(term_id, entity_id2).unwrap();
        
        let entities = adapter.entities_for_term(term_id);
        assert_eq!(entities.len(), 2);
        assert!(entities.contains(&entity_id1));
        assert!(entities.contains(&entity_id2));
    }
    
    #[test]
    fn test_unassign_term() {
        let mut adapter = create_test_adapter();
        let taxonomy_id = adapter.create_taxonomy("Catégories".to_string());
        let term_id = adapter.add_term(taxonomy_id, "Technologie".to_string()).unwrap();
        let entity_id = generate_test_id();
        
        adapter.assign_term(term_id, entity_id).unwrap();
        adapter.unassign_term(term_id, entity_id).unwrap();
        
        let terms = adapter.terms_for_entity(entity_id);
        assert!(!terms.contains(&term_id));
    }
}
```

### Tests d'intégration

```rust
#[test]
fn test_integration_with_database() {
    let db = setup_test_database();
    let mut adapter = TaxonomyAdapter::new(/* ... */);
    
    // Test création taxonomie
    let taxonomy_id = adapter.create_taxonomy("Catégories".to_string());
    
    // Vérifier en DB
    let db_taxonomy = db.get_taxonomy(taxonomy_id).unwrap();
    assert_eq!(db_taxonomy.name, "Catégories");
}
```

---

## 7. Références

- **Implémentation mémoire :** `crates/miyukini-spm-cms-taxonomies/src/memory.rs`
- **Guide général :** `docs/spm-cms/Miyukini Framework - Guide Adaptateurs Produits.md`
