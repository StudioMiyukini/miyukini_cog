# Miyukini Framework - Adaptateur HierarchyManager

> Guide d'implémentation de l'adaptateur HierarchyManager pour le Module Hiérarchie SPM CMS.

---

## Contexte

Le Module Hiérarchie gère la structure arborescente d'entités (contenus, pages, etc.). Il permet de créer des nœuds, de gérer les relations parent-enfant, et de naviguer dans la hiérarchie.

**Caractéristiques :**
- Structure arborescente (chaque nœud a au plus un parent)
- Pas de cycles (arbre acyclique)
- Références bidirectionnelles (parent ↔ children)

---

## 1. Trait à implémenter

### HierarchyManager

Le trait `HierarchyManager` expose **7 méthodes** à implémenter :

1. `create_root()` - Créer un nœud racine
2. `create_child()` - Créer un nœud enfant
3. `parent()` - Obtenir le parent d'un nœud
4. `children()` - Obtenir les enfants directs
5. `ancestors()` - Obtenir tous les ancêtres
6. `path_to_root()` - Obtenir le chemin jusqu'à la racine
7. `move_node()` - Déplacer un nœud
8. `remove_node()` - Supprimer un nœud

**Note :** Le trait utilise `&mut self` pour certaines opérations (création, déplacement, suppression).

**Module :** `miyukini-spm-cms-hierarchy`

**Import :**
```rust
use miyukini_spm_cms_hierarchy::{
    HierarchyManager, HierarchyError, NodeId, EntityId
};
```

---

## 2. Structure de l'adaptateur

### Structure de base

```rust
use miyukini_spm_cms_hierarchy::{
    HierarchyManager, HierarchyError, NodeId, EntityId
};
use miyukini_kernel::{Id, IdGenerator, Logger};
use std::sync::Arc;

pub struct HierarchyAdapter {
    // Dépendances kernel
    id_generator: Arc<dyn IdGenerator>,
    logger: Option<Arc<dyn Logger>>,
    
    // Stack technique du produit
    // Exemples :
    // db: Arc<Database>,
    // repository: HierarchyRepository,
}

impl HierarchyAdapter {
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
- `NodeId` : Identifiant d'un nœud dans la hiérarchie (alias `Id`)
- `EntityId` : Identifiant de l'entité externe référencée (alias `Id`)
- `HierarchyError` : Erreurs (NodeNotFound, CycleDetected, InvalidOperation)

**Structure interne (exemple) :**
```rust
struct Node {
    node_id: NodeId,
    entity_id: EntityId,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}
