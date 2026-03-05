# Ever Buddy - Transition Rules Contract

## Contexte

Ce document dÃ©finit les **rÃ¨gles contractuelles de transition** entre Ã©tats de cycle de vie dans l'Ã©cosystÃ¨me Miyukini. Il spÃ©cifie la matrice des transitions valides, les pÃ©riodes minimales obligatoires, les conditions de validation, et les rÃ¨gles de documentation associÃ©es.

Ce contrat opÃ©rationnalise les principes dÃ©finis dans la [Documentation Fondatrice d'Ever Buddy](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md), Section 4 (Concepts fondamentaux).

**RÃ©fÃ©rence canonique :** [Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) â€” dÃ©finitions des Ã©tats de vie (DRAFT, ACTIVE, DEPRECATED, RETIRED, ARCHIVED).

---

## PortÃ©e / Scope

- **Applicable Ã  :** Toutes les transitions d'Ã©tat de cycle de vie dans l'Ã©cosystÃ¨me Miyukini
- **Audience :** Architectes, dÃ©veloppeurs, implÃ©menteurs de cores, opÃ©rateurs
- **Statut :** Contrat opÃ©rationnel normatif â€” NON NÃ‰GOCIABLE

---

## 1. Matrice des Transitions Valides

### 1.1 DÃ©finition

Une **transition** est le passage d'un Ã©tat de cycle de vie Ã  un autre. Les transitions sont :
- **Atomiques** : Un Ã©lÃ©ment passe de l'Ã©tat A Ã  l'Ã©tat B sans Ã©tat transitoire (INV-EB-3)
- **DocumentÃ©es** : Chaque transition requiert une documentation obligatoire (INV-EB-7)
- **ValidÃ©es** : Ever Buddy vÃ©rifie que la transition respecte les rÃ¨gles

### 1.2 Matrice ComplÃ¨te

| Depuis \ Vers | DRAFT | ACTIVE | DEPRECATED | RETIRED | ARCHIVED |
|---------------|-------|--------|------------|---------|----------|
| **DRAFT**     | â€”     | âœ“      | âœ—          | âœ—       | âœ“        |
| **ACTIVE**    | âœ—     | â€”      | âœ“          | âœ—       | âœ—        |
| **DEPRECATED**| âœ—     | âœ“*     | â€”          | âœ“       | âœ—        |
| **RETIRED**   | âœ—     | âœ—      | âœ—          | â€”       | âœ“        |
| **ARCHIVED**  | âœ—     | âœ—      | âœ—          | âœ—       | â€”        |

**LÃ©gende :**
- âœ“ = Transition valide
- âœ— = Transition invalide (structurellement interdite)
- âœ“* = Transition conditionnelle (voir Section 2.3)
- â€” = Non applicable (mÃªme Ã©tat)

### 1.3 Transitions Valides DÃ©taillÃ©es

| Code | Transition | Description | Condition |
|------|------------|-------------|-----------|
| **T-DA** | DRAFT â†’ ACTIVE | Activation | Ã‰lÃ©ment prÃªt pour production |
| **T-DAR** | DRAFT â†’ ARCHIVED | Abandon prÃ©coce | Ã‰lÃ©ment abandonnÃ© avant activation |
| **T-AD** | ACTIVE â†’ DEPRECATED | DÃ©prÃ©ciation | Successeur identifiÃ© ou abandon dÃ©cidÃ© |
| **T-DE** | DEPRECATED â†’ ACTIVE | RÃ©activation* | Successeur annulÃ©, Ã©lÃ©ment fonctionnel |
| **T-DR** | DEPRECATED â†’ RETIRED | Retirement | PÃ©riode de dÃ©prÃ©ciation Ã©coulÃ©e |
| **T-RA** | RETIRED â†’ ARCHIVED | Archivage | PÃ©riode de grÃ¢ce Ã©coulÃ©e |

### 1.4 Transitions Interdites (Exhaustif)

Les transitions suivantes sont **structurellement interdites** :

| Transition | Raison de l'interdiction |
|------------|-------------------------|
| DRAFT â†’ DEPRECATED | Un Ã©lÃ©ment non activÃ© ne peut Ãªtre dÃ©prÃ©ciÃ© |
| DRAFT â†’ RETIRED | Un Ã©lÃ©ment non activÃ© ne peut Ãªtre retirÃ© |
| ACTIVE â†’ DRAFT | RÃ©gression interdite â€” pas de retour en brouillon |
| ACTIVE â†’ RETIRED | **Passage obligatoire par DEPRECATED** (INV-EB-4) |
| ACTIVE â†’ ARCHIVED | SÃ©quence obligatoire : ACTIVE â†’ DEPRECATED â†’ RETIRED â†’ ARCHIVED |
| DEPRECATED â†’ DRAFT | RÃ©gression interdite |
| DEPRECATED â†’ ARCHIVED | Passage obligatoire par RETIRED |
| RETIRED â†’ DRAFT | RÃ©activation interdite aprÃ¨s retirement |
| RETIRED â†’ ACTIVE | RÃ©activation interdite aprÃ¨s retirement |
| RETIRED â†’ DEPRECATED | RÃ©gression interdite |
| ARCHIVED â†’ * | **Aucune sortie possible** â€” Ã©tat terminal absolu |

---

## 2. RÃ¨gles de Transition

### 2.1 RÃ¨gle Fondamentale : Passage Obligatoire par DEPRECATED (INV-EB-4)

> **Aucun Ã©lÃ©ment ACTIVE ne peut passer directement Ã  RETIRED ou ARCHIVED.**

La transition par DEPRECATED est **obligatoire** sans exception. Cette rÃ¨gle protÃ¨ge les consommateurs contre les ruptures brutales.

**SÃ©quence obligatoire pour fin de vie :**

```
ACTIVE â†’ DEPRECATED â†’ RETIRED â†’ ARCHIVED
```

**Violation :** Toute tentative de contournement est rejetÃ©e par Ever Buddy.

### 2.2 RÃ¨gle d'AtomicitÃ© des Transitions (INV-EB-3)

Chaque Ã©lÃ©ment possÃ¨de **exactement un** Ã©tat de cycle de vie Ã  tout moment.

- âŒ Pas d'Ã©tat intermÃ©diaire
- âŒ Pas d'Ã©tat incertain
- âŒ Pas d'Ã©tat non dÃ©fini
- âœ“ Transitions atomiques uniquement

### 2.3 RÃ¨gle de RÃ©activation Conditionnelle (DEPRECATED â†’ ACTIVE)

La transition DEPRECATED â†’ ACTIVE est **conditionnelle**. Elle n'est autorisÃ©e que si :

| Condition | Obligatoire |
|-----------|-------------|
| Le successeur prÃ©vu est annulÃ© | âœ“ |
| L'Ã©lÃ©ment dÃ©prÃ©ciÃ© est encore fonctionnel | âœ“ |
| La dÃ©cision de rÃ©activation est documentÃ©e avec justification | âœ“ |
| L'historique conserve la trace de la dÃ©prÃ©ciation temporaire | âœ“ |

**ScÃ©nario typique :** Le dÃ©veloppement du successeur Ã©choue ou est abandonnÃ©, nÃ©cessitant le maintien de l'Ã©lÃ©ment dÃ©prÃ©ciÃ©.

### 2.4 RÃ¨gle de PrÃ©dictibilitÃ© (INV-EB-9)

Les rÃ¨gles de transition sont **publiques et stables**.

| Garantie | Description |
|----------|-------------|
| Transparence | Tout consommateur peut connaÃ®tre Ã  l'avance les conditions de transition |
| StabilitÃ© | Les rÃ¨gles ne changent pas frÃ©quemment |
| Non-rÃ©troactivitÃ© | Aucune rÃ¨gle ne peut Ãªtre modifiÃ©e rÃ©troactivement (INV-EB-11) |

### 2.5 RÃ¨gle de Non-RÃ©troactivitÃ© (INV-EB-11)

Les rÃ¨gles d'Ã©volution s'appliquent aux transitions **futures** uniquement.

- Un changement de rÃ¨gle ne modifie pas le statut d'Ã©lÃ©ments dÃ©jÃ  en transition
- Les transitions en cours continuent selon les rÃ¨gles initiales
- Cette rÃ¨gle protÃ¨ge les transitions en cours de complÃ©tion

---

## 3. PÃ©riodes Minimales de Transition

### 3.1 DÃ©finition

Chaque type de transition possÃ¨de une **pÃ©riode minimale non nÃ©gociable**. Ces pÃ©riodes sont des **minimums absolus** â€” Ever Buddy peut recommander des pÃ©riodes plus longues selon l'impact et l'adoption.

### 3.2 Tableau des PÃ©riodes Minimales

| Transition | PÃ©riode Minimale | Notes |
|------------|------------------|-------|
| **DRAFT â†’ ACTIVE** | Aucune | Activation immÃ©diate possible |
| **ACTIVE â†’ DEPRECATED** | 1 cycle de release | Communication prÃ©alable obligatoire |
| **DEPRECATED â†’ RETIRED** | DÃ©finie par catÃ©gorie | Voir Section 3.3 |
| **RETIRED â†’ ARCHIVED** | PÃ©riode de grÃ¢ce | Pour consommateurs existants |
| **DRAFT â†’ ARCHIVED** | Aucune | Abandon immÃ©diat possible |
| **DEPRECATED â†’ ACTIVE** | Aucune | RÃ©activation immÃ©diate si conditions remplies |

### 3.3 PÃ©riodes par CatÃ©gorie d'Ã‰lÃ©ment

Les pÃ©riodes de dÃ©prÃ©ciation (DEPRECATED â†’ RETIRED) varient selon la catÃ©gorie :

| CatÃ©gorie | Description | PÃ©riode Minimale | Ruptures |
|-----------|-------------|------------------|----------|
| **Contrats Fondateurs (FONDATION)** | Documents contractuels non nÃ©gociables | TrÃ¨s longue (plusieurs gÃ©nÃ©rations) | Quasiment interdites |
| **Contrats OpÃ©rationnels** | Contrats de fonctionnement standard | Standard (plusieurs cycles) | Possibles avec justification |
| **Interfaces Techniques** | APIs, surfaces d'appel | Courte (quelques cycles) | Possibles avec documentation |
| **Ã‰lÃ©ments Internes** | Composants internes non exposÃ©s | Aucune garantie | Sans prÃ©avis autorisÃ©es |

### 3.4 Facteurs d'Extension

Ever Buddy peut recommander des pÃ©riodes **plus longues** que les minimums selon :

| Facteur | Impact |
|---------|--------|
| Nombre de consommateurs | Plus de consommateurs = pÃ©riode plus longue |
| CriticitÃ© de l'Ã©lÃ©ment | Ã‰lÃ©ment critique = pÃ©riode plus longue |
| ComplexitÃ© de migration | Migration complexe = pÃ©riode plus longue |
| Taux d'adoption du successeur | Adoption lente = pÃ©riode plus longue |

### 3.5 PÃ©riode de GrÃ¢ce

La **pÃ©riode de grÃ¢ce** est le temps supplÃ©mentaire accordÃ© aprÃ¨s la date prÃ©vue de retirement.

| CaractÃ©ristique | Description |
|-----------------|-------------|
| DÃ©clenchement | AprÃ¨s pÃ©riode de retirement standard |
| But | Permettre aux consommateurs retardataires de migrer |
| Attribution | Au cas par cas, sur demande justifiÃ©e |
| DurÃ©e | Variable selon la situation |

---

## 4. Documentation Obligatoire des Transitions (INV-EB-7)

### 4.1 RÃ¨gle Fondamentale

> **Toute transition d'Ã©tat doit Ãªtre documentÃ©e. Une transition sans documentation est invalide.**

### 4.2 Contenu Obligatoire

Chaque transition DOIT inclure :

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `transition_id` | Identifiant unique de la transition | âœ“ |
| `element_id` | Identifiant de l'Ã©lÃ©ment concernÃ© | âœ“ |
| `from_state` | Ã‰tat de dÃ©part | âœ“ |
| `to_state` | Ã‰tat d'arrivÃ©e | âœ“ |
| `reason` | Raison de la transition | âœ“ |
| `impact` | Impact sur les consommateurs | âœ“ |
| `migration_path` | Chemin de migration (si applicable) | Conditionnel |
| `effective_date` | Date effective de la transition | âœ“ |
| `requested_by` | Demandeur de la transition | âœ“ |
| `validated_by` | Validateur (Ever Buddy) | âœ“ |

### 4.3 Documentation Additionnelle par Type de Transition

| Transition | Documentation Additionnelle |
|------------|----------------------------|
| ACTIVE â†’ DEPRECATED | Successeur identifiÃ©, pÃ©riode de dÃ©prÃ©ciation prÃ©vue |
| DEPRECATED â†’ RETIRED | Taux d'adoption du successeur, consommateurs non migrÃ©s |
| DEPRECATED â†’ ACTIVE | Justification de rÃ©activation, statut du successeur annulÃ© |

---

## 5. Validation des Transitions

### 5.1 Processus de Validation

1. **Demande** : Un core ou produit demande une transition d'Ã©tat
2. **VÃ©rification** : Ever Buddy vÃ©rifie que la transition est valide selon ce contrat
3. **Documentation** : Ever Buddy vÃ©rifie que la documentation est complÃ¨te
4. **Enregistrement** : Si valide, la transition est enregistrÃ©e dans l'historique immuable
5. **Communication** : Ever Buddy communique la transition aux consommateurs concernÃ©s

### 5.2 CritÃ¨res de Rejet

Une transition est **rejetÃ©e** si :

| CritÃ¨re | Description |
|---------|-------------|
| Transition invalide | La transition n'est pas dans la matrice des transitions valides |
| Documentation incomplÃ¨te | Un champ obligatoire est manquant |
| PÃ©riode non respectÃ©e | La pÃ©riode minimale n'est pas Ã©coulÃ©e |
| Condition non remplie | Pour les transitions conditionnelles (ex: rÃ©activation) |

### 5.3 Message de Rejet

En cas de rejet, Ever Buddy fournit :

- Le code de la transition tentÃ©e
- La raison du rejet
- Les conditions Ã  remplir pour que la transition soit acceptÃ©e
- La rÃ©fÃ©rence Ã  ce contrat

---

## 6. Invariants Applicables

Ce contrat opÃ©rationnalise les invariants suivants de la Documentation Fondatrice :

| Invariant | Ã‰noncÃ© | Application |
|-----------|--------|-------------|
| **INV-EB-3** | Aucun Ã©tat ambigu | Transitions atomiques, un seul Ã©tat Ã  tout moment |
| **INV-EB-4** | PÃ©riode de dÃ©prÃ©ciation obligatoire | Passage obligatoire par DEPRECATED |
| **INV-EB-7** | Documentation obligatoire | Chaque transition doit Ãªtre documentÃ©e |
| **INV-EB-9** | PrÃ©dictibilitÃ© des transitions | RÃ¨gles publiques et stables |
| **INV-EB-11** | Non-rÃ©troactivitÃ© | RÃ¨gles appliquÃ©es aux transitions futures |

---

## 7. ConformitÃ© aux Lois d'Autonomie

Ce contrat est conforme aux [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md) :

| Loi | ConformitÃ© | MÃ©canisme |
|-----|------------|-----------|
| **LOI-1** | âœ… | Validation locale des transitions, pas de dÃ©pendance externe |
| **LOI-2** | âœ… | Transitions validÃ©es en mode isolÃ© |
| **LOI-3** | âœ… | Historique des transitions souverain localement |
| **LOI-4** | âœ… | PÃ©riodes dÃ©finies en cycles, pas en temps absolu |

---

## 8. Exemples de Transitions

### 8.1 Transition Standard : ACTIVE â†’ DEPRECATED

```yaml
transition_id: "TR-2026-001"
element_id: "API-USER-V2"
from_state: "ACTIVE"
to_state: "DEPRECATED"
reason: "Nouvelle version V3 disponible avec amÃ©liorations de performance"
impact: "Les consommateurs doivent migrer vers API-USER-V3"
migration_path: "Guide de migration disponible dans docs/migration/user-api-v2-to-v3.md"
effective_date: "2026-02-01"
successor_id: "API-USER-V3"
deprecation_period: "3 cycles de release"
requested_by: "Core-Architecture"
validated_by: "Ever Buddy"
```

### 8.2 Transition Conditionnelle : DEPRECATED â†’ ACTIVE (RÃ©activation)

```yaml
transition_id: "TR-2026-002"
element_id: "API-USER-V2"
from_state: "DEPRECATED"
to_state: "ACTIVE"
reason: "DÃ©veloppement du successeur V3 annulÃ© - ressources insuffisantes"
impact: "API V2 redevient la version supportÃ©e"
successor_cancelled: true
successor_id: "API-USER-V3"
successor_cancellation_reason: "Contraintes techniques insurmontables"
element_functional: true
effective_date: "2026-03-15"
requested_by: "Core-Architecture"
validated_by: "Ever Buddy"
```

---

## 9. Mini log de gÃ©nÃ©ration

### DÃ©cision D1 : ExhaustivitÃ© des transitions interdites

**Contexte :** NÃ©cessitÃ© de documenter explicitement toutes les transitions interdites, pas seulement celles mentionnÃ©es dans la Documentation Fondatrice.

**DÃ©cision :** Lister exhaustivement les 11 transitions interdites avec leur raison.

**Justification :** La clartÃ© et la prÃ©dictibilitÃ© (INV-EB-9) exigent que les interdictions soient explicites, pas implicites.

### DÃ©cision D2 : Format de documentation des transitions

**Contexte :** La Documentation Fondatrice mentionne l'obligation de documentation (INV-EB-7) mais ne dÃ©finit pas de format.

**DÃ©cision :** DÃ©finir un format structurÃ© avec champs obligatoires.

**Justification :** Un format normalisÃ© facilite la validation automatique et l'audit.

### VÃ©rification de cohÃ©rence

**VÃ©rifications effectuÃ©es :**
- âœ… CohÃ©rence avec la matrice de la Documentation Fondatrice (Section 4)
- âœ… CohÃ©rence avec les invariants INV-EB-3, INV-EB-4, INV-EB-7, INV-EB-9, INV-EB-11
- âœ… CohÃ©rence avec les catÃ©gories d'Ã©lÃ©ments dÃ©finies dans la Documentation Fondatrice
- âœ… ConformitÃ© aux Lois d'Autonomie SystÃ¨me

**Conclusion :** Aucune contradiction dÃ©tectÃ©e avec la Documentation Fondatrice.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contrat opÃ©rationnel â€” Normatif  
**RÃ©fÃ©rence :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) Section 4  
**Type :** Contrat de cycle de vie

