# Module Publication — SPM CMS (Phase 1)

Module Phase 1 du SPM CMS. Gestion du cycle éditorial générique des entités de contenu : statuts (draft, scheduled, published, archived), dates de publication, publication différée (scheduling).

## Description

Le Module Publication fournit le contrat fonctionnel pour gérer le cycle de vie éditorial des contenus dans un CMS. Il expose un trait `PublicationManager` que le produit implémente pour adapter vers sa stack technique (DB, sérialisation, etc.).

**Principe :** Le module ne connaît pas la persistance, les permissions, ni la logique métier spécifique. Il définit uniquement les opérations fonctionnelles pour gérer les statuts de publication et les dates.

## Dépendances

- `miyukini-kernel` : Utilise `Id`, `IdGenerator`, `Clock`

Aucune autre dépendance externe.

## Contrat fonctionnel

Le module expose le trait `PublicationManager` avec les opérations suivantes :

- **Création :** `create_publication` (crée une publication en état Draft)
- **Statut :** `status` (lit le statut actuel), `effective_status` (statut effectif avec prise en compte du temps)
- **Publication :** `publish_now` (publie immédiatement), `schedule` (programme une publication future)
- **Archivage :** `archive` (archive une publication publiée)

## Utilisation

### Implémenter PublicationManager

Le produit implémente le trait `PublicationManager` pour adapter vers sa stack :

```rust
use miyukini_spm_cms_publication::{PublicationManager, PublicationError};
use miyukini_kernel::Id;

pub struct MyPublicationAdapter {
    // Votre repository DB, etc.
}

impl PublicationManager for MyPublicationAdapter {
    fn create_publication(&self, content_id: Id) -> Result<Id, PublicationError> {
        // 1. Générer ID via kernel
        // 2. Obtenir horodatage via kernel
        // 3. Créer en état Draft
        // 4. Persister dans votre DB
        // 5. Retourner ID
    }
    
    fn publish_now(&self, publication_id: Id) -> Result<(), PublicationError> {
        // 1. Vérifier les transitions autorisées
        // 2. Mettre à jour le statut
        // 3. Persister
    }
    
    // ... autres méthodes
}
```

### Utiliser l'implémentation mémoire (tests/démo)

Pour les tests et démos, une implémentation en mémoire est disponible :

```toml
[dependencies]
miyukini-spm-cms-publication = { path = "...", features = ["memory"] }
```

```rust
use miyukini_spm_cms_publication::{PublicationManager, MemoryPublicationManager};
use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use std::time::{Duration, SystemTime};

let manager = MemoryPublicationManager::new();
let content_id = UuidIdGenerator::new().generate();

// Créer une publication
let publication_id = manager.create_publication(content_id)?;

// Programmer une publication
let future = SystemTime::now() + Duration::from_secs(3600);
manager.schedule(publication_id, future)?;

// Publier immédiatement
manager.publish_now(publication_id)?;

// Archiver
manager.archive(publication_id)?;
```

## Opérations

- **Création :** `create_publication` (crée une publication en état Draft)
- **Statut :** `status` (lit le statut actuel), `effective_status` (statut effectif, prend en compte le temps pour les publications Scheduled)
- **Publication :** `publish_now` (publie immédiatement depuis Draft ou Scheduled), `schedule` (programme une publication future, Draft → Scheduled)
- **Archivage :** `archive` (archive une publication publiée, Published → Archived)

## Types principaux

- `Publication` : Entité publication avec statut et date de publication
- `PublicationId` : Identifiant de publication (alias vers `Id` du kernel)
- `ContentId` : Identifiant de contenu (alias vers `Id` du kernel)
- `PublicationStatus` : Statuts possibles (`Draft`, `Scheduled`, `Published`, `Archived`)
- `PublicationError` : Erreurs possibles (`NotFound`, `InvalidTransition`, `InvalidSchedule`, `Other`)

## Invariants

1. **État initial :** Une publication commence toujours en état Draft
2. **Transitions autorisées :**
   - Draft → Scheduled (via `schedule`)
   - Draft → Published (via `publish_now`)
   - Scheduled → Published (via `publish_now`, seulement si maintenant >= publish_at)
   - Published → Archived (via `archive`)
3. **Transitions interdites :** Toute autre transition retourne `InvalidTransition`
4. **Date de publication :** `schedule` avec une date passée retourne `InvalidSchedule`
5. **Statut effectif :** `effective_status` retourne Published si une publication Scheduled a dépassé sa date de publication
6. **État final :** Une publication archivée ne peut plus changer d'état

## Hors-scope

- Workflows métier (validation, approbation)
- Permissions et accès
- Rendu et affichage
- SEO et référencement
- Notifications
- Jobs de publication automatique (le produit implémente les jobs qui appellent `publish_now`)
- Logique métier spécifique (règles de publication par type de contenu, etc.)

## Tests

```bash
cargo test --features memory
```

Les tests utilisent `MemoryPublicationManager` pour valider le contrat.

## Exemples

Voir les tests dans `tests/publication_tests.rs` pour des exemples d'usage.

Exécuter la démo console :

```bash
cargo run --example demo --features memory
```
