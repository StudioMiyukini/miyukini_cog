# WorrySentinel - Core Interaction Contract

## 1. Contexte

Ce document formalise les **interactions de WorrySentinel avec les autres Cores** du Miyukini Core System. Il définit les contrats d'interface, les flux d'échange, et les responsabilités de chaque partie dans les interactions impliquant la gouvernance de sécurité.

WorrySentinel, en tant que **core de gouvernance transversale** (Strate 4 — Gouvernance de sécurité), interagit avec tous les autres cores selon deux flux distincts :
- **Flux descendant (gouvernance)** : WorrySentinel impose des contraintes verticales sur les cores fonctionnels
- **Flux montant (observation)** : WorrySentinel observe et corrèle les signaux remontant des cores

**Document de référence :** [WorrySentinel - Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)

---

## 2. Portée / Scope

- **Applicable à :** Toute interaction entre WorrySentinel et les autres cores
- **Audience :** Architectes, développeurs, intégrateurs
- **Statut :** Document contractuel normatif — CONTRAT D'INTERACTION

---

## 3. Principes généraux d'interaction

### 3.1 Nature des relations

WorrySentinel entretient des relations avec les autres cores qui suivent des patterns spécifiques :

| Pattern | Description | Cores concernés |
|---------|-------------|-----------------|
| **Gouvernance** | WorrySentinel impose des contraintes sécuritaires | StrongFather, MasterButler, BorderGuard, LogisticsSteward |
| **Observation** | WorrySentinel reçoit des signaux pour évaluer l'état | Kernel, CaringNanny, KindMother, BondingBrother |
| **Escalade** | WorrySentinel signale le besoin d'intervention humaine | TAMR |
| **Exposition** | WorrySentinel expose la gouvernance pour consultation | MiyukiniAdmin |

### 3.2 Invariants d'interaction

**INV-INT-WS-1 : WorrySentinel gouverne mais n'exécute jamais**

WorrySentinel impose des contraintes, définit des niveaux, déclare des états, mais n'exécute jamais d'action technique. L'exécution est toujours du ressort des cores fonctionnels.

**INV-INT-WS-2 : WorrySentinel n'implémente jamais**

WorrySentinel ne définit jamais de mécanisme cryptographique, d'algorithme de sécurité, ou de contrôle technique. Il gouverne le "quoi" mais jamais le "comment".

**INV-INT-WS-3 : Flux explicites et traçables**

Chaque interaction a une direction explicite. Les flux bidirectionnels sont documentés comme deux flux unidirectionnels distincts avec traçabilité complète.

**INV-INT-WS-4 : Aucune modification d'état par WorrySentinel**

WorrySentinel ne modifie jamais directement l'état des autres cores. Il déclare des contraintes que les cores doivent appliquer eux-mêmes.

**INV-INT-WS-5 : Pression verticale, pas remplacement**

WorrySentinel agit comme une pression verticale sur les cores fonctionnels. Il contraint sans remplacer, gouverne sans se substituer aux responsabilités des autres cores.

---

## 4. Flux d'interaction globaux

### 4.1 Flux descendant — Gouvernance

WorrySentinel impose des contraintes verticales sur tous les cores fonctionnels :

```
                    WorrySentinel
                         │
                         │ impose contraintes
                         ▼
    ┌────────────────────┼────────────────────┐
    │                    │                    │
    ▼                    ▼                    ▼
┌──────────┐      ┌──────────┐      ┌──────────┐
│StrongFather│     │MasterButler│    │BorderGuard│
│ sévérité  │     │permissions │    │durcissement│
│ décisions │     │ actives    │    │   I/O      │
└──────────┘      └──────────┘      └──────────┘
    │                    │                    │
    ▼                    ▼                    ▼
┌──────────┐      ┌──────────┐      ┌──────────┐
│Logistics │      │   TAMR    │      │  Kernel  │
│ Steward  │      │  droits   │      │ fréquence│
│ quotas   │      │  humains  │      │  sondes  │
└──────────┘      └──────────┘      └──────────┘
```

**Principe :** WorrySentinel ne remplace rien. Il contraint tout.

### 4.2 Flux montant — Observation

WorrySentinel observe et corrèle les signaux remontant des cores :

