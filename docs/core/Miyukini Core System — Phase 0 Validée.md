# Miyukini Core System — Phase 0 Validée

## 1. Objectif du document

Ce document fige officiellement le périmètre de la **Phase 0** du projet Miyukini Core System. Il définit ce qui est considéré comme stable et validé, et établit les règles de non-régression.

**Rôle :** Référence contractuelle pour les évolutions futures. Toute modification du périmètre Phase 0 nécessite une nouvelle phase.

**Ce qu'il fige :**
- Périmètre fonctionnel du kernel (infra)
- Périmètre fonctionnel des modules SPM CMS Phase 0
- Contrats publics (traits, types, erreurs)
- Règles de stabilité et d'évolution

**Ce qu'il ne couvre pas :**
- Implémentation détaillée (code, algorithmes)
- Architecture technique interne
- Phase 1 et phases ultérieures
- Produits consommateurs (sauf produit pilote de validation)

---

## 2. Portée de la Phase 0

### Ce que la Phase 0 inclut

**Kernel (infra) :**
- 5 modules : `config`, `id`, `time`, `log`, `lifecycle`
- Contrats publics (traits, types, implémentations par défaut)
- Aucune dépendance externe lourde

**SPM CMS :**
- 3 modules fondation : Contenu, Hiérarchie, Taxonomies
- Contrats fonctionnels (traits, types, erreurs)
- Implémentations mémoire pour tests/démo (feature `memory`)

**Validation :**
- Produit pilote `mini-cms` consommant les 3 modules SPM CMS
- Tests unitaires pour chaque module
- Documentation minimale (README, contrats)

### Ce que la Phase 0 exclut explicitement

**Kernel :**
- Connexions / pooling (Phase 2)
- Clients HTTP / retry (Phase 2)
- Gestion d'erreurs avancée (Phase 2)
- Métriques / tracing distribué (Phase 2)

**SPM CMS :**
- Modules cœur (Références Média, Publication, Blocs) → Phase 1
- Recherche full-text → Phase 1
- Permissions / accès → Hors-scope SPM
- Rendu / UI → Hors-scope SPM
- Logique métier spécifique → Produit

---

## 3. Kernel (infra)

### Rôle du kernel

Fondation technique réutilisable pour plusieurs produits (SaaS, web, mobile, jeu). Couche d'exécution et de coordination : boot, config, arrêt, observabilité de base.

### Responsabilités garanties

