# Master Butler — Internal State Machine (Informative)

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il décrit une machine d'état interne conceptuelle permettant de traduire les contrats Master Butler en logique runtime, sans exposer d'implémentation.

**Objectif pédagogique :** Ce document vise à aider les développeurs à comprendre comment les concepts contractuels se traduisent en états runtime, sans introduire de nouvelles règles contractuelles.

**Relation avec les contrats FONDATION :** Ce document fait référence aux contrats FONDATION existants mais ne les étend pas, ne les modifie pas, et ne crée aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1. Objectif

Ce document décrit une machine d'état interne conceptuelle qui permet de comprendre comment une instance Master Butler peut être modélisée en termes d'états runtime, en se basant strictement sur les invariants, garanties, et interdictions définis dans les contrats FONDATION.

### 1.2. Nature conceptuelle

Cette machine d'état est **purement conceptuelle**. Elle ne présuppose aucune implémentation technique, aucune structure de données, ou aucun mécanisme de gestion d'état. Elle sert uniquement à illustrer comment les concepts contractuels peuvent être organisés en états logiques.

### 1.3. Spécificité de Master Butler

Master Butler diffère fondamentalement des autres cores :

| Core | Nature | États typiques |
|------|--------|----------------|
| **KindMother** | Gestionnaire de données | États liés à la persistance, synchronisation, corruption |
| **StrongFather** | Moteur de décision | États liés à l'évaluation, décision, conflit |
| **Master Butler** | Registre passif | États liés à la disponibilité, intégrité, accessibilité |

En tant que **registre passif**, Master Butler a une machine d'état plus simple que les autres cores. Ses états reflètent principalement la **disponibilité et l'intégrité** du registre des capacités et permissions.

### 1.4. Sources contractuelles

Cette machine d'état est dérivée des contrats FONDATION suivants :

