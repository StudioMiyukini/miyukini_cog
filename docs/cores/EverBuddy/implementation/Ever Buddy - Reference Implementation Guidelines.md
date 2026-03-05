# Ever Buddy â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter Ever Buddy correctement, sans violer les contrats FONDATION.

**Objectif pÃ©dagogique :** Ce document vise Ã  aider les dÃ©veloppeurs Ã  comprendre comment traduire les contrats FONDATION en implÃ©mentation, en respectant strictement les invariants, garanties, et interdictions.

**Avertissement :** Ce document ne doit pas Ãªtre interprÃ©tÃ© abusivement. Il ne crÃ©e aucune nouvelle rÃ¨gle contractuelle et ne modifie aucun contrat existant. Les contrats FONDATION priment toujours sur ce guide.

**Relation avec les contrats FONDATION :** Ce document fait rÃ©fÃ©rence aux contrats FONDATION existants mais ne les Ã©tend pas, ne les modifie pas, et ne crÃ©e aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1. Objectif

Ce document fournit des lignes directrices pour implÃ©menter Ever Buddy de maniÃ¨re conforme aux contrats FONDATION. Il explique comment traduire les concepts contractuels en logique d'implÃ©mentation sans interprÃ©tation abusive.

### 1.2. Nature informative

Ce document est **purement informatif**. Il ne dÃ©finit pas de nouvelles rÃ¨gles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la comprÃ©hension et l'application des contrats FONDATION.

### 1.3. Rappel de la mission d'Ever Buddy

Ever Buddy est le **core de cycle de vie et d'Ã©volution** (Strate 4). Il rÃ©pond Ã  la question fondamentale :

> **"Comment le systÃ¨me Ã©volue-t-il sans jamais se rompre ?"**

Ever Buddy **observe, enregistre, et guide** l'Ã©volution du systÃ¨me. Il **ne migre jamais**, **ne modifie jamais**, et **n'exÃ©cute jamais**.

### 1.4. Sources contractuelles

Ce document se base sur les contrats FONDATION, avec un focus particulier sur :

