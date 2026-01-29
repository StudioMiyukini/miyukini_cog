# TAMR — Architecture & Flows

## 1. Introduction

### Objet du document

Ce document définit le **TAMR — Architecture & Flows** : un contrat normatif, non négociable, et de statut FONDATION qui établit l'architecture conceptuelle des flux d'intervention humaine dans le Miyukini Core System v2.4. Il précise comment les quatre flux (Approval, Override, Escalation, Supervision) s'articulent, quels acteurs ils impliquent, et comment ils s'intègrent à l'écosystème.

TAMR ne possède pas de composants internes exécutables : il définit un **cadre conceptuel**. Ce document décrit l'architecture de ce cadre et les flux d'intervention que les produits et les cores doivent respecter.

### Portée

Ce document s'applique à **toute l'architecture des interventions humaines** et définit de manière absolue :
- la position de TAMR dans l'écosystème,
- les quatre flux d'intervention (Approval, Override, Escalation, Supervision),
- les acteurs et les responsabilités par flux,
- les points de convergence (BondingBrother, StrongFather, KindMother),
- les invariants architecturaux des flux.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce document **synthétise et illustre** l'architecture des flux définie dans :
- **[TAMR — Documentation Fondatrice](../foundation/TAMR%20-%20Documentation%20Fondatrice.md)** : Types et principes des interventions
- **[TAMR — Intervention Types Contract](../contracts/intervention/TAMR%20-%20Intervention%20Types%20Contract.md)** : Définition formelle des quatre types
- **[TAMR — Intervention Points Contract](../contracts/intervention/TAMR%20-%20Intervention%20Points%20Contract.md)** : Points où les interventions sont possibles
- **[TAMR — StrongFather Integration Contract](../contracts/integration/TAMR%20-%20StrongFather%20Integration%20Contract.md)** : Relation TAMR / StrongFather
- **[TAMR — KindMother Integration Contract](../contracts/integration/TAMR%20-%20KindMother%20Integration%20Contract.md)** : Persistance des traces
- **[TAMR — BondingBrother Integration Contract](../contracts/integration/TAMR%20-%20BondingBrother%20Integration%20Contract.md)** : Médiation des intentions

Il ne contredit aucun autre contrat et constitue une vue architecturale consolidée des flux.

---

## 2. Contexte

TAMR (The Authority Must Rest) est le **Human Interaction Core** du Miyukini Core System. Il définit où, quand et comment l'humain intervient, sans prendre de décision ni persister de donnée. Les quatre types d'intervention — Approval, Override, Escalation, Supervision — traversent tous l'écosystème via des flux explicites : intention → médiation (BondingBrother) → évaluation (StrongFather) → exécution et trace (produit + KindMother). Ce document décrit ces flux et leur architecture commune.

---

## 3. Portée / Scope

**Ce document couvre :**
- L'architecture conceptuelle des flux d'intervention humaine
- Le détail des quatre flux : Approval, Override, Escalation, Supervision
- Les acteurs (Processus, Produit, BondingBrother, StrongFather, KindMother, Humain)
- Les points de convergence et les invariants des flux
- La conformité aux Lois d'Autonomie et aux références (Glossaire, Doctrine Sécurité, Integrity Degradation, Security Levels)

**Ce document ne couvre pas :**
- Les détails des types d'intervention (voir Intervention Types Contract)
- Les points d'intervention et déclencheurs (voir Intervention Points Contract)
- Les limites d'autorité et limites inviolables (voir contrats boundaries)
- L'implémentation technique (responsabilité produit)

---

## 4. Architecture conceptuelle

