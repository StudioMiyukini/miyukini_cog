# StrongFather — Index de Navigation

## Contexte

StrongFather est le **moteur de décision stratégique et politique** du Miyukini Core System. Il évalue des intentions, applique des politiques, établit des priorités, et produit des décisions sans jamais posséder d'autorité sur l'exécution ou la persistance.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## Structure de la documentation

### Foundation

Documents fondateurs définissant l'identité et le rôle de StrongFather.

| Document | Description |
|----------|-------------|
| [Documentation Fondatrice](./foundation/StrongFather%20-%20Documentation%20Fondatrice.md) | Définition conceptuelle, rôle, positionnement, invariants fondamentaux |

---

### Contracts

Contrats FONDATION normatifs et non négociables.

#### Decision
| Document | Description |
|----------|-------------|
| [Core Decision Contract](./contracts/decision/StrongFather%20-%20Core%20Decision%20Contract.md) | Types de décisions, entrées/sorties, garanties décisionnelles |
| [Decision Graph Specification](./contracts/decision/StrongFather%20-%20Decision%20Graph%20Specification.md) | Graphe de décision, nœuds, arêtes, parcours |

#### Intent
| Document | Description |
|----------|-------------|
| [Intent Model Contract](./contracts/intent/StrongFather%20-%20Intent%20Model%20Contract.md) | Structure des intentions, composants, cycle de vie |

#### Policy
| Document | Description |
|----------|-------------|
| [Policy Engine Contract](./contracts/policy/StrongFather%20-%20Policy%20Engine%20Contract.md) | Moteur de politiques, types, application, résolution |
| [Policy Source Contract](./contracts/policy/StrongFather%20-%20Policy%20Source%20Contract.md) | Source de politiques, chargement, validation |
| [Policy Language Specification](./contracts/policy/StrongFather%20—%20Policy%20Language%20Specification.md) | Syntaxe et sémantique du langage de politiques |

#### Boundaries
| Document | Description |
|----------|-------------|
| [Boundary & Isolation Contract](./contracts/boundaries/StrongFather%20-%20Boundary%20&%20Isolation%20Contract.md) | Frontières, relations autorisées/interdites, isolation |
| [Execution Prohibition Contract](./contracts/boundaries/StrongFather%20-%20Execution%20Prohibition%20Contract.md) | Interdictions absolues, distinction évaluation/exécution |

#### Audit
| Document | Description |
|----------|-------------|
| [Audit & Trace Contract](./contracts/audit/StrongFather%20-%20Audit%20&%20Trace%20Contract.md) | Traçabilité, traces, audit |
| [Error & Rejection Model](./contracts/audit/StrongFather%20-%20Error%20&%20Rejection%20Model.md) | Distinction erreur/rejet, catégories |

#### Governance
| Document | Description |
|----------|-------------|
| [Invariants & Guarantees](./contracts/governance/StrongFather%20-%20Invariants%20&%20Guarantees.md) | Catalogue consolidé des invariants et garanties |
| [Violations & Anti-Patterns](./contracts/governance/StrongFather%20-%20Violations%20&%20Anti-Patterns.md) | Violations cataloguées, anti-patterns |
| [Conformance & Certification Rules](./contracts/governance/StrongFather%20-%20Conformance%20&%20Certification%20Rules.md) | Critères de conformité, certification |

#### Integration
| Document | Description |
|----------|-------------|
| [LogisticsSteward Integration Contract](./contracts/integration/StrongFather%20-%20LogisticsSteward%20Integration%20Contract.md) | Validation des arbitrages de LogisticsSteward, résolution des conflits de règles |

#### Security
| Document | Description |
|----------|-------------|
| [Security & Threat Model Contract](./contracts/security/StrongFather%20—%20Security%20&%20Threat%20Model%20Contract.md) | Modèle de menace, implications sécurité, adaptation niveaux T0-T4 |

---

### Architecture

Documentation architecturale.

| Document | Description |
|----------|-------------|
| [Architecture & Flows](./architecture/StrongFather%20-%20Architecture%20&%20Flows.md) | Architecture conceptuelle, composants, flux |
| [Integration Readiness Contract](./architecture/StrongFather%20-%20Integration%20Readiness%20Contract.md) | Conditions d'intégration, prérequis |

---

### Lifecycle

Gestion du cycle de vie et de l'évolution.

| Document | Description |
|----------|-------------|
| [Versioning & Evolution Contract](./lifecycle/StrongFather%20—%20Versioning%20&%20Evolution%20Contract.md) | Versioning, compatibilité, dépréciation |
| [Release & Freeze Contract](./lifecycle/StrongFather%20—%20Release%20&%20Freeze%20Contract.md) | Gel des contrats, inventaire |
| [Migration & Compatibility Contract](./lifecycle/StrongFather%20—%20Migration%20&%20Compatibility%20Contract.md) | Migration progressive, rollback |

---

### Operations

Documentation opérationnelle.

| Document | Description |
|----------|-------------|
| [Operational Runbook](./operations/StrongFather%20—%20Operational%20Runbook.md) | Guide SRE/Ops, déploiement, monitoring |
| [Performance & Scalability Contract](./operations/StrongFather%20—%20Performance%20&%20Scalability%20Contract.md) | Contraintes de performance, optimisations |

---

### Implementation

Guides d'implémentation.

| Document | Description |
|----------|-------------|
| [Implementation Overview](./implementation/guidelines/StrongFather%20-%20Implementation%20Overview.md) | Introduction, principes, traduction Rust |
| [Implementation Patterns](./implementation/guidelines/StrongFather%20-%20Implementation%20Patterns.md) | Patterns recommandés |
| [Implementation Prohibitions](./implementation/guidelines/StrongFather%20-%20Implementation%20Prohibitions.md) | Patterns interdits, pièges |
| [Testing & Validation Contract](./implementation/StrongFather%20—%20Testing%20&%20Validation%20Contract.md) | Règles de test, validation |

---

### Reference

Documentation de référence et exemples.

| Document | Description |
|----------|-------------|
| [Examples - Intentions](./reference/examples/StrongFather%20-%20Examples%20Intentions.md) | Exemples d'intentions |
| [Examples - Policies](./reference/examples/StrongFather%20-%20Examples%20Policies.md) | Exemples de politiques |
| [Examples - Decisions](./reference/examples/StrongFather%20-%20Examples%20Decisions.md) | Exemples de décisions |
| [FAQ & Common Questions](./reference/StrongFather%20—%20FAQ%20&%20Common%20Questions.md) | Questions fréquentes |

---

## Invariants clés

| Invariant | Description |
|-----------|-------------|
| **INV-SF-1** | Aucune autorité sur l'exécution |
| **INV-SF-2** | Aucune autorité sur la persistance |
| **INV-SF-3** | Aucune modification d'état |
| **INV-SF-4** | Aucune logique temporelle technique |
| **INV-SF-5** | Zero-trust |

---

## Relations avec les autres Cores

| Core | Relation |
|------|----------|
| **KindMother** | Complémentaire — StrongFather décide, KindMother persiste |
| **WorrySentinel** | Collaboration — Règles de sécurité, révocation de mandats |
| **BondingBrother** | Interface — Communication mandatée entre Opérateurs |
| **LogisticsSteward** | Autorité — StrongFather valide/invalide les décisions d'arbitrage de LogisticsSteward, tranche les conflits de règles |

---

**Date de création :** 2026-01-27  
**Version :** 1.0  
**Statut :** Index de navigation