- **Documentation Fondatrice** : Invariants INV-EB-1 Ã  INV-EB-12, responsabilitÃ©s exclusives, interdictions
- **Lifecycle States Contract** : Ã‰tats DRAFT, ACTIVE, DEPRECATED, RETIRED, ARCHIVED
- **Transition Rules Contract** : Matrice des transitions valides, pÃ©riodes minimales
- **Compatibility Rules Contract** : RÃ©trocompatibilitÃ©, compatibilitÃ© amont, ruptures
- **Invariants & Guarantees** : Garanties structurelles non nÃ©gociables
- **[Miyukini Conceptual References â€” Lois Autonomie SystÃ¨me](..//..//..//miyukini-webway-system//reference//_index.md)** : Les lignes directrices d'implÃ©mentation doivent respecter les 6 lois d'autonomie, notamment **LOI-1** (aucune dÃ©pendance externe critique), **LOI-3** (Ã©tat local souverain), **LOI-4** (pas de temps global requis).

---

## 2. Principes gÃ©nÃ©raux Ã  respecter absolument

### 2.1. SÃ©paration gouvernance / exÃ©cution (INV-EB-1)

**Principe contractuel :**

L'invariant INV-EB-1 Ã©tablit qu'Ever Buddy ne possÃ¨de **jamais** la capacitÃ© d'exÃ©cuter une migration, une transformation, ou une modification de donnÃ©es. Il dÃ©finit les rÃ¨gles et observe les transitions, mais toute exÃ©cution est dÃ©lÃ©guÃ©e aux autoritÃ©s compÃ©tentes.

**Traduction en logique d'implÃ©mentation :**

- **Ever Buddy OBSERVE** : Il enregistre les Ã©tats, les transitions, et l'historique.
- **Ever Buddy DÃ‰FINIT** : Il Ã©tablit les rÃ¨gles d'Ã©volution et de compatibilitÃ©.
- **Ever Buddy NE FAIT JAMAIS** : Il n'exÃ©cute aucune migration, ne modifie aucune donnÃ©e.

**Ce que cela signifie concrÃ¨tement :**

- Aucun mÃ©canisme d'Ã©criture de donnÃ©es ne doit Ãªtre accessible Ã  Ever Buddy
- Les migrations sont exÃ©cutÃ©es par KindMother (pour les donnÃ©es) ou par les produits (pour leur code)
- Ever Buddy fournit les rÃ¨gles, les calendriers, et les validations â€” jamais l'exÃ©cution

### 2.2. TraÃ§abilitÃ© complÃ¨te et immuable (INV-EB-2)

**Principe contractuel :**

L'invariant INV-EB-2 Ã©tablit que toute transition d'Ã©tat de cycle de vie est **obligatoirement** enregistrÃ©e et cet enregistrement est **immuable**. L'historique ne peut Ãªtre ni modifiÃ©, ni effacÃ©, ni falsifiÃ©.

**Traduction en logique d'implÃ©mentation :**

- **TraÃ§abilitÃ© systÃ©matique** : Chaque transition DOIT Ãªtre enregistrÃ©e avec son contexte complet.
- **ImmuabilitÃ© garantie** : Les traces NE PEUVENT JAMAIS Ãªtre modifiÃ©es aprÃ¨s crÃ©ation.
- **AccessibilitÃ© auditÃ©e** : L'historique DOIT Ãªtre accessible pour audit par les acteurs autorisÃ©s.

**Ce que cela signifie concrÃ¨tement :**

- Toute transition est tracÃ©e avec : raison, impact, date, chemin de migration (si applicable)
- Les traces sont append-only â€” aucune modification, aucune suppression
- L'historique complet est consultable pour comprendre les dÃ©cisions passÃ©es

### 2.3. Aucun Ã©tat ambigu (INV-EB-3)

**Principe contractuel :**

L'invariant INV-EB-3 Ã©tablit que chaque Ã©lÃ©ment du systÃ¨me possÃ¨de **exactement un** Ã©tat de cycle de vie Ã  tout moment. Il n'existe pas d'Ã©tat intermÃ©diaire, incertain, ou non dÃ©fini. Les transitions sont atomiques.

**Traduction en logique d'implÃ©mentation :**

- **Ã‰tat unique** : Un Ã©lÃ©ment est DRAFT, ACTIVE, DEPRECATED, RETIRED, ou ARCHIVED â€” jamais entre deux.
- **Transitions atomiques** : Le passage d'un Ã©tat Ã  un autre est instantanÃ©, sans Ã©tat transitoire.
- **Pas d'ambiguÃ¯tÃ©** : Ã€ tout instant, l'Ã©tat d'un Ã©lÃ©ment est dÃ©terminable sans incertitude.

**Ce que cela signifie concrÃ¨tement :**

- Le registre des Ã©tats ne contient jamais d'Ã©tats "en transition" ou "indÃ©terminÃ©s"
- Une transition rÃ©ussit ou Ã©choue â€” pas d'Ã©tat intermÃ©diaire
- L'API d'interrogation retourne toujours un Ã©tat clair et dÃ©fini

### 2.4. PÃ©riode de dÃ©prÃ©ciation obligatoire (INV-EB-4)

**Principe contractuel :**

L'invariant INV-EB-4 Ã©tablit qu'aucun Ã©lÃ©ment ACTIVE ne peut passer directement Ã  RETIRED ou ARCHIVED. La transition par DEPRECATED est **obligatoire**. La pÃ©riode de dÃ©prÃ©ciation minimale ne peut Ãªtre contournÃ©e.

**Traduction en logique d'implÃ©mentation :**

- **Passage obligatoire par DEPRECATED** : Toute transition ACTIVE â†’ RETIRED DOIT passer par DEPRECATED.
- **PÃ©riode minimale respectÃ©e** : La durÃ©e de dÃ©prÃ©ciation est dÃ©finie par catÃ©gorie d'Ã©lÃ©ment et non nÃ©gociable.
- **Aucun raccourci** : Aucun mÃ©canisme ne peut permettre de contourner cette rÃ¨gle.

**Ce que cela signifie concrÃ¨tement :**

- La matrice des transitions est strictement appliquÃ©e
- Toute tentative de transition ACTIVE â†’ RETIRED est rejetÃ©e
- Les pÃ©riodes minimales sont vÃ©rifiÃ©es avant toute transition DEPRECATED â†’ RETIRED

### 2.5. RÃ©trocompatibilitÃ© par dÃ©faut (INV-EB-5)

**Principe contractuel :**

L'invariant INV-EB-5 Ã©tablit que toute Ã©volution est **prÃ©sumÃ©e rÃ©trocompatible** sauf dÃ©claration explicite contraire. Si une Ã©volution est incompatible, elle doit Ãªtre explicitement dÃ©clarÃ©e comme telle.

**Traduction en logique d'implÃ©mentation :**

- **PrÃ©somption de compatibilitÃ©** : Par dÃ©faut, une nouvelle version est considÃ©rÃ©e rÃ©trocompatible.
- **DÃ©claration explicite des ruptures** : Les breaking changes DOIVENT Ãªtre dÃ©clarÃ©s explicitement.
- **Justification obligatoire** : Toute rupture DOIT Ãªtre justifiÃ©e et accompagnÃ©e d'un plan de transition.

**Ce que cela signifie concrÃ¨tement :**

- Le systÃ¨me suppose qu'une Ã©volution mineure est compatible
- Les ruptures sont des exceptions qui nÃ©cessitent une documentation explicite
- Aucune rupture silencieuse n'est autorisÃ©e

### 2.6. Vision long terme obligatoire (INV-EB-6)

**Principe contractuel :**

L'invariant INV-EB-6 Ã©tablit que toute dÃ©cision d'Ã©volution doit considÃ©rer l'impact sur **au moins deux gÃ©nÃ©rations** de versions. Une Ã©volution qui rÃ©sout un problÃ¨me immÃ©diat mais crÃ©e un problÃ¨me futur plus grave est invalide.

**Traduction en logique d'implÃ©mentation :**

- **Analyse prospective** : Chaque Ã©volution DOIT Ãªtre Ã©valuÃ©e sur son impact Ã  long terme.
- **Rejet des solutions court-termistes** : Une solution qui accumule la dette est invalide.
- **PensÃ©e gÃ©nÃ©rationnelle** : Ever Buddy pense en gÃ©nÃ©rations, pas en sprints.

**Ce que cela signifie concrÃ¨tement :**

- Les dÃ©cisions d'Ã©volution incluent une analyse d'impact sur N+1 et N+2 gÃ©nÃ©rations
- Les solutions qui crÃ©ent des problÃ¨mes futurs sont rejetÃ©es mÃªme si elles rÃ©solvent un problÃ¨me immÃ©diat
- La dette structurelle est surveillÃ©e et limitÃ©e

---

## 3. Comment traduire les contrats en logique sans interprÃ©tation abusive

### 3.1. Respecter les invariants comme contraintes absolues

**Principe :**

Les invariants contractuels (INV-EB-*) sont des contraintes absolues qui DOIVENT toujours Ãªtre vraies. Ils ne sont pas des suggestions ou des recommandations.

**Traduction :**

- **VÃ©rification systÃ©matique** : Chaque invariant DOIT Ãªtre vÃ©rifiÃ© Ã  chaque opÃ©ration.
- **PrÃ©servation garantie** : Toute opÃ©ration DOIT garantir que les invariants sont prÃ©servÃ©s aprÃ¨s exÃ©cution.
- **Pas d'interprÃ©tation** : Les invariants ne peuvent pas Ãªtre interprÃ©tÃ©s ou adaptÃ©s.

**Exemple conceptuel :**

Si l'invariant INV-EB-4 (pÃ©riode de dÃ©prÃ©ciation obligatoire) exige le passage par DEPRECATED, alors aucune transition directe ACTIVE â†’ RETIRED n'est possible, mÃªme pour des raisons "urgentes" ou "exceptionnelles".

### 3.2. ImplÃ©menter la traÃ§abilitÃ© comme obligation, pas comme option

**Principe :**

La traÃ§abilitÃ© complÃ¨te et immuable (INV-EB-2) est une obligation structurelle, pas une fonctionnalitÃ© optionnelle.

**Traduction :**

- **TraÃ§abilitÃ© obligatoire** : Chaque transition DOIT Ãªtre tracÃ©e. Aucune exception.
- **ImmuabilitÃ© structurelle** : Le mÃ©canisme de stockage DOIT garantir l'immuabilitÃ©.
- **AccessibilitÃ© auditÃ©e** : Les traces DOIVENT Ãªtre accessibles pour audit.

**Exemple conceptuel :**

MÃªme si une transition semble "triviale" (ex: passage d'un Ã©lÃ©ment interne en DEPRECATED), elle DOIT Ãªtre tracÃ©e avec le mÃªme niveau de dÃ©tail qu'une transition majeure.

### 3.3. Traiter la matrice des transitions comme non nÃ©gociable

**Principe :**

La matrice des transitions valides est absolue. Seules les transitions marquÃ©es âœ“ sont autorisÃ©es.

**Traduction :**

- **Validation stricte** : Toute transition DOIT Ãªtre validÃ©e contre la matrice avant exÃ©cution.
- **Rejet immÃ©diat** : Les transitions invalides sont rejetÃ©es immÃ©diatement, sans exception.
- **Pas de contournement** : Aucun mÃ©canisme ne peut permettre une transition invalide.

**Exemple conceptuel :**

Une demande de transition RETIRED â†’ ACTIVE est structurellement impossible. Elle est rejetÃ©e sans Ã©valuation de la raison ou de l'urgence.

### 3.4. Ne pas "optimiser" en contournant les pÃ©riodes de transition

**Principe :**

Les pÃ©riodes minimales de transition sont des protections pour les consommateurs, pas des inefficacitÃ©s Ã  Ã©liminer.

**Traduction :**

- **PÃ©riodes respectÃ©es** : Les pÃ©riodes minimales ne peuvent pas Ãªtre raccourcies.
- **Pas d'optimisation au dÃ©triment de la protection** : La protection des consommateurs prime sur la commoditÃ©.
- **Calcul honnÃªte** : Les pÃ©riodes sont calculÃ©es selon les rÃ¨gles dÃ©finies, sans manipulation.

**Exemple conceptuel :**

MÃªme si tous les consommateurs connus ont migrÃ©, la pÃ©riode minimale de dÃ©prÃ©ciation doit Ãªtre respectÃ©e. Il peut exister des consommateurs inconnus.

---

## 4. Ce qu'un dÃ©veloppeur ne doit jamais faire

### 4.1. ExÃ©cuter une migration (INV-EB-1)

**Interdiction contractuelle :**

L'invariant INV-EB-1 Ã©tablit qu'Ever Buddy ne possÃ¨de **jamais** la capacitÃ© d'exÃ©cuter une migration.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- ImplÃ©menter des mÃ©canismes de migration de donnÃ©es dans Ever Buddy
- Permettre Ã  Ever Buddy de modifier directement des structures ou des donnÃ©es
- CrÃ©er des "migrations automatiques" exÃ©cutÃ©es par Ever Buddy
- DÃ©lÃ©guer l'exÃ©cution de migrations Ã  Ever Buddy

**ConsÃ©quence de la violation :**

- Violation de l'invariant INV-EB-1 (aucune exÃ©cution de migration)
- Violation de la sÃ©paration gouvernance / exÃ©cution
- Compromission de l'architecture fondamentale du Miyukini Core System

### 4.2. Modifier l'historique (INV-EB-2)

**Interdiction contractuelle :**

L'invariant INV-EB-2 Ã©tablit que l'historique des transitions est **immuable**.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- ImplÃ©menter des mÃ©canismes de modification de l'historique
- Permettre la suppression de traces, mÃªme "obsolÃ¨tes"
- CrÃ©er des mÃ©canismes de "correction" de l'historique
- Exposer des APIs de modification des enregistrements passÃ©s

**ConsÃ©quence de la violation :**

- Violation de l'invariant INV-EB-2 (traÃ§abilitÃ© immuable)
- Compromission de l'auditabilitÃ© du systÃ¨me
- Perte de confiance dans l'historique des Ã©volutions

### 4.3. Permettre des Ã©tats ambigus (INV-EB-3)

**Interdiction contractuelle :**

L'invariant INV-EB-3 Ã©tablit qu'il n'existe pas d'Ã©tat intermÃ©diaire ou incertain.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- CrÃ©er des Ã©tats "en transition" ou "pending"
- Permettre des transitions non atomiques
- Exposer des Ã©tats incertains ou indÃ©terminÃ©s
- ImplÃ©menter des transitions qui peuvent rester "en cours" indÃ©finiment

**ConsÃ©quence de la violation :**

- Violation de l'invariant INV-EB-3 (aucun Ã©tat ambigu)
- Compromission de la prÃ©dictibilitÃ© du systÃ¨me
- Confusion sur l'Ã©tat rÃ©el des Ã©lÃ©ments

### 4.4. Contourner la dÃ©prÃ©ciation (INV-EB-4)

**Interdiction contractuelle :**

L'invariant INV-EB-4 Ã©tablit que le passage par DEPRECATED est **obligatoire**.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- CrÃ©er des "fast paths" pour Ã©viter la dÃ©prÃ©ciation
- Permettre des transitions directes ACTIVE â†’ RETIRED
- ImplÃ©menter des "exceptions d'urgence" qui contournent DEPRECATED
- RÃ©duire les pÃ©riodes minimales de dÃ©prÃ©ciation

**ConsÃ©quence de la violation :**

- Violation de l'invariant INV-EB-4 (dÃ©prÃ©ciation obligatoire)
- Rupture brutale pour les consommateurs
- Perte de confiance dans les rÃ¨gles d'Ã©volution

### 4.5. Prendre des dÃ©cisions Ã  la place de StrongFather

**Interdiction contractuelle :**

Ever Buddy fournit le contexte de cycle de vie, mais la dÃ©cision d'autoriser ou non une action appartient Ã  StrongFather.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- ImplÃ©menter des dÃ©cisions d'autorisation dans Ever Buddy
- Bloquer des actions directement sans passer par StrongFather
- CrÃ©er des mÃ©canismes de "dÃ©cision automatique" dans Ever Buddy
- Confondre "information de cycle de vie" et "dÃ©cision d'autorisation"

**ConsÃ©quence de la violation :**

- Violation de la sÃ©paration des autoritÃ©s entre cores
- Conflit d'autoritÃ© avec StrongFather
- Compromission de l'architecture de gouvernance

### 4.6. Modifier les donnÃ©es de KindMother

**Interdiction contractuelle :**

Ever Buddy **ne modifie jamais** les donnÃ©es gÃ©rÃ©es par KindMother.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- Permettre Ã  Ever Buddy d'Ã©crire dans les donnÃ©es de KindMother
- CrÃ©er des "mises Ã  jour de schÃ©ma" exÃ©cutÃ©es par Ever Buddy
- ImplÃ©menter des "corrections de donnÃ©es" dans Ever Buddy
- AccÃ©der directement aux mÃ©canismes de persistance de KindMother

**ConsÃ©quence de la violation :**

- Violation de l'autoritÃ© exclusive de KindMother sur les donnÃ©es
- Violation de l'invariant INV-EB-1 (pas d'exÃ©cution)
- Compromission de l'intÃ©gritÃ© des donnÃ©es

---

## 5. Anti-patterns classiques

### 5.1. Anti-pattern 1 : Migration automatique

**Description :**

Tentative d'implÃ©menter des migrations automatiques exÃ©cutÃ©es par Ever Buddy lors des transitions d'Ã©tat.

**Exemple conceptuel :**

Un dÃ©veloppeur crÃ©e un mÃ©canisme oÃ¹ quand un schÃ©ma passe de DEPRECATED Ã  RETIRED, Ever Buddy exÃ©cute automatiquement une migration des donnÃ©es vers le nouveau schÃ©ma.

**ConsÃ©quence :**

- Violation de l'invariant INV-EB-1 (aucune exÃ©cution de migration)
- Violation de l'autoritÃ© de KindMother sur les donnÃ©es
- Couplage dangereux entre gouvernance et exÃ©cution

**Correction :**

Ever Buddy dÃ©finit les rÃ¨gles de migration et communique les calendriers. L'exÃ©cution de la migration est la responsabilitÃ© de KindMother (pour les donnÃ©es) ou des produits (pour leur code).

### 5.2. Anti-pattern 2 : Historique modifiable

**Description :**

Tentative de permettre la modification de l'historique pour "corriger des erreurs" ou "nettoyer les donnÃ©es obsolÃ¨tes".

**Exemple conceptuel :**

Un dÃ©veloppeur implÃ©mente une fonction "cleanHistory()" pour supprimer les anciennes traces de transition jugÃ©es "inutiles".

**ConsÃ©quence :**

- Violation de l'invariant INV-EB-2 (traÃ§abilitÃ© immuable)
- Perte de la capacitÃ© d'audit
- Compromission de la confiance dans l'historique

**Correction :**

L'historique est strictement append-only. Les traces ne sont jamais modifiÃ©es ni supprimÃ©es. Si l'espace devient un problÃ¨me, des mÃ©canismes d'archivage (pas de suppression) peuvent Ãªtre envisagÃ©s.

### 5.3. Anti-pattern 3 : Ã‰tats de transition

**Description :**

Tentative de crÃ©er des Ã©tats intermÃ©diaires pour gÃ©rer les transitions complexes.

**Exemple conceptuel :**

Un dÃ©veloppeur crÃ©e un Ã©tat "DEPRECATING" entre ACTIVE et DEPRECATED pour gÃ©rer la "transition en cours".

**ConsÃ©quence :**

- Violation de l'invariant INV-EB-3 (aucun Ã©tat ambigu)
- AmbiguÃ¯tÃ© sur l'Ã©tat rÃ©el des Ã©lÃ©ments
- ComplexitÃ© inutile et risque d'Ã©tats bloquÃ©s

**Correction :**

Les transitions sont atomiques. Un Ã©lÃ©ment est ACTIVE, puis instantanÃ©ment DEPRECATED. Il n'y a pas d'Ã©tat intermÃ©diaire. Les processus de prÃ©paration se font avant la transition, pas pendant.

### 5.4. Anti-pattern 4 : Fast path de retirement

**Description :**

Tentative de crÃ©er un chemin rapide pour retirer des Ã©lÃ©ments "urgents" sans passer par la dÃ©prÃ©ciation.

**Exemple conceptuel :**

Un dÃ©veloppeur crÃ©e une fonction "forceRetire()" qui permet de passer directement de ACTIVE Ã  RETIRED en cas d'"urgence sÃ©curitÃ©".

**ConsÃ©quence :**

- Violation de l'invariant INV-EB-4 (dÃ©prÃ©ciation obligatoire)
- Rupture brutale pour les consommateurs
- Perte de confiance dans les rÃ¨gles d'Ã©volution

**Correction :**

MÃªme en cas d'urgence, le passage par DEPRECATED est obligatoire. La pÃ©riode de dÃ©prÃ©ciation peut Ãªtre rÃ©duite au minimum dÃ©fini, mais jamais contournÃ©e. Les urgences de sÃ©curitÃ© peuvent justifier une pÃ©riode minimale trÃ¨s courte, mais pas l'absence de pÃ©riode.

### 5.5. Anti-pattern 5 : DÃ©cision d'autorisation intÃ©grÃ©e

**Description :**

Tentative de faire prendre des dÃ©cisions d'autorisation Ã  Ever Buddy basÃ©es sur l'Ã©tat de cycle de vie.

**Exemple conceptuel :**

Un dÃ©veloppeur implÃ©mente dans Ever Buddy la logique "si l'Ã©lÃ©ment est DEPRECATED, bloquer les nouvelles intÃ©grations".

**ConsÃ©quence :**

- Violation de la sÃ©paration des autoritÃ©s
- Conflit avec StrongFather (qui dÃ©cide des autorisations)
- Couplage dangereux entre cores

**Correction :**

Ever Buddy fournit l'information de cycle de vie Ã  StrongFather ("cet Ã©lÃ©ment est DEPRECATED"). StrongFather dÃ©cide si l'action est autorisÃ©e. Ever Buddy ne prend jamais de dÃ©cision d'autorisation.

---

## 6. Bonnes pratiques conceptuelles

### 6.1. Registre d'Ã©tats centralisÃ© et souverain

**Pratique :**

Maintenir un registre centralisÃ© des Ã©tats de cycle de vie, accessible en lecture par tous les cores mais en Ã©criture uniquement par Ever Buddy.

**Justification :**

- Respecte l'autoritÃ© exclusive d'Ever Buddy sur les Ã©tats de vie (Section 5.1 de la Documentation Fondatrice)
- Garantit l'unicitÃ© de l'Ã©tat (INV-EB-3)
- Facilite la consultation par les autres cores

**ImplÃ©mentation conceptuelle :**

- Registre centralisÃ© avec Ã©tats courants
- API de lecture accessible aux autres cores
- API d'Ã©criture rÃ©servÃ©e Ã  Ever Buddy
- Synchronisation avec l'historique immuable

### 6.2. Historique append-only avec signatures

**Pratique :**

ImplÃ©menter l'historique comme une structure append-only avec des signatures cryptographiques pour garantir l'intÃ©gritÃ©.

**Justification :**

- Respecte l'invariant INV-EB-2 (traÃ§abilitÃ© immuable)
- Garantit l'impossibilitÃ© de modification
- Permet l'audit et la vÃ©rification

**ImplÃ©mentation conceptuelle :**

- Structure de donnÃ©es append-only (log immuable)
- Signature de chaque entrÃ©e
- ChaÃ®nage des signatures pour dÃ©tecter les modifications
- Pas de mÃ©canisme de suppression ou modification

### 6.3. Validation stricte des transitions avant enregistrement

**Pratique :**

Valider toute transition contre la matrice des transitions valides AVANT de l'enregistrer.

**Justification :**

- Respecte la matrice des transitions (Section 4 de la Documentation Fondatrice)
- Garantit qu'aucune transition invalide n'est enregistrÃ©e
- PrÃ©serve la cohÃ©rence du systÃ¨me

**ImplÃ©mentation conceptuelle :**

- Validation de la transition (Ã©tat source â†’ Ã©tat cible) contre la matrice
- VÃ©rification des pÃ©riodes minimales si applicable
- Rejet immÃ©diat si la transition est invalide
- Enregistrement atomique si la transition est valide

### 6.4. SÃ©paration claire entre observation et action

**Pratique :**

Maintenir une sÃ©paration architecturale claire entre les fonctions d'observation/enregistrement d'Ever Buddy et les fonctions d'action des autres cores.

**Justification :**

- Respecte l'invariant INV-EB-1 (aucune exÃ©cution)
- Garantit la sÃ©paration gouvernance / exÃ©cution
- Facilite l'audit et la comprÃ©hension

**ImplÃ©mentation conceptuelle :**

- Ever Buddy n'a accÃ¨s Ã  aucun mÃ©canisme d'Ã©criture de donnÃ©es mÃ©tier
- Les interfaces d'Ever Buddy sont strictement de lecture et d'enregistrement de transitions
- Les actions de migration sont explicitement dÃ©lÃ©guÃ©es aux autoritÃ©s compÃ©tentes

### 6.5. Communication proactive des calendriers d'Ã©volution

**Pratique :**

Communiquer proactivement les calendriers de dÃ©prÃ©ciation et les plans de transition Ã  tous les consommateurs concernÃ©s.

**Justification :**

- Respecte l'invariant INV-EB-12 (responsabilitÃ© de l'annonce)
- Garantit que les consommateurs ont le temps de rÃ©agir
- PrÃ©serve la confiance dans les rÃ¨gles d'Ã©volution

