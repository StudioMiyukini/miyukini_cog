# Module Recherche & Indexation — SPM CMS (Phase 2.2)

Module Phase 2.2 du SPM CMS. Indexation fonctionnelle et recherche générique d'entités : indexation, mise à jour, recherche par critères.

## Description

Le Module Recherche & Indexation fournit le contrat fonctionnel pour indexer et rechercher des entités (contenus, médias, etc.) dans un CMS. Il expose un trait `SearchManager` que le produit implémente pour adapter vers sa stack technique (DB, moteur de recherche, etc.).

**Principe :** Le module ne connaît pas la persistance, les permissions, ni la logique métier spécifique. Il définit uniquement les opérations fonctionnelles pour indexer des champs d'entités et rechercher par critères simples. Il ne fait pas de scoring, de ranking, ni de full-text linguistique.

## Dépendances

- `miyukini-kernel` : Utilise `Id`

Aucune autre dépendance externe.

## Contrat fonctionnel

Voir [docs/spm-cms/modules/search/contrat.md](../../docs/spm-cms/modules/search/contrat.md) pour le contrat complet.

Le module expose le trait `SearchManager` avec les opérations suivantes :

- **Indexation :** `index_entity` (indexe ou met à jour une entité), `unindex_entity` (désindexe une entité)
- **Recherche :** `search` (recherche par critères avec opérateurs ET/OU), `list_indexed_entities` (liste les entités indexées)
- **Consultation :** `is_indexed` (vérifie si une entité est indexée), `get_indexed_fields` (récupère les champs indexés)
- **Maintenance :** `clear_index` (vide l'index)

## Utilisation

### Implémenter SearchManager

Le produit implémente le trait `SearchManager` pour adapter vers sa stack :

```rust
use miyukini_spm_cms_search::{SearchManager, IndexedField, SearchQuery, SearchError};
use miyukini_kernel::Id;

pub struct MySearchAdapter {
    // Votre index (DB, Elasticsearch, etc.)
}

impl SearchManager for MySearchAdapter {
    fn index_entity(
        &self,
        entity_id: Id,
        entity_type: String,
        fields: Vec<IndexedField>,
    ) -> Result<(), SearchError> {
        // 1. Indexer les champs dans votre moteur de recherche
        // 2. Retourner confirmation
    }
    
    fn search(
        &self,
        query: SearchQuery,
        offset: usize,
        limit: usize,
    ) -> Result<SearchResult, SearchError> {
        // 1. Construire la requête pour votre moteur
        // 2. Exécuter la recherche
        // 3. Appliquer la pagination
        // 4. Retourner les résultats
    }
    
    // ... autres méthodes
}
```

### Utiliser l'implémentation mémoire (tests/démo)

Pour les tests et démos, une implémentation en mémoire est disponible :

```toml
[dependencies]
miyukini-spm-cms-search = { path = "...", features = ["memory"] }
```

```rust
use miyukini_spm_cms_search::{
    SearchManager, IndexedField, SearchQuery, SearchCriterion, SearchOperator,
    LogicalOperator, MemorySearchManager, SearchType,
};
use miyukini_kernel::{IdGenerator, UuidIdGenerator};

let manager = MemorySearchManager::new();
let id_gen = UuidIdGenerator::new();
let entity_id = id_gen.generate();

// Indexer une entité
let fields = vec![
    IndexedField::new("title".to_string(), "Mon article".to_string(), SearchType::Contains),
    IndexedField::new("status".to_string(), "published".to_string(), SearchType::Exact),
];
manager.index_entity(entity_id, "content".to_string(), fields)?;

// Rechercher
let query = SearchQuery::new()
    .with_logical_operator(LogicalOperator::And)
    .add_criterion(SearchCriterion::new(
        "status".to_string(),
        SearchOperator::Equals,
        "published".to_string(),
    ));
let result = manager.search(query, 0, 10)?;
```

## Opérations

- **Indexation :** `index_entity` (indexe ou met à jour), `unindex_entity` (désindexe)
- **Recherche :** `search` (recherche par critères avec ET/OU), `list_indexed_entities` (liste paginée)
- **Consultation :** `is_indexed` (vérifie présence), `get_indexed_fields` (récupère champs)
- **Maintenance :** `clear_index` (vide l'index, opération destructive)

## Types principaux

- `EntityId` : Identifiant d'entité (alias vers `Id` du kernel)
- `EntityType` : Type d'entité (opaque, défini par le produit)
- `IndexedField` : Champ indexé (nom, valeur, type de recherche)
- `SearchQuery` : Requête de recherche (critères, opérateurs logiques)
- `SearchResult` : Résultat de recherche (liste d'IDs, total, pagination)
- `SearchOperator` : Opérateur de recherche (Equals, Contains, GreaterThan, etc.)
- `LogicalOperator` : Opérateur logique (And, Or)
- `SearchType` : Type de recherche pour un champ (Exact, Contains, GreaterThan, etc.)
- `SearchError` : Erreurs possibles (`EntityNotIndexed`, `FieldNotFound`, `InvalidOperator`, `InvalidQuery`, `Other`)

## Invariants

1. **Identité unique :** Une entité ne peut être indexée qu'une seule fois par type d'entité
2. **Cohérence fonctionnelle :** Les champs indexés reflètent l'état de l'entité au moment de l'indexation
3. **Recherche déterministe :** Pour une même requête et un même index, les résultats sont déterministes (ordre non garanti)
4. **Données opaques :** Les noms de champs, valeurs, types de recherche sont stockés sans transformation
5. **Indépendance des entités source :** L'index peut contenir des références vers des entités supprimées (le produit gère la cohérence)

## Hors-scope

- Scoring et ranking (calcul de pertinence)
- Full-text search linguistique (stemming, lemmatisation)
- Recherche sémantique (embeddings, similarité)
- Permissions et accès (filtrage par permissions)
- SEO et référencement
- Synchronisation automatique avec entités source
- Cache de résultats
- Recherche distribuée (réplication, sharding)

## Tests

```bash
cargo test --features memory
```

Les tests utilisent `MemorySearchManager` pour valider le contrat.

## Exemples

Voir les tests dans `tests/search_tests.rs` pour des exemples d'usage.

Exécuter la démo console :

```bash
cargo run --example demo --features memory
```
