# Ever Buddy - Version Semantics Contract

## Contexte

Ce document dÃ©finit le **contrat de sÃ©mantique de versionnement** gouvernÃ© par Ever Buddy. Le versionnement conceptuel est la maniÃ¨re dont Ever Buddy identifie et distingue les diffÃ©rentes versions d'un Ã©lÃ©ment du systÃ¨me Miyukini.

Le versionnement sÃ©mantique est au cÅ“ur de la capacitÃ© du systÃ¨me Ã  Ã©voluer sans rupture. Il permet aux consommateurs d'anticiper l'impact d'une mise Ã  jour et de planifier leurs propres Ã©volutions en consÃ©quence.

**Document source :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

---

## PortÃ©e / Scope

- **Applicable Ã  :** Tous les Ã©lÃ©ments versionnÃ©s du systÃ¨me Miyukini (contrats, interfaces, structures, capacitÃ©s)
- **Audience :** Architectes, dÃ©veloppeurs, cores consommateurs, produits
- **Statut :** Document contractuel normatif â€” VERSION SEMANTICS CONTRACT
- **AutoritÃ© :** Ever Buddy (Strate 4) â€” Core de cycle de vie et d'Ã©volution

---

## 1. Principes fondamentaux du versionnement

### 1.1 Versionnement sÃ©mantique et significatif

Le versionnement conceptuel d'Ever Buddy est **sÃ©mantique et significatif**. Chaque composant du numÃ©ro de version communique une information claire sur la nature du changement :

| Composant | Signification | Impact pour les consommateurs |
|-----------|---------------|-------------------------------|
| **Majeur** | Changement incompatible, rupture de contrat | Migration obligatoire |
| **Mineur** | Ajout de fonctionnalitÃ©, rÃ©trocompatible | Adoption optionnelle |
| **Correctif** | Correction de bug, aucun changement fonctionnel | Adoption recommandÃ©e |

### 1.2 Format de version

Le format canonique de version est :

```
MAJEUR.MINEUR.CORRECTIF
```

**Exemples :**
- `1.0.0` â€” PremiÃ¨re version stable
- `1.2.0` â€” Ajout de fonctionnalitÃ©s depuis la v1.0.0
- `1.2.3` â€” Trois corrections de bugs depuis la v1.2.0
- `2.0.0` â€” Rupture de compatibilitÃ© par rapport Ã  la v1.x

### 1.3 RÃ¨gle de non-rÃ©gression numÃ©rique

Les numÃ©ros de version ne peuvent que croÃ®tre. Une fois qu'une version est publiÃ©e, elle ne peut Ãªtre ni retirÃ©e, ni renumÃ©rotÃ©e. Cette rÃ¨gle garantit la traÃ§abilitÃ© et la confiance des consommateurs.

---

## 2. Types de changements et versionnement

### 2.1 Changement majeur (MAJEUR)

Un **changement majeur** est toute modification qui rompt la compatibilitÃ© avec les versions prÃ©cÃ©dentes.

**Exemples de changements majeurs :**
- Suppression d'une capacitÃ© existante
- Modification d'une signature de contrat
- Changement de comportement d'une fonctionnalitÃ© existante
- Restructuration incompatible des donnÃ©es
- Suppression d'un Ã©tat de cycle de vie
- Modification des rÃ¨gles de transition

**ConsÃ©quences :**
- IncrÃ©mentation du numÃ©ro majeur
- Remise Ã  zÃ©ro des numÃ©ros mineur et correctif
- PÃ©riode de dÃ©prÃ©ciation obligatoire de l'ancienne version (INV-EB-4)
- Documentation obligatoire du chemin de migration (INV-EB-7)

