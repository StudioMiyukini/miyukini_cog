# Master Butler â€” Index de Navigation

## Contexte

Master Butler est le **Capability & Permission Core** du Miyukini Core System. Il incarne la connaissance de ce qui est possible : quelles capacitÃ©s existent dans le systÃ¨me, quelles permissions sont dÃ©finies, et quels droits peuvent Ãªtre accordÃ©s.

Master Butler rÃ©pond Ã  une question fondamentale : **"Quelles sont les capacitÃ©s du systÃ¨me, et quelles permissions existent pour y accÃ©der ?"**

**Strate :** 4 (Cores SystÃ¨me)  
**RÃ´le :** Registre des capacitÃ©s et permissions  
**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## Structure de la documentation

### Foundation

Documents fondateurs dÃ©finissant l'identitÃ© et le rÃ´le de Master Butler.

| Document | Description |
|----------|-------------|
| [Documentation Fondatrice](./foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md) | DÃ©finition conceptuelle, rÃ´le, positionnement, invariants fondamentaux |

---

### Contracts

Contrats FONDATION normatifs et non nÃ©gociables.

#### API

| Document | Description |
|----------|-------------|
| [Capability API Contract](./contracts/api/Master%20Butler%20-%20Capability%20API%20Contract.md) | DÃ©claration et interrogation des capacitÃ©s |
| [Permission API Contract](./contracts/api/Master%20Butler%20-%20Permission%20API%20Contract.md) | DÃ©finition et gestion des permissions |
| [Discovery API Contract](./contracts/api/Master%20Butler%20-%20Discovery%20API%20Contract.md) | API de dÃ©couverte des capacitÃ©s et permissions |

#### Registry

| Document | Description |
|----------|-------------|
| [Capability Registry Contract](./contracts/registry/Master%20Butler%20-%20Capability%20Registry%20Contract.md) | Structure et gestion du registre des capacitÃ©s |
| [Permission Registry Contract](./contracts/registry/Master%20Butler%20-%20Permission%20Registry%20Contract.md) | Structure et gestion du registre des permissions |
| [Association Model Contract](./contracts/registry/Master%20Butler%20-%20Association%20Model%20Contract.md) | ModÃ¨le d'association capacitÃ©s-permissions |

#### Tools

| Document | Description |
|----------|-------------|
| [Tool Governance Contract](./contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) | Gouvernance des Tools, dÃ©claration et catalogue |
| [Toolkit Composition Contract](./contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) | Composition et validation des Toolkits |

#### Integration

| Document | Description |
|----------|-------------|
| [StrongFather Integration Contract](./contracts/integration/Master%20Butler%20-%20StrongFather%20Integration%20Contract.md) | IntÃ©gration avec StrongFather pour les dÃ©cisions |
| [BondingBrother Integration Contract](./contracts/integration/Master%20Butler%20-%20BondingBrother%20Integration%20Contract.md) | IntÃ©gration avec BondingBrother pour la mÃ©diation |
| [LogisticsSteward Integration Contract](./contracts/integration/Master%20Butler%20-%20LogisticsSteward%20Integration%20Contract.md) | IntÃ©gration avec LogisticsSteward pour la gouvernance des ressources |
| [Operator Declaration Contract](./contracts/integration/Master%20Butler%20-%20Operator%20Declaration%20Contract.md) | DÃ©claration des capacitÃ©s par les OpÃ©rateurs |

#### Boundaries

| Document | Description |
|----------|-------------|
| [Boundary & Scope Contract](./contracts/boundaries/Master%20Butler%20-%20Boundary%20&%20Scope%20Contract.md) | FrontiÃ¨res et pÃ©rimÃ¨tre de Master Butler |
| [Authority Limits Contract](./contracts/boundaries/Master%20Butler%20-%20Authority%20Limits%20Contract.md) | Limites d'autoritÃ©, ce que Master Butler ne fait pas |

#### Observability

| Document | Description |
|----------|-------------|
| [Audit & Traceability Contract](./contracts/observability/Master%20Butler%20-%20Audit%20&%20Traceability%20Contract.md) | TraÃ§abilitÃ© des dÃ©clarations et modifications |
| [Observability Contract](./contracts/observability/Master%20Butler%20-%20Observability%20Contract.md) | MÃ©triques et monitoring du registre |

#### Security

