# Ever Buddy - Metrics & Alerting Contract

## Contexte

Ce contrat définit les **métriques de surveillance** et le **système d'alerte** d'Ever Buddy. En tant que core de cycle de vie et d'évolution, Ever Buddy observe en permanence l'état des éléments du système et alerte les consommateurs lorsque des conditions anormales sont détectées.

Ce document extrait et formalise les métriques et alertes décrites dans la **Section 8** de la [Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md).

## Portée / Scope

- **Applicable à :** Toute implémentation d'Ever Buddy, tout consommateur d'informations de cycle de vie
- **Audience :** Architectes, développeurs, opérateurs système, Caring Nanny
- **Statut :** Contrat normatif — OBSERVABILITY CONTRACT
- **Référence fondatrice :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md), Section 8

---

## 1. Principes fondamentaux de l'observabilité

### 1.1 Observation sans exécution

Ever Buddy observe et mesure, mais **n'exécute jamais de correction automatique**. L'invariant **INV-EB-1** (aucune exécution de migration) s'applique également aux métriques et alertes.

**Ce que Ever Buddy fait :**
- Collecter des métriques
- Calculer des indicateurs
- Émettre des alertes
- Fournir des recommandations

**Ce que Ever Buddy ne fait PAS :**
- Corriger automatiquement les anomalies
- Forcer des migrations
- Modifier des états sans validation
- Supprimer des éléments

### 1.2 Métriques conceptuelles, pas techniques

Les métriques d'Ever Buddy portent sur les **états de cycle de vie et les évolutions**, pas sur les performances techniques. Ever Buddy ne mesure pas :
- La latence des APIs
- L'utilisation mémoire
- Le temps de réponse