```

---

## 3. Implémentation des méthodes

### 3.1. create_root()

**Responsabilité :** Créer un nœud racine (sans parent).

**Étapes :**
1. Générer l'ID du nœud via `id_generator.generate()`
2. Créer le nœud avec `parent = None`
3. Persister
4. Retourner le `NodeId`

**Exemple :**
```rust
impl HierarchyManager for HierarchyAdapter {
    fn create_root(&mut self, entity_id: EntityId) -> NodeId {
        // 1. Générer ID
        let node_id = self.id_generator.generate();
        
        // 2. Créer nœud
        let node = Node {
            node_id,
            entity_id,
            parent: None,
            children: Vec::new(),
        };
        
        // 3. Persister
        self.repository.save(node).unwrap();
        
        // 4. Logger
        self.log_operation("create_root", &format!("Created root node {:?}", node_id));
        
        node_id
    }
}
```

**Points d'attention :**
- Un nœud racine n'a pas de parent (`None`)
- L'`entity_id` peut référencer un contenu, une page, etc.

---

### 3.2. create_child()

**Responsabilité :** Créer un nœud enfant sous un parent donné.

**Étapes :**
1. Vérifier que le parent existe
2. Générer l'ID du nœud
3. Vérifier qu'il n'y a pas de cycle
4. Créer le nœud avec `parent = Some(parent_id)`
5. Ajouter le nœud aux enfants du parent
6. Persister

**Exemple :**
```rust
fn create_child(&mut self, parent: NodeId, entity_id: EntityId) -> Result<NodeId, HierarchyError> {
    // 1. Vérifier parent existe
    let parent_node = self.repository.find(parent)
        .map_err(|_| HierarchyError::NodeNotFound)?
        .ok_or(HierarchyError::NodeNotFound)?;
    
    // 2. Générer ID
    let node_id = self.id_generator.generate();
    
    // 3. Vérifier cycle
    if self.would_create_cycle(node_id, Some(parent)) {
        return Err(HierarchyError::CycleDetected);
    }
    
    // 4. Créer nœud
    let mut node = Node {
        node_id,
        entity_id,
        parent: Some(parent),
        children: Vec::new(),
    };
    
    // 5. Ajouter aux enfants du parent
    let mut parent_node = parent_node;
    parent_node.children.push(node_id);
    self.repository.update(parent_node)?;
    
    // 6. Persister
    self.repository.save(node)?;
    
    Ok(node_id)
}
```

**Points d'attention :**
- Détecter les cycles avant création
- Mettre à jour la liste des enfants du parent
- Utiliser une transaction pour garantir la cohérence

---

### 3.3. parent()

**Responsabilité :** Retourner le parent d'un nœud, s'il existe.

**Exemple :**
```rust
fn parent(&self, node: NodeId) -> Option<NodeId> {
    self.repository.find(node)
        .ok()
        .flatten()
        .and_then(|n| n.parent)
}
```

**Points d'attention :**
- Retourner `None` pour un nœud racine
- Gérer le cas où le nœud n'existe pas

---

### 3.4. children()

**Responsabilité :** Retourner la liste des enfants directs d'un nœud.

**Exemple :**
```rust
fn children(&self, node: NodeId) -> Vec<NodeId> {
    self.repository.find(node)
        .ok()
        .flatten()
        .map(|n| n.children)
        .unwrap_or_default()
}
```

**Points d'attention :**
- Retourner un vecteur vide si le nœud n'a pas d'enfants
- Retourner uniquement les enfants directs (pas récursif)

---

### 3.5. ancestors()

**Responsabilité :** Retourner tous les ancêtres d'un nœud (du parent direct jusqu'à la racine).

**Étapes :**
1. Récupérer le parent
2. Remonter récursivement jusqu'à la racine
3. Retourner la liste (du plus proche au plus lointain)

**Exemple :**
```rust
fn ancestors(&self, node: NodeId) -> Vec<NodeId> {
    let mut ancestors = Vec::new();
    let mut current = self.parent(node);
    
    while let Some(parent_id) = current {
        ancestors.push(parent_id);
        current = self.parent(parent_id);
    }
    
    ancestors
}
```

**Points d'attention :**
- L'ordre est du parent direct vers la racine
- Retourner un vecteur vide si le nœud est une racine

---

### 3.6. path_to_root()

**Responsabilité :** Retourner le chemin complet d'un nœud jusqu'à la racine.

**Différence avec `ancestors()` :** Inclut le nœud lui-même en premier.

**Exemple :**
```rust
fn path_to_root(&self, node: NodeId) -> Vec<NodeId> {
    let mut path = vec![node];
    let mut current = self.parent(node);
    
    while let Some(parent_id) = current {
        path.push(parent_id);
        current = self.parent(parent_id);
    }
    
    path
}
```

**Points d'attention :**
- Le premier élément est le nœud lui-même
- Le dernier élément est la racine

---

### 3.7. move_node()

**Responsabilité :** Déplacer un nœud sous un nouveau parent.

**Étapes :**
1. Vérifier que le nœud et le nouveau parent existent
2. Vérifier qu'on ne déplace pas vers lui-même
3. Vérifier qu'il n'y a pas de cycle
4. Retirer le nœud des enfants de l'ancien parent
5. Ajouter le nœud aux enfants du nouveau parent
6. Mettre à jour le parent du nœud

**Exemple :**
```rust
fn move_node(&mut self, node: NodeId, new_parent: NodeId) -> Result<(), HierarchyError> {
    // 1. Vérifier existence
    let node_data = self.repository.find(node)
        .map_err(|_| HierarchyError::NodeNotFound)?
        .ok_or(HierarchyError::NodeNotFound)?;
    
    let new_parent_data = self.repository.find(new_parent)
        .map_err(|_| HierarchyError::NodeNotFound)?
        .ok_or(HierarchyError::NodeNotFound)?;
    
    // 2. Vérifier pas vers lui-même
    if node == new_parent {
        return Err(HierarchyError::InvalidOperation);
    }
    
    // 3. Vérifier cycle
    if self.would_create_cycle(node, Some(new_parent)) {
        return Err(HierarchyError::CycleDetected);
    }
    
    // 4. Retirer de l'ancien parent
    if let Some(old_parent_id) = node_data.parent {
        let mut old_parent = self.repository.find(old_parent_id).unwrap().unwrap();
        old_parent.children.retain(|&child_id| child_id != node);
        self.repository.update(old_parent)?;
    }
    
    // 5. Ajouter au nouveau parent
    let mut new_parent = new_parent_data;
    new_parent.children.push(node);
    self.repository.update(new_parent)?;
    
    // 6. Mettre à jour le nœud
    let mut updated_node = node_data;
    updated_node.parent = Some(new_parent_id);
    self.repository.update(updated_node)?;
    
    Ok(())
}
```

**Points d'attention :**
- Utiliser une transaction pour garantir la cohérence
- Détecter les cycles avant déplacement
- Mettre à jour les trois entités (nœud, ancien parent, nouveau parent)

---

### 3.8. remove_node()

**Responsabilité :** Supprimer un nœud de la hiérarchie.

**Comportement :** Les enfants deviennent des racines (Phase 0).

**Étapes :**
1. Vérifier que le nœud existe
2. Retirer le nœud des enfants de son parent
3. Rendre les enfants orphelins (parent = None)
4. Supprimer le nœud

**Exemple :**
```rust
fn remove_node(&mut self, node: NodeId) -> Result<(), HierarchyError> {
    // 1. Vérifier existence
    let node_data = self.repository.find(node)
        .map_err(|_| HierarchyError::NodeNotFound)?
        .ok_or(HierarchyError::NodeNotFound)?;
    
    let children = node_data.children.clone();
    let parent_id = node_data.parent;
    
    // 2. Retirer du parent
    if let Some(parent_id) = parent_id {
        let mut parent = self.repository.find(parent_id).unwrap().unwrap();
        parent.children.retain(|&child_id| child_id != node);
        self.repository.update(parent)?;
    }
    
    // 3. Rendre enfants orphelins
    for child_id in children {
        let mut child = self.repository.find(child_id).unwrap().unwrap();
        child.parent = None;
        self.repository.update(child)?;
    }
    
    // 4. Supprimer nœud
    self.repository.delete(node)?;
    
    Ok(())
}
```

**Points d'attention :**
- Comportement Phase 0 : enfants deviennent racines
- En production, définir le comportement souhaité (cascade, refus, etc.)
- Utiliser une transaction

---

## 4. Méthodes auxiliaires

### Détection de cycles

```rust
impl HierarchyAdapter {
    /// Vérifie si l'ajout d'un parent créerait un cycle.
    fn would_create_cycle(&self, node: NodeId, proposed_parent: Option<NodeId>) -> bool {
        let Some(parent_id) = proposed_parent else {
            return false; // Pas de parent = pas de cycle
        };
        
        // Parcourir les ancêtres du parent proposé
        // Si on trouve le nœud dans les ancêtres, alors il y aurait un cycle
        let mut current = Some(parent_id);
        while let Some(ancestor_id) = current {
            if ancestor_id == node {
                return true; // Cycle détecté
            }
            current = self.parent(ancestor_id);
        }
        
        false
    }
}
```

### Gestion des erreurs

```rust
impl HierarchyAdapter {
    fn translate_error(&self, error: DbError) -> HierarchyError {
        match error {
            DbError::NotFound => HierarchyError::NodeNotFound,
            DbError::ConstraintViolation => HierarchyError::InvalidOperation,
            _ => HierarchyError::InvalidOperation,
        }
    }
    
