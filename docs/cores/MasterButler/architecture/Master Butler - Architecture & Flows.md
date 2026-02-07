# Master Butler — Architecture & Flows

## 1. Introduction

### Objet du contrat

Ce document définit le **Master Butler — Architecture & Flows** : un contrat normatif, non négociable, et de statut FONDATION qui établit l'architecture conceptuelle de Master Butler et les flux de gestion des capacités et permissions, définissant comment les composants internes de Master Butler sont organisés et comment les opérations transitent à travers le système dans le Miyukini Core System v2.4.

Ce contrat précise l'architecture conceptuelle, les composants internes, les flux de déclaration, de définition, de découverte et d'interrogation, et les interactions entre composants.

### Portée

Ce contrat s'applique à **toute l'architecture de Master Butler** et définit de manière absolue :
- l'architecture conceptuelle de Master Butler,
- les composants internes et leurs responsabilités,
- les flux de déclaration et d'interrogation,
- les interactions entre composants,
- les invariants architecturaux.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat **synthétise et illustre** l'architecture définie dans :
- **Master Butler — Documentation Fondatrice** : Positionnement architectural
- **Master Butler — Capability Registry Contract** : Modèle du registre des capacités
- **Master Butler — Permission Registry Contract** : Modèle du registre des permissions
- **Master Butler — Boundary & Scope Contract** : Frontières et limites d'autorité

Ce contrat ne contredit aucun autre contrat et constitue une vue architecturale consolidée.

---

## 2. Architecture conceptuelle

### 2.1. Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              ÉCOSYSTÈME MIYUKINI                             │
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                           OPÉRATEURS                                   │  │
│  │                                                                       │  │
│  │   ┌───────────────────────────────────────────────────────────────┐  │  │
│  │   │              DÉCLARATION / INTERROGATION                       │  │  │
│  │   │                                                               │  │  │
│  │   │   [Capacités]                      [Permissions]              │  │  │
│  │   │       │                                 │                     │  │  │
│  │   │       ▼                                 ▼                     │  │  │
│  │   │   ┌─────────────────────────────────────────────────────┐    │  │  │
│  │   │   │                  MASTER BUTLER                       │    │  │  │
│  │   │   │           (Capability & Permission Core)             │    │  │  │
│  │   │   │                                                     │    │  │  │
│  │   │   │   ┌─────────────┐       ┌─────────────┐            │    │  │  │
│  │   │   │   │  Registre   │◀─────▶│  Registre   │            │    │  │  │
│  │   │   │   │ Capacités   │       │ Permissions │            │    │  │  │
│  │   │   │   └─────────────┘       └─────────────┘            │    │  │  │
│  │   │   │                                                     │    │  │  │
│  │   │   └─────────────────────────────────────────────────────┘    │  │  │
│  │   │                         │                                    │  │  │
│  │   │                         ▼                                    │  │  │
│  │   │            ┌─────────────────────────┐                       │  │  │
│  │   │            │  StrongFather           │                       │  │  │
│  │   │            │  (interrogation)        │                       │  │  │
│  │   │            └─────────────────────────┘                       │  │  │
│  │   │                                                               │  │  │
│  │   └───────────────────────────────────────────────────────────────┘  │  │
│  │                                                                       │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                       MODULES SPM                                      │  │
│  │                  (déclarent leurs capacités)                          │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                           KERNEL                                       │  │
│  │                     (Id, Clock, Logger)                               │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2. Positionnement de Master Butler

**Master Butler est un registre central** :

- Il n'est pas un moteur de décision (c'est StrongFather)
- Il n'est pas un exécutant (c'est les Tools)
- Il n'est pas un gestionnaire de données (c'est KindMother)
- Il est le catalogue exhaustif des capacités et permissions du système

**Nature fondamentale :**

| Aspect | Description |
|--------|-------------|
| **Type** | Registre passif, interrogeable |
| **Rôle** | Recenser, cataloguer, exposer |
| **Autorité** | Aucune (ne décide jamais) |
| **Persistance** | Via KindMother (indirecte) |

