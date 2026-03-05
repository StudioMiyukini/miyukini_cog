# LogisticsSteward â€” Index de Navigation

## Contexte

LogisticsSteward est le **core responsable de la gouvernance de l'allocation, de la priorisation et de la limitation des ressources** au sein d'un environnement Miyukini. Il arbitre l'usage des ressources selon des rÃ¨gles explicites, des politiques dÃ©clarÃ©es et un Ã©tat systÃ¨me certifiÃ© â€” sans jamais les contrÃ´ler techniquement (sÃ©paration stricte avec le Kernel).

LogisticsSteward reprÃ©sente le **ministÃ¨re du budget et des ressources** du systÃ¨me : il connaÃ®t les rÃ¨gles d'allocation, il sait qui peut utiliser quoi, il dÃ©finit les prioritÃ©s et les limites â€” sans jamais mesurer, allouer, ou optimiser lui-mÃªme.

**Strate :** 3 (Gouvernance Ressources)  
**RÃ´le :** Arbitrage et gouvernance de l'allocation des ressources  
**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## Question fondamentale

> **"Qui a le droit d'utiliser quoi, quand, et Ã  quel niveau de prioritÃ© ?"**

Cette question se dÃ©cline en :
- Quels quotas s'appliquent Ã  chaque entitÃ© (opÃ©rateurs, Ã©quipes, services) ?
- Quelles prioritÃ©s relatives gouvernent l'arbitrage ?
- Quels plafonds d'usage sont en vigueur ?
- Quelle stratÃ©gie de dÃ©gradation appliquer en cas de surcharge ?

---

## Structure de la documentation

### Foundation

Documents fondateurs dÃ©finissant l'identitÃ© et le rÃ´le de LogisticsSteward.

| Document | Description |
|----------|-------------|
| [Documentation Fondatrice](./foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md) | DÃ©finition conceptuelle, rÃ´le, positionnement, invariants fondamentaux |

---

### Architecture

Documentation architecturale.

| Document | Description |
|----------|-------------|
| [Architecture & Flows](./architecture/LogisticsSteward%20-%20Architecture%20&%20Flows.md) | Architecture conceptuelle, composants, flux d'arbitrage |
| [Core Interaction Contract](./architecture/LogisticsSteward%20-%20Core%20Interaction%20Contract.md) | ModÃ¨le d'interaction avec les autres cores |

---

### Contracts

Contrats FONDATION normatifs et non nÃ©gociables.

#### Resources

| Document | Description |
|----------|-------------|
| [Quota Definition Contract](./contracts/resources/LogisticsSteward%20-%20Quota%20Definition%20Contract.md) | DÃ©finition formelle des quotas, types, rÃ¨gles d'attribution |
| [Priority Management Contract](./contracts/resources/LogisticsSteward%20-%20Priority%20Management%20Contract.md) | Niveaux de prioritÃ©, rÃ¨gles de prÃ©emption |
| [Resource Arbitration Contract](./contracts/resources/LogisticsSteward%20-%20Resource%20Arbitration%20Contract.md) | Processus d'arbitrage, entrÃ©es/sorties, garanties |

#### Degradation

| Document | Description |
|----------|-------------|
| [Degradation Strategy Contract](./contracts/degradation/LogisticsSteward%20-%20Degradation%20Strategy%20Contract.md) | Logique de dÃ©gradation contrÃ´lÃ©e, paliers, restauration |

#### Governance

| Document | Description |
|----------|-------------|
| [Invariants & Guarantees](./contracts/governance/LogisticsSteward%20-%20Invariants%20&%20Guarantees.md) | Catalogue consolidÃ© des invariants INV-LS-1 Ã  INV-LS-N |
| [Violations & Anti-Patterns](./contracts/governance/LogisticsSteward%20-%20Violations%20&%20Anti-Patterns.md) | Violations cataloguÃ©es, anti-patterns |

#### Integration

| Document | Description |
|----------|-------------|
| [Kernel Integration Contract](./contracts/integration/LogisticsSteward%20-%20Kernel%20Integration%20Contract.md) | Ã‰tat systÃ¨me abstrait, lecture seule, exÃ©cution des arbitrages |
| [StrongFather Integration Contract](./contracts/integration/LogisticsSteward%20-%20StrongFather%20Integration%20Contract.md) | Validation des arbitrages, rÃ©solution des conflits |
| [MasterButler Integration Contract](./contracts/integration/LogisticsSteward%20-%20MasterButler%20Integration%20Contract.md) | Limitation de l'usage des capacitÃ©s exposÃ©es |
| [WorrySentinel Integration Contract](./contracts/integration/LogisticsSteward%20-%20WorrySentinel%20Integration%20Contract.md) | Surveillance, dÃ©tection de dÃ©rives, durcissement |
| [BondingBrother Integration Contract](./contracts/integration/LogisticsSteward%20-%20BondingBrother%20Integration%20Contract.md) | Transport des dÃ©cisions d'arbitrage |
| [MiyukiniAdmin Integration Contract](_index.md) | RÃ¨gles spÃ©cifiques admin, prioritÃ©s maximales, exceptions |