    fn log_operation(&self, operation: &str, details: &str) {
        if let Some(logger) = &self.logger {
            logger.info(&format!("[HierarchyAdapter] {}: {}", operation, details));
        }
    }
}
```

---

## 5. Points d'attention spécifiques

### Détection de cycles

- **Critique :** Toujours vérifier les cycles avant création/déplacement
- **Algorithme :** Parcourir les ancêtres du parent proposé
- **Performance :** Optimiser pour les grandes hiérarchies (cache, index)

### Cohérence des données

- **Transactions :** Utiliser des transactions pour les opérations multi-étapes
- **Bidirectionnalité :** Maintenir la cohérence parent ↔ children
- **Intégrité :** Vérifier l'existence des nœuds avant opérations

### Performance

- **Index DB :** Créer des index sur `parent` et `node_id`
- **Requêtes récursives :** Optimiser les requêtes récursives (CTE, closure tables)
- **Cache :** Mettre en cache les relations fréquemment accédées

### Comportement de suppression

- **Phase 0 :** Enfants deviennent racines
- **Production :** Définir le comportement souhaité :
  - Cascade (supprimer les enfants)
  - Refus si enfants existent
  - Orphelinage (enfants deviennent racines)

---

## 6. Cas d'usage courants

### Créer une hiérarchie de pages

```rust
// Créer racine
let root_id = adapter.create_root(home_page_id);