**Dépendances :**

- Master Butler ne dépend d'aucun composant externe pour ses opérations fondamentales (conformité à **LOI-1** : aucune dépendance externe critique)
- Master Butler reçoit les déclarations des modules et opérateurs
- Master Butler peut utiliser KindMother pour persister son registre

Cette architecture respecte les lois d'autonomie système définies dans [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md).

---

## 3. Composants internes de Master Butler

### 3.1. Surface d'entrée

**Définition :**

La **surface d'entrée** est le point d'accès unique de Master Butler. Elle reçoit les déclarations, les définitions, et les requêtes d'interrogation.

**Responsabilités :**

- Recevoir les déclarations de capacités des modules
- Recevoir les définitions de permissions des opérateurs
- Recevoir les requêtes d'interrogation de StrongFather et BondingBrother
- Router vers le composant approprié

**Caractéristiques :**

- Point d'entrée unique (pas d'entrées multiples)
- Interface conceptuelle standardisée
- Pas de logique métier

### 3.2. Validateur de déclaration

**Définition :**

Le **validateur de déclaration** vérifie la validité structurelle des déclarations de capacités et des définitions de permissions avant leur enregistrement.

**Responsabilités :**

- Vérifier la structure des déclarations de capacités
- Vérifier la structure des définitions de permissions
- Vérifier l'unicité des identifiants
- Vérifier les références aux capacités existantes (pour les permissions)
- Rejeter les déclarations structurellement invalides

**Règles appliquées :**

- Capability Registry Contract : règles de formation des capacités
- Permission Registry Contract : règles de formation des permissions

### 3.3. Registre des capacités

**Définition :**

Le **registre des capacités** est la structure centrale qui stocke l'inventaire exhaustif des capacités du système.

**Responsabilités :**

- Stocker les capacités déclarées
- Maintenir les métadonnées des capacités
- Gérer les relations entre capacités (dépendances, hiérarchies)
- Répondre aux requêtes de recherche de capacités
- Historiser les modifications

**Contenu du registre :**

| Élément | Description |
|---------|-------------|
| **Identifiant** | Identifiant unique et immuable de la capacité |
| **Nom** | Nom humainement lisible |
| **Description** | Description de la capacité |
| **Module d'origine** | Module ou opérateur qui a déclaré la capacité |
| **Type** | Type de capacité (action, lecture, écriture, etc.) |
| **Métadonnées** | Informations supplémentaires |
| **Date de déclaration** | Horodatage de la déclaration |
| **Version** | Version de la capacité |

### 3.4. Registre des permissions

**Définition :**

Le **registre des permissions** est la structure centrale qui stocke l'inventaire exhaustif des permissions définies dans le système.

**Responsabilités :**

- Stocker les permissions définies
- Maintenir les associations permissions-capacités
- Gérer les métadonnées des permissions
- Répondre aux requêtes de recherche de permissions
- Historiser les modifications

**Contenu du registre :**

| Élément | Description |
|---------|-------------|
| **Identifiant** | Identifiant unique de la permission |
| **Nom** | Nom humainement lisible |
| **Description** | Description de la permission |
| **Capacités associées** | Liste des capacités couvertes par cette permission |
| **Niveau** | Niveau de la permission (si applicable) |
| **Métadonnées** | Informations supplémentaires |
| **Date de définition** | Horodatage de la définition |

### 3.5. Moteur de recherche

**Définition :**

Le **moteur de recherche** traite les requêtes de découverte et d'interrogation sur les capacités et permissions.

**Responsabilités :**

- Rechercher des capacités par critères (module, type, nom)
- Rechercher des permissions par critères
- Calculer les contextes de capacité
- Filtrer les résultats selon le contexte d'interrogation

**Types de recherches :**

| Type | Description |
|------|-------------|
| **Par module** | Capacités d'un module spécifique |
| **Par type** | Capacités d'un type d'action |
| **Par permission** | Capacités associées à une permission |
| **Par contexte** | Capacités accessibles dans un contexte donné |

### 3.6. Calculateur de contexte

