# StrongFather — Decision Graph Specification

## 1. Introduction

### Objet du contrat

Ce document définit le **StrongFather — Decision Graph Specification** : un contrat normatif, non négociable, et de statut FONDATION qui établit la spécification conceptuelle du graphe de décision de StrongFather, définissant comment les évaluations sont structurées, comment les politiques sont composées, et comment les décisions sont dérivées dans le système Miyukini Core System v2.4.

Ce contrat précise la nature conceptuelle du graphe de décision, ses nœuds, ses arêtes, ses propriétés, et les règles de parcours.

### Portée

Ce contrat s'applique à **toutes les évaluations de StrongFather** et définit de manière absolue :
- la définition formelle du graphe de décision,
- les types de nœuds du graphe,
- les types d'arêtes du graphe,
- les règles de composition,
- les propriétés du graphe,
- les invariants du graphe.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **StrongFather — Policy Engine Contract** : Définit comment les politiques sont appliquées
- **StrongFather — Core Decision Contract** : Définit les types de décisions produites
- **StrongFather — Intent Model Contract** : Définit les intentions évaluées
- **[Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Conformité aux lois d'autonomie, notamment **LOI-1** (aucune dépendance externe critique) : le graphe de décision fonctionne entièrement localement

Il n'introduit aucune contradiction, et constitue la spécification formelle du graphe de décision.

---

## 2. Définition du graphe de décision

### 2.1. Nature du graphe

Le **graphe de décision** est une structure conceptuelle qui représente le processus d'évaluation d'une intention dans StrongFather. Il modélise le chemin de l'intention vers la décision à travers l'application des politiques.

**Caractéristiques du graphe :**

- **Dirigé** : Le graphe a une direction (de l'intention vers la décision)
- **Acyclique** : Le graphe ne contient pas de cycles
- **Fini** : Le graphe a un nombre fini de nœuds et d'arêtes
- **Terminant** : Tout parcours du graphe termine par une décision

### 2.2. Objectif du graphe

Le graphe de décision permet :

1. **Visualisation** : Comprendre le processus d'évaluation
2. **Validation** : Vérifier la cohérence des politiques
3. **Traçabilité** : Suivre le chemin d'une évaluation
4. **Optimisation** : Identifier les chemins critiques
5. **Audit** : Reconstruire le raisonnement

### 2.3. Abstraction conceptuelle

Le graphe de décision est une **abstraction conceptuelle**. Il ne présuppose aucune implémentation technique particulière. Il peut être implémenté de différentes manières tout en respectant cette spécification.

---

## 3. Types de nœuds

### 3.1. Nœud d'entrée (ENTRY)

**Définition :**

Le **nœud d'entrée** est le point d'entrée unique du graphe. Il représente la réception d'une intention pour évaluation.

**Caractéristiques :**

- Un seul nœud d'entrée par graphe
- Contient l'intention complète
- Contient le contexte d'évaluation
- Pas d'arête entrante

**Contenu :**

- Identifiant de l'intention
- Type d'action
- Sujet
- Contexte d'appel
- Données de l'intention

### 3.2. Nœud de validation structurelle (VALIDATION)

**Définition :**

Le **nœud de validation** vérifie la validité structurelle de l'intention avant l'évaluation des politiques.

**Caractéristiques :**

- Suit immédiatement le nœud d'entrée
- Vérifie les composants obligatoires
- Vérifie la cohérence structurelle
- Peut conduire à un rejet structurel

**Sorties possibles :**

- VALIDE : L'intention est structurellement valide
- INVALIDE : L'intention est structurellement invalide (→ rejet structurel)

### 3.3. Nœud de politique (POLICY)

**Définition :**

Un **nœud de politique** représente l'évaluation d'une politique spécifique.

**Caractéristiques :**

- Un nœud par politique évaluée
- Évalue la politique selon le contexte
- Produit un résultat d'évaluation

**Contenu :**

- Identifiant de la politique
- Type de politique
- Condition de la politique
- Contexte d'évaluation

**Sorties possibles :**

- SATISFAITE : La politique est satisfaite
- NON_SATISFAITE : La politique n'est pas satisfaite
- INDÉTERMINÉE : La politique ne peut pas être évaluée

### 3.4. Nœud de composition (COMPOSITION)

**Définition :**

Un **nœud de composition** agrège les résultats de plusieurs évaluations de politiques selon les règles de composition.

**Caractéristiques :**

- Reçoit les résultats de plusieurs nœuds de politique
- Applique les règles de composition (Policy Engine Contract)
- Produit un résultat agrégé

**Règles de composition appliquées :**

- Unanimité pour l'acceptation
- Refus prioritaire
- Ambiguïté si indétermination

**Sorties possibles :**

- TOUTES_SATISFAITES : Toutes les politiques sont satisfaites
- AU_MOINS_UNE_NON_SATISFAITE : Au moins une politique n'est pas satisfaite
- AU_MOINS_UNE_INDÉTERMINÉE : Au moins une politique est indéterminée

### 3.5. Nœud de priorité (PRIORITY)

**Définition :**

Un **nœud de priorité** calcule la priorité relative de l'intention si les politiques sont satisfaites.

**Caractéristiques :**

- Activé uniquement si la composition est TOUTES_SATISFAITES
- Applique les politiques de priorité
- Produit une valeur de priorité

**Contenu :**

- Politiques de priorité appliquées
- Critères de priorité
- Priorité calculée

### 3.6. Nœud de décision (DECISION)

**Définition :**

Un **nœud de décision** représente la production d'une décision finale.

**Caractéristiques :**

- Point de sortie du graphe
- Produit une décision complète
- Inclut la justification

**Types de nœuds de décision :**

- DECISION_ACCEPTÉE : L'intention est acceptée
- DECISION_REFUSÉE : L'intention est refusée
- DECISION_AMBIGUË : L'intention nécessite des clarifications
- DECISION_DIFFÉRÉE : L'intention dépend d'un contexte futur

---

## 4. Types d'arêtes

### 4.1. Arête de séquence (SEQUENCE)

**Définition :**

Une **arête de séquence** représente une succession obligatoire entre deux nœuds.

**Caractéristiques :**

- Le nœud cible est toujours atteint après le nœud source
- Pas de condition
- Représente un flux obligatoire

**Notation :** →

### 4.2. Arête conditionnelle (CONDITIONAL)

**Définition :**

Une **arête conditionnelle** représente une succession conditionnelle basée sur un résultat.

**Caractéristiques :**

- Le nœud cible est atteint uniquement si la condition est vraie
- La condition est basée sur le résultat du nœud source
- Représente un branchement

**Notation :** →[condition]

### 4.3. Arête de composition (AGGREGATION)

**Définition :**

Une **arête de composition** relie plusieurs nœuds sources à un nœud de composition.

**Caractéristiques :**

- Plusieurs sources vers une cible
- Représente l'agrégation de résultats
- Le nœud de composition attend tous les résultats

**Notation :** ⇒

---

## 5. Structure du graphe

### 5.1. Structure standard

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│   [ENTRY] ──→ [VALIDATION]                                 │
│                    │                                        │
│                    ├──→[INVALIDE]──→ [DECISION_REFUSÉE]    │
│                    │                                        │
│                    └──→[VALIDE]                             │
│                           │                                 │
│                           ▼                                 │
│              ┌────────────────────────┐                    │
│              │   Évaluation des       │                    │
│              │   politiques           │                    │
│              │                        │                    │
│              │  [POLICY_1]            │                    │
│              │  [POLICY_2]            │                    │
│              │  [POLICY_N]            │                    │
│              └────────┬───────────────┘                    │
│                       ⇓                                     │
│                 [COMPOSITION]                               │
│                       │                                     │
│    ┌──────────────────┼──────────────────┐                 │
│    │                  │                  │                 │
│    ▼                  ▼                  ▼                 │
│ [TOUTES_SAT]    [NON_SAT]          [INDÉT]                │
│    │                  │                  │                 │
│    ▼                  ▼                  ▼                 │
│ [PRIORITY]    [DECISION_REF]    [DECISION_AMB/DIFF]       │
│    │                                                       │
│    ▼                                                       │
│ [DECISION_ACC]                                             │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 5.2. Phases du graphe

**Phase 1 : Validation structurelle**

Nœuds : ENTRY → VALIDATION

Objectif : Vérifier la validité structurelle de l'intention.

**Phase 2 : Évaluation des politiques**

Nœuds : POLICY_1, POLICY_2, ..., POLICY_N

Objectif : Évaluer chaque politique applicable.

**Phase 3 : Composition**

Nœuds : COMPOSITION

Objectif : Agréger les résultats selon les règles de composition.

**Phase 4 : Priorité (conditionnelle)**

Nœuds : PRIORITY

Objectif : Calculer la priorité si toutes les politiques sont satisfaites.

**Phase 5 : Décision**

Nœuds : DECISION_*

Objectif : Produire la décision finale.

---

## 6. Propriétés du graphe

### 6.1. Propriétés structurelles

**PROP-1 : Unicité de l'entrée**

Le graphe possède exactement un nœud d'entrée.

**PROP-2 : Unicité de la sortie logique**

Tout parcours du graphe termine par exactement un nœud de décision.

**PROP-3 : Acyclicité**

Le graphe ne contient aucun cycle. Tout chemin de l'entrée vers une sortie est fini.

**PROP-4 : Connexité**

Tous les nœuds sont atteignables depuis le nœud d'entrée.

### 6.2. Propriétés de parcours

**PROP-5 : Déterminisme**

Pour une intention et un ensemble de politiques donnés, le parcours du graphe est déterministe.

**PROP-6 : Terminaison**

Tout parcours du graphe termine en un temps fini.

**PROP-7 : Complétude**

Toutes les politiques applicables sont évaluées avant la composition.

### 6.3. Propriétés de résultat

**PROP-8 : Unicité du résultat**

Un parcours produit exactement une décision.

**PROP-9 : Justification complète**

Le chemin parcouru constitue la justification de la décision.

---

## 7. Règles de parcours

### 7.1. Règles d'entrée

**R-PARC-1 : Entrée unique**

Le parcours commence toujours par le nœud d'entrée.

**R-PARC-2 : Validation obligatoire**

Le nœud de validation est toujours traversé après l'entrée.

### 7.2. Règles d'évaluation

**R-PARC-3 : Évaluation parallèle conceptuelle**

Les nœuds de politique peuvent être conceptuellement évalués en parallèle.

**R-PARC-4 : Indépendance des évaluations**

L'évaluation d'une politique n'influence pas l'évaluation d'une autre.

**R-PARC-5 : Attente de composition**

La composition attend tous les résultats des politiques avant de s'exécuter.

### 7.3. Règles de sortie

**R-PARC-6 : Sortie unique**

Un seul nœud de décision est atteint par parcours.

**R-PARC-7 : Sortie obligatoire**

Tout parcours doit atteindre un nœud de décision.

---

## 8. Invariants du graphe

### 8.1. Invariants structurels

**INV-GRAPH-1 : Acyclicité**

Le graphe ne contient jamais de cycle.

**INV-GRAPH-2 : Entrée unique**

Le graphe possède toujours exactement un nœud d'entrée.

**INV-GRAPH-3 : Connexité**

Tous les nœuds sont toujours atteignables depuis l'entrée.

### 8.2. Invariants de parcours

**INV-GRAPH-4 : Terminaison garantie**

Tout parcours termine toujours par un nœud de décision.

**INV-GRAPH-5 : Déterminisme garanti**

Un même parcours avec les mêmes entrées produit toujours le même résultat.

**INV-GRAPH-6 : Pas d'effet de bord**

Le parcours du graphe ne produit jamais d'effet de bord.

---

## 9. Règles de fermeture du contrat

### 9.1. Contrat fermé

Ce contrat est **fermé**. Seuls les types de nœuds, les types d'arêtes, et les propriétés explicitement définies sont valides.

### 9.2. Interdiction d'extension implicite

Aucune extension implicite n'est autorisée :

- **INTERD-GRAPH-1** : Aucun type de nœud non défini n'est reconnu
- **INTERD-GRAPH-2** : Aucun type d'arête non défini n'est reconnu
- **INTERD-GRAPH-3** : Aucune propriété non définie n'est garantie

---

## 10. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable la spécification du graphe de décision de StrongFather.

Il garantit que :
- la structure du graphe est formellement définie,
- les types de nœuds et d'arêtes sont exhaustifs,
- les propriétés du graphe sont garanties,
- les règles de parcours sont explicites,
- les invariants sont maintenus,
- le contrat est fermé et non extensible implicitement.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

## 11. Validation conceptuelle

### 11.1. Cas conformes

Les cas suivants sont **conformes** à ce contrat :

1. **Parcours complet** : Entrée → Validation (valide) → Politiques → Composition (toutes satisfaites) → Priorité → Décision acceptée.

2. **Rejet structurel** : Entrée → Validation (invalide) → Décision refusée (type structurel).

3. **Rejet de politique** : Entrée → Validation (valide) → Politiques → Composition (non satisfaite) → Décision refusée.

### 11.2. Cas de violation

Les cas suivants **violent** ce contrat :

1. **Cycle dans le graphe** : Un parcours revient à un nœud déjà visité. Viole INV-GRAPH-1.

2. **Parcours sans décision** : Un parcours se termine sans nœud de décision. Viole INV-GRAPH-4.

3. **Entrées multiples** : Le graphe a plusieurs nœuds d'entrée. Viole INV-GRAPH-2.

---

**Document créé le :** 2026-01-25  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, StrongFather Documentation Fondatrice  
**Type :** Spécification de graphe de décision non négociable

---

## 12. Mini log de génération

### Warning W1 : Abstraction conceptuelle

**Warning rencontré :** Risque de spécification trop technique.

**Décision prise :** Précision que le graphe est une abstraction conceptuelle qui ne présuppose aucune implémentation technique.

**Correction effectuée :** Section 2.3 ajoutée pour clarifier l'abstraction.

### Warning W2 : Parallélisme des évaluations

**Warning rencontré :** Les évaluations de politiques peuvent-elles être parallèles ?

**Décision prise :** Définition d'un parallélisme conceptuel (R-PARC-3) avec indépendance des évaluations (R-PARC-4).

**Correction effectuée :** Section 7.2 précise le parallélisme conceptuel.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Policy Engine Contract : Confirmée (règles de composition)
- ✅ Cohérence avec Core Decision Contract : Confirmée (types de décisions)
- ✅ Cohérence avec Intent Model Contract : Confirmée (nœud d'entrée)
- ✅ Acyclicité et terminaison : Confirmées (propriétés et invariants)

**Conclusion :** Aucune contradiction détectée.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
