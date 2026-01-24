---
name: Implémentation Module Contenu SPM
overview: "Implémenter le Module Contenu du SPM CMS selon le contrat fonctionnel : crate Rust avec traits/types publics, opérations CRUD, statuts, relations, versioning, sans persistance ni logique métier, dépendant uniquement du kernel."
todos:
  - id: create-crate-structure
    content: Créer la crate miyukini-spm-cms-content avec structure de base (Cargo.toml, src/lib.rs, modules)
    status: completed
  - id: implement-base-types
    content: Implémenter types de base (ContentStatus, ContentError, Content, ContentInput, ContentUpdates)
    status: completed
  - id: implement-trait
    content: Implémenter trait ContentManager avec toutes les opérations du contrat
    status: completed
  - id: implement-relations
    content: Implémenter types et opérations de relations (ContentRelation, add_relation, remove_relation, list_relations)
    status: completed
  - id: implement-versioning
    content: Implémenter types et opérations de versioning (ContentVersion, create_version, restore_version, list_versions)
    status: completed
  - id: implement-memory-adapter
    content: Implémenter MemoryContentManager (en mémoire) pour tests et démo
    status: completed
  - id: write-tests
    content: Écrire tests unitaires pour toutes les opérations et invariants
    status: completed
  - id: write-documentation
    content: Écrire documentation (README, doc comments, exemples)
    status: completed
isProject: false
---

# Plan : Implémentation Module Contenu (SPM)

## 1. Structure de la crate

**Crate :** `crates/miyukini-spm-cms-content`

**Structure :**

```
crates/miyukini-spm-cms-content/
├── Cargo.toml
└── src/
    ├── lib.rs              # Point d'entrée, ré-exports
    ├── content.rs          # Types et trait ContentManager
    ├── status.rs           # Statuts (brouillon, publié, archivé)
    ├── relation.rs         # Types de relations
    ├── version.rs          # Versioning (optionnel)
    └── error.rs            # Types d'erreur
```

**Dépendances (Cargo.toml) :**

- `miyukini-kernel` (path vers kernel)
- Aucune autre dépendance externe

---

## 2. Types et entités

### Types de base

**Fichier :** `src/content.rs`

**Types à définir :**

- `ContentId` : Alias vers `Id` du kernel (pas de nouveau type)
- `Content` : Structure représentant un contenu
  - `id: Id`
  - `content_type: String` (défini par le produit)
  - `status: ContentStatus`
  - `created_at: SystemTime` (via Clock)
  - `updated_at: SystemTime` (via Clock)
  - `metadata: ContentMetadata` (type opaque ou générique défini par le produit)
- `ContentInput` : Structure pour créer un contenu
- `ContentUpdates` : Structure pour modifier un contenu (partielle)
- `ContentFilters` : Structure pour filtrer les contenus (définie par le produit)
- `ContentListResult` : Résultat de liste avec pagination

**Fichier :** `src/status.rs`

**Types :**

- `ContentStatus` : Enum (Draft, Published, Archived)
- Méthodes de conversion et validation

**Fichier :** `src/relation.rs`

**Types :**

- `RelationType` : String ou enum (défini par le produit, mais type générique)
- `ContentRelation` : Structure (source_id, relation_type, target_id)
- `RelationInput` : Structure pour créer une relation

**Fichier :** `src/version.rs`

**Types :**

- `VersionId` : Alias vers `Id`
- `ContentVersion` : Structure (version_id, content_id, snapshot, created_at)
- `VersionInput` : Structure pour créer une version

**Fichier :** `src/error.rs`

**Types :**

- `ContentError` : Enum d'erreurs (NotFound, InvalidStatus, InvalidRelation, etc.)

---

## 3. Trait ContentManager

**Fichier :** `src/content.rs`

**Trait :**