**ImplÃ©mentation conceptuelle :**

- Publication des calendriers de dÃ©prÃ©ciation
- Notifications aux consommateurs concernÃ©s
- Suivi de l'accusÃ© de rÃ©ception des annonces
- Documentation des communications pour audit

### 6.6. Surveillance active de la dette structurelle

**Pratique :**

Surveiller activement le ratio de dette structurelle et alerter quand il dÃ©passe les seuils dÃ©finis.

**Justification :**

- Respecte la responsabilitÃ© de surveillance de la dette (Section 5.4 de la Documentation Fondatrice)
- EmpÃªche l'accumulation non contrÃ´lÃ©e
- Permet des actions correctives avant que la dette ne devienne critique

**ImplÃ©mentation conceptuelle :**

- Calcul du debt ratio : (DEPRECATED + RETIRED) / ACTIVE
- DÃ©finition de seuils d'alerte
- Ã‰mission d'alertes quand les seuils sont dÃ©passÃ©s
- Recommandations de nettoyage

---

## 7. Check-list mentale avant toute feature

Avant d'implÃ©menter une nouvelle fonctionnalitÃ© liÃ©e Ã  Ever Buddy, un dÃ©veloppeur DOIT vÃ©rifier mentalement :

### 7.1. VÃ©rification des invariants

