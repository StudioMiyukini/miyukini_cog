# StrongFather â€” Decision Graph Specification

## 1. Introduction

### Objet du contrat

Ce document dÃ©finit le **StrongFather â€” Decision Graph Specification** : un contrat normatif, non nÃ©gociable, et de statut FONDATION qui Ã©tablit la spÃ©cification conceptuelle du graphe de dÃ©cision de StrongFather, dÃ©finissant comment les Ã©valuations sont structurÃ©es, comment les politiques sont composÃ©es, et comment les dÃ©cisions sont dÃ©rivÃ©es dans le systÃ¨me Miyukini Core System v2.4.

Ce contrat prÃ©cise la nature conceptuelle du graphe de dÃ©cision, ses nÅ“uds, ses arÃªtes, ses propriÃ©tÃ©s, et les rÃ¨gles de parcours.

### PortÃ©e

Ce contrat s'applique Ã  **toutes les Ã©valuations de StrongFather** et dÃ©finit de maniÃ¨re absolue :
- la dÃ©finition formelle du graphe de dÃ©cision,
- les types de nÅ“uds du graphe,
- les types d'arÃªtes du graphe,
- les rÃ¨gles de composition,
- les propriÃ©tÃ©s du graphe,
- les invariants du graphe.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues qui ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

### Relation avec les autres contrats

Ce contrat complÃ¨te et respecte les documents contractuels existants :
- **StrongFather â€” Policy Engine Contract** : DÃ©finit comment les politiques sont appliquÃ©es
- **StrongFather â€” Core Decision Contract** : DÃ©finit les types de dÃ©cisions produites
- **StrongFather â€” Intent Model Contract** : DÃ©finit les intentions Ã©valuÃ©es
- **[Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)** : ConformitÃ© aux lois d'autonomie, notamment **LOI-1** (aucune dÃ©pendance externe critique) : le graphe de dÃ©cision fonctionne entiÃ¨rement localement

Il n'introduit aucune contradiction, et constitue la spÃ©cification formelle du graphe de dÃ©cision.

---

## 2. DÃ©finition du graphe de dÃ©cision

### 2.1. Nature du graphe

Le **graphe de dÃ©cision** est une structure conceptuelle qui reprÃ©sente le processus d'Ã©valuation d'une intention dans StrongFather. Il modÃ©lise le chemin de l'intention vers la dÃ©cision Ã  travers l'application des politiques.

**CaractÃ©ristiques du graphe :**

