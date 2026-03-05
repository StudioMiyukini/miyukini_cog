# Border Guard â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter Border Guard correctement, sans violer les contrats FONDATION.

**Objectif pÃ©dagogique :** Ce document vise Ã  aider les dÃ©veloppeurs Ã  comprendre comment traduire les contrats FONDATION en implÃ©mentation, en respectant strictement les invariants, garanties, et interdictions.

**Avertissement :** Ce document ne doit pas Ãªtre interprÃ©tÃ© abusivement. Il ne crÃ©e aucune nouvelle rÃ¨gle contractuelle et ne modifie aucun contrat existant. Les contrats FONDATION priment toujours sur ce guide.

**Relation avec les contrats FONDATION :** Ce document fait rÃ©fÃ©rence aux contrats FONDATION existants mais ne les Ã©tend pas, ne les modifie pas, et ne crÃ©e aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1. Objectif

Ce document fournit des lignes directrices pour implÃ©menter Border Guard de maniÃ¨re conforme aux contrats FONDATION. Il explique comment traduire les concepts contractuels en logique d'implÃ©mentation sans interprÃ©tation abusive.

### 1.2. Nature informative

Ce document est **purement informatif**. Il ne dÃ©finit pas de nouvelles rÃ¨gles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la comprÃ©hension et l'application des contrats FONDATION.

### 1.3. Rappel de la mission de Border Guard

Border Guard est le **core de dÃ©finition des frontiÃ¨res et des rÃ¨gles d'entrÃ©e/sortie** du Miyukini Core System. Il rÃ©pond Ã  la question fondamentale :

> **"OÃ¹ sont les frontiÃ¨res du systÃ¨me, et quelles rÃ¨gles gouvernent leur franchissement ?"**

Border Guard **dÃ©finit, classifie, et Ã©tablit des rÃ¨gles**. Il **ne filtre jamais**, **ne bloque jamais**, **n'exÃ©cute jamais**, et **ne dÃ©cide jamais**.

### 1.4. Sources contractuelles

Ce document se base sur les contrats FONDATION, avec un focus particulier sur :

