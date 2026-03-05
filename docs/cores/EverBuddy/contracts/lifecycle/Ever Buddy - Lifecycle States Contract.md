# Ever Buddy - Lifecycle States Contract

## 1. Contexte

Ce document dÃ©finit le **contrat normatif des Ã©tats de cycle de vie** gouvernÃ©s par Ever Buddy. Les Ã©tats de cycle de vie sont les fondations de la gouvernance temporelle du systÃ¨me Miyukini. Chaque Ã©lÃ©ment du systÃ¨me (contrat, structure, interface, entitÃ©) possÃ¨de un Ã©tat de vie qui dÃ©termine son statut, ses garanties, et les actions possibles le concernant.

Ce contrat est **dÃ©rivÃ© de la Documentation Fondatrice d'Ever Buddy** (Section 4 - Concepts fondamentaux) et constitue la rÃ©fÃ©rence normative pour toute implÃ©mentation.

**Document source :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

---

## 2. PortÃ©e / Scope

- **Applicable Ã  :** Tous les Ã©lÃ©ments gouvernÃ©s par Ever Buddy (contrats, structures, interfaces, entitÃ©s, Tools, Toolkits)
- **Audience :** Architectes, dÃ©veloppeurs, cores, adaptateurs
- **Statut :** Contrat normatif â€” Non nÃ©gociable
- **DÃ©pendances :** Documentation Fondatrice Ever Buddy, Glossaire Miyukini

---

## 3. DÃ©finition des Ã©tats de cycle de vie

Chaque Ã©lÃ©ment du systÃ¨me possÃ¨de **exactement un** Ã©tat de cycle de vie Ã  tout moment (INV-EB-3). Les cinq Ã©tats valides sont dÃ©finis ci-dessous.

### 3.1 DRAFT (Brouillon)

**DÃ©finition canonique :**

> L'Ã©lÃ©ment est en cours de dÃ©finition. Il n'est pas encore utilisable en production, peut changer librement, et n'a aucun engagement de stabilitÃ©.

**CaractÃ©ristiques :**

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| **DisponibilitÃ© production** | âŒ Non |
| **StabilitÃ© garantie** | âŒ Aucune |
| **Changements autorisÃ©s** | âœ… Libres, sans contrainte |
| **Consommateurs attendus** | Aucun (dÃ©veloppement interne uniquement) |
| **Documentation requise** | Minimale (intention et direction) |
| **Support** | Aucun |

**Comportements :**

- Un Ã©lÃ©ment DRAFT **n'est pas exposÃ©** aux consommateurs externes
- Les modifications sont **libres et non annoncÃ©es**
- Aucun engagement de **rÃ©trocompatibilitÃ©**
- L'Ã©lÃ©ment peut Ãªtre **abandonnÃ© sans prÃ©avis**
- Les tests sont exploratoires, pas de validation formelle requise

**Conditions de sortie :**

- L'Ã©lÃ©ment peut transitionner vers **ACTIVE** quand il est jugÃ© prÃªt pour la production
- L'Ã©lÃ©ment peut transitionner directement vers **ARCHIVED** s'il est abandonnÃ© avant activation

