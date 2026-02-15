---
name: Implémentation Miyukini COG vers. 0.1.0
overview: "Plan d'implémentation complet de Miyukini COG vers. 0.1.0 suivant les trois documents de référence : Documentation Implementation Reference, MSCM MIP Compliance Checklist, et Quick Reference Guide. Le plan couvre les 3 phases (Kernel, Cores système, MiyukiniAdmin) avec respect strict des protocoles MSCM/MIP et des règles d'implémentation."
todos:
  - id: init-workspace
    content: Initialiser le workspace Rust avec Cargo.toml et structure de base
    status: completed
  - id: kernel-config
    content: Implémenter kernel_config.rs avec trait Config et EnvConfig
    status: completed
  - id: kernel-id
    content: Implémenter kernel_id.rs avec type Id et IdGenerator
    status: completed
  - id: kernel-time
    content: Implémenter kernel_time.rs avec trait Clock et DefaultClock
    status: completed
  - id: kernel-log
    content: Implémenter kernel_log.rs avec enum Level et trait Logger
    status: completed
  - id: kernel-lifecycle
    content: Implémenter kernel_lifecycle.rs avec trait Lifecycle
    status: completed
  - id: kernel-lib
    content: Créer kernel_lib.rs avec exports publics
    status: completed
  - id: kernel-tests
    content: Créer tests d'intégration Kernel
    status: completed
  - id: mip-phase1
    content: Générer index MIP Phase 1 (Kernel)
    status: completed
  - id: strongfather
    content: Implémenter StrongFather (6 fichiers + lib)
    status: completed
  - id: kindmother
    content: Implémenter KindMother (6 fichiers + lib)
    status: completed
  - id: borderguard
    content: Implémenter BorderGuard (3 fichiers + lib)
    status: completed
  - id: caringnanny
    content: Implémenter CaringNanny (3 fichiers + lib)
    status: completed
  - id: masterbutler
    content: Implémenter MasterButler (3 fichiers + lib)
    status: completed
  - id: bondingbrother
    content: Implémenter BondingBrother (3 fichiers + lib)
    status: completed
  - id: everbuddy
    content: Implémenter EverBuddy (3 fichiers + lib)
    status: completed
  - id: worrysentinel
    content: Implémenter WorrySentinel (3 fichiers + lib)
    status: completed
  - id: tamr
    content: Implémenter TAMR (3 fichiers + lib)
    status: completed
  - id: logisticssteward
    content: Implémenter LogisticsSteward (3 fichiers + lib)
    status: completed
  - id: mip-phase2
    content: Générer index MIP Phase 2 (Kernel + Cores)
    status: completed
  - id: miyukiniadmin-init
    content: Initialiser projet MiyukiniAdmin (backend + frontend)
    status: completed
  - id: miyukiniadmin-backend
    content: Implémenter backend MiyukiniAdmin (10 fichiers)
    status: completed
  - id: miyukiniadmin-frontend
    content: Implémenter frontend MiyukiniAdmin (10 fichiers)
    status: completed
  - id: miyukiniadmin-tests
    content: Créer tests d'intégration MiyukiniAdmin
    status: completed
  - id: mip-final
    content: Générer index MIP final (Kernel + Cores + MiyukiniAdmin)
    status: completed
  - id: audit-final
    content: Effectuer audit final de conformité
    status: completed
  - id: document-gel
    content: Rédiger document de gel COG vers. 0.1.0
    status: completed
isProject: false
---

# Plan d'Implémentation Miyukini COG vers. 0.1.0

## Contexte et État Actuel

**État du projet :**

- Documentation complète et détaillée disponible
- Aucun code source Rust existant (départ de zéro)
- Aucun index MIP généré
- Workspace Rust à initialiser

**Références principales :**

- `docs/implementation/Miyukini COG 0.1 - Documentation Implementation Reference.md`
- `docs/implementation/Miyukini COG 0.1 - MSCM MIP Compliance Checklist.md`
- `docs/implementation/Miyukini COG 0.1 - Quick Reference Guide.md`

## Architecture d'Implémentation

### Ordre Strict d'Implémentation