- **INV-EB-1 est-il prÃ©servÃ© ?** : La fonctionnalitÃ© n'exÃ©cute-t-elle aucune migration ?
- **INV-EB-2 est-il prÃ©servÃ© ?** : L'historique reste-t-il immuable ?
- **INV-EB-3 est-il prÃ©servÃ© ?** : Aucun Ã©tat ambigu n'est-il crÃ©Ã© ?
- **INV-EB-4 est-il prÃ©servÃ© ?** : Le passage par DEPRECATED est-il obligatoire ?
- **INV-EB-5 est-il prÃ©servÃ© ?** : La rÃ©trocompatibilitÃ© est-elle prÃ©sumÃ©e par dÃ©faut ?
- **INV-EB-6 est-il prÃ©servÃ© ?** : L'impact long terme est-il considÃ©rÃ© ?
- **INV-EB-7 est-il prÃ©servÃ© ?** : La documentation est-elle obligatoire ?
- **INV-EB-8 est-il prÃ©servÃ© ?** : Les rÃ¨gles sont-elles universelles ?
- **INV-EB-9 est-il prÃ©servÃ© ?** : Les rÃ¨gles sont-elles publiques et stables ?
- **INV-EB-10 est-il prÃ©servÃ© ?** : Un seul successeur est-il dÃ©clarÃ© ?
- **INV-EB-11 est-il prÃ©servÃ© ?** : Les changements de rÃ¨gles ne sont-ils pas rÃ©troactifs ?
- **INV-EB-12 est-il prÃ©servÃ© ?** : La responsabilitÃ© d'annonce est-elle respectÃ©e ?