- **Documentation Fondatrice** : Invariants INV-BG-1 Ã  INV-BG-10, responsabilitÃ©s exclusives, interdictions
- **Boundary Definition Contract** : Types de frontiÃ¨res, propriÃ©tÃ©s, taxonomie
- **Trust Level Classification Contract** : Niveaux de confiance (trusted, verified, unknown, hostile)
- **Crossing Rules Contract** : RÃ¨gles dÃ©claratives de franchissement
- **Invariants & Guarantees** : Garanties structurelles non nÃ©gociables
- **Violations & Anti-Patterns** : Ce qu'il ne faut jamais faire
- **[Miyukini Conceptual References â€” Lois Autonomie SystÃ¨me](..//..//..//miyukini-webway-system//reference//_index.md)** : Les lignes directrices d'implÃ©mentation doivent respecter les 6 lois d'autonomie, notamment **LOI-1** (aucune dÃ©pendance externe critique), **LOI-6** (fÃ©dÃ©ration explicite et rÃ©versible).

---

## 2. Principes gÃ©nÃ©raux Ã  respecter absolument

### 2.1. Aucune capacitÃ© d'exÃ©cution (INV-BG-1)

**Principe contractuel :**

L'invariant INV-BG-1 Ã©tablit que Border Guard ne possÃ¨de **jamais** la capacitÃ© d'exÃ©cuter une action : filtrage, blocage, interception, application. Il dÃ©finit les rÃ¨gles et classifie les sources, mais toute exÃ©cution est dÃ©lÃ©guÃ©e aux autoritÃ©s compÃ©tentes.

**Traduction en logique d'implÃ©mentation :**

- **Border Guard DÃ‰FINIT** : Il Ã©tablit les frontiÃ¨res, les rÃ¨gles de franchissement, les niveaux de confiance.
- **Border Guard CLASSIFIE** : Il attribue les niveaux de confiance aux sources et destinations.
- **Border Guard NE FAIT JAMAIS** : Il ne filtre pas, ne bloque pas, n'intercepte pas, n'applique pas.

**Ce que cela signifie concrÃ¨tement :**

- Aucun mÃ©canisme de filtrage ne doit Ãªtre accessible Ã  Border Guard
- Le blocage est exÃ©cutÃ© par BondingBrother (selon dÃ©cision de StrongFather)
- Border Guard fournit les rÃ¨gles dÃ©claratives â€” jamais l'exÃ©cution

### 2.2. Aucune persistance directe (INV-BG-2)

**Principe contractuel :**

L'invariant INV-BG-2 Ã©tablit que Border Guard n'accÃ¨de **jamais** directement Ã  la persistance. Toute dÃ©finition de frontiÃ¨re ou de rÃ¨gle qui doit Ãªtre persistÃ©e est transmise Ã  KindMother via les canaux appropriÃ©s.

**Traduction en logique d'implÃ©mentation :**

- **Pas d'accÃ¨s DB** : Border Guard ne contient pas de drivers de base de donnÃ©es.
- **Pas d'Ã©criture fichier** : Border Guard n'Ã©crit jamais sur le systÃ¨me de fichiers.
- **DÃ©lÃ©gation Ã  KindMother** : Les dÃ©finitions Ã  persister sont transmises Ã  KindMother.

**Ce que cela signifie concrÃ¨tement :**

- Les dÃ©finitions de frontiÃ¨res sont maintenues en mÃ©moire par Border Guard
- La persistance est dÃ©lÃ©guÃ©e via Ã©vÃ©nements ou canaux vers KindMother
- Aucune importation de bibliothÃ¨ques de persistance dans Border Guard

### 2.3. Aucune dÃ©cision autonome (INV-BG-3)

**Principe contractuel :**

L'invariant INV-BG-3 Ã©tablit que Border Guard ne prend **jamais** de dÃ©cision de maniÃ¨re autonome. Il informe, il classifie, il dÃ©finit, mais la dÃ©cision finale appartient toujours Ã  StrongFather ou aux autoritÃ©s appropriÃ©es.

**Traduction en logique d'implÃ©mentation :**

- **Border Guard INFORME** : Il fournit le contexte de confiance Ã  StrongFather.
- **Border Guard CLASSIFIE** : Il attribue un niveau de confiance (trusted, verified, unknown, hostile).
- **Border Guard NE DÃ‰CIDE JAMAIS** : La dÃ©cision d'accepter ou refuser appartient Ã  StrongFather.

**Ce que cela signifie concrÃ¨tement :**

- Aucune mÃ©thode `decide()`, `allow()`, `deny()` dans Border Guard
- Les classifications sont des informations, pas des verdicts
- StrongFather consulte Border Guard, puis dÃ©cide

### 2.4. Classification exhaustive (INV-BG-4)

**Principe contractuel :**

L'invariant INV-BG-4 Ã©tablit que toute source, destination, ou interaction **doit** Ãªtre classifiÃ©e selon un niveau de confiance. Aucune interaction ne peut exister sans classification. Par dÃ©faut, tout ce qui n'est pas explicitement classifiÃ© est considÃ©rÃ© comme "unknown".

**Traduction en logique d'implÃ©mentation :**

- **Classification systÃ©matique** : Toute source qui traverse une frontiÃ¨re a un niveau de confiance.
- **DÃ©faut = unknown** : Si pas de classification explicite, le niveau est "unknown".
- **Aucune exception** : Pas de traitement "sans classification".

**Ce que cela signifie concrÃ¨tement :**

- Toute API de Border Guard retourne un niveau de confiance (jamais null)
- Le niveau "unknown" est le dÃ©faut sÃ©curitaire
- Les rÃ¨gles de franchissement s'appliquent selon le niveau retournÃ©

### 2.5. FrontiÃ¨res explicites (INV-BG-5)

**Principe contractuel :**

L'invariant INV-BG-5 Ã©tablit que toute frontiÃ¨re **doit** Ãªtre explicitement dÃ©finie et documentÃ©e. Aucune frontiÃ¨re implicite n'est autorisÃ©e. Si une dÃ©marcation existe dans le systÃ¨me, elle doit Ãªtre formalisÃ©e par Border Guard.

**Traduction en logique d'implÃ©mentation :**

- **Registre exhaustif** : Toutes les frontiÃ¨res sont dans le registre de Border Guard.
- **DÃ©finition formelle** : Chaque frontiÃ¨re a un identifiant, un type, une direction, une permÃ©abilitÃ©.
- **Pas de frontiÃ¨re cachÃ©e** : Aucun contrÃ´le de franchissement sans frontiÃ¨re dÃ©finie.

**Ce que cela signifie concrÃ¨tement :**

- Le registre des frontiÃ¨res est la source de vÃ©ritÃ©
- Toute demande de rÃ¨gles pour une frontiÃ¨re non dÃ©finie retourne NOT_FOUND
- Les frontiÃ¨res sont crÃ©Ã©es explicitement, jamais infÃ©rÃ©es

### 2.6. RÃ¨gles dÃ©claratives (INV-BG-6)

**Principe contractuel :**

L'invariant INV-BG-6 Ã©tablit que toutes les rÃ¨gles de franchissement **doivent** Ãªtre dÃ©claratives. Aucune rÃ¨gle procÃ©durale ou impÃ©rative n'est autorisÃ©e. Une rÃ¨gle exprime ce qui est requis, pas comment le vÃ©rifier.

**Traduction en logique d'implÃ©mentation :**

- **Conditions, pas procÃ©dures** : "Authentification requise" plutÃ´t que "VÃ©rifier le token JWT".
- **Ce qui est requis, pas comment** : "Niveau verified minimum" plutÃ´t que "Appeler le service auth".
- **NeutralitÃ© technique** : Les rÃ¨gles ne rÃ©fÃ©rencent pas de technologies spÃ©cifiques.

**Ce que cela signifie concrÃ¨tement :**

- Les rÃ¨gles sont des expressions de conditions
- L'implÃ©mentation technique des vÃ©rifications appartient Ã  BondingBrother
- Border Guard ne contient jamais de code de vÃ©rification

### 2.7. SÃ©paration dÃ©finition/application (INV-BG-7)

**Principe contractuel :**

L'invariant INV-BG-7 Ã©tablit que la dÃ©finition des frontiÃ¨res et des rÃ¨gles est **strictement sÃ©parÃ©e** de leur application. Border Guard dÃ©finit, BondingBrother applique. Cette sÃ©paration est non nÃ©gociable et ne peut Ãªtre contournÃ©e.

**Traduction en logique d'implÃ©mentation :**

- **Interface claire** : Border Guard expose des APIs de consultation (GET), pas d'action.
- **Contrat d'interface** : Les rÃ¨gles fournies sont dÃ©claratives, l'application est libre.
- **IndÃ©pendance** : Border Guard peut Ã©voluer sans modifier BondingBrother, et inversement.

**Ce que cela signifie concrÃ¨tement :**

- Border Guard fournit des rÃ¨gles via des consultations
- BondingBrother implÃ©mente la vÃ©rification technique de ces rÃ¨gles
- Aucune dÃ©pendance circulaire entre Border Guard et BondingBrother

### 2.8. TraÃ§abilitÃ© complÃ¨te (INV-BG-8)

**Principe contractuel :**

L'invariant INV-BG-8 Ã©tablit que toute dÃ©finition de frontiÃ¨re, toute classification de confiance, toute rÃ¨gle Ã©tablie **doit** Ãªtre traÃ§able avec son origine, sa date, et sa justification.

**Traduction en logique d'implÃ©mentation :**

- **MÃ©tadonnÃ©es obligatoires** : `createdAt`, `createdBy`, `justification`, `version`.
- **Historique** : Les modifications sont tracÃ©es.
- **Audit possible** : Toute dÃ©finition peut Ãªtre auditÃ©e.

**Ce que cela signifie concrÃ¨tement :**

- Chaque frontiÃ¨re, rÃ¨gle, classification a des mÃ©tadonnÃ©es complÃ¨tes
- L'historique des modifications est conservÃ©
- Les consultations peuvent inclure l'origine de la dÃ©finition

### 2.9. CohÃ©rence globale (INV-BG-9)

**Principe contractuel :**

L'invariant INV-BG-9 Ã©tablit que les dÃ©finitions de Border Guard **doivent** Ãªtre globalement cohÃ©rentes. Aucune contradiction entre frontiÃ¨res, niveaux de confiance, ou rÃ¨gles n'est autorisÃ©e.

**Traduction en logique d'implÃ©mentation :**

- **Validation Ã  la crÃ©ation** : Toute nouvelle dÃ©finition est validÃ©e contre l'existant.
- **Pas de contradiction** : Deux rÃ¨gles ne peuvent pas avoir des rÃ©sultats opposÃ©s.
- **HiÃ©rarchie respectÃ©e** : Les zones de confiance sont cohÃ©rentes entre elles.

**Ce que cela signifie concrÃ¨tement :**

- Un mÃ©canisme de validation de cohÃ©rence existe
- Les crÃ©ations qui crÃ©ent des contradictions sont rejetÃ©es
- La cohÃ©rence globale peut Ãªtre auditÃ©e

### 2.10. NeutralitÃ© conceptuelle (INV-BG-10)

**Principe contractuel :**

L'invariant INV-BG-10 Ã©tablit que Border Guard **ne fait jamais** de supposition sur la technologie d'implÃ©mentation. Les dÃ©finitions sont purement conceptuelles et peuvent Ãªtre implÃ©mentÃ©es par n'importe quelle technologie.

**Traduction en logique d'implÃ©mentation :**

- **Pas de rÃ©fÃ©rence technique** : "Authentification requise", pas "JWT RS256 requis".
- **PortabilitÃ©** : Les dÃ©finitions fonctionnent indÃ©pendamment de la stack technique.
- **Abstraction** : "DonnÃ©es chiffrÃ©es", pas "AES-256-GCM".

**Ce que cela signifie concrÃ¨tement :**

- Aucune bibliothÃ¨que technique (crypto, auth, rÃ©seau) importÃ©e dans Border Guard
- Les dÃ©finitions sont des contrats conceptuels
- L'implÃ©mentation technique est du ressort des adaptateurs

---

## 3. Comment traduire les contrats en logique sans interprÃ©tation abusive

### 3.1. Respecter les invariants comme contraintes absolues

**Principe :**

Les invariants contractuels (INV-BG-*) sont des contraintes absolues qui DOIVENT toujours Ãªtre vraies. Ils ne sont pas des suggestions ou des recommandations.

**Traduction :**

- **VÃ©rification systÃ©matique** : Chaque invariant DOIT Ãªtre vÃ©rifiÃ© Ã  chaque opÃ©ration.
- **PrÃ©servation garantie** : Toute opÃ©ration DOIT garantir que les invariants sont prÃ©servÃ©s aprÃ¨s exÃ©cution.
- **Pas d'interprÃ©tation** : Les invariants ne peuvent pas Ãªtre interprÃ©tÃ©s ou adaptÃ©s.

**Exemple conceptuel :**

Si l'invariant INV-BG-1 (aucune capacitÃ© d'exÃ©cution) interdit le filtrage, alors aucune mÃ©thode de filtrage n'est possible dans Border Guard, mÃªme pour des raisons "pratiques" ou de "performance".