- **DirigÃ©** : Le graphe a une direction (de l'intention vers la dÃ©cision)
- **Acyclique** : Le graphe ne contient pas de cycles
- **Fini** : Le graphe a un nombre fini de nÅ“uds et d'arÃªtes
- **Terminant** : Tout parcours du graphe termine par une dÃ©cision

### 2.2. Objectif du graphe

Le graphe de dÃ©cision permet :

1. **Visualisation** : Comprendre le processus d'Ã©valuation
2. **Validation** : VÃ©rifier la cohÃ©rence des politiques
3. **TraÃ§abilitÃ©** : Suivre le chemin d'une Ã©valuation
4. **Optimisation** : Identifier les chemins critiques
5. **Audit** : Reconstruire le raisonnement

### 2.3. Abstraction conceptuelle

Le graphe de dÃ©cision est une **abstraction conceptuelle**. Il ne prÃ©suppose aucune implÃ©mentation technique particuliÃ¨re. Il peut Ãªtre implÃ©mentÃ© de diffÃ©rentes maniÃ¨res tout en respectant cette spÃ©cification.

---

## 3. Types de nÅ“uds

### 3.1. NÅ“ud d'entrÃ©e (ENTRY)

**DÃ©finition :**

Le **nÅ“ud d'entrÃ©e** est le point d'entrÃ©e unique du graphe. Il reprÃ©sente la rÃ©ception d'une intention pour Ã©valuation.

**CaractÃ©ristiques :**

- Un seul nÅ“ud d'entrÃ©e par graphe
- Contient l'intention complÃ¨te
- Contient le contexte d'Ã©valuation
- Pas d'arÃªte entrante

**Contenu :**

- Identifiant de l'intention
- Type d'action
- Sujet
- Contexte d'appel
- DonnÃ©es de l'intention

### 3.2. NÅ“ud de validation structurelle (VALIDATION)

**DÃ©finition :**

Le **nÅ“ud de validation** vÃ©rifie la validitÃ© structurelle de l'intention avant l'Ã©valuation des politiques.

**CaractÃ©ristiques :**

- Suit immÃ©diatement le nÅ“ud d'entrÃ©e
- VÃ©rifie les composants obligatoires
- VÃ©rifie la cohÃ©rence structurelle
- Peut conduire Ã  un rejet structurel

**Sorties possibles :**

- VALIDE : L'intention est structurellement valide
- INVALIDE : L'intention est structurellement invalide (â†’ rejet structurel)

### 3.3. NÅ“ud de politique (POLICY)

**DÃ©finition :**

Un **nÅ“ud de politique** reprÃ©sente l'Ã©valuation d'une politique spÃ©cifique.

**CaractÃ©ristiques :**

- Un nÅ“ud par politique Ã©valuÃ©e
- Ã‰value la politique selon le contexte
- Produit un rÃ©sultat d'Ã©valuation

**Contenu :**

- Identifiant de la politique
- Type de politique
- Condition de la politique
- Contexte d'Ã©valuation

**Sorties possibles :**

- SATISFAITE : La politique est satisfaite
- NON_SATISFAITE : La politique n'est pas satisfaite
- INDÃ‰TERMINÃ‰E : La politique ne peut pas Ãªtre Ã©valuÃ©e

### 3.4. NÅ“ud de composition (COMPOSITION)

**DÃ©finition :**

Un **nÅ“ud de composition** agrÃ¨ge les rÃ©sultats de plusieurs Ã©valuations de politiques selon les rÃ¨gles de composition.

**CaractÃ©ristiques :**

- ReÃ§oit les rÃ©sultats de plusieurs nÅ“uds de politique
- Applique les rÃ¨gles de composition (Policy Engine Contract)
- Produit un rÃ©sultat agrÃ©gÃ©

**RÃ¨gles de composition appliquÃ©es :**

- UnanimitÃ© pour l'acceptation
- Refus prioritaire
- AmbiguÃ¯tÃ© si indÃ©termination

**Sorties possibles :**

- TOUTES_SATISFAITES : Toutes les politiques sont satisfaites
- AU_MOINS_UNE_NON_SATISFAITE : Au moins une politique n'est pas satisfaite
- AU_MOINS_UNE_INDÃ‰TERMINÃ‰E : Au moins une politique est indÃ©terminÃ©e

### 3.5. NÅ“ud de prioritÃ© (PRIORITY)

**DÃ©finition :**

Un **nÅ“ud de prioritÃ©** calcule la prioritÃ© relative de l'intention si les politiques sont satisfaites.

**CaractÃ©ristiques :**

- ActivÃ© uniquement si la composition est TOUTES_SATISFAITES
- Applique les politiques de prioritÃ©
- Produit une valeur de prioritÃ©

**Contenu :**

- Politiques de prioritÃ© appliquÃ©es
- CritÃ¨res de prioritÃ©
- PrioritÃ© calculÃ©e

### 3.6. NÅ“ud de dÃ©cision (DECISION)

**DÃ©finition :**

Un **nÅ“ud de dÃ©cision** reprÃ©sente la production d'une dÃ©cision finale.

**CaractÃ©ristiques :**

- Point de sortie du graphe
- Produit une dÃ©cision complÃ¨te
- Inclut la justification

**Types de nÅ“uds de dÃ©cision :**

- DECISION_ACCEPTÃ‰E : L'intention est acceptÃ©e
- DECISION_REFUSÃ‰E : L'intention est refusÃ©e
- DECISION_AMBIGUÃ‹ : L'intention nÃ©cessite des clarifications
- DECISION_DIFFÃ‰RÃ‰E : L'intention dÃ©pend d'un contexte futur

---

## 4. Types d'arÃªtes

### 4.1. ArÃªte de sÃ©quence (SEQUENCE)

**DÃ©finition :**

Une **arÃªte de sÃ©quence** reprÃ©sente une succession obligatoire entre deux nÅ“uds.

**CaractÃ©ristiques :**

- Le nÅ“ud cible est toujours atteint aprÃ¨s le nÅ“ud source
- Pas de condition
- ReprÃ©sente un flux obligatoire

**Notation :** â†’

### 4.2. ArÃªte conditionnelle (CONDITIONAL)

**DÃ©finition :**

Une **arÃªte conditionnelle** reprÃ©sente une succession conditionnelle basÃ©e sur un rÃ©sultat.

**CaractÃ©ristiques :**

- Le nÅ“ud cible est atteint uniquement si la condition est vraie
- La condition est basÃ©e sur le rÃ©sultat du nÅ“ud source
- ReprÃ©sente un branchement

**Notation :** â†’[condition]

### 4.3. ArÃªte de composition (AGGREGATION)

**DÃ©finition :**

Une **arÃªte de composition** relie plusieurs nÅ“uds sources Ã  un nÅ“ud de composition.

**CaractÃ©ristiques :**

- Plusieurs sources vers une cible
- ReprÃ©sente l'agrÃ©gation de rÃ©sultats
- Le nÅ“ud de composition attend tous les rÃ©sultats

**Notation :** â‡’

---

## 5. Structure du graphe

### 5.1. Structure standard

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                                                             â”‚
â”‚   [ENTRY] â”€â”€â†’ [VALIDATION]                                 â”‚
â”‚                    â”‚                                        â”‚
â”‚                    â”œâ”€â”€â†’[INVALIDE]â”€â”€â†’ [DECISION_REFUSÃ‰E]    â”‚
â”‚                    â”‚                                        â”‚
â”‚                    â””â”€â”€â†’[VALIDE]                             â”‚
â”‚                           â”‚                                 â”‚
â”‚                           â–¼                                 â”‚
â”‚              â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                    â”‚
â”‚              â”‚   Ã‰valuation des       â”‚                    â”‚
â”‚              â”‚   politiques           â”‚                    â”‚
â”‚              â”‚                        â”‚                    â”‚
â”‚              â”‚  [POLICY_1]            â”‚                    â”‚
â”‚              â”‚  [POLICY_2]            â”‚                    â”‚
â”‚              â”‚  [POLICY_N]            â”‚                    â”‚
â”‚              â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                    â”‚
â”‚                       â‡“                                     â”‚
â”‚                 [COMPOSITION]                               â”‚
â”‚                       â”‚                                     â”‚
â”‚    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                 â”‚
â”‚    â”‚                  â”‚                  â”‚                 â”‚
â”‚    â–¼                  â–¼                  â–¼                 â”‚
â”‚ [TOUTES_SAT]    [NON_SAT]          [INDÃ‰T]                â”‚
â”‚    â”‚                  â”‚                  â”‚                 â”‚
â”‚    â–¼                  â–¼                  â–¼                 â”‚
â”‚ [PRIORITY]    [DECISION_REF]    [DECISION_AMB/DIFF]       â”‚
â”‚    â”‚                                                       â”‚
â”‚    â–¼                                                       â”‚
â”‚ [DECISION_ACC]                                             â”‚
â”‚                                                             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 5.2. Phases du graphe

**Phase 1 : Validation structurelle**

NÅ“uds : ENTRY â†’ VALIDATION

Objectif : VÃ©rifier la validitÃ© structurelle de l'intention.

**Phase 2 : Ã‰valuation des politiques**

NÅ“uds : POLICY_1, POLICY_2, ..., POLICY_N

Objectif : Ã‰valuer chaque politique applicable.

**Phase 3 : Composition**

NÅ“uds : COMPOSITION

Objectif : AgrÃ©ger les rÃ©sultats selon les rÃ¨gles de composition.

**Phase 4 : PrioritÃ© (conditionnelle)**

NÅ“uds : PRIORITY

Objectif : Calculer la prioritÃ© si toutes les politiques sont satisfaites.

**Phase 5 : DÃ©cision**

NÅ“uds : DECISION_*

Objectif : Produire la dÃ©cision finale.

---

## 6. PropriÃ©tÃ©s du graphe

### 6.1. PropriÃ©tÃ©s structurelles

**PROP-1 : UnicitÃ© de l'entrÃ©e**

Le graphe possÃ¨de exactement un nÅ“ud d'entrÃ©e.

**PROP-2 : UnicitÃ© de la sortie logique**

Tout parcours du graphe termine par exactement un nÅ“ud de dÃ©cision.

**PROP-3 : AcyclicitÃ©**

Le graphe ne contient aucun cycle. Tout chemin de l'entrÃ©e vers une sortie est fini.

**PROP-4 : ConnexitÃ©**

Tous les nÅ“uds sont atteignables depuis le nÅ“ud d'entrÃ©e.

### 6.2. PropriÃ©tÃ©s de parcours

**PROP-5 : DÃ©terminisme**

Pour une intention et un ensemble de politiques donnÃ©s, le parcours du graphe est dÃ©terministe.

**PROP-6 : Terminaison**

Tout parcours du graphe termine en un temps fini.

**PROP-7 : ComplÃ©tude**

Toutes les politiques applicables sont Ã©valuÃ©es avant la composition.

### 6.3. PropriÃ©tÃ©s de rÃ©sultat

**PROP-8 : UnicitÃ© du rÃ©sultat**

Un parcours produit exactement une dÃ©cision.

**PROP-9 : Justification complÃ¨te**

Le chemin parcouru constitue la justification de la dÃ©cision.

---

## 7. RÃ¨gles de parcours

### 7.1. RÃ¨gles d'entrÃ©e

**R-PARC-1 : EntrÃ©e unique**

Le parcours commence toujours par le nÅ“ud d'entrÃ©e.

**R-PARC-2 : Validation obligatoire**

Le nÅ“ud de validation est toujours traversÃ© aprÃ¨s l'entrÃ©e.

### 7.2. RÃ¨gles d'Ã©valuation

**R-PARC-3 : Ã‰valuation parallÃ¨le conceptuelle**

Les nÅ“uds de politique peuvent Ãªtre conceptuellement Ã©valuÃ©s en parallÃ¨le.

**R-PARC-4 : IndÃ©pendance des Ã©valuations**

L'Ã©valuation d'une politique n'influence pas l'Ã©valuation d'une autre.

**R-PARC-5 : Attente de composition**

La composition attend tous les rÃ©sultats des politiques avant de s'exÃ©cuter.

### 7.3. RÃ¨gles de sortie

**R-PARC-6 : Sortie unique**

Un seul nÅ“ud de dÃ©cision est atteint par parcours.

**R-PARC-7 : Sortie obligatoire**

Tout parcours doit atteindre un nÅ“ud de dÃ©cision.

---

## 8. Invariants du graphe

### 8.1. Invariants structurels

**INV-GRAPH-1 : AcyclicitÃ©**

Le graphe ne contient jamais de cycle.

**INV-GRAPH-2 : EntrÃ©e unique**

Le graphe possÃ¨de toujours exactement un nÅ“ud d'entrÃ©e.

**INV-GRAPH-3 : ConnexitÃ©**

Tous les nÅ“uds sont toujours atteignables depuis l'entrÃ©e.

### 8.2. Invariants de parcours

**INV-GRAPH-4 : Terminaison garantie**

Tout parcours termine toujours par un nÅ“ud de dÃ©cision.

**INV-GRAPH-5 : DÃ©terminisme garanti**

Un mÃªme parcours avec les mÃªmes entrÃ©es produit toujours le mÃªme rÃ©sultat.

**INV-GRAPH-6 : Pas d'effet de bord**

Le parcours du graphe ne produit jamais d'effet de bord.

---

## 9. RÃ¨gles de fermeture du contrat

### 9.1. Contrat fermÃ©

Ce contrat est **fermÃ©**. Seuls les types de nÅ“uds, les types d'arÃªtes, et les propriÃ©tÃ©s explicitement dÃ©finies sont valides.

### 9.2. Interdiction d'extension implicite

Aucune extension implicite n'est autorisÃ©e :

- **INTERD-GRAPH-1** : Aucun type de nÅ“ud non dÃ©fini n'est reconnu
- **INTERD-GRAPH-2** : Aucun type d'arÃªte non dÃ©fini n'est reconnu
- **INTERD-GRAPH-3** : Aucune propriÃ©tÃ© non dÃ©finie n'est garantie

---

## 10. Conclusion contractuelle

Ce contrat Ã©tablit de maniÃ¨re dÃ©finitive et non nÃ©gociable la spÃ©cification du graphe de dÃ©cision de StrongFather.

Il garantit que :
- la structure du graphe est formellement dÃ©finie,
- les types de nÅ“uds et d'arÃªtes sont exhaustifs,
- les propriÃ©tÃ©s du graphe sont garanties,
- les rÃ¨gles de parcours sont explicites,
- les invariants sont maintenus,
- le contrat est fermÃ© et non extensible implicitement.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisÃ©e.

---

## 11. Validation conceptuelle

### 11.1. Cas conformes

Les cas suivants sont **conformes** Ã  ce contrat :

1. **Parcours complet** : EntrÃ©e â†’ Validation (valide) â†’ Politiques â†’ Composition (toutes satisfaites) â†’ PrioritÃ© â†’ DÃ©cision acceptÃ©e.

2. **Rejet structurel** : EntrÃ©e â†’ Validation (invalide) â†’ DÃ©cision refusÃ©e (type structurel).

3. **Rejet de politique** : EntrÃ©e â†’ Validation (valide) â†’ Politiques â†’ Composition (non satisfaite) â†’ DÃ©cision refusÃ©e.

### 11.2. Cas de violation

Les cas suivants **violent** ce contrat :

1. **Cycle dans le graphe** : Un parcours revient Ã  un nÅ“ud dÃ©jÃ  visitÃ©. Viole INV-GRAPH-1.

2. **Parcours sans dÃ©cision** : Un parcours se termine sans nÅ“ud de dÃ©cision. Viole INV-GRAPH-4.

3. **EntrÃ©es multiples** : Le graphe a plusieurs nÅ“uds d'entrÃ©e. Viole INV-GRAPH-2.

---

**Document crÃ©Ã© le :** 2026-01-25  
**Version :** 1.0  
**Statut :** FONDATION â€” Contrat normatif validÃ©  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, StrongFather Documentation Fondatrice  
**Type :** SpÃ©cification de graphe de dÃ©cision non nÃ©gociable

---

## 12. Mini log de gÃ©nÃ©ration

### Warning W1 : Abstraction conceptuelle

**Warning rencontrÃ© :** Risque de spÃ©cification trop technique.

**DÃ©cision prise :** PrÃ©cision que le graphe est une abstraction conceptuelle qui ne prÃ©suppose aucune implÃ©mentation technique.

**Correction effectuÃ©e :** Section 2.3 ajoutÃ©e pour clarifier l'abstraction.

### Warning W2 : ParallÃ©lisme des Ã©valuations

**Warning rencontrÃ© :** Les Ã©valuations de politiques peuvent-elles Ãªtre parallÃ¨les ?

**DÃ©cision prise :** DÃ©finition d'un parallÃ©lisme conceptuel (R-PARC-3) avec indÃ©pendance des Ã©valuations (R-PARC-4).

**Correction effectuÃ©e :** Section 7.2 prÃ©cise le parallÃ©lisme conceptuel.

### VÃ©rification de cohÃ©rence

**VÃ©rification effectuÃ©e :**
- âœ… CohÃ©rence avec Policy Engine Contract : ConfirmÃ©e (rÃ¨gles de composition)
- âœ… CohÃ©rence avec Core Decision Contract : ConfirmÃ©e (types de dÃ©cisions)
- âœ… CohÃ©rence avec Intent Model Contract : ConfirmÃ©e (nÅ“ud d'entrÃ©e)
- âœ… AcyclicitÃ© et terminaison : ConfirmÃ©es (propriÃ©tÃ©s et invariants)

**Conclusion :** Aucune contradiction dÃ©tectÃ©e.

---

*Aucune autre erreur, warning, ou ambiguÃ¯tÃ© rencontrÃ©e lors de la rÃ©daction de ce document.*