**Définition :**

Le **calculateur de contexte** détermine les capacités et permissions disponibles dans une situation donnée.

**Responsabilités :**

- Recevoir un contexte (identité, rôles, module cible)
- Déterminer les capacités accessibles
- Déterminer les permissions applicables
- Produire un contexte de capacité complet

**Entrées :**

- Identité du demandeur
- Rôles du demandeur
- Module ou composant ciblé

**Sortie :**

- Contexte de capacité (capacités accessibles + permissions applicables)

### 3.7. Producteur de réponse

**Définition :**

Le **producteur de réponse** génère les réponses aux interrogations de Master Butler.

**Responsabilités :**

- Formater les réponses aux requêtes
- Assembler les métadonnées demandées
- Produire des réponses structurées et cohérentes

**Types de réponses :**

| Type | Description |
|------|-------------|
| **Confirmation** | Confirmation d'enregistrement |
| **Liste** | Liste de capacités ou permissions |
| **Contexte** | Contexte de capacité complet |
| **Existence** | Réponse à une vérification d'existence |

### 3.8. Traceur

**Définition :**

Le **traceur** enregistre les traces de toutes les opérations de Master Butler pour audit et diagnostic.

**Responsabilités :**

- Tracer les déclarations de capacités
- Tracer les définitions de permissions
- Tracer les interrogations
- Tracer les modifications du registre

**Règles appliquées :**

- Toute modification du registre est tracée
- Toute interrogation peut être tracée (selon configuration)
- Les traces incluent le contexte (qui, quand, quoi)

---

## 4. Flux d'opération

### 4.1. Flux de déclaration de capacité

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    FLUX DE DÉCLARATION DE CAPACITÉ                           │
│                                                                             │
│   [Module / Opérateur]                                                      │
│        │                                                                    │
│        │ Déclaration de capacité                                           │
│        ▼                                                                    │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  1. SURFACE D'ENTRÉE                                                 │  │
│   │     - Réception de la déclaration                                   │  │
│   │     - Routage vers le validateur                                    │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  2. VALIDATEUR DE DÉCLARATION                                        │  │
│   │     - Vérification structurelle                                      │  │
│   │     - Vérification de l'unicité de l'identifiant                    │  │
│   │     - Si invalide → Rejet avec raison                               │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │ (si valide)                                  │
│                              ▼                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  3. REGISTRE DES CAPACITÉS                                           │  │
│   │     - Enregistrement de la capacité                                  │  │
│   │     - Stockage des métadonnées                                       │  │
│   │     - Mise à jour des index                                          │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  4. PRODUCTEUR DE RÉPONSE                                            │  │
│   │     - Génération de la confirmation                                  │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   [Module / Opérateur] ◀──────── Confirmation                              │
│                                                                             │
│   ══════════════════════════════════════════════════════════════════════   │
│   │ TRACEUR (en parallèle)                                               │  │
│   │   - Trace de déclaration                                             │  │
│   │   - Contexte (qui, quand, quoi)                                      │  │
│   ══════════════════════════════════════════════════════════════════════   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Caractéristiques :**

- Idempotent : déclarer deux fois la même capacité n'a pas d'effet supplémentaire
- Traçable : chaque déclaration est enregistrée
- Validé : seules les déclarations structurellement valides sont acceptées

### 4.2. Flux de définition de permission

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    FLUX DE DÉFINITION DE PERMISSION                          │
│                                                                             │
│   [Opérateur]                                                               │
│        │                                                                    │
│        │ Définition de permission                                          │
│        ▼                                                                    │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  1. SURFACE D'ENTRÉE                                                 │  │
│   │     - Réception de la définition                                    │  │
│   │     - Routage vers le validateur                                    │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  2. VALIDATEUR DE DÉCLARATION                                        │  │
│   │     - Vérification structurelle                                      │  │
│   │     - Vérification de l'existence des capacités référencées         │  │
│   │     - Si invalide → Rejet avec raison                               │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │ (si valide)                                  │
│                              ▼                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  3. REGISTRE DES PERMISSIONS                                         │  │
│   │     - Enregistrement de la permission                                │  │
│   │     - Association aux capacités                                      │  │
│   │     - Stockage des métadonnées                                       │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  4. PRODUCTEUR DE RÉPONSE                                            │  │
│   │     - Génération de la confirmation                                  │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   [Opérateur] ◀──────── Confirmation                                       │
│                                                                             │
│   ══════════════════════════════════════════════════════════════════════   │
│   │ TRACEUR (en parallèle)                                               │  │
│   │   - Trace de définition                                              │  │
│   │   - Contexte (qui, quand, quoi)                                      │  │
│   ══════════════════════════════════════════════════════════════════════   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Caractéristiques :**