```
┌──────────┐      ┌──────────┐      ┌──────────┐
│  Kernel  │      │BorderGuard│     │StrongFather│
│ signaux  │      │ anomalies │     │ décisions │
│clock, id │      │    I/O    │     │  refusées │
└────┬─────┘      └────┬─────┘      └────┬─────┘
     │                 │                 │
     │                 ▼                 │
     │          ┌──────────┐            │
     │          │KindMother │            │
     │          │incohérences│           │
     │          └────┬─────┘            │
     │               │                   │
     ▼               ▼                   ▼
    ┌────────────────┼────────────────────┐
    │                │                    │
    │                ▼                    │
    │     ┌────────────────────┐         │
    │     │  BondingBrother    │         │
    │     │  comportements     │         │
    │     │    produits        │         │
    │     └────────┬───────────┘         │
    │              │                      │
    │              ▼                      │
    │     ┌────────────────────┐         │
    │     │   CaringNanny      │         │
    │     │   consolidation    │         │
    │     └────────┬───────────┘         │
    │              │                      │
    └──────────────┼──────────────────────┘
                   │
                   ▼
              WorrySentinel
           observe, corrèle,
           déclare un état
```

**Principe :** WorrySentinel observe, corrèle, et déclare un état global basé sur les signaux consolidés.

---

## 5. Relations avec chaque Core

### 5.1 Relation avec le Kernel

**Type de relation :** Observation

**Principe fondamental :**

> Le Kernel fournit les signaux de base (clock, id, traces). WorrySentinel observe ces signaux pour évaluer l'état du système mais n'utilise jamais le Kernel directement pour sa logique de gouvernance.

**Responsabilités respectives :**

| Aspect | Kernel | WorrySentinel |
|--------|--------|---------------|
| Fourniture de signaux | ✅ Autorité | ❌ Consommateur |
| Horloge logique | ✅ Source | ❌ Utilisateur (traçabilité) |
| Génération d'identifiants | ✅ Autorité | ❌ Utilisateur (audit) |
| Fréquence des sondes | ✅ Exécution | ✅ Gouvernance |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│   Kernel    │  Signaux systeme    │WorrySentinel│
│             │ ──────────────────► │             │
│             │                      │             │
│             │  Contrainte fréquence│             │
│             │ ◄────────────────── │             │
│             │                      │             │
│ (exécute)   │                      │ (gouverne)  │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| K → WS | Signaux système (anomalies, métriques) | `SystemSignal` |
| K → WS | État des sondes | `ProbeStatus` |
| WS → K | Fréquence de sondage requise | `ProbeFrequencyConstraint` |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-K-1** | WorrySentinel n'appelle jamais directement le Kernel pour ses décisions de gouvernance |
| **COL-K-2** | Le Kernel exécute les contraintes de fréquence imposées par WorrySentinel |
| **COL-K-3** | Les signaux du Kernel sont une source d'observation, pas une dépendance fonctionnelle |
| **COL-K-4** | En mode isolé, WorrySentinel fonctionne sans signaux du Kernel (dégradation gracieuse) |

---

### 5.2 Relation avec StrongFather

**Type de relation :** Gouvernance

**Principe fondamental :**

> StrongFather décide si une action est autorisée. WorrySentinel gouverne la sévérité selon laquelle StrongFather doit décider, en fonction du niveau de sécurité et de l'état de confiance.

**Responsabilités respectives :**

| Aspect | StrongFather | WorrySentinel |
|--------|--------------|---------------|
| Décision d'autorisation | ✅ Autorité | ❌ Aucune |
| Sévérité des politiques | ❌ Exécution | ✅ Gouvernance |
| Évaluation des intentions | ✅ Autorité | ❌ Aucune |
| Niveau de sécurité applicable | ❌ Consommateur | ✅ Fournisseur |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│WorrySentinel│  Niveau sécurité +   │ StrongFather│
│             │  État confiance      │             │
│             │ ──────────────────► │             │
│             │                      │             │
│             │  Décisions refusées  │             │
│             │ ◄────────────────── │             │
│             │                      │             │
│ (gouverne)  │                      │  (décide)   │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| WS → SF | Niveau de sécurité applicable | `SecurityLevel` (0-4) |
| WS → SF | État de confiance du système | `TrustState` (T0-T4) |
| WS → SF | Sévérité requise | `SeverityConstraint` |
| SF → WS | Décisions refusées (pour observation) | `DecisionRejectionSignal` |

**Impact de la gouvernance sur StrongFather :**