### 7.2. VÃ©rification de la sÃ©paration des responsabilitÃ©s

- **Ever Buddy reste-t-il observateur ?** : La fonctionnalitÃ© n'exÃ©cute-t-elle rien ?
- **L'autoritÃ© de KindMother est-elle respectÃ©e ?** : Aucune modification de donnÃ©es ?
- **L'autoritÃ© de StrongFather est-elle respectÃ©e ?** : Aucune dÃ©cision d'autorisation ?
- **Les autres cores sont-ils informÃ©s, pas contraints ?** : Ever Buddy informe, il ne commande pas.

### 7.3. VÃ©rification de la conformitÃ© aux Lois d'Autonomie

- **LOI-1 respectÃ©e ?** : Aucune dÃ©pendance externe critique pour les Ã©tats de vie ?
- **LOI-3 respectÃ©e ?** : L'Ã©tat local est souverain ?
- **LOI-4 respectÃ©e ?** : Pas de temps global requis pour les transitions ?

### 7.4. VÃ©rification de la traÃ§abilitÃ©

- **Toutes les transitions sont-elles tracÃ©es ?** : Aucune transition silencieuse ?
- **Les traces sont-elles immuables ?** : Aucune modification possible ?
- **Les traces sont-elles accessibles ?** : Audit possible ?