### 3.2. SÃ©parer strictement dÃ©finition et application

**Principe :**

La sÃ©paration entre dÃ©finition (Border Guard) et application (BondingBrother) est fondamentale. C'est la rÃ¨gle structurante de toute l'architecture.

**Traduction :**

- **Border Guard = Quoi** : Quelles sont les rÃ¨gles ? Quel niveau de confiance ?
- **BondingBrother = Comment** : Comment vÃ©rifier techniquement ces rÃ¨gles ?
- **Aucun chevauchement** : Border Guard ne fait jamais le travail de BondingBrother.

**Exemple conceptuel :**

Border Guard dÃ©finit : "La source doit Ãªtre authentifiÃ©e avec un niveau verified minimum."
BondingBrother applique : "VÃ©rifier le token JWT, valider la signature, vÃ©rifier l'expiration."

### 3.3. Traiter la classification comme un service d'information

**Principe :**

La classification de confiance est une information, pas une dÃ©cision. Border Guard informe du niveau de confiance ; il ne dÃ©cide pas des consÃ©quences.

**Traduction :**

- **Information pure** : `getTrustLevel(source)` retourne un niveau (trusted, verified, unknown, hostile).
- **Pas de verdict** : Border Guard ne dit pas "bloquÃ©" ou "autorisÃ©".
- **StrongFather dÃ©cide** : La dÃ©cision basÃ©e sur le niveau appartient Ã  StrongFather.

**Exemple conceptuel :**

Border Guard : "Cette source est classifiÃ©e 'hostile'."
StrongFather : "Je dÃ©cide de bloquer cette source."
BondingBrother : "J'exÃ©cute le blocage."

### 3.4. ImplÃ©menter la traÃ§abilitÃ© comme obligation structurelle