#### Security

| Document | Description |
|----------|-------------|
| [Threat Model Contract](./contracts/security/LogisticsSteward%20-%20Threat%20Model%20Contract.md) | ModÃ¨le de menaces pour l'arbitrage des ressources |

---

### Implementation

Guides d'implÃ©mentation.

| Document | Description |
|----------|-------------|
| [Reference Implementation Guidelines](./implementation/LogisticsSteward%20-%20Reference%20Implementation%20Guidelines.md) | Guidelines d'implÃ©mentation de rÃ©fÃ©rence |

---

### Reference

Documentation de rÃ©fÃ©rence et exemples.

| Document | Description |
|----------|-------------|
| [Vocabulary & Glossary](./reference/LogisticsSteward%20-%20Vocabulary%20&%20Glossary.md) | Vocabulaire canonique de LogisticsSteward |
| [FAQ & Common Questions](./reference/LogisticsSteward%20-%20FAQ%20&%20Common%20Questions.md) | Questions frÃ©quentes |
| [Examples & Use Cases](./reference/LogisticsSteward%20-%20Examples%20&%20Use%20Cases.md) | Exemples et cas d'usage |

---

## Invariants clÃ©s

| Invariant | Description |
|-----------|-------------|
| **INV-LS-1** | Aucune capacitÃ© d'exÃ©cution â€” LogisticsSteward ne mesure pas, n'alloue pas, ne planifie pas |
| **INV-LS-2** | Aucune autoritÃ© technique â€” Pas de contrÃ´le sur CPU, mÃ©moire, rÃ©seau, IO |
| **INV-LS-3** | Lecture seule sur l'Ã©tat systÃ¨me â€” ReÃ§oit un Ã©tat certifiÃ© du Kernel, ne le modifie jamais |
| **INV-LS-4** | Aucune optimisation locale â€” Gouverne par rÃ¨gles explicites, pas par heuristiques |
| **INV-LS-5** | DÃ©cisions dÃ©terministes â€” Ã€ entrÃ©es identiques, arbitrage identique |
| **INV-LS-6** | TraÃ§abilitÃ© complÃ¨te â€” Toute dÃ©cision d'arbitrage est traÃ§able avec origine et justification |
| **INV-LS-7** | SÃ©paration gouvernance/exÃ©cution â€” LogisticsSteward dÃ©cide, Kernel exÃ©cute |

---

## Interdictions

| Code | Interdiction |
|------|--------------|
| **INTERD-LS-1** | LogisticsSteward ne peut pas mesurer les ressources systÃ¨me |
| **INTERD-LS-2** | LogisticsSteward ne peut pas allouer de mÃ©moire |
| **INTERD-LS-3** | LogisticsSteward ne peut pas planifier de threads |
| **INTERD-LS-4** | LogisticsSteward ne peut pas piloter de scheduler bas niveau |
| **INTERD-LS-5** | LogisticsSteward ne peut pas optimiser l'exÃ©cution |
| **INTERD-LS-6** | LogisticsSteward ne peut pas persister de donnÃ©es directement |
| **INTERD-LS-7** | LogisticsSteward ne peut pas auto-appliquer ses dÃ©cisions |

---

## Types de rÃ¨gles gÃ©rÃ©es

| Type | Description |
|------|-------------|
| **Quotas** | Limites conceptuelles d'usage de ressources par entitÃ© |
| **PrioritÃ©s** | Niveaux relatifs dÃ©terminant l'ordre d'arbitrage |
| **Plafonds** | Limites maximales absolues non dÃ©passables |
| **Restrictions** | Limitations temporaires contextuelles |
| **DÃ©gradations** | Politiques de rÃ©duction contrÃ´lÃ©e des capacitÃ©s |

---

## EntitÃ©s concernÃ©es

| EntitÃ© | Description |
|--------|-------------|
| **OpÃ©rateurs** | Acteurs individuels du systÃ¨me |
| **Ã‰quipes d'opÃ©rateurs** | Groupes d'opÃ©rateurs partageant des rÃ¨gles |
| **Outils / Toolkits** | Services ou ensembles de services |
| **Services exposÃ©s** | Points d'accÃ¨s utilisateur final |
| **MiyukiniAdmin** | Administration systÃ¨me (rÃ¨gles spÃ©cifiques) |

---

## Relations avec les autres Cores