---

## 8. Conclusion

Ce document fournit des lignes directrices pour implÃ©menter Ever Buddy de maniÃ¨re conforme aux contrats FONDATION.

**Points clÃ©s :**

- Ever Buddy **observe, enregistre, et guide** â€” il **n'exÃ©cute jamais**
- Les invariants INV-EB-1 Ã  INV-EB-12 sont des **contraintes absolues**
- La **traÃ§abilitÃ© est immuable** et la **dÃ©prÃ©ciation est obligatoire**
- La **sÃ©paration gouvernance / exÃ©cution** est fondamentale
- Les **Lois d'Autonomie** doivent Ãªtre respectÃ©es

**Nature informative :**

Ce document est purement informatif et ne crÃ©e aucune nouvelle obligation contractuelle. Il sert uniquement Ã  guider la comprÃ©hension et l'application des contrats FONDATION.

**Rappel :** Les contrats FONDATION priment toujours sur ce guide. En cas de doute, se rÃ©fÃ©rer Ã  la Documentation Fondatrice et aux contrats spÃ©cifiques.

**Phrase fondatrice Ã  garder en mÃ©moire :**

> **Ever Buddy est le compagnon de toujours qui observe, enregistre, et guide l'Ã©volution du systÃ¨me, garantissant que chaque changement respecte la continuitÃ©, que chaque transition est traÃ§able, et que l'avenir est prÃ©parÃ© sans sacrifier le prÃ©sent.**