### 4.1. Vue d'ensemble de la place de TAMR

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        ÉCOSYSTÈME MIYUKINI                               │
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │  TAMR (cadre conceptuel)                                           │  │
│  │  • Types : APPROVAL, OVERRIDE, ESCALATION, SUPERVISION             │  │
│  │  • Points d'intervention, limites d'autorité                       │  │
│  │  • Exigences de traçabilité                                        │  │
│  │  • Ne décide pas, ne persiste pas, ne médie pas                    │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                    │                                    │
│                    cadre utilisé par les flux ci-dessous                 │
│                                    ▼                                    │
│  ┌─────────────┐    ┌──────────────────┐    ┌─────────────────────┐   │
│  │ Processus   │───▶│ BondingBrother   │───▶│ StrongFather        │   │
│  │ / Produit   │    │ (médiation        │    │ (autorise / refuse   │   │
│  │             │    │  des intentions)  │    │  selon politiques)   │   │
│  └─────────────┘    └──────────────────┘    └──────────┬──────────┘   │
│                                                          │              │
│                                                          ▼              │
│  ┌─────────────┐    ┌──────────────────┐    ┌─────────────────────┐   │
│  │ Humain      │◀───│ Produit (UI,      │◀───│ Décision            │   │
│  │ (intervient)│    │  notification)     │    │ (autorisé/refusé)    │   │
│  └─────────────┘    └─────────┬─────────┘    └─────────────────────┘   │
│                               │                                         │
│                               ▼                                         │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │  KindMother (persistance des traces d'intervention)                 │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 4.2. Principe commun à tous les flux

Chaque flux d'intervention humaine respecte la séquence conceptuelle suivante :

1. **Déclenchement** : Un point d'intervention est atteint ou une intention est émise.
2. **Intention** : Une intention d'intervention (type TAMR + point + acteur + contexte) est créée.
3. **Médiation** : L'intention transite par BondingBrother.
4. **Évaluation** : StrongFather évalue si l'intervention est autorisée (selon le cadre TAMR et les politiques).
5. **Exécution** : Si autorisée, l'humain effectue l'intervention via le produit.
6. **Trace** : L'intervention est tracée selon les exigences TAMR et persistée par KindMother.

TAMR définit le **cadre** (types, points, limites, traçabilité). Il ne participe pas à l'exécution des étapes 2 à 6.

---

## 5. Flux Approval (Approbation)

### 5.1. Objectif

Valider ou refuser une action **avant** son exécution. Le système propose, l'humain décide.

### 5.2. Acteurs

| Acteur | Rôle |
|--------|------|
| Processus automatisé | Atteint un point d'approbation, crée la demande |
| Produit | Notifie l'approbateur, présente l'interface de décision |
| BondingBrother | Médie l'intention d'approbation |
| StrongFather | Évalue si l'approbation est requise et si l'acteur peut approuver |
| Approbateur (humain) | Approuve ou refuse |
| KindMother | Persiste la trace de l'approbation |

### 5.3. Flux détaillé

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         FLUX APPROVAL (APPROBATION)                       │
│                                                                         │
│  1. Processus atteint un point d'approbation (point déclaré TAMR)       │
│        │                                                                │
│        ▼                                                                │
│  2. Système crée une demande d'approbation (intention type APPROVAL)    │
│        │                                                                │
│        ▼                                                                │
│  3. Intention transite par BondingBrother                               │
│        │                                                                │
│        ▼                                                                │
│  4. StrongFather évalue :                                               │
│        • L'approbation est-elle requise pour ce contexte ?               │
│        • Qui est l'approbateur désigné ?                                │
│        • Cet acteur est-il autorisé à approuver ?                       │
│        │                                                                │
│        ├── Refusé / Ambigü / Différé ──▶ Fin (pas d'approbation)        │
│        │                                                                │
│        ▼ Autorisé                                                       │
│  5. Produit notifie l'approbateur désigné                               │
│        │                                                                │
│        ▼                                                                │
│  6. Approbateur approuve ou refuse (ou expiration → comportement défaut) │
│        │                                                                │
│        ▼                                                                │
│  7. Intervention tracée (identité, décision, moment, contexte)          │
│        │                                                                │
│        ▼                                                                │
│  8. KindMother persiste la trace                                        │
│        │                                                                │
│        ▼                                                                │
│  9. Processus reprend selon la décision (exécution si APPROUVÉ,         │
│     abandon ou alternative si REFUSÉ / EXPIRÉ)                           │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 5.4. États et résultats

- **États** : DEMANDÉE → EN_ATTENTE → RÉSOLUE (ou EXPIRÉ).
- **Résultats** : APPROUVÉ, REFUSÉ, EXPIRÉ (comportement par défaut explicite requis, voir Intervention Types Contract).

### 5.5. Invariants rappelés

- **INV-TAMR-1** : Toute approbation est tracée.
- **INV-TYPE-1** : Liste fermée des types ; APPROVAL est l'un des quatre.
- **R-APPR-1** : Identité de l'approbateur obligatoire.

---

## 6. Flux Override (Dérogation)

### 6.1. Objectif

Contredire une décision automatique : forcer une action refusée (FORCE) ou bloquer une action approuvée (BLOCK). Exceptionnel, justifié, audité.

### 6.2. Acteurs

| Acteur | Rôle |
|--------|------|
| Décision automatique | Préalable (acceptée ou refusée) |
| Humain autorisé | Demande l'override, fournit la justification |
| BondingBrother | Médie l'intention d'override |
| StrongFather | Évalue si l'override est autorisé ; vérifie les limites inviolables TAMR |
| KindMother | Persiste la trace (avec justification) |

### 6.3. Flux détaillé

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         FLUX OVERRIDE (DÉROGATION)                       │
│                                                                         │
│  1. Décision automatique émise (acceptée ou refusée)                   │
│        │                                                                │
│        ▼                                                                │
│  2. Un humain autorisé demande un override (intention type OVERRIDE)    │
│        │                                                                │
│        ▼                                                                │
│  3. Intention transite par BondingBrother                               │
│        │                                                                │
│        ▼                                                                │
│  4. StrongFather évalue :                                               │
│        • L'override franchirait-il une limite inviolable TAMR ?         │
│          → OUI : REFUS obligatoire (aucune exception)                   │
│        • L'acteur est-il autorisé à déroger selon les politiques ?       │
│        │                                                                │
│        ├── Refusé (limite inviolable ou politique) ──▶ Fin             │
│        │                                                                │
│        ▼ Autorisé                                                       │
│  5. L'humain fournit une justification explicite (obligatoire)          │
│        │                                                                │
│        ▼                                                                │
│  6. Override appliqué (FORCE ou BLOCK)                                   │
│        │                                                                │
│        ▼                                                                │
│  7. Intervention tracée (identité, justification, décision originale,  │
│     moment, contexte, confirmation limites vérifiées)                    │
│        │                                                                │
│        ▼                                                                │
│  8. KindMother persiste la trace                                        │
│        │                                                                │
│        ▼                                                                │
│  9. Processus reprend avec la décision overridée                        │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 6.4. Règles rappelées

- **INV-TAMR-7** : Tout override nécessite une justification explicite enregistrée.
- **R-OVER-2** : Aucun override ne peut franchir une limite inviolable (voir Inviolable Limits Contract).
- **INV-OVER-1** : Non-franchissement des limites infranchissables.

---

## 7. Flux Escalation (Escalade)

### 7.1. Objectif

Élever une décision vers un niveau d'autorité supérieur humain. Hiérarchique, collaborative, tracée ; ne doit jamais bloquer indéfiniment le système.

### 7.2. Acteurs

| Acteur | Rôle |
|--------|------|
| Initiateur (humain) | Déclenche l'escalade, fournit le motif |
| BondingBrother | Médie l'intention d'escalade |
| StrongFather | Identifie le niveau d'escalade approprié, autorise ou refuse |
| Produit | Notifie le(s) responsable(s) du niveau supérieur |
| Responsable(s) niveau supérieur | Prend(ent) la décision |
| KindMother | Persiste la trace (chemin d'escalade, résolution) |

### 7.3. Flux détaillé

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         FLUX ESCALATION (ESCALADE)                       │
│                                                                         │
│  1. Situation nécessitant une escalade identifiée                        │
│        │                                                                │
│        ▼                                                                │
│  2. Demande d'escalade créée (intention type ESCALATION, motif explicite)│
│        │                                                                │
│        ▼                                                                │
│  3. Intention transite par BondingBrother                                │
│        │                                                                │
│        ▼                                                                │
│  4. StrongFather identifie le niveau d'escalade et autorise ou refuse   │
│        │                                                                │
│        ├── Refusé ──▶ Fin                                               │
│        │                                                                │
│        ▼ Autorisé                                                       │
│  5. Produit notifie le(s) responsable(s) du niveau supérieur            │
│        │                                                                │
│        ▼                                                                │
│  6. État : INITIÉE → EN_COURS → RÉSOLUE (ou ANNULÉE / timeout)         │
│        │                                                                │
│        ▼                                                                │
│  7. Le(s) responsable(s) prennent une décision                           │
│        │                                                                │
│        ▼                                                                │
│  8. Escalade et résolution tracées (chemin, niveaux, moments, résolution)│
│        │                                                                │
│        ▼                                                                │
│  9. KindMother persiste la trace                                        │
│        │                                                                │
│        ▼                                                                │
│ 10. Processus reprend selon la décision escaladée                       │
│     Si non résolu dans le délai : comportement par défaut (timeout,      │
│     délégation automatique, rejet par défaut) — INV-TAMR-8               │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 7.4. Invariants rappelés

- **INV-TAMR-8** : Une escalade ne bloque pas indéfiniment le système (mécanismes de timeout / comportement par défaut obligatoires).
- **R-ESC-2** : Chaîne de responsabilité définie ; **R-ESC-5** : Comportement par défaut en cas de non-résolution explicite.

---

## 8. Flux Supervision (Supervision)

### 8.1. Objectif

Observer le système de manière continue, avec capacité d'intervenir si nécessaire. Passive par défaut, activable (approval, override ou escalade), non intrusive, de durée limitée.

### 8.2. Acteurs

| Acteur | Rôle |
|--------|------|
| Processus / Produit | Active la supervision, expose l'état supervisé |
| Superviseur (humain) | Observe, peut déclencher une intervention |
| BondingBrother / StrongFather | Utilisés si le superviseur déclenche une intervention (APPROVAL, OVERRIDE, ESCALATION) |
| KindMother | Persiste les traces (début/fin supervision, interventions déclenchées) |

### 8.3. Flux détaillé

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         FLUX SUPERVISION                                 │
│                                                                         │
│  1. Processus activé pour supervision humaine (périmètre et durée définis)│
│        │                                                                │
│        ▼                                                                │
│  2. Système enregistre l'état supervisé (sans modifier le comportement) │
│        │                                                                │
│        ▼                                                                │
│  3. État : ACTIVÉE                                                       │
│        │                                                                │
│        ▼                                                                │
│  4. L'humain superviseur observe via les interfaces produit            │
│        │                                                                │
│        ├── Si nécessaire : déclenche une intervention                   │
│        │   (APPROVAL / OVERRIDE / ESCALATION)                            │
│        │   → Les flux correspondants s'exécutent (sections 5, 6, 7)      │
│        │   → Chaque intervention a sa propre trace                      │
│        │                                                                │
│        ├── Fin explicite ou timeout ──▶ État TERMINÉE                   │
│        │                                                                │
│        ▼                                                                │
│  5. Supervision tracée (superviseur, périmètre, début, fin, raison,     │
│     interventions déclenchées éventuelles)                              │
│        │                                                                │
│        ▼                                                                │
│  6. KindMother persiste la trace                                        │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 8.4. Règles rappelées

- **R-SUP-4** : La supervision en état passif ne modifie jamais le comportement du système.
- **R-SUP-5** : Toute intervention déclenchée par le superviseur est typée (APPROVAL, OVERRIDE ou ESCALATION) et tracée séparément.

---

## 9. Synthèse des flux et points de convergence

### 9.1. Tableau récapitulatif

| Flux        | Déclencheur principal        | Passage StrongFather | Justification obligatoire | Trace KindMother |
|------------|------------------------------|------------------------|---------------------------|------------------|
| **Approval**   | Point d'approbation atteint   | Oui                    | Non                       | Oui              |
| **Override**  | Demande humain post-décision | Oui (+ limites inviol.) | Oui                       | Oui              |
| **Escalation**| Demande humain / situation   | Oui                    | Motif explicite           | Oui              |
| **Supervision**| Activation processus         | Si intervention déclenchée | Non (pour la supervision seule) | Oui        |

### 9.2. Points de convergence communs

- **BondingBrother** : Toute intention d'intervention (sauf pure observation en supervision) transite par BondingBrother avant évaluation.
- **StrongFather** : Toute intervention exécutée doit avoir été autorisée par StrongFather (sauf supervision passive ; dès qu'une intervention est déclenchée, StrongFather est sollicité).
- **KindMother** : Toute intervention (et toute résolution d'escalade, toute supervision) est tracée selon TAMR et persistée par KindMother.
- **TAMR** : Fournit le cadre (types, points, limites, exigences de trace) ; ne décide pas, ne persiste pas, ne médie pas.

### 9.3. Relations entre flux

Une supervision peut déclencher une approbation, un override ou une escalade. Une approbation peut mener à une escalade (délégation). Un override peut mener à une escalade. Chaque intervention conserve son type et sa trace propre ; les liens sont tracés (voir Intervention Types Contract, section 7).

---

## 10. Invariants architecturaux des flux

### 10.1. Invariants de structure

**INV-ARCH-TAMR-1 : Médiation obligatoire**

Toute intention d'intervention (hors pure observation en supervision) transite par BondingBrother avant évaluation par StrongFather.

**INV-ARCH-TAMR-2 : Évaluation StrongFather**

Aucune intervention (approval, override, escalade, ou intervention déclenchée depuis une supervision) ne peut être exécutée sans évaluation StrongFather et décision d'autorisation (sauf comportement par défaut en cas de timeout / expiration).

**INV-ARCH-TAMR-3 : Traçabilité persistée**

Toute intervention exécutée est tracée selon les exigences TAMR et persistée par KindMother.

### 10.2. Invariants de comportement

**INV-ARCH-TAMR-4 : TAMR ne décide pas**

TAMR ne prend aucune décision d'autorisation ou de refus. Les flux décrivent l'usage du cadre TAMR par les cores et le produit, pas une exécution par TAMR.

**INV-ARCH-TAMR-5 : Limites inviolables**

Aucun flux (en particulier Override) ne peut contourner les limites inviolables définies par TAMR. StrongFather refuse toute intention qui les franchirait.

**INV-ARCH-TAMR-6 : Escalade non bloquante**

Le flux Escalation prévoit toujours un comportement par défaut en cas de non-résolution (timeout, délégation, rejet par défaut). Aucun blocage indéfini.

---

## 11. Références

Ce document s'appuie sur les références suivantes :

| Document | Usage |
|----------|--------|
| [Miyukini Conceptual References — Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Terminologie TAMR (intervention, approbation, override, escalade, supervision, point d'intervention, trace, etc.) |
| [Miyukini Conceptual References — Doctrine Securite Fondamentale](../../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md) | Principes de sécurité applicables aux flux d'intervention |
| [Miyukini Conceptual References — Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) | Conformité LOI-1 à LOI-6 (flux locaux, pas de dépendance externe critique, isolement accepté) |
| [Miyukini Conceptual References — Integrity Degradation System](../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md) | Adaptation des points et flux selon niveaux T0–T4 |
| [Miyukini Conceptual References — Security Levels](../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) | Niveaux de sécurité 0–4 et impact sur les interventions |

Les flux décrits fonctionnent en conformité avec ces références (évaluation locale, traçabilité, pas de dépendance externe bloquante).

---

## 12. Conclusion contractuelle

Ce document établit de manière définitive et non négociable l'architecture des flux d'intervention humaine TAMR.

Il garantit que :
- les quatre flux (Approval, Override, Escalation, Supervision) sont décrits de façon explicite et cohérente avec les contrats TAMR ;
- les acteurs et les points de convergence (BondingBrother, StrongFather, KindMother) sont identifiés ;
- les invariants architecturaux des flux sont maintenus ;
- les références (Glossaire, Doctrine Sécurité, Lois Autonomie, Integrity Degradation, Security Levels) sont intégrées.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Document créé le :** 2026-01-28  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, TAMR Documentation Fondatrice v1.4, TAMR Intervention Types Contract, TAMR Intervention Points Contract, TAMR StrongFather / KindMother / BondingBrother Integration Contracts  
**Type :** Architecture et flux d'intervention humaine non négociables
