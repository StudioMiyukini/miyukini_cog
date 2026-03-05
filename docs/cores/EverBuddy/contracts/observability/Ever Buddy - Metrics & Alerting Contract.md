# Ever Buddy - Metrics & Alerting Contract

## Contexte

Ce contrat dÃ©finit les **mÃ©triques de surveillance** et le **systÃ¨me d'alerte** d'Ever Buddy. En tant que core de cycle de vie et d'Ã©volution, Ever Buddy observe en permanence l'Ã©tat des Ã©lÃ©ments du systÃ¨me et alerte les consommateurs lorsque des conditions anormales sont dÃ©tectÃ©es.

Ce document extrait et formalise les mÃ©triques et alertes dÃ©crites dans la **Section 8** de la [Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md).

## PortÃ©e / Scope

- **Applicable Ã  :** Toute implÃ©mentation d'Ever Buddy, tout consommateur d'informations de cycle de vie
- **Audience :** Architectes, dÃ©veloppeurs, opÃ©rateurs systÃ¨me, Caring Nanny
- **Statut :** Contrat normatif â€” OBSERVABILITY CONTRACT
- **RÃ©fÃ©rence fondatrice :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md), Section 8

---

## 1. Principes fondamentaux de l'observabilitÃ©

### 1.1 Observation sans exÃ©cution

Ever Buddy observe et mesure, mais **n'exÃ©cute jamais de correction automatique**. L'invariant **INV-EB-1** (aucune exÃ©cution de migration) s'applique Ã©galement aux mÃ©triques et alertes.

**Ce que Ever Buddy fait :**
- Collecter des mÃ©triques
- Calculer des indicateurs
- Ã‰mettre des alertes
- Fournir des recommandations

**Ce que Ever Buddy ne fait PAS :**
- Corriger automatiquement les anomalies
- Forcer des migrations
- Modifier des Ã©tats sans validation
- Supprimer des Ã©lÃ©ments

### 1.2 MÃ©triques conceptuelles, pas techniques

Les mÃ©triques d'Ever Buddy portent sur les **Ã©tats de cycle de vie et les Ã©volutions**, pas sur les performances techniques. Ever Buddy ne mesure pas :
- La latence des APIs
- L'utilisation mÃ©moire
- Le temps de rÃ©ponse