| État de confiance | Impact sur les décisions StrongFather |
|-------------------|--------------------------------------|
| **T0 (Normal)** | Décisions normales, sévérité standard |
| **T1 (Instable)** | Logging renforcé, sévérité légèrement accrue |
| **T2 (Dégradé)** | Décisions plus strictes, capacités non essentielles refusées |
| **T3 (Restreint)** | Décisions critiques → AMBIGUË / DIFFÉRÉE, TAMR requis |
| **T4 (Bloqué)** | Plus aucune décision opérationnelle autorisée |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-SF-1** | WorrySentinel ne prend jamais de décision à la place de StrongFather |
| **COL-SF-2** | StrongFather adapte sa sévérité selon les contraintes de WorrySentinel |
| **COL-SF-3** | Les décisions refusées par StrongFather sont observées par WorrySentinel pour corrélation |
| **COL-SF-4** | StrongFather ne peut pas ignorer un état de confiance T3+ |

**Référence Documentation Fondatrice :** Section 9.2 (Relation avec StrongFather)

---

### 5.3 Relation avec KindMother

**Type de relation :** Observation indirecte

**Principe fondamental :**

> KindMother persiste les données. WorrySentinel observe les incohérences détectées par KindMother comme signaux d'intégrité, mais n'accède jamais directement à KindMother.

**Responsabilités respectives :**

| Aspect | KindMother | WorrySentinel |
|--------|------------|---------------|
| Persistance des données | ✅ Autorité | ❌ Aucune |
| Détection d'incohérences | ✅ Source | ❌ Observateur |
| Accès aux données | ✅ Autorité | ❌ INTERDIT |
| Signalement d'anomalies | ✅ Émetteur | ❌ Destinataire (via CaringNanny) |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│ KindMother  │  Incohérences       │  CaringNanny │
│             │ ──────────────────► │  (consolide) │
│             │                      │              │
│             │                      └──────┬───────┘
│             │                             │
│             │                             ▼
│             │                      ┌─────────────┐
│             │                      │WorrySentinel│
│ (persiste)  │                      │ (observe)   │
└─────────────┘                      └─────────────┘
```

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-KM-1** | WorrySentinel n'appelle jamais KindMother directement (INV-WS-3) |
| **COL-KM-2** | Les incohérences détectées par KindMother sont relayées via CaringNanny |
| **COL-KM-3** | WorrySentinel ne peut jamais lire ou modifier des données persistées |
| **COL-KM-4** | Les signaux d'incohérence contribuent à l'évaluation de l'état de confiance |

**Référence Documentation Fondatrice :** Section 9.3 (Relation avec KindMother) — INV-WS-3

---

### 5.4 Relation avec CaringNanny

**Type de relation :** Observation consolidée + Proposition

**Principe fondamental :**

> CaringNanny consolide les signaux d'intégrité du système. WorrySentinel observe ces signaux consolidés et CaringNanny peut proposer des transitions d'état de confiance.

**Responsabilités respectives :**

| Aspect | CaringNanny | WorrySentinel |
|--------|-------------|---------------|
| Consolidation des signaux | ✅ Autorité | ❌ Consommateur |
| Évaluation de l'intégrité | ✅ Production | ✅ Décision finale |
| Proposition de transition | ✅ Émetteur | ❌ Destinataire |
| Décision de transition | ❌ Aucune | ✅ Autorité |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│ CaringNanny │  Signaux consolidés  │WorrySentinel│
│             │ ──────────────────► │             │
│             │                      │             │
│             │  Proposition transit.│             │
│             │ ──────────────────► │             │
│             │                      │             │
│             │  État global actuel  │             │
│             │ ◄────────────────── │             │
│             │                      │             │
│(consolide)  │                      │ (gouverne)  │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| CN → WS | Signaux d'intégrité consolidés | `IntegritySignalBundle` |
| CN → WS | Proposition de transition d'état | `TransitionProposal` |
| CN → WS | Indicateurs de santé | `HealthIndicators` |
| WS → CN | État de confiance actuel | `CurrentTrustState` |
| WS → CN | Règles de consolidation | `ConsolidationRules` |

**Structure des propositions de transition :**

```typescript
interface TransitionProposal {
  // Identification
  proposal_id: UUID;
  
  // Transition proposée
  current_state: TrustState;        // T0-T4
  proposed_state: TrustState;       // T0-T4
  
  // Justification
  signals: ConsolidatedSignal[];
  confidence_score: NormalizedScore; // 0-100
  
