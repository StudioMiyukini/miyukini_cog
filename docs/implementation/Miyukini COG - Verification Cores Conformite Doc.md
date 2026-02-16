# Miyukini COG — Vérification conformité Cores / Documentation

## Contexte

Ce document rapporte la **vérification de conformité** du code des Cores (Strate 4 + BondingBrother Strate 5) par rapport à la documentation (Reference Implementation Guidelines, Documentation Fondatrice, invariants).

**Date vérification :** 2026-02-16

---

## 1. BorderGuard

### Doc (résumé)

- **Mission :** Définir les frontières, classifier les niveaux de confiance, établir les règles. Ne jamais filtrer, bloquer, exécuter, décider (INV-BG-1, INV-BG-3).
- **Persistance :** Aucune persistance directe (INV-BG-2). Définitions en mémoire ; persistance déléguée à KindMother.
- **Classification :** Défaut = `unknown` (INV-BG-4). Toute API retourne un niveau (jamais null).
- **Interdictions :** Pas de `filter()`, `block()`, `allow()`, `deny()`, `decide()`.

### Vérification code

| Point | Attendu | Code | Conforme |
|-------|---------|------|:--------:|
| Pas de filtrage/blocage/décision | Aucune méthode filter, block, allow, deny, decide | Aucune trouvée dans `crates/borderguard` | ✅ |
| Pas de persistance directe | Pas de DB, pas de fichier | Aucun `std::fs`, `sqlite`, `libsql` dans le crate | ✅ |
| Registre en mémoire | BoundaryRegistry avec get/list | `BoundaryRegistry` : `get_boundary`, `list_boundaries`, `get_crossing_rules` ; `register_*` pour alimentation par l’adaptateur | ✅ |
| Classificateur défaut Unknown | Entité non enregistrée → Unknown | `DefaultTrustLevelClassifier::classify` retourne `unwrap_or(TrustLevel::Unknown)` | ✅ |
| Métadonnées traçabilité (INV-BG-8) | created_at, created_by, justification, version | `BoundaryMetadata` avec ces champs | ✅ |

**Verdict :** Conforme.

---

## 2. BondingBrother

### Doc (résumé)

- **Mission :** Médiation, traduction, pas de décision. Connexions et sync sous mandat.
- **LOI-1 :** Aucune dépendance externe critique.

### Vérification code

| Point | Attendu | Code | Conforme |
|-------|---------|------|:--------:|
| ConnectionManager | Établir/récupérer une Connection sous mandat | `DefaultConnectionManager::connect` crée une `Connection`, refus si déjà connecté | ✅ |
| SyncManager | Exécuter une stratégie (délégation persistance à KindMother) | `DefaultSyncManager::sync` no-op (orchestration sans persistance) | ✅ |
| Translator | Transformer données selon Translation | `DefaultTranslator` : identité si même format, sinon `UnsupportedFormat` | ✅ |
| Pas de persistance directe | Pas de DB/fichier dans le Core | Aucun usage trouvé | ✅ |

**Verdict :** Conforme.

---

## 3. CaringNanny

### Doc (résumé)

- **Mission :** Observer, détecter, classer, propager. Ne jamais modifier, décider, exécuter (INV-CN-1, INV-CN-2, INV-CN-3).
- **Observations :** Stockées dans un historique propre à Caring Nanny, pas dans les données métier.

### Vérification code

| Point | Attendu | Code | Conforme |
|-------|---------|------|:--------:|
| Observer pur | Pas d’écriture de données métier | `DefaultObserver` : `observe()` pousse dans un `Vec<SystemEvent>` interne (historique propre) | ✅ |
| Pas d’exécution | Aucun déclenchement d’action | Aucun callback d’action, pas de trigger | ✅ |
| HealthChecker | Retourner un statut (informatif) | `DefaultHealthChecker::check` retourne `HealthStatus` ; `set_status` met à jour le registre interne (vue observée), pas les données métier | ✅ |
| Pas de persistance directe | Pas de DB/fichier | Aucun usage trouvé | ✅ |

**Verdict :** Conforme.

---

## 4. MasterButler

### Doc (résumé)

- **Mission :** Recenser les capacités, définir les permissions, API de découverte. Ne décide jamais, n’exécute jamais (INV-MB-2).
- **Information pure :** Pas de booléen d’autorisation ; retours descriptifs.

### Vérification code

| Point | Attendu | Code | Conforme |
|-------|---------|------|:--------:|
| Pas de décision | Pas de allow/deny | Aucune méthode allow, deny, decide dans le crate | ✅ |
| Orchestrator | Exécuter un workflow (étapes déléguées) | `DefaultOrchestrator::execute` parcourt les steps et retourne `Vec<StepResult::Success>` sans appeler de logique métier (exécution réelle déléguée aux produits) | ✅ |
| Pas de persistance directe | Pas de DB/fichier | Aucun usage trouvé | ✅ |

**Verdict :** Conforme. L’orchestrateur par défaut ne fait qu’enchaîner des résultats sans exécuter de capacité métier.

---

## 5. EverBuddy

### Doc (résumé)