| Core | Relation |
|------|----------|
| **Kernel** | Fournisseur â€” Kernel fournit l'Ã©tat systÃ¨me abstrait (lecture seule), exÃ©cute les arbitrages dÃ©cidÃ©s |
| **StrongFather** | Validateur â€” StrongFather valide/invalide les dÃ©cisions d'arbitrage, tranche en cas de conflit |
| **MasterButler** | Limitation â€” MasterButler expose les capacitÃ©s, LogisticsSteward limite leur usage (pas leur existence) |
| **WorrySentinel** | Surveillance â€” WorrySentinel supervise les dÃ©rives, peut invalider un Ã©tat jugÃ© incohÃ©rent, dÃ©clenche durcissement |
| **BondingBrother** | Transport â€” BondingBrother transporte les dÃ©cisions d'arbitrage sans les interprÃ©ter |
| **MiyukiniAdmin** | GouvernÃ© spÃ©cial â€” MiyukiniAdmin peut obtenir prioritÃ©s maximales, soumis Ã  gouvernance sauf protocole d'exception |

### Diagramme de relations

```mermaid
graph TB
    subgraph Strate4[Strate 4 - Cores SystÃ¨me]
        SF[StrongFather<br/>Validation]
        KM[KindMother<br/>Persistance]
        WS[WorrySentinel<br/>Surveillance]
    end

    subgraph Strate3[Strate 3 - Gouvernance Ressources]
        LS[LogisticsSteward<br/>Arbitrage]
    end

    subgraph Strate2[Strate 2 - CapacitÃ©s]
        MB[MasterButler<br/>CapacitÃ©s]
    end

    subgraph Strate5[Strate 5 - Liaison]
        BB[BondingBrother<br/>Transport]
    end

    subgraph Kernel[Kernel]
        K[Kernel<br/>Ã‰tat systÃ¨me]
    end

    subgraph Admin[Administration]
        MA[MiyukiniAdmin<br/>PrioritÃ©s spÃ©ciales]
    end

    K -->|"Ã©tat systÃ¨me abstrait"| LS
    LS -->|"dÃ©cisions Ã  valider"| SF
    SF -->|"validation/invalidation"| LS
    LS -->|"limite usage"| MB
    WS -->|"surveillance, durcissement"| LS
    LS -->|"dÃ©cisions d'arbitrage"| BB
    LS -->|"rÃ¨gles spÃ©cifiques"| MA

    classDef coreGouvernance fill:#e8f5e9
    classDef coreData fill:#e1f5fe
    classDef coreDecision fill:#fff3e0
    classDef liaison fill:#f3e5f5
    classDef supervision fill:#fce4ec
    classDef kernel fill:#e0e0e0
    classDef admin fill:#ede7f6

    class LS coreGouvernance
    class KM coreData
    class SF coreDecision
    class BB liaison
    class WS supervision
    class K kernel
    class MA admin
    class MB coreDecision
```

---

## ConformitÃ© aux Lois d'Autonomie SystÃ¨me

LogisticsSteward est **critique pour l'autonomie** selon les [Lois d'Autonomie SystÃ¨me](..//..//miyukini-webway-system//reference//_index.md) :

| Loi | ConformitÃ© | Note |
|-----|------------|------|
| **LOI-1** | âœ… RÃ´le critique | Gouvernance locale des ressources, rÃ¨gles chargÃ©es au dÃ©marrage |
| **LOI-2** | âœ… | Permet la gestion des ressources en environnement isolÃ© |
| **LOI-3** | âœ… | Politiques d'arbitrage locales et souveraines |
| **LOI-5** | âœ… | Core conceptuel lÃ©ger, sans exÃ©cution technique, optimisÃ© ressources |

---

## Concepts clÃ©s

| Concept | Description |
|---------|-------------|
| **Quota** | Limite conceptuelle d'usage attribuÃ©e Ã  une entitÃ© |
| **PrioritÃ©** | Niveau relatif dÃ©terminant l'ordre d'accÃ¨s aux ressources |
| **Arbitrage** | Processus de dÃ©cision sur l'allocation selon rÃ¨gles et Ã©tat |
| **Ã‰tat systÃ¨me abstrait** | ReprÃ©sentation normalisÃ©e de l'Ã©tat rÃ©el fournie par le Kernel |
| **DÃ©gradation contrÃ´lÃ©e** | RÃ©duction volontaire et prÃ©visible des capacitÃ©s |
| **Plafond** | Limite maximale absolue d'usage |
| **PrÃ©emption** | CapacitÃ© Ã  interrompre un usage au profit d'une prioritÃ© supÃ©rieure |

---

## Phrase fondatrice

> **LogisticsSteward est le core qui empÃªche le chaos silencieux. Il garantit que aucun acteur ne prend trop, le systÃ¨me reste stable, la performance est prÃ©servÃ©e par la gouvernance, et la dÃ©gradation est un choix, pas un accident. Il ne connaÃ®t pas le hardware. Il connaÃ®t les limites.**

---

## Documents de rÃ©fÃ©rence

- [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Pyramide Architecture Complete](..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Definition COG](..//..//miyukini-webway-system//reference//_index.md)

---

**Date de crÃ©ation :** 2026-01-28  
**Version :** 0.1 (Draft)  
**Statut :** Index de navigation â€” En cours de documentation