- Référentielle : une permission doit référencer des capacités existantes
- Traçable : chaque définition est enregistrée
- Validé : seules les définitions valides sont acceptées

### 4.3. Flux de découverte de capacités

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    FLUX DE DÉCOUVERTE DE CAPACITÉS                           │
│                                                                             │
│   [Opérateur / BondingBrother]                                              │
│        │                                                                    │
│        │ Requête de découverte                                             │
│        ▼                                                                    │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  1. SURFACE D'ENTRÉE                                                 │  │
│   │     - Réception de la requête                                       │  │
│   │     - Routage vers le moteur de recherche                           │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  2. MOTEUR DE RECHERCHE                                              │  │
│   │     - Analyse des critères de recherche                              │  │
│   │     - Interrogation du registre des capacités                       │  │
│   │     - Filtrage selon le contexte (si applicable)                    │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  3. PRODUCTEUR DE RÉPONSE                                            │  │
│   │     - Formatage de la liste des capacités                           │  │
│   │     - Inclusion des métadonnées demandées                           │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   [Opérateur / BondingBrother] ◀──────── Liste des capacités               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Caractéristiques :**

- Lecture seule : ne modifie pas le registre
- Filtrable : peut être filtré par module, type, ou contexte
- Complet : retourne les métadonnées des capacités

### 4.4. Flux d'interrogation par StrongFather

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                 FLUX D'INTERROGATION PAR STRONGFATHER                        │
│                                                                             │
│   [StrongFather]                                                            │
│        │                                                                    │
│        │ "Cette capacité existe-t-elle ?"                                  │
│        │ "Quelles permissions sont requises ?"                             │
│        ▼                                                                    │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  1. SURFACE D'ENTRÉE                                                 │  │
│   │     - Réception de l'interrogation                                  │  │
│   │     - Identification du type de requête                             │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  2. MOTEUR DE RECHERCHE                                              │  │
│   │     - Vérification d'existence de la capacité                       │  │
│   │     - Recherche des permissions associées                           │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  3. PRODUCTEUR DE RÉPONSE                                            │  │
│   │     - Formatage de la réponse                                        │  │
│   │     - Informations de capacité                                       │  │
│   │     - Permissions requises                                           │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   [StrongFather] ◀──────── Informations                                    │
│                                                                             │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │   NOTE : Master Butler répond avec des INFORMATIONS                  │  │
│   │          Master Butler ne suggère JAMAIS de décision                 │  │
│   │          StrongFather utilise ces informations pour DÉCIDER          │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Caractéristiques :**

- Informatif : fournit des informations, pas des décisions
- Exhaustif : retourne toutes les informations pertinentes
- Neutre : ne suggère jamais de verdict

### 4.5. Flux de calcul de contexte de capacité

