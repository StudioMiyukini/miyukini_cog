# StrongFather â€” Index de Navigation

## Contexte

StrongFather est le **moteur de dÃ©cision stratÃ©gique et politique** du Miyukini Core System. Il Ã©value des intentions, applique des politiques, Ã©tablit des prioritÃ©s, et produit des dÃ©cisions sans jamais possÃ©der d'autoritÃ© sur l'exÃ©cution ou la persistance.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## Structure de la documentation

### Foundation

Documents fondateurs dÃ©finissant l'identitÃ© et le rÃ´le de StrongFather.

| Document | Description |
|----------|-------------|
| [Documentation Fondatrice](./foundation/StrongFather%20-%20Documentation%20Fondatrice.md) | DÃ©finition conceptuelle, rÃ´le, positionnement, invariants fondamentaux |

---

### Contracts

Contrats FONDATION normatifs et non nÃ©gociables.

#### Decision
| Document | Description |
|----------|-------------|
| [Core Decision Contract](./contracts/decision/StrongFather%20-%20Core%20Decision%20Contract.md) | Types de dÃ©cisions, entrÃ©es/sorties, garanties dÃ©cisionnelles |
| [Decision Graph Specification](./contracts/decision/StrongFather%20-%20Decision%20Graph%20Specification.md) | Graphe de dÃ©cision, nÅ“uds, arÃªtes, parcours |

#### Intent
| Document | Description |
|----------|-------------|
| [Intent Model Contract](./contracts/intent/StrongFather%20-%20Intent%20Model%20Contract.md) | Structure des intentions, composants, cycle de vie |

#### Policy
| Document | Description |
|----------|-------------|
| [Policy Engine Contract](./contracts/policy/StrongFather%20-%20Policy%20Engine%20Contract.md) | Moteur de politiques, types, application, rÃ©solution |
| [Policy Source Contract](./contracts/policy/StrongFather%20-%20Policy%20Source%20Contract.md) | Source de politiques, chargement, validation |
| [Policy Language Specification](_index.md) | Syntaxe et sÃ©mantique du langage de politiques |

#### Boundaries
| Document | Description |
|----------|-------------|
| [Boundary & Isolation Contract](./contracts/boundaries/StrongFather%20-%20Boundary%20&%20Isolation%20Contract.md) | FrontiÃ¨res, relations autorisÃ©es/interdites, isolation |
| [Execution Prohibition Contract](./contracts/boundaries/StrongFather%20-%20Execution%20Prohibition%20Contract.md) | Interdictions absolues, distinction Ã©valuation/exÃ©cution |

#### Audit
| Document | Description |
|----------|-------------|
| [Audit & Trace Contract](./contracts/audit/StrongFather%20-%20Audit%20&%20Trace%20Contract.md) | TraÃ§abilitÃ©, traces, audit |
| [Error & Rejection Model](./contracts/audit/StrongFather%20-%20Error%20&%20Rejection%20Model.md) | Distinction erreur/rejet, catÃ©gories |

#### Governance
| Document | Description |
|----------|-------------|
| [Invariants & Guarantees](./contracts/governance/StrongFather%20-%20Invariants%20&%20Guarantees.md) | Catalogue consolidÃ© des invariants et garanties |
| [Violations & Anti-Patterns](./contracts/governance/StrongFather%20-%20Violations%20&%20Anti-Patterns.md) | Violations cataloguÃ©es, anti-patterns |
| [Conformance & Certification Rules](./contracts/governance/StrongFather%20-%20Conformance%20&%20Certification%20Rules.md) | CritÃ¨res de conformitÃ©, certification |

#### Integration
| Document | Description |
|----------|-------------|
| [LogisticsSteward Integration Contract](./contracts/integration/StrongFather%20-%20LogisticsSteward%20Integration%20Contract.md) | Validation des arbitrages de LogisticsSteward, rÃ©solution des conflits de rÃ¨gles |

#### Security
| Document | Description |
|----------|-------------|
| [Security & Threat Model Contract](_index.md) | ModÃ¨le de menace, implications sÃ©curitÃ©, adaptation niveaux T0-T4 |

---

### Architecture

Documentation architecturale.

| Document | Description |
|----------|-------------|
| [Architecture & Flows](./architecture/StrongFather%20-%20Architecture%20&%20Flows.md) | Architecture conceptuelle, composants, flux |
| [Integration Readiness Contract](./architecture/StrongFather%20-%20Integration%20Readiness%20Contract.md) | Conditions d'intÃ©gration, prÃ©requis |

---

### Lifecycle

Gestion du cycle de vie et de l'Ã©volution.

| Document | Description |
|----------|-------------|
| [Versioning & Evolution Contract](_index.md) | Versioning, compatibilitÃ©, dÃ©prÃ©ciation |
| [Release & Freeze Contract](_index.md) | Gel des contrats, inventaire |
| [Migration & Compatibility Contract](_index.md) | Migration progressive, rollback |

---

### Operations

Documentation opÃ©rationnelle.

| Document | Description |
|----------|-------------|
| [Operational Runbook](_index.md) | Guide SRE/Ops, dÃ©ploiement, monitoring |
| [Performance & Scalability Contract](_index.md) | Contraintes de performance, optimisations |

---

### Implementation

Guides d'implÃ©mentation.

| Document | Description |
|----------|-------------|
| [Implementation Overview](./implementation/guidelines/StrongFather%20-%20Implementation%20Overview.md) | Introduction, principes, traduction Rust |
| [Implementation Patterns](./implementation/guidelines/StrongFather%20-%20Implementation%20Patterns.md) | Patterns recommandÃ©s |
| [Implementation Prohibitions](./implementation/guidelines/StrongFather%20-%20Implementation%20Prohibitions.md) | Patterns interdits, piÃ¨ges |
| [Testing & Validation Contract](_index.md) | RÃ¨gles de test, validation |

---

### Reference

Documentation de rÃ©fÃ©rence et exemples.

| Document | Description |
|----------|-------------|
| [Examples - Intentions](./reference/examples/StrongFather%20-%20Examples%20Intentions.md) | Exemples d'intentions |
| [Examples - Policies](./reference/examples/StrongFather%20-%20Examples%20Policies.md) | Exemples de politiques |
| [Examples - Decisions](./reference/examples/StrongFather%20-%20Examples%20Decisions.md) | Exemples de dÃ©cisions |
| [FAQ & Common Questions](_index.md) | Questions frÃ©quentes |

---

## Invariants clÃ©s

| Invariant | Description |
|-----------|-------------|
| **INV-SF-1** | Aucune autoritÃ© sur l'exÃ©cution |
| **INV-SF-2** | Aucune autoritÃ© sur la persistance |
| **INV-SF-3** | Aucune modification d'Ã©tat |
| **INV-SF-4** | Aucune logique temporelle technique |
| **INV-SF-5** | Zero-trust |

---

## Relations avec les autres Cores

| Core | Relation |
|------|----------|
| **KindMother** | ComplÃ©mentaire â€” StrongFather dÃ©cide, KindMother persiste |
| **WorrySentinel** | Collaboration â€” RÃ¨gles de sÃ©curitÃ©, rÃ©vocation de mandats |
| **BondingBrother** | Interface â€” Communication mandatÃ©e entre OpÃ©rateurs |
| **LogisticsSteward** | AutoritÃ© â€” StrongFather valide/invalide les dÃ©cisions d'arbitrage de LogisticsSteward, tranche les conflits de rÃ¨gles |

---

**Date de crÃ©ation :** 2026-01-27  
**Version :** 1.0  
**Statut :** Index de navigation