Ces aspects relÃ¨vent de **Caring Nanny** (observation d'Ã©tat systÃ¨me) et non d'Ever Buddy.

### 1.3 Alimentation de Caring Nanny

Ever Buddy **alimente Caring Nanny** avec les indicateurs d'Ã©volution qui affectent la santÃ© du systÃ¨me. Cette relation est unidirectionnelle : Ever Buddy fournit, Caring Nanny observe.

---

## 2. MÃ©triques d'Ã©tat

Les mÃ©triques d'Ã©tat mesurent la **distribution actuelle** des Ã©lÃ©ments du systÃ¨me selon leur Ã©tat de [cycle de vie](../lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md).

### 2.1 Compteurs par Ã©tat

| MÃ©trique | Description | UnitÃ© |
|----------|-------------|-------|
| **COUNT_DRAFT** | Nombre d'Ã©lÃ©ments en Ã©tat DRAFT | Entier â‰¥ 0 |
| **COUNT_ACTIVE** | Nombre d'Ã©lÃ©ments en Ã©tat ACTIVE | Entier â‰¥ 0 |
| **COUNT_DEPRECATED** | Nombre d'Ã©lÃ©ments en Ã©tat DEPRECATED | Entier â‰¥ 0 |
| **COUNT_RETIRED** | Nombre d'Ã©lÃ©ments en Ã©tat RETIRED | Entier â‰¥ 0 |
| **COUNT_ARCHIVED** | Nombre d'Ã©lÃ©ments en Ã©tat ARCHIVED | Entier â‰¥ 0 |
| **COUNT_TOTAL** | Nombre total d'Ã©lÃ©ments suivis | Entier â‰¥ 0 |

**RÃ¨gle :** COUNT_TOTAL = COUNT_DRAFT + COUNT_ACTIVE + COUNT_DEPRECATED + COUNT_RETIRED + COUNT_ARCHIVED

### 2.2 Debt Ratio

Le **debt ratio** mesure la proportion d'Ã©lÃ©ments en fin de vie par rapport aux Ã©lÃ©ments actifs.

```
DEBT_RATIO = (COUNT_DEPRECATED + COUNT_RETIRED) / COUNT_ACTIVE
```

**InterprÃ©tation :**

| Valeur | InterprÃ©tation | Action recommandÃ©e |
|--------|----------------|-------------------|
| 0.00 - 0.10 | Sain | Aucune |
| 0.11 - 0.25 | Acceptable | Surveillance |
| 0.26 - 0.50 | Attention | Planifier nettoyage |
| 0.51 - 1.00 | Alerte | Nettoyage prioritaire |
| > 1.00 | Critique | Intervention urgente |

**Cas limite :** Si COUNT_ACTIVE = 0, le DEBT_RATIO est dÃ©fini comme "UNDEFINED" (pas de division par zÃ©ro). Cette situation est elle-mÃªme une anomalie Ã  signaler.

### 2.3 Ã‚ge moyen par Ã©tat

L'**Ã¢ge moyen** mesure depuis combien de temps les Ã©lÃ©ments sont dans leur Ã©tat actuel.

| MÃ©trique | Description | UnitÃ© |
|----------|-------------|-------|
| **AGE_MEAN_DRAFT** | DurÃ©e moyenne en DRAFT | Cycles de release |
| **AGE_MEAN_ACTIVE** | DurÃ©e moyenne en ACTIVE | Cycles de release |
| **AGE_MEAN_DEPRECATED** | DurÃ©e moyenne en DEPRECATED | Cycles de release |
| **AGE_MEAN_RETIRED** | DurÃ©e moyenne en RETIRED | Cycles de release |
| **AGE_MEAN_ARCHIVED** | DurÃ©e moyenne en ARCHIVED | Cycles de release |

**Note importante :** Les Ã¢ges sont mesurÃ©s en **cycles de release**, pas en temps absolu, conformÃ©ment Ã  la **LOI-4** (pas de temps global requis).

### 2.4 RÃ©partition par catÃ©gorie

Ever Buddy distingue les Ã©lÃ©ments par [catÃ©gorie](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) :

| CatÃ©gorie | Description |
|-----------|-------------|
| **FONDATION** | Contrats fondateurs |
| **OPERATIONAL** | Contrats opÃ©rationnels |
| **TECHNICAL** | Interfaces techniques |
| **INTERNAL** | Ã‰lÃ©ments internes |

**MÃ©triques dÃ©rivÃ©es :**
- COUNT_FONDATION_DEPRECATED : Nombre de contrats fondateurs dÃ©prÃ©ciÃ©s
- COUNT_OPERATIONAL_DEPRECATED : Nombre de contrats opÃ©rationnels dÃ©prÃ©ciÃ©s
- etc.

---

## 3. MÃ©triques de transition

Les mÃ©triques de transition mesurent le **mouvement** des Ã©lÃ©ments entre Ã©tats.

### 3.1 Transitions en cours

| MÃ©trique | Description | UnitÃ© |
|----------|-------------|-------|
| **TRANSITIONS_IN_PROGRESS** | Nombre de transitions actives | Entier â‰¥ 0 |
| **TRANSITIONS_DEPRECATED_TO_RETIRED** | Transitions DEPRECATED â†’ RETIRED en cours | Entier â‰¥ 0 |
| **TRANSITIONS_RETIRED_TO_ARCHIVED** | Transitions RETIRED â†’ ARCHIVED en cours | Entier â‰¥ 0 |

### 3.2 DurÃ©e des pÃ©riodes de dÃ©prÃ©ciation

| MÃ©trique | Description | UnitÃ© |
|----------|-------------|-------|
| **DEPRECATION_DURATION_MEAN** | DurÃ©e moyenne des pÃ©riodes de dÃ©prÃ©ciation | Cycles de release |
| **DEPRECATION_DURATION_MIN** | DurÃ©e minimale observÃ©e | Cycles de release |
| **DEPRECATION_DURATION_MAX** | DurÃ©e maximale observÃ©e | Cycles de release |

### 3.3 Taux d'adoption des successeurs

Le **taux d'adoption** (adoption rate) mesure la progression de la migration vers un successeur.

```
ADOPTION_RATE = (consommateurs_migres / consommateurs_total) Ã— 100
```

| MÃ©trique | Description | UnitÃ© |
|----------|-------------|-------|
| **ADOPTION_RATE_MEAN** | Taux d'adoption moyen sur toutes les transitions | Pourcentage |
| **ADOPTION_RATE_MIN** | Taux d'adoption le plus bas (transition la plus lente) | Pourcentage |
| **ADOPTION_RATE_MAX** | Taux d'adoption le plus haut | Pourcentage |

**InterprÃ©tation :**

| Valeur | InterprÃ©tation | Action recommandÃ©e |
|--------|----------------|-------------------|
| 0 - 25% | DÃ©marrage | Communiquer davantage |
| 26 - 50% | Progression | Maintenir effort |
| 51 - 75% | Bonne adoption | PrÃ©parer fin de transition |
| 76 - 95% | Fin de transition | Identifier retardataires |
| 96 - 100% | Complet | ProcÃ©der au retirement |

### 3.4 RÃ©activations

| MÃ©trique | Description | UnitÃ© |
|----------|-------------|-------|
| **REACTIVATION_COUNT** | Nombre total de rÃ©activations (DEPRECATED â†’ ACTIVE) | Entier â‰¥ 0 |
| **REACTIVATION_RATE** | Pourcentage de dÃ©prÃ©ciations qui aboutissent Ã  une rÃ©activation | Pourcentage |

**Note :** Un taux de rÃ©activation Ã©levÃ© peut indiquer des problÃ¨mes de planification ou des successeurs inadaptÃ©s.

---

## 4. MÃ©triques d'alerte

Les mÃ©triques d'alerte mesurent les **conditions anormales** qui nÃ©cessitent une attention.

### 4.1 Transitions bloquÃ©es

Une transition est **bloquÃ©e** lorsqu'elle dÃ©passe la pÃ©riode prÃ©vue sans progresser.

| MÃ©trique | Description | Seuil par dÃ©faut |
|----------|-------------|------------------|
| **BLOCKED_TRANSITIONS** | Nombre de transitions bloquÃ©es | â‰¥ 1 dÃ©clenche alerte |
| **BLOCKED_DURATION_MAX** | DurÃ©e de la transition la plus bloquÃ©e | Variable selon catÃ©gorie |

**CritÃ¨res de blocage par catÃ©gorie :**

| CatÃ©gorie | PÃ©riode max avant blocage |
|-----------|---------------------------|
| FONDATION | 6 cycles de release |
| OPERATIONAL | 4 cycles de release |
| TECHNICAL | 3 cycles de release |
| INTERNAL | 2 cycles de release |

### 4.2 Consommateurs non migrÃ©s

| MÃ©trique | Description | Seuil d'alerte |
|----------|-------------|----------------|
| **CONSUMERS_NOT_MIGRATED** | Nombre de consommateurs non migrÃ©s | â‰¥ 1 Ã  l'approche du retirement |
| **CONSUMERS_AT_RISK** | Consommateurs non migrÃ©s avec retirement imminent | â‰¥ 1 dÃ©clenche alerte critique |

**DÃ©finition :** Un consommateur est "at risk" si le retirement est prÃ©vu dans â‰¤ 1 cycle de release et qu'il n'a pas encore migrÃ©.

### 4.3 Dette structurelle excessive

| MÃ©trique | Description | Seuil d'alerte |
|----------|-------------|----------------|
| **DEBT_EXCESSIVE** | Indicateur binaire de dette excessive | DEBT_RATIO > 0.50 |
| **DEBT_CRITICAL** | Indicateur binaire de dette critique | DEBT_RATIO > 1.00 |

### 4.4 Violations de rÃ¨gles d'Ã©volution

| MÃ©trique | Description | Seuil d'alerte |
|----------|-------------|----------------|
| **RULE_VIOLATIONS** | Nombre de violations des rÃ¨gles d'Ã©volution | â‰¥ 1 dÃ©clenche alerte |
| **INVARIANT_BREACHES** | Nombre de tentatives de violation d'invariants | â‰¥ 1 dÃ©clenche alerte critique |

**Types de violations dÃ©tectÃ©es :**
- Tentative de transition invalide (cf. [Transition Rules Contract](../lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md))
- DÃ©prÃ©ciation sans successeur identifiÃ©
- PÃ©riode de dÃ©prÃ©ciation infÃ©rieure au minimum
- Transition sans documentation
- Breaking change non dÃ©clarÃ©

---

## 5. SystÃ¨me d'alerte

### 5.1 Flux d'alerte

Le flux d'alerte suit une sÃ©quence en 5 Ã©tapes, conforme Ã  la Section 8 de la Documentation Fondatrice :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    FLUX D'ALERTE EVER BUDDY                      â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                  â”‚
â”‚   1. DÃ‰TECTION                                                   â”‚
â”‚   â””â”€â–º Ever Buddy dÃ©tecte une condition anormale                  â”‚
â”‚       (dette excessive, transition bloquÃ©e, incompatibilitÃ©)     â”‚
â”‚                           â”‚                                      â”‚
â”‚                           â–¼                                      â”‚
â”‚   2. Ã‰VALUATION                                                  â”‚
â”‚   â””â”€â–º Ever Buddy Ã©value la gravitÃ© et l'urgence                  â”‚
â”‚       (niveau INFO, WARNING, ERROR, CRITICAL)                    â”‚
â”‚                           â”‚                                      â”‚
â”‚                           â–¼                                      â”‚
â”‚   3. ALERTE                                                      â”‚
â”‚   â””â”€â–º Ever Buddy Ã©met une alerte vers les consommateurs          â”‚
â”‚       concernÃ©s                                                  â”‚
â”‚                           â”‚                                      â”‚
â”‚                           â–¼                                      â”‚
â”‚   4. RECOMMANDATION                                              â”‚
â”‚   â””â”€â–º Ever Buddy fournit des recommandations pour rÃ©soudre       â”‚
â”‚       la situation                                               â”‚
â”‚                           â”‚                                      â”‚
â”‚                           â–¼                                      â”‚
â”‚   5. SUIVI                                                       â”‚
â”‚   â””â”€â–º Ever Buddy suit la rÃ©solution et clÃ´ture l'alerte          â”‚
â”‚       une fois rÃ©solue                                           â”‚
â”‚                                                                  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 5.2 Niveaux de gravitÃ©

| Niveau | Code | Description | Exemple |
|--------|------|-------------|---------|
| **INFO** | I | Information sans action requise | Transition dÃ©marrÃ©e |
| **WARNING** | W | Condition Ã  surveiller | Debt ratio > 0.25 |
| **ERROR** | E | Anomalie nÃ©cessitant action | Transition bloquÃ©e |
| **CRITICAL** | C | Situation critique urgente | Invariant menacÃ© |

### 5.3 Structure d'une alerte

Chaque alerte Ã©mise par Ever Buddy contient :

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `alert_id` | Identifiant unique de l'alerte | âœ… |
| `timestamp` | Moment de l'alerte (cycle de release, pas temps absolu) | âœ… |
| `level` | Niveau de gravitÃ© (INFO, WARNING, ERROR, CRITICAL) | âœ… |
| `type` | Type d'alerte (voir section 5.4) | âœ… |
| `element_id` | Identifiant de l'Ã©lÃ©ment concernÃ© | âŒ (si global) |
| `description` | Description lisible de la situation | âœ… |
| `context` | Contexte additionnel (mÃ©triques, historique) | âŒ |
| `recommendations` | Actions recommandÃ©es | âœ… |
| `consumers_affected` | Liste des consommateurs concernÃ©s | âŒ |
| `status` | Statut de l'alerte (OPEN, ACKNOWLEDGED, RESOLVED) | âœ… |

### 5.4 Types d'alertes

| Type | Description | Niveau par dÃ©faut |
|------|-------------|-------------------|
| `DEBT_THRESHOLD` | Seuil de dette dÃ©passÃ© | WARNING / ERROR |
| `TRANSITION_BLOCKED` | Transition bloquÃ©e | ERROR |
| `CONSUMER_AT_RISK` | Consommateur non migrÃ© avant retirement | WARNING / ERROR |
| `RULE_VIOLATION` | Violation de rÃ¨gle d'Ã©volution | ERROR |
| `INVARIANT_BREACH` | Tentative de violation d'invariant | CRITICAL |
| `ADOPTION_STALLED` | Adoption du successeur stagnante | WARNING |
| `REACTIVATION_FREQUENT` | Taux de rÃ©activation anormal | WARNING |
| `INCOMPATIBILITY_DETECTED` | IncompatibilitÃ© entre versions | ERROR |
| `MISSING_SUCCESSOR` | Ã‰lÃ©ment dÃ©prÃ©ciÃ© sans successeur | WARNING |
| `DOCUMENTATION_MISSING` | Transition sans documentation | WARNING |

### 5.5 Destinataires des alertes

Les alertes sont transmises selon leur portÃ©e :

| PortÃ©e | Destinataires |
|--------|---------------|
| **Global** | Caring Nanny, tous les cores |
| **CatÃ©gorie** | Cores concernÃ©s par la catÃ©gorie |
| **Ã‰lÃ©ment** | Consommateurs directs de l'Ã©lÃ©ment |
| **Transition** | Consommateurs de l'Ã©lÃ©ment en transition |

**RÃ¨gle importante :** Les produits ne reÃ§oivent **jamais** les alertes directement. Elles sont transmises via **BondingBrother** qui filtre et adapte les informations.

---

## 6. Seuils et configuration

### 6.1 Seuils par dÃ©faut

| Seuil | Valeur par dÃ©faut | Modifiable |
|-------|-------------------|------------|
| DEBT_WARNING_THRESHOLD | 0.25 | âœ… Par catÃ©gorie |
| DEBT_ERROR_THRESHOLD | 0.50 | âœ… Par catÃ©gorie |
| DEBT_CRITICAL_THRESHOLD | 1.00 | âŒ Fixe |
| BLOCKED_CYCLES_FONDATION | 6 | âœ… |
| BLOCKED_CYCLES_OPERATIONAL | 4 | âœ… |
| BLOCKED_CYCLES_TECHNICAL | 3 | âœ… |
| BLOCKED_CYCLES_INTERNAL | 2 | âœ… |
| ADOPTION_STALLED_THRESHOLD | 3 cycles sans progression | âœ… |
| REACTIVATION_WARNING_RATE | 15% | âœ… |

### 6.2 RÃ¨gles de modification

- Les seuils peuvent Ãªtre ajustÃ©s par catÃ©gorie, pas par Ã©lÃ©ment individuel (INV-EB-8 : indÃ©pendance des dÃ©cisions)
- Les modifications de seuils sont elles-mÃªmes tracÃ©es et documentÃ©es
- Les seuils critiques (DEBT_CRITICAL_THRESHOLD, violations d'invariants) ne sont **pas modifiables**

---

## 7. Tableau de bord conceptuel

Ever Buddy expose un tableau de bord conceptuel avec les mÃ©triques clÃ©s :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                 EVER BUDDY - TABLEAU DE BORD                     â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                  â”‚
â”‚  SANTÃ‰ GLOBALE           â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€           â”‚  DISTRIBUTION DES Ã‰TATS            â”‚ â”‚
â”‚  Debt Ratio: 0.18        â”‚  â–“â–“â–“â–“â–“â–“â–“â–“â–“â–“â–“â–“â–“â–“â–“â–“â–“â–“â–“â–“ ACTIVE (156) â”‚ â”‚
â”‚  Status: âœ… SAIN          â”‚  â–“â–“â–“ DEPRECATED (28)               â”‚ â”‚
â”‚                          â”‚  â–“ RETIRED (5)                     â”‚ â”‚
â”‚  ALERTES ACTIVES         â”‚  â–“â–“ DRAFT (12)                     â”‚ â”‚
â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€           â”‚  â–‘ ARCHIVED (8)                    â”‚ â”‚
â”‚  âš ï¸ WARNING: 2            â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚  âŒ ERROR: 0                                                     â”‚
â”‚  ðŸ”´ CRITICAL: 0           TRANSITIONS EN COURS: 7                â”‚
â”‚                          ADOPTION MOYENNE: 67%                  â”‚
â”‚                                                                  â”‚
â”‚  DERNIÃˆRES ALERTES                                               â”‚
â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€                                               â”‚
â”‚  [W] DEBT_THRESHOLD - CatÃ©gorie TECHNICAL: ratio 0.31           â”‚
â”‚  [W] ADOPTION_STALLED - Element XYZ: pas de progression         â”‚
â”‚                                                                  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 8. Invariants de l'observabilitÃ©

### OBS-EB-1 : MÃ©triques sans effet de bord

La collecte et le calcul des mÃ©triques ne modifient **jamais** l'Ã©tat du systÃ¨me. Observer ne change pas ce qui est observÃ©.

### OBS-EB-2 : Alertes traÃ§ables

Chaque alerte Ã©mise est enregistrÃ©e dans l'historique immuable, conformÃ©ment Ã  **INV-EB-2**.

### OBS-EB-3 : Recommandations non contraignantes

Les recommandations fournies par Ever Buddy sont indicatives. Ever Buddy **ne force jamais** une action, conformÃ©ment Ã  **INV-EB-8**.

### OBS-EB-4 : Absence de temps absolu

Toutes les mÃ©triques temporelles utilisent des **cycles de release** comme unitÃ©, jamais des timestamps absolus, conformÃ©ment Ã  **LOI-4**.

### OBS-EB-5 : Filtrage par BondingBrother

Les alertes destinÃ©es aux produits passent **obligatoirement** par BondingBrother, qui filtre et adapte les informations.

---

## 9. Relation avec les autres cores

### 9.1 Caring Nanny

Ever Buddy **alimente** Caring Nanny avec les indicateurs d'Ã©volution :
- Debt ratio actuel
- Nombre de transitions en cours
- Alertes actives

Caring Nanny utilise ces informations pour Ã©valuer la santÃ© globale du systÃ¨me.

### 9.2 StrongFather

Ever Buddy peut **recommander** Ã  StrongFather des interventions humaines via TAMR lorsque des situations critiques sont dÃ©tectÃ©es.

### 9.3 BondingBrother

BondingBrother **relaie** les alertes vers les produits concernÃ©s, aprÃ¨s filtrage et adaptation.

### 9.4 WorrySentinel

WorrySentinel peut **consulter** Ever Buddy pour Ã©valuer si des transitions ou des incompatibilitÃ©s affectent les niveaux de sÃ©curitÃ©.

---

## 10. RÃ©fÃ©rences croisÃ©es

### Documents fondateurs
- [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) â€” Section 8 : MÃ©triques surveillÃ©es

### Contrats associÃ©s
- [Ever Buddy - Lifecycle States Contract](../lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md) â€” DÃ©finition des Ã©tats
- [Ever Buddy - Transition Rules Contract](../lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md) â€” RÃ¨gles de transition
- [Ever Buddy - Debt Tracking Contract](./Ever%20Buddy%20-%20Debt%20Tracking%20Contract.md) â€” Surveillance de la dette

### Glossaire
- [Glossaire Miyukini](..//..//..//..//miyukini-webway-system//reference//_index.md) :
  - **Debt ratio** â€” Rapport entre Ã©lÃ©ments DEPRECATED/RETIRED et ACTIVE
  - **Adoption rate** â€” Pourcentage de consommateurs migrÃ©s
  - **Caring Nanny** â€” Core d'observation d'Ã©tat
  - **DEPRECATED** â€” Ã‰tat de vie : Ã©lÃ©ment fonctionnel mais usage dÃ©couragÃ©
  - **Ever Buddy** â€” Core de cycle de vie et d'Ã©volution

### Lois d'autonomie
- [Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) :
  - **LOI-4** â€” Pas de temps global requis (mÃ©triques en cycles, pas en temps)
  - **LOI-5** â€” CoÃ»t proportionnel au hardware (mÃ©triques lÃ©gÃ¨res)

---

## 11. ConformitÃ© aux invariants

Ce contrat respecte les invariants d'Ever Buddy :

| Invariant | ConformitÃ© | MÃ©canisme |
|-----------|------------|-----------|
| **INV-EB-1** | âœ… Conforme | MÃ©triques en lecture seule, pas d'exÃ©cution |
| **INV-EB-2** | âœ… Conforme | Alertes enregistrÃ©es dans l'historique immuable |
| **INV-EB-7** | âœ… Conforme | Alertes documentÃ©es avec contexte |
| **INV-EB-8** | âœ… Conforme | Seuils universels par catÃ©gorie, pas par Ã©lÃ©ment |
| **INV-EB-9** | âœ… Conforme | RÃ¨gles d'alerte publiques et stables |

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contrat normatif â€” OBSERVABILITY CONTRACT  
**RÃ©fÃ©rence :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md), Section 8