```
┌─────────────────────────────────────────────────────────────────────────────┐
│              FLUX DE CALCUL DE CONTEXTE DE CAPACITÉ                          │
│                                                                             │
│   [BondingBrother]                                                          │
│        │                                                                    │
│        │ Demande de contexte de capacité                                   │
│        │ (identité, rôles, module cible)                                   │
│        ▼                                                                    │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  1. SURFACE D'ENTRÉE                                                 │  │
│   │     - Réception du contexte d'entrée                                │  │
│   │     - Routage vers le calculateur                                   │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  2. CALCULATEUR DE CONTEXTE                                          │  │
│   │     - Récupération des capacités du module cible                    │  │
│   │     - Récupération des permissions des rôles                        │  │
│   │     - Calcul des capacités accessibles                              │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  3. PRODUCTEUR DE RÉPONSE                                            │  │
│   │     - Assemblage du contexte de capacité                            │  │
│   │     - Capacités accessibles                                          │  │
│   │     - Permissions applicables                                        │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   [BondingBrother] ◀──────── Contexte de capacité                          │
│                                                                             │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │   NOTE : Le contexte de capacité est une PROJECTION                  │  │
│   │          Il indique ce qui EST POSSIBLE, pas ce qui EST AUTORISÉ     │  │
│   │          L'AUTORISATION reste du ressort de StrongFather             │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Caractéristiques :**

- Projection : indique ce qui est possible, pas ce qui est autorisé
- Contextuel : dépend de l'identité, des rôles, et du module cible
- Informatif : ne modifie pas le registre

---

## 5. Interactions entre composants

### 5.1. Règles d'interaction

**R-INTER-1 : Point d'entrée unique**

Toutes les opérations passent par la surface d'entrée.

**R-INTER-2 : Flux unidirectionnel**

Le flux d'opération est unidirectionnel : de l'entrée vers la sortie.

**R-INTER-3 : Pas de callback**

Aucun composant ne rappelle un composant précédent dans le flux.

**R-INTER-4 : Indépendance du traceur**

Le traceur fonctionne en parallèle sans affecter le flux principal.

**R-INTER-5 : Isolation des registres**

Les registres des capacités et des permissions sont conceptuellement séparés, liés par les associations.

### 5.2. Dépendances entre composants

```
Surface d'entrée
        │
        ├──▶ Validateur de déclaration ──▶ Registre des capacités
        │                              └──▶ Registre des permissions
        │
        ├──▶ Moteur de recherche ──▶ Registre des capacités
        │                       └──▶ Registre des permissions
        │
        └──▶ Calculateur de contexte ──▶ Moteur de recherche
                                    └──▶ Producteur de réponse