- **Master Butler — Documentation Fondatrice** : Invariants INV-MB-1 à INV-MB-8
- **Master Butler — Architecture & Flows** : Invariants architecturaux INV-ARCH-*, INV-DATA-*
- **Master Butler — Capability Registry Contract** : Règles de formation et gestion des capacités
- **Master Butler — Permission Registry Contract** : Règles de formation et gestion des permissions
- **[Miyukini Conceptual References — Lois Autonomie Système](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Les états illustrent notamment **LOI-1** (aucune dépendance externe critique) et **LOI-5** (coût proportionnel au hardware).

---

## 2. Mapping concepts contractuels → états runtime

### 2.1. États dérivés des invariants fondateurs

Les invariants contractuels de la Documentation Fondatrice se traduisent en propriétés d'état qui doivent toujours être vraies :

**Invariants fondateurs (INV-MB-1 à INV-MB-8) :**
- **Exhaustivité préservée** : Le registre contient toutes les capacités du système (INV-MB-1)
- **Non-décision respectée** : Aucune réponse ne contient de verdict (INV-MB-2)
- **Idempotence garantie** : Les déclarations répétées n'ont pas d'effet supplémentaire (INV-MB-3)
- **Identifiants immuables** : Les identifiants ne changent jamais après déclaration (INV-MB-4)
- **Traçabilité complète** : Toute modification est tracée (INV-MB-5)
- **Séparation capacité/permission** : Les registres sont conceptuellement séparés (INV-MB-6)
- **Pas de logique métier** : Aucune règle métier n'est appliquée (INV-MB-7)
- **Accessibilité universelle** : Tous les composants autorisés peuvent interroger (INV-MB-8)

### 2.2. États dérivés des invariants architecturaux

Les invariants architecturaux (INV-ARCH-*) se traduisent en contraintes structurelles :

- **Point d'entrée unique actif** : La surface d'entrée est opérationnelle (INV-ARCH-1)
- **Registres séparés** : Les registres capacités et permissions sont distincts (INV-ARCH-2)
- **Flux acyclique maintenu** : Le flux d'opération est unidirectionnel (INV-ARCH-3)
- **Mode lecture optimisé** : La majorité des opérations sont des lectures (INV-ARCH-4)
- **Non-décision absolue** : Aucune décision n'est produite (INV-ARCH-6)

### 2.3. États dérivés des invariants de données

Les invariants de données (INV-DATA-*) se traduisent en états d'intégrité :

- **Exhaustivité du registre** : Toutes les capacités sont recensées (INV-DATA-1)
- **Immutabilité des identifiants** : Les identifiants sont stables (INV-DATA-2)
- **Traçabilité active** : Le traceur fonctionne (INV-DATA-3)
- **Intégrité référentielle** : Les permissions référencent des capacités existantes (INV-DATA-4)

### 2.4. États dérivés des flux d'opération

Les flux d'opération définissent des états transitoires :

- **Déclaration en cours** : Une capacité est en cours d'enregistrement
- **Définition en cours** : Une permission est en cours de définition
- **Interrogation en cours** : Une requête est en cours de traitement
- **Calcul de contexte en cours** : Un contexte de capacité est en cours de calcul

---

## 3. États typiques d'un registre Master Butler

### 3.1. Registre sain

**Définition conceptuelle :**

Le registre est dans un état **sain** lorsque tous les invariants contractuels sont respectés et que toutes les opérations autorisées peuvent être effectuées.

**Caractéristiques :**
- Tous les invariants INV-MB-* sont respectés
- Tous les invariants INV-ARCH-* sont respectés
- Tous les invariants INV-DATA-* sont respectés
- Les opérations de déclaration peuvent être effectuées
- Les opérations d'interrogation peuvent être effectuées
- Les opérations de découverte peuvent être effectuées
- La traçabilité est opérationnelle
- Aucune corruption n'est détectée

**Opérations autorisées :**
- Déclaration de capacités (flux de déclaration)
- Définition de permissions (flux de définition)
- Interrogation par StrongFather (flux d'interrogation)
- Découverte de capacités (flux de découverte)
- Calcul de contexte de capacité (flux de calcul)

**Alignement contractuel :**
- Respecte tous les invariants INV-MB-1 à INV-MB-8
- Respecte tous les invariants architecturaux INV-ARCH-1 à INV-ARCH-7
- Respecte tous les invariants de données INV-DATA-1 à INV-DATA-4

### 3.2. Registre en initialisation

**Définition conceptuelle :**

Le registre est en **initialisation** lorsqu'il est en cours de démarrage et reçoit les déclarations initiales des modules et opérateurs.

**Caractéristiques :**
- Le registre est vide ou partiellement rempli
- Les modules déclarent leurs capacités (INV-MB-1 en cours de satisfaction)
- Les opérateurs définissent les permissions
- La traçabilité est active (INV-MB-5)
- Les interrogations peuvent retourner des résultats partiels

**Opérations autorisées :**
- Déclaration de capacités (prioritaire)
- Définition de permissions (après déclaration des capacités référencées)
- Interrogations (résultats potentiellement partiels)

**Opérations limitées :**
- Les interrogations peuvent retourner des résultats incomplets
- Les calculs de contexte peuvent être partiels

**Alignement contractuel :**
- INV-MB-1 (exhaustivité) en cours de satisfaction
- INV-MB-3 (idempotence) permet les redéclarations
- INV-MB-8 (accessibilité) garantit l'accès même pendant l'initialisation

### 3.3. Registre dégradé

**Définition conceptuelle :**

Le registre est dans un état **dégradé** lorsque certains invariants sont préservés mais certaines opérations sont limitées, tout en restant fonctionnel.

**Caractéristiques :**
- Les invariants fondamentaux sont préservés (INV-MB-1, INV-MB-2, INV-MB-4, INV-MB-5)
- La charge peut être excessive, nécessitant une limitation
- Les opérations de lecture (interrogation, découverte) sont prioritaires
- Les opérations d'écriture (déclaration, définition) peuvent être ralenties
- L'intégrité est préservée malgré la dégradation

**Opérations autorisées :**
- Les opérations de lecture sont autorisées (prioritaires)
- Les opérations d'écriture sont autorisées mais peuvent être ralenties

**Opérations limitées :**
- Les déclarations peuvent être mises en file d'attente
- Les calculs de contexte complexes peuvent être différés

**Alignement contractuel :**
- Respecte INV-MB-8 (accessibilité) : le registre reste accessible
- Respecte INV-ARCH-4 (lecture majoritaire) : les lectures sont prioritaires
- Conformité à LOI-5 (coût proportionnel) : la dégradation préserve les ressources

### 3.4. Registre en synchronisation

**Définition conceptuelle :**

Le registre est en **synchronisation** lorsqu'il est en cours de synchronisation avec KindMother (persistance) ou lors d'une mise à jour majeure.

**Caractéristiques :**
- Le registre se synchronise avec le support de persistance (KindMother)
- Les invariants sont temporairement vérifiés
- Les opérations de lecture restent possibles (sur les données en mémoire)
- Les opérations d'écriture peuvent être bloquées temporairement
- La cohérence est maintenue entre mémoire et persistance

**Opérations autorisées :**
- Les opérations de lecture sont autorisées (données en mémoire)
- Les interrogations retournent les données disponibles

**Opérations limitées :**
- Les déclarations peuvent être bloquées temporairement
- Les définitions peuvent être bloquées temporairement

**Alignement contractuel :**
- Respecte INV-MB-8 (accessibilité) : les lectures restent possibles
- La synchronisation préserve INV-DATA-1 (exhaustivité) et INV-DATA-3 (traçabilité)

### 3.5. Registre corrompu (conceptuel)

**Définition conceptuelle :**

Le registre est **corrompu** lorsqu'une corruption est détectée dans les données ou la structure, et que toutes les opérations sont bloquées jusqu'à réparation.

**Caractéristiques :**
- Une corruption est détectée dans le registre des capacités ou des permissions
- L'invariant INV-DATA-1 (exhaustivité) peut être violé
- L'invariant INV-DATA-4 (intégrité référentielle) peut être violé
- Toutes les opérations sont bloquées
- La corruption est signalée immédiatement
- La traçabilité de la détection est enregistrée
- La réparation est requise avant toute reprise

**Opérations bloquées :**
- Toutes les opérations de déclaration sont bloquées
- Toutes les opérations de définition sont bloquées
- Toutes les opérations d'interrogation sont bloquées
- Toutes les opérations de découverte sont bloquées

**Opérations possibles :**
- Les opérations de diagnostic peuvent être limitées
- Les opérations de réparation peuvent être autorisées sous autorité légitime
- La traçabilité de la détection continue

**Alignement contractuel :**
- Violation détectée de INV-DATA-1 ou INV-DATA-4
- Blocage préventif pour préserver INV-MB-2 (non-décision) : éviter de fournir des informations erronées
- La réparation doit rétablir tous les invariants

### 3.6. Registre inaccessible

**Définition conceptuelle :**

Le registre est **inaccessible** lorsque l'invariant INV-MB-8 (accessibilité universelle) ne peut être satisfait, bloquant toutes les opérations.

**Caractéristiques :**
- La surface d'entrée (INV-ARCH-1) est indisponible
- Les composants autorisés ne peuvent pas interroger Master Butler
- L'isolation du système est préservée
- La traçabilité peut être limitée

**Opérations bloquées :**
- Toutes les opérations externes sont bloquées
- Les interrogations de StrongFather et BondingBrother échouent

**Alignement contractuel :**
- Violation de INV-MB-8 (accessibilité universelle)
- Violation de INV-ARCH-1 (point d'entrée unique)
- La récupération doit rétablir l'accessibilité

---

## 4. Transitions autorisées

### 4.1. Transitions normales

**Initialisation → Sain :**
- **Condition :** Tous les modules ont déclaré leurs capacités, les permissions sont définies
- **Mécanisme :** Le registre est complet et cohérent
- **Préservation :** Tous les invariants sont satisfaits

**Sain → Dégradé :**
- **Condition :** Charge excessive détectée, nécessitant une limitation
- **Mécanisme :** Application d'une politique de dégradation contrôlée
- **Préservation :** L'intégrité est préservée, les invariants fondamentaux restent respectés
- **Réversibilité :** La transition est réversible si les conditions s'améliorent

**Dégradé → Sain :**
- **Condition :** Les conditions de charge s'améliorent
- **Mécanisme :** Retour à l'état normal, toutes les opérations redeviennent disponibles
- **Préservation :** L'intégrité est préservée pendant et après la transition

**Sain → Synchronisation :**
- **Condition :** Synchronisation avec KindMother déclenchée
- **Mécanisme :** Blocage temporaire des écritures, persistance en cours
- **Préservation :** Les lectures restent possibles

**Synchronisation → Sain :**
- **Condition :** Synchronisation terminée avec succès
- **Mécanisme :** Reprise de toutes les opérations
- **Préservation :** Cohérence entre mémoire et persistance

### 4.2. Transitions de récupération

**Corrompu → Réparation :**
- **Condition :** Processus de réparation initié sous autorité légitime
- **Mécanisme :** Réparation du registre (resynchronisation, restauration, correction manuelle)
- **Préservation :** L'isolation est préservée pendant la réparation

**Réparation → Sain :**
- **Condition :** Réparation réussie, corruption éliminée, intégrité rétablie
- **Mécanisme :** Vérification de l'intégrité, rétablissement des invariants
- **Préservation :** Tous les invariants sont rétablis

**Inaccessible → Sain :**
- **Condition :** Surface d'entrée rétablie, accessibilité restaurée
- **Mécanisme :** Redémarrage ou récupération de la surface d'entrée
- **Préservation :** INV-MB-8 (accessibilité) est rétabli

### 4.3. Transitions interdites

**Sain → Corrompu directement (sans détection) :**
- **Interdiction :** Un registre sain ne peut pas devenir corrompu directement sans passer par une détection de corruption
- **Justification :** La corruption doit être détectée avant d'être déclarée. Un registre sain ne peut pas "sauter" directement à l'état corrompu.

**Dégradé → Corrompu directement :**
- **Interdiction :** Un registre dégradé n'est pas corrompu. La dégradation préserve l'intégrité.
- **Justification :** La dégradation est une limitation de performances, pas une corruption des données.

**Initialisation → Corrompu directement :**
- **Interdiction :** Un registre en initialisation ne peut pas être corrompu (il est vide ou partiellement rempli).
- **Justification :** La corruption implique une altération de données existantes.

---

## 5. Distinction erreurs récupérables vs terminales

### 5.1. Erreurs récupérables

**Définition :** Les erreurs récupérables sont des situations où le registre peut continuer à fonctionner, même de manière limitée, et où la récupération est possible sans réparation majeure.

**Types d'erreurs récupérables :**

**Dégradation :**
- **Nature :** Charge excessive, ressources limitées
- **État résultant :** Registre dégradé
- **Récupération :** Amélioration des conditions, retour à l'état sain
- **Alignement :** Conformité à LOI-5 (coût proportionnel)

**Synchronisation prolongée :**
- **Nature :** Synchronisation avec KindMother plus longue que prévu
- **État résultant :** Registre en synchronisation
- **Récupération :** Fin de la synchronisation
- **Alignement :** Conformité à LOI-1 (pas de dépendance critique)

**Déclaration rejetée :**
- **Nature :** Déclaration structurellement invalide
- **État résultant :** Registre sain (la déclaration invalide est rejetée)
- **Récupération :** Le déclarant corrige et redéclare
- **Alignement :** INV-MB-3 (idempotence)

**Référence invalide :**
- **Nature :** Permission référençant une capacité inexistante
- **État résultant :** Registre sain (la définition invalide est rejetée)
- **Récupération :** Le définisseur corrige et redéfinit
- **Alignement :** INV-DATA-4 (intégrité référentielle)

### 5.2. Erreurs terminales

**Définition :** Les erreurs terminales sont des situations où le registre ne peut plus fonctionner et où une réparation majeure est nécessaire avant toute reprise.

**Types d'erreurs terminales :**

**Corruption détectée :**
- **Nature :** Corruption de l'intégrité, de la cohérence, ou de la structure du registre
- **État résultant :** Registre corrompu
- **Récupération :** Réparation (resynchronisation, restauration, intervention manuelle)
- **Alignement :** Blocage préventif pour préserver INV-MB-2 (non-décision)

**Inaccessibilité prolongée :**
- **Nature :** Surface d'entrée indisponible sans récupération possible
- **État résultant :** Registre inaccessible
- **Récupération :** Redémarrage ou intervention manuelle
- **Alignement :** Violation de INV-MB-8 (accessibilité)

---

## 6. Règles de stabilité

### 6.1. Quand un registre peut continuer

Un registre peut continuer à fonctionner (même de manière limitée) lorsque :

**Conditions minimales :**
- Les invariants fondamentaux sont préservés (INV-MB-1, INV-MB-2, INV-MB-4, INV-MB-5, INV-MB-6, INV-MB-7)
- L'intégrité des données n'est pas compromise (INV-DATA-1, INV-DATA-4)
- Aucune corruption n'est détectée
- L'accessibilité est maintenue (INV-MB-8)

**États permettant la continuation :**
- **Registre sain :** Toutes les opérations sont autorisées
- **Registre en initialisation :** Déclarations et interrogations autorisées
- **Registre dégradé :** Opérations limitées mais fonctionnelles, intégrité préservée
- **Registre en synchronisation :** Lectures autorisées, écritures temporairement bloquées

### 6.2. Quand un registre doit refuser toute opération

Un registre DOIT refuser toute opération lorsque :

**Conditions absolues :**
- La corruption est détectée (violation de INV-DATA-1 ou INV-DATA-4)
- L'intégrité est compromise de manière irréparable
- L'accessibilité est totalement perdue (violation de INV-MB-8)

**États nécessitant le refus :**
- **Registre corrompu :** Toutes les opérations sont bloquées jusqu'à réparation
- **Registre inaccessible :** Toutes les opérations externes sont bloquées

**Justification contractuelle :**
- Préserver INV-MB-2 (non-décision) : éviter de fournir des informations erronées qui pourraient conduire à des décisions incorrectes de StrongFather
- Préserver l'intégrité du système : éviter la propagation de données corrompues

### 6.3. Alignement avec les invariants contractuels

**Principe fondamental :**

Les règles de stabilité sont directement dérivées des invariants contractuels. Un registre peut continuer si et seulement si les invariants fondamentaux sont préservés. Un registre doit refuser toute opération si et seulement si un invariant fondamental est violé de manière irréparable.

**Mapping invariants → règles de stabilité :**

- **INV-MB-1 (Exhaustivité)** : Si violé de manière irréparable → refus de toute opération
- **INV-MB-2 (Non-décision)** : Si le registre risque de fournir des informations erronées → refus de toute opération
- **INV-MB-4 (Immutabilité)** : Si violé → corruption détectée → refus de toute opération
- **INV-MB-5 (Traçabilité)** : Si violé → limitation des opérations
- **INV-MB-8 (Accessibilité)** : Si violé → refus de toute opération externe

---

## 7. Schéma conceptuel de la machine à états

```
┌─────────────────────────────────────────────────────────────────────────────┐
│           MACHINE À ÉTATS CONCEPTUELLE D'UN REGISTRE MASTER BUTLER           │
│                                                                             │
│  ┌───────────────────┐                                                     │
│  │  INITIALISATION   │ ◄─── Démarrage, déclarations en cours               │
│  │                   │                                                     │
│  │ • Registre vide   │                                                     │
│  │   ou partiel      │                                                     │
│  │ • Déclarations    │                                                     │
│  │   en cours        │                                                     │
│  │ • Interrogations  │                                                     │
│  │   partielles      │                                                     │
│  └─────────┬─────────┘                                                     │
│            │                                                               │
│            │ Toutes capacités déclarées                                    │
│            │ Permissions définies                                          │
│            ▼                                                               │
│  ┌───────────────────┐                                                     │
│  │       SAIN        │ ◄─── État normal, toutes opérations autorisées      │
│  │                   │                                                     │
│  │ • Tous invariants │                                                     │
│  │   respectés       │                                                     │
│  │ • Toutes          │                                                     │
│  │   opérations      │                                                     │
│  │   autorisées      │                                                     │
│  └─────────┬─────────┘                                                     │
│            │                                                               │
│            ├─────────────────────────────────────────────────────┐         │
│            │                                                     │         │
│            │ Charge excessive                                   │ Sync     │
│            │                                                     │ avec    │
│            ▼                                                     ▼ KM      │
│  ┌───────────────────┐                             ┌───────────────────┐   │
│  │     DÉGRADÉ       │                             │  SYNCHRONISATION  │   │
│  │                   │                             │                   │   │
│  │ • Intégrité       │                             │ • Lectures OK     │   │
│  │   préservée       │                             │ • Écritures       │   │
│  │ • Lectures        │                             │   bloquées temp.  │   │
│  │   prioritaires    │                             │ • Persistance     │   │
│  │ • Écritures       │                             │   en cours        │   │
│  │   ralenties       │                             │                   │   │
│  └─────────┬─────────┘                             └─────────┬─────────┘   │
│            │                                                 │             │
│            │ Charge normale                                  │ Sync OK     │
│            │                                                 │             │
│            └───────────────────────┬─────────────────────────┘             │
│                                    │                                       │
│                                    ▼                                       │
│                          ┌───────────────────┐                             │
│                          │       SAIN        │                             │
│                          └───────────────────┘                             │
│                                    │                                       │
│                                    │ Corruption détectée                   │
│                                    │ (violation INV-DATA-*)                │
│                                    ▼                                       │
│                          ┌───────────────────┐                             │
│                          │     CORROMPU      │                             │
│                          │                   │                             │
│                          │ • Toutes          │                             │
│                          │   opérations      │                             │
│                          │   bloquées        │                             │
│                          │ • Réparation      │                             │
│                          │   requise         │                             │
│                          └─────────┬─────────┘                             │
│                                    │                                       │
│                                    │ Réparation réussie                    │
│                                    │ (intégrité rétablie)                  │
│                                    ▼                                       │
│                          ┌───────────────────┐                             │
│                          │       SAIN        │                             │
│                          └───────────────────┘                             │
│                                                                             │
│  ┌───────────────────┐                                                     │
│  │   INACCESSIBLE    │ ◄─── Surface d'entrée indisponible                  │
│  │                   │                                                     │
│  │ • Opérations      │                                                     │
│  │   externes        │                                                     │
│  │   bloquées        │                                                     │
│  │ • Violation       │                                                     │
│  │   INV-MB-8        │                                                     │
│  └─────────┬─────────┘                                                     │
│            │                                                               │
│            │ Surface rétablie                                              │
│            ▼                                                               │
│  ┌───────────────────┐                                                     │
│  │       SAIN        │                                                     │
│  └───────────────────┘                                                     │
│                                                                             │
│  TRANSITIONS INTERDITES :                                                  │
│  ✗ Sain → Corrompu directement (corruption doit être détectée)            │
│  ✗ Dégradé → Corrompu (dégradation préserve l'intégrité)                  │
│  ✗ Initialisation → Corrompu (pas de données à corrompre)                 │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 8. Comportement spécifique : Non-décision absolue

### 8.1. Invariant central

L'invariant **INV-MB-2 (Non-décision)** est central à Master Butler. Contrairement aux autres cores, Master Butler ne prend **jamais** de décision. Cette caractéristique influence sa machine d'état :

**Impact sur les états :**

| État | Impact de INV-MB-2 |
|------|-------------------|
| **Sain** | Les réponses contiennent des informations, jamais des verdicts |
| **Dégradé** | Les réponses restent informatives, même limitées |
| **Corrompu** | Le blocage est préventif : éviter de fournir des informations erronées |
| **Inaccessible** | Le blocage protège contre l'absence de réponse (pire qu'une réponse erronée) |

### 8.2. Conséquence sur la corruption

La corruption dans Master Butler est particulièrement grave car elle peut conduire StrongFather à prendre des décisions basées sur des informations erronées :

```
Master Butler corrompu → Informations erronées → StrongFather mal informé
                                               → Décision incorrecte
                                               → Violation de sécurité potentielle
```

C'est pourquoi l'état **corrompu** bloque toutes les opérations : mieux vaut ne pas répondre que de répondre avec des informations fausses.

---

## 9. Relation avec les Lois d'Autonomie Système

### 9.1. LOI-1 : Aucune dépendance externe critique

Les états de Master Butler respectent LOI-1 :

- **État sain** : Fonctionne localement sans dépendance externe
- **État dégradé** : Continue à fonctionner avec ressources limitées
- **État synchronisation** : La synchronisation avec KindMother est non bloquante pour les lectures

### 9.2. LOI-5 : Coût proportionnel au hardware

Les états de Master Butler respectent LOI-5 :

- **Registre passif** : Empreinte minimale sur les ressources
- **Dégradation contrôlée** : Limite l'utilisation des ressources
- **Pas de workers permanents** : Consommation à la demande

---

## 10. Conclusion

Ce document décrit une machine d'état interne conceptuelle permettant de comprendre comment les contrats FONDATION se traduisent en états runtime pour un registre Master Butler.

**Points clés :**
- Les états sont dérivés des invariants, garanties, et interdictions contractuels
- La nature de **registre passif** de Master Butler simplifie sa machine d'état
- L'invariant **INV-MB-2 (Non-décision)** est central et influence tous les états
- Les transitions respectent les règles contractuelles
- La distinction entre erreurs récupérables et terminales guide les réponses systémiques
- Les règles de stabilité sont alignées avec les invariants contractuels

**Nature informative :**
Ce document est purement informatif et ne crée aucune nouvelle obligation contractuelle. Il sert uniquement à illustrer comment les concepts contractuels peuvent être organisés en états logiques pour faciliter la compréhension et l'implémentation.

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** POST-FONDATION — Informatif, non normatif, non contractuel  
**Référence :** Miyukini Core System v2.4, Master Butler Documentation Fondatrice, Master Butler Architecture & Flows, Master Butler Capability Registry Contract, Master Butler Permission Registry Contract  
**Type :** Document informatif conceptuel

---

## 11. Mini log — erreurs / warnings / arbitrages rencontrés

### Arbitrage A1 : Simplicité de la machine d'état

**Arbitrage rencontré :** Master Butler étant un registre passif, sa machine d'état est-elle nécessairement plus simple que celle de KindMother ?

**Décision prise :** Oui. Master Butler a moins d'états que KindMother car il ne gère pas de données métier, ne synchronise pas d'instances, et ne prend pas de décisions. Les états reflètent principalement la disponibilité et l'intégrité du registre.

**Justification :** La Documentation Fondatrice définit Master Butler comme un "registre vivant" qui "ne décide jamais, n'exécute jamais". Cette nature passive implique une machine d'état plus simple.

**Documentation :** Section 1.3 (Spécificité de Master Butler) avec comparaison des cores.

### Arbitrage A2 : État d'initialisation

**Arbitrage rencontré :** Faut-il un état d'initialisation distinct de l'état sain ?

**Décision prise :** Oui. L'état d'initialisation est nécessaire car l'invariant INV-MB-1 (exhaustivité) ne peut être satisfait que progressivement lors du démarrage.

**Justification :** Les modules déclarent leurs capacités lors de leur démarrage. Pendant cette phase, le registre est incomplet mais fonctionnel.

**Documentation :** Section 3.2 (Registre en initialisation).

### Arbitrage A3 : Impact de INV-MB-2 sur la corruption

**Arbitrage rencontré :** Pourquoi la corruption est-elle particulièrement grave pour Master Butler ?

**Décision prise :** La corruption peut conduire StrongFather à prendre des décisions basées sur des informations erronées, ce qui est pire qu'une absence de réponse.

**Justification :** Master Butler informe StrongFather. Des informations erronées peuvent conduire à des décisions incorrectes avec des conséquences de sécurité.

**Documentation :** Section 8 (Comportement spécifique : Non-décision absolue).

### Arbitrage A4 : État inaccessible vs corrompu

**Arbitrage rencontré :** Faut-il distinguer l'état inaccessible de l'état corrompu ?

**Décision prise :** Oui. L'inaccessibilité est une violation de INV-MB-8 (accessibilité) distincte de la corruption (violation de INV-DATA-*).

**Justification :** Les causes et les récupérations sont différentes : l'inaccessibilité peut être résolue par un redémarrage, la corruption nécessite une réparation des données.

**Documentation :** Sections 3.5 et 3.6 avec distinction explicite.

---

*Aucune autre erreur, warning, ou arbitrage rencontré lors de la rédaction de ce document.*