  // Metadata
  timestamp: LogicalClock;
}
```

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-CN-1** | CaringNanny consolide les signaux de tous les cores et les transmet à WorrySentinel |
| **COL-CN-2** | CaringNanny peut proposer des transitions mais WorrySentinel décide |
| **COL-CN-3** | WorrySentinel gouverne les règles selon lesquelles CaringNanny consolide |
| **COL-CN-4** | Une proposition refusée par WorrySentinel n'est pas appliquée |

**Référence Documentation Fondatrice :** Section 9.4 (Relation avec CaringNanny)

---

### 5.5 Relation avec BorderGuard

**Type de relation :** Gouvernance

**Principe fondamental :**

> BorderGuard définit les frontières d'intégration. WorrySentinel gouverne le durcissement de ces frontières selon le niveau de sécurité et l'état de confiance.

**Responsabilités respectives :**

| Aspect | BorderGuard | WorrySentinel |
|--------|-------------|---------------|
| Définition des frontières | ✅ Autorité | ❌ Aucune |
| Classification de confiance | ✅ Autorité | ❌ Aucune |
| Durcissement des frontières | ✅ Exécution | ✅ Gouvernance |
| Signalement d'anomalies I/O | ✅ Émetteur | ❌ Observateur |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│WorrySentinel│  Niveau durcissement │ BorderGuard │
│             │ ──────────────────► │             │
│             │                      │             │
│             │  Anomalies I/O       │             │
│             │ ◄────────────────── │             │
│             │                      │             │
│ (gouverne)  │                      │ (définit)   │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| WS → BG | Niveau de durcissement requis | `HardeningLevel` |
| WS → BG | Frontières à bloquer (état T3+) | `BlockedBoundaries` |
| BG → WS | Anomalies I/O détectées | `IOAnomalySignal` |
| BG → WS | Passages vers "hostile" | `HostileDetectionSignal` |

**Impact de la gouvernance sur BorderGuard :**

| État de confiance | Impact sur BorderGuard |
|-------------------|------------------------|
| **T0 (Normal)** | Frontières normales, classification standard |
| **T1 (Instable)** | Surveillance accrue des passages |
| **T2 (Dégradé)** | Durcissement des règles de franchissement |
| **T3 (Restreint)** | Fermeture des frontières non essentielles |
| **T4 (Bloqué)** | Toutes les frontières fermées (mode isolation) |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-BG-1** | WorrySentinel ne définit jamais de frontière (responsabilité de BorderGuard) |
| **COL-BG-2** | BorderGuard applique le durcissement imposé par WorrySentinel |
| **COL-BG-3** | Les anomalies I/O détectées par BorderGuard sont observées par WorrySentinel |
| **COL-BG-4** | En état T3+, BorderGuard doit fermer les frontières non essentielles |

**Référence Documentation Fondatrice :** Section 9.5 (Relation avec BorderGuard)

---

### 5.6 Relation avec MasterButler

**Type de relation :** Gouvernance

**Principe fondamental :**

> MasterButler expose les capacités disponibles. WorrySentinel gouverne les permissions actives en limitant les capacités accessibles selon le niveau de sécurité et l'état de confiance.

**Responsabilités respectives :**

| Aspect | MasterButler | WorrySentinel |
|--------|--------------|---------------|
| Catalogue des capacités | ✅ Autorité | ❌ Aucune |
| Exposition des capacités | ✅ Exécution | ❌ Aucune |
| Limitation des capacités | ✅ Exécution | ✅ Gouvernance |
| Permissions actives | ❌ Consommateur | ✅ Définition |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│WorrySentinel│  Capacités limitées  │MasterButler │
│             │ ──────────────────► │             │
│             │                      │             │
│             │  État permissions    │             │
│             │ ◄────────────────── │             │
│             │                      │             │
│ (gouverne)  │                      │ (expose)    │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| WS → MB | Capacités à limiter | `CapabilityLimitations` |
| WS → MB | Capacités à bloquer (état T2+) | `BlockedCapabilities` |
| MB → WS | État des permissions actives | `PermissionStateReport` |

**Impact de la gouvernance sur MasterButler :**

| État de confiance | Impact sur MasterButler |
|-------------------|-------------------------|
| **T0 (Normal)** | Toutes les capacités disponibles |
| **T1 (Instable)** | Logging renforcé des usages de capacités |
| **T2 (Dégradé)** | Capacités sensibles limitées |
| **T3 (Restreint)** | Seules capacités essentielles disponibles |
| **T4 (Bloqué)** | Aucune capacité disponible (mode diagnostic) |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-MB-1** | WorrySentinel ne modifie jamais le catalogue de capacités |
| **COL-MB-2** | MasterButler applique les limitations imposées par WorrySentinel |
| **COL-MB-3** | MasterButler peut consulter WorrySentinel pour connaître les permissions actives |
| **COL-MB-4** | Les capacités critiques sont bloquées automatiquement en état T3+ |

---

### 5.7 Relation avec BondingBrother

**Type de relation :** Observation

**Principe fondamental :**

> BondingBrother médiatise les échanges entre produits et écosystème. WorrySentinel observe les comportements des produits via BondingBrother pour détecter des anomalies.

**Responsabilités respectives :**

| Aspect | BondingBrother | WorrySentinel |
|--------|----------------|---------------|
| Médiation produits ↔ cores | ✅ Autorité | ❌ Aucune |
| Transport des décisions | ✅ Exécution | ❌ Aucune |
| Observation comportements | ✅ Source | ❌ Consommateur |
| Signalement d'anomalies | ✅ Émetteur | ❌ Observateur |

**Flux d'interaction :**

```
┌─────────────┐                      ┌───────────────┐
│BondingBrother│ Comportements       │ WorrySentinel │
│             │ produits             │               │
│             │ ──────────────────► │               │
│             │                      │               │
│             │ Contraintes état     │               │
│             │ ◄────────────────── │               │
│             │                      │               │
│ (transporte)│                      │  (observe)    │
└─────────────┘                      └───────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| BB → WS | Comportements anormaux des produits | `ProductBehaviorSignal` |
| BB → WS | Patterns d'usage suspects | `SuspiciousPatternSignal` |
| WS → BB | Contraintes liées à l'état global | `StateConstraints` |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-BB-1** | WorrySentinel n'interagit jamais directement avec les produits |
| **COL-BB-2** | BondingBrother remonte les comportements anormaux à WorrySentinel |
| **COL-BB-3** | WorrySentinel peut imposer des contraintes sur les échanges en état dégradé |
| **COL-BB-4** | BondingBrother informe les produits de l'état global (via contraintes) |