---

**Document crÃ©Ã© le :** 2026-01-27  
**Version :** 1.0  
**Statut :** POST-FONDATION â€” Informatif, non normatif, non contractuel  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, Ever Buddy Documentation Fondatrice, Tous les contrats FONDATION  
**Type :** Guide d'implÃ©mentation informatif

---

## 9. ConformitÃ© MSCM/MIP

### 9.1 Obligation de balisage MSCM

Tout code implÃ©mentÃ© pour Ever Buddy DOIT Ãªtre balisÃ© selon le protocole MSCM v1.

**RÃ©fÃ©rence :** [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md)

**Obligations minimales :**
- Chaque bloc fonctionnel DOIT avoir un identifiant unique (`@id`)
- Le rÃ´le sÃ©mantique DOIT Ãªtre explicite (`@role`)
- La couche architecturale DOIT Ãªtre dÃ©clarÃ©e (`@layer`)
- Une description humaine DOIT accompagner chaque bloc (`@human`)

### 9.2 IntÃ©gration MIP

AprÃ¨s implÃ©mentation, l'index MIP DOIT Ãªtre rÃ©gÃ©nÃ©rÃ© pour :
- Valider l'intÃ©gritÃ© des blocs MSCM
- Mettre Ã  jour le graphe de dÃ©pendances
- VÃ©rifier la cohÃ©rence hiÃ©rarchique

