---
name: Reorganisation StrongFather Docs
overview: Restructurer la documentation StrongFather (29 fichiers, ~19 000 lignes) en une arborescence logique organisée par domaine, en éliminant les redondances avec le glossaire global et en découpant les documents trop volumineux.
todos:
  - id: create-folders
    content: Créer la structure de dossiers (foundation, contracts, architecture, lifecycle, operations, implementation, reference)
    status: completed
  - id: delete-glossary
    content: Supprimer le glossaire StrongFather redondant et ajouter une note de référence vers le glossaire global
    status: completed
  - id: move-foundation
    content: Déplacer Documentation Fondatrice vers foundation/
    status: completed
  - id: move-contracts
    content: Déplacer les 11 contrats FONDATION vers leurs sous-dossiers (decision, intent, policy, boundaries, audit, governance)
    status: completed
  - id: move-architecture
    content: Déplacer Architecture & Flows et Integration Readiness vers architecture/
    status: completed
  - id: move-lifecycle
    content: Déplacer Versioning, Release & Freeze, Migration vers lifecycle/
    status: completed
  - id: move-operations
    content: Déplacer Operational Runbook, Performance, Security vers operations/
    status: completed
  - id: split-implementation
    content: Découper Reference Implementation Guidelines en 3 fichiers et déplacer vers implementation/guidelines/
    status: completed
  - id: split-examples
    content: Découper Examples & Use Cases en 3 fichiers et déplacer vers reference/examples/
    status: completed
  - id: move-reference
    content: Déplacer FAQ et Testing vers leurs dossiers
    status: completed
  - id: create-index
    content: Créer le fichier _index.md de navigation
    status: completed
  - id: archive-logs
    content: Archiver AUDIT_DOCUMENTATION.md et STRUCTURE_CREATION_LOG.md
    status: completed
  - id: update-links
    content: Mettre à jour toutes les références croisées dans les documents
    status: completed
  - id: validate
    content: Valider la cohérence globale et vérifier les liens
    status: completed
isProject: false
---

# Réorganisation de la documentation StrongFather

## Diagnostic

### Problèmes identifiés

**1. Structure plate non organisée**

- 29 fichiers dans un seul dossier sans hiérarchie
- Mélange de contrats FONDATION, guides opérationnels, FAQ et exemples

**2. Redondances majeures**

- Le [Glossaire StrongFather](docs/core/StrongFather/StrongFather%20—%20Glossary%20&%20Terminology.md) (1066 lignes) duplique largement le [Glossaire Miyukini](docs/reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) (1260 lignes)
- StrongFather y est déjà défini dans le glossaire global (lignes 862-884)

**3. Documents trop volumineux**

- Reference Implementation Guidelines : ~2091 lignes
- Policy Language Specification : ~1184 lignes
- Examples & Use Cases : ~1104 lignes

**4. Chevauchements conceptuels**

- Architecture & Flows ↔ Decision Graph Specification (flux d'évaluation)
- Release & Freeze ↔ Versioning & Evolution (règles de gel)

---

## Nouvelle structure proposée

```
docs/core/StrongFather/
├── _index.md                                    # Navigation
│
├── foundation/                                  # Documents fondateurs
│   └── StrongFather - Documentation Fondatrice.md
│
├── contracts/                                   # Contrats FONDATION
│   ├── decision/
│   │   ├── StrongFather - Core Decision Contract.md
│   │   └── StrongFather - Decision Graph Specification.md
│   ├── intent/
│   │   └── StrongFather - Intent Model Contract.md
│   ├── policy/
│   │   ├── StrongFather - Policy Engine Contract.md
│   │   ├── StrongFather - Policy Source Contract.md
│   │   └── StrongFather - Policy Language Specification.md
│   ├── boundaries/
│   │   ├── StrongFather - Boundary & Isolation Contract.md
│   │   └── StrongFather - Execution Prohibition Contract.md
│   ├── audit/
│   │   ├── StrongFather - Audit & Trace Contract.md
│   │   └── StrongFather - Error & Rejection Model.md
│   └── governance/
│       ├── StrongFather - Invariants & Guarantees.md
│       ├── StrongFather - Violations & Anti-Patterns.md
│       └── StrongFather - Conformance & Certification Rules.md
│
├── architecture/
│   ├── StrongFather - Architecture & Flows.md
│   └── StrongFather - Integration Readiness Contract.md
│
├── lifecycle/
│   ├── StrongFather - Versioning & Evolution Contract.md
│   ├── StrongFather - Release & Freeze Contract.md
│   └── StrongFather - Migration & Compatibility Contract.md
│
├── operations/
│   ├── StrongFather - Operational Runbook.md
│   ├── StrongFather - Performance & Scalability Contract.md
│   └── StrongFather - Security & Threat Model Contract.md
│
├── implementation/
│   ├── guidelines/
│   │   ├── StrongFather - Implementation Overview.md
│   │   ├── StrongFather - Implementation Patterns.md
│   │   └── StrongFather - Implementation Prohibitions.md
│   └── StrongFather - Testing & Validation Contract.md
│
└── reference/
    ├── examples/
    │   ├── StrongFather - Examples Intentions.md
    │   ├── StrongFather - Examples Policies.md
    │   └── StrongFather - Examples Decisions.md
    └── StrongFather - FAQ & Common Questions.md
```

---

## Actions principales

### Phase 1 : Préparation

- Créer la structure de dossiers
- Supprimer le glossaire StrongFather (redondant) et ajouter une référence au glossaire global

### Phase 2 : Déplacement des documents

- Déplacer chaque document dans son dossier approprié selon la nouvelle structure

### Phase 3 : Découpage des gros documents

- **Reference Implementation Guidelines** (~2091 lignes) devient 3 fichiers :
  - Implementation Overview (~300 lignes)
  - Implementation Patterns (~800 lignes)
  - Implementation Prohibitions (~500 lignes)
- **Examples & Use Cases** (~1104 lignes) devient 3 fichiers :
  - Examples Intentions (~350 lignes)
  - Examples Policies (~350 lignes)
  - Examples Decisions (~350 lignes)

### Phase 4 : Consolidation et corrections

- Consolider les doublons terminologiques en référençant le glossaire global
- Corriger les liens cassés après déplacement
- Créer le fichier `_index.md` de navigation

### Phase 5 : Nettoyage

- Supprimer les fichiers de log (AUDIT_DOCUMENTATION.md, STRUCTURE_CREATION_LOG.md) ou les archiver
- Vérifier la cohérence des références croisées

---

## Fichiers à supprimer ou archiver


| Fichier                                  | Action    | Raison                               |
| ---------------------------------------- | --------- | ------------------------------------ |
| StrongFather — Glossary & Terminology.md | Supprimer | Redondant avec glossaire global      |
| AUDIT_DOCUMENTATION.md                   | Archiver  | Document de travail, pas contractuel |
| STRUCTURE_CREATION_LOG.md                | Archiver  | Document de travail, pas contractuel |


---

## Références à mettre à jour

Dans le [Glossaire global](docs/reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md), la définition de StrongFather (lignes 862-884) est déjà complète. Les autres documents StrongFather doivent y faire référence plutôt que redéfinir les termes.

---

## Validation finale

- Vérifier que tous les invariants (INV-*) sont correctement référencés dans `Invariants & Guarantees.md`
- Vérifier que les liens relatifs fonctionnent après réorganisation
- S'assurer que le glossaire global est la source unique de terminologie