**Principe :**

La traÃ§abilitÃ© n'est pas une fonctionnalitÃ© optionnelle. C'est une obligation structurelle (INV-BG-8) qui s'applique Ã  toute dÃ©finition.

**Traduction :**

- **MÃ©tadonnÃ©es obligatoires** : Toute crÃ©ation ou modification a des mÃ©tadonnÃ©es.
- **Pas d'exception** : MÃªme les dÃ©finitions "triviales" sont traÃ§ables.
- **Historique immuable** : Les traces ne peuvent pas Ãªtre supprimÃ©es.

**Exemple conceptuel :**

MÃªme une frontiÃ¨re interne "technique" entre deux modules doit Ãªtre traÃ§able avec qui l'a crÃ©Ã©e, quand, et pourquoi.

---

## 4. Ce qu'un dÃ©veloppeur ne doit jamais faire

### 4.1. ExÃ©cuter un filtrage ou un blocage (INV-BG-1)

**Interdiction contractuelle :**

L'invariant INV-BG-1 Ã©tablit que Border Guard ne possÃ¨de **jamais** la capacitÃ© d'exÃ©cuter une action.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- ImplÃ©menter des mÃ©thodes `filter()`, `block()`, `intercept()` dans Border Guard
- Permettre Ã  Border Guard de rejeter directement une requÃªte
- CrÃ©er des middlewares d'exÃ©cution dans Border Guard
- Lancer des exceptions de blocage depuis Border Guard

**ConsÃ©quence de la violation :**

- Violation de l'invariant INV-BG-1 (aucune exÃ©cution)
- Violation de la sÃ©paration dÃ©finition / application
- Compromission de l'architecture fondamentale

### 4.2. AccÃ©der directement Ã  la persistance (INV-BG-2)

**Interdiction contractuelle :**

L'invariant INV-BG-2 Ã©tablit que Border Guard n'accÃ¨de **jamais** directement Ã  la persistance.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- Importer des drivers de base de donnÃ©es dans Border Guard
- Ã‰crire des queries SQL ou NoSQL dans Border Guard
- AccÃ©der au systÃ¨me de fichiers pour persister des dÃ©finitions
- ImplÃ©menter un cache persistÃ© dans Border Guard

**ConsÃ©quence de la violation :**

- Violation de l'invariant INV-BG-2 (aucune persistance directe)
- Violation de la souverainetÃ© de KindMother sur les donnÃ©es
- Risque de dÃ©synchronisation

### 4.3. Prendre des dÃ©cisions autonomes (INV-BG-3)

**Interdiction contractuelle :**

L'invariant INV-BG-3 Ã©tablit que Border Guard ne prend **jamais** de dÃ©cision de maniÃ¨re autonome.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- ImplÃ©menter des mÃ©thodes `decide()`, `allow()`, `deny()` dans Border Guard
- Retourner des verdicts (accept/reject) depuis Border Guard
- CrÃ©er des logiques if/else dÃ©cisionnelles dans Border Guard
- Ã‰mettre des dÃ©cisions d'autorisation depuis Border Guard

**ConsÃ©quence de la violation :**

- Violation de l'invariant INV-BG-3 (aucune dÃ©cision autonome)
- Usurpation du rÃ´le de StrongFather
- Compromission de l'architecture de gouvernance

### 4.4. ImplÃ©menter des rÃ¨gles procÃ©durales (INV-BG-6)

**Interdiction contractuelle :**

L'invariant INV-BG-6 Ã©tablit que les rÃ¨gles **doivent** Ãªtre dÃ©claratives.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- Ã‰crire des rÃ¨gles qui dÃ©crivent "comment faire" plutÃ´t que "ce qui est requis"
- Inclure du pseudo-code ou des sÃ©quences d'Ã©tapes dans les rÃ¨gles
- RÃ©fÃ©rencer des technologies spÃ©cifiques dans les rÃ¨gles
- CrÃ©er des rÃ¨gles qui contiennent de la logique d'exÃ©cution

**ConsÃ©quence de la violation :**

- Violation de l'invariant INV-BG-6 (rÃ¨gles dÃ©claratives)
- Couplage avec l'implÃ©mentation technique
- ImpossibilitÃ© de portage vers d'autres technologies

### 4.5. CrÃ©er des frontiÃ¨res implicites (INV-BG-5)

**Interdiction contractuelle :**

L'invariant INV-BG-5 Ã©tablit que les frontiÃ¨res **doivent** Ãªtre explicites.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- VÃ©rifier des permissions sans frontiÃ¨re dÃ©finie formellement
- CrÃ©er des zones de confiance implicites dans le code
- Ajouter des points de contrÃ´le non rÃ©fÃ©rencÃ©s dans Border Guard
- InfÃ©rer l'existence de frontiÃ¨res depuis le comportement du systÃ¨me

**ConsÃ©quence de la violation :**

- Violation de l'invariant INV-BG-5 (frontiÃ¨res explicites)
- IncohÃ©rence de sÃ©curitÃ©
- ImpossibilitÃ© d'audit complet

### 4.6. Omettre la traÃ§abilitÃ© (INV-BG-8)

**Interdiction contractuelle :**

L'invariant INV-BG-8 Ã©tablit que toute dÃ©finition **doit** Ãªtre traÃ§able.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- CrÃ©er des dÃ©finitions sans mÃ©tadonnÃ©es (createdAt, createdBy, justification)
- Modifier des dÃ©finitions sans tracer la modification
- Supprimer l'historique des dÃ©finitions
- Omettre la justification pour des dÃ©finitions "Ã©videntes"

**ConsÃ©quence de la violation :**