| Document | Description |
|----------|-------------|
| [Threat Model Contract](./contracts/security/Master%20Butler%20-%20Threat%20Model%20Contract.md) | ModÃ¨le de menaces pour le registre |
| [Access Control Contract](./contracts/security/Master%20Butler%20-%20Access%20Control%20Contract.md) | ContrÃ´le d'accÃ¨s au registre |

#### Governance

| Document | Description |
|----------|-------------|
| [Invariants & Guarantees](./contracts/governance/Master%20Butler%20-%20Invariants%20&%20Guarantees.md) | Catalogue consolidÃ© des invariants et garanties |
| [Violations & Anti-Patterns](./contracts/governance/Master%20Butler%20-%20Violations%20&%20Anti-Patterns.md) | Violations cataloguÃ©es, anti-patterns |

---

### Architecture

Documentation architecturale.

| Document | Description |
|----------|-------------|
| [Architecture & Flows](./architecture/Master%20Butler%20-%20Architecture%20&%20Flows.md) | Architecture conceptuelle, composants, flux |
| [Internal State Machine](./architecture/Master%20Butler%20-%20Internal%20State%20Machine.md) | Machine Ã  Ã©tats interne du registre |

---

### Implementation

Guides d'implÃ©mentation.

| Document | Description |
|----------|-------------|
| [Reference Implementation Guidelines](./implementation/Master%20Butler%20-%20Reference%20Implementation%20Guidelines.md) | Guidelines d'implÃ©mentation de rÃ©fÃ©rence |
| [Implementation Patterns](_index.md) | Patterns recommandÃ©s |
| [Implementation Prohibitions](_index.md) | Patterns interdits, piÃ¨ges |

---

### Reference

Documentation de rÃ©fÃ©rence et exemples.

| Document | Description |
|----------|-------------|
| [Examples Capabilities](_index.md) | Exemples de dÃ©clarations de capacitÃ©s |
| [Examples Permissions](_index.md) | Exemples de dÃ©finitions de permissions |
| [FAQ & Common Questions](_index.md) | Questions frÃ©quentes |

---

## Invariants clÃ©s

| Invariant | Description |
|-----------|-------------|
| **INV-MB-1** | ExhaustivitÃ© â€” Toute capacitÃ© existante est recensÃ©e dans Master Butler |
| **INV-MB-2** | Non-dÃ©cision â€” Master Butler ne prend jamais de dÃ©cision |
| **INV-MB-3** | Idempotence â€” Les dÃ©clarations de capacitÃ©s sont idempotentes |
| **INV-MB-4** | ImmutabilitÃ© des identifiants â€” Les identifiants de capacitÃ©s ne changent jamais |
| **INV-MB-5** | TraÃ§abilitÃ© complÃ¨te â€” Toute modification du registre est tracÃ©e |
| **INV-MB-6** | SÃ©paration capacitÃ©/permission â€” CapacitÃ©s et permissions sont strictement sÃ©parÃ©es |
| **INV-MB-7** | Pas de logique mÃ©tier â€” Master Butler ne contient aucune logique mÃ©tier |
| **INV-MB-8** | AccessibilitÃ© universelle â€” Master Butler est accessible Ã  tous les composants autorisÃ©s |

---

## Interdictions

| Code | Interdiction |
|------|--------------|
| **INTERD-MB-1** | Master Butler ne peut pas dÃ©cider si une action est autorisÃ©e ou refusÃ©e |
| **INTERD-MB-2** | Master Butler ne peut pas vÃ©rifier les permissions en temps rÃ©el |
| **INTERD-MB-3** | Master Butler ne peut pas exÃ©cuter d'action fonctionnelle |
| **INTERD-MB-4** | Master Butler ne peut pas stocker de donnÃ©es mÃ©tier |
| **INTERD-MB-5** | Master Butler ne peut pas gÃ©rer les identitÃ©s |
| **INTERD-MB-6** | Master Butler ne peut pas dÃ©finir de politiques de dÃ©cision |
| **INTERD-MB-7** | Master Butler ne peut pas appliquer de contraintes mÃ©tier |
| **INTERD-MB-8** | Master Butler ne peut pas persister directement |

---

## Relations avec les autres Cores