### 9.3 Check-list MSCM

Avant toute livraison, vÃ©rifier :
- [ ] Tous les blocs critiques sont balisÃ©s MSCM
- [ ] Les identifiants sont uniques globalement
- [ ] Les couches (layer) sont cohÃ©rentes avec l'architecture
- [ ] L'index MIP peut Ãªtre rÃ©gÃ©nÃ©rÃ© sans erreur

---

## 10. Mini log â€” erreurs / warnings / arbitrages rencontrÃ©s

### Arbitrage A1 : Niveau de dÃ©tail des exemples

**Arbitrage rencontrÃ© :** Quel niveau de dÃ©tail donner aux exemples sans prescrire d'implÃ©mentation technique ?

**DÃ©cision prise :** Les exemples restent purement conceptuels et narratifs. Aucun code, aucune structure de donnÃ©es spÃ©cifique.

**Justification :** Ce document est informatif et non normatif. Les choix techniques appartiennent aux Ã©quipes d'implÃ©mentation.

**Documentation :** Sections 4 (anti-patterns) et 5 (bonnes pratiques) avec exemples conceptuels uniquement.

### Arbitrage A2 : RÃ©fÃ©rences aux autres cores

**Arbitrage rencontrÃ© :** Comment rÃ©fÃ©rencer les interactions avec les autres cores sans crÃ©er de dÃ©pendances documentaires ?

**DÃ©cision prise :** RÃ©fÃ©rences gÃ©nÃ©riques aux responsabilitÃ©s des autres cores (KindMother pour les donnÃ©es, StrongFather pour les dÃ©cisions) sans lier Ã  des documents spÃ©cifiques de ces cores.

**Justification :** Permet l'Ã©volution indÃ©pendante des documentations tout en prÃ©servant la cohÃ©rence conceptuelle.

**Documentation :** Sections 2, 4, 6 avec rÃ©fÃ©rences gÃ©nÃ©riques.

### Arbitrage A3 : Check-list exhaustive vs utilisable

**Arbitrage rencontrÃ© :** La check-list des 12 invariants est-elle trop longue pour Ãªtre utilisable ?

**DÃ©cision prise :** Conserver la liste complÃ¨te car chaque invariant est non nÃ©gociable. La longueur reflÃ¨te la complexitÃ© rÃ©elle des contraintes.

**Justification :** Omettre des invariants de la check-list risquerait de les faire oublier. La vÃ©rification systÃ©matique est prÃ©fÃ©rable Ã  une simplification dangereuse.

**Documentation :** Section 7 avec les 12 invariants listÃ©s.

### Arbitrage A4 : Anti-patterns spÃ©cifiques vs gÃ©nÃ©riques

**Arbitrage rencontrÃ© :** Fournir des anti-patterns trÃ¨s spÃ©cifiques (qui pourraient devenir obsolÃ¨tes) ou gÃ©nÃ©riques (qui pourraient Ãªtre trop abstraits) ?

**DÃ©cision prise :** Anti-patterns gÃ©nÃ©riques mais illustrÃ©s par des exemples conceptuels spÃ©cifiques.

**Justification :** Les anti-patterns gÃ©nÃ©riques restent valides dans le temps. Les exemples concrets aident Ã  la comprÃ©hension sans prescrire d'implÃ©mentation.

**Documentation :** Section 5 avec anti-patterns gÃ©nÃ©riques et exemples conceptuels.

---

*Aucune autre erreur, warning, ou arbitrage rencontrÃ© lors de la rÃ©daction de ce document.*