---

### 5.8 Relation avec LogisticsSteward

**Type de relation :** Supervision + Gouvernance

**Principe fondamental :**

> LogisticsSteward gouverne l'allocation des ressources. WorrySentinel supervise LogisticsSteward pour détecter les dérives et peut imposer un durcissement des règles d'arbitrage.

**Responsabilités respectives :**

| Aspect | LogisticsSteward | WorrySentinel |
|--------|------------------|---------------|
| Gouvernance des ressources | ✅ Autorité | ❌ Aucune |
| Arbitrage de quotas | ✅ Autorité | ❌ Aucune |
| Durcissement des quotas | ✅ Exécution | ✅ Déclenchement |
| Détection de dérives | ❌ Source | ✅ Observateur |

**Flux d'interaction :**

```
┌─────────────┐                      ┌───────────────┐
│WorrySentinel│                      │LogisticsSteward│
│             │ ←── signaux alloc ── │               │
│             │                      │               │
│             │ ── contraintes ───→ │               │
│             │                      │               │
│             │ ── durcissement ──→ │               │
│             │    (si T1+)          │               │
│             │                      │               │
│             │ ←── confirmation ─── │               │
│             │                      │               │
│(supervise)  │                      │ (arbitre)     │
└─────────────┘                      └───────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| LS → WS | Signaux d'allocation | `AllocationSignal` |
| LS → WS | Dérives détectées | `AllocationDrift` |
| WS → LS | Contraintes sécuritaires | `SecurityConstraints` |
| WS → LS | Directive de durcissement | `HardeningDirective` |
| LS → WS | Confirmation d'application | `ApplicationConfirmation` |

**Règles d'interaction (RÈGLE-WS-LS-*) :**

| ID | Règle |
|----|-------|
| **RÈGLE-WS-LS-1** | WorrySentinel peut imposer des contraintes sécuritaires sur les décisions d'arbitrage de LogisticsSteward |
| **RÈGLE-WS-LS-2** | En état T2+, LogisticsSteward doit appliquer des quotas plus restrictifs selon les directives de WorrySentinel |
| **RÈGLE-WS-LS-3** | WorrySentinel observe les patterns d'allocation de ressources pour détecter des anomalies sécuritaires |
| **RÈGLE-WS-LS-4** | Toute dérive d'allocation signalée par WorrySentinel doit être traitée par LogisticsSteward |

**Impact de la gouvernance sur LogisticsSteward :**

| État de confiance | Impact sur LogisticsSteward |
|-------------------|----------------------------|
| **T0 (Normal)** | Arbitrage normal, quotas standards |
| **T1 (Instable)** | Surveillance accrue des allocations |
| **T2 (Dégradé)** | Quotas réduits, priorités aplaties |
| **T3 (Restreint)** | Quotas minimaux, ressources essentielles uniquement |
| **T4 (Bloqué)** | Gel des allocations, mode maintenance |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-LS-1** | WorrySentinel ne se substitue jamais à LogisticsSteward pour l'arbitrage |
| **COL-LS-2** | LogisticsSteward reste souverain sur les décisions d'allocation |
| **COL-LS-3** | WorrySentinel supervise sans remplacer |
| **COL-LS-4** | Les directives de durcissement sont obligatoires en état T2+ |

**Référence Documentation Fondatrice :** Section 9.6 (Relation avec LogisticsSteward)

---

### 5.9 Relation avec TAMR

**Type de relation :** Escalade + Gouvernance

**Principe fondamental :**

> TAMR définit quand l'humain intervient. WorrySentinel gouverne les droits humains selon l'état de confiance et signale les situations nécessitant une intervention.

**Responsabilités respectives :**

| Aspect | TAMR | WorrySentinel |
|--------|------|---------------|
| Points d'intervention humaine | ✅ Autorité | ❌ Aucune |
| Validation humaine | ✅ Exécution | ❌ Aucune |
| Droits humains actifs | ❌ Exécution | ✅ Gouvernance |
| Signalement besoin intervention | ❌ Destinataire | ✅ Émetteur |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│WorrySentinel│  Droits humains      │    TAMR     │
│             │  applicables         │             │
│             │ ──────────────────► │             │
│             │                      │             │
│             │  Besoin intervention │             │
│             │ ──────────────────► │             │
│             │                      │             │
│             │  Override état       │             │
│             │ ◄────────────────── │             │
│             │                      │             │
│ (gouverne)  │                      │ (valide)    │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| WS → TAMR | Droits humains applicables selon état | `HumanRightsConstraints` |
| WS → TAMR | Demande d'intervention | `InterventionRequest` |
| TAMR → WS | Override d'état de confiance | `TrustStateOverride` |
| TAMR → WS | Validation de transition | `TransitionValidation` |

**Cas nécessitant une escalade vers TAMR :**

| Cas | Description | Sévérité |
|-----|-------------|----------|
| Transition vers T3 | Confirmation humaine requise | Élevée |
| Transition vers T4 | Confirmation humaine obligatoire | Critique |
| Override d'état | Humain souhaite modifier l'état | Variable |
| Ambiguïté sécuritaire | Signaux contradictoires | Moyenne |
| Sortie de T4 | Restauration du système | Critique |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-TAMR-1** | WorrySentinel signale automatiquement les cas d'escalade à TAMR |
| **COL-TAMR-2** | TAMR peut valider ou refuser une transition d'état proposée |
| **COL-TAMR-3** | En état T3+, TAMR est requis pour toute décision critique |
| **COL-TAMR-4** | TAMR peut forcer un override d'état (traçabilité obligatoire) |
| **COL-TAMR-5** | La sortie de T4 nécessite obligatoirement une validation TAMR |

**Référence Documentation Fondatrice :** Section 9.5 (Relation avec TAMR)

---

### 5.10 Relation avec MiyukiniAdmin

**Type de relation :** Exposition + Configuration

**Principe fondamental :**

> MiyukiniAdmin est l'interface d'administration. WorrySentinel expose la gouvernance de sécurité pour consultation et permet une configuration limitée sous validation StrongFather.

**Responsabilités respectives :**

| Aspect | MiyukiniAdmin | WorrySentinel |
|--------|---------------|---------------|
| Interface d'administration | ✅ Autorité | ❌ Aucune |
| Consultation gouvernance | ✅ Client | ✅ Fournisseur |
| Configuration gouvernance | ✅ Demandeur | ✅ Sous validation SF |
| Modification directe | ❌ INTERDIT | ❌ N/A |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│MiyukiniAdmin│  Consultation état   │WorrySentinel│
│             │ ──────────────────► │             │
│             │                      │             │
│             │  État + historique   │             │
│             │ ◄────────────────── │             │
│             │                      │             │
│             │  Demande config      │             │
│             │ ──────────────────► │             │
│             │       │              │             │
│             │       ▼              │             │
│             │  ┌─────────────┐    │             │
│             │  │ StrongFather│    │             │
│             │  │ (validation)│    │             │
│             │  └─────────────┘    │             │
│             │                      │             │
│ (administre)│                      │ (expose)    │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| MA → WS | Demande état actuel | `StateQuery` |
| WS → MA | État de confiance + niveau sécurité | `GovernanceState` |
| WS → MA | Historique des transitions | `TransitionHistory` |
| MA → WS | Demande de configuration | `ConfigurationRequest` |
| WS → MA | Résultat de configuration (après validation SF) | `ConfigurationResult` |

**Interactions autorisées (INTERACTION-ADMIN-*) :**

| ID | Interaction | Validation requise |
|----|-------------|-------------------|
| **INTERACTION-ADMIN-1** | Consultation des niveaux de sécurité | Non |
| **INTERACTION-ADMIN-2** | Consultation des états de confiance | Non |
| **INTERACTION-ADMIN-3** | Configuration de la gouvernance | Oui (StrongFather) |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **RÈGLE-ADMIN-1** | Toute configuration de gouvernance par MiyukiniAdmin doit être validée par StrongFather |
| **RÈGLE-ADMIN-2** | Toute interaction avec MiyukiniAdmin concernant la gouvernance de sécurité est tracée avec identité, moment, et justification |
| **COL-MA-1** | MiyukiniAdmin peut consulter librement l'état de gouvernance |
| **COL-MA-2** | MiyukiniAdmin ne peut jamais modifier directement l'état de confiance |
| **COL-MA-3** | Les configurations sont soumises à validation, pas imposées |

**Référence Documentation Fondatrice :** Section 11 (Interaction avec MiyukiniAdmin)

---

## 6. Relation avec les produits

### 6.1 Principe fondamental

**Les produits ne parlent jamais directement à WorrySentinel.**

Toute interaction passe par BondingBrother qui médiatise les échanges.

```
┌─────────────┐                                    ┌─────────────┐
│  Produits   │ ──────────────────────────────────► │WorrySentinel│
│             │              ❌ INTERDIT            │             │
└─────────────┘                                    └─────────────┘

