# Caring Nanny - Versioning & Evolution Contract

## 1. Contexte

Ce document dÃ©finit les rÃ¨gles de versionnement et d'Ã©volution de Caring Nanny. Il Ã©tablit comment Caring Nanny Ã©volue dans le temps tout en prÃ©servant la stabilitÃ© de l'observation d'Ã©tat, les invariants fondamentaux, et les garanties envers les consommateurs d'Ã©tat du Miyukini Core System.

Ce document complÃ¨te la Section 7 (Invariants non nÃ©gociables) de la [Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) et s'appuie sur le document [Invariants et Garanties](../governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md) pour dÃ©finir les rÃ¨gles d'Ã©volution des propriÃ©tÃ©s contractuelles.

L'Ã©volution respecte les [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md) : les nouvelles versions doivent maintenir l'autonomie locale (**LOI-1**, **LOI-2**, **LOI-3**), le fonctionnement sans temps global (**LOI-4**), et la proportionnalitÃ© des ressources (**LOI-5**).

## 2. PortÃ©e / Scope

Ce document couvre :
- Le schÃ©ma de versionnement (sÃ©mantique)
- Les rÃ¨gles d'Ã©volution des contrats d'observation
- Les rÃ¨gles d'Ã©volution des invariants et garanties
- Les rÃ¨gles de dÃ©prÃ©ciation
- Les rÃ¨gles de compatibilitÃ© ascendante
- Le processus d'Ã©volution
- La gestion des breaking changes
- Les rÃ¨gles de gel

Ce document **ne couvre pas** :
- Les dÃ©tails d'implÃ©mentation du versionnement
- Le versionnement des autoritÃ©s (KindMother, StrongFather, BondingBrother)
- Les rÃ¨gles de test et validation (voir Testing & Validation Contract)
- Les rÃ¨gles de performance (voir Performance & Scalability Contract)

---

## 3. Principes fondamentaux

### 3.1 Observateur stable

**Principe EVOL-CN-01 : Observation stable**

L'interface d'observation de Caring Nanny est stable. Les consommateurs d'Ã©tat peuvent compter sur la cohÃ©rence et la disponibilitÃ© de l'observation. Les changements rÃ©tro-incompatibles sont exceptionnels et suivent un processus formel.

**Implications :**
- Les composants interrogeant Caring Nanny continuent de fonctionner
- Les nouvelles catÃ©gories d'Ã©tat sont additives
- Les breaking changes nÃ©cessitent une version majeure
- PÃ©riode de dÃ©prÃ©ciation avant suppression

### 3.2 Ã‰volution par extension

**Principe EVOL-CN-02 : Extension, pas modification**