**RÃ©fÃ©rence Glossaire :** [Breaking change](..//..//..//..//miyukini-webway-system//reference//_index.md#breaking-change-changement-de-rupture)

### 2.2 Changement mineur (MINEUR)

Un **changement mineur** est un ajout de fonctionnalitÃ© qui prÃ©serve la rÃ©trocompatibilitÃ©.

**Exemples de changements mineurs :**
- Ajout d'une nouvelle capacitÃ©
- Ajout d'un nouveau champ optionnel
- Extension d'une interface sans modification des mÃ©thodes existantes
- Ajout d'un nouvel Ã©tat de vie (avec transitions dÃ©finies)
- Nouvelles rÃ¨gles d'Ã©volution qui n'affectent pas l'existant

**ConsÃ©quences :**
- IncrÃ©mentation du numÃ©ro mineur
- Remise Ã  zÃ©ro du numÃ©ro correctif
- Les consommateurs existants continuent de fonctionner sans modification
- Adoption optionnelle des nouvelles fonctionnalitÃ©s

### 2.3 Changement correctif (CORRECTIF)

Un **changement correctif** est une correction de bug qui ne modifie pas le comportement fonctionnel attendu.

**Exemples de changements correctifs :**
- Correction d'une erreur de calcul
- Correction d'une fuite de mÃ©moire
- Correction d'une condition de concurrence
- AmÃ©lioration de performance sans changement fonctionnel
- Correction de documentation erronÃ©e

**ConsÃ©quences :**
- IncrÃ©mentation du numÃ©ro correctif uniquement
- Aucun impact sur les consommateurs
- Adoption recommandÃ©e pour bÃ©nÃ©ficier des corrections

---

## 3. Versionnement par catÃ©gorie d'Ã©lÃ©ment

### 3.1 Contrats fondateurs (FONDATION)

Les contrats de statut **FONDATION** ont des rÃ¨gles de versionnement particuliÃ¨rement strictes :

| Aspect | RÃ¨gle |
|--------|-------|
| **Changements majeurs** | ExtrÃªmement rares, nÃ©cessitent approbation multi-niveaux |
| **PÃ©riode de dÃ©prÃ©ciation** | Minimale : 3 gÃ©nÃ©rations de versions |
| **Ruptures** | Quasiment interdites, uniquement en dernier recours |
| **FrÃ©quence de changement** | Ã‰volution trÃ¨s lente (annÃ©es) |

**Invariant applicable :** INV-EB-6 (Vision long terme obligatoire)

### 3.2 Contrats opÃ©rationnels

Les contrats opÃ©rationnels suivent des rÃ¨gles de versionnement standards :

| Aspect | RÃ¨gle |
|--------|-------|
| **Changements majeurs** | Possibles avec justification |
| **PÃ©riode de dÃ©prÃ©ciation** | Minimale : 1 gÃ©nÃ©ration de versions |
| **Ruptures** | AutorisÃ©es avec plan de migration |
| **FrÃ©quence de changement** | Ã‰volution modÃ©rÃ©e (mois Ã  annÃ©es) |

### 3.3 Interfaces techniques

Les interfaces techniques ont des rÃ¨gles de versionnement plus souples :

| Aspect | RÃ¨gle |
|--------|-------|
| **Changements majeurs** | Relativement courants |
| **PÃ©riode de dÃ©prÃ©ciation** | Minimale : 2 cycles de release |
| **Ruptures** | Possibles avec documentation |
| **FrÃ©quence de changement** | Ã‰volution plus rapide (semaines Ã  mois) |

### 3.4 Ã‰lÃ©ments internes

Les Ã©lÃ©ments internes n'ont pas de garantie de stabilitÃ© externe :

| Aspect | RÃ¨gle |
|--------|-------|
| **Changements majeurs** | Libres sans contrainte externe |
| **PÃ©riode de dÃ©prÃ©ciation** | Aucune obligation |
| **Ruptures** | AutorisÃ©es sans prÃ©avis |
| **FrÃ©quence de changement** | Ã‰volution libre |

**ATTENTION :** Les Ã©lÃ©ments internes ne doivent jamais Ãªtre exposÃ©s aux consommateurs externes. Tout Ã©lÃ©ment exposÃ© devient un contrat de facto.

---

## 4. RÃ¨gles de compatibilitÃ© par type de version

### 4.1 RÃ©trocompatibilitÃ© (comportement par dÃ©faut)

ConformÃ©ment Ã  l'invariant **INV-EB-5**, toute Ã©volution est **prÃ©sumÃ©e rÃ©trocompatible** sauf dÃ©claration explicite contraire.

| Type de version | RÃ©trocompatibilitÃ© garantie |
|-----------------|----------------------------|
| Majeur (n.0.0) | âŒ Non |
| Mineur (x.n.0) | âœ… Oui |
| Correctif (x.y.n) | âœ… Oui |

**RÃ©fÃ©rence Glossaire :** [RÃ©trocompatible](..//..//..//..//miyukini-webway-system//reference//_index.md#retrocompatible)

### 4.2 CompatibilitÃ© en amont

La compatibilitÃ© en amont (anciennes versions fonctionnant avec les nouvelles) est :

| Type de version | CompatibilitÃ© amont |
|-----------------|---------------------|
| Majeur | âŒ Non garantie |
| Mineur | âš ï¸ Partielle possible |
| Correctif | âœ… GÃ©nÃ©ralement oui |

**Note :** La compatibilitÃ© en amont est rarement garantie et souvent techniquement impossible. Elle ne doit pas Ãªtre une attente par dÃ©faut.

### 4.3 FenÃªtre de compatibilitÃ©

Chaque Ã©lÃ©ment dÃ©finit une **fenÃªtre de compatibilitÃ©** qui spÃ©cifie les versions avec lesquelles il garantit l'interopÃ©rabilitÃ©.

**Format :**
```
Compatible avec : vX.Y Ã  vX.Z
```

**Exemples :**
- `Compatible avec : v1.0 Ã  v1.9` â€” Supporte toutes les versions 1.x
- `Compatible avec : v2.3 Ã  v2.5` â€” FenÃªtre restreinte

**RÃ©fÃ©rence Glossaire :** [Compatibility window](..//..//..//..//miyukini-webway-system//reference//_index.md#compatibility-window)

---

## 5. Cycle de vie des versions

### 5.1 Ã‰tats de version

Chaque version possÃ¨de un Ã©tat de cycle de vie gouvernÃ© par Ever Buddy :

| Ã‰tat | Description | Versionnement |
|------|-------------|---------------|
| **DRAFT** | Version en dÃ©veloppement | Version provisoire (0.x.x ou suffixe -draft) |
| **ACTIVE** | Version en usage normal | Version stable (x.y.z) |
| **DEPRECATED** | Version dÃ©couragÃ©e | Conserve son numÃ©ro + marqueur dÃ©prÃ©ciation |
| **RETIRED** | Version retirÃ©e | NumÃ©ro figÃ©, plus de correctifs |
| **ARCHIVED** | Version archivÃ©e | NumÃ©ro conservÃ© pour rÃ©fÃ©rence |

### 5.2 Versions DRAFT (0.x.x)

Les versions dont le numÃ©ro majeur est **0** (zÃ©ro) sont considÃ©rÃ©es comme instables :

| RÃ¨gle | Description |
|-------|-------------|
| **StabilitÃ©** | Aucune garantie de stabilitÃ© |
| **Changements** | Tout changement possible sans incrÃ©mentation majeure |
| **Consommateurs** | Usage en connaissance de cause uniquement |
| **Transition** | Passage Ã  1.0.0 = premiÃ¨re version stable |

**Exemple de progression :**
```
0.1.0 â†’ 0.2.0 â†’ 0.9.0 â†’ 1.0.0 (premiÃ¨re version stable)
```

### 5.3 Succession de versions

La **chaÃ®ne d'Ã©volution** trace toutes les versions d'un Ã©lÃ©ment :

```
v1.0.0 â†’ v1.1.0 â†’ v1.2.0 â†’ v2.0.0 â†’ v2.1.0 â†’ ...
         â””â†’ v1.1.1 (correctif)
```

**RÃ¨gles :**
- Une version peut avoir plusieurs successeurs (branches)
- Une version a un seul prÃ©dÃ©cesseur direct
- Les correctifs se branchent depuis leur version mineure parente

**RÃ©fÃ©rence Glossaire :** [Evolution chain](..//..//..//..//miyukini-webway-system//reference//_index.md#evolution-chain)

---

## 6. DÃ©claration des changements

### 6.1 Changelog obligatoire

Toute nouvelle version doit Ãªtre accompagnÃ©e d'un **changelog** documentant :

| Section | Contenu |
|---------|---------|
| **Added** | Nouvelles fonctionnalitÃ©s |
| **Changed** | Modifications de comportement |
| **Deprecated** | Ã‰lÃ©ments dÃ©prÃ©ciÃ©s |
| **Removed** | Ã‰lÃ©ments supprimÃ©s |
| **Fixed** | Corrections de bugs |
| **Security** | Corrections de sÃ©curitÃ© |

### 6.2 Classification de l'impact

Chaque changement doit Ãªtre classifiÃ© selon son impact :

| Impact | Description | Type de version |
|--------|-------------|-----------------|
| **Breaking** | Rompt la compatibilitÃ© | Majeur |
| **Feature** | Ajoute une fonctionnalitÃ© | Mineur |
| **Fix** | Corrige un bug | Correctif |
| **None** | Aucun impact fonctionnel | Correctif ou aucun |

### 6.3 DÃ©claration des ruptures

ConformÃ©ment Ã  l'invariant **INV-EB-5**, si une Ã©volution est incompatible, elle doit Ãªtre **explicitement dÃ©clarÃ©e** :

```markdown
## Version 2.0.0

### âš ï¸ BREAKING CHANGES

- **[BREAKING]** Suppression de la mÃ©thode `legacyMethod()`
  - Migration : Utiliser `newMethod()` Ã  la place
  - Raison : Obsolescence et maintenance coÃ»teuse

- **[BREAKING]** Modification du format de `DataStructure`
  - Migration : Voir guide de migration section 3.2
  - Raison : Performance et cohÃ©rence
```

---

## 7. Versionnement et transitions d'Ã©tat

### 7.1 Impact des transitions sur le versionnement

| Transition | Impact sur la version |
|------------|----------------------|
| DRAFT â†’ ACTIVE | Passage Ã  version stable (1.0.0 si premiÃ¨re) |
| ACTIVE â†’ DEPRECATED | Aucun changement de numÃ©ro |
| DEPRECATED â†’ RETIRED | Aucun changement de numÃ©ro |
| RETIRED â†’ ARCHIVED | Aucun changement de numÃ©ro |
| DEPRECATED â†’ ACTIVE (rÃ©activation) | Aucun changement de numÃ©ro |

### 7.2 Correctifs sur versions dÃ©prÃ©ciÃ©es

Les versions **DEPRECATED** peuvent recevoir des correctifs de sÃ©curitÃ© critiques :

| Type de correctif | AutorisÃ© sur DEPRECATED |
|-------------------|------------------------|
| Correction de sÃ©curitÃ© | âœ… Oui |
| Correction de bug critique | âœ… Oui |
| Correction de bug mineur | âŒ Non |
| Nouvelle fonctionnalitÃ© | âŒ Non |

Les versions **RETIRED** ne reÃ§oivent aucun correctif.

---

## 8. Invariants de versionnement

### 8.1 Invariants applicables

Les invariants suivants d'Ever Buddy s'appliquent au versionnement :

| Invariant | Application au versionnement |
|-----------|------------------------------|
| **INV-EB-4** | Pas de passage direct ACTIVE â†’ RETIRED sans version DEPRECATED |
| **INV-EB-5** | RÃ©trocompatibilitÃ© prÃ©sumÃ©e sauf dÃ©claration majeure explicite |
| **INV-EB-6** | Impact sur au moins deux gÃ©nÃ©rations de versions considÃ©rÃ© |
| **INV-EB-7** | Chaque version documentÃ©e avec raison et chemin de migration |
| **INV-EB-9** | RÃ¨gles de versionnement publiques et stables |
| **INV-EB-11** | Changements de rÃ¨gles non rÃ©troactifs |

### 8.2 Violations de versionnement

Les actions suivantes sont des **violations** du contrat de versionnement :

| Violation | GravitÃ© |
|-----------|---------|
| Changement majeur sans incrÃ©ment majeur | ðŸ”´ Critique |
| Rupture de compatibilitÃ© non documentÃ©e | ðŸ”´ Critique |
| RenumÃ©rotation d'une version publiÃ©e | ðŸ”´ Critique |
| Correctif incluant une nouvelle fonctionnalitÃ© | ðŸŸ¡ ModÃ©rÃ©e |
| Version mineure sans rÃ©trocompatibilitÃ© | ðŸ”´ Critique |
| Absence de changelog | ðŸŸ¡ ModÃ©rÃ©e |

---

## 9. Comparaison et ordering des versions

### 9.1 RÃ¨gles de comparaison

Les versions sont comparÃ©es composant par composant, de gauche Ã  droite :

```
1.9.0 < 1.10.0 < 2.0.0
1.0.0 < 1.0.1 < 1.1.0
```

### 9.2 PrÃ©cÃ©dence

```
MAJEUR > MINEUR > CORRECTIF
```

**Exemple de tri :**
```
1.0.0 â†’ 1.0.1 â†’ 1.1.0 â†’ 1.1.1 â†’ 1.2.0 â†’ 2.0.0
```

### 9.3 Versions prÃ©-release (DRAFT)

Les versions DRAFT (0.x.x) sont toujours infÃ©rieures Ã  leur premiÃ¨re version stable :

```
0.9.9 < 1.0.0
```

---

## 10. Interactions avec les autres cores

### 10.1 Consultation par StrongFather

StrongFather peut consulter Ever Buddy pour connaÃ®tre :
- La version actuelle d'un Ã©lÃ©ment
- La compatibilitÃ© entre deux versions
- Les rÃ¨gles de migration applicables

### 10.2 Guidance pour BondingBrother

BondingBrother utilise les informations de versionnement pour :
- Adapter les traductions entre versions diffÃ©rentes
- Communiquer les avertissements de compatibilitÃ©
- Guider les produits dans leurs migrations

### 10.3 Information vers Master Butler

Master Butler est informÃ© par Ever Buddy de :
- L'Ã©tat de vie de chaque capacitÃ© versionnÃ©e
- Les versions supportÃ©es de chaque Tool
- Les compatibilitÃ©s Tool â†” Environnement

---

## 11. ConformitÃ© aux Lois d'Autonomie

### LOI-4 : Pas de temps global requis

Le versionnement sÃ©mantique respecte LOI-4 :
- Les versions sont des **numÃ©ros discrets**, pas des timestamps
- Les comparaisons de versions entre nÅ“uds utilisent des **numÃ©ros**, pas des dates
- Les pÃ©riodes de dÃ©prÃ©ciation sont dÃ©finies en **cycles de release**, pas en temps absolu

### LOI-3 : L'Ã©tat local est souverain

Chaque nÅ“ud maintient son propre registre de versions :
- La version locale est la vÃ©ritÃ© locale
- Ã€ la reconnexion, rÃ©conciliation explicite des versions

---

## 12. RÃ©sumÃ© des rÃ¨gles de versionnement

| RÃ¨gle | Description |
|-------|-------------|
| **R-VS-1** | Format : MAJEUR.MINEUR.CORRECTIF |
| **R-VS-2** | Majeur = rupture de compatibilitÃ© |
| **R-VS-3** | Mineur = ajout rÃ©trocompatible |
| **R-VS-4** | Correctif = bug fix uniquement |
| **R-VS-5** | NumÃ©ros croissants uniquement |
| **R-VS-6** | Version 0.x.x = instable |
| **R-VS-7** | Changelog obligatoire |
| **R-VS-8** | Ruptures explicitement dÃ©clarÃ©es |
| **R-VS-9** | CatÃ©gorie dÃ©termine les pÃ©riodes de dÃ©prÃ©ciation |
| **R-VS-10** | RÃ©trocompatibilitÃ© prÃ©sumÃ©e par dÃ©faut |

---

## 13. RÃ©fÃ©rences croisÃ©es

- [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) â€” Document source de ce contrat
- [Ever Buddy - Compatibility Rules Contract](./Ever%20Buddy%20-%20Compatibility%20Rules%20Contract.md) â€” RÃ¨gles de compatibilitÃ© dÃ©taillÃ©es
- [Ever Buddy - Lifecycle States Contract](../lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md) â€” Ã‰tats de cycle de vie
- [Ever Buddy - Transition Rules Contract](../lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md) â€” RÃ¨gles de transition
- [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) â€” DÃ©finitions canoniques
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) â€” ConformitÃ© LOI-1 Ã  LOI-6

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contractuel normatif â€” VERSION SEMANTICS CONTRACT  
**AutoritÃ© :** Ever Buddy (Strate 4)  
**Source :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md), Section 4 â€” Concepts fondamentaux

