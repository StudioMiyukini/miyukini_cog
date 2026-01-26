# KindMother - Moteur de Données Autoritaire

Moteur interne de données du Miyukini Core System v2.4.

## Statut : ✅ Clôture Complète

Ce crate implémente KindMother, le moteur de données autoritaire offline-first du Miyukini Core System.

### Fonctionnalités implémentées

- ✅ **Write Intent Lifecycle** — Cycle de vie complet des intentions d'écriture
- ✅ **Runtime Boundaries** — Validation structurelle de toutes les opérations
- ✅ **Permission Boundary** — Permissions conceptuelles (read, write, sync)
- ✅ **Persistance interne** — Storage opaque avec atomicité et détection de corruption
- ✅ **Sync & Conflict Resolution** — Détection de conflits (autoritaire, temporel, sémantique)
- ✅ **Threat Model Enforcement** — Détection de bypass, replay, saturation, brute-force
- ✅ **Observabilité** — Journal d'intention, rejets, quarantaines
- ✅ **Dégradation contrôlée** — États Degraded et Quarantined avec transitions explicites
- ✅ **Machine d'état KM** — Booting, Healthy, Degraded, Quarantined

### Caractéristiques clés

- **Offline-first** — Aucune dépendance réseau
- **Autorité locale** — KindMother est l'unique médiateur des données
- **Rejet explicite** — Toute opération non autorisée est explicitement rejetée
- **Traçabilité** — Toutes les décisions sont traçables
- **Pas d'auto-réparation** — Dégradation contrôlée sans escalade implicite

## Structure

```
src/
├── lib.rs          # Point d'entrée principal, exports publics
├── core.rs         # KindMother, WriteIntent, Contexts
├── api.rs          # CoreDataAPI (surface d'appel unique)
├── runtime.rs      # Runtime Boundaries (validation)
├── state.rs        # Machine d'état (KMState)
├── lifecycle.rs    # Write Intent Lifecycle
├── sync.rs         # SyncIntent, ConflictDetector, SyncManager
├── threat.rs       # ThreatDetector, types de menaces
├── observability.rs # Événements, journaux, audit
├── storage.rs      # Storage interne (privé)
└── errors.rs       # Types d'erreurs (KMError)

tests/
├── console_tests.rs  # Tests de base
└── unit_tests.rs     # Tests unitaires complets (56 tests)

examples/
├── demo.rs                    # Démonstration basique
├── lifecycle_test.rs          # Test cycle de vie WriteIntent
├── runtime_boundaries_test.rs # Test Runtime Boundaries
├── permissions_test.rs        # Test Permission Boundary
├── persistence_test.rs        # Test persistance
├── sync_test.rs               # Test synchronisation et conflits
├── threat_detection_test.rs   # Test détection de menaces
├── corruption_test.rs         # Test corruption et dégradation
├── observability_test.rs      # Test observabilité
└── offline_first_test.rs      # Test scénario offline-first
```

## Utilisation

### Exemple basique

```rust
use kindmother::*;

// Création du moteur
let mut km = KindMother::with_identity("instance-1".to_string(), "domain-1".to_string());

// Transition vers Healthy
km.recover_to_healthy("Démarrage réussi").unwrap();

// Création et traitement d'une intention
let mut intent = WriteIntent::new("intent-1".to_string(), "create".to_string());
intent.start_validation().unwrap();
intent.accept().unwrap();
intent.apply().unwrap();

// Persistance
km.persist_intent(intent, "domain-1").unwrap();

// Observabilité
println!("Événements: {}", km.observability().event_count());
```

### Exécuter les tests

```bash
# Tous les tests (75 tests)
cargo test --package kindmother

# Tests unitaires uniquement
cargo test --package kindmother --test unit_tests

# Un exemple spécifique
cargo run --example sync_test --package kindmother
```

### Exécuter un exemple

```bash
cargo run --example demo --package kindmother
cargo run --example offline_first_test --package kindmother
```

## Contraintes absolues respectées

- ❌ **Aucun panic** — Pas de `panic!()` dans le code de production
- ❌ **Aucun unwrap sauvage** — Gestion d'erreurs explicite
- ❌ **Aucun accès direct DB** — Storage opaque
- ❌ **Aucun réseau** — Offline-first
- ❌ **Aucun protocole** — Pas de sérialisation réseau
- ❌ **Aucune logique métier** — Validation structurelle uniquement
- ❌ **Aucune autorité implicite** — Permissions explicites
- ✅ **Rejets explicites** — Avec KMError et raison
- ✅ **Rejets traçables** — Via RejectionLog
- ✅ **Rejets justifiés** — Raison dans chaque erreur

## Documentation

- **MINI_LOG.md** — Décisions d'implémentation et arbitrages contractuels
- **AUDIT_FINAL.md** — Audit de conformité aux contrats FONDATION

## Verdict d'audit

**✅ CONFORME** aux contrats FONDATION.

Voir `AUDIT_FINAL.md` pour le détail complet.

---

**Version :** 0.2.0 (Clôture complète)  
**Date :** 2026-01-25  
**Contrats :** Conformité aux contrats FONDATION KindMother