```
Phase 1 : Kernel (fondation)
   ↓
Phase 2 : Cores système (par ordre de dépendance)
   ├── StrongFather (décision pure, aucune dépendance)
   ├── KindMother (persistance, dépend de Kernel)
   ├── BorderGuard (frontières, dépend de Kernel)
   ├── CaringNanny (observation, dépend de Kernel)
   ├── MasterButler (orchestration, dépend de StrongFather + KindMother)
   ├── BondingBrother (liaison, dépend de StrongFather + KindMother)
   ├── EverBuddy (compatibilité, dépend de KindMother)
   ├── WorrySentinel (sécurité, dépend de CaringNanny)
   ├── TAMR (taxonomies, dépend de KindMother)
   └── LogisticsSteward (ressources, dépend de CaringNanny)
   ↓
Phase 3 : MiyukiniAdmin (opérateur souverain)
```

### Règles de Distribution des Tâches

- **1 agent = 1 fichier** (règle absolue)
- **Contexte vierge obligatoire** pour chaque agent
- **Maximum 4 agents simultanés**
- **Nomenclature** : `[xx] - [nom du fichier]` où `xx` est le préfixe de regroupement
- **Pas de mutualisation** de tâches

## Phase 1 : Kernel

### 1.1 Initialisation du Workspace Rust

**Tâche :** `[00] - Initialisation workspace Rust`

**Actions :**

- Créer `Cargo.toml` à la racine (workspace)
- Créer structure de base `miyukini-kernel/`
- Configurer `Cargo.toml` du crate kernel
- Créer structure de dossiers `src/` et `tests/`

**Références :**

- `docs/kernel/Miyukini Core System - Structure du Kernel.md`

### 1.2 Modules Kernel (5 modules)

**Ordre d'implémentation :**

1. `**[01] - kernel_config.rs**`
  - Trait `Config`
  - Struct `EnvConfig`
  - Implémentation `Config` pour `EnvConfig`
  - Balisage MSCM complet
  - Tests unitaires
2. `**[01] - kernel_id.rs**`
  - Type `Id` (opaque)
  - Trait `IdGenerator`
  - Struct `UuidIdGenerator`
  - Implémentation complète
  - Balisage MSCM complet
  - Tests unitaires
3. `**[01] - kernel_time.rs**`
  - Trait `Clock`
  - Struct `DefaultClock`
  - Struct `FakeClock` (pour tests)
  - Implémentation complète
  - Balisage MSCM complet
  - Tests unitaires
4. `**[01] - kernel_log.rs**`
  - Enum `Level`
  - Trait `Logger`
  - Struct `DefaultLogger`
  - Implémentation complète
  - Balisage MSCM complet
  - Tests unitaires
5. `**[02] - kernel_lifecycle.rs**`
  - Trait `Lifecycle`
  - Struct `DefaultLifecycle`
  - Implémentation complète (hooks LIFO)
  - Balisage MSCM complet
  - Tests unitaires

**Références Kernel :**

- `docs/kernel/Miyukini Core System - Definition Kernel.md`
- `docs/kernel/Miyukini Core System - Revue Traits API v0.1.md`
- `docs/kernel/implementation/Kernel - Reference Implementation Guidelines.md`
- `docs/kernel/tests/Kernel - Tests Unitaires Specification.md`
- `docs/kernel/contracts/Kernel - Invariants & Guarantees.md`

### 1.3 Module lib.rs

**Tâche :** `[02] - kernel_lib.rs`

**Actions :**

- Exposer tous les modules publics
- Ré-exports des types principaux
- Documentation du module

### 1.4 Tests d'Intégration Kernel

**Tâche :** `[03] - kernel_integration_tests.rs`

**Actions :**

- Tests d'intégration entre modules
- Tests de conformité aux invariants (INV-K-*)
- Validation des contrats

### 1.5 Génération Index MIP Phase 1

**Tâche :** `[04] - Generation index MIP Phase 1`

**Actions :**

- Scanner tous les fichiers Kernel
- Parser les blocs MSCM
- Générer l'index MIP complet dans `mscm_index/`
- Valider l'intégrité (`registry.json → integrity: "ok"`)

**Références :**

- `docs/protocols/Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol.md`

### 1.6 Validation Phase 1

**Check-list avant passage Phase 2 :**

- Tous les modules Kernel implémentés
- Tests unitaires passants
- Tests d'intégration passants
- Balisage MSCM complet (100% couverture)
- Index MIP généré et valide
- Conformité aux invariants Kernel (INV-K-1 à INV-K-10)
- Documentation inline complète