**RÃ©fÃ©rence Glossaire :** [BROUILLON (..//..//..//..//miyukini-webway-system//reference//_index.md#brouillon-draft--Ã©tat-de-vie)

---

### 3.2 ACTIVE (Actif)

**DÃ©finition canonique :**

> L'Ã©lÃ©ment est en usage normal. Il est stable, documentÃ©, supportÃ©, et utilisable par tous les consommateurs autorisÃ©s. Les changements sont soumis aux rÃ¨gles de compatibilitÃ©.

**CaractÃ©ristiques :**

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| **DisponibilitÃ© production** | âœ… Oui |
| **StabilitÃ© garantie** | âœ… ComplÃ¨te |
| **Changements autorisÃ©s** | Soumis aux rÃ¨gles de compatibilitÃ© |
| **Consommateurs attendus** | Tous les consommateurs autorisÃ©s |
| **Documentation requise** | ComplÃ¨te et Ã  jour |
| **Support** | Actif (corrections, Ã©volutions mineures) |

**Comportements :**

- L'Ã©lÃ©ment est **la version de rÃ©fÃ©rence** pour les consommateurs
- Toute modification est soumise aux **rÃ¨gles de compatibilitÃ©** (INV-EB-5)
- Les Ã©volutions mineures (rÃ©trocompatibles) sont autorisÃ©es
- Les Ã©volutions majeures (incompatibles) nÃ©cessitent une **nouvelle version**
- L'Ã©lÃ©ment est **documentÃ©, testÃ©, et supportÃ©**

**Garanties aux consommateurs :**

| Garantie | Description |
|----------|-------------|
| **StabilitÃ© fonctionnelle** | Le comportement documentÃ© ne change pas |
| **RÃ©trocompatibilitÃ© par dÃ©faut** | Les Ã©volutions prÃ©servent la compatibilitÃ© sauf dÃ©claration explicite |
| **Support actif** | Les bugs critiques sont corrigÃ©s |
| **Documentation maintenue** | La documentation reflÃ¨te l'Ã©tat actuel |
| **PrÃ©avis de dÃ©prÃ©ciation** | Minimum 1 cycle de release avant dÃ©prÃ©ciation |

**Conditions de sortie :**

- L'Ã©lÃ©ment peut transitionner vers **DEPRECATED** avec annonce prÃ©alable obligatoire
- L'Ã©lÃ©ment **ne peut jamais** transitionner directement vers RETIRED ou ARCHIVED (INV-EB-4)

**RÃ©fÃ©rence Glossaire :** [ACTIF (..//..//..//..//miyukini-webway-system//reference//_index.md#actif-active--Ã©tat-de-vie)

---

### 3.3 DEPRECATED (DÃ©prÃ©ciÃ©)

**DÃ©finition canonique :**

> L'Ã©lÃ©ment est toujours fonctionnel mais son usage est dÃ©couragÃ©. Un successeur existe ou est en prÃ©paration. Les consommateurs sont avertis de migrer. La pÃ©riode de dÃ©prÃ©ciation est dÃ©finie et communiquÃ©e.

**CaractÃ©ristiques :**

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| **DisponibilitÃ© production** | âœ… Oui (mais usage dÃ©couragÃ©) |
| **StabilitÃ© garantie** | âœ… Maintenue |
| **Changements autorisÃ©s** | Corrections critiques uniquement |
| **Consommateurs attendus** | Existants (nouveaux usages dÃ©couragÃ©s) |
| **Documentation requise** | ComplÃ¨te + guide de migration |
| **Support** | Maintenance minimale |

**Comportements :**

- L'Ã©lÃ©ment **reste fonctionnel** pendant toute la pÃ©riode de dÃ©prÃ©ciation
- Un **successeur est identifiÃ©** (ou l'absence de successeur est explicite) (INV-EB-10)
- La **pÃ©riode de dÃ©prÃ©ciation** est dÃ©finie et communiquÃ©e
- Les consommateurs reÃ§oivent des **alertes de migration**
- Seules les **corrections critiques de sÃ©curitÃ©** sont appliquÃ©es
- Les **nouvelles fonctionnalitÃ©s sont refusÃ©es**

**Informations obligatoires lors de la dÃ©prÃ©ciation :**

| Information | Obligatoire | Description |
|-------------|-------------|-------------|
| **Raison de dÃ©prÃ©ciation** | âœ… | Pourquoi l'Ã©lÃ©ment est dÃ©prÃ©ciÃ© |
| **Successeur identifiÃ©** | âœ… | L'Ã©lÃ©ment de remplacement (ou "aucun") |
| **Date de dÃ©but de dÃ©prÃ©ciation** | âœ… | Quand la pÃ©riode commence |
| **Date prÃ©vue de retirement** | âœ… | Quand la pÃ©riode se termine |
| **Guide de migration** | âœ… | Comment migrer vers le successeur |
| **Impact sur les consommateurs** | âœ… | Ce qui change pour eux |

**Conditions de sortie :**

- L'Ã©lÃ©ment peut transitionner vers **RETIRED** Ã  la fin de la pÃ©riode de dÃ©prÃ©ciation
- L'Ã©lÃ©ment peut Ãªtre **rÃ©activÃ© vers ACTIVE** si le successeur est annulÃ© (cas exceptionnel)

**RÃ©fÃ©rence Glossaire :** [DÃ‰PRÃ‰CIÃ‰ (..//..//..//..//miyukini-webway-system//reference//_index.md#dÃ©prÃ©ciÃ©-deprecated--Ã©tat-de-vie)

---

### 3.4 RETIRED (RetirÃ©)

**DÃ©finition canonique :**

> L'Ã©lÃ©ment n'est plus activement supportÃ© mais reste fonctionnel pour les consommateurs existants. Aucune nouvelle fonctionnalitÃ©, uniquement des corrections critiques de sÃ©curitÃ©.

**CaractÃ©ristiques :**

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| **DisponibilitÃ© production** | âš ï¸ LimitÃ©e (existants uniquement) |
| **StabilitÃ© garantie** | âš ï¸ Best effort |
| **Changements autorisÃ©s** | Corrections sÃ©curitÃ© critiques uniquement |
| **Consommateurs attendus** | Uniquement ceux n'ayant pas pu migrer |
| **Documentation requise** | ArchivÃ©e (non maintenue) |
| **Support** | Aucun (sauf sÃ©curitÃ© critique) |

**Comportements :**

- L'Ã©lÃ©ment **reste techniquement fonctionnel** mais n'est plus recommandÃ©
- **Aucune nouvelle fonctionnalitÃ©** n'est ajoutÃ©e
- **Aucune correction de bug** (sauf sÃ©curitÃ© critique)
- Les **nouveaux consommateurs sont bloquÃ©s** (l'Ã©lÃ©ment n'est pas proposÃ©)
- La **documentation n'est plus maintenue**
- Une **pÃ©riode de grÃ¢ce** peut Ãªtre accordÃ©e aux retardataires

**Droits rÃ©siduels des consommateurs existants :**

| Droit | Garanti |
|-------|---------|
| **Fonctionnement continu** | âœ… Tant que l'Ã©lÃ©ment est RETIRED |
| **Corrections de sÃ©curitÃ©** | âœ… Critiques uniquement |
| **Support technique** | âŒ Non |
| **Nouvelles fonctionnalitÃ©s** | âŒ Non |
| **Documentation Ã  jour** | âŒ Non |

**Conditions de sortie :**

- L'Ã©lÃ©ment peut transitionner vers **ARCHIVED** aprÃ¨s la pÃ©riode de grÃ¢ce
- L'Ã©lÃ©ment **ne peut jamais** revenir Ã  ACTIVE ou DEPRECATED

**RÃ©fÃ©rence Glossaire :** [RETIRÃ‰ (..//..//..//..//miyukini-webway-system//reference//_index.md#retirÃ©-retired--Ã©tat-de-vie)

---

### 3.5 ARCHIVED (ArchivÃ©)

**DÃ©finition canonique :**

> L'Ã©lÃ©ment n'est plus fonctionnel. Il est conservÃ© uniquement pour rÃ©fÃ©rence historique et traÃ§abilitÃ©. Aucune garantie de fonctionnement.

**CaractÃ©ristiques :**

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| **DisponibilitÃ© production** | âŒ Non |
| **StabilitÃ© garantie** | âŒ Aucune |
| **Changements autorisÃ©s** | âŒ Aucun (Ã©lÃ©ment gelÃ©) |
| **Consommateurs attendus** | Aucun |
| **Documentation requise** | ArchivÃ©e (lecture seule) |
| **Support** | âŒ Aucun |

**Comportements :**

- L'Ã©lÃ©ment **n'est plus exÃ©cutable** en production
- L'Ã©lÃ©ment est conservÃ© comme **tombstone** (rÃ©fÃ©rence historique)
- La **traÃ§abilitÃ© complÃ¨te** est maintenue (INV-EB-2)
- **Aucune modification** n'est possible
- L'Ã©lÃ©ment **ne peut jamais Ãªtre rÃ©activÃ©**

**Ce qui est conservÃ© dans l'archive :**

| Ã‰lÃ©ment | ConservÃ© |
|---------|----------|
| **MÃ©tadonnÃ©es** | âœ… ID, nom, version, dates |
| **Historique des transitions** | âœ… ChaÃ®ne d'Ã©volution complÃ¨te |
| **Documentation finale** | âœ… Snapshot au moment de l'archivage |
| **Raison de l'archivage** | âœ… Justification documentÃ©e |
| **RÃ©fÃ©rence au successeur** | âœ… Si applicable |
| **DonnÃ©es fonctionnelles** | âŒ Non (tombstone uniquement) |

**Conditions de sortie :**

- **Aucune transition possible** depuis ARCHIVED
- L'Ã©tat ARCHIVED est **terminal et dÃ©finitif**

---

## 4. Tableau rÃ©capitulatif des Ã©tats

| Ã‰tat | Production | StabilitÃ© | Support | Ã‰volutions | RÃ©versible |
|------|------------|-----------|---------|------------|------------|
| **DRAFT** | âŒ | âŒ | âŒ | Libres | âœ… â†’ ACTIVE ou ARCHIVED |
| **ACTIVE** | âœ… | âœ… | âœ… | Compatibles | âœ… â†’ DEPRECATED |
| **DEPRECATED** | âš ï¸ | âœ… | âš ï¸ | SÃ©curitÃ© seulement | âš ï¸ â†’ RETIRED ou ACTIVE* |
| **RETIRED** | âš ï¸ | âš ï¸ | âŒ | SÃ©curitÃ© critique | âœ… â†’ ARCHIVED |
| **ARCHIVED** | âŒ | âŒ | âŒ | Aucune | âŒ Terminal |

*La rÃ©activation DEPRECATED â†’ ACTIVE est exceptionnelle et conditionnÃ©e.

---

## 5. RÃ¨gles applicables aux Ã©tats

### 5.1 RÃ¨gle d'unicitÃ© d'Ã©tat (INV-EB-3)

> Chaque Ã©lÃ©ment du systÃ¨me possÃ¨de **exactement un** Ã©tat de cycle de vie Ã  tout moment. Il n'existe pas d'Ã©tat intermÃ©diaire, incertain, ou non dÃ©fini.

**Violations :**
- Un Ã©lÃ©ment sans Ã©tat dÃ©clarÃ©
- Un Ã©lÃ©ment avec plusieurs Ã©tats simultanÃ©s
- Un Ã©lÃ©ment dans un Ã©tat "en transition"

### 5.2 RÃ¨gle de dÃ©prÃ©ciation obligatoire (INV-EB-4)

> Aucun Ã©lÃ©ment ACTIVE ne peut passer directement Ã  RETIRED ou ARCHIVED. La transition par DEPRECATED est **obligatoire**.

**Violations :**
- ACTIVE â†’ RETIRED (interdit)
- ACTIVE â†’ ARCHIVED (interdit)
- Toute tentative de "fast-track" vers retirement

### 5.3 RÃ¨gle de rÃ©trocompatibilitÃ© par dÃ©faut (INV-EB-5)

> Toute Ã©volution est **prÃ©sumÃ©e rÃ©trocompatible** sauf dÃ©claration explicite contraire.

**Implication pour les Ã©tats :**
- Un Ã©lÃ©ment ACTIVE qui Ã©volue reste ACTIVE
- Une Ã©volution incompatible crÃ©e un **nouvel Ã©lÃ©ment** (nouvelle version majeure)
- L'ancien Ã©lÃ©ment passe Ã  DEPRECATED

### 5.4 RÃ¨gle de documentation obligatoire (INV-EB-7)

> Toute transition d'Ã©tat doit Ãªtre **documentÃ©e**.

**Documentation minimale par transition :**

| Transition | Documentation requise |
|------------|----------------------|
| DRAFT â†’ ACTIVE | Raison d'activation, documentation complÃ¨te |
| ACTIVE â†’ DEPRECATED | Raison, successeur, pÃ©riode, guide de migration |
| DEPRECATED â†’ RETIRED | Confirmation fin de pÃ©riode, consommateurs restants |
| RETIRED â†’ ARCHIVED | Raison d'archivage, snapshot final |
| DRAFT â†’ ARCHIVED | Raison d'abandon |
| DEPRECATED â†’ ACTIVE | Justification de rÃ©activation (exceptionnel) |

---

## 6. Application aux Tools et Toolkits

Ever Buddy gouverne le cycle de vie des **Tools** (Strate 6) avec les mÃªmes Ã©tats, mais avec des rÃ¨gles spÃ©cifiques.

### 6.1 Ã‰tats de vie des Tools

| Ã‰tat | Description Tool |
|------|------------------|
| **DRAFT** | Tool en dÃ©veloppement, non disponible |
| **ACTIVE** | Tool disponible et supportÃ© |
| **DEPRECATED** | Tool fonctionnel mais usage dÃ©couragÃ© |
| **RETIRED** | Tool retirÃ©, non disponible |

Note : L'Ã©tat ARCHIVED n'est gÃ©nÃ©ralement pas utilisÃ© pour les Tools (ils sont directement retirÃ©s puis oubliÃ©s au niveau opÃ©rationnel, mais tracÃ©s au niveau historique).

### 6.2 RÃ¨gles spÃ©cifiques aux Tools

| RÃ¨gle | Description |
|-------|-------------|
| **RÃˆGLE-TOOL-EV-1** | Tout Tool a un Ã©tat de vie explicite |
| **RÃˆGLE-TOOL-EV-2** | Un Tool DEPRECATED a un successeur identifiÃ© |
| **RÃˆGLE-TOOL-EV-3** | La transition vers RETIRED passe obligatoirement par DEPRECATED |
| **RÃˆGLE-TOOL-EV-4** | La compatibilitÃ© Tool â†” Environnement est vÃ©rifiÃ©e |

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Tools et Toolkits](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 7. CatÃ©gories d'Ã©lÃ©ments et rÃ¨gles d'Ã©tat

Les rÃ¨gles d'Ã©tat varient selon la **catÃ©gorie** de l'Ã©lÃ©ment.

### 7.1 Contrats fondateurs (FONDATION)

| Aspect | Valeur |
|--------|--------|
| **Temps minimum en ACTIVE** | TrÃ¨s long (plusieurs gÃ©nÃ©rations) |
| **PÃ©riode de dÃ©prÃ©ciation** | TrÃ¨s longue |
| **Ruptures** | Quasi interdites |
| **Exemple** | Documentation Fondatrice d'un core |

### 7.2 Contrats opÃ©rationnels

| Aspect | Valeur |
|--------|--------|
| **Temps minimum en ACTIVE** | Standard |
| **PÃ©riode de dÃ©prÃ©ciation** | Standard |
| **Ruptures** | Possibles avec justification |
| **Exemple** | API Contract, Interface Contract |

### 7.3 Interfaces techniques

| Aspect | Valeur |
|--------|--------|
| **Temps minimum en ACTIVE** | Court |
| **PÃ©riode de dÃ©prÃ©ciation** | Courte |
| **Ruptures** | Possibles avec documentation |
| **Exemple** | Adaptateur, Tool |

### 7.4 Ã‰lÃ©ments internes

| Aspect | Valeur |
|--------|--------|
| **Temps minimum en ACTIVE** | Aucun minimum |
| **PÃ©riode de dÃ©prÃ©ciation** | Optionnelle |
| **Ruptures** | Sans prÃ©avis autorisÃ©es |
| **Exemple** | ImplÃ©mentation interne |

---

## 8. Invariants applicables aux Ã©tats

Ce contrat est gouvernÃ© par les invariants suivants de la Documentation Fondatrice :

| Invariant | Ã‰noncÃ© | Application aux Ã©tats |
|-----------|--------|----------------------|
| **INV-EB-2** | TraÃ§abilitÃ© complÃ¨te et immuable | Chaque transition d'Ã©tat est enregistrÃ©e |
| **INV-EB-3** | Aucun Ã©tat ambigu | Un seul Ã©tat Ã  tout moment |
| **INV-EB-4** | PÃ©riode de dÃ©prÃ©ciation obligatoire | DEPRECATED obligatoire avant RETIRED |
| **INV-EB-5** | RÃ©trocompatibilitÃ© par dÃ©faut | Les Ã©volutions en ACTIVE sont compatibles |
| **INV-EB-7** | Documentation obligatoire | Chaque transition est documentÃ©e |
| **INV-EB-9** | PrÃ©dictibilitÃ© des transitions | Les rÃ¨gles d'Ã©tat sont publiques |

---

## 9. ConformitÃ© aux Lois d'Autonomie

Ce contrat respecte les Lois d'Autonomie SystÃ¨me :

| Loi | ConformitÃ© | MÃ©canisme |
|-----|------------|-----------|
| **LOI-1** | âœ… | Ã‰tats stockÃ©s localement, pas de dÃ©pendance externe |
| **LOI-2** | âœ… | Ã‰tats valides en mode isolÃ© |
| **LOI-3** | âœ… | Ã‰tat local souverain |
| **LOI-4** | âœ… | Ã‰tats discrets, pas de temps global requis |

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 10. RÃ©fÃ©rences croisÃ©es

- **Document source :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)
- **Contrat complÃ©mentaire :** [Ever Buddy - Transition Rules Contract](./Ever%20Buddy%20-%20Transition%20Rules%20Contract.md) (transitions entre Ã©tats)
- **Glossaire :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)
- **Lois d'Autonomie :** [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)
- **Tools et Toolkits :** [Miyukini Conceptual References - Tools et Toolkits](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contrat normatif â€” Non nÃ©gociable  
**DÃ©rivÃ© de :** Ever Buddy - Documentation Fondatrice v1.3, Section 4  
**Type :** Contrat de cycle de vie

