# Module Médias — SPM CMS (Phase 1)

Module Phase 1 du SPM CMS. Gestion générique des médias (assets) et de leurs métadonnées : création, lecture, suppression, et liens avec des entités externes.

## Description

Le Module Médias fournit le contrat fonctionnel pour gérer les assets (images, vidéos, fichiers) dans un CMS. Il expose un trait `MediaManager` que le produit implémente pour adapter vers sa stack technique (DB, stockage fichiers, etc.).

**Principe :** Le module ne connaît pas la persistance, les permissions, ni la logique métier spécifique. Il définit uniquement les opérations fonctionnelles pour gérer les médias et leurs liens avec des entités.

## Dépendances

- `miyukini-kernel` : Utilise `Id`, `IdGenerator`, `Clock`

Aucune autre dépendance externe.

## Contrat fonctionnel

Le module expose le trait `MediaManager` avec les opérations suivantes :

- **CRUD :** `create_media`, `get_media`, `delete_media`
- **Liens entités :** `attach_media_to_entity`, `detach_media_from_entity`, `list_media_for_entity`

## Utilisation

### Implémenter MediaManager

Le produit implémente le trait `MediaManager` pour adapter vers sa stack :

```rust
use miyukini_spm_cms_media::{MediaManager, MediaInput, MediaError};
use miyukini_kernel::Id;

pub struct MyMediaAdapter {
    // Votre repository DB, stockage fichiers, etc.
}

impl MediaManager for MyMediaAdapter {
    fn create_media(&self, input: MediaInput) -> Result<Id, MediaError> {
        // 1. Générer ID via kernel
        // 2. Obtenir horodatage via kernel
        // 3. Persister dans votre DB
        // 4. Stocker le fichier (si nécessaire)
        // 5. Retourner ID
    }
    
    fn get_media(&self, id: Id) -> Result<Media, MediaError> {
        // 1. Récupérer depuis votre DB
        // 2. Retourner Media
    }
    
    // ... autres méthodes
}
```

### Utiliser l'implémentation mémoire (tests/démo)

Pour les tests et démos, une implémentation en mémoire est disponible :

```toml
[dependencies]
miyukini-spm-cms-media = { path = "...", features = ["memory"] }
```

```rust
use miyukini_spm_cms_media::{MediaManager, MediaInput, MemoryMediaManager};
use miyukini_kernel::{IdGenerator, UuidIdGenerator};

let manager = MemoryMediaManager::new();
let input = MediaInput::new("image/jpeg".to_string(), b"metadata".to_vec());
let id = manager.create_media(input)?;
let media = manager.get_media(id)?;

// Attacher à une entité
let entity_id = UuidIdGenerator::new().generate();
manager.attach_media_to_entity(id, entity_id)?;

// Lister les médias d'une entité
let media_list = manager.list_media_for_entity(entity_id)?;
```

## Opérations

- **CRUD :** `create_media` (crée un média), `get_media` (lit un média), `delete_media` (supprime un média)
- **Liens entités :** `attach_media_to_entity` (attache un média à une entité), `detach_media_from_entity` (détache un média d'une entité), `list_media_for_entity` (liste tous les médias d'une entité)

## Types principaux

- `Media` : Entité média avec type MIME et métadonnées
- `MediaId` : Identifiant de média (alias vers `Id` du kernel)
- `EntityId` : Identifiant d'entité externe (alias vers `Id` du kernel)
- `MediaInput` : Données pour créer un média
- `MediaError` : Erreurs possibles (`NotFound`, `EntityNotFound`, `ConstraintViolation`, `Other`)

## Invariants

1. **Identité unique :** Un média a un identifiant unique et immuable
2. **Dates cohérentes :** `updated_at >= created_at`
3. **Liens bidirectionnels :** Les liens média ↔ entité sont cohérents (un média peut être attaché à plusieurs entités, une entité peut avoir plusieurs médias)
4. **Suppression en cascade :** La suppression d'un média supprime automatiquement tous ses liens avec les entités

## Hors-scope

- Upload HTTP (gestion des fichiers uploadés)
- Transformation d'image (redimensionnement, compression)
- CDN et distribution
- Rendu et affichage
- Permissions et accès
- Stockage physique des fichiers (le produit gère le stockage)
- Recherche et indexation
- Intégrations externes (S3, Cloudinary, etc.)

## Tests

```bash
cargo test --features memory
```

Les tests utilisent `MemoryMediaManager` pour valider le contrat.

## Exemples

Voir les tests dans `tests/media_tests.rs` pour des exemples d'usage.

Exécuter la démo console :

```bash
cargo run --example demo --features memory
```
