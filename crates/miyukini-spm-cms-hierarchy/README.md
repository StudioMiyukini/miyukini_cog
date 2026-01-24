# Module Hiérarchie — SPM CMS (Phase 0)

Module fondation du SPM CMS. Organisation générique d'entités externes en arbre : création de racines et d'enfants, navigation (parent, children, ancestors, path_to_root), déplacement de nœuds, suppression.

## Description

Le Module Hiérarchie fournit le contrat fonctionnel pour organiser des entités en structure arborescente. Il expose un trait `HierarchyManager` que le produit implémente pour adapter vers sa stack technique (DB, sérialisation, etc.).

**Principe :** Le module ne connaît pas la persistance, les permissions, ni la logique métier spécifique. Il définit uniquement les opérations fonctionnelles pour gérer une hiérarchie d'entités référencées par des IDs opaques.

## Dépendances

- `miyukini-kernel` : Utilise `Id`, `IdGenerator`

Aucune autre dépendance externe.

## Contrat fonctionnel

Le module expose le trait `HierarchyManager` avec les opérations suivantes :

- **Création :** `create_root`, `create_child`
- **Navigation :** `parent`, `children`, `ancestors`, `path_to_root`
- **Modification :** `move_node`
- **Suppression :** `remove_node`

## Utilisation

### Implémenter HierarchyManager

Le produit implémente le trait `HierarchyManager` pour adapter vers sa stack :

```rust
use miyukini_spm_cms_hierarchy::{HierarchyManager, EntityId, HierarchyError};
use miyukini_kernel::Id;

pub struct MyHierarchyAdapter {
    // Votre repository DB, etc.
}

impl HierarchyManager for MyHierarchyAdapter {
    fn create_root(&mut self, entity_id: EntityId) -> NodeId {
        // 1. Générer NodeId via kernel
        // 2. Persister dans votre DB
        // 3. Retourner NodeId
    }
    
    fn create_child(&mut self, parent: NodeId, entity_id: EntityId) -> Result<NodeId, HierarchyError> {
        // 1. Vérifier que le parent existe
        // 2. Vérifier qu'on ne crée pas de cycle
        // 3. Générer NodeId via kernel
        // 4. Persister dans votre DB
        // 5. Retourner NodeId
    }
    
    // ... autres méthodes
}
```

### Utiliser l'implémentation mémoire (tests/démo)

Pour les tests et démos, une implémentation en mémoire est disponible :

```toml
[dependencies]
miyukini-spm-cms-hierarchy = { path = "...", features = ["memory"] }
```

```rust
use miyukini_spm_cms_hierarchy::{HierarchyManager, MemoryHierarchyManager};
use miyukini_kernel::{IdGenerator, UuidIdGenerator};

let mut manager = MemoryHierarchyManager::new();
let id_gen = UuidIdGenerator::new();

let root_id = manager.create_root(id_gen.generate());
let child_id = manager.create_child(root_id, id_gen.generate())?;
let path = manager.path_to_root(child_id);
```

## Opérations

- **Création :** `create_root` (crée un nœud racine), `create_child` (crée un nœud enfant)
- **Navigation :** `parent` (retourne le parent), `children` (liste les enfants directs), `ancestors` (liste tous les ancêtres), `path_to_root` (chemin complet vers la racine)
- **Modification :** `move_node` (déplace un nœud sous un nouveau parent)
- **Suppression :** `remove_node` (supprime un nœud, les enfants deviennent des racines en Phase 0)

## Types principaux

- `NodeId` : Identifiant de nœud dans la hiérarchie (alias vers `Id` du kernel)
- `EntityId` : Identifiant d'entité externe référencée par un nœud (alias vers `Id` du kernel)
- `HierarchyError` : Erreurs possibles (`NodeNotFound`, `CycleDetected`, `InvalidOperation`)

## Invariants

1. **Parent unique :** Chaque nœud a au plus un parent (structure arborescente)
2. **Acyclique :** Pas de cycles (l'arbre reste acyclique)
3. **Cohérence référentielle :** Les références sont cohérentes (parent/children bidirectionnel)
4. **Suppression :** En Phase 0, la suppression d'un nœud transforme ses enfants en racines

## Hors-scope

- Stockage et persistance (DB, fichiers)
- Permissions et accès
- Logique métier spécifique (CMS, SEO, navigation)
- Rendu et affichage
- Gestion des utilisateurs
- Intégrations externes

## Tests

```bash
cargo test --features memory
```

Les tests utilisent `MemoryHierarchyManager` pour valider le contrat.

## Exemples

Voir les tests dans `tests/hierarchy_tests.rs` pour des exemples d'usage.

Exécuter la démo console :

```bash
cargo run --example demo --features memory
```