┌─────────────┐    via     ┌───────────────┐       ┌─────────────┐
│  Produits   │ ─────────► │BondingBrother │ ────► │WorrySentinel│
│             │            │               │       │ (observation)│
└─────────────┘            └───────────────┘       └─────────────┘
               ✅ AUTORISÉ
```

### 6.2 Ce que les produits reçoivent (via BondingBrother)

| Type | Description |
|------|-------------|
| État global | État de confiance actuel (T0-T4) |
| Contraintes | Limitations actives dues à l'état |
| Alertes | Notifications de changement d'état |

### 6.3 Ce que les produits ne peuvent pas demander

| Demande | Statut |
|---------|--------|
| Modification de l'état de confiance | ❌ INTERDIT |
| Bypass des contraintes de sécurité | ❌ INTERDIT |
| Configuration directe de WorrySentinel | ❌ INTERDIT |

---

## 7. Diagramme d'interaction globale

```
                                    ┌────────────────────────────────────────────────┐
                                    │              WORRY SENTINEL                     │
                                    │  ┌───────────┐  ┌───────────┐  ┌───────────┐   │
                                    │  │ Niveaux   │  │  États    │  │Dégradation│   │
                                    │  │ sécurité  │  │ confiance │  │progressive│   │
                                    │  │   (0-4)   │  │  (T0-T4)  │  │           │   │
                                    │  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘   │
                                    │        │              │              │         │
                                    │        └──────────────┼──────────────┘         │
                                    │                       │                        │
                                    └───────────────────────┼────────────────────────┘
                                                            │
           ┌────────────────────────────────────────────────┼────────────────────────────────────────────────┐
           │                         │                      │                       │                        │
           │ FLUX DESCENDANT         │                      │                       │         FLUX MONTANT   │
           │ (gouvernance)           ▼                      │                       ▼         (observation)  │
           │                 ┌───────────────┐              │               ┌───────────────┐                │
           │                 │ StrongFather  │              │               │    Kernel     │                │
           │                 │  (sévérité)   │              │               │  (signaux)    │                │
           │                 └───────────────┘              │               └───────────────┘                │
           │                         │                      │                       │                        │
           ▼                         ▼                      │                       ▼                        │
   ┌───────────────┐         ┌───────────────┐              │               ┌───────────────┐                │
   │ MasterButler  │         │  BorderGuard  │              │               │  KindMother   │                │
   │ (permissions) │         │(durcissement) │              │               │(incohérences) │                │
   └───────────────┘         └───────────────┘              │               └───────┬───────┘                │
           │                         │                      │                       │                        │
           ▼                         ▼                      │                       ▼                        │
   ┌───────────────┐         ┌───────────────┐              │               ┌───────────────┐                │
   │Logistics      │         │     TAMR      │              │               │BondingBrother │                │
   │Steward(quotas)│         │(droits humain)│              │               │ (comportements│                │
   └───────────────┘         └───────────────┘              │               │   produits)   │                │
                                                            │               └───────┬───────┘                │
                                                            │                       │                        │
                                                            │                       ▼                        │
                                                            │               ┌───────────────┐                │
                                                            │               │  CaringNanny  │◄───────────────┘
                                                            │               │(consolidation)│
                                                            │               └───────┬───────┘
                                                            │                       │
                                                            └───────────────────────┘
                                                                    Propositions
                                                                    de transition