- Violation de l'invariant INV-BG-8 (traÃ§abilitÃ© complÃ¨te)
- ImpossibilitÃ© d'audit
- Perte de responsabilitÃ© attribuable

---

## 5. Anti-patterns classiques

### 5.1. Anti-pattern 1 : Filtrage intÃ©grÃ©

**Description :**

Tentative d'implÃ©menter des mÃ©canismes de filtrage directement dans Border Guard.

**Exemple conceptuel :**

Un dÃ©veloppeur crÃ©e une mÃ©thode `filterIncomingRequests()` dans Border Guard qui rejette les requÃªtes non conformes aux rÃ¨gles.

**ConsÃ©quence :**

- Violation de l'invariant INV-BG-1 (aucune exÃ©cution)
- Violation de l'invariant INV-BG-7 (sÃ©paration dÃ©finition/application)
- Couplage dangereux entre dÃ©finition et exÃ©cution

**Correction :**

Border Guard dÃ©finit les rÃ¨gles. BondingBrother les consulte et exÃ©cute le filtrage.

### 5.2. Anti-pattern 2 : DÃ©cision cachÃ©e

**Description :**

Tentative de prendre des dÃ©cisions de maniÃ¨re dÃ©guisÃ©e en classification.

**Exemple conceptuel :**

Un dÃ©veloppeur crÃ©e une mÃ©thode `isAllowed(source)` qui retourne `true` ou `false` plutÃ´t qu'un niveau de confiance.

**ConsÃ©quence :**

- Violation de l'invariant INV-BG-3 (aucune dÃ©cision autonome)
- Border Guard usurpe le rÃ´le de StrongFather
- DÃ©cisions prises sans vision globale

**Correction :**

Border Guard retourne `getTrustLevel(source)` qui retourne un niveau de confiance. StrongFather dÃ©cide si ce niveau permet l'action.

### 5.3. Anti-pattern 3 : RÃ¨gles techniques

**Description :**

Tentative de dÃ©finir des rÃ¨gles qui incluent des dÃ©tails d'implÃ©mentation technique.

**Exemple conceptuel :**

Un dÃ©veloppeur crÃ©e une rÃ¨gle "Le token JWT doit Ãªtre signÃ© avec RS256 et avoir un claim 'role' Ã©gal Ã  'admin'".

**ConsÃ©quence :**

- Violation de l'invariant INV-BG-6 (rÃ¨gles dÃ©claratives)
- Violation de l'invariant INV-BG-10 (neutralitÃ© conceptuelle)
- Couplage avec une technologie spÃ©cifique

**Correction :**

La rÃ¨gle devient "Authentification requise avec niveau de privilÃ¨ge administrateur". L'implÃ©mentation technique (JWT, SAML, session...) appartient aux adaptateurs.

### 5.4. Anti-pattern 4 : FrontiÃ¨re Ã  la volÃ©e

**Description :**

Tentative de crÃ©er des frontiÃ¨res dynamiquement au moment du besoin sans les formaliser.

**Exemple conceptuel :**

Un dÃ©veloppeur crÃ©e une vÃ©rification de permission inline dans le code produit, sans frontiÃ¨re dÃ©finie dans Border Guard.

**ConsÃ©quence :**

- Violation de l'invariant INV-BG-5 (frontiÃ¨res explicites)
- FrontiÃ¨res fantÃ´mes non auditables
- IncohÃ©rence de sÃ©curitÃ©

**Correction :**

Toute dÃ©marcation de confiance est d'abord dÃ©finie formellement dans Border Guard, puis utilisÃ©e.

### 5.5. Anti-pattern 5 : Persistance directe

**Description :**

Tentative de persister les dÃ©finitions directement depuis Border Guard.

**Exemple conceptuel :**

Un dÃ©veloppeur ajoute un appel `await db.boundaries.insert(boundary)` dans Border Guard.

**ConsÃ©quence :**

- Violation de l'invariant INV-BG-2 (aucune persistance directe)
- Violation de la souverainetÃ© de KindMother
- Risque de dÃ©synchronisation

**Correction :**

Border Guard Ã©met un Ã©vÃ©nement `boundary-defined`. KindMother Ã©coute et persiste.

### 5.6. Anti-pattern 6 : Classification sans dÃ©faut

**Description :**

Tentative de traiter des sources sans les classifier explicitement.

**Exemple conceptuel :**

Un dÃ©veloppeur crÃ©e un chemin de code qui traite une source sans appeler `getTrustLevel()`, assumant qu'elle est de confiance.

**ConsÃ©quence :**

- Violation de l'invariant INV-BG-4 (classification exhaustive)
- Faille de sÃ©curitÃ© potentielle
- Sources non classifiÃ©es traitÃ©es comme de confiance

**Correction :**

Toute source est classifiÃ©e. Si non classifiÃ©e explicitement, le dÃ©faut est "unknown".

---

## 6. Bonnes pratiques conceptuelles

### 6.1. Registre de frontiÃ¨res centralisÃ©

**Pratique :**

Maintenir un registre centralisÃ© des frontiÃ¨res, accessible en lecture par tous les cores mais modifiable uniquement par Border Guard.

**Justification :**

- Respecte l'autoritÃ© exclusive de Border Guard sur les frontiÃ¨res (INV-BG-5)
- Garantit l'unicitÃ© et l'exhaustivitÃ© des dÃ©finitions
- Facilite la consultation par les autres cores

**ImplÃ©mentation conceptuelle :**

- Registre en mÃ©moire avec toutes les frontiÃ¨res dÃ©finies
- API de lecture accessible aux autres cores
- API de modification rÃ©servÃ©e aux canaux autorisÃ©s
- Synchronisation de la persistance via KindMother

### 6.2. Classificateur de confiance avec dÃ©faut sÃ©curitaire

**Pratique :**