```rust
pub trait ContentManager {
    // CRUD de base
    fn create_content(&self, input: ContentInput) -> Result<Id, ContentError>;
    fn get_content(&self, id: Id) -> Result<Content, ContentError>;
    fn update_content(&self, id: Id, updates: ContentUpdates) -> Result<(), ContentError>;
    fn delete_content(&self, id: Id, soft: bool) -> Result<(), ContentError>;
    
    // Liste
    fn list_contents(&self, filters: ContentFilters) -> Result<ContentListResult, ContentError>;
    
    // Relations
    fn add_relation(&self, source_id: Id, relation_type: String, target_id: Id) -> Result<(), ContentError>;
    fn remove_relation(&self, source_id: Id, relation_type: String, target_id: Id) -> Result<(), ContentError>;
    fn list_relations(&self, content_id: Id) -> Result<Vec<ContentRelation>, ContentError>;
    
    // Versioning (optionnel)
    fn create_version(&self, content_id: Id) -> Result<Id, ContentError>;
    fn get_version(&self, content_id: Id, version_id: Id) -> Result<ContentVersion, ContentError>;
    fn list_versions(&self, content_id: Id) -> Result<Vec<ContentVersion>, ContentError>;
    fn restore_version(&self, content_id: Id, version_id: Id) -> Result<(), ContentError>;
}
```

**Contraintes :**

- Le trait ne connaît pas la persistance
- Le trait utilise `Id` et `SystemTime` du kernel
- Les types de contenu, métadonnées, filtres sont définis par le produit (types génériques ou opaques)

---

## 4. Implémentation minimale (pour tests/démo)

**Fichier :** `src/memory.rs` (optionnel, pour démo)

**Structure :**

- `MemoryContentManager` : Implémentation en mémoire du trait `ContentManager`
- Stockage dans `HashMap<Id, Content>`
- Utilise `UuidIdGenerator` et `DefaultClock` du kernel
- Pas de persistance réelle

**Usage :** Uniquement pour démo/tests. Le produit fournira sa propre implémentation.

---

## 5. Tests

**Fichier :** `src/lib.rs` ou `tests/`

**Tests à implémenter :**

- Création de contenu (ID généré, dates, statut)
- Lecture de contenu (existant, inexistant)
- Modification de contenu (dates mises à jour)
- Suppression (douce vs définitive)
- Liste avec filtres
- Relations (création, suppression, liste)
- Versioning (création, restauration, liste)
- Invariants (dates cohérentes, statuts valides)

**Tests d'intégration :** Utilisation du kernel (Id, Clock) dans les tests

---

## 6. Documentation

**Fichier :** `src/lib.rs`

**Documentation :**

- Description du module
- Exemples d'usage
- Lien vers le contrat fonctionnel

**Fichier :** `README.md` (à créer dans la crate)

**Contenu :**

- Description
- Dépendances
- Guide d'utilisation
- Exemples

---

## 7. Intégration workspace

**Fichier :** `Cargo.toml` (racine)

**Modification :**

```toml
[workspace]
resolver = "2"
members = [
    "crates/miyukini-kernel",
    "crates/demo-logging-lifecycle",
    "crates/miyukini-spm-cms-content"  # Nouveau
]
```

---

## 8. Règles strictes

**Ne PAS ajouter :**

- Persistance réelle (DB, fichiers)
- Validation métier spécifique
- Permissions ou accès
- Logique de rendu
- Dépendances externes (sauf kernel)

**Respecter :**

- Contrat fonctionnel à la lettre
- Types opaques pour métadonnées (produit définit la structure)
- Types génériques pour filtres (produit définit les critères)
- Utilisation exclusive du kernel (Id, Clock, Logger optionnel)

---

## 9. Ordre d'implémentation

1. **Structure de base :** Créer la crate, Cargo.toml, structure de fichiers
2. **Types de base :** ContentStatus, ContentError, types de base
3. **Types Content :** Content, ContentInput, ContentUpdates
4. **Trait ContentManager :** Définition du trait avec toutes les méthodes
5. **Types Relations :** ContentRelation, RelationInput
6. **Types Versioning :** ContentVersion, VersionInput
7. **Implémentation mémoire :** MemoryContentManager (pour tests/démo)
8. **Tests :** Tests unitaires pour chaque opération
9. **Documentation :** README et doc comments

---

## 10. Critères de validation

**Fonctionnel :**

- Toutes les opérations du contrat sont implémentées
- Les invariants sont respectés
- Les types sont opaques/génériques pour l'extensibilité produit

**Technique :**

- Dépend uniquement du kernel
- Aucune dépendance externe
- Tests passent
- Documentation complète

**Architecture :**

- Trait ContentManager exposé publiquement
- Types internes si nécessaire (pas d'exposition inutile)
- Pas de connaissance du produit
- Pas de persistance imposée