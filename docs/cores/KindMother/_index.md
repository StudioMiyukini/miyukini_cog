# KindMother â€” Index de Navigation

## Contexte

KindMother est le **moteur interne de donnÃ©es** du Miyukini Core System. Il constitue la couche d'abstraction et d'orchestration de la persistance pour l'ensemble du systÃ¨me. KindMother gÃ¨re l'identitÃ© des instances de base de donnÃ©es (mÃ¨re et filles), garantit la cohÃ©rence des donnÃ©es, supporte le mode offline-first avec synchronisation automatique, et centralise la gestion des permissions conceptuelles.

**Strate :** 4 (Cores SystÃ¨me)  
**RÃ´le :** Persistance et gestion des donnÃ©es  
**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## Structure de la documentation

### Foundation

Documents fondateurs dÃ©finissant l'identitÃ© et le rÃ´le de KindMother.

| Document | Description |
|----------|-------------|
| [Documentation Fondatrice](./foundation/KindMother%20-%20Documentation%20Fondatrice.md) | DÃ©finition conceptuelle, rÃ´le, positionnement, responsabilitÃ©s |

---

### Contracts

Contrats FONDATION normatifs et non nÃ©gociables.

#### API

| Document | Description |
|----------|-------------|
| [CoreDataAPI Contract](./contracts/api/KindMother%20-%20CoreDataAPI%20Contract.md) | Contrat principal de l'API de donnÃ©es |
| [CoreDataAPI (_index.md).md) | Surface d'appel et opÃ©rations disponibles |
| [Interface & Contrat d'IntÃ©gration](_index.md) | Contrat d'intÃ©gration avec les adaptateurs produits |

#### Instance

| Document | Description |
|----------|-------------|
| [Instance Model Contract](./contracts/instance/KindMother%20-%20Instance%20Model%20Contract.md) | ModÃ¨le d'instances (DB MÃ¨re, DB Filles) |
| [Instance & Authority Domain Model Contract](./contracts/instance/KindMother%20-%20Instance%20&%20Authority%20Domain%20Model%20Contract.md) | Domaines d'autoritÃ© et relations entre instances |

#### Lifecycle

| Document | Description |
|----------|-------------|
| [Write Intent Lifecycle Contract](./contracts/lifecycle/KindMother%20-%20Write%20Intent%20Lifecycle%20Contract.md) | Cycle de vie des intentions d'Ã©criture |

#### Sync

| Document | Description |
|----------|-------------|
| [Sync & Conflict Resolution Contract](./contracts/sync/KindMother%20-%20Sync%20&%20Conflict%20Resolution%20Contract.md) | Synchronisation mÃ¨re/fille, rÃ©solution de conflits |

#### Authority

| Document | Description |
|----------|-------------|
| [Authority Graph & Cross-Domain Contract](./contracts/authority/KindMother%20-%20Authority%20Graph%20&%20Cross-Domain%20Contract.md) | Graphe d'autoritÃ©, relations inter-domaines |
| [Identity & Cross-Domain Trust Contract](./contracts/authority/KindMother%20-%20Identity%20&%20Cross-Domain%20Trust%20Contract.md) | IdentitÃ© des instances, confiance inter-domaines |

#### Boundaries

| Document | Description |
|----------|-------------|
| [Runtime Boundary & Enforcement Contract](./contracts/boundaries/KindMother%20-%20Runtime%20Boundary%20&%20Enforcement%20Contract.md) | FrontiÃ¨res d'exÃ©cution, application des rÃ¨gles |
| [Internal Boundary Contract](./contracts/boundaries/KindMother%20-%20Internal%20Boundary%20Contract.md) | FrontiÃ¨res internes, isolation des couches |

#### Persistence

| Document | Description |
|----------|-------------|
| [Persistence & Storage Contract](./contracts/persistence/KindMother%20-%20Persistence%20&%20Storage%20Contract.md) | Contrat de persistance, abstraction SQLite |

#### Security

| Document | Description |
|----------|-------------|
| [Threat Model & Attack Surface Contract](./contracts/security/KindMother%20-%20Threat%20Model%20&%20Attack%20Surface%20Contract.md) | ModÃ¨le de menaces, surface d'attaque |

#### Compliance

| Document | Description |
|----------|-------------|
| [Adapter Compliance Contract](./contracts/compliance/KindMother%20-%20Adapter%20Compliance%20Contract.md) | ConformitÃ© des adaptateurs produits |

#### Observability

| Document | Description |
|----------|-------------|
| [Observability & Audit Contract](./contracts/observability/KindMother%20-%20Observability%20&%20Audit%20Contract.md) | ObservabilitÃ©, traÃ§abilitÃ©, audit |
| [Failure & Degradation Contract](./contracts/observability/KindMother%20-%20Failure%20&%20Degradation%20Contract.md) | Gestion des pannes, modes dÃ©gradÃ©s |

---

### Architecture

Documentation architecturale.

| Document | Description |
|----------|-------------|
| [Internal State Machine (_index.md).md) | Machine Ã  Ã©tats interne (informatif) |
| [Core Server](./architecture/KindMother%20-%20Core%20Server.md) | **NOUVEAU** â€” Architecture du serveur isolÃ©, mÃ©caniques d'arbitrage, gestion multi-bases |
| [Client (DÃ©lÃ©gation)](./architecture/KindMother%20-%20Client.md) | **NOUVEAU** â€” Pattern de dÃ©lÃ©gation, API client, intÃ©gration dans les services COG |

---

### Implementation

Guides d'implÃ©mentation.

| Document | Description |
|----------|-------------|
| [Reference Implementation Guidelines](./implementation/KindMother%20-%20Reference%20Implementation%20Guidelines.md) | Guidelines d'implÃ©mentation de rÃ©fÃ©rence |
| [Systeme Persistance libSQL Migration](./implementation/KindMother%20-%20Systeme%20Persistance%20libSQL%20Migration.md) | **NOUVEAU** â€” Migration vers libSQL chiffrÃ©, architecture processus isolÃ©, guide technique complet |

---

### Reference

Documentation de rÃ©fÃ©rence et exemples.

| Document | Description |
|----------|-------------|
| [Adapter Examples (_index.md).md) | Exemples d'adaptateurs (non-normatif) |

---

## Invariants clÃ©s

| Invariant | Description |
|-----------|-------------|
| **INV-KM-1** | KindMother ne dÃ©cide jamais â€” la dÃ©cision appartient Ã  StrongFather |
| **INV-KM-2** | KindMother ne contient aucune logique mÃ©tier |
| **INV-KM-3** | SQLite est un dÃ©tail d'implÃ©mentation â€” jamais exposÃ© |
| **INV-KM-4** | Aucun module SPM ne parle directement Ã  une base de donnÃ©es |
| **INV-KM-5** | Offline-first est un principe fondamental non nÃ©gociable |

---

## Interdictions

| Code | Interdiction |
|------|--------------|
| **INTERD-KM-1** | KindMother ne peut pas prendre de dÃ©cisions stratÃ©giques |
| **INTERD-KM-2** | KindMother ne peut pas exposer SQLite ou ses schÃ©mas |
| **INTERD-KM-3** | KindMother ne peut pas bloquer le systÃ¨me en attente de rÃ©seau |
| **INTERD-KM-4** | KindMother ne peut pas contenir de logique mÃ©tier spÃ©cifique |

---

## Relations avec les autres Cores

| Core | Relation |
|------|----------|
| **StrongFather** | ComplÃ©mentaire â€” StrongFather dÃ©cide, KindMother persiste |
| **BondingBrother** | Interface â€” Traduction et dÃ©lÃ©gation via KindMother Integration Contract |
| **WorrySentinel** | SÃ©curitÃ© â€” RÃ©vocation de mandats, autoritÃ© sÃ©curitÃ© |
| **Caring Nanny** | Monitoring â€” DÃ©tection d'anomalies patterns KindMother |

### Diagramme de relations

```mermaid
graph TB
    subgraph Strate4[Strate 4 - Cores SystÃ¨me]
        SF[StrongFather<br/>DÃ©cision]
        KM[KindMother<br/>Persistance]
        WS[WorrySentinel<br/>SÃ©curitÃ©]
    end

    subgraph Strate5[Strate 5 - Liaison]
        BB[BondingBrother<br/>MÃ©diation]
    end

    subgraph Strate3[Strate 3 - Supervision]
        CN[Caring Nanny<br/>Monitoring]
    end

    BB -->|"DÃ©lÃ¨gue donnÃ©es (KM-DELEG-*)"| KM
    BB -->|"DÃ©lÃ¨gue dÃ©cisions"| SF
    SF -.->|"ComplÃ©mentaire (INV-SF-2)"| KM
    KM -.->|"Monitoring patterns"| CN
    WS -.->|"RÃ©vocation mandats"| KM

    classDef coreData fill:#e1f5fe
    classDef coreDecision fill:#fff3e0
    classDef liaison fill:#f3e5f5
    classDef supervision fill:#e8f5e9

    class KM coreData
    class SF coreDecision
    class BB liaison
    class CN supervision
```

---

## ConformitÃ© aux Lois d'Autonomie SystÃ¨me

KindMother est **entiÃ¨rement conforme** aux [Lois d'Autonomie SystÃ¨me](..//..//miyukini-webway-system//reference//_index.md) :

| Loi | ConformitÃ© | Note |
|-----|------------|------|
| **LOI-1** | âœ… | Persistance locale toujours disponible (offline-first) |
| **LOI-2** | âœ… | WriteIntent acceptÃ©s localement, synchronisÃ©s plus tard |
| **LOI-3** | âœ… | DB Fille souveraine localement |
| **LOI-4** | âœ… | Deltas et points de sync, pas de temps global |
| **LOI-5** | âœ… | SQLite optimisÃ© pour ressources limitÃ©es |
| **LOI-6** | âœ… | Synchronisation explicite et rÃ©versible |

---

## Concepts clÃ©s

| Concept | Description |
|---------|-------------|
| **DB MÃ¨re** | Source de vÃ©ritÃ© unique, autoritÃ© finale |
| **DB Fille** | Instance locale dÃ©rivÃ©e, mode offline-first |
| **WriteIntent** | Intention d'Ã©criture avant validation |
| **Delta** | DiffÃ©rence entre Ã©tats pour synchronisation |
| **Authority Domain** | Domaine d'autoritÃ© d'une instance |

---

**Date de crÃ©ation :** 2026-01-27  
**Version :** 1.1  
**Statut :** Index de navigation  
**DerniÃ¨re mise Ã  jour :** 2026-01-27 (correction chemins vers sous-dossiers)