ImplÃ©menter le classificateur de confiance avec un dÃ©faut "unknown" systÃ©matique pour toute source non explicitement classifiÃ©e.

**Justification :**

- Respecte l'invariant INV-BG-4 (classification exhaustive)
- Garantit un comportement sÃ©curitaire par dÃ©faut
- EmpÃªche les failles par omission de classification

**ImplÃ©mentation conceptuelle :**

- Toute requÃªte de classification retourne un niveau
- Si pas de classification explicite, retour de "unknown"
- Le niveau "unknown" dÃ©clenche les rÃ¨gles restrictives par dÃ©faut

### 6.3. RÃ¨gles structurÃ©es en conditions dÃ©claratives

**Pratique :**

Structurer les rÃ¨gles de franchissement comme des ensembles de conditions dÃ©claratives, sans logique procÃ©durale.

**Justification :**

- Respecte l'invariant INV-BG-6 (rÃ¨gles dÃ©claratives)
- Respecte l'invariant INV-BG-10 (neutralitÃ© conceptuelle)
- Permet l'implÃ©mentation technique libre par BondingBrother

**ImplÃ©mentation conceptuelle :**

- RÃ¨gle = liste de conditions Ã  satisfaire
- Chaque condition est une expression dÃ©clarative
- Pas de verbes d'action, pas de sÃ©quences d'Ã©tapes
- BondingBrother traduit en vÃ©rifications techniques

### 6.4. Validation de cohÃ©rence Ã  chaque modification

**Pratique :**

Valider la cohÃ©rence globale des dÃ©finitions Ã  chaque crÃ©ation ou modification.

**Justification :**

- Respecte l'invariant INV-BG-9 (cohÃ©rence globale)
- EmpÃªche les contradictions entre rÃ¨gles
- Garantit un comportement prÃ©visible

**ImplÃ©mentation conceptuelle :**

- Avant toute crÃ©ation : vÃ©rification de non-contradiction
- Avant toute modification : vÃ©rification d'impact sur la cohÃ©rence
- Rejet des modifications qui crÃ©ent des incohÃ©rences
- Audit pÃ©riodique de la cohÃ©rence globale

### 6.5. MÃ©tadonnÃ©es de traÃ§abilitÃ© systÃ©matiques

**Pratique :**

Inclure systÃ©matiquement les mÃ©tadonnÃ©es de traÃ§abilitÃ© sur chaque dÃ©finition.

**Justification :**

- Respecte l'invariant INV-BG-8 (traÃ§abilitÃ© complÃ¨te)
- Permet l'audit et l'attribution de responsabilitÃ©
- Facilite la comprÃ©hension des dÃ©cisions passÃ©es

**ImplÃ©mentation conceptuelle :**

- Chaque dÃ©finition inclut : `createdAt`, `createdBy`, `justification`, `version`
- L'historique des modifications est conservÃ©
- Les consultations peuvent inclure les mÃ©tadonnÃ©es

### 6.6. SÃ©paration claire des interfaces

**Pratique :**

Exposer des interfaces distinctes pour chaque type de consommateur (StrongFather, BondingBrother, CaringNanny).

**Justification :**

- Respecte les contrats d'intÃ©gration avec chaque core
- EmpÃªche les usages non prÃ©vus
- Facilite l'Ã©volution indÃ©pendante

**ImplÃ©mentation conceptuelle :**

- Interface StrongFather : contexte de confiance pour dÃ©cision
- Interface BondingBrother : rÃ¨gles de franchissement pour application
- Interface CaringNanny : Ã©tat des frontiÃ¨res pour observation

---

## 7. Check-list mentale avant toute feature

Avant d'implÃ©menter une nouvelle fonctionnalitÃ© liÃ©e Ã  Border Guard, un dÃ©veloppeur DOIT vÃ©rifier mentalement :

### 7.1. VÃ©rification des invariants d'identitÃ©

- **INV-BG-1 est-il prÃ©servÃ© ?** : La fonctionnalitÃ© n'exÃ©cute-t-elle aucune action (filtrage, blocage, interception) ?
- **INV-BG-3 est-il prÃ©servÃ© ?** : La fonctionnalitÃ© ne prend-elle aucune dÃ©cision autonome ?

### 7.2. VÃ©rification des invariants de comportement

- **INV-BG-2 est-il prÃ©servÃ© ?** : La fonctionnalitÃ© n'accÃ¨de-t-elle pas directement Ã  la persistance ?
- **INV-BG-4 est-il prÃ©servÃ© ?** : Toute source est-elle classifiÃ©e (dÃ©faut = unknown) ?
- **INV-BG-5 est-il prÃ©servÃ© ?** : Toute frontiÃ¨re est-elle explicitement dÃ©finie ?
- **INV-BG-6 est-il prÃ©servÃ© ?** : Les rÃ¨gles sont-elles purement dÃ©claratives ?

### 7.3. VÃ©rification des invariants de qualitÃ©

- **INV-BG-7 est-il prÃ©servÃ© ?** : La dÃ©finition est-elle strictement sÃ©parÃ©e de l'application ?
- **INV-BG-8 est-il prÃ©servÃ© ?** : La traÃ§abilitÃ© est-elle complÃ¨te (origine, date, justification) ?
- **INV-BG-9 est-il prÃ©servÃ© ?** : La cohÃ©rence globale est-elle maintenue ?
- **INV-BG-10 est-il prÃ©servÃ© ?** : Aucune supposition technique n'est-elle faite ?

### 7.4. VÃ©rification de la sÃ©paration des responsabilitÃ©s