| Core | Relation |
|------|----------|
| **StrongFather** | ComplÃ©mentaire â€” Master Butler expose, StrongFather dÃ©cide |
| **KindMother** | Support â€” Master Butler utilise KindMother pour la persistance du registre |
| **BondingBrother** | Interface â€” RÃ©pond aux interrogations pour la traduction des intentions |
| **WorrySentinel** | SÃ©curitÃ© â€” Validation des niveaux de sÃ©curitÃ© pour les Tools |
| **Caring Nanny** | Ã‰tat â€” CohÃ©rence d'Ã©tat pour l'utilisation des Tools |
| **Ever Buddy** | Lifecycle â€” Gestion du cycle de vie des Tools et versions |
| **LogisticsSteward** | Gouvernance â€” Master Butler expose les capacitÃ©s, LogisticsSteward limite leur usage |

### Diagramme de relations

```mermaid
graph TB
    subgraph Strate4[Strate 4 - Cores SystÃ¨me]
        SF[StrongFather<br/>DÃ©cision]
        KM[KindMother<br/>Persistance]
        MB[Master Butler<br/>CapacitÃ©s & Permissions]
        WS[WorrySentinel<br/>SÃ©curitÃ©]
    end

    subgraph Strate5[Strate 5 - Liaison]
        BB[BondingBrother<br/>MÃ©diation]
    end

    subgraph Strate3[Strate 3 - Gouvernance Ressources]
        LS[LogisticsSteward<br/>Arbitrage]
        CN[Caring Nanny<br/>Ã‰tat]
        EB[Ever Buddy<br/>Lifecycle]
    end

    SF -->|"Interroge capacitÃ©s"| MB
    BB -->|"Interroge permissions"| MB
    MB -->|"Persiste registre"| KM
    WS -.->|"Valide niveaux sÃ©curitÃ©"| MB
    EB -.->|"GÃ¨re versions Tools"| MB
    CN -.->|"Ã‰tats systÃ¨me"| MB
    LS -->|"Interroge capacitÃ©s disponibles"| MB
    LS -.->|"Limite usage capacitÃ©s"| MB

    classDef coreCapability fill:#fff9c4
    classDef coreData fill:#e1f5fe
    classDef coreDecision fill:#fff3e0
    classDef liaison fill:#f3e5f5
    classDef supervision fill:#e8f5e9
    classDef gouvernance fill:#ffe0b2

    class MB coreCapability
    class KM coreData
    class SF coreDecision
    class BB liaison
    class CN,EB supervision
    class LS gouvernance
```

---

## Gouvernance des Tools et Toolkits

Master Butler est le **catalogue central** des Tools et Toolkits. Il est responsable de :

| ResponsabilitÃ© | Description |
|----------------|-------------|
| **DÃ©clarer** | Quels Tools existent dans l'environnement |
| **Lier** | Capability â†’ Tool |
| **DÃ©finir les Toolkits** | Quels Tools composent chaque Toolkit |
| **Autoriser** | Qui peut appeler quel Tool/Toolkit |

**Documentation complÃ¨te :** [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)

---

## ConformitÃ© aux Lois d'Autonomie SystÃ¨me

Master Butler est **entiÃ¨rement conforme** aux [Lois d'Autonomie SystÃ¨me](..//..//miyukini-webway-system//reference//_index.md) :

| Loi | ConformitÃ© | Note |
|-----|------------|------|
| **LOI-1** | âœ… | Registre local, interrogations locales, aucune dÃ©pendance externe |
| **LOI-5** | âœ… | Registre pur de mÃ©tadonnÃ©es lÃ©gÃ¨res, pas de workers, consommation Ã  la demande |

---

## Concepts clÃ©s

| Concept | Description |
|---------|-------------|
| **CapacitÃ© (Capability)** | Pouvoir technique intrinsÃ¨que Ã  un composant |
| **Permission** | Droit accordÃ© pour accÃ©der Ã  une capacitÃ© |
| **Registre** | Structure centrale contenant capacitÃ©s et permissions |
| **DÃ©claration** | Acte d'informer Master Butler des capacitÃ©s d'un composant |
| **DÃ©finition** | Acte de crÃ©er une permission dans Master Butler |
| **Contexte de capacitÃ©** | Ensemble des capacitÃ©s/permissions dans une situation donnÃ©e |
| **Tool** | CapacitÃ© exÃ©cutable gouvernÃ©e |
| **Toolkit** | Composition officielle de Tools |

---

**Date de crÃ©ation :** 2026-01-27  
**Version :** 1.0  
**Statut :** Index de navigation