```

---

## 8. Synthèse des contrats d'interface

### 8.1 Matrice des interactions

| Core | Direction | Nature | Données échangées |
|------|-----------|--------|-------------------|
| **Kernel** | K → WS, WS → K | Observation | Signaux ↔ Contraintes sondes |
| **StrongFather** | WS → SF, SF → WS | Gouvernance | Niveaux/états → Décisions refusées |
| **KindMother** | KM → CN → WS | Observation indirecte | Incohérences (via CaringNanny) |
| **CaringNanny** | CN ↔ WS | Observation + Proposition | Signaux consolidés ↔ État actuel |
| **BorderGuard** | WS → BG, BG → WS | Gouvernance | Durcissement ↔ Anomalies I/O |
| **MasterButler** | WS → MB, MB → WS | Gouvernance | Capacités limitées ↔ État permissions |
| **BondingBrother** | BB → WS, WS → BB | Observation | Comportements ↔ Contraintes |
| **LogisticsSteward** | LS ↔ WS | Supervision + Gouvernance | Signaux alloc ↔ Contraintes/Durcissement |
| **TAMR** | WS → TAMR, TAMR → WS | Escalade + Gouvernance | Droits/Interventions ↔ Overrides |
| **MiyukiniAdmin** | MA → WS, WS → MA | Exposition | Consultation ↔ État/Configuration |

### 8.2 Garanties de service

| Garantie | Valeur | Condition |
|----------|--------|-----------|
| Disponibilité de la gouvernance | 100% | Invariant structural |
| Traçabilité des interactions | 100% | Invariant INV-WS-8 |
| Non-blocage des flux | 100% | Invariant structural |
| Cohérence inter-états | 100% | Invariant INV-GOV-2 |

---

## 9. Conformité aux Lois d'Autonomie

### 9.1 LOI-1 : Aucune dépendance externe critique

Toutes les interactions sont locales. WorrySentinel n'a pas besoin de service externe pour interagir avec les autres cores.

### 9.2 LOI-2 : Le système accepte l'isolement

En mode isolé, WorrySentinel continue de gouverner la sécurité localement. Les états de confiance sont maintenus sans dépendance externe.

### 9.3 LOI-6 : L'autonomie n'empêche pas la fédération

Les informations de gouvernance peuvent être partagées entre COG via BondingBrother, avec contraintes de WorrySentinel.

---

## 10. Références

### Documents fondateurs

- [WorrySentinel - Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)

### Contrats associés

- [WorrySentinel - Architecture & Flows](./WorrySentinel%20-%20Architecture%20&%20Flows.md)

### Documents de référence

- [Miyukini Conceptual References - Security Levels](../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)
- [Miyukini Conceptual References - Integrity Degradation System](../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

---

## 11. Mini log de génération

### Décision structurelle D1 : Séparation flux descendant/montant

**Décision prise :** Le document est structuré autour des deux flux fondamentaux de WorrySentinel (gouvernance et observation) pour refléter sa nature de pression verticale transversale.

**Application :** Section 4 dédiée aux flux globaux, et chaque relation avec un core précise sa direction principale.

### Décision structurelle D2 : Relations multiples avec certains cores

**Décision prise :** Certains cores ont des relations bidirectionnelles avec WorrySentinel (ex: CaringNanny, LogisticsSteward). Chaque direction est documentée comme flux distinct.

**Application :** Contrats d'interface avec directions explicites pour chaque échange.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Documentation Fondatrice : Toutes les relations documentées en Section 9
- ✅ Respect INV-WS-1 : Aucune autorité sur l'implémentation
- ✅ Respect INV-WS-2 : Aucune autorité sur l'exécution
- ✅ Respect INV-WS-3 : Aucune autorité sur la persistance
- ✅ Respect INV-WS-4 : Aucune modification d'état directe
- ✅ Respect INV-WS-5 : Aucune logique temporelle technique
- ✅ Flux descendant conforme : Section 9 (gouvernance)
- ✅ Flux montant conforme : Section 9 (observation)
- ✅ Relation LogisticsSteward conforme : RÈGLE-WS-LS-1 à RÈGLE-WS-LS-4

**Conclusion :** Aucune contradiction détectée. Le document est cohérent avec la Documentation Fondatrice.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat normatif — ARCHITECTURE  
**Référence :** WorrySentinel - Documentation Fondatrice v1.2, Sections 9 et 11
