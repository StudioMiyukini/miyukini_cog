# Ever Buddy - Vocabulary & Glossary

## Contexte

Ce document constitue le **vocabulaire canonique** d'Ever Buddy, le core de cycle de vie et d'Ã©volution du Miyukini Core System (Strate 4). Il regroupe toutes les dÃ©finitions officielles des termes utilisÃ©s dans le domaine d'Ever Buddy.

**Document de rÃ©fÃ©rence :** [Ever Buddy - Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

**Glossaire gÃ©nÃ©ral :** [Miyukini Conceptual References - Glossaire](..//..//..//miyukini-webway-system//reference//_index.md)

## PortÃ©e / Scope

- **Applicable Ã  :** Toute documentation Ever Buddy, communications, implÃ©mentations
- **Audience :** Architectes, dÃ©veloppeurs, intÃ©grateurs, Ã©quipes documentation
- **Statut :** Document de rÃ©fÃ©rence normatif â€” Vocabulaire canonique Ever Buddy

---

## Ã‰tats de cycle de vie

### DRAFT (BROUILLON)

Ã‰tat d'un Ã©lÃ©ment en cours de dÃ©finition. Il n'est pas encore utilisable en production, peut changer librement, et n'a aucun engagement de stabilitÃ©.

**CaractÃ©ristiques :**
- Non utilisable en production
- Changements libres autorisÃ©s
- Aucune garantie de stabilitÃ©

**Transitions possibles :** DRAFT â†’ ACTIVE, DRAFT â†’ ARCHIVED

**RÃ©fÃ©rence glossaire :** [BROUILLON (..//..//..//miyukini-webway-system//reference//_index.md#brouillon-draft--Ã©tat-de-vie)

---

### ACTIVE (ACTIF)

Ã‰tat d'un Ã©lÃ©ment en usage normal. Il est stable, documentÃ©, supportÃ©, et utilisable par tous les consommateurs autorisÃ©s. Les changements sont soumis aux rÃ¨gles de compatibilitÃ©.

**CaractÃ©ristiques :**
- Stable et documentÃ©
- SupportÃ© activement
- Changements soumis aux rÃ¨gles de compatibilitÃ©
- Utilisable par tous les consommateurs autorisÃ©s

**Transitions possibles :** ACTIVE â†’ DEPRECATED

**RÃ©fÃ©rence glossaire :** [ACTIF (..//..//..//miyukini-webway-system//reference//_index.md#actif-active--Ã©tat-de-vie)

---

### DEPRECATED (DÃ‰PRÃ‰CIÃ‰)

Ã‰tat d'un Ã©lÃ©ment toujours fonctionnel mais dont l'usage est dÃ©couragÃ©. Un successeur existe ou est en prÃ©paration. Les consommateurs sont avertis de migrer. La pÃ©riode de dÃ©prÃ©ciation est dÃ©finie et communiquÃ©e.

**CaractÃ©ristiques :**
- Toujours fonctionnel
- Usage dÃ©couragÃ©
- Successeur identifiÃ© (ou annoncÃ© comme "sans successeur")
- PÃ©riode de dÃ©prÃ©ciation dÃ©finie

**Transitions possibles :** DEPRECATED â†’ RETIRED, DEPRECATED â†’ ACTIVE (rÃ©activation exceptionnelle)

**RÃ©fÃ©rence glossaire :** [DÃ‰PRÃ‰CIÃ‰ (..//..//..//miyukini-webway-system//reference//_index.md#dÃ©prÃ©ciÃ©-deprecated--Ã©tat-de-vie)

---

### RETIRED (RETIRÃ‰)

Ã‰tat d'un Ã©lÃ©ment qui n'est plus activement supportÃ© mais reste fonctionnel pour les consommateurs existants. Aucune nouvelle fonctionnalitÃ© n'est ajoutÃ©e, uniquement des corrections critiques de sÃ©curitÃ©.

**CaractÃ©ristiques :**
- Plus activement supportÃ©
- Fonctionnel pour consommateurs existants
- Uniquement corrections critiques de sÃ©curitÃ©
- Aucune nouvelle fonctionnalitÃ©

**Transitions possibles :** RETIRED â†’ ARCHIVED

**RÃ©fÃ©rence glossaire :** [RETIRÃ‰ (..//..//..//miyukini-webway-system//reference//_index.md#retirÃ©-retired--Ã©tat-de-vie)

---

### ARCHIVED (ARCHIVÃ‰)

Ã‰tat d'un Ã©lÃ©ment qui n'est plus fonctionnel. Il est conservÃ© uniquement pour rÃ©fÃ©rence historique et traÃ§abilitÃ©. Aucune garantie de fonctionnement.

**CaractÃ©ristiques :**
- Non fonctionnel
- Conservation pour rÃ©fÃ©rence historique uniquement
- Aucune garantie de fonctionnement
- TraÃ§abilitÃ© prÃ©servÃ©e

**Transitions possibles :** Aucune (Ã©tat terminal)

---

## Concepts de transition

### Transition

Une **transition** est le passage d'un Ã©tat de cycle de vie Ã  un autre. Les transitions sont atomiques, documentÃ©es, et validÃ©es par Ever Buddy.

**CaractÃ©ristiques :**
- Atomique (pas d'Ã©tat intermÃ©diaire)
- DocumentÃ©e obligatoirement
- ValidÃ©e par Ever Buddy
- EnregistrÃ©e dans l'historique immuable

**Invariant associÃ© :** INV-EB-3 (Aucun Ã©tat ambigu)

---

### Ã‰volution contrÃ´lÃ©e

Une **Ã©volution contrÃ´lÃ©e** est un changement structurel qui respecte les principes de continuitÃ© et de compatibilitÃ©.

**Composantes obligatoires :**

| Composante | Description |
|------------|-------------|
| **Annonce prÃ©alable** | L'Ã©volution est communiquÃ©e avant sa mise en Å“uvre |
| **PÃ©riode de transition** | DurÃ©e pendant laquelle l'ancien et le nouveau coexistent |
| **Documentation des diffÃ©rences** | Les changements sont explicitement documentÃ©s |
| **Chemin de migration** | Guide pour passer de l'ancien au nouveau |
| **CritÃ¨res de complÃ©tion** | Conditions claires dÃ©finissant la fin de transition |

---

### Coexistence

La **coexistence** est la pÃ©riode pendant laquelle deux versions (ou plus) d'un Ã©lÃ©ment sont simultanÃ©ment disponibles. Cette pÃ©riode permet aux consommateurs de migrer progressivement.

**RÃ¨gles :**
- L'ancienne et la nouvelle version sont toutes deux fonctionnelles
- Les consommateurs choisissent leur rythme de migration
- La pÃ©riode a une durÃ©e dÃ©finie et communiquÃ©e

---

### Sunset

Le **sunset** est le processus planifiÃ© de fin de vie d'un Ã©lÃ©ment. Il comprend la sÃ©quence complÃ¨te de retrait.

**SÃ©quence du sunset :**
1. DÃ©prÃ©ciation (ACTIVE â†’ DEPRECATED)
2. PÃ©riode de transition (coexistence)
3. Retirement (DEPRECATED â†’ RETIRED)
4. PÃ©riode de grÃ¢ce (optionnelle)
5. Archivage (RETIRED â†’ ARCHIVED)

---

### PÃ©riode de grÃ¢ce (Grace period)

La **pÃ©riode de grÃ¢ce** est le temps supplÃ©mentaire accordÃ© aprÃ¨s la date prÃ©vue de retirement, pour permettre aux consommateurs retardataires de migrer. Cette pÃ©riode est accordÃ©e au cas par cas.

**CaractÃ©ristiques :**
- AccordÃ©e exceptionnellement
- DurÃ©e variable selon l'impact
- Ne suspend pas le statut RETIRED

---

## Concepts de compatibilitÃ©

### RÃ©trocompatibilitÃ©

Un Ã©lÃ©ment est **rÃ©trocompatible** quand le nouveau fonctionne avec l'ancien. Les consommateurs existants continuent de fonctionner sans modification.

**Invariant associÃ© :** INV-EB-5 (RÃ©trocompatibilitÃ© par dÃ©faut)

**RÃ¨gle :** Toute Ã©volution est prÃ©sumÃ©e rÃ©trocompatible sauf dÃ©claration explicite contraire.

---

### CompatibilitÃ© en amont

Un Ã©lÃ©ment est **compatible en amont** quand l'ancien fonctionne avec le nouveau. Les nouvelles fonctionnalitÃ©s sont accessibles aux anciennes versions.

**Note :** La compatibilitÃ© en amont est rare et souvent impossible Ã  garantir.

---

### IncompatibilitÃ©

Un Ã©lÃ©ment est **incompatible** quand le nouveau ne fonctionne pas avec l'ancien. Une migration est obligatoire.

**ConsÃ©quences :**
- Transition de version majeure requise
- PÃ©riode de dÃ©prÃ©ciation obligatoire (INV-EB-4)
- Documentation explicite du breaking change

---

### Breaking change

Un **breaking change** est un changement qui rompt la compatibilitÃ© avec les versions prÃ©cÃ©dentes.

**Exigences :**
- Transition de version majeure
- PÃ©riode de dÃ©prÃ©ciation de l'ancienne version
- Justification documentÃ©e
- Plan de transition fourni

---

### FenÃªtre de compatibilitÃ© (Compatibility window)

La **fenÃªtre de compatibilitÃ©** est la plage de versions avec lesquelles un Ã©lÃ©ment garantit la compatibilitÃ©.

**Exemple :** "Compatible avec v2.0 Ã  v2.4" dÃ©finit une fenÃªtre de compatibilitÃ© de 5 versions mineures.

**Usage :** Permet aux consommateurs de planifier leurs propres Ã©volutions.

---

## Concepts de versionnement

### Versionnement sÃ©mantique

Le **versionnement sÃ©mantique** est la maniÃ¨re dont Ever Buddy identifie et distingue les diffÃ©rentes versions d'un Ã©lÃ©ment.

**Structure : MAJEUR.MINEUR.CORRECTIF**

| Niveau | Signification | Impact compatibilitÃ© |
|--------|---------------|---------------------|
| **Majeur** | Changement incompatible, rupture de contrat | Breaking change |
| **Mineur** | Ajout de fonctionnalitÃ© | RÃ©trocompatible |
| **Correctif** | Correction de bug | Aucun changement fonctionnel |

---

### GÃ©nÃ©ration

Une **gÃ©nÃ©ration** est une version majeure d'un Ã©lÃ©ment ou d'un ensemble d'Ã©lÃ©ments qui partagent une base conceptuelle commune.

**CaractÃ©ristiques :**
- NumÃ©rotÃ©e (1.x, 2.x, 3.x...)
- TraÃ§able dans l'historique
- Base conceptuelle distincte de la gÃ©nÃ©ration prÃ©cÃ©dente

**Invariant associÃ© :** INV-EB-6 (Vision long terme obligatoire â€” considÃ©rer au moins deux gÃ©nÃ©rations)

---

### ChaÃ®ne d'Ã©volution (Evolution chain)

La **chaÃ®ne d'Ã©volution** est la sÃ©quence complÃ¨te des versions d'un Ã©lÃ©ment, de sa crÃ©ation Ã  son Ã©tat actuel. Elle inclut tous les prÃ©dÃ©cesseurs et successeurs.

**Contenu :**
- Toutes les versions depuis la crÃ©ation
- Toutes les transitions enregistrÃ©es
- Tous les successeurs et prÃ©dÃ©cesseurs

---

## Concepts de succession

### Successeur

Un **successeur** est l'Ã©lÃ©ment qui remplace un Ã©lÃ©ment dÃ©prÃ©ciÃ© ou retirÃ©. Le successeur peut Ãªtre une nouvelle version du mÃªme Ã©lÃ©ment ou un Ã©lÃ©ment entiÃ¨rement diffÃ©rent.

**Invariant associÃ© :** INV-EB-10 (UnicitÃ© du successeur dÃ©clarÃ©)

**RÃ¨gle :** Un Ã©lÃ©ment dÃ©prÃ©ciÃ© possÃ¨de au plus un successeur dÃ©clarÃ© Ã  tout moment.

---

### PrÃ©dÃ©cesseur

Un **prÃ©dÃ©cesseur** est l'Ã©lÃ©ment qui a Ã©tÃ© remplacÃ© par l'Ã©lÃ©ment actuel. La chaÃ®ne des prÃ©dÃ©cesseurs forme l'historique d'Ã©volution.

---

## Concepts de dette et surveillance

### Dette structurelle

La **dette structurelle** est l'ensemble des Ã©lÃ©ments DEPRECATED ou RETIRED qui persistent dans le systÃ¨me. Cette dette n'est pas nÃ©cessairement nÃ©gative â€” elle est le prix de la continuitÃ©.

**Surveillance :** Ever Buddy mesure et alerte quand la dette devient excessive.

**Voir aussi :** [Ever Buddy - Debt Tracking Contract](../contracts/observability/Ever%20Buddy%20-%20Debt%20Tracking%20Contract.md)

---

### Debt ratio

Le **debt ratio** est le rapport entre les Ã©lÃ©ments DEPRECATED/RETIRED et les Ã©lÃ©ments ACTIVE.

**Formule :** `Debt ratio = (DEPRECATED + RETIRED) / ACTIVE`

**Usage :** Ever Buddy surveille ce ratio et alerte quand il dÃ©passe des seuils dÃ©finis.

---

### Taux d'adoption (Adoption rate)

Le **taux d'adoption** est le pourcentage de consommateurs qui ont migrÃ© vers le successeur d'un Ã©lÃ©ment dÃ©prÃ©ciÃ©.

**Usage :** Ever Buddy surveille ce taux pour dÃ©terminer quand une transition peut Ãªtre complÃ©tÃ©e.

**CritÃ¨re de complÃ©tion :** Un taux d'adoption suffisant permet le passage de DEPRECATED Ã  RETIRED.

---

## Concepts techniques

### Migration

Une **migration** est l'ensemble des actions nÃ©cessaires pour passer d'une version Ã  une autre.

**RÃ¨gle fondamentale :** Ever Buddy dÃ©finit les migrations conceptuellement mais **ne les exÃ©cute jamais** (INV-EB-1).

**ResponsabilitÃ© d'exÃ©cution :**
- KindMother pour les donnÃ©es
- Produits pour leur code

---

### Freeze (Gel)

Un **freeze** est le gel d'un Ã©lÃ©ment Ã  un Ã©tat donnÃ©. Un Ã©lÃ©ment gelÃ© ne peut plus Ã©voluer (sauf corrections critiques de sÃ©curitÃ©).

**Usage :** Stabiliser les versions en production.

**RÃ©fÃ©rence glossaire :** [Gel local (..//..//..//miyukini-webway-system//reference//_index.md#gel-local-local-freeze)

---

### Tombstone

Un **tombstone** est l'enregistrement minimal conservÃ© pour un Ã©lÃ©ment archivÃ©. Il contient uniquement les mÃ©tadonnÃ©es nÃ©cessaires Ã  la traÃ§abilitÃ© historique, pas les donnÃ©es fonctionnelles.

**Contenu d'un tombstone :**
- Identifiant de l'Ã©lÃ©ment
- Dates de crÃ©ation et d'archivage
- ChaÃ®ne d'Ã©volution (successeur, prÃ©dÃ©cesseur)
- Raison de l'archivage

---

## CatÃ©gories d'Ã©lÃ©ments

Ever Buddy distingue les Ã©lÃ©ments par leur catÃ©gorie, chaque catÃ©gorie ayant des rÃ¨gles d'Ã©volution spÃ©cifiques.

### Contrats fondateurs (FONDATION)

**CaractÃ©ristiques :**
- Ã‰volution extrÃªmement lente
- PÃ©riodes de transition trÃ¨s longues
- Ruptures quasiment interdites

**Exemples :** Documentations fondatrices des cores, invariants systÃ¨me

---

### Contrats opÃ©rationnels

**CaractÃ©ristiques :**
- Ã‰volution modÃ©rÃ©e
- PÃ©riodes de transition standards
- Ruptures possibles avec justification

**Exemples :** Contrats d'API, spÃ©cifications d'interfaces

---

### Interfaces techniques

**CaractÃ©ristiques :**
- Ã‰volution plus rapide
- PÃ©riodes de transition courtes
- Ruptures possibles avec documentation

**Exemples :** Interfaces de modules, adaptateurs

---

### Ã‰lÃ©ments internes

**CaractÃ©ristiques :**
- Ã‰volution libre
- Pas de garantie de stabilitÃ© externe
- Ruptures sans prÃ©avis autorisÃ©es

**Exemples :** ImplÃ©mentations internes, utilitaires privÃ©s

---

## Matrice rÃ©capitulative des transitions

| Depuis \ Vers | DRAFT | ACTIVE | DEPRECATED | RETIRED | ARCHIVED |
|---------------|-------|--------|------------|---------|----------|
| **DRAFT**     | â€”     | âœ“      | âœ—          | âœ—       | âœ“        |
| **ACTIVE**    | âœ—     | â€”      | âœ“          | âœ—       | âœ—        |
| **DEPRECATED**| âœ—     | âœ“*     | â€”          | âœ“       | âœ—        |
| **RETIRED**   | âœ—     | âœ—      | âœ—          | â€”       | âœ“        |
| **ARCHIVED**  | âœ—     | âœ—      | âœ—          | âœ—       | â€”        |

*La rÃ©activation DEPRECATED â†’ ACTIVE est possible uniquement si le successeur est annulÃ© et que l'Ã©lÃ©ment dÃ©prÃ©ciÃ© est toujours fonctionnel.

---

## Tableau de correspondance terminologique

| Terme anglais | Terme franÃ§ais | DÃ©finition courte |
|---------------|----------------|-------------------|
| Lifecycle | Cycle de vie | Ensemble des Ã©tats d'un Ã©lÃ©ment |
| Transition | Transition | Passage d'un Ã©tat Ã  un autre |
| Generation | GÃ©nÃ©ration | Version majeure conceptuelle |
| Coexistence | Coexistence | PÃ©riode de double disponibilitÃ© |
| Sunset | Sunset | Processus planifiÃ© de fin de vie |
| Successor | Successeur | Ã‰lÃ©ment de remplacement |
| Predecessor | PrÃ©dÃ©cesseur | Ã‰lÃ©ment remplacÃ© |
| Breaking change | Rupture de compatibilitÃ© | Changement incompatible |
| Migration | Migration | Actions pour changer de version |
| Freeze | Gel | Blocage d'Ã©volution |
| Debt ratio | Ratio de dette | Mesure de dette structurelle |
| Adoption rate | Taux d'adoption | Pourcentage de migration |
| Grace period | PÃ©riode de grÃ¢ce | Temps supplÃ©mentaire accordÃ© |
| Compatibility window | FenÃªtre de compatibilitÃ© | Plage de versions compatibles |
| Evolution chain | ChaÃ®ne d'Ã©volution | Historique complet des versions |
| Tombstone | Tombstone | Enregistrement minimal archivÃ© |

---

## RÃ©fÃ©rences croisÃ©es

### Documents Ever Buddy

- [Ever Buddy - Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) â€” Section 9 : Vocabulaire canonique
- [Ever Buddy - Lifecycle States Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md) â€” Ã‰tats dÃ©taillÃ©s
- [Ever Buddy - Transition Rules Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md) â€” RÃ¨gles de transition
- [Ever Buddy - Compatibility Rules Contract](../contracts/compatibility/Ever%20Buddy%20-%20Compatibility%20Rules%20Contract.md) â€” RÃ¨gles de compatibilitÃ©
- [Ever Buddy - Version Semantics Contract](../contracts/compatibility/Ever%20Buddy%20-%20Version%20Semantics%20Contract.md) â€” Versionnement sÃ©mantique
- [Ever Buddy - Debt Tracking Contract](../contracts/observability/Ever%20Buddy%20-%20Debt%20Tracking%20Contract.md) â€” Surveillance dette
- [Ever Buddy - Invariants & Guarantees](../contracts/governance/Ever%20Buddy%20-%20Invariants%20%26%20Guarantees.md) â€” INV-EB-1 Ã  INV-EB-12

### Documents de rÃ©fÃ©rence Miyukini

- [Miyukini Conceptual References - Glossaire](..//..//..//miyukini-webway-system//reference//_index.md) â€” Glossaire gÃ©nÃ©ral
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//miyukini-webway-system//reference//_index.md) â€” ConformitÃ© LOI-1 Ã  LOI-6

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Document de rÃ©fÃ©rence normatif â€” Vocabulaire canonique Ever Buddy  
**Source :** Documentation Fondatrice Ever Buddy v1.3, Section 9

