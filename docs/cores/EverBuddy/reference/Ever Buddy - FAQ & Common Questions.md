# Ever Buddy - FAQ & Common Questions

## Contexte

Ce document rÃ©pond aux **questions frÃ©quemment posÃ©es** concernant Ever Buddy, le core de cycle de vie et d'Ã©volution du Miyukini Core System. Il synthÃ©tise les interrogations courantes des architectes, dÃ©veloppeurs, et intÃ©grateurs qui travaillent avec Ever Buddy.

Les rÃ©ponses sont dÃ©rivÃ©es de la **Documentation Fondatrice** et des contrats normatifs d'Ever Buddy. Ce document ne crÃ©e pas de nouvelles rÃ¨gles â€” il clarifie l'existant.

**Document de rÃ©fÃ©rence :** [Ever Buddy - Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

---

## PortÃ©e / Scope

- **Applicable Ã  :** Toute personne travaillant avec Ever Buddy
- **Audience :** Architectes, dÃ©veloppeurs, intÃ©grateurs, Ã©quipes produit
- **Statut :** Document de rÃ©fÃ©rence â€” Informatif
- **DÃ©pendances :** Documentation Fondatrice Ever Buddy, Glossaire Miyukini

---

## 1. Questions gÃ©nÃ©rales sur Ever Buddy

### Q1.1 : Qu'est-ce qu'Ever Buddy exactement ?

**Ever Buddy est le core de cycle de vie et d'Ã©volution** (Strate 4) du Miyukini Core System. Il reprÃ©sente la **conscience temporelle** du systÃ¨me : il observe ce qui a Ã©tÃ©, ce qui est, et ce qui sera.

**RÃ´le principal :** Gouverner l'Ã©volution des structures, des contrats, et des entitÃ©s dans le temps, sans jamais exÃ©cuter de migration technique ou modifier directement les donnÃ©es.

**Question fondamentale :** *"Comment le systÃ¨me Ã©volue-t-il sans jamais se rompre ?"*

**RÃ©fÃ©rence :** [Documentation Fondatrice, Section 1](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#1-introduction)

---

### Q1.2 : Pourquoi Ever Buddy existe-t-il ? Quel problÃ¨me rÃ©sout-il ?

Ever Buddy rÃ©sout les problÃ¨mes liÃ©s Ã  l'Ã©volution non contrÃ´lÃ©e des systÃ¨mes :

| ProblÃ¨me | Solution Ever Buddy |
|----------|---------------------|
| **Ruptures non contrÃ´lÃ©es** | Ã‰tats de cycle de vie explicites et transitions validÃ©es |
| **Dette structurelle invisible** | Surveillance continue du debt ratio |
| **Transitions brutales** | PÃ©riode de dÃ©prÃ©ciation obligatoire (INV-EB-4) |
| **Perte de mÃ©moire** | TraÃ§abilitÃ© complÃ¨te et immuable (INV-EB-2) |
| **Ã‰volutions contradictoires** | Gouvernance centralisÃ©e des Ã©volutions |

**RÃ©fÃ©rence :** [Documentation Fondatrice, Section 2](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#2-raison-dÃªtre)

---

### Q1.3 : Quelle est la phrase fondatrice d'Ever Buddy ?

> **Ever Buddy est le compagnon de toujours qui observe, enregistre, et guide l'Ã©volution du systÃ¨me, garantissant que chaque changement respecte la continuitÃ©, que chaque transition est traÃ§able, et que l'avenir est prÃ©parÃ© sans sacrifier le prÃ©sent.**

Cette phrase rÃ©sume l'essence d'Ever Buddy :
- **Compagnon** : PrÃ©sent mais non autoritaire
- **Observateur** : Pas exÃ©cuteur
- **Guide** : Influence sans contrainte
- **Gardien de la continuitÃ©** : Vision long terme

**RÃ©fÃ©rence :** [Documentation Fondatrice, Section 11](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#11-conclusion-et-statut-contractuel)

---

## 2. Questions sur les Ã©tats de cycle de vie

### Q2.1 : Quels sont les Ã©tats de cycle de vie possibles ?

Ever Buddy dÃ©finit **cinq Ã©tats de cycle de vie** :

| Ã‰tat | Description | Production | StabilitÃ© |
|------|-------------|------------|-----------|
| **DRAFT** | En cours de dÃ©finition | âŒ | âŒ |
| **ACTIVE** | En usage normal, supportÃ© | âœ… | âœ… |
| **DEPRECATED** | Fonctionnel mais usage dÃ©couragÃ© | âš ï¸ | âœ… |
| **RETIRED** | Non supportÃ©, corrections critiques seulement | âš ï¸ | âš ï¸ |
| **ARCHIVED** | Non fonctionnel, rÃ©fÃ©rence historique | âŒ | âŒ |

**RÃ©fÃ©rence Glossaire :** [DRAFT](..//..//..//miyukini-webway-system//reference//_index.md#brouillon-draft--Ã©tat-de-vie), [ACTIVE](..//..//..//miyukini-webway-system//reference//_index.md#actif-active--Ã©tat-de-vie), [DEPRECATED](..//..//..//miyukini-webway-system//reference//_index.md#dÃ©prÃ©ciÃ©-deprecated--Ã©tat-de-vie), [RETIRED](..//..//..//miyukini-webway-system//reference//_index.md#retirÃ©-retired--Ã©tat-de-vie)

**RÃ©fÃ©rence contrat :** [Lifecycle States Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md)

---

### Q2.2 : Un Ã©lÃ©ment peut-il Ãªtre dans plusieurs Ã©tats Ã  la fois ?

**Non, jamais.** L'invariant INV-EB-3 garantit :

> Chaque Ã©lÃ©ment du systÃ¨me possÃ¨de **exactement un** Ã©tat de cycle de vie Ã  tout moment. Il n'existe pas d'Ã©tat intermÃ©diaire, incertain, ou non dÃ©fini.

Les transitions sont **atomiques** : un Ã©lÃ©ment passe de l'Ã©tat A Ã  l'Ã©tat B sans Ã©tat transitoire.

**Violations dÃ©tectÃ©es :**
- Un Ã©lÃ©ment sans Ã©tat dÃ©clarÃ©
- Un Ã©lÃ©ment avec plusieurs Ã©tats simultanÃ©s
- Un Ã©lÃ©ment dans un Ã©tat "en transition"

**RÃ©fÃ©rence :** [Documentation Fondatrice, INV-EB-3](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#inv-eb-3--aucun-Ã©tat-ambigu)

---

### Q2.3 : Peut-on passer directement de ACTIVE Ã  RETIRED ?

**Non, c'est structurellement interdit.** L'invariant INV-EB-4 Ã©tablit :

> Aucun Ã©lÃ©ment ACTIVE ne peut passer directement Ã  RETIRED ou ARCHIVED. La transition par DEPRECATED est **obligatoire**.

La matrice des transitions valides :

| Depuis \ Vers | DRAFT | ACTIVE | DEPRECATED | RETIRED | ARCHIVED |
|---------------|-------|--------|------------|---------|----------|
| DRAFT         | â€”     | âœ“      | âœ—          | âœ—       | âœ“        |
| ACTIVE        | âœ—     | â€”      | âœ“          | âœ—       | âœ—        |
| DEPRECATED    | âœ—     | âœ“*     | â€”          | âœ“       | âœ—        |
| RETIRED       | âœ—     | âœ—      | âœ—          | â€”       | âœ“        |
| ARCHIVED      | âœ—     | âœ—      | âœ—          | âœ—       | â€”        |

*La rÃ©activation DEPRECATED â†’ ACTIVE est exceptionnelle (successeur annulÃ©).

**RÃ©fÃ©rence contrat :** [Transition Rules Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md)

---

### Q2.4 : Peut-on rÃ©activer un Ã©lÃ©ment dÃ©prÃ©ciÃ© ?

**Oui, mais c'est exceptionnel.** La transition DEPRECATED â†’ ACTIVE est possible uniquement si :

1. Le successeur prÃ©vu est annulÃ©
2. L'Ã©lÃ©ment dÃ©prÃ©ciÃ© est toujours fonctionnel
3. La dÃ©cision de rÃ©activation est documentÃ©e avec justification

L'historique conserve la trace de la dÃ©prÃ©ciation temporaire.

**RÃ©fÃ©rence :** [Documentation Fondatrice, Section 10, ScÃ©nario 4](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#scÃ©nario-4--rÃ©activation-dun-Ã©lÃ©ment-dÃ©prÃ©ciÃ©)

---

### Q2.5 : Un Ã©lÃ©ment ARCHIVED peut-il Ãªtre rÃ©activÃ© ?

**Non, jamais.** L'Ã©tat ARCHIVED est **terminal et dÃ©finitif**. Aucune transition n'est possible depuis ARCHIVED.

Si un Ã©lÃ©ment archivÃ© doit revivre, il faut crÃ©er un **nouvel Ã©lÃ©ment** inspirÃ© de l'archivÃ©, pas le rÃ©activer.

**RÃ©fÃ©rence contrat :** [Lifecycle States Contract, Section 3.5](../contracts/lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md#35-archived-archivÃ©)

---

## 3. Questions sur la compatibilitÃ© et les versions

### Q3.1 : Qu'est-ce que la rÃ©trocompatibilitÃ© par dÃ©faut ?

L'invariant INV-EB-5 Ã©tablit :

> Toute Ã©volution est **prÃ©sumÃ©e rÃ©trocompatible** sauf dÃ©claration explicite contraire.

Cela signifie que :
- Si vous ne dÃ©clarez rien, votre Ã©volution est considÃ©rÃ©e rÃ©trocompatible
- Une Ã©volution incompatible **doit** Ãªtre explicitement dÃ©clarÃ©e
- Les breaking changes nÃ©cessitent une justification et un plan de transition

**RÃ©fÃ©rence :** [Documentation Fondatrice, INV-EB-5](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#inv-eb-5--rÃ©trocompatibilitÃ©-par-dÃ©faut)

---

### Q3.2 : Comment fonctionne le versionnement sÃ©mantique ?

Ever Buddy utilise un **versionnement sÃ©mantique** (majeur.mineur.correctif) :

| Type | Signification | Exemple |
|------|---------------|---------|
| **Majeur** | Changement incompatible, rupture de contrat | 1.0 â†’ 2.0 |
| **Mineur** | Ajout de fonctionnalitÃ©, rÃ©trocompatible | 1.0 â†’ 1.1 |
| **Correctif** | Correction de bug, aucun changement fonctionnel | 1.0.0 â†’ 1.0.1 |

**Important :** Le versionnement sÃ©mantique ne dÃ©pend pas d'horloges synchronisÃ©es (conformitÃ© LOI-4).

**RÃ©fÃ©rence contrat :** [Version Semantics Contract](../contracts/compatibility/Ever%20Buddy%20-%20Version%20Semantics%20Contract.md)

---

### Q3.3 : Qu'est-ce qu'une fenÃªtre de compatibilitÃ© ?

La **fenÃªtre de compatibilitÃ©** est la plage de versions avec lesquelles un Ã©lÃ©ment garantit la compatibilitÃ©.

**Exemple :** "Compatible avec v2.0 Ã  v2.4" signifie que l'Ã©lÃ©ment fonctionne avec les versions 2.0, 2.1, 2.2, 2.3 et 2.4.

Les fenÃªtres de compatibilitÃ© sont dÃ©finies par Ever Buddy et appliquÃ©es par Border Guard aux frontiÃ¨res du systÃ¨me.

**RÃ©fÃ©rence Glossaire :** [Compatibility Window](..//..//..//miyukini-webway-system//reference//_index.md)

---

### Q3.4 : Qu'est-ce qu'un breaking change ?

Un **breaking change** est un changement qui rompt la compatibilitÃ© avec les versions prÃ©cÃ©dentes.

**ConsÃ©quences d'un breaking change :**
- Transition de version majeure obligatoire
- PÃ©riode de dÃ©prÃ©ciation de l'ancienne version
- Documentation explicite des diffÃ©rences
- Chemin de migration fourni

**RÃ©fÃ©rence :** [Documentation Fondatrice, Section 9 - Vocabulaire](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#9-vocabulaire-canonique)

---

## 4. Questions sur les responsabilitÃ©s et limites

### Q4.1 : Ever Buddy exÃ©cute-t-il des migrations ?

**Non, jamais.** L'invariant INV-EB-1 est absolu :

> Ever Buddy ne possÃ¨de **jamais** la capacitÃ© d'exÃ©cuter une migration, une transformation, ou une modification de donnÃ©es.

Ever Buddy **gouverne** les migrations :
- Il dÃ©finit les rÃ¨gles de migration
- Il communique les chemins de migration
- Il observe l'avancement des migrations

Mais l'**exÃ©cution** est la responsabilitÃ© de :
- **KindMother** pour les donnÃ©es
- **Les produits** pour leur code

**RÃ©fÃ©rence :** [Documentation Fondatrice, INV-EB-1](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#inv-eb-1--aucune-exÃ©cution-de-migration)

---

### Q4.2 : Ever Buddy peut-il forcer une Ã©volution ?

**Non, jamais.** Ever Buddy influence par la guidance, pas par la contrainte.

Il peut :
- âœ… Recommander
- âœ… Alerter
- âœ… Planifier

Il ne peut pas :
- âŒ Imposer
- âŒ Forcer
- âŒ Contraindre

Les produits et les autres cores conservent leur autonomie.

**RÃ©fÃ©rence :** [Documentation Fondatrice, Section 6](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#6-ce-que-ever-buddy-ne-fait-pas)

---

### Q4.3 : Ever Buddy dÃ©cide-t-il des permissions ?

**Non.** Ever Buddy **ne dÃ©cide jamais** si une action est permise. Cette dÃ©cision appartient Ã  **StrongFather**.

Ever Buddy fournit le **contexte** nÃ©cessaire Ã  la dÃ©cision :
- L'Ã©lÃ©ment est-il DEPRECATED ?
- Quelle est la fenÃªtre de compatibilitÃ© ?
- Y a-t-il un successeur ?

Mais la **dÃ©cision finale** est prise par StrongFather.

**RÃ©fÃ©rence Glossaire :** [StrongFather](..//..//..//miyukini-webway-system//reference//_index.md#strongfather)

**RÃ©fÃ©rence contrat :** [Core Interaction Contract, Section 2.2](../architecture/Ever%20Buddy%20-%20Core%20Interaction%20Contract.md#22-relation-avec-strongfather)

---

### Q4.4 : Ever Buddy modifie-t-il les donnÃ©es ?

**Non, jamais.** Ever Buddy ne modifie jamais les donnÃ©es gÃ©rÃ©es par KindMother.

Il peut :
- âœ… Observer
- âœ… Enregistrer (son propre historique)
- âœ… Recommander

Il ne peut pas :
- âŒ Modifier
- âŒ Supprimer
- âŒ Transformer

Toute modification est sous l'autoritÃ© exclusive de **KindMother**.

**RÃ©fÃ©rence :** [Documentation Fondatrice, Section 6](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#ne-modifie-jamais-les-donnÃ©es)

---

## 5. Questions sur les interactions avec les autres cores

### Q5.1 : Comment Ever Buddy interagit-il avec KindMother ?

**Relation : ComplÃ©mentaire**

| Aspect | KindMother | Ever Buddy |
|--------|------------|------------|
| DonnÃ©es Ã  instant T | âœ… AutoritÃ© | âŒ Lecture seule |
| SchÃ©mas de donnÃ©es | âœ… DÃ©finition | âœ… RÃ¨gles d'Ã©volution |
| Migrations de donnÃ©es | âœ… ExÃ©cution | âœ… DÃ©finition des rÃ¨gles |

KindMother notifie Ever Buddy de tout nouveau schÃ©ma. Ever Buddy dÃ©finit les rÃ¨gles d'Ã©volution. KindMother peut refuser une migration si elle viole ses propres invariants.

**RÃ©fÃ©rence contrat :** [Core Interaction Contract, Section 2.1](../architecture/Ever%20Buddy%20-%20Core%20Interaction%20Contract.md#21-relation-avec-kindmother)

---

### Q5.2 : Comment StrongFather utilise-t-il Ever Buddy ?

**Relation : Consultative**

StrongFather **consulte** Ever Buddy pour obtenir le contexte de cycle de vie :

| Information demandÃ©e | Usage par StrongFather |
|---------------------|------------------------|
| `current_state` | Ã‰valuer si l'action est permise |
| `deprecation_date` | Ã‰valuer l'urgence de migration |
| `successor_id` | Rediriger vers le successeur |
| `compatibility_level` | Ã‰valuer les risques |

StrongFather peut ignorer les recommandations d'Ever Buddy (mais c'est tracÃ©).

**RÃ©fÃ©rence contrat :** [Core Interaction Contract, Section 2.2](../architecture/Ever%20Buddy%20-%20Core%20Interaction%20Contract.md#22-relation-avec-strongfather)

---

### Q5.3 : Les produits parlent-ils directement Ã  Ever Buddy ?

**Non, jamais.** Les produits interagissent avec Ever Buddy **exclusivement via BondingBrother**.

```
Produits â†’ BondingBrother â†’ Ever Buddy
             (traduction)

âŒ Produits â†’ Ever Buddy (INTERDIT)
```

BondingBrother traduit les demandes et filtre les rÃ©ponses.

**RÃ©fÃ©rence contrat :** [Core Interaction Contract, Section 4](../architecture/Ever%20Buddy%20-%20Core%20Interaction%20Contract.md#4-relation-avec-les-produits)

---

### Q5.4 : Quand Ever Buddy escalade-t-il vers TAMR ?

Ever Buddy signale Ã  TAMR (intervention humaine) les transitions critiques :

| Cas | SÃ©vÃ©ritÃ© |
|-----|----------|
| Migration majeure (version majeure) | Ã‰levÃ©e |
| Rupture de compatibilitÃ© (breaking change) | Ã‰levÃ©e |
| AccÃ©lÃ©ration de dÃ©prÃ©ciation | Moyenne |
| Archivage d'Ã©lÃ©ments FONDATION | Critique |
| RÃ©activation DEPRECATED â†’ ACTIVE | Moyenne |

TAMR peut bloquer une transition en attente de validation humaine.

**RÃ©fÃ©rence contrat :** [Core Interaction Contract, Section 2.7](../architecture/Ever%20Buddy%20-%20Core%20Interaction%20Contract.md#27-relation-avec-tamr)

---

## 6. Questions sur la dette structurelle

### Q6.1 : Qu'est-ce que la dette structurelle ?

La **dette structurelle** est l'ensemble des Ã©lÃ©ments DEPRECATED ou RETIRED qui persistent dans le systÃ¨me.

Cette dette n'est **pas nÃ©cessairement nÃ©gative** â€” elle est le prix de la continuitÃ©. Cependant, Ever Buddy la surveille et alerte quand elle devient excessive.

**RÃ©fÃ©rence :** [Documentation Fondatrice, Section 4](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#dette-structurelle)

---

### Q6.2 : Comment la dette est-elle mesurÃ©e ?

Ever Buddy utilise le **debt ratio** :

```
debt_ratio = (DEPRECATED + RETIRED) / ACTIVE
```

| Debt Ratio | Signification |
|------------|---------------|
| < 0.1 | Sain |
| 0.1 - 0.3 | Acceptable |
| 0.3 - 0.5 | Attention requise |
| > 0.5 | Critique, action requise |

**RÃ©fÃ©rence contrat :** [Debt Tracking Contract](../contracts/observability/Ever%20Buddy%20-%20Debt%20Tracking%20Contract.md)

---

### Q6.3 : Que faire quand la dette est excessive ?

Ever Buddy recommande un **plan de nettoyage** :

1. Identifier les Ã©lÃ©ments RETIRED les plus anciens
2. VÃ©rifier qu'aucun consommateur ne les utilise encore
3. Les faire transitionner vers ARCHIVED
4. RÃ©pÃ©ter jusqu'Ã  ce que le debt ratio revienne sous le seuil

**RÃ©fÃ©rence :** [Documentation Fondatrice, Section 10, ScÃ©nario 5](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#scÃ©nario-5--dette-structurelle-excessive)

---

## 7. Questions sur la traÃ§abilitÃ©

### Q7.1 : L'historique des Ã©volutions peut-il Ãªtre modifiÃ© ?

**Non, jamais.** L'invariant INV-EB-2 garantit :

> Toute transition d'Ã©tat de cycle de vie est **obligatoirement** enregistrÃ©e et cet enregistrement est **immuable**. L'historique ne peut Ãªtre ni modifiÃ©, ni effacÃ©, ni falsifiÃ©.

L'immuabilitÃ© de l'historique garantit l'auditabilitÃ© et la comprÃ©hension des Ã©volutions passÃ©es.

**RÃ©fÃ©rence :** [Documentation Fondatrice, INV-EB-2](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#inv-eb-2--traÃ§abilitÃ©-complÃ¨te-et-immuable)

---

### Q7.2 : Quelle documentation est requise pour une transition ?

L'invariant INV-EB-7 exige que toute transition soit **documentÃ©e** avec :

| Information | Obligatoire |
|-------------|-------------|
| Raison de la transition | âœ… |
| Impact sur les consommateurs | âœ… |
| Chemin de migration (si applicable) | âœ… |
| Date effective | âœ… |

**Une transition sans documentation est invalide.**

**RÃ©fÃ©rence :** [Documentation Fondatrice, INV-EB-7](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#inv-eb-7--documentation-obligatoire)

---

### Q7.3 : Qu'est-ce qu'un tombstone ?

Un **tombstone** est l'enregistrement minimal conservÃ© pour un Ã©lÃ©ment archivÃ©.

**Ce qui est conservÃ© :**
- âœ… MÃ©tadonnÃ©es (ID, nom, version, dates)
- âœ… Historique des transitions
- âœ… Documentation finale (snapshot)
- âœ… Raison de l'archivage
- âœ… RÃ©fÃ©rence au successeur (si applicable)

**Ce qui n'est pas conservÃ© :**
- âŒ DonnÃ©es fonctionnelles

**RÃ©fÃ©rence :** [Documentation Fondatrice, Section 9](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#tombstone)

---

## 8. Questions sur les Tools et Toolkits

### Q8.1 : Ever Buddy gÃ¨re-t-il le cycle de vie des Tools ?

**Oui.** Ever Buddy est responsable du **cycle de vie** des Tools et Toolkits (Strate 6) :

| ResponsabilitÃ© | Description |
|----------------|-------------|
| **Versions** | GÃ¨re les versions de chaque Tool |
| **DÃ©prÃ©ciation** | Marque les Tools comme DEPRECATED |
| **CompatibilitÃ©** | VÃ©rifie Tool â†” Environnement |
| **Migration** | GÃ¨re la transition vers nouvelle version |

**Question Ã  laquelle Ever Buddy rÃ©pond :**

> *"Est-ce que cet outil existe encore, est compatible, ou doit Ãªtre migrÃ© ?"*

**RÃ©fÃ©rence :** [Documentation Fondatrice, Section 3](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#responsabilitÃ©-spÃ©cifique--cycle-de-vie-des-tools-et-toolkits)

---

### Q8.2 : Quelles sont les rÃ¨gles spÃ©cifiques aux Tools ?

| RÃ¨gle | Description |
|-------|-------------|
| **RÃˆGLE-TOOL-EV-1** | Tout Tool a un Ã©tat de vie explicite |
| **RÃˆGLE-TOOL-EV-2** | Un Tool DEPRECATED a un successeur identifiÃ© |
| **RÃˆGLE-TOOL-EV-3** | La transition vers RETIRED passe obligatoirement par DEPRECATED |
| **RÃˆGLE-TOOL-EV-4** | La compatibilitÃ© Tool â†” Environnement est vÃ©rifiÃ©e |

**RÃ©fÃ©rence :** [Tools et Toolkits](..//..//..//miyukini-webway-system//reference//_index.md)

---

## 9. Questions sur la conformitÃ©

### Q9.1 : Ever Buddy respecte-t-il les Lois d'Autonomie ?

**Oui, pleinement.** Ever Buddy est conforme Ã  toutes les lois :

| Loi | ConformitÃ© | MÃ©canisme |
|-----|------------|-----------|
| **LOI-1** | âœ… | Registre d'Ã©tats local, rÃ¨gles statiques |
| **LOI-2** | âœ… | Transitions validÃ©es localement |
| **LOI-3** | âœ… | Historique immuable local (INV-EB-2) |
| **LOI-4** | âœ… | Ã‰tats discrets et versionnement sÃ©mantique |
| **LOI-5** | âœ… | Observation pure, pas d'exÃ©cution |
| **LOI-6** | âœ… | FÃ©dÃ©ration via BondingBrother optionnelle |

**Question de validation :** *"Est-ce que Ever Buddy fonctionne encore si le systÃ¨me est seul, lent, et isolÃ© ?"* â€” **Oui.**

**RÃ©fÃ©rence :** [Documentation Fondatrice, Section 12](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#12-conformitÃ©-aux-lois-dautonomie-systÃ¨me)

**RÃ©fÃ©rence Glossaire :** [LOI-1 Ã  LOI-8](..//..//..//miyukini-webway-system//reference//_index.md#loi-1-Ã -loi-8-lois-dautonomie)

---

### Q9.2 : Ever Buddy fonctionne-t-il en mode isolÃ© ?

**Oui.** En mode isolÃ©, Ever Buddy continue de :
- Gouverner les cycles de vie locaux
- Valider les transitions localement
- Maintenir l'historique local

La synchronisation des Ã©tats entre nÅ“uds (via BondingBrother) est optionnelle et non bloquante.

**RÃ©fÃ©rence :** [Documentation Fondatrice, LOI-2](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#loi-2--le-systÃ¨me-accepte-lisolement-comme-Ã©tat-normal)

---

## 10. Questions pratiques

### Q10.1 : Comment dÃ©prÃ©cier un Ã©lÃ©ment ?

**Ã‰tapes obligatoires :**

1. **Identifier le successeur** (ou dÃ©clarer "aucun successeur")
2. **DÃ©finir la pÃ©riode de dÃ©prÃ©ciation** (minimum selon la catÃ©gorie)
3. **Documenter la transition** avec :
   - Raison de dÃ©prÃ©ciation
   - Guide de migration
   - Date de retirement prÃ©vue
4. **Communiquer** via BondingBrother aux consommateurs
5. **Transition** : ACTIVE â†’ DEPRECATED

**Attention :** La communication prÃ©alable est obligatoire (minimum 1 cycle de release).

**RÃ©fÃ©rence :** [Documentation Fondatrice, Section 4](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#pÃ©riodes-minimales-de-transition)

---

### Q10.2 : Combien de temps dure une pÃ©riode de dÃ©prÃ©ciation ?

La durÃ©e dÃ©pend de la **catÃ©gorie de l'Ã©lÃ©ment** :

| CatÃ©gorie | PÃ©riode minimale |
|-----------|------------------|
| **Contrats fondateurs (FONDATION)** | TrÃ¨s longue (plusieurs gÃ©nÃ©rations) |
| **Contrats opÃ©rationnels** | Standard |
| **Interfaces techniques** | Courte |
| **Ã‰lÃ©ments internes** | Optionnelle |

Ces pÃ©riodes sont des **minimums**. Ever Buddy peut recommander des pÃ©riodes plus longues selon l'impact et l'adoption.

**RÃ©fÃ©rence contrat :** [Lifecycle States Contract, Section 7](../contracts/lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md#7-catÃ©gories-dÃ©lÃ©ments-et-rÃ¨gles-dÃ©tat)

---

### Q10.3 : Comment savoir si mon Ã©lÃ©ment est compatible avec une version ?

**Demander Ã  Ever Buddy** (via BondingBrother) :
- La **fenÃªtre de compatibilitÃ©** de votre Ã©lÃ©ment
- Le **niveau de compatibilitÃ©** avec la version cible

**Niveaux de compatibilitÃ© :**

| Niveau | Signification |
|--------|---------------|
| **RÃ©trocompatible** | Le nouveau fonctionne avec l'ancien |
| **Compatible en amont** | L'ancien fonctionne avec le nouveau (rare) |
| **Incompatible** | Migration obligatoire |

**RÃ©fÃ©rence :** [Documentation Fondatrice, Section 4 - CompatibilitÃ©](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#compatibilitÃ©)

---

### Q10.4 : Que se passe-t-il si je n'ai pas migrÃ© Ã  temps ?

Si vous n'avez pas migrÃ© avant la date de retirement :

1. **PÃ©riode de grÃ¢ce** : Temps supplÃ©mentaire accordÃ© au cas par cas
2. **Restrictions** : L'Ã©lÃ©ment RETIRED n'est plus proposÃ© aux nouveaux consommateurs
3. **Support minimal** : Uniquement corrections critiques de sÃ©curitÃ©
4. **Archivage** : AprÃ¨s la pÃ©riode de grÃ¢ce, l'Ã©lÃ©ment devient ARCHIVED et non fonctionnel

**Recommandation :** Migrer pendant la pÃ©riode DEPRECATED, pas aprÃ¨s.

**RÃ©fÃ©rence :** [Documentation Fondatrice, Section 9 - Grace period](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#grace-period)

---

## 11. Questions sur les invariants

### Q11.1 : Quels sont les invariants d'Ever Buddy ?

| Invariant | Ã‰noncÃ© |
|-----------|--------|
| **INV-EB-1** | Aucune exÃ©cution de migration |
| **INV-EB-2** | TraÃ§abilitÃ© complÃ¨te et immuable |
| **INV-EB-3** | Aucun Ã©tat ambigu |
| **INV-EB-4** | PÃ©riode de dÃ©prÃ©ciation obligatoire |
| **INV-EB-5** | RÃ©trocompatibilitÃ© par dÃ©faut |
| **INV-EB-6** | Vision long terme obligatoire |
| **INV-EB-7** | Documentation obligatoire |
| **INV-EB-8** | IndÃ©pendance des dÃ©cisions |
| **INV-EB-9** | PrÃ©dictibilitÃ© des transitions |
| **INV-EB-10** | UnicitÃ© du successeur dÃ©clarÃ© |
| **INV-EB-11** | Non-rÃ©troactivitÃ© des changements de rÃ¨gles |
| **INV-EB-12** | ResponsabilitÃ© de l'annonce |

**RÃ©fÃ©rence contrat :** [Invariants & Guarantees](../contracts/governance/Ever%20Buddy%20-%20Invariants%20%26%20Guarantees.md)

---

### Q11.2 : Les invariants peuvent-ils Ãªtre modifiÃ©s ?

**Non pour les invariants fondamentaux.** Les invariants de la Documentation Fondatrice sont de statut **FONDATION** â€” non nÃ©gociables.

L'invariant INV-EB-11 Ã©tablit que :

> Les rÃ¨gles d'Ã©volution s'appliquent aux transitions **futures**. Un changement de rÃ¨gle ne peut pas modifier le statut d'Ã©lÃ©ments dÃ©jÃ  en transition.

**RÃ©fÃ©rence :** [Documentation Fondatrice, INV-EB-11](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#inv-eb-11--non-rÃ©troactivitÃ©-des-changements-de-rÃ¨gles)

---

## 12. RÃ©fÃ©rences

### Documents fondateurs

- [Ever Buddy - Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

### Contrats associÃ©s

- [Lifecycle States Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md)
- [Transition Rules Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md)
- [Compatibility Rules Contract](../contracts/compatibility/Ever%20Buddy%20-%20Compatibility%20Rules%20Contract.md)
- [Version Semantics Contract](../contracts/compatibility/Ever%20Buddy%20-%20Version%20Semantics%20Contract.md)
- [Invariants & Guarantees](../contracts/governance/Ever%20Buddy%20-%20Invariants%20%26%20Guarantees.md)
- [Debt Tracking Contract](../contracts/observability/Ever%20Buddy%20-%20Debt%20Tracking%20Contract.md)
- [Core Interaction Contract](../architecture/Ever%20Buddy%20-%20Core%20Interaction%20Contract.md)

### RÃ©fÃ©rences externes

- [Glossaire Miyukini](..//..//..//miyukini-webway-system//reference//_index.md)
- [Lois d'Autonomie SystÃ¨me](..//..//..//miyukini-webway-system//reference//_index.md)
- [Tools et Toolkits](..//..//..//miyukini-webway-system//reference//_index.md)

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Document de rÃ©fÃ©rence â€” Informatif  
**DÃ©rivÃ© de :** Ever Buddy - Documentation Fondatrice v1.3  
**Type :** FAQ et questions frÃ©quentes