- **Border Guard reste-t-il conceptuel ?** : La fonctionnalitÃ© dÃ©finit-elle sans exÃ©cuter ?
- **L'autoritÃ© de KindMother est-elle respectÃ©e ?** : Aucune persistance directe ?
- **L'autoritÃ© de StrongFather est-elle respectÃ©e ?** : Aucune dÃ©cision d'autorisation ?
- **L'autoritÃ© de BondingBrother est-elle respectÃ©e ?** : Aucune application de rÃ¨gles ?

### 7.5. VÃ©rification de la conformitÃ© aux Lois d'Autonomie

- **LOI-1 respectÃ©e ?** : Aucune dÃ©pendance externe critique pour les dÃ©finitions ?
- **LOI-2 respectÃ©e ?** : Les frontiÃ¨res fonctionnent-elles en mode isolÃ© ?
- **LOI-6 respectÃ©e ?** : La fÃ©dÃ©ration reste-t-elle explicite, contrÃ´lÃ©e, rÃ©versible ?

### 7.6. VÃ©rification de la traÃ§abilitÃ© et de la cohÃ©rence

- **Toutes les dÃ©finitions sont-elles traÃ§ables ?** : MÃ©tadonnÃ©es complÃ¨tes ?
- **La cohÃ©rence globale est-elle vÃ©rifiÃ©e ?** : Pas de contradiction dÃ©tectable ?
- **L'audit est-il possible ?** : Toute dÃ©finition peut-elle Ãªtre auditÃ©e ?

---

## 8. Interactions avec les autres cores â€” Guide pratique

### 8.1. Interaction avec StrongFather

**Nature de l'interaction :** Border Guard **informe** StrongFather sur le contexte de confiance.

**Ce que Border Guard fournit :**

- Niveau de confiance de la source (trusted, verified, unknown, hostile)
- FrontiÃ¨res traversÃ©es par l'intention
- RÃ¨gles de franchissement applicables
- Ã‰tat des intÃ©grations concernÃ©es

**Ce que Border Guard ne fait JAMAIS :**

- DÃ©cider Ã  la place de StrongFather
- Retourner un verdict (accept/reject)
- Bloquer une intention

**Exemple de flux :**

1. StrongFather Ã©value une intention
2. StrongFather demande Ã  Border Guard : "Quel est le contexte de confiance de cette intention ?"
3. Border Guard retourne : niveau de confiance, frontiÃ¨res, rÃ¨gles
4. StrongFather utilise ces informations pour prendre sa dÃ©cision

### 8.2. Interaction avec BondingBrother

**Nature de l'interaction :** Border Guard **dÃ©finit** les rÃ¨gles que BondingBrother **applique**.

**Ce que Border Guard fournit :**

- RÃ¨gles de franchissement pour chaque frontiÃ¨re
- Niveaux de confiance des sources
- Ã‰tat des intÃ©grations
- FrontiÃ¨res identifiÃ©es entre source et destination

**Ce que Border Guard ne fait JAMAIS :**

- Filtrer les interactions
- Appliquer les rÃ¨gles
- ExÃ©cuter des vÃ©rifications techniques
- Bloquer des accÃ¨s

**Exemple de flux :**

1. BondingBrother reÃ§oit une intention Ã  mÃ©dier
2. BondingBrother demande Ã  Border Guard : "Quelles sont les frontiÃ¨res et les rÃ¨gles ?"
3. Border Guard retourne : frontiÃ¨res traversÃ©es, rÃ¨gles dÃ©claratives
4. BondingBrother applique les rÃ¨gles techniquement

### 8.3. Interaction avec CaringNanny

**Nature de l'interaction :** Border Guard **informe** CaringNanny sur l'Ã©tat des frontiÃ¨res.

**Ce que Border Guard fournit :**

- CrÃ©ation/modification/suppression de frontiÃ¨res
- Changements d'Ã©tat (intÃ©gration suspendue, frontiÃ¨re fermÃ©e)
- Anomalies dÃ©tectÃ©es sur les frontiÃ¨res

**Ce que Border Guard ne fait JAMAIS :**

- Modifier l'Ã©tat global du systÃ¨me
- DÃ©cider de l'Ã©tat de santÃ©
- Agir sur l'Ã©tat observÃ©

**Exemple de flux :**

1. Border Guard dÃ©tecte un changement (intÃ©gration rÃ©voquÃ©e)
2. Border Guard notifie CaringNanny : "L'intÃ©gration X est rÃ©voquÃ©e"
3. CaringNanny intÃ¨gre cette information dans l'Ã©tat global

### 8.4. Interaction avec KindMother

**Nature de l'interaction :** Border Guard **dÃ©lÃ¨gue** la persistance Ã  KindMother.

**Ce que Border Guard transmet :**

- DÃ©finitions de frontiÃ¨res Ã  persister
- Classifications de confiance Ã  stocker
- Historique des modifications

**Ce que Border Guard ne fait JAMAIS :**

- AccÃ©der directement Ã  la base de donnÃ©es
- Ã‰crire des fichiers
- GÃ©rer un cache persistÃ©

**Exemple de flux :**

1. Border Guard crÃ©e une nouvelle frontiÃ¨re
2. Border Guard Ã©met un Ã©vÃ©nement : "FrontiÃ¨re X dÃ©finie"
3. KindMother reÃ§oit l'Ã©vÃ©nement et persiste la dÃ©finition

---

## 9. Conclusion

Ce document fournit des lignes directrices pour implÃ©menter Border Guard de maniÃ¨re conforme aux contrats FONDATION.

**Points clÃ©s :**

- Border Guard **dÃ©finit, classifie, et Ã©tablit des rÃ¨gles** â€” il **n'exÃ©cute jamais**
- Les invariants INV-BG-1 Ã  INV-BG-10 sont des **contraintes absolues**
- La **sÃ©paration dÃ©finition/application** avec BondingBrother est fondamentale
- La **traÃ§abilitÃ© est obligatoire** et la **cohÃ©rence est vÃ©rifiÃ©e**
- Les **Lois d'Autonomie** doivent Ãªtre respectÃ©es