## Phase 2 : Cores Système

### 2.1 StrongFather (Core 1)

**Ordre d'implémentation :**

1. `**[10] - strongfather_intent.rs**`
  - Modèle d'intention
  - Balisage MSCM
  - Tests unitaires
2. `**[10] - strongfather_policy.rs**`
  - Modèle de politique
  - Balisage MSCM
  - Tests unitaires
3. `**[10] - strongfather_decision.rs**`
  - Modèle de décision
  - Balisage MSCM
  - Tests unitaires
4. `**[10] - strongfather_policy_engine.rs**`
  - Moteur d'évaluation
  - Balisage MSCM
  - Tests unitaires
5. `**[10] - strongfather_priority.rs**`
  - Gestion des priorités
  - Balisage MSCM
  - Tests unitaires
6. `**[10] - strongfather_validator.rs**`
  - Validation d'intentions
  - Balisage MSCM
  - Tests unitaires
7. `**[11] - strongfather_lib.rs**`
  - Module principal
  - Exports publics

**Références StrongFather :**

- `docs/core/StrongFather/foundation/StrongFather - Documentation Fondatrice.md`
- `docs/core/StrongFather/implementation/guidelines/StrongFather - Implementation Overview.md`
- `docs/core/StrongFather/implementation/guidelines/StrongFather - Implementation Patterns.md`
- `docs/core/StrongFather/implementation/guidelines/StrongFather - Implementation Prohibitions.md`

**Check-list StrongFather :**

