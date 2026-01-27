# KindMother — Internal State Machine (Informative)

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il décrit une machine d'état interne conceptuelle permettant de traduire les contrats KindMother en logique runtime, sans exposer d'implémentation.

**Objectif pédagogique :** Ce document vise à aider les développeurs à comprendre comment les concepts contractuels se traduisent en états runtime, sans introduire de nouvelles règles contractuelles.

**Relation avec les contrats FONDATION :** Ce document fait référence aux contrats FONDATION existants mais ne les étend pas, ne les modifie pas, et ne crée aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1. Objectif

Ce document décrit une machine d'état interne conceptuelle qui permet de comprendre comment une instance KindMother peut être modélisée en termes d'états runtime, en se basant strictement sur les invariants, garanties, et interdictions définis dans les contrats FONDATION.

### 1.2. Nature conceptuelle

Cette machine d'état est **purement conceptuelle**. Elle ne présuppose aucune implémentation technique, aucune structure de données, ou aucun mécanisme de gestion d'état. Elle sert uniquement à illustrer comment les concepts contractuels peuvent être organisés en états logiques.

### 1.3. Sources contractuelles

Cette machine d'état est dérivée des contrats FONDATION suivants :

- **Instance Model Contract** : Invariants INST-1 à INST-8, INST-M-1 à INST-M-5, INST-F-1 à INST-F-5
- **Persistence & Storage Contract** : Garanties G-PERSIST-*, corruption et réparation (INV-CORR-*)
- **Runtime Boundary & Enforcement Contract** : Réponses systémiques (R1 à R4), violations détectables
- **Write Intent Lifecycle Contract** : États des intentions d'écriture
- **[Miyukini Conceptual References — Lois Autonomie Système](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Les états illustrent notamment **LOI-2** (isolement comme état normal) et **LOI-3** (état local souverain) à travers les états d'autonomie et de désynchronisation des Instance Filles.

---

## 2. Mapping concepts contractuels → états runtime

### 2.1. États dérivés des invariants

Les invariants contractuels se traduisent en propriétés d'état qui doivent toujours être vraies :

**Invariants communs (INST-1 à INST-8) :**
- **Identité unique préservée** : L'instance maintient son identité unique (INST-1)
- **Autorité KindMother respectée** : L'autorité exclusive de KindMother est reconnue (INST-2)
- **Isolation maintenue** : L'isolation systémique est préservée (INST-3)
- **Persistance interne opérationnelle** : La persistance interne fonctionne (INST-4)
- **Cycle de vie contrôlé** : Le cycle de vie est sous contrôle (INST-5)
- **Validation obligatoire respectée** : Toutes les opérations sont validées (INST-6)
- **Traçabilité complète** : La traçabilité est assurée (INST-7)
- **Protection contre corruption active** : La protection contre les corruptions est active (INST-8)

**Invariants Instance Mère (INST-M-1 à INST-M-5) :**
- **Autorité de référence exercée** : L'autorité de référence est exercée (INST-M-1)
- **Source de vérité maintenue** : La source de vérité est maintenue (INST-M-2)
- **Persistance de référence opérationnelle** : La persistance de référence fonctionne (INST-M-3)
- **Point de convergence actif** : Le point de convergence est actif (INST-M-4)
- **Cohérence de référence préservée** : La cohérence de référence est préservée (INST-M-5)

**Invariants Instance Fille (INST-F-1 à INST-F-5) :**
- **Autorité Mère reconnue** : L'autorité de l'Instance Mère est reconnue (INST-F-1)
- **Copie locale synchronisée** : La copie locale est synchronisée (INST-F-2)
- **Synchronisation périodique effectuée** : La synchronisation périodique est effectuée (INST-F-3)
- **Autonomie limitée respectée** : L'autonomie limitée est respectée (INST-F-4)
  - Cet état respecte **LOI-2** (le système accepte l'isolement comme état normal) : l'Instance Fille peut fonctionner de manière autonome même en l'absence de connexion avec l'Instance Mère, l'isolement n'est pas traité comme une erreur.
  - Il respecte également **LOI-3** (l'état local est souverain) : l'Instance Fille détient l'autorité locale sur son état, et la réconciliation avec l'Instance Mère est explicite et traçable.
- **Soumission à validation effectuée** : Les opérations sont soumises à validation (INST-F-5)

### 2.2. États dérivés des garanties de persistance

Les garanties de persistance (G-PERSIST-*) se traduisent en états de disponibilité des données :

- **Durabilité garantie** : Les données validées sont durables (G-PERSIST-1)
- **Atomicité préservée** : Les opérations sont atomiques (G-PERSIST-2)
- **Cohérence maintenue** : La cohérence est maintenue (G-PERSIST-3)
- **Intégrité protégée** : L'intégrité est protégée (G-PERSIST-4)
- **Isolation garantie** : L'isolation est garantie (G-PERSIST-5)

### 2.3. États dérivés des Runtime Boundaries

Les Runtime Boundaries définissent des conditions de validation qui se traduisent en états de validité :

- **Boundary d'appel valide** : Les appels sont légaux
- **Boundary de contexte valide** : Le contexte est complet et valide
- **Boundary d'instance valide** : L'instance est dans un état valide
- **Boundary de permissions valide** : Les permissions sont suffisantes
- **Boundary de cohérence valide** : La cohérence est préservée
- **Boundary de contournement valide** : Aucun contournement détecté
- **Boundary de charge acceptable** : La charge est raisonnable

### 2.4. États dérivés du cycle de vie des Write Intents

Les Write Intents passent par des états qui influencent l'état de l'instance :

- **Intentions en attente** : Intentions créées mais non encore validées
- **Intentions en validation** : Intentions en cours de validation
- **Intentions acceptées** : Intentions validées et éligibles pour application
- **Intentions appliquées** : Intentions appliquées et persistées
- **Intentions rejetées** : Intentions rejetées et archivées

---

## 3. États typiques d'une instance

### 3.1. Instance saine

**Définition conceptuelle :**

Une instance est dans un état **sain** lorsque tous les invariants contractuels sont respectés et que toutes les opérations autorisées peuvent être effectuées.

**Caractéristiques :**
- Tous les invariants INST-* sont respectés
- Toutes les Runtime Boundaries peuvent être traversées avec succès
- La persistance est opérationnelle et intègre
- Les opérations de lecture et d'écriture sont autorisées
- La synchronisation (pour une Instance Fille) peut être effectuée
- Aucune corruption n'est détectée
- La charge est acceptable

**Opérations autorisées :**
- Toutes les opérations CoreDataAPI sont autorisées
- Les Write Intents peuvent être créées, validées, et appliquées
- La synchronisation peut être effectuée (Instance Fille)
- Les lectures retournent des données cohérentes

**Alignement contractuel :**
- Respecte tous les invariants INST-1 à INST-8
- Respecte les garanties G-PERSIST-* (durabilité, atomicité, cohérence, intégrité, isolation)
- Permet toutes les opérations autorisées par la CoreDataAPI

### 3.2. Instance dégradée

**Définition conceptuelle :**

Une instance est dans un état **dégradé** lorsque certains invariants sont préservés mais certaines opérations sont limitées, tout en restant fonctionnelle.

**Caractéristiques :**
- Les invariants fondamentaux (INST-1, INST-2, INST-3, INST-6, INST-7, INST-8) sont respectés
- Certaines opérations peuvent être limitées (dégradation contrôlée R4)
- La charge peut être excessive, nécessitant une limitation
- La synchronisation peut être ralentie ou limitée (Instance Fille)
- Les lectures restent possibles mais peuvent être limitées
- Les écritures peuvent être ralenties ou limitées
- L'intégrité est préservée malgré la dégradation

**Opérations autorisées :**
- Les opérations de lecture sont autorisées mais peuvent être limitées
- Les opérations d'écriture sont autorisées mais peuvent être ralenties
- La synchronisation peut être limitée (Instance Fille)
- Les opérations d'inspection restent possibles

**Opérations limitées :**
- Certaines opérations peuvent être neutralisées (R2) ou dégradées (R4)
- La charge excessive peut limiter le débit des opérations
- Certaines opérations peuvent être reportées

**Alignement contractuel :**
- Respecte l'invariant INST-8 (protection contre corruption)
- Respecte la garantie G-PERSIST-4 (intégrité protégée)
- Applique la réponse systémique R4 (dégradation contrôlée) du Runtime Boundary Contract
- Préserve l'intégrité malgré la dégradation

### 3.3. Instance en quarantaine

**Définition conceptuelle :**

Une instance est en **quarantaine** lorsque des violations répétées ou critiques ont été détectées, et que les opérations sont bloquées temporairement ou définitivement.

**Caractéristiques :**
- Les invariants fondamentaux sont préservés (INST-1, INST-2, INST-3)
- Les opérations sont bloquées (réponse systémique R3)
- La traçabilité est maintenue (INST-7)
- La protection contre corruption reste active (INST-8)
- L'isolation est préservée (INST-3)
- La persistance reste intègre mais inaccessible pour les opérations normales

**Opérations bloquées :**
- Toutes les opérations CoreDataAPI sont bloquées depuis la source mise en quarantaine
- Les Write Intents ne peuvent pas être créées depuis la source mise en quarantaine
- Les lectures sont bloquées depuis la source mise en quarantaine
- La synchronisation est bloquée depuis la source mise en quarantaine

**Opérations possibles :**
- Les opérations d'inspection peuvent être limitées
- La traçabilité continue pour documenter la quarantaine
- Les opérations de réparation peuvent être autorisées sous autorité légitime

**Alignement contractuel :**
- Applique la réponse systémique R3 (mise en quarantaine) du Runtime Boundary Contract
- Respecte l'invariant INST-8 (protection contre corruption)
- Préserve l'intégrité en bloquant les opérations suspectes
- Respecte l'invariant INST-7 (traçabilité complète)

### 3.4. Instance désynchronisée

**Définition conceptuelle :**

Une Instance Fille est **désynchronisée** lorsqu'il existe un écart détectable entre son état local et l'état de référence de l'Instance Mère, nécessitant une synchronisation.

**Caractéristiques (Instance Fille uniquement) :**
- L'invariant INST-F-2 (copie locale synchronisée) est temporairement violé
- L'invariant INST-F-3 (synchronisation périodique) nécessite une action
- L'autonomie limitée (INST-F-4) est toujours respectée
- La soumission à validation (INST-F-5) est toujours possible
- Les opérations locales peuvent continuer dans les limites autorisées
- La synchronisation est requise pour rétablir la cohérence

**Opérations autorisées :**
- Les opérations locales peuvent continuer (autonomie limitée)
  - Cette garantie respecte **LOI-2** (le système accepte l'isolement comme état normal) : même en état désynchronisé, l'Instance Fille continue à fonctionner localement, l'isolement n'est pas traité comme une erreur bloquante.
  - Elle respecte également **LOI-3** (l'état local est souverain) : l'Instance Fille détient l'autorité locale sur son état même lorsqu'elle est désynchronisée, et la réconciliation avec l'Instance Mère est explicite et traçable.
- Les Write Intents locales peuvent être créées et appliquées localement
- Les lectures locales sont possibles
- La synchronisation peut être déclenchée

**Opérations limitées :**
- Certaines opérations peuvent être limitées jusqu'à synchronisation
- Les opérations qui nécessitent la validation définitive de la Mère sont en attente

**Alignement contractuel :**
- Respecte l'invariant INST-F-4 (autonomie limitée)
- Respecte l'invariant INST-F-5 (soumission à validation)
- Nécessite la synchronisation pour rétablir INST-F-2 et INST-F-3
- Aligné avec le Sync & Conflict Resolution Contract

### 3.5. Instance corrompue (conceptuelle)

**Définition conceptuelle :**

Une instance est **corrompue** lorsque la corruption est détectée dans la persistance, et que toutes les opérations sont bloquées jusqu'à réparation.

**Caractéristiques :**
- L'invariant INST-8 (protection contre corruption) est violé par la détection de corruption
- La garantie G-PERSIST-4 (intégrité protégée) est violée
- Toutes les opérations sont bloquées (interdiction I8 du Runtime Boundary Contract)
- La corruption est signalée immédiatement
- La traçabilité de la détection est enregistrée
- La réparation est requise avant toute reprise

**Opérations bloquées :**
- Toutes les opérations CoreDataAPI sont bloquées
- Aucune Write Intent ne peut être créée, validée, ou appliquée
- Aucune lecture n'est possible
- Aucune synchronisation n'est possible
- Aucune opération d'inspection normale n'est possible

**Opérations possibles :**
- Les opérations de réparation peuvent être autorisées sous autorité légitime
- La traçabilité de la détection continue
- Les opérations de diagnostic peuvent être limitées

**Alignement contractuel :**
- Violation de l'invariant INST-8 (corruption détectée)
- Application de l'interdiction I8 (pas de continuation après corruption) du Runtime Boundary Contract
- Application des invariants INV-CORR-* (corruption détectable, opérations bloquées, signalement immédiat)
- Nécessite la réparation selon le Persistence & Storage Contract

---

## 4. Transitions autorisées

### 4.1. Transitions normales

**Saine → Dégradée :**
- **Condition :** Charge excessive détectée, nécessitant une dégradation contrôlée
- **Mécanisme :** Application de la réponse systémique R4 (dégradation contrôlée)
- **Préservation :** L'intégrité est préservée, les invariants fondamentaux restent respectés
- **Réversibilité :** La transition est réversible si les conditions s'améliorent

**Dégradée → Saine :**
- **Condition :** Les conditions de charge s'améliorent, la dégradation n'est plus nécessaire
- **Mécanisme :** Retour à l'état normal, toutes les opérations redeviennent disponibles
- **Préservation :** L'intégrité est préservée pendant et après la transition

**Saine → Désynchronisée (Instance Fille uniquement) :**
- **Condition :** Écart détecté entre l'état local et l'état de référence de la Mère
- **Mécanisme :** Détection de désynchronisation lors d'une tentative de synchronisation ou d'inspection
- **Préservation :** L'autonomie limitée est préservée, les opérations locales peuvent continuer

**Désynchronisée → Saine (Instance Fille uniquement) :**
- **Condition :** Synchronisation réussie avec l'Instance Mère
- **Mécanisme :** Synchronisation complète rétablissant la cohérence
- **Préservation :** Tous les invariants sont rétablis, la cohérence est garantie

### 4.2. Transitions de récupération

**Corrompue → Réparation :**
- **Condition :** Processus de réparation initié sous autorité légitime
- **Mécanisme :** Réparation selon le Persistence & Storage Contract (resynchronisation avec Mère, restauration, ou intervention manuelle)
- **Préservation :** L'isolation est préservée pendant la réparation

**Réparation → Saine :**
- **Condition :** Réparation réussie, corruption éliminée, intégrité rétablie
- **Mécanisme :** Vérification de l'intégrité, rétablissement des invariants
- **Préservation :** Tous les invariants sont rétablis, l'intégrité est garantie

### 4.3. Transitions interdites

**Saine → Corrompue directement :**
- **Interdiction :** Une instance saine ne peut pas devenir corrompue directement sans passer par une détection de corruption
- **Justification :** La corruption doit être détectée avant d'être déclarée. Une instance saine ne peut pas "sauter" directement à l'état corrompu.

**Désynchronisée → Corrompue directement :**
- **Interdiction :** Une instance désynchronisée n'est pas corrompue. La désynchronisation est un état récupérable, pas une corruption.
- **Justification :** La désynchronisation est un écart de cohérence récupérable par synchronisation. La corruption est une altération de l'intégrité nécessitant une réparation.

**Dégradée → Corrompue directement :**
- **Interdiction :** Une instance dégradée ne devient pas corrompue directement. La dégradation préserve l'intégrité.
- **Justification :** La dégradation contrôlée préserve l'intégrité (réponse R4). La corruption est une violation de l'intégrité.

---

## 5. Distinction erreurs récupérables vs terminales

### 5.1. Erreurs récupérables

**Définition :** Les erreurs récupérables sont des situations où l'instance peut continuer à fonctionner, même de manière limitée, et où la récupération est possible sans réparation majeure.

**Types d'erreurs récupérables :**

**Dégradation :**
- **Nature :** Charge excessive, ressources limitées
- **État résultant :** Instance dégradée
- **Récupération :** Amélioration des conditions, retour à l'état sain
- **Alignement :** Réponse systémique R4 (dégradation contrôlée)

**Désynchronisation :**
- **Nature :** Écart entre Instance Fille et Instance Mère
- **État résultant :** Instance désynchronisée
- **Récupération :** Synchronisation réussie avec l'Instance Mère
- **Alignement :** Sync & Conflict Resolution Contract

**Violations temporaires :**
- **Nature :** Violations détectées mais non critiques, non répétées
- **État résultant :** Instance saine (avec rejet des opérations violantes)
- **Récupération :** Correction des violations, opérations valides continuent
- **Alignement :** Réponse systémique R1 (rejet) du Runtime Boundary Contract

### 5.2. Erreurs terminales

**Définition :** Les erreurs terminales sont des situations où l'instance ne peut plus fonctionner et où une réparation majeure est nécessaire avant toute reprise.

**Types d'erreurs terminales :**

**Corruption détectée :**
- **Nature :** Corruption de l'intégrité, de la cohérence, ou de la structure du stockage
- **État résultant :** Instance corrompue
- **Récupération :** Réparation selon le Persistence & Storage Contract (resynchronisation avec Mère, restauration, ou intervention manuelle)
- **Alignement :** Invariants INV-CORR-* du Persistence & Storage Contract, interdiction I8 du Runtime Boundary Contract

**Violations critiques répétées :**
- **Nature :** Tentatives répétées de contournement, violations de sécurité critiques
- **État résultant :** Instance en quarantaine
- **Récupération :** Intervention manuelle sous autorité légitime, levée de quarantaine
- **Alignement :** Réponse systémique R3 (mise en quarantaine) du Runtime Boundary Contract

---

## 6. Règles de stabilité

### 6.1. Quand une instance peut continuer

Une instance peut continuer à fonctionner (même de manière limitée) lorsque :

**Conditions minimales :**
- Les invariants fondamentaux sont préservés (INST-1, INST-2, INST-3, INST-6, INST-7, INST-8)
- L'intégrité n'est pas compromise (G-PERSIST-4)
- Aucune corruption n'est détectée (INST-8)
- L'isolation est maintenue (INST-3)
- La traçabilité est assurée (INST-7)

**États permettant la continuation :**
- **Instance saine :** Toutes les opérations sont autorisées
- **Instance dégradée :** Opérations limitées mais fonctionnelles, intégrité préservée
- **Instance désynchronisée (Fille) :** Opérations locales autorisées, synchronisation requise
- **Instance en quarantaine :** Opérations bloquées depuis la source mise en quarantaine, mais l'instance elle-même peut continuer pour d'autres sources

### 6.2. Quand une instance doit refuser toute opération

Une instance DOIT refuser toute opération lorsque :

**Conditions absolues :**
- La corruption est détectée (violation de INST-8, INV-CORR-*)
- L'intégrité est compromise de manière irréparable
- La persistance est corrompue et non réparable sans intervention

**États nécessitant le refus :**
- **Instance corrompue :** Toutes les opérations sont bloquées jusqu'à réparation (interdiction I8)

**Alignement contractuel :**
- Interdiction I8 du Runtime Boundary Contract : "KindMother ne commet JAMAIS l'erreur de continuer à exécuter des opérations après avoir détecté une corruption"
- Invariants INV-CORR-2 : "Aucune opération n'est exécutée sur des données corrompues"
- Invariant INST-8 : "Toute instance DOIT être protégée contre les corruptions. Si une corruption est détectée, toutes les opérations sont bloquées jusqu'à réparation"

### 6.3. Alignement avec les invariants contractuels

**Principe fondamental :**

Les règles de stabilité sont directement dérivées des invariants contractuels. Une instance peut continuer si et seulement si les invariants fondamentaux sont préservés. Une instance doit refuser toute opération si et seulement si un invariant fondamental est violé de manière irréparable.

**Mapping invariants → règles de stabilité :**

- **INST-1 (Identité unique) :** Si violé, l'instance n'est plus identifiable → refus de toute opération
- **INST-2 (Autorité exclusive) :** Si violé, l'autorité de KindMother est compromise → refus de toute opération
- **INST-3 (Isolation) :** Si violé, l'isolation est compromise → refus de toute opération
- **INST-6 (Validation obligatoire) :** Si violé, des opérations non validées peuvent être exécutées → refus de toute opération
- **INST-7 (Traçabilité) :** Si violé, la traçabilité est compromise → limitation des opérations
- **INST-8 (Protection corruption) :** Si violé (corruption détectée), toutes les opérations sont bloquées → refus de toute opération

---

## 7. Schéma conceptuel de la machine à états

```
┌─────────────────────────────────────────────────────────────────┐
│        MACHINE À ÉTATS CONCEPTUELLE D'UNE INSTANCE                │
│                                                                   │
│  ┌───────────┐                                                   │
│  │   SAINE   │ ◄─── État normal, toutes opérations autorisées   │
│  │           │                                                   │
│  │ • Tous    │                                                   │
│  │   invariants│                                                 │
│  │   respectés│                                                  │
│  │ • Toutes  │                                                   │
│  │   opérations│                                                 │
│  │   autorisées│                                                 │
│  └─────┬─────┘                                                   │
│        │                                                          │
│        │ Charge excessive → Dégradation contrôlée (R4)          │
│        │ Désynchronisation (Fille) → Détection écart              │
│        │ Corruption détectée → Blocage (I8)                       │
│        │ Violations répétées → Quarantaine (R3)                   │
│        │                                                          │
│        ├──────────────────────────────────────────────────────┐ │
│        │                                                       │ │
│        ▼                                                       ▼ │
│  ┌──────────────┐                                    ┌──────────────┐
│  │  DÉGRADÉE    │                                    │ DÉSYNCHRONISÉE│
│  │              │                                    │ (Fille uniquement)│
│  │ • Intégrité  │                                    │              │
│  │   préservée  │                                    │ • Autonomie  │
│  │ • Opérations │                                    │   limitée    │
│  │   limitées   │                                    │ • Sync requise│
│  │ • Réversible │                                    │ • Réversible │
│  └──────┬───────┘                                    └──────┬───────┘
│         │                                                    │
│         │ Conditions améliorées                             │ Sync réussie
│         │                                                   │
│         └───────────────────┬───────────────────────────────┘
│                             │
│                             ▼
│                    ┌───────────┐
│                    │   SAINE   │
│                    └───────────┘
│                             │
│                             │ Corruption détectée
│                             │ (violation INST-8)
│                             ▼
│                    ┌──────────────┐
│                    │  CORROMPUE   │
│                    │              │
│                    │ • Toutes     │
│                    │   opérations│
│                    │   bloquées  │
│                    │ • Réparation│
│                    │   requise   │
│                    └──────┬───────┘
│                           │
│                           │ Réparation réussie
│                           │ (intégrité rétablie)
│                           ▼
│                    ┌───────────┐
│                    │   SAINE   │
│                    └───────────┘
│                                                                   │
│  ┌──────────────┐                                                 │
│  │ QUARANTAINE  │ ◄─── Violations répétées/critiques (R3)        │
│  │              │                                                 │
│  │ • Opérations │                                                 │
│  │   bloquées   │                                                 │
│  │   depuis     │                                                 │
│  │   source     │                                                 │
│  │ • Intégrité  │                                                 │
│  │   préservée  │                                                 │
│  │ • Réversible │                                                 │
│  │   (intervention)│                                              │
│  └──────────────┘                                                 │
│                                                                   │
│  TRANSITIONS INTERDITES :                                        │
│  ✗ Saine → Corrompue directement (corruption doit être détectée) │
│  ✗ Désynchronisée → Corrompue (désync ≠ corruption)             │
│  ✗ Dégradée → Corrompue (dégradation préserve intégrité)        │
└─────────────────────────────────────────────────────────────────┘
```

---

## 8. Conclusion

Ce document décrit une machine d'état interne conceptuelle permettant de comprendre comment les contrats FONDATION se traduisent en états runtime pour une instance KindMother.

**Points clés :**
- Les états sont dérivés des invariants, garanties, et interdictions contractuels
- Les transitions respectent les règles contractuelles
- La distinction entre erreurs récupérables et terminales guide les réponses systémiques
- Les règles de stabilité sont alignées avec les invariants contractuels

**Nature informative :**
Ce document est purement informatif et ne crée aucune nouvelle obligation contractuelle. Il sert uniquement à illustrer comment les concepts contractuels peuvent être organisés en états logiques pour faciliter la compréhension et l'implémentation.

---

**Document créé le :** 2026-01-25  
**Version :** 1.0  
**Statut :** POST-FONDATION — Informatif, non normatif, non contractuel  
**Référence :** Miyukini Core System v2.4, KindMother Documentation, Instance Model Contract, Persistence & Storage Contract, Runtime Boundary & Enforcement Contract, Write Intent Lifecycle Contract  
**Type :** Document informatif conceptuel

---

## 9. Mini log — erreurs / warnings / arbitrages rencontrés

### Arbitrage A1 : Distinction entre désynchronisation et corruption

**Arbitrage rencontré :** Il était nécessaire de clarifier la distinction entre une instance désynchronisée (Instance Fille avec écart de cohérence récupérable) et une instance corrompue (violation de l'intégrité nécessitant une réparation).

**Décision prise :** La désynchronisation est un état récupérable par synchronisation, tandis que la corruption est une violation de l'intégrité nécessitant une réparation. Les deux états sont distincts et ne peuvent pas être confondus.

**Justification :** Cette distinction est essentielle car les réponses systémiques sont différentes : la désynchronisation permet la continuation avec synchronisation requise, tandis que la corruption bloque toutes les opérations jusqu'à réparation.

**Documentation :** Section 3.4 (Instance désynchronisée) et section 3.5 (Instance corrompue) avec distinction explicite.

### Arbitrage A2 : Transitions interdites

**Arbitrage rencontré :** Il était nécessaire de définir quelles transitions sont interdites pour éviter des états incohérents.

**Décision prise :** Les transitions directes vers l'état corrompu depuis un état sain, dégradé, ou désynchronisé sont interdites. La corruption doit être détectée avant d'être déclarée.

**Justification :** Cette règle garantit que la corruption est toujours détectée avant d'être déclarée, et que les états récupérables (dégradé, désynchronisé) ne sont pas confondus avec la corruption.

**Documentation :** Section 4.3 (Transitions interdites) avec justifications explicites.

### Arbitrage A3 : États spécifiques à l'Instance Fille

**Arbitrage rencontré :** L'état "désynchronisée" s'applique uniquement aux Instances Filles. Il était nécessaire de clarifier cette spécificité.

**Décision prise :** L'état désynchronisée est explicitement limité aux Instances Filles, car il décrit un écart avec l'Instance Mère. Une Instance Mère ne peut pas être désynchronisée (elle est la source de référence).

**Justification :** Cette limitation est alignée avec les invariants INST-F-2 et INST-F-3 qui s'appliquent uniquement aux Instances Filles.

**Documentation :** Section 3.4 (Instance désynchronisée) avec mention explicite "Instance Fille uniquement".

### Arbitrage A4 : Réversibilité de la quarantaine

**Arbitrage rencontré :** La quarantaine est-elle réversible ou permanente ? Comment une instance sort-elle de la quarantaine ?

**Décision prise :** La quarantaine est réversible sous intervention manuelle avec autorité légitime. L'instance elle-même ne peut pas sortir automatiquement de la quarantaine, car cela nécessite une décision d'autorité.

**Justification :** Cette décision est alignée avec la réponse systémique R3 (mise en quarantaine) qui peut être temporaire ou permanente selon la gravité. La levée nécessite une intervention externe.

**Documentation :** Section 3.3 (Instance en quarantaine) avec mention de la réversibilité sous intervention.

### Arbitrage A5 : Machine à états vs contrats

**Arbitrage rencontré :** Comment s'assurer que la machine à états ne crée pas de nouvelles règles contractuelles ?

**Décision prise :** Chaque état et transition est explicitement référencé aux contrats FONDATION (invariants, garanties, interdictions, réponses systémiques). Aucun nouvel état ou transition n'est introduit sans référence contractuelle.

**Justification :** Cette approche garantit que la machine à états est purement dérivative et informative, sans créer de nouvelles obligations contractuelles.

**Documentation :** Section 2 (Mapping concepts contractuels → états runtime) avec références explicites aux contrats FONDATION.

---

*Aucune autre erreur, warning, ou arbitrage rencontré lors de la rédaction de ce document.*