// Créer enfants
let about_id = adapter.create_child(root_id, about_page_id)?;
let contact_id = adapter.create_child(root_id, contact_page_id)?;

// Créer sous-pages
let team_id = adapter.create_child(about_id, team_page_id)?;
```

### Naviguer dans la hiérarchie

```rust
// Obtenir le parent
let parent = adapter.parent(node_id);

// Obtenir les enfants
let children = adapter.children(node_id);

// Obtenir tous les ancêtres
let ancestors = adapter.ancestors(node_id);

// Obtenir le chemin complet
let path = adapter.path_to_root(node_id);
```

### Réorganiser la hiérarchie

```rust
// Déplacer un nœud
adapter.move_node(node_id, new_parent_id)?;

// Supprimer un nœud (enfants deviennent racines)
adapter.remove_node(node_id)?;
```

---

## 7. Tests recommandés

### Tests unitaires

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_root() {
        let mut adapter = create_test_adapter();
        let entity_id = generate_test_id();
        
        let node_id = adapter.create_root(entity_id);
        assert!(!node_id.is_nil());
        
        assert_eq!(adapter.parent(node_id), None);
    }
    
    #[test]
    fn test_create_child() {
        let mut adapter = create_test_adapter();
        let root_id = adapter.create_root(generate_test_id());
        let child_id = adapter.create_child(root_id, generate_test_id()).unwrap();
        
        assert_eq!(adapter.parent(child_id), Some(root_id));
        assert!(adapter.children(root_id).contains(&child_id));
    }
    
    #[test]
    fn test_cycle_detection() {
        let mut adapter = create_test_adapter();
        let root_id = adapter.create_root(generate_test_id());
        let child_id = adapter.create_child(root_id, generate_test_id()).unwrap();
        
        // Essayer de créer un cycle (child devient parent de root)
        let result = adapter.move_node(root_id, child_id);
        assert_eq!(result, Err(HierarchyError::CycleDetected));
    }
    
    #[test]
    fn test_ancestors() {
        let mut adapter = create_test_adapter();
        let root_id = adapter.create_root(generate_test_id());
        let child_id = adapter.create_child(root_id, generate_test_id()).unwrap();
        let grandchild_id = adapter.create_child(child_id, generate_test_id()).unwrap();
        
        let ancestors = adapter.ancestors(grandchild_id);
        assert_eq!(ancestors, vec![child_id, root_id]);
    }
    
    #[test]
    fn test_path_to_root() {
        let mut adapter = create_test_adapter();
        let root_id = adapter.create_root(generate_test_id());
        let child_id = adapter.create_child(root_id, generate_test_id()).unwrap();
        
        let path = adapter.path_to_root(child_id);
        assert_eq!(path, vec![child_id, root_id]);
    }
}
```

---

## 8. Références

- **Contrat module :** Voir `docs/spm-cms/modules/hierarchy/` (si disponible)
- **Implémentation mémoire :** `crates/miyukini-spm-cms-hierarchy/src/memory.rs`
- **Guide général :** `docs/spm-cms/Miyukini Framework - Guide Adaptateurs Produits.md`