Caring Nanny Ã©volue par extension (ajout de fonctionnalitÃ©s d'observation) plutÃ´t que par modification (changement de fonctionnalitÃ©s existantes).

**Implications :**
- Nouvelles catÃ©gories d'Ã©tat (au-delÃ  de healthy, degraded, offline, syncing, error)
- Nouvelles conditions dÃ©tectables
- Nouveaux formats de notification
- PrÃ©servation des interfaces d'interrogation existantes
- Coexistence de plusieurs versions d'interfaces

### 3.3 Invariants immuables

**Principe EVOL-CN-03 : Invariants non nÃ©gociables**

Les invariants de Caring Nanny ne changent jamais. Toute Ã©volution doit les prÃ©server.

**Invariants protÃ©gÃ©s (non modifiables) :**
- **INV-CN-1** : Observateur pur (pas de modification d'Ã©tat)
- **INV-CN-2** : Aucune capacitÃ© d'exÃ©cution
- **INV-CN-3** : Non-autoritaire
- **INV-CN-4** : Ã‰tat cohÃ©rent
- **INV-CN-5** : TraÃ§abilitÃ© complÃ¨te
- **INV-CN-6** : Non-bloquant
- **INV-CN-7** : Propagation fidÃ¨le

**Implications :**
- Pas de modification des invariants fondamentaux
- Pas de compromis sur les invariants pour de nouvelles fonctionnalitÃ©s
- Toute Ã©volution est vÃ©rifiÃ©e contre les invariants

### 3.4 ConformitÃ© aux Lois d'Autonomie

**Principe EVOL-CN-04 : Autonomie prÃ©servÃ©e**

Toute Ã©volution de Caring Nanny doit maintenir la conformitÃ© aux Lois d'Autonomie SystÃ¨me.

**RÃ¨gles :**
- **LOI-1** : L'observation doit continuer Ã  fonctionner localement sans dÃ©pendance externe
- **LOI-2** : L'Ã©tat "offline" reste un Ã©tat normal, pas une erreur
- **LOI-3** : L'historique local reste souverain
- **LOI-4** : Pas de temps global requis pour les nouvelles fonctionnalitÃ©s
- **LOI-5** : Les nouvelles fonctionnalitÃ©s respectent la proportionnalitÃ© des ressources

---

## 4. SchÃ©ma de versionnement

### 4.1 Version sÃ©mantique

Caring Nanny utilise le versionnement sÃ©mantique (Semantic Versioning) : `MAJEUR.MINEUR.PATCH`

**Format :** `v<MAJEUR>.<MINEUR>.<PATCH>[-<PRE-RELEASE>][+<BUILD>]`

**Exemples :**
- `v1.0.0` : Version majeure initiale
- `v1.1.0` : Nouvelle fonctionnalitÃ© d'observation (compatible)
- `v1.1.1` : Correction de bug (compatible)
- `v2.0.0` : Breaking change
- `v2.0.0-alpha.1` : PrÃ©-version (alpha)
- `v2.0.0-beta.1` : PrÃ©-version (beta)
- `v2.0.0+20260127` : Build avec mÃ©tadonnÃ©es

### 4.2 RÃ¨gles d'incrÃ©mentation

#### 4.2.1 Version MAJEUR (X.0.0)

**IncrÃ©mentation quand :**
- Breaking change de l'interface d'observation
- Modification d'une garantie envers les consommateurs
- Changement de comportement de propagation
- Suppression d'une catÃ©gorie d'Ã©tat
- Modification du format de l'historique

**RÃ¨gle R-VER-CN-1 : Breaking change formel**

Tout breaking change nÃ©cessite :
1. Justification documentÃ©e
2. PÃ©riode de dÃ©prÃ©ciation (minimum 6 mois)
3. Plan de migration
4. Communication aux consommateurs d'Ã©tat

#### 4.2.2 Version MINEUR (x.Y.0)

**IncrÃ©mentation quand :**
- Nouvelle catÃ©gorie d'Ã©tat (additive, compatible)
- Nouvelle condition dÃ©tectable
- Nouveau type de notification
- Extension de l'interface d'observation (champs optionnels)
- Nouvelle garantie (sans modification des existantes)

**RÃ¨gle R-VER-CN-2 : CompatibilitÃ© ascendante**

Les versions mineures sont rÃ©tro-compatibles :
- Les consommateurs utilisant une version mineure antÃ©rieure continuent de fonctionner
- Les nouvelles fonctionnalitÃ©s d'observation sont optionnelles
- Les catÃ©gories d'Ã©tat existantes ne sont pas modifiÃ©es

#### 4.2.3 Version PATCH (x.y.Z)

**IncrÃ©mentation quand :**
- Correction de bug d'observation
- AmÃ©lioration de performance (sans changement d'interface)
- Correction de documentation
- Correction de formulation

**RÃ¨gle R-VER-CN-3 : Pas de changement d'interface**

Les versions patch ne modifient jamais l'interface d'observation publique.

---

## 5. Versionnement des composants conceptuels

### 5.1 Interface d'observation

**Composants conceptuels :** Observer, StateAggregator, TransitionDetector, Propagator, HistoryKeeper

**RÃ¨gles :**
- VersionnÃ©s ensemble avec Caring Nanny
- Breaking change = nouvelle version MAJEUR
- Extension = nouvelle version MINEUR
- Correction = nouvelle version PATCH

### 5.2 CatÃ©gories d'Ã©tat

**Ã‰tats actuels :** healthy, degraded, offline, syncing, error

**RÃ¨gle R-VER-CN-4 : Extension des catÃ©gories**

- Ajout d'une nouvelle catÃ©gorie : version MINEUR
- Modification de la sÃ©mantique d'une catÃ©gorie : version MAJEUR
- Suppression d'une catÃ©gorie : version MAJEUR (avec dÃ©prÃ©ciation prÃ©alable)

**Exemple :**
- `v1.0.0` : Ã‰tats de base (healthy, degraded, offline, syncing, error)
- `v1.1.0` : Ajout de `maintenance` (nouvel Ã©tat)
- `v2.0.0` : Modification de la sÃ©mantique de `error` (breaking)

### 5.3 Format de l'historique

**Composant :** Structure des observations enregistrÃ©es

**RÃ¨gles :**
- Ajout de champs optionnels : version MINEUR
- Modification de champs existants : version MAJEUR
- Changement de format de stockage : version MAJEUR

### 5.4 Contrats internes

**Documents :** Invariants & Garanties, State Model Contract, Observation Flow Contract, Propagation Flow Contract

**RÃ¨gle R-VER-CN-5 : Alignement des contrats**

Les documents contractuels sont versionnÃ©s et alignÃ©s avec les versions de Caring Nanny :
- Document `v1.0.0` correspond Ã  Caring Nanny `v1.0.0`
- Document `v2.0.0` correspond Ã  Caring Nanny `v2.0.0`

---

## 6. RÃ¨gles de compatibilitÃ©

### 6.1 CompatibilitÃ© ascendante

**RÃ¨gle R-COMP-CN-1 : RÃ©trocompatibilitÃ©**

Les versions mineures et patch sont rÃ©tro-compatibles :
- Un consommateur utilisant `v1.0.0` fonctionne avec Caring Nanny `v1.5.0`
- Un consommateur utilisant `v1.5.0` fonctionne avec Caring Nanny `v1.0.0` (sauf nouvelles fonctionnalitÃ©s)

**Garanties :**
- Les interfaces d'observation existantes ne sont pas modifiÃ©es
- Les catÃ©gories d'Ã©tat existantes sont prÃ©servÃ©es
- Les comportements de propagation existants sont prÃ©servÃ©s
- Les nouvelles fonctionnalitÃ©s sont optionnelles

### 6.2 CompatibilitÃ© descendante

**RÃ¨gle R-COMP-CN-2 : Pas de garantie descendante**

Caring Nanny ne garantit pas la compatibilitÃ© descendante :
- Un consommateur utilisant `v2.0.0` peut ne pas fonctionner avec Caring Nanny `v1.5.0`
- Les consommateurs doivent utiliser une version compatible de Caring Nanny

**Implications :**
- Les consommateurs doivent spÃ©cifier la version minimale requise
- Les breaking changes sont documentÃ©s
- Les migrations sont guidÃ©es

### 6.3 Coexistence de versions

**RÃ¨gle R-COMP-CN-3 : Support multi-version**

Caring Nanny peut supporter plusieurs versions d'interfaces simultanÃ©ment pendant les pÃ©riodes de transition.

**DurÃ©e de support :**
- Version N : SupportÃ©e (actuelle)
- Version N-1 : SupportÃ©e (minimum 12 mois aprÃ¨s version N)
- Version N-2 : DÃ©prÃ©ciÃ©e (support limitÃ©)
- Version N-3 : Non supportÃ©e

---

## 7. Ã‰volution des invariants

### 7.1 RÃ¨gles d'Ã©volution des invariants

**RÃ¨gle R-EVOL-INV-CN-1 : Ajout d'invariant**

Un nouvel invariant peut Ãªtre ajoutÃ© dans une version MINEUR s'il :
- N'affaiblit aucun invariant existant
- N'introduit pas d'incompatibilitÃ© pour les consommateurs
- Est documentÃ© et justifiÃ©
- PrÃ©serve la nature d'observateur pur

**RÃ¨gle R-EVOL-INV-CN-2 : Modification d'invariant**

Un invariant existant ne peut Ãªtre modifiÃ© que dans une version MAJEUR avec :
- Justification de la modification
- Guide de migration
- PÃ©riode de dÃ©prÃ©ciation si applicable

**RÃ¨gle R-EVOL-INV-CN-3 : Suppression d'invariant**

Un invariant existant ne peut Ãªtre supprimÃ© que dans une version MAJEUR aprÃ¨s :
- DÃ©prÃ©ciation dans au moins deux versions MINEUR
- Justification de la suppression
- Guide de migration

**RÃ¨gle R-EVOL-INV-CN-4 : Invariants fondamentaux**

Les invariants fondamentaux (INV-CN-1 Ã  INV-CN-7) ne peuvent **jamais** Ãªtre modifiÃ©s ou supprimÃ©s. Ils dÃ©finissent l'essence de Caring Nanny.

### 7.2 Garanties d'Ã©volution des invariants

**G-EVOL-INV-CN-1 : CompatibilitÃ© prÃ©servÃ©e**

L'ajout d'un invariant ne peut pas rendre non conforme une implÃ©mentation conforme.

**G-EVOL-INV-CN-2 : DÃ©prÃ©ciation avant suppression**

Tout invariant supprimÃ© doit avoir Ã©tÃ© dÃ©prÃ©ciÃ© au prÃ©alable (sauf invariants fondamentaux qui ne peuvent Ãªtre supprimÃ©s).

---

## 8. Ã‰volution des garanties

### 8.1 RÃ¨gles d'Ã©volution des garanties

**RÃ¨gle R-EVOL-GAR-CN-1 : Ajout de garantie**

Une nouvelle garantie peut Ãªtre ajoutÃ©e dans une version MINEUR si elle :
- N'affaiblit aucune garantie existante
- N'introduit pas d'incompatibilitÃ©
- Est documentÃ©e et justifiÃ©e

**RÃ¨gle R-EVOL-GAR-CN-2 : Modification de garantie**

Une garantie existante ne peut Ãªtre modifiÃ©e que dans une version MAJEUR avec :
- Justification de la modification
- Guide de migration
- PÃ©riode de dÃ©prÃ©ciation si applicable

**RÃ¨gle R-EVOL-GAR-CN-3 : Suppression de garantie**

Une garantie existante ne peut Ãªtre supprimÃ©e que dans une version MAJEUR aprÃ¨s :
- DÃ©prÃ©ciation dans au moins deux versions MINEUR
- Justification de la suppression
- Guide de migration

**RÃ¨gle R-EVOL-GAR-CN-4 : Garanties fondamentales**

Les garanties envers les autoritÃ©s (GAR-AUTH-*) et les garanties de cohÃ©rence (GAR-CONS-01 Ã  GAR-CONS-04) ne peuvent jamais Ãªtre modifiÃ©es ou supprimÃ©es.

### 8.2 Garanties d'Ã©volution des garanties

**G-EVOL-GAR-CN-1 : CompatibilitÃ© prÃ©servÃ©e**

L'ajout d'une garantie ne peut pas rendre non conforme une implÃ©mentation conforme.

**G-EVOL-GAR-CN-2 : DÃ©prÃ©ciation avant suppression**

Toute garantie supprimÃ©e doit avoir Ã©tÃ© dÃ©prÃ©ciÃ©e au prÃ©alable (sauf garanties fondamentales qui ne peuvent Ãªtre supprimÃ©es).

---

## 9. DÃ©prÃ©ciation

### 9.1 Marquage de dÃ©prÃ©ciation

**RÃ¨gle R-DEPR-CN-1 : Marquage explicite**

Tout Ã©lÃ©ment dÃ©prÃ©ciÃ© est marquÃ© explicitement avec :
- Le statut DÃ‰PRÃ‰CIÃ‰
- La version de dÃ©prÃ©ciation
- La version de suppression prÃ©vue
- La raison de la dÃ©prÃ©ciation
- Les instructions de migration

**Format de marquage :**
```
@deprecated Depuis v1.5.0, sera supprimÃ© en v2.0.0. 
Raison : [justification]
Migration : [instructions]
```

### 9.2 PÃ©riode de dÃ©prÃ©ciation

**RÃ¨gle R-DEPR-CN-2 : Minimum 6 mois**

La pÃ©riode de dÃ©prÃ©ciation est d'au minimum 6 mois, recommandÃ© 12 mois pour les Ã©lÃ©ments largement utilisÃ©s.

**Calcul :**
- Date de dÃ©prÃ©ciation : Date de publication de la version avec marquage DÃ‰PRÃ‰CIÃ‰
- Date de suppression : Date de publication de la version MAJEUR suivante
- PÃ©riode : Minimum 6 mois entre les deux

### 9.3 Communication de dÃ©prÃ©ciation

**RÃ¨gle R-DEPR-CN-3 : Communication proactive**

La dÃ©prÃ©ciation est communiquÃ©e :
- Dans les notes de version
- Dans la documentation
- Dans les logs d'observation (si applicable)

**Contenu :**
- Ce qui est dÃ©prÃ©ciÃ©
- Pourquoi c'est dÃ©prÃ©ciÃ©
- Quand ce sera supprimÃ©
- Comment migrer

### 9.4 Cas interdits de dÃ©prÃ©ciation

**RÃ¨gle R-DEPR-CN-4 : Ã‰lÃ©ments non dÃ©prÃ©ciables**

Les Ã©lÃ©ments suivants ne peuvent jamais Ãªtre dÃ©prÃ©ciÃ©s :
- Invariants fondamentaux (INV-CN-1 Ã  INV-CN-7)
- Garanties fondamentales (GAR-AUTH-*, GAR-CONS-01 Ã  GAR-CONS-04)
- Nature d'observateur pur
- Distinction avec les autoritÃ©s (KindMother, StrongFather, BondingBrother)

---

## 10. Migration conceptuelle

### 10.1 Types de migrations

**MIG-TYPE-CN-1 : Migration automatique**

Une migration est **automatique** si elle ne nÃ©cessite aucune modification pour les consommateurs d'Ã©tat.

**Exemple :** Ajout d'une nouvelle catÃ©gorie d'Ã©tat optionnelle.

**MIG-TYPE-CN-2 : Migration guidÃ©e**

Une migration est **guidÃ©e** si elle nÃ©cessite des modifications documentÃ©es.

**Exemple :** Changement de format d'une notification de changement d'Ã©tat.

**MIG-TYPE-CN-3 : Migration majeure**

Une migration est **majeure** si elle nÃ©cessite une adaptation significative des consommateurs.

**Exemple :** Passage d'une version MAJEUR avec modifications de l'interface d'observation.

### 10.2 Processus de migration

**Phase 1 : Analyse**
1. Identification des changements incompatibles
2. Ã‰valuation de l'impact sur les consommateurs d'Ã©tat
3. DÃ©finition du plan de migration

**Phase 2 : Documentation**
1. RÃ©daction du guide de migration
2. Documentation des changements
3. CrÃ©ation des tests de migration

**Phase 3 : ImplÃ©mentation**
1. Adaptation de l'implÃ©mentation
2. ExÃ©cution des tests de migration
3. VÃ©rification de la conformitÃ©

**Phase 4 : Validation**
1. Tests de conformitÃ© aux invariants
2. Validation de la migration
3. Certification de conformitÃ©

### 10.3 Garanties de migration

**G-MIG-CN-1 : Guide disponible**

Un guide de migration est toujours disponible pour toute version MAJEUR.

**G-MIG-CN-2 : Migration testable**

Toute migration peut Ãªtre vÃ©rifiÃ©e par des tests de conformitÃ©.

**G-MIG-CN-3 : Support de transition**

Un support de transition est fourni pendant la pÃ©riode de migration (minimum 12 mois).

---

## 11. RÃ¨gles de gel

### 11.1 DÃ©finition du gel

**DÃ©finition :**

Le **gel** est l'Ã©tat d'un contrat Caring Nanny oÃ¹ aucune modification n'est autorisÃ©e, garantissant la stabilitÃ© absolue du contrat.

**CaractÃ©ristiques :**
- **ImmutabilitÃ©** : Un contrat gelÃ© ne peut plus Ãªtre modifiÃ©
- **StabilitÃ©** : Un contrat gelÃ© garantit la stabilitÃ© contractuelle
- **IrrÃ©versibilitÃ©** : Un gel ne peut pas Ãªtre annulÃ©
- **Permanence** : Un contrat gelÃ© reste gelÃ© dÃ©finitivement

### 11.2 Conditions de gel

**RÃ¨gle R-GEL-CN-1 : Gel aprÃ¨s stabilisation**

Un contrat peut Ãªtre gelÃ© aprÃ¨s une pÃ©riode de stabilisation et de validation complÃ¨te.

**RÃ¨gle R-GEL-CN-2 : Gel par dÃ©cision formelle**

Le gel d'un contrat est une dÃ©cision architecturale formelle, documentÃ©e et irrÃ©versible.

**RÃ¨gle R-GEL-CN-3 : Gel des contrats fondateurs**

Les contrats fondateurs (Documentation Fondatrice, Invariants & Garanties) peuvent Ãªtre gelÃ©s aprÃ¨s validation complÃ¨te.

### 11.3 RÃ¨gles de gel

**RÃ¨gle R-GEL-CN-4 : Aucune modification autorisÃ©e**

Un contrat gelÃ© ne peut plus Ãªtre modifiÃ©, mÃªme pour des corrections mineures.

**RÃ¨gle R-GEL-CN-5 : Nouvelle version pour Ã©volution**

Toute Ã©volution d'un contrat gelÃ© nÃ©cessite la crÃ©ation d'un nouveau contrat ou d'une nouvelle version MAJEUR.

**RÃ¨gle R-GEL-CN-6 : Documentation du gel**

Le gel d'un contrat DOIT Ãªtre documentÃ© avec :
- La date de gel
- La version gelÃ©e
- La raison du gel
- Les implications du gel

### 11.4 Garanties de gel

**G-GEL-CN-1 : ImmutabilitÃ© garantie**

Un contrat gelÃ© ne peut jamais Ãªtre modifiÃ©.

**G-GEL-CN-2 : StabilitÃ© garantie**

Un contrat gelÃ© garantit la stabilitÃ© contractuelle absolue.

**G-GEL-CN-3 : CompatibilitÃ© prÃ©servÃ©e**

Un contrat gelÃ© reste compatible avec toutes les implÃ©mentations conformes.

---

## 12. Processus d'Ã©volution

### 12.1 Proposition d'Ã©volution

**Ã‰tape 1 : Proposition**
- Description de l'Ã©volution
- Justification (besoin, bÃ©nÃ©fice)
- Impact (consommateurs, autoritÃ©s, invariants)
- Plan de migration (si breaking change)

**Ã‰tape 2 : VÃ©rification**
- VÃ©rification contre les invariants (INV-CN-1 Ã  INV-CN-7)
- VÃ©rification de conformitÃ© aux Lois d'Autonomie
- VÃ©rification de compatibilitÃ©
- Validation architecturale

### 12.2 ImplÃ©mentation

**Ã‰tape 3 : ImplÃ©mentation**
- DÃ©veloppement selon les rÃ¨gles d'Ã©volution
- Tests de compatibilitÃ©
- Tests de rÃ©gression
- Documentation

**Ã‰tape 4 : DÃ©prÃ©ciation (si nÃ©cessaire)**
- Marquage DÃ‰PRÃ‰CIÃ‰
- Communication aux consommateurs
- PÃ©riode de dÃ©prÃ©ciation

### 12.3 Publication

**Ã‰tape 5 : Release**
- Versionnement selon les rÃ¨gles
- Notes de version
- Documentation de migration
- Communication

**Ã‰tape 6 : Support**
- Support de la nouvelle version
- Support des versions prÃ©cÃ©dentes (selon politique)
- Monitoring des migrations

---

## 13. Gestion des breaking changes

### 13.1 Types de breaking changes

**Breaking change d'interface :**
- Modification du format d'interrogation
- Suppression d'une mÃ©thode d'observation
- Modification de la structure des rÃ©ponses

**Breaking change de contrat :**
- Modification d'une garantie
- Changement de format de l'historique
- Modification de la sÃ©mantique d'une catÃ©gorie d'Ã©tat

**Breaking change de comportement :**
- Modification du flux d'observation
- Modification du flux de propagation
- Changement de rÃ¨gles d'agrÃ©gation

### 13.2 Processus de breaking change

**RÃ¨gle R-BREAK-CN-1 : Processus formel**

Tout breaking change suit un processus formel :

1. **Justification :** Pourquoi ce breaking change est nÃ©cessaire
2. **Impact analysis :** Quels consommateurs sont affectÃ©s
3. **VÃ©rification invariants :** ConformitÃ© aux invariants fondamentaux
4. **Plan de migration :** Comment migrer
5. **PÃ©riode de dÃ©prÃ©ciation :** Minimum 6 mois
6. **Communication :** Annonce, documentation, support
7. **Version MAJEUR :** IncrÃ©mentation obligatoire

### 13.3 Exceptions

**Exception BREAK-CN-EXCEPT-1 : SÃ©curitÃ© critique**

En cas de vulnÃ©rabilitÃ© de sÃ©curitÃ© critique, un breaking change peut Ãªtre appliquÃ© immÃ©diatement avec version MAJEUR, mais avec communication urgente et support de migration.

**Exception BREAK-CN-EXCEPT-2 : Correction d'invariant violÃ©**

Si un invariant est violÃ© par erreur dans une version prÃ©cÃ©dente, la correction (qui peut Ãªtre un breaking change) est appliquÃ©e avec version MAJEUR et communication.

---

## 14. Exemples

### 14.1 Ã‰volution mineure (v1.0.0 â†’ v1.1.0)

**Changement :** Ajout d'une nouvelle catÃ©gorie d'Ã©tat `maintenance`

**Impact :** Aucun (additif)

**CompatibilitÃ© :** RÃ©tro-compatible

**Migration :** Aucune nÃ©cessaire (les consommateurs qui ne connaissent pas `maintenance` continuent de fonctionner)

### 14.2 Ã‰volution majeure (v1.5.0 â†’ v2.0.0)

**Changement :** Modification du format de notification de changement d'Ã©tat

**Processus :**
1. v1.5.0 : Ancien format marquÃ© `@deprecated`, nouveau format ajoutÃ©
2. v1.6.0 - v1.9.0 : PÃ©riode de dÃ©prÃ©ciation (12 mois)
3. v2.0.0 : Ancien format supprimÃ©

**Impact :** Consommateurs utilisant l'ancien format doivent migrer

**Migration :** Guide de migration fourni

### 14.3 Ã‰volution patch (v1.5.0 â†’ v1.5.1)

**Changement :** Correction d'un bug dans la dÃ©tection de transition

**Impact :** Aucun (correction)

**CompatibilitÃ© :** RÃ©tro-compatible

**Migration :** Aucune nÃ©cessaire

---

## 15. RÃ¨gles de fermeture du contrat

### 15.1 Contrat fermÃ©

Ce contrat est **fermÃ©**. Seules les rÃ¨gles de versioning, compatibilitÃ©, dÃ©prÃ©ciation, migration, et gel explicitement dÃ©finies sont valides.

### 15.2 Interdiction d'extension implicite

Aucune extension implicite des rÃ¨gles d'Ã©volution n'est autorisÃ©e. Toute nouvelle rÃ¨gle doit Ãªtre ajoutÃ©e explicitement via une nouvelle version du contrat.

---

## 16. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit les rÃ¨gles de versionnement et d'Ã©volution de Caring Nanny qui doivent Ãªtre respectÃ©es pour garantir la stabilitÃ© et l'Ã©volutivitÃ©.

Toute Ã©volution de Caring Nanny doit respecter ces rÃ¨gles. Toute violation doit Ãªtre corrigÃ©e ou justifiÃ©e par une exception documentÃ©e.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :**
- [Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) v1.6 (Section 7)
- [Invariants et Garanties](../governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md) v1.0
- [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md)

