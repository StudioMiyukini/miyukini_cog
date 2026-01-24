# Module Contenu — SPM CMS

Module fondation du SPM CMS. Gestion des entités de contenu (pages, articles, blocs) : CRUD, statuts (brouillon/publié/archivé), relations, versioning, métadonnées.

## Description

Le Module Contenu fournit le contrat fonctionnel pour gérer les contenus dans un CMS. Il expose un trait `ContentManager` que le produit implémente pour adapter vers sa stack technique (DB, sérialisation, etc.).

**Principe :** Le module ne connaît pas la persistance, les permissions, ni la logique métier spécifique. Il définit uniquement les opérations fonctionnelles.

## Dépendances

- `miyukini-kernel` : Utilise `Id`, `Clock`, `Logger` (optionnel)

Aucune autre dépendance externe.

## Contrat fonctionnel

Voir [docs/spm-cms/modules/content/contrat.md](../../docs/spm-cms/modules/content/contrat.md) pour le contrat complet.

## Utilisation

### Implémenter ContentManager

Le produit implémente le trait `ContentManager` pour adapter vers sa stack :

```rust
use miyukini_spm_cms_content::{ContentManager, ContentInput, ContentError};
use miyukini_kernel::Id;

pub struct MyContentAdapter {
    // Votre repository DB, etc.
}

impl ContentManager for MyContentAdapter {
    fn create_content(&self, input: ContentInput) -> Result<Id, ContentError> {
        // 1. Générer ID via kernel
        // 2. Obtenir horodatage via kernel
        // 3. Persister dans votre DB
        // 4. Retourner ID
    }
    
    // ... autres méthodes
}
```

### Utiliser l'implémentation mémoire (tests/démo)

Pour les tests et démos, une implémentation en mémoire est disponible :

```toml
[dependencies]
miyukini-spm-cms-content = { path = "...", features = ["memory"] }
```

```rust
use miyukini_spm_cms_content::{ContentManager, ContentInput, MemoryContentManager};

let manager = MemoryContentManager::new();
let input = ContentInput::new("article".to_string(), b"metadata".to_vec());
let id = manager.create_content(input)?;
let content = manager.get_content(id)?;
```

## Opérations

- **CRUD :** `create_content`, `get_content`, `update_content`, `delete_content`
- **Liste :** `list_contents` (avec filtres et pagination)
- **Relations :** `add_relation`, `remove_relation`, `list_relations`
- **Versioning :** `create_version`, `get_version`, `list_versions`, `restore_version`

## Types principaux

- `Content` : Entité contenu
- `ContentStatus` : Statuts (Draft, Published, Archived)
- `ContentInput` : Données pour créer un contenu
- `ContentUpdates` : Mises à jour partielles
- `ContentFilters` : Filtres pour la liste
- `ContentRelation` : Relation entre contenus
- `ContentVersion` : Version d'un contenu

## Invariants

1. Identité unique : un contenu a un identifiant unique et immuable
2. Dates cohérentes : `updated_at >= created_at`
3. Statuts valides : seuls Draft, Published, Archived sont autorisés
4. Relations acycliques : validation par le produit si nécessaire
5. Versioning cohérent : si activé, chaque modification peut créer une version
6. Intégrité référentielle : relations pointent vers contenus existants

## Hors-scope

- Stockage et persistance (DB, fichiers)
- Permissions et accès
- Rendu et affichage
- Recherche full-text (Module Recherche)
- Workflow métier
- SEO et référencement
- Intégrations externes

## Tests

```bash
cargo test --features memory
```

Les tests utilisent `MemoryContentManager` pour valider le contrat.

## Exemples

Voir les tests dans `tests/content_tests.rs` pour des exemples d'usage.