- **Mission :** Observer, enregistrer, guider l’évolution. Ne migre jamais, ne modifie jamais, n’exécute jamais (INV-EB-1).
- **Migrations :** Exécutées par KindMother ou les produits ; Ever Buddy fournit règles et traçabilité.

### Vérification code

| Point | Attendu | Code | Conforme |
|-------|---------|------|:--------:|
| Pas d’exécution de migration réelle | Aucune modification de données | `DefaultMigrationExecutor::execute` enregistre la migration dans un `Vec` (traçabilité) et retourne `Ok(())` ; pas de SQL ni écriture fichier | ✅ |
| VersionManager | Fournir la version courante | `DefaultVersionManager::get_current` retourne la version en mémoire | ✅ |
| Pas de persistance directe | Pas de DB/fichier | Aucun usage trouvé | ✅ |

**Verdict :** Conforme.

---

## 6. WorrySentinel

### Doc (résumé)

- **Mission :** Gouvernant conceptuel, pas exécuteur. Structures déclaratives, interfaces de consultation (INV-WS-1 à INV-WS-5).
- **INV-WS-3 :** Aucune persistance directe.
- **INV-WS-4 :** « WorrySentinel ne doit jamais contenir de méthodes `&mut self` qui modifient un état système. »

### Vérification code

| Point | Attendu | Code | Conforme |
|-------|---------|------|:--------:|
| Pas de verify_ / execute_ / save_ | Pas de contrôle ou persistance | Aucune fonction de vérification ou de persistance ; uniquement `get_current`, `detect` | ✅ |
| Pas de persistance directe | Pas de DB/fichier | Aucun usage trouvé | ✅ |
| ThreatDetector | Retourner un niveau (informatif) | `DefaultThreatDetector::detect()` retourne le max des signaux ; pas de décision d’action | ✅ |
| INV-WS-4 (pas de `&mut self` modifiant état) | Aucune méthode `&mut self` modifiant un état | Conformité maximale : aucun setter. `DefaultSecurityLevelManager` et `DefaultDegradationManager` configurés uniquement via `new(…)` ; `DefaultThreatDetector` via `new()` ou `with_signals(…)`. L’adaptateur fournit la vue au construction (instances immuables). | ✅ |

**Verdict :** Conforme (INV-WS-1 à INV-WS-5). Conformité maximale appliquée : setters supprimés, configuration au constructeur uniquement.

---

## 7. TAMR

### Doc (résumé)

- Gestion des taxonomies et métadonnées. Pas de logique métier applicative ; structure et classification uniquement.
- CRUD déclaratif, persistance déléguée à KindMother.

### Vérification code

| Point | Attendu | Code | Conforme |
|-------|---------|------|:--------:|
| TaxonomyManager | get / registre en mémoire | `DefaultTaxonomyManager::get`, `register` ; HashMap | ✅ |
| MetadataManager | get / registre en mémoire | `DefaultMetadataManager::get`, `register` ; HashMap | ✅ |
| Pas de persistance directe | Pas de DB/fichier | Aucun usage trouvé | ✅ |

**Verdict :** Conforme.

---

## 8. LogisticsSteward

### Doc (résumé)

- Définir et lister ressources et allocations. Décision d’allocation = StrongFather ; LogisticsSteward enregistre et informe.

### Vérification code

| Point | Attendu | Code | Conforme |
|-------|---------|------|:--------:|
| ResourceManager | get par type | `DefaultResourceManager::get`, `register` | ✅ |
| AllocationManager | allocate (enregistrement) | `DefaultAllocationManager::allocate` vérifie la capacité, enregistre l’allocation, décrémente le disponible ; pas de décision ALLOW/DENY (la décision est supposée déjà prise) | ✅ |
| Pas de persistance directe | Pas de DB/fichier | Aucun usage trouvé | ✅ |

**Verdict :** Conforme.

---

## 9. Synthèse

| Core | Conforme | Remarque |
|------|:--------:|-----------|
| BorderGuard | ✅ | Aucune violation détectée. |
| BondingBrother | ✅ | — |
| CaringNanny | ✅ | — |
| MasterButler | ✅ | — |
| EverBuddy | ✅ | — |
| WorrySentinel | ✅ | Conformité maximale : setters supprimés, API immuable (constructeurs uniquement). |
| TAMR | ✅ | — |
| LogisticsSteward | ✅ | — |

**Conformité maximale appliquée :** WorrySentinel ne expose plus aucune méthode `&mut self` modifiant un état. Configuration au constructeur uniquement (`new`, `with_signals` pour le ThreatDetector). L’adaptateur crée une nouvelle instance pour refléter une vue mise à jour.

---

## Références

- BorderGuard : `docs/cores/BorderGuard/implementation/Border Guard - Reference Implementation Guidelines.md`
- WorrySentinel : `docs/cores/WorrySentinel/implementation/WorrySentinel - Reference Implementation Guidelines.md`
- CaringNanny : `docs/cores/CaringNanny/implementation/Caring Nanny - Reference Implementation Guidelines.md`
- MasterButler : `docs/cores/MasterButler/implementation/Master Butler - Reference Implementation Guidelines.md`
- EverBuddy : `docs/cores/EverBuddy/implementation/Ever Buddy - Reference Implementation Guidelines.md`