Ces aspects relèvent de **Caring Nanny** (observation d'état système) et non d'Ever Buddy.

### 1.3 Alimentation de Caring Nanny

Ever Buddy **alimente Caring Nanny** avec les indicateurs d'évolution qui affectent la santé du système. Cette relation est unidirectionnelle : Ever Buddy fournit, Caring Nanny observe.

---

## 2. Métriques d'état

Les métriques d'état mesurent la **distribution actuelle** des éléments du système selon leur état de [cycle de vie](../lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md).

### 2.1 Compteurs par état

| Métrique | Description | Unité |
|----------|-------------|-------|
| **COUNT_DRAFT** | Nombre d'éléments en état DRAFT | Entier ≥ 0 |
| **COUNT_ACTIVE** | Nombre d'éléments en état ACTIVE | Entier ≥ 0 |
| **COUNT_DEPRECATED** | Nombre d'éléments en état DEPRECATED | Entier ≥ 0 |
| **COUNT_RETIRED** | Nombre d'éléments en état RETIRED | Entier ≥ 0 |
| **COUNT_ARCHIVED** | Nombre d'éléments en état ARCHIVED | Entier ≥ 0 |
| **COUNT_TOTAL** | Nombre total d'éléments suivis | Entier ≥ 0 |

**Règle :** COUNT_TOTAL = COUNT_DRAFT + COUNT_ACTIVE + COUNT_DEPRECATED + COUNT_RETIRED + COUNT_ARCHIVED

### 2.2 Debt Ratio

Le **debt ratio** mesure la proportion d'éléments en fin de vie par rapport aux éléments actifs.

```
DEBT_RATIO = (COUNT_DEPRECATED + COUNT_RETIRED) / COUNT_ACTIVE
```

**Interprétation :**

| Valeur | Interprétation | Action recommandée |
|--------|----------------|-------------------|
| 0.00 - 0.10 | Sain | Aucune |
| 0.11 - 0.25 | Acceptable | Surveillance |
| 0.26 - 0.50 | Attention | Planifier nettoyage |
| 0.51 - 1.00 | Alerte | Nettoyage prioritaire |
| > 1.00 | Critique | Intervention urgente |

**Cas limite :** Si COUNT_ACTIVE = 0, le DEBT_RATIO est défini comme "UNDEFINED" (pas de division par zéro). Cette situation est elle-même une anomalie à signaler.

### 2.3 Âge moyen par état

L'**âge moyen** mesure depuis combien de temps les éléments sont dans leur état actuel.

| Métrique | Description | Unité |
|----------|-------------|-------|
| **AGE_MEAN_DRAFT** | Durée moyenne en DRAFT | Cycles de release |
| **AGE_MEAN_ACTIVE** | Durée moyenne en ACTIVE | Cycles de release |
| **AGE_MEAN_DEPRECATED** | Durée moyenne en DEPRECATED | Cycles de release |
| **AGE_MEAN_RETIRED** | Durée moyenne en RETIRED | Cycles de release |
| **AGE_MEAN_ARCHIVED** | Durée moyenne en ARCHIVED | Cycles de release |

**Note importante :** Les âges sont mesurés en **cycles de release**, pas en temps absolu, conformément à la **LOI-4** (pas de temps global requis).

### 2.4 Répartition par catégorie

Ever Buddy distingue les éléments par [catégorie](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) :

| Catégorie | Description |
|-----------|-------------|
| **FONDATION** | Contrats fondateurs |
| **OPERATIONAL** | Contrats opérationnels |
| **TECHNICAL** | Interfaces techniques |
| **INTERNAL** | Éléments internes |

**Métriques dérivées :**
- COUNT_FONDATION_DEPRECATED : Nombre de contrats fondateurs dépréciés
- COUNT_OPERATIONAL_DEPRECATED : Nombre de contrats opérationnels dépréciés
- etc.

---

## 3. Métriques de transition

Les métriques de transition mesurent le **mouvement** des éléments entre états.

### 3.1 Transitions en cours

| Métrique | Description | Unité |
|----------|-------------|-------|
| **TRANSITIONS_IN_PROGRESS** | Nombre de transitions actives | Entier ≥ 0 |
| **TRANSITIONS_DEPRECATED_TO_RETIRED** | Transitions DEPRECATED → RETIRED en cours | Entier ≥ 0 |
| **TRANSITIONS_RETIRED_TO_ARCHIVED** | Transitions RETIRED → ARCHIVED en cours | Entier ≥ 0 |

### 3.2 Durée des périodes de dépréciation

| Métrique | Description | Unité |
|----------|-------------|-------|
| **DEPRECATION_DURATION_MEAN** | Durée moyenne des périodes de dépréciation | Cycles de release |
| **DEPRECATION_DURATION_MIN** | Durée minimale observée | Cycles de release |
| **DEPRECATION_DURATION_MAX** | Durée maximale observée | Cycles de release |

### 3.3 Taux d'adoption des successeurs

Le **taux d'adoption** (adoption rate) mesure la progression de la migration vers un successeur.

```
ADOPTION_RATE = (consommateurs_migres / consommateurs_total) × 100
```

| Métrique | Description | Unité |
|----------|-------------|-------|
| **ADOPTION_RATE_MEAN** | Taux d'adoption moyen sur toutes les transitions | Pourcentage |
| **ADOPTION_RATE_MIN** | Taux d'adoption le plus bas (transition la plus lente) | Pourcentage |
| **ADOPTION_RATE_MAX** | Taux d'adoption le plus haut | Pourcentage |

**Interprétation :**

| Valeur | Interprétation | Action recommandée |
|--------|----------------|-------------------|
| 0 - 25% | Démarrage | Communiquer davantage |
| 26 - 50% | Progression | Maintenir effort |
| 51 - 75% | Bonne adoption | Préparer fin de transition |
| 76 - 95% | Fin de transition | Identifier retardataires |
| 96 - 100% | Complet | Procéder au retirement |

### 3.4 Réactivations

| Métrique | Description | Unité |
|----------|-------------|-------|
| **REACTIVATION_COUNT** | Nombre total de réactivations (DEPRECATED → ACTIVE) | Entier ≥ 0 |
| **REACTIVATION_RATE** | Pourcentage de dépréciations qui aboutissent à une réactivation | Pourcentage |

**Note :** Un taux de réactivation élevé peut indiquer des problèmes de planification ou des successeurs inadaptés.

---

## 4. Métriques d'alerte

Les métriques d'alerte mesurent les **conditions anormales** qui nécessitent une attention.

### 4.1 Transitions bloquées

Une transition est **bloquée** lorsqu'elle dépasse la période prévue sans progresser.

| Métrique | Description | Seuil par défaut |
|----------|-------------|------------------|
| **BLOCKED_TRANSITIONS** | Nombre de transitions bloquées | ≥ 1 déclenche alerte |
| **BLOCKED_DURATION_MAX** | Durée de la transition la plus bloquée | Variable selon catégorie |

**Critères de blocage par catégorie :**

| Catégorie | Période max avant blocage |
|-----------|---------------------------|
| FONDATION | 6 cycles de release |
| OPERATIONAL | 4 cycles de release |
| TECHNICAL | 3 cycles de release |
| INTERNAL | 2 cycles de release |

### 4.2 Consommateurs non migrés

| Métrique | Description | Seuil d'alerte |
|----------|-------------|----------------|
| **CONSUMERS_NOT_MIGRATED** | Nombre de consommateurs non migrés | ≥ 1 à l'approche du retirement |
| **CONSUMERS_AT_RISK** | Consommateurs non migrés avec retirement imminent | ≥ 1 déclenche alerte critique |

**Définition :** Un consommateur est "at risk" si le retirement est prévu dans ≤ 1 cycle de release et qu'il n'a pas encore migré.

### 4.3 Dette structurelle excessive

| Métrique | Description | Seuil d'alerte |
|----------|-------------|----------------|
| **DEBT_EXCESSIVE** | Indicateur binaire de dette excessive | DEBT_RATIO > 0.50 |
| **DEBT_CRITICAL** | Indicateur binaire de dette critique | DEBT_RATIO > 1.00 |

### 4.4 Violations de règles d'évolution

| Métrique | Description | Seuil d'alerte |
|----------|-------------|----------------|
| **RULE_VIOLATIONS** | Nombre de violations des règles d'évolution | ≥ 1 déclenche alerte |
| **INVARIANT_BREACHES** | Nombre de tentatives de violation d'invariants | ≥ 1 déclenche alerte critique |

**Types de violations détectées :**
- Tentative de transition invalide (cf. [Transition Rules Contract](../lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md))
- Dépréciation sans successeur identifié
- Période de dépréciation inférieure au minimum
- Transition sans documentation
- Breaking change non déclaré

---

## 5. Système d'alerte

### 5.1 Flux d'alerte

Le flux d'alerte suit une séquence en 5 étapes, conforme à la Section 8 de la Documentation Fondatrice :

```
┌─────────────────────────────────────────────────────────────────┐
│                    FLUX D'ALERTE EVER BUDDY                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   1. DÉTECTION                                                   │
│   └─► Ever Buddy détecte une condition anormale                  │
│       (dette excessive, transition bloquée, incompatibilité)     │
│                           │                                      │
│                           ▼                                      │
│   2. ÉVALUATION                                                  │
│   └─► Ever Buddy évalue la gravité et l'urgence                  │
│       (niveau INFO, WARNING, ERROR, CRITICAL)                    │
│                           │                                      │
│                           ▼                                      │
│   3. ALERTE                                                      │
│   └─► Ever Buddy émet une alerte vers les consommateurs          │
│       concernés                                                  │
│                           │                                      │
│                           ▼                                      │
│   4. RECOMMANDATION                                              │
│   └─► Ever Buddy fournit des recommandations pour résoudre       │
│       la situation                                               │
│                           │                                      │
│                           ▼                                      │
│   5. SUIVI                                                       │
│   └─► Ever Buddy suit la résolution et clôture l'alerte          │
│       une fois résolue                                           │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 Niveaux de gravité

| Niveau | Code | Description | Exemple |
|--------|------|-------------|---------|
| **INFO** | I | Information sans action requise | Transition démarrée |
| **WARNING** | W | Condition à surveiller | Debt ratio > 0.25 |
| **ERROR** | E | Anomalie nécessitant action | Transition bloquée |
| **CRITICAL** | C | Situation critique urgente | Invariant menacé |

### 5.3 Structure d'une alerte

Chaque alerte émise par Ever Buddy contient :

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `alert_id` | Identifiant unique de l'alerte | ✅ |
| `timestamp` | Moment de l'alerte (cycle de release, pas temps absolu) | ✅ |
| `level` | Niveau de gravité (INFO, WARNING, ERROR, CRITICAL) | ✅ |
| `type` | Type d'alerte (voir section 5.4) | ✅ |
| `element_id` | Identifiant de l'élément concerné | ❌ (si global) |
| `description` | Description lisible de la situation | ✅ |
| `context` | Contexte additionnel (métriques, historique) | ❌ |
| `recommendations` | Actions recommandées | ✅ |
| `consumers_affected` | Liste des consommateurs concernés | ❌ |
| `status` | Statut de l'alerte (OPEN, ACKNOWLEDGED, RESOLVED) | ✅ |

### 5.4 Types d'alertes

| Type | Description | Niveau par défaut |
|------|-------------|-------------------|
| `DEBT_THRESHOLD` | Seuil de dette dépassé | WARNING / ERROR |
| `TRANSITION_BLOCKED` | Transition bloquée | ERROR |
| `CONSUMER_AT_RISK` | Consommateur non migré avant retirement | WARNING / ERROR |
| `RULE_VIOLATION` | Violation de règle d'évolution | ERROR |
| `INVARIANT_BREACH` | Tentative de violation d'invariant | CRITICAL |
| `ADOPTION_STALLED` | Adoption du successeur stagnante | WARNING |
| `REACTIVATION_FREQUENT` | Taux de réactivation anormal | WARNING |
| `INCOMPATIBILITY_DETECTED` | Incompatibilité entre versions | ERROR |
| `MISSING_SUCCESSOR` | Élément déprécié sans successeur | WARNING |
| `DOCUMENTATION_MISSING` | Transition sans documentation | WARNING |

### 5.5 Destinataires des alertes

Les alertes sont transmises selon leur portée :

| Portée | Destinataires |
|--------|---------------|
| **Global** | Caring Nanny, tous les cores |
| **Catégorie** | Cores concernés par la catégorie |
| **Élément** | Consommateurs directs de l'élément |
| **Transition** | Consommateurs de l'élément en transition |

**Règle importante :** Les produits ne reçoivent **jamais** les alertes directement. Elles sont transmises via **BondingBrother** qui filtre et adapte les informations.

---

## 6. Seuils et configuration

### 6.1 Seuils par défaut

| Seuil | Valeur par défaut | Modifiable |
|-------|-------------------|------------|
| DEBT_WARNING_THRESHOLD | 0.25 | ✅ Par catégorie |
| DEBT_ERROR_THRESHOLD | 0.50 | ✅ Par catégorie |
| DEBT_CRITICAL_THRESHOLD | 1.00 | ❌ Fixe |
| BLOCKED_CYCLES_FONDATION | 6 | ✅ |
| BLOCKED_CYCLES_OPERATIONAL | 4 | ✅ |
| BLOCKED_CYCLES_TECHNICAL | 3 | ✅ |
| BLOCKED_CYCLES_INTERNAL | 2 | ✅ |
| ADOPTION_STALLED_THRESHOLD | 3 cycles sans progression | ✅ |
| REACTIVATION_WARNING_RATE | 15% | ✅ |

### 6.2 Règles de modification

- Les seuils peuvent être ajustés par catégorie, pas par élément individuel (INV-EB-8 : indépendance des décisions)
- Les modifications de seuils sont elles-mêmes tracées et documentées
- Les seuils critiques (DEBT_CRITICAL_THRESHOLD, violations d'invariants) ne sont **pas modifiables**

---

## 7. Tableau de bord conceptuel

Ever Buddy expose un tableau de bord conceptuel avec les métriques clés :

```
┌─────────────────────────────────────────────────────────────────┐
│                 EVER BUDDY - TABLEAU DE BORD                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  SANTÉ GLOBALE           ┌─────────────────────────────────────┐ │
│  ─────────────           │  DISTRIBUTION DES ÉTATS            │ │
│  Debt Ratio: 0.18        │  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ ACTIVE (156) │ │
│  Status: ✅ SAIN          │  ▓▓▓ DEPRECATED (28)               │ │
│                          │  ▓ RETIRED (5)                     │ │
│  ALERTES ACTIVES         │  ▓▓ DRAFT (12)                     │ │
│  ─────────────           │  ░ ARCHIVED (8)                    │ │
│  ⚠️ WARNING: 2            └─────────────────────────────────────┘ │
│  ❌ ERROR: 0                                                     │
│  🔴 CRITICAL: 0           TRANSITIONS EN COURS: 7                │
│                          ADOPTION MOYENNE: 67%                  │
│                                                                  │
│  DERNIÈRES ALERTES                                               │
│  ─────────────────                                               │
│  [W] DEBT_THRESHOLD - Catégorie TECHNICAL: ratio 0.31           │
│  [W] ADOPTION_STALLED - Element XYZ: pas de progression         │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 8. Invariants de l'observabilité

### OBS-EB-1 : Métriques sans effet de bord

La collecte et le calcul des métriques ne modifient **jamais** l'état du système. Observer ne change pas ce qui est observé.

### OBS-EB-2 : Alertes traçables

Chaque alerte émise est enregistrée dans l'historique immuable, conformément à **INV-EB-2**.

### OBS-EB-3 : Recommandations non contraignantes

Les recommandations fournies par Ever Buddy sont indicatives. Ever Buddy **ne force jamais** une action, conformément à **INV-EB-8**.

### OBS-EB-4 : Absence de temps absolu

Toutes les métriques temporelles utilisent des **cycles de release** comme unité, jamais des timestamps absolus, conformément à **LOI-4**.

### OBS-EB-5 : Filtrage par BondingBrother

Les alertes destinées aux produits passent **obligatoirement** par BondingBrother, qui filtre et adapte les informations.

---

## 9. Relation avec les autres cores

### 9.1 Caring Nanny

Ever Buddy **alimente** Caring Nanny avec les indicateurs d'évolution :
- Debt ratio actuel
- Nombre de transitions en cours
- Alertes actives

Caring Nanny utilise ces informations pour évaluer la santé globale du système.

### 9.2 StrongFather

Ever Buddy peut **recommander** à StrongFather des interventions humaines via TAMR lorsque des situations critiques sont détectées.

### 9.3 BondingBrother

BondingBrother **relaie** les alertes vers les produits concernés, après filtrage et adaptation.

### 9.4 WorrySentinel

WorrySentinel peut **consulter** Ever Buddy pour évaluer si des transitions ou des incompatibilités affectent les niveaux de sécurité.

---

## 10. Références croisées

### Documents fondateurs
- [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) — Section 8 : Métriques surveillées

### Contrats associés
- [Ever Buddy - Lifecycle States Contract](../lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md) — Définition des états
- [Ever Buddy - Transition Rules Contract](../lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md) — Règles de transition
- [Ever Buddy - Debt Tracking Contract](./Ever%20Buddy%20-%20Debt%20Tracking%20Contract.md) — Surveillance de la dette

### Glossaire
- [Glossaire Miyukini](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) :
  - **Debt ratio** — Rapport entre éléments DEPRECATED/RETIRED et ACTIVE
  - **Adoption rate** — Pourcentage de consommateurs migrés
  - **Caring Nanny** — Core d'observation d'état
  - **DEPRECATED** — État de vie : élément fonctionnel mais usage découragé
  - **Ever Buddy** — Core de cycle de vie et d'évolution

### Lois d'autonomie
- [Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) :
  - **LOI-4** — Pas de temps global requis (métriques en cycles, pas en temps)
  - **LOI-5** — Coût proportionnel au hardware (métriques légères)

---

## 11. Conformité aux invariants

Ce contrat respecte les invariants d'Ever Buddy :

| Invariant | Conformité | Mécanisme |
|-----------|------------|-----------|
| **INV-EB-1** | ✅ Conforme | Métriques en lecture seule, pas d'exécution |
| **INV-EB-2** | ✅ Conforme | Alertes enregistrées dans l'historique immuable |
| **INV-EB-7** | ✅ Conforme | Alertes documentées avec contexte |
| **INV-EB-8** | ✅ Conforme | Seuils universels par catégorie, pas par élément |
| **INV-EB-9** | ✅ Conforme | Règles d'alerte publiques et stables |

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contrat normatif — OBSERVABILITY CONTRACT  
**Référence :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md), Section 8