Traceur ──▶ (observe tous les composants)
```

### 5.3. Interactions avec les Cores

**Avec StrongFather :**

```
┌────────────────┐          ┌────────────────┐
│  StrongFather  │ ──────▶  │ Master Butler  │
│                │ question │                │
│                │ ◀────── │                │
│   (décide)     │ info     │  (informe)     │
└────────────────┘          └────────────────┘
```

StrongFather interroge Master Butler pour connaître les capacités et permissions. Master Butler répond avec des informations, jamais avec des décisions.

**Avec BondingBrother :**

```
┌────────────────┐          ┌────────────────┐
│ BondingBrother │ ──────▶  │ Master Butler  │
│                │ contexte │                │
│                │ ◀────── │                │
│   (traduit)    │ capacités│  (catalogue)   │
└────────────────┘          └────────────────┘
```

BondingBrother interroge Master Butler pour connaître le contexte de capacité lors de la traduction des intentions.

**Avec KindMother :**

```
┌────────────────┐          ┌────────────────┐
│ Master Butler  │ ──────▶  │  KindMother    │
│                │ données  │                │
│  (registre)    │ ◀────── │  (persiste)    │
│                │ stockage │                │
└────────────────┘          └────────────────┘
```

Master Butler peut utiliser KindMother comme support de persistance pour son registre, mais ne gère jamais directement la persistance.

---

## 6. Gouvernance des Tools et Toolkits

### 6.1. Rôle architectural de Master Butler

Master Butler est le **catalogue central** des Tools et Toolkits dans l'environnement Miyukini.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     GOUVERNANCE DES TOOLS                                    │
│                                                                             │
│   ┌───────────────────────────────────────────────────────────────────┐    │
│   │                      MASTER BUTLER                                 │    │
│   │                                                                   │    │
│   │   ┌─────────────────────────────────────────────────────────┐    │    │
│   │   │  Catalogue des Tools                                     │    │    │
│   │   │                                                         │    │    │
│   │   │   Tool: layout.render                                   │    │    │
│   │   │   Tool: input.capture                                   │    │    │
│   │   │   Tool: file.read                                       │    │    │
│   │   │   Tool: query.execute                                   │    │    │
│   │   │   ...                                                   │    │    │
│   │   └─────────────────────────────────────────────────────────┘    │    │
│   │                                                                   │    │
│   │   ┌─────────────────────────────────────────────────────────┐    │    │
│   │   │  Catalogue des Toolkits                                  │    │    │
│   │   │                                                         │    │    │
│   │   │   Toolkit: ui.standard                                  │    │    │
│   │   │     ├─ layout.render                                    │    │    │
│   │   │     ├─ input.capture                                    │    │    │
│   │   │     └─ form.validate                                    │    │    │
│   │   │                                                         │    │    │
│   │   │   Toolkit: data.access                                  │    │    │
│   │   │     ├─ query.execute                                    │    │    │
│   │   │     └─ cache.get                                        │    │    │
│   │   └─────────────────────────────────────────────────────────┘    │    │
│   │                                                                   │    │
│   │   ┌─────────────────────────────────────────────────────────┐    │    │
│   │   │  Permissions d'accès                                     │    │    │
│   │   │                                                         │    │    │
│   │   │   Permission: tool.ui.use → [layout.render, ...]        │    │    │
│   │   │   Permission: tool.data.read → [query.execute, ...]     │    │    │
│   │   └─────────────────────────────────────────────────────────┘    │    │
│   │                                                                   │    │
│   └───────────────────────────────────────────────────────────────────┘    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 6.2. Flux d'appel d'un Tool

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        FLUX D'APPEL D'UN TOOL                                │
│                                                                             │
│   Opérateur (Strate 7)                                                      │
│        │                                                                    │
│        │ "Je veux utiliser layout.render"                                  │
│        ▼                                                                    │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  BONDING BROTHER (médiation)                                         │  │
│   │     - Traduit l'intention                                            │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  MASTER BUTLER                                                       │  │
│   │     - "Ce Tool existe-t-il ?"                                       │  │
│   │     - "Quelles permissions sont requises ?"                         │  │
│   │     - Réponse : informations sur le Tool                            │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  STRONGFATHER (décision)                                             │  │
│   │     - Évalue l'intention                                            │  │
│   │     - Vérifie les permissions                                       │  │
│   │     - Produit une décision (ACCEPTÉ / REFUSÉ)                       │  │
│   └──────────────────────────┬──────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼ (si ACCEPTÉ)                                 │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  TOOL (exécution)                                                    │  │
│   │     - Exécute l'action                                              │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 6.3. Responsabilités de Master Butler pour les Tools

| Responsabilité | Description |
|----------------|-------------|
| **Déclarer** | Quels Tools existent dans l'environnement |
| **Lier** | Capability → Tool |
| **Définir les Toolkits** | Quels Tools composent chaque Toolkit |
| **Définir les permissions** | Qui peut appeler quel Tool/Toolkit |

| Ce que Master Butler NE fait PAS | Pourquoi |
|----------------------------------|----------|
| Implémenter les Tools | Master Butler catalogue, n'implémente pas |
| Exécuter les Tools | L'exécution appartient aux Tools eux-mêmes |
| Décider de l'usage | StrongFather décide |
| Gérer le cycle de vie | Ever Buddy gère le cycle de vie |

---

## 7. Invariants architecturaux

### 7.1. Invariants de structure

**INV-ARCH-1 : Point d'entrée unique**

La surface d'entrée est le seul point d'accès de Master Butler.

**INV-ARCH-2 : Séparation des registres**

Les registres des capacités et des permissions sont conceptuellement séparés.

**INV-ARCH-3 : Flux acyclique**

Le flux d'opération est acyclique. Aucun composant ne rappelle un composant précédent.

### 7.2. Invariants de comportement

**INV-ARCH-4 : Lecture majoritaire**

La majorité des opérations de Master Butler sont des lectures (interrogations, découvertes).

**INV-ARCH-5 : Idempotence des déclarations**

Déclarer deux fois la même capacité n'a pas d'effet supplémentaire.

**INV-ARCH-6 : Non-décision absolue**

Master Butler ne prend JAMAIS de décision. Il fournit des informations, jamais des verdicts.

**INV-ARCH-7 : Traceur isolé**

Le traceur n'affecte jamais le comportement des autres composants.

### 7.3. Invariants de données

**INV-DATA-1 : Exhaustivité du registre**

Toute capacité existant dans le système est recensée dans le registre.

**INV-DATA-2 : Immutabilité des identifiants**

Les identifiants de capacités sont immuables une fois déclarés.

**INV-DATA-3 : Traçabilité complète**

Toute modification du registre est tracée avec son contexte.

**INV-DATA-4 : Intégrité référentielle**

Une permission ne peut référencer qu'une capacité existante.

---

## 8. Règles de fermeture du contrat

### 8.1. Contrat fermé

Ce contrat est **fermé**. Seuls les composants, les flux, et les interactions explicitement définis sont valides.

### 8.2. Interdiction d'extension implicite

Aucun composant, flux, ou interaction non défini n'est autorisé.

---

## 9. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable l'architecture et les flux de Master Butler.

Il garantit que :
- l'architecture est explicitement définie,
- les composants internes sont identifiés et documentés,
- les flux d'opération sont formalisés,
- les interactions sont explicites,
- les invariants architecturaux sont maintenus,
- le contrat est fermé et non extensible implicitement.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

## 10. Validation conceptuelle

### 10.1. Cas conformes

Les cas suivants sont **conformes** à ce contrat :

1. **Déclaration standard** : Un module déclare une capacité via la surface d'entrée, le validateur vérifie, le registre stocke, une confirmation est retournée.

2. **Interrogation par StrongFather** : StrongFather demande si une capacité existe, Master Butler répond avec les informations, sans suggérer de décision.

3. **Calcul de contexte** : BondingBrother demande un contexte de capacité, Master Butler calcule les capacités accessibles et retourne une projection.

### 10.2. Cas de violation

Les cas suivants **violent** ce contrat :

1. **Décision implicite** : Master Butler retourne "autorisé" ou "refusé" au lieu d'informations factuelles. Viole INV-ARCH-6.

2. **Entrée multiple** : Une déclaration contourne la surface d'entrée pour accéder directement au registre. Viole INV-ARCH-1.

3. **Référence invalide** : Une permission est définie avec une référence à une capacité inexistante. Viole INV-DATA-4.

4. **Modification d'identifiant** : Un identifiant de capacité est modifié après déclaration. Viole INV-DATA-2.

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, Master Butler Documentation Fondatrice  
**Type :** Architecture et flux non négociables

---

## 11. Mini log de génération

### Décision éditoriale E1 : Composants internes

**Décision prise :** Définition de 8 composants internes (surface d'entrée, validateur, registre capacités, registre permissions, moteur de recherche, calculateur de contexte, producteur de réponse, traceur).

**Application :** Section 3 définit chaque composant avec ses responsabilités.

### Décision éditoriale E2 : Diagrammes ASCII

**Décision prise :** Utilisation de diagrammes ASCII pour illustrer l'architecture et les flux.

**Application :** Sections 2, 4, 5, et 6 contiennent des diagrammes ASCII.

### Décision éditoriale E3 : Gouvernance des Tools

**Décision prise :** Inclusion d'une section dédiée à la gouvernance des Tools et Toolkits, conformément à la Documentation Fondatrice.

**Application :** Section 6 détaille le rôle architectural de Master Butler pour les Tools.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Documentation Fondatrice : Confirmée (positionnement)
- ✅ Cohérence avec Capability Registry Contract : Confirmée (registre des capacités)
- ✅ Cohérence avec Permission Registry Contract : Confirmée (registre des permissions)
- ✅ Cohérence avec Tools et Toolkits Reference : Confirmée (gouvernance des Tools)

**Conclusion :** Aucune contradiction détectée.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