**Nature informative :**

Ce document est purement informatif et ne crÃ©e aucune nouvelle obligation contractuelle. Il sert uniquement Ã  guider la comprÃ©hension et l'application des contrats FONDATION.

**Rappel :** Les contrats FONDATION priment toujours sur ce guide. En cas de doute, se rÃ©fÃ©rer Ã  la Documentation Fondatrice et aux contrats spÃ©cifiques.

**Phrase fondatrice Ã  garder en mÃ©moire :**

> **Border Guard est l'autoritÃ© de dÃ©finition des frontiÃ¨res et des niveaux de confiance qui Ã©tablit les rÃ¨gles de franchissement sans jamais les appliquer lui-mÃªme, sÃ©parant strictement la dÃ©finition conceptuelle de l'exÃ©cution technique.**

---

**Document crÃ©Ã© le :** 2026-01-28  
**Version :** 1.0  
**Statut :** POST-FONDATION â€” Informatif, non normatif, non contractuel  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, Border Guard Documentation Fondatrice, Tous les contrats FONDATION  
**Type :** Guide d'implÃ©mentation informatif

---

## 10. ConformitÃ© MSCM/MIP

### 10.1 Obligation de balisage MSCM

Tout code implÃ©mentÃ© pour Border Guard DOIT Ãªtre balisÃ© selon le protocole MSCM v1.

**RÃ©fÃ©rence :** [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md)

**Obligations minimales :**
- Chaque bloc fonctionnel DOIT avoir un identifiant unique (`@id`)
- Le rÃ´le sÃ©mantique DOIT Ãªtre explicite (`@role`)
- La couche architecturale DOIT Ãªtre dÃ©clarÃ©e (`@layer`)
- Une description humaine DOIT accompagner chaque bloc (`@human`)

### 10.2 IntÃ©gration MIP

AprÃ¨s implÃ©mentation, l'index MIP DOIT Ãªtre rÃ©gÃ©nÃ©rÃ© pour :
- Valider l'intÃ©gritÃ© des blocs MSCM
- Mettre Ã  jour le graphe de dÃ©pendances
- VÃ©rifier la cohÃ©rence hiÃ©rarchique

### 10.3 Check-list MSCM

Avant toute livraison, vÃ©rifier :
- [ ] Tous les blocs critiques sont balisÃ©s MSCM
- [ ] Les identifiants sont uniques globalement
- [ ] Les couches (layer) sont cohÃ©rentes avec l'architecture
- [ ] L'index MIP peut Ãªtre rÃ©gÃ©nÃ©rÃ© sans erreur

---

## 11. Mini log â€” erreurs / warnings / arbitrages rencontrÃ©s

### Arbitrage A1 : Niveau de dÃ©tail des exemples

**Arbitrage rencontrÃ© :** Quel niveau de dÃ©tail donner aux exemples sans prescrire d'implÃ©mentation technique ?

**DÃ©cision prise :** Les exemples restent purement conceptuels et narratifs. Aucun code, aucune structure de donnÃ©es spÃ©cifique.

**Justification :** Ce document est informatif et non normatif. Les choix techniques appartiennent aux Ã©quipes d'implÃ©mentation.

**Documentation :** Sections 5 (anti-patterns) et 6 (bonnes pratiques) avec exemples conceptuels uniquement.

### Arbitrage A2 : RÃ©fÃ©rences aux contrats d'intÃ©gration

**Arbitrage rencontrÃ© :** Comment rÃ©fÃ©rencer les interactions avec les autres cores sans dupliquer les contrats d'intÃ©gration ?

**DÃ©cision prise :** Section 8 fournit un guide pratique des interactions, avec renvoi vers les contrats d'intÃ©gration pour les dÃ©tails.

**Justification :** Permet une comprÃ©hension rapide sans crÃ©er de redondance avec les contrats existants.

**Documentation :** Section 8 avec rÃ©fÃ©rences vers les contrats d'intÃ©gration.

### Arbitrage A3 : Check-list exhaustive vs utilisable

**Arbitrage rencontrÃ© :** La check-list des 10 invariants + vÃ©rifications additionnelles est-elle trop longue ?

**DÃ©cision prise :** Conserver la liste complÃ¨te car chaque invariant est non nÃ©gociable. Organisation en sous-sections pour faciliter la lecture.

**Justification :** Omettre des invariants de la check-list risquerait de les faire oublier. La vÃ©rification systÃ©matique est prÃ©fÃ©rable Ã  une simplification dangereuse.

**Documentation :** Section 7 avec les invariants organisÃ©s par catÃ©gorie (identitÃ©, comportement, qualitÃ©).

### Arbitrage A4 : Anti-patterns spÃ©cifiques vs gÃ©nÃ©riques

**Arbitrage rencontrÃ© :** Fournir des anti-patterns trÃ¨s spÃ©cifiques (qui pourraient devenir obsolÃ¨tes) ou gÃ©nÃ©riques (qui pourraient Ãªtre trop abstraits) ?

**DÃ©cision prise :** Anti-patterns gÃ©nÃ©riques mais illustrÃ©s par des exemples conceptuels spÃ©cifiques, en Ã©vitant le code technique.

**Justification :** Les anti-patterns gÃ©nÃ©riques restent valides dans le temps. Les exemples conceptuels aident Ã  la comprÃ©hension sans prescrire d'implÃ©mentation.

**Documentation :** Section 5 avec 6 anti-patterns et corrections conceptuelles.

---

*Aucune autre erreur, warning, ou arbitrage rencontrÃ© lors de la rÃ©daction de ce document.*