**Boot et lifecycle :**
- Ordre d'initialisation et d'arrêt des briques techniques
- Hooks de shutdown (pas d'orchestration métier)

**Configuration :**
- Chargement depuis environnement, fichiers, secrets
- Accès structuré (pas de politique métier)

**Identifiants :**
- Génération d'IDs (UUID v4 par défaut)
- Parsing d'IDs (format opaque)

**Abstraction temps :**
- `now()` injectable en test
- Pas de timezone (choix du produit)

**Logging structuré :**
- Niveaux (Error, Warn, Info, Debug, Trace)
- Contrat minimal (pas de backend imposé)

### Ce que le kernel ne fera jamais

- Jobs métier, hooks applicatifs, workflows
- Clients externes (email, paiement, API tierce)
- Protocoles applicatifs (HTTP, WebSocket)
- ORM, couche d'accès données
- Framework applicatif complet
- Politique de configuration métier

---

## 4. SPM CMS — Modules validés

### Module Contenu

**Responsabilité :** Gestion des entités de contenu (pages, articles, blocs) : CRUD, statuts (brouillon/publié/archivé), relations, versioning, métadonnées.

**Garanties fonctionnelles :**
- CRUD complet (create, get, update, delete)
- Statuts fonctionnels (Draft, Published, Archived)
- Relations entre contenus (type opaque)
- Versioning (snapshot, restore)
- Liste avec filtres et pagination
- Métadonnées (format opaque, Vec<u8>)

**Hors-scope explicite :**
- Stockage et persistance (DB, fichiers)
- Permissions et accès
- Rendu et affichage
- Recherche full-text
- Workflow métier
- SEO et référencement

### Module Hiérarchie

**Responsabilité :** Organisation générique d'entités externes en arbre : création de racines et d'enfants, navigation (parent, children, ancestors, path_to_root), déplacement de nœuds, suppression.

**Garanties fonctionnelles :**
- Création de racines et d'enfants
- Navigation (parent, children, ancestors, path_to_root)
- Déplacement de nœuds (avec détection de cycles)
- Suppression de nœuds (enfants deviennent racines en Phase 0)
- Détection et refus de cycles
- Cohérence référentielle (parent/children bidirectionnel)

**Hors-scope explicite :**
- Stockage et persistance (DB, fichiers)
- Permissions et accès
- Logique métier spécifique (CMS, SEO, navigation)
- Rendu et affichage
- Gestion des utilisateurs
- Hiérarchie entre termes (Module Taxonomies)

### Module Taxonomies

**Responsabilité :** Système de classification générique d'entités : création de taxonomies, ajout de termes, assignation/désassignation de termes à des entités, recherche bidirectionnelle.

**Garanties fonctionnelles :**
- Création de taxonomies
- Ajout de termes à une taxonomie
- Assignation/désassignation de termes à des entités (idempotent)
- Recherche bidirectionnelle (termes pour entité, entités pour terme)
- Appartenance terme → taxonomie

**Hors-scope explicite :**
- Stockage et persistance (DB, fichiers)
- Permissions et accès
- Logique métier spécifique (SEO, navigation, sémantique)
- Rendu et affichage
- Gestion des utilisateurs
- Hiérarchie entre termes (Module Hiérarchie)
- Recherche par taxonomie (Module Recherche, Phase 1)

---

## 5. Produit pilote de validation

**Nom :** `mini-cms`

**Rôle du produit pilote :**
Valider la Phase 0 complète en consommant les 3 modules SPM CMS avec un scénario réel (création de contenus, organisation hiérarchique, classification par taxonomies).

**Ce qu'il prouve (factuellement) :**
- Les 3 modules SPM CMS sont fonctionnels et testés
- Un produit peut consommer les modules sans logique métier spécifique
- Les contrats sont respectés (traits, types, erreurs)
- L'intégration kernel est validée (Id, Clock, Logger)
- Les dépendances entre modules sont claires et unidirectionnelles
- Les tests passent (15 tests hierarchy, 15 tests taxonomies, tests content)
- Le produit pilote s'exécute avec succès et affiche un CR complet

**Statut :** Phase 0 validée ✓

---

## 6. Règles de stabilité

### Ce qui est considéré comme stable

**Kernel :**
- Signatures des traits publics (Config, IdGenerator, Clock, Logger, Lifecycle)
- Types publics (Id, Level, etc.)
- Comportement fonctionnel des implémentations par défaut

**SPM CMS :**
- Signatures des traits publics (ContentManager, HierarchyManager, TaxonomyManager)
- Types publics (ContentId, NodeId, TaxonomyId, TermId, EntityId)
- Enums d'erreurs publics (ContentError, HierarchyError, TaxonomyError)
- Comportement fonctionnel des opérations (invariants, garanties)

### Ce qui nécessite une nouvelle phase

- Ajout de méthodes aux traits publics
- Modification de signatures existantes
- Suppression de méthodes ou types publics
- Changement de comportement fonctionnel (invariants, garanties)
- Ajout de modules au kernel ou au SPM CMS

### Principe de non-régression

Toute modification du périmètre Phase 0 doit :
1. Maintenir la compatibilité ascendante des contrats publics
2. Conserver les invariants et garanties fonctionnelles
3. Passer tous les tests existants
4. Valider avec le produit pilote `mini-cms`

---

## 7. Règles d'évolution après Phase 0

### Quand créer un nouveau module

**Kernel :**
- Besoin partagé par ≥2 types de produits (SaaS, web, mobile, jeu)
- Responsabilité strictement infra et transverse
- Aucune logique métier spécifique

**SPM CMS :**
- Besoin partagé par ≥2 produits CMS
- Responsabilité fonctionnelle générique (pas de métier)
- Dépendances claires et unidirectionnelles

### Quand refuser une généralisation

- Logique métier spécifique (validation, règles business)
- Politique de configuration métier
- Protocoles applicatifs (HTTP, WebSocket)
- Clients externes (email, paiement, API tierce)
- Rendu / UI
- Permissions / accès (hors-scope SPM)

### Où placer la logique métier

**Dans le produit :**
- Validation métier
- Règles business
- Workflows applicatifs
- Politiques de configuration
- Clients externes
- Rendu / UI
- Permissions / accès

**Dans le SPM (Phase 1+) :**
- Modules fonctionnels génériques (Références Média, Publication, Blocs)
- Recherche full-text
- Capacités partagées par plusieurs produits CMS

---

## 8. Mini résumé erreurs / warnings rencontrés

**Erreurs rencontrées pendant Phase 0 :**

1. **Compilation :** Warnings `unused_imports` (Clock, Config) dans mini-cms → Résolu par suppression des imports inutilisés
2. **Compilation :** Warnings `dead_code` (champs name, taxonomy_id, label) dans modules SPM → Résolu par `#[allow(dead_code)]` avec commentaire Phase 1+
3. **Compilation :** Warnings `unused_mut` dans tests → Résolu par correction des déclarations `mut`
4. **Tests :** Aucune erreur fonctionnelle, tous les tests passent (14 hierarchy, 15 taxonomies, tests content)

**Aucune erreur bloquante.** Phase 0 compilée et testée avec succès.

---

## 9. Statut final

**Phase 0 : VALIDÉE ✓**

- Kernel (infra) : 5 modules validés
- SPM CMS : 3 modules validés (Contenu, Hiérarchie, Taxonomies)
- Produit pilote : `mini-cms` fonctionnel
- Tests : Tous passent
- Documentation : README et contrats disponibles

**Date de validation :** 2026-01-24

**Prochaine étape :** Phase 1 (Modules cœur SPM CMS : Références Média, Publication, Blocs)
