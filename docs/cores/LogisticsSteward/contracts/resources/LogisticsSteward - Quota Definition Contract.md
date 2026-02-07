# LogisticsSteward - Quota Definition Contract

## 1. Contexte

Ce document définit formellement ce qu'est un **quota** dans l'écosystème Miyukini, ses propriétés, ses types, et les règles d'attribution. Le quota est l'unité fondamentale de gouvernance des ressources gérée par LogisticsSteward.

**Document fondateur :** [LogisticsSteward - Documentation Fondatrice](../../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et non négociable**. Il dérive directement de la Documentation Fondatrice (Section 6.3 - Types de Règles Gérées) et du Vocabulaire Canonique (Section 12).

---

## 2. Portée / Scope

- **Applicable à :** Toute entité consommant des ressources dans l'écosystème Miyukini
- **Responsable :** LogisticsSteward (responsabilité exclusive de définition et d'attribution des quotas)
- **Consommateurs :** StrongFather (validation), Kernel (exécution), MasterButler (limitation des capacités)
- **Ne couvre pas :** L'exécution technique des limitations (responsabilité du Kernel)

---

## 3. Définition canonique du quota

### 3.1 Qu'est-ce qu'un quota ?

Un **quota** est une limite déclarée sur l'usage d'une ressource conceptuelle par une entité. Il représente un droit d'usage quantifié, défini par des règles explicites.

**Caractéristiques fondamentales :**

1. **Déclaratif** — Un quota est une déclaration de limite, pas une mesure technique
2. **Explicite** — Tout quota doit être formellement défini et documenté
3. **Déterministe** — À contexte identique, le quota calculé est toujours le même
4. **Auditable** — Toute attribution de quota est traçable avec son origine et sa justification
5. **Révisable** — Un quota peut être modifié selon des règles définies

**Ce qu'un quota n'est PAS :**

- ❌ Une mesure technique de ressource (CPU, RAM, IO)
- ❌ Un compteur d'utilisation en temps réel
- ❌ Un mécanisme de throttling technique
- ❌ Une allocation mémoire ou système
- ❌ Un scheduler ou ordonnanceur

### 3.2 Responsabilité de LogisticsSteward

LogisticsSteward est **exclusivement responsable** de la définition et de l'attribution des quotas. Cette responsabilité inclut :

- Définir les types de quotas existants
- Attribuer des quotas aux entités selon les règles
- Modifier les quotas en fonction du contexte
- Maintenir le registre exhaustif des quotas du système
- Proposer des décisions d'arbitrage basées sur les quotas

**Invariant associé :** INV-LS-5 — Toute règle (dont les quotas) est **explicite**, jamais implicite.

---

## 4. Propriétés d'un quota

Tout quota possède les propriétés obligatoires suivantes :

### 4.1 Identité

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Identifiant** | Identifiant unique et stable dans le système | ✅ Oui |
| **Nom** | Nom descriptif et non ambigu | ✅ Oui |
| **Description** | Description du quota et de sa raison d'être | ✅ Oui |
| **Date de création** | Horodatage de création du quota | ✅ Oui |

### 4.2 Définition

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Type de ressource** | Ressource conceptuelle concernée | ✅ Oui |
| **Unité** | Unité de mesure conceptuelle (requêtes, opérations, sessions) | ✅ Oui |
| **Valeur** | Valeur numérique de la limite | ✅ Oui |
| **Période** | Fenêtre temporelle d'application (si applicable) | ⚠️ Selon type |
| **Portée** | Niveau d'application (entité, équipe, global) | ✅ Oui |

### 4.3 Attribution

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Entité cible** | Entité à laquelle le quota est attribué | ✅ Oui |
| **Règle source** | Règle ayant généré cette attribution | ✅ Oui |
| **Priorité héritée** | Niveau de priorité associé à l'entité | ✅ Oui |
| **Conditions** | Conditions d'application du quota | ⚠️ Optionnel |

### 4.4 Traçabilité

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Origine** | Qui a créé ce quota | ✅ Oui |
| **Justification** | Pourquoi ce quota existe | ✅ Oui |
| **Historique** | Historique des modifications | ✅ Oui |
| **Validation** | Statut de validation par StrongFather | ✅ Oui |

**Invariant associé :** INV-LS-6 — Toute décision (dont l'attribution de quota) est **traçable** avec son origine, sa date, et sa justification.

---

## 5. Taxonomie des types de quotas

LogisticsSteward reconnaît cinq types canoniques de quotas.

### 5.1 Quota de volume

**Définition :** Limite le nombre total d'opérations ou d'unités consommables sur une période.

| Aspect | Spécification |
|--------|---------------|
| **Unité typique** | Requêtes, opérations, transactions |
| **Période typique** | Minute, heure, jour, mois |
| **Renouvellement** | À la fin de la période |
| **Usage typique** | Limitation des appels API, des créations d'entités |

**Exemples de quotas de volume :**

- Quota de 1000 requêtes API par heure par opérateur
- Quota de 50 créations d'utilisateurs par jour par équipe
- Quota de 10 exports de données par mois par service

**Propriétés spécifiques :**

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Période** | Fenêtre temporelle de renouvellement | ✅ Oui |
| **Mode de calcul** | Glissant ou fixe | ✅ Oui |
| **Report** | Report du non-consommé autorisé ou non | ✅ Oui |

### 5.2 Quota de concurrence

**Définition :** Limite le nombre d'opérations simultanées ou de ressources actives.

| Aspect | Spécification |
|--------|---------------|
| **Unité typique** | Sessions, connexions, processus actifs |
| **Période** | Non applicable (instantané) |
| **Renouvellement** | À la libération de la ressource |
| **Usage typique** | Limitation des sessions actives, des téléchargements parallèles |

**Exemples de quotas de concurrence :**

- Quota de 5 sessions actives simultanées par utilisateur
- Quota de 3 téléchargements parallèles par opérateur
- Quota de 10 connexions WebSocket simultanées par service

**Propriétés spécifiques :**

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Mode d'acquisition** | FIFO, LIFO, prioritaire | ✅ Oui |
| **Timeout** | Durée maximale d'une acquisition | ⚠️ Optionnel |
| **Préemption** | Préemption autorisée ou non | ✅ Oui |

### 5.3 Quota de capacité

**Définition :** Limite la quantité totale d'une ressource stockable ou réservable.

| Aspect | Spécification |
|--------|---------------|
| **Unité typique** | Octets, enregistrements, entités |
| **Période** | Non applicable (cumul) |
| **Renouvellement** | À la libération ou suppression |
| **Usage typique** | Limitation du stockage, du nombre d'objets |

**Exemples de quotas de capacité :**

- Quota de 10 Go de stockage par opérateur
- Quota de 1000 documents par équipe
- Quota de 50 intégrations actives par service

**Propriétés spécifiques :**

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Seuil d'alerte** | Pourcentage déclenchant une alerte | ✅ Oui |
| **Comportement saturation** | Blocage, file d'attente, rejet | ✅ Oui |
| **Nettoyage automatique** | Politique de nettoyage si applicable | ⚠️ Optionnel |

### 5.4 Quota de priorité

**Définition :** Définit le niveau de service ou de priorité d'accès aux ressources.

| Aspect | Spécification |
|--------|---------------|
| **Unité typique** | Niveau (1-10), classe (gold, silver, bronze) |
| **Période** | Non applicable (permanent jusqu'à modification) |
| **Renouvellement** | Sur décision explicite |
| **Usage typique** | Différenciation de service, QoS conceptuel |

**Exemples de quotas de priorité :**

- Quota de priorité niveau 8/10 pour MiyukiniAdmin
- Quota de priorité classe "gold" pour les opérateurs premium
- Quota de priorité niveau 3/10 pour les services non critiques

**Propriétés spécifiques :**

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Échelle** | Échelle de priorité utilisée | ✅ Oui |
| **Préemption autorisée** | Peut préempter les priorités inférieures | ✅ Oui |
| **Héritage** | Priorité héritée par les sous-entités | ✅ Oui |

### 5.5 Quota conditionnel

**Définition :** Quota dont la valeur varie selon le contexte ou les conditions du système.

| Aspect | Spécification |
|--------|---------------|
| **Unité typique** | Variable selon le quota sous-jacent |
| **Période** | Variable selon le quota sous-jacent |
| **Renouvellement** | À chaque évaluation des conditions |
| **Usage typique** | Adaptation dynamique aux conditions système |

**Exemples de quotas conditionnels :**

- Quota de 1000 requêtes/h en conditions normales, 200/h en dégradation
- Quota de 5 sessions si charge faible, 2 si charge élevée
- Quota de stockage illimité pour admin, 5 Go pour utilisateurs standard

**Propriétés spécifiques :**

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Conditions** | Ensemble des conditions évaluées | ✅ Oui |
| **Valeurs associées** | Valeur du quota pour chaque condition | ✅ Oui |
| **Valeur par défaut** | Valeur si aucune condition ne matche | ✅ Oui |
| **Fréquence réévaluation** | Quand les conditions sont réévaluées | ✅ Oui |

---

## 6. Règles d'attribution des quotas

### 6.1 Règles générales

| Règle | Description |
|-------|-------------|
| **RÈGLE-QUOTA-1** | Tout quota doit être explicitement attribué (pas de quota implicite) |
| **RÈGLE-QUOTA-2** | Toute attribution doit référencer une règle source |
| **RÈGLE-QUOTA-3** | Un quota attribué doit être validé par StrongFather avant application |
| **RÈGLE-QUOTA-4** | Les quotas s'appliquent selon la hiérarchie : global < équipe < entité |
| **RÈGLE-QUOTA-5** | En cas de conflit, le quota le plus restrictif s'applique (sauf exception validée) |

### 6.2 Hiérarchie d'attribution

Les quotas peuvent être attribués à différents niveaux, avec une hiérarchie claire :

```
┌─────────────────────────────────────────────────┐
│ NIVEAU GLOBAL                                   │
│ Quotas par défaut pour tout l'écosystème        │
│ (appliqués si aucun quota spécifique)           │
└─────────────────────────────────────────────────┘
          ▼
┌─────────────────────────────────────────────────┐
│ NIVEAU ÉQUIPE                                   │
│ Quotas spécifiques à une équipe d'opérateurs    │
│ (remplace les quotas globaux pour l'équipe)     │
└─────────────────────────────────────────────────┘
          ▼
┌─────────────────────────────────────────────────┐
│ NIVEAU ENTITÉ                                   │
│ Quotas spécifiques à une entité                 │
│ (remplace les quotas équipe pour l'entité)      │
└─────────────────────────────────────────────────┘
          ▼
┌─────────────────────────────────────────────────┐
│ NIVEAU EXCEPTION                                │
│ Quotas d'exception validés par StrongFather     │
│ (contourne la hiérarchie normale)               │
└─────────────────────────────────────────────────┘
```

### 6.3 Processus d'attribution

```
[Demande d'attribution de quota]
        │
        ▼
[LogisticsSteward]
  ├── Identification de l'entité cible
  ├── Détermination du type de quota
  ├── Calcul de la valeur selon les règles
  ├── Vérification des conflits
        │
        ▼
[Proposition d'attribution]
        │
        ▼
[StrongFather]
  └── Validation / Invalidation
        │
        ▼
[Si validé: Attribution effective]
  └── Journalisation et traçabilité
```

### 6.4 Règles de modification

| Règle | Description |
|-------|-------------|
| **RÈGLE-MOD-1** | Toute modification de quota suit le même processus que l'attribution |
| **RÈGLE-MOD-2** | Une modification doit être justifiée et tracée |
| **RÈGLE-MOD-3** | La réduction de quota est effective immédiatement |
| **RÈGLE-MOD-4** | L'augmentation de quota peut être différée selon les conditions |
| **RÈGLE-MOD-5** | MiyukiniAdmin peut demander des modifications exceptionnelles |

---

## 7. Entités concernées par les quotas

### 7.1 Opérateurs

| Aspect | Spécification |
|--------|---------------|
| **Quotas typiques** | Volume, concurrence, capacité |
| **Attribution** | Par défaut global, personnalisable |
| **Héritage** | Peut hériter de son équipe |

### 7.2 Équipes d'opérateurs

| Aspect | Spécification |
|--------|---------------|
| **Quotas typiques** | Volume partagé, capacité partagée |
| **Attribution** | Explicite à la création |
| **Distribution** | Partagé ou réparti entre membres |

### 7.3 Outils et Toolkits

| Aspect | Spécification |
|--------|---------------|
| **Quotas typiques** | Volume, concurrence |
| **Attribution** | Selon criticité déclarée |
| **Priorité** | Selon type d'outil |

### 7.4 Services exposés

| Aspect | Spécification |
|--------|---------------|
| **Quotas typiques** | Volume, concurrence, capacité |
| **Attribution** | Selon SLA déclaré |
| **Priorité** | Selon criticité du service |

### 7.5 MiyukiniAdmin

**Règles spécifiques :** MiyukiniAdmin a des quotas particuliers définis dans la Documentation Fondatrice (Section 9.1).

| Aspect | Spécification |
|--------|---------------|
| **Quotas par défaut** | Priorité maximale possible |
| **Gouvernance** | Reste soumis aux règles globales |
| **Exception** | Tout bypass nécessite un protocole d'exception |
| **Traçabilité** | Chaque exception est journalisée |

---

## 8. Adaptation des quotas selon le contexte

### 8.1 Adaptation selon le niveau de dégradation

Les quotas s'adaptent automatiquement selon le niveau de dégradation du système.

**Référence :** [LogisticsSteward - Documentation Fondatrice](../../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md) (Section 7.3 - Niveaux de Dégradation)

| Niveau | Impact sur les quotas |
|--------|----------------------|
| **D0 - Normal** | Quotas nominaux appliqués |
| **D1 - Prudent** | Quotas réduits de 10-20% pour non-critiques |
| **D2 - Restreint** | Quotas réduits de 30-50% pour non-critiques |
| **D3 - Critique** | Quotas minimaux, priorité aux services vitaux |
| **D4 - Survie** | Quotas d'urgence, seuls les quotas critiques maintenus |

### 8.2 Règles d'adaptation

| Règle | Description |
|-------|-------------|
| **RÈGLE-ADAPT-1** | L'adaptation des quotas est automatique selon l'état système |
| **RÈGLE-ADAPT-2** | L'adaptation est progressive (pas de changement brutal) |
| **RÈGLE-ADAPT-3** | L'adaptation est réversible (retour à la normale explicite) |
| **RÈGLE-ADAPT-4** | L'adaptation est traçable (chaque changement est journalisé) |
| **RÈGLE-ADAPT-5** | Les quotas critiques (admin, services vitaux) sont préservés en dernier |

---

## 9. Consommation et suivi des quotas

### 9.1 Principes de suivi

LogisticsSteward **ne mesure jamais** directement la consommation des quotas. Cette responsabilité appartient au Kernel.

**Ce que LogisticsSteward fait :**

- Définit les quotas et leurs règles
- Reçoit l'état de consommation du Kernel (état système abstrait)
- Prend des décisions d'arbitrage basées sur cet état

**Ce que LogisticsSteward ne fait pas :**

- Comptabiliser la consommation en temps réel
- Mesurer les ressources système
- Appliquer techniquement les limitations

### 9.2 État de consommation

L'état de consommation est fourni par le Kernel sous forme normalisée :

| Information | Description |
|-------------|-------------|
| **Quota concerné** | Identifiant du quota |
| **Entité concernée** | Identifiant de l'entité |
| **Valeur consommée** | Quantité déjà utilisée |
| **Valeur restante** | Quantité encore disponible |
| **Pourcentage** | Taux d'utilisation (0-100%) |
| **Statut** | Normal, alerte, saturé |

### 9.3 Seuils et alertes

| Seuil | Déclencheur | Action |
|-------|-------------|--------|
| **Seuil d'information** (50%) | Information préventive | Journalisation |
| **Seuil d'alerte** (80%) | Avertissement | Notification, journalisation |
| **Seuil critique** (95%) | Pré-saturation | Alerte, possible réduction préventive |
| **Saturation** (100%) | Quota épuisé | Arbitrage de rejet ou file d'attente |

---

## 10. Interactions avec les autres cores

### 10.1 Flux vers StrongFather

LogisticsSteward soumet à StrongFather les **attributions de quotas** pour validation :

- Nouvelles attributions de quotas
- Modifications de quotas existants
- Demandes d'exception de quota
- Conflits de quotas à trancher

### 10.2 Flux vers/depuis Kernel

**Depuis Kernel :** LogisticsSteward reçoit l'état système abstrait incluant :

- État de consommation des quotas par entité
- Niveau de charge global
- État de dégradation éventuel

**Vers Kernel :** LogisticsSteward fournit les décisions d'arbitrage à exécuter :

- Quotas validés à appliquer
- Modifications de quotas
- Décisions de limitation

### 10.3 Flux vers MasterButler

LogisticsSteward informe MasterButler des **limitations d'usage** :

- Capacités limitées par les quotas
- Services restreints selon les quotas
- Fonctionnalités désactivées par manque de quota

### 10.4 Flux vers BondingBrother

LogisticsSteward transmet via BondingBrother les **décisions d'arbitrage** :

- Notifications de quota atteint
- Décisions d'allocation
- Changements de quotas

---

## 11. Anti-patterns de définition de quotas

| Anti-pattern | Description | Pourquoi c'est interdit |
|--------------|-------------|-------------------------|
| **Quota implicite** | Quota non déclaré formellement | Viole INV-LS-5 et RÈGLE-QUOTA-1 |
| **Quota technique** | Quota basé sur des métriques techniques | Viole la séparation Kernel/LogisticsSteward |
| **Quota sans validation** | Quota appliqué sans validation StrongFather | Viole INV-LS-8 |
| **Quota non traçable** | Quota sans origine ni justification | Viole INV-LS-6 |
| **Quota auto-appliqué** | Quota appliqué directement par LogisticsSteward | Viole INV-LS-7 |
| **Quota discriminatoire** | Quota sans règle objective | Viole INV-LS-4 (déterminisme) |

---

## 12. Conformité aux Lois d'Autonomie Système

Ce contrat respecte les **Lois d'Autonomie Système** définies dans [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md).

### LOI-1 : Aucune dépendance externe critique à l'exécution

**Conformité :** ✅ Les quotas sont définis et attribués localement, sans dépendance à un service externe.

### LOI-2 : Le système accepte l'isolement comme état normal

**Conformité :** ✅ Les quotas continuent de s'appliquer en environnement isolé avec l'état local disponible.

### LOI-3 : L'état local est souverain

**Conformité :** ✅ Les quotas attribués localement sont la vérité, réconciliation explicite à la reconnexion.

### LOI-5 : Le coût doit être proportionnel au hardware

**Conformité :** ✅ La gestion des quotas est légère (déclaratif, pas de mesure technique).

---

## 13. Références croisées

### Invariants associés (Documentation Fondatrice - Section 4)

| Invariant | Énoncé | Relation |
|-----------|--------|----------|
| INV-LS-4 | Décisions déterministes | Quotas calculés de manière déterministe |
| INV-LS-5 | Règles explicites | Fondement de ce contrat |
| INV-LS-6 | Traçabilité complète | Toute attribution est traçable |
| INV-LS-7 | Séparation Kernel | LogisticsSteward n'applique pas les quotas |
| INV-LS-8 | Validation StrongFather | Quotas validés avant application |

### Documents associés

| Document | Relation |
|----------|----------|
| [LogisticsSteward - Documentation Fondatrice](../../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md) | Document source |
| [LogisticsSteward - Priority Management Contract](./LogisticsSteward%20-%20Priority%20Management%20Contract.md) | Gestion des priorités associées |
| [LogisticsSteward - Resource Arbitration Contract](./LogisticsSteward%20-%20Resource%20Arbitration%20Contract.md) | Processus d'arbitrage utilisant les quotas |
| [LogisticsSteward - Degradation Strategy Contract](../degradation/LogisticsSteward%20-%20Degradation%20Strategy%20Contract.md) | Adaptation des quotas en dégradation |
| [LogisticsSteward - Kernel Integration Contract](../integration/LogisticsSteward%20-%20Kernel%20Integration%20Contract.md) | État système et exécution |

### Références glossaire

| Terme | Définition |
|-------|------------|
| **Quota** | Limite déclarée sur l'usage d'une ressource conceptuelle par une entité |
| **Quota de volume** | Limite sur le nombre d'opérations sur une période |
| **Quota de concurrence** | Limite sur le nombre d'opérations simultanées |
| **Quota de capacité** | Limite sur la quantité totale stockable ou réservable |
| **Quota de priorité** | Définition du niveau de service ou de priorité d'accès |
| **Quota conditionnel** | Quota dont la valeur varie selon le contexte |
| **Attribution** | Processus d'assignation d'un quota à une entité |

**Source :** [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 14. Synthèse contractuelle

### Garanties de ce contrat

Ce contrat garantit que :

1. **Les quotas sont définis** — Cinq types canoniques avec propriétés explicites
2. **L'attribution est formelle** — Processus explicite avec validation StrongFather
3. **La hiérarchie est claire** — Global < Équipe < Entité < Exception
4. **L'adaptation est automatique** — Les quotas s'adaptent au niveau de dégradation
5. **La traçabilité est complète** — Toute attribution est documentée et traçable
6. **La séparation est respectée** — LogisticsSteward définit, Kernel exécute

### Phrase de synthèse

> **Un quota est une limite déclarée, explicite et traçable, attribuée à une entité selon des règles déterministes, validée par StrongFather, et exécutée par le Kernel — jamais directement par LogisticsSteward.**

---

**Version :** 1.0.0  
**Date :** 2026-01-28  
**Statut :** Contrat — Normatif  
**Référence :** LogisticsSteward v1.0, Documentation Fondatrice Section 6.3 et 12  
**Type :** Contrat de définition de quotas