- Aucune autorité sur l'exécution
- Aucune autorité sur la persistance
- Décisions pures (pas d'effets de bord)
- Tous les blocs balisés MSCM
- Tests unitaires complets
- Conformité invariants INV-SF-*

### 2.2 KindMother (Core 2)

**Ordre d'implémentation :**

1. `**[20] - kindmother_state.rs**`
  - Gestion d'état
  - Balisage MSCM
  - Tests unitaires
2. `**[20] - kindmother_storage.rs**`
  - Abstraction stockage
  - Balisage MSCM
  - Tests unitaires
3. `**[20] - kindmother_sync.rs**`
  - Synchronisation
  - Balisage MSCM
  - Tests unitaires
4. `**[20] - kindmother_api.rs**`
  - API CoreData
  - Balisage MSCM
  - Tests unitaires
5. `**[20] - kindmother_threat.rs**`
  - Détection de menaces
  - Balisage MSCM
  - Tests unitaires
6. `**[20] - kindmother_observability.rs**`
  - Observabilité
  - Balisage MSCM
  - Tests unitaires
7. `**[21] - kindmother_lib.rs**`
  - Module principal

**Références KindMother :**

- `docs/core/KindMother/foundation/KindMother - Documentation Fondatrice.md`
- `docs/core/KindMother/implementation/KindMother - Reference Implementation Guidelines.md`
- `docs/core/KindMother/contracts/KindMother - CoreDataAPI Contract.md`

**Check-list KindMother :**

- Séparation stricte décision/exécution
- Persistance fiable
- Détection de corruption
- Tous les blocs balisés MSCM
- Tests unitaires complets
- Conformité invariants INV-KM-*

### 2.3 BorderGuard (Core 3)

**Ordre d'implémentation :**

1. `**[30] - borderguard_boundary.rs**`
  - Définition de frontière
  - Balisage MSCM
  - Tests unitaires
2. `**[30] - borderguard_trust_level.rs**`
  - Niveaux de confiance
  - Balisage MSCM
  - Tests unitaires
3. `**[30] - borderguard_crossing.rs**`
  - Règles de franchissement
  - Balisage MSCM
  - Tests unitaires
4. `**[31] - borderguard_lib.rs**`
  - Module principal

**Références BorderGuard :**

- `docs/core/BorderGuard/foundation/Border Guard - Documentation Fondatrice.md`
- `docs/core/BorderGuard/implementation/Border Guard - Reference Implementation Guidelines.md`

### 2.4 CaringNanny (Core 4)

**Ordre d'implémentation :**

1. `**[40] - caringnanny_observer.rs**`
  - Observation d'événements
  - Balisage MSCM
  - Tests unitaires
2. `**[40] - caringnanny_metrics.rs**`
  - Collecte de métriques
  - Balisage MSCM
  - Tests unitaires
3. `**[40] - caringnanny_health.rs**`
  - État de santé
  - Balisage MSCM
  - Tests unitaires
4. `**[41] - caringnanny_lib.rs**`
  - Module principal

**Références CaringNanny :**

- `docs/core/CaringNanny/foundation/Caring Nanny - Documentation Fondatrice.md`
- `docs/core/CaringNanny/implementation/Caring Nanny - Reference Implementation Guidelines.md`

### 2.5 MasterButler (Core 5)

**Dépendances :** StrongFather + KindMother

**Ordre d'implémentation :**

1. `**[50] - masterbutler_workflow.rs**`
  - Définition de workflow
  - Balisage MSCM
  - Tests unitaires
2. `**[50] - masterbutler_orchestrator.rs**`
  - Orchestration d'exécution
  - Balisage MSCM
  - Tests unitaires
3. `**[50] - masterbutler_step.rs**`
  - Étapes de workflow
  - Balisage MSCM
  - Tests unitaires
4. `**[51] - masterbutler_lib.rs**`
  - Module principal

**Références MasterButler :**

- `docs/core/MasterButler/foundation/Master Butler - Documentation Fondatrice.md`
- `docs/core/MasterButler/implementation/Master Butler - Reference Implementation Guidelines.md`

### 2.6 BondingBrother (Core 6)

**Dépendances :** StrongFather + KindMother

**Ordre d'implémentation :**

1. `**[60] - bondingbrother_connection.rs**`
  - Gestion de connexions
  - Balisage MSCM
  - Tests unitaires
2. `**[60] - bondingbrother_sync.rs**`
  - Synchronisation
  - Balisage MSCM
  - Tests unitaires
3. `**[60] - bondingbrother_translation.rs**`
  - Traduction de formats
  - Balisage MSCM
  - Tests unitaires
4. `**[61] - bondingbrother_lib.rs**`
  - Module principal

**Références BondingBrother :**

- `docs/core/BondingBrother/foundation/BondingBrother - Documentation Fondatrice.md`
- `docs/core/BondingBrother/implementation/BondingBrother - Reference Implementation Guidelines.md`

### 2.7 EverBuddy (Core 7)

**Dépendances :** KindMother

**Ordre d'implémentation :**

1. `**[70] - everbuddy_compatibility.rs**`
  - Gestion de compatibilité
  - Balisage MSCM
  - Tests unitaires
2. `**[70] - everbuddy_migration.rs**`
  - Migration de versions
  - Balisage MSCM
  - Tests unitaires
3. `**[70] - everbuddy_version.rs**`
  - Gestion de versions
  - Balisage MSCM
  - Tests unitaires
4. `**[71] - everbuddy_lib.rs**`
  - Module principal

**Références EverBuddy :**

- `docs/core/EverBuddy/foundation/Ever Buddy - Documentation Fondatrice.md`
- `docs/core/EverBuddy/implementation/Ever Buddy - Reference Implementation Guidelines.md`

### 2.8 WorrySentinel (Core 8)

**Dépendances :** CaringNanny

**Ordre d'implémentation :**

1. `**[80] - worrysentinel_threat_detector.rs**`
  - Détection de menaces
  - Balisage MSCM
  - Tests unitaires
2. `**[80] - worrysentinel_security_level.rs**`
  - Niveaux de sécurité
  - Balisage MSCM
  - Tests unitaires
3. `**[80] - worrysentinel_degradation.rs**`
  - Gestion de dégradation
  - Balisage MSCM
  - Tests unitaires
4. `**[81] - worrysentinel_lib.rs**`
  - Module principal

**Références WorrySentinel :**

- `docs/core/WorrySentinel/foundation/WorrySentinel - Documentation Fondatrice.md`
- `docs/core/WorrySentinel/implementation/WorrySentinel - Reference Implementation Guidelines.md`

### 2.9 TAMR (Core 9)

**Dépendances :** KindMother

**Ordre d'implémentation :**

1. `**[90] - tamr_taxonomy.rs**`
  - Gestion de taxonomies
  - Balisage MSCM
  - Tests unitaires
2. `**[90] - tamr_metadata.rs**`
  - Gestion de métadonnées
  - Balisage MSCM
  - Tests unitaires
3. `**[90] - tamr_classification.rs**`
  - Classification
  - Balisage MSCM
  - Tests unitaires
4. `**[91] - tamr_lib.rs**`
  - Module principal

**Références TAMR :**

- `docs/core/TAMR/foundation/TAMR - Documentation Fondatrice.md`
- `docs/core/TAMR/implementation/TAMR - Reference Implementation Guidelines.md`

### 2.10 LogisticsSteward (Core 10)

**Dépendances :** CaringNanny

**Ordre d'implémentation :**

1. `**[100] - logisticssteward_resource.rs**`
  - Gestion de ressources
  - Balisage MSCM
  - Tests unitaires
2. `**[100] - logisticssteward_optimization.rs**`
  - Optimisation
  - Balisage MSCM
  - Tests unitaires
3. `**[100] - logisticssteward_allocation.rs**`
  - Allocation de ressources
  - Balisage MSCM
  - Tests unitaires
4. `**[101] - logisticssteward_lib.rs**`
  - Module principal

**Références LogisticsSteward :**

- `docs/core/LogisticsSteward/foundation/LogisticsSteward - Documentation Fondatrice.md`
- `docs/core/LogisticsSteward/implementation/LogisticsSteward - Reference Implementation Guidelines.md`

### 2.11 Génération Index MIP Phase 2

**Tâche :** `[110] - Generation index MIP Phase 2`

**Actions :**

- Régénérer l'index MIP avec tous les Cores
- Valider l'intégrité
- Vérifier les dépendances entre Cores
- Valider la hiérarchie

### 2.12 Validation Phase 2

**Check-list avant passage Phase 3 :**

- Tous les Cores implémentés dans l'ordre strict
- Contrats d'intégration respectés
- Tests obligatoires passants pour chaque Core
- Balisage MSCM complet (100% couverture)
- Index MIP régénéré et valide
- Conformité aux invariants de chaque Core
- Documentation inline complète

## Phase 3 : MiyukiniAdmin

### 3.1 Initialisation Projet MiyukiniAdmin

**Tâche :** `[200] - Initialisation projet MiyukiniAdmin`

**Actions :**

- Créer structure `miyukini-admin/`
- Configurer backend Rust (`backend/Cargo.toml`)
- Configurer frontend TypeScript (`frontend/package.json`)
- Structure de dossiers complète

**Références :**

- `docs/core/MiyukiniAdmin/foundation/MiyukiniAdmin - Documentation Fondatrice.md`
- `docs/core/MiyukiniAdmin/implementation/MiyukiniAdmin - Reference Implementation Guidelines.md`
- `docs/core/MiyukiniAdmin/architecture/MiyukiniAdmin - Architecture & Components.md`

### 3.2 Backend MiyukiniAdmin

**Ordre d'implémentation :**

1. `**[201] - miyukiniadmin_config.rs**`
  - Configuration backend
  - Balisage MSCM
  - Tests unitaires
2. `**[201] - miyukiniadmin_monitoring_service.rs**`
  - Service de monitoring
  - Balisage MSCM
  - Tests unitaires
3. `**[201] - miyukiniadmin_database_service.rs**`
  - Service base de données
  - Balisage MSCM
  - Tests unitaires
4. `**[201] - miyukiniadmin_security_service.rs**`
  - Service sécurité
  - Balisage MSCM
  - Tests unitaires
5. `**[201] - miyukiniadmin_testing_service.rs**`
  - Service tests
  - Balisage MSCM
  - Tests unitaires
6. `**[201] - miyukiniadmin_bonding_brother_bridge.rs**`
  - Pont vers BondingBrother
  - Balisage MSCM
  - Tests unitaires
7. `**[201] - miyukiniadmin_audit_logger.rs**`
  - Journalisation d'audit
  - Balisage MSCM
  - Tests unitaires
8. `**[201] - miyukiniadmin_api_handlers.rs**`
  - Handlers API
  - Balisage MSCM
  - Tests unitaires
9. `**[201] - miyukiniadmin_api_routes.rs**`
  - Routes API
  - Balisage MSCM
  - Tests unitaires
10. `**[202] - miyukiniadmin_backend_main.rs**`
  - Point d'entrée backend
    - Balisage MSCM

### 3.3 Frontend MiyukiniAdmin

**Ordre d'implémentation :**

1. `**[210] - miyukiniadmin_frontend_types.ts**`
  - Types TypeScript
  - Documentation
2. `**[210] - miyukiniadmin_frontend_services.ts**`
  - Services API
  - Documentation
3. `**[210] - miyukiniadmin_frontend_store.ts**`
  - Store (Redux/Zustand)
  - Documentation
4. `**[210] - miyukiniadmin_frontend_dashboard.tsx**`
  - Composant Dashboard
  - Documentation
5. `**[210] - miyukiniadmin_frontend_database_view.tsx**`
  - Vue base de données
  - Documentation
6. `**[210] - miyukiniadmin_frontend_security_view.tsx**`
  - Vue sécurité
  - Documentation
7. `**[210] - miyukiniadmin_frontend_testing_view.tsx**`
  - Vue tests
  - Documentation
8. `**[210] - miyukiniadmin_frontend_common_components.tsx**`
  - Composants communs
  - Documentation
9. `**[211] - miyukiniadmin_frontend_app.tsx**`
  - Composant App principal
  - Documentation
10. `**[211] - miyukiniadmin_frontend_main.tsx**`
  - Point d'entrée frontend
    - Documentation

### 3.4 Tests d'Intégration MiyukiniAdmin

**Tâche :** `[220] - miyukiniadmin_integration_tests.rs`

**Actions :**

- Tests d'intégration backend-frontend
- Tests d'intégration avec tous les Cores
- Validation des contrats

### 3.5 Génération Index MIP Final

**Tâche :** `[230] - Generation index MIP final`

**Actions :**

- Régénérer l'index MIP complet (Kernel + Cores + MiyukiniAdmin)
- Valider l'intégrité finale
- Vérifier toutes les dépendances
- Valider la hiérarchie complète
- Générer `registry.json` avec `integrity: "ok"`

### 3.6 Validation Phase 3

**Check-list avant gel :**

- MiyukiniAdmin backend implémenté
- MiyukiniAdmin frontend implémenté
- Intégration avec tous les Cores validée
- Tests end-to-end passants
- Balisage MSCM complet (100% couverture)
- Index MIP final généré et valide
- Documentation inline complète
- Conformité aux invariants MiyukiniAdmin

## Phase 4 : Gel et Versionnement

### 4.1 Audit Final

**Tâche :** `[300] - Audit final conformité`

**Actions :**

- Audit de code complet
- Vérification de tous les invariants
- Vérification de tous les contrats
- Vérification MSCM/MIP complète
- Rapport d'audit

**Références :**

- `docs/qa/Audit - Qualite et Risques Derive Implementation v1.md`

### 4.2 Document de Gel

**Tâche :** `[301] - Document de gel COG vers. 0.1.0`

**Contenu obligatoire :**

- Liste exhaustive des éléments gelés
- Version explicite (v0.1.0)
- Date de gel
- Index MIP final inclus
- Règles d'évolution futures
- Conditions de dégel

**Références :**

- `docs/kernel/Kernel - Gel et Versionnement v0.1.md`

### 4.3 Critères de Gel

**Check-list finale :**

- Implémentation complète (Kernel + Cores + MiyukiniAdmin)
- Tests complets passants
- Conformité MSCM/MIP complète
- Audit validé
- Documentation complète
- Index MIP final valide (`integrity: "ok"`)
- Document de gel rédigé
- Version attribuée (v0.1.0)

## Règles Strictes d'Implémentation

### Protocole MSCM Obligatoire

**Obligations minimales pour chaque bloc fonctionnel :**

- `@id` : Identifiant unique global (OBLIGATOIRE)
- `@role` : Rôle sémantique explicite (OBLIGATOIRE)
- `@layer` : Couche architecturale déclarée (OBLIGATOIRE)
- `@human` : Description humaine compréhensible (OBLIGATOIRE)
- `@do` : Action principale du bloc (recommandé)
- `@depends` : Dépendances inter-blocs déclarées (si applicable)

**Format de balisage :**

```rust
/// @id: kernel_config_load
/// @role: infrastructure
/// @layer: kernel
/// @human: Charge la configuration depuis les variables d'environnement
/// @do: load_config_from_env
pub fn load_config() -> Result<Config, ConfigError> {
    // ...
}
```

### Protocole MIP Obligatoire

**Règles MIP :**

- L'index est **externe** au code (dossier `mscm_index/`)
- L'index est **reconstruit**, jamais modifié manuellement
- Le code est la seule source de vérité
- Régénérer l'index MIP après chaque modification

**Fichiers d'index obligatoires :**

- `registry.json` : Métadonnées et intégrité
- `blocks.json` : Identité sémantique
- `hierarchy.json` : Structure hiérarchique
- `graph.json` : Relations transverses
- `flows.json` : Processus métier
- `domains.json` : Vision métier
- `layers.json` : Architecture technique
- `dependencies.json` : Dépendances logiques
- `files.json` : Cartographie code
- `stats.json` : Métriques

### Règles pour Agents IA

**Règles absolues :**

- ✅ Respecter rigoureusement les protocoles référencés
- ✅ Ne jamais contourner les invariants documentés
- ✅ Toujours baliser le code en MSCM
- ✅ Régénérer l'index MIP après chaque modification
- ✅ Respecter l'ordre d'implémentation strict
- ✅ Qualité optimale (pas de code "quick and dirty")
- ✅ Tests unitaires obligatoires
- ✅ Documentation inline complète

**Règle d'arrêt stricte :**
Un agent DOIT S'ARRÊTER IMMÉDIATEMENT si :

- ❌ Une ambiguïté bloquante est détectée
- ❌ Une dépendance manquante est rencontrée
- ❌ La fenêtre de contexte devient insuffisante
- ❌ Le fichier et le test unitaire sont terminés et corrects

**Action :** Rendre la main à l'humain, aucun fichier partiel généré.

## Vérifications Avant Chaque Livraison

### Vérification d'un Fichier

- Code implémenté selon la documentation de référence
- Balisage MSCM complet et conforme
- Tests unitaires présents et passants
- Gestion d'erreurs explicite
- Documentation inline complète
- Aucune dépendance non autorisée
- Conformité aux invariants du composant

### Vérification d'une Phase

- Tous les composants de la phase implémentés
- Tous les tests passants
- Documentation à jour
- Index MIP régénéré sans erreur
- Intégrité validée
- Graphe cohérent
- Invariants respectés
- Contrats d'intégration respectés

## Références Documentaires Complètes

### Protocoles

- `docs/protocols/Miyukini Prompt Protocol - Implémentation générale.md`
- `docs/protocols/Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol.md`

### Kernel

- `docs/kernel/Miyukini Core System - Definition Kernel.md`
- `docs/kernel/Miyukini Core System - Structure du Kernel.md`
- `docs/kernel/Miyukini Core System - Revue Traits API v0.1.md`
- `docs/kernel/contracts/Kernel - Invariants & Guarantees.md`
- `docs/kernel/implementation/Kernel - Reference Implementation Guidelines.md`
- `docs/kernel/tests/Kernel - Tests Unitaires Specification.md`

### Cores Système

Chaque Core a sa documentation dans `docs/core/<CoreName>/` :

- `foundation/` - Documentation fondatrice
- `implementation/` - Guidelines d'implémentation
- `contracts/` - Contrats d'intégration

### MiyukiniAdmin

- `docs/core/MiyukiniAdmin/foundation/MiyukiniAdmin - Documentation Fondatrice.md`
- `docs/core/MiyukiniAdmin/implementation/MiyukiniAdmin - Reference Implementation Guidelines.md`
- `docs/core/MiyukiniAdmin/architecture/MiyukiniAdmin - Architecture & Components.md`

### Références Conceptuelles

- `docs/reference/Miyukini Conceptual References - Definition COG.md`
- `docs/reference/Miyukini Conceptual References - Pyramide Architecture Complete.md`
- `docs/reference/Miyukini Conceptual References - Lois Autonomie Systeme.md`
- `docs/reference/Miyukini Conceptual References - Glossaire.md`

## Notes Importantes

1. **Aucune étape ne peut être sautée** : Le cycle strict (Planification → Distribution → Vérification → Gel) doit être respecté
2. **Contexte vierge obligatoire** : Chaque agent démarre avec un contexte propre
3. **Maximum 4 agents simultanés** : Limite stricte pour éviter la surcharge contextuelle
4. **1 agent = 1 fichier** : Règle absolue, pas de mutualisation
5. **Balisage MSCM obligatoire** : Un fichier sans balisage MSCM conforme est non livrable
6. **Index MIP obligatoire** : Un projet sans index MIP valide ne peut pas être gelé

**Toute implémentation hors de ce cadre est considérée comme non conforme.**