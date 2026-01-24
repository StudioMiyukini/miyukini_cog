# Module Taxonomies — SPM CMS (Phase 0)

Module fondation du SPM CMS. Système de classification générique d'entités : création de taxonomies, ajout de termes, assignation/désassignation de termes à des entités, recherche bidirectionnelle.

## Description

Le Module Taxonomies fournit le contrat fonctionnel pour classifier des entités en utilisant des taxonomies et des termes. Il expose un trait `TaxonomyManager` que le produit implémente pour adapter vers sa stack technique (DB, sérialisation, etc.).

**Principe :** Le module ne connaît pas la persistance, les permissions, ni la logique métier spécifique. Il définit uniquement les opérations fonctionnelles pour gérer un système de classification générique d'entités référencées par des IDs opaques.

## Dépendances

- `miyukini-kernel` : Utilise `Id`, `IdGenerator`

Aucune autre dépendance externe.

## Contrat fonctionnel

Le module expose le trait `TaxonomyManager` avec les opérations suivantes :

- **Création :** `create_taxonomy` (crée une taxonomie), `add_term` (ajoute un terme à une taxonomie)
- **Assignation :** `assign_term` (assigne un terme à une entité), `unassign_term` (désassigne un terme d'une entité)
- **Recherche :** `terms_for_entity` (liste les termes d'une entité), `entities_for_term` (liste les entités d'un terme)

## Utilisation

### Implémenter TaxonomyManager

Le produit implémente le trait `TaxonomyManager` pour adapter vers sa stack :

```rust
use miyukini_spm_cms_taxonomies::{TaxonomyManager, EntityId, TaxonomyError};
use miyukini_kernel::Id;

pub struct MyTaxonomyAdapter {
    // Votre repository DB, etc.
}

impl TaxonomyManager for MyTaxonomyAdapter {
    fn create_taxonomy(&mut self, name: String) -> TaxonomyId {
        // 1. Générer TaxonomyId via kernel
        // 2. Persister dans votre DB
        // 3. Retourner TaxonomyId
    }
    
    fn add_term(&mut self, taxonomy: TaxonomyId, label: String) -> Result<TermId, TaxonomyError> {
        // 1. Vérifier que la taxonomie existe
        // 2. Générer TermId via kernel
        // 3. Persister dans votre DB
        // 4. Retourner TermId
    }
    
    // ... autres méthodes
}
```

### Utiliser l'implémentation mémoire (tests/démo)

Pour les tests et démos, une implémentation en mémoire est disponible :

```toml
[dependencies]
miyukini-spm-cms-taxonomies = { path = "...", features = ["memory"] }
```

```rust
use miyukini_spm_cms_taxonomies::{TaxonomyManager, MemoryTaxonomyManager};
use miyukini_kernel::{IdGenerator, UuidIdGenerator};

let mut manager = MemoryTaxonomyManager::new();
let id_gen = UuidIdGenerator::new();

let taxonomy_id = manager.create_taxonomy("Tags".to_string());
let term_id = manager.add_term(taxonomy_id, "News".to_string())?;
let entity_id = id_gen.generate();
manager.assign_term(term_id, entity_id)?;
let terms = manager.terms_for_entity(entity_id);
```

## Opérations

- **Création :** `create_taxonomy` (crée une taxonomie), `add_term` (ajoute un terme à une taxonomie)
- **Assignation :** `assign_term` (assigne un terme à une entité, idempotent), `unassign_term` (désassigne un terme d'une entité, idempotent)
- **Recherche :** `terms_for_entity` (retourne les termes assignés à une entité), `entities_for_term` (retourne les entités assignées à un terme)

## Types principaux

- `TaxonomyId` : Identifiant de taxonomie (alias vers `Id` du kernel)
- `TermId` : Identifiant de terme (alias vers `Id` du kernel)
- `EntityId` : Identifiant d'entité externe référencée (alias vers `Id` du kernel)
- `TaxonomyError` : Erreurs possibles (`TaxonomyNotFound`, `TermNotFound`, `InvalidOperation`)

## Invariants

1. **Appartenance :** Chaque terme appartient à une taxonomie
2. **Bidirectionnalité :** Les assignations sont bidirectionnelles (terme ↔ entité)
3. **Idempotence :** Assigner/désassigner plusieurs fois le même terme à la même entité est idempotent
4. **Suppression en cascade :** La suppression d'une taxonomie supprime ses termes et assignations (implémentation mémoire)

## Hors-scope

- Stockage et persistance (DB, fichiers)
- Permissions et accès
- Logique métier spécifique (SEO, navigation, sémantique)
- Rendu et affichage
- Gestion des utilisateurs
- Hiérarchie entre termes (Module Hiérarchie)
- Intégrations externes

## Tests

```bash
cargo test --features memory
```

Les tests utilisent `MemoryTaxonomyManager` pour valider le contrat.

## Exemples

Voir les tests dans `tests/taxonomy_tests.rs` pour des exemples d'usage.

Exécuter la démo console :

```bash
cargo run --example demo --features memory
```
