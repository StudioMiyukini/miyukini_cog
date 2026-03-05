# Master Butler â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter Master Butler correctement, sans violer les contrats FONDATION.

**Objectif pÃ©dagogique :** Ce document vise Ã  aider les dÃ©veloppeurs Ã  comprendre comment traduire les contrats FONDATION en implÃ©mentation, en respectant strictement les invariants, garanties, et interdictions.

**Avertissement :** Ce document ne doit pas Ãªtre interprÃ©tÃ© abusivement. Il ne crÃ©e aucune nouvelle rÃ¨gle contractuelle et ne modifie aucun contrat existant. Les contrats FONDATION priment toujours sur ce guide.

**Relation avec les contrats FONDATION :** Ce document fait rÃ©fÃ©rence aux contrats FONDATION existants mais ne les Ã©tend pas, ne les modifie pas, et ne crÃ©e aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1. Objectif

Ce document fournit des lignes directrices pour implÃ©menter Master Butler de maniÃ¨re conforme aux contrats FONDATION. Il explique comment traduire les concepts contractuels en logique d'implÃ©mentation sans interprÃ©tation abusive.

Master Butler est le **Capability & Permission Core** du Miyukini Core System : il recense les capacitÃ©s, dÃ©finit les permissions, fournit une API de dÃ©couverte, mais **ne dÃ©cide jamais** et **n'exÃ©cute jamais**.

### 1.2. Nature informative

Ce document est **purement informatif**. Il ne dÃ©finit pas de nouvelles rÃ¨gles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la comprÃ©hension et l'application des contrats FONDATION.

### 1.3. Sources contractuelles

Ce document se base sur tous les contrats FONDATION de Master Butler, avec un focus particulier sur :

- **Documentation Fondatrice** : Invariants INV-MB-1 Ã  INV-MB-8, responsabilitÃ©s, interdictions
- **Capability Registry Contract** : ModÃ¨le du registre des capacitÃ©s
- **Permission Registry Contract** : ModÃ¨le du registre des permissions
- **Capability API Contract** : DÃ©claration et interrogation des capacitÃ©s
- **Permission API Contract** : DÃ©finition et gestion des permissions
- **Discovery API Contract** : API de dÃ©couverte
- **Tool Governance Contract** : Gouvernance des Tools et Toolkits
- **[Miyukini Conceptual References â€” Lois Autonomie SystÃ¨me](..//..//..//miyukini-webway-system//reference//_index.md)** : Les lignes directrices d'implÃ©mentation doivent respecter les lois d'autonomie, notamment **LOI-1** (aucune dÃ©pendance externe critique), **LOI-5** (coÃ»t proportionnel au hardware).

---

## 2. Principes gÃ©nÃ©raux Ã  respecter absolument

### 2.1. Registre pur et passif (INV-MB-1)

**Principe contractuel :**

L'invariant INV-MB-1 Ã©tablit que le registre de Master Butler est **exhaustif**. Toute capacitÃ© existant dans le systÃ¨me est recensÃ©e dans Master Butler. Si une capacitÃ© n'est pas dans le registre, elle n'existe pas officiellement dans le systÃ¨me.

**Traduction en logique d'implÃ©mentation :**

- **Registre exhaustif** : Toutes les capacitÃ©s du systÃ¨me DOIVENT Ãªtre prÃ©sentes dans le registre. Aucune capacitÃ© non dÃ©clarÃ©e ne peut exister officiellement.

- **Enregistrement obligatoire** : Aucun module ne peut exposer une capacitÃ© sans la dÃ©clarer Ã  Master Butler. Tout contournement est interdit.

- **Source de vÃ©ritÃ© unique** : Le registre de Master Butler est la seule source de vÃ©ritÃ© pour les capacitÃ©s et permissions. Aucun registre parallÃ¨le n'est autorisÃ©.

**Ce que cela signifie concrÃ¨tement :**

- Tout module DOIT dÃ©clarer ses capacitÃ©s Ã  Master Butler lors de son initialisation
- Les capacitÃ©s non dÃ©clarÃ©es sont considÃ©rÃ©es comme inexistantes
- Le registre est l'unique rÃ©fÃ©rence pour toute interrogation sur les possibilitÃ©s du systÃ¨me

### 2.2. Non-dÃ©cision absolue (INV-MB-2)

**Principe contractuel :**

L'invariant INV-MB-2 Ã©tablit que Master Butler **ne prend jamais de dÃ©cision**. Il fournit des informations, rÃ©pond Ã  des questions, mais ne produit jamais de verdict "autorisÃ©" ou "refusÃ©". Toute dÃ©cision appartient Ã  StrongFather.

**Traduction en logique d'implÃ©mentation :**

- **Information pure** : Master Butler retourne des informations (capacitÃ©s, permissions, associations), jamais des dÃ©cisions.

- **Pas de boolÃ©en d'autorisation** : Aucune mÃ©thode de Master Butler ne retourne un boolÃ©en d'autorisation directe. Les retours sont des informations descriptives.

- **NeutralitÃ© absolue** : Master Butler ne recommande pas, ne suggÃ¨re pas, ne juge pas. Il expose les faits.

**Ce que cela signifie concrÃ¨tement :**

- Une requÃªte "L'utilisateur X a-t-il accÃ¨s ?" DOIT retourner les permissions de X, pas une dÃ©cision oui/non
- StrongFather utilise ces informations pour prendre la dÃ©cision
- Master Butler ne filtre jamais selon une logique de dÃ©cision

### 2.3. Idempotence des dÃ©clarations (INV-MB-3)

**Principe contractuel :**

L'invariant INV-MB-3 Ã©tablit que les dÃ©clarations de capacitÃ©s sont **idempotentes**. DÃ©clarer deux fois la mÃªme capacitÃ© n'a pas d'effet supplÃ©mentaire. Le registre reste cohÃ©rent quel que soit l'ordre ou le nombre de dÃ©clarations.

**Traduction en logique d'implÃ©mentation :**

- **DÃ©duplication automatique** : Une capacitÃ© dÃ©clarÃ©e plusieurs fois ne crÃ©e pas de duplications dans le registre.

- **Ordre indÃ©pendant** : L'ordre des dÃ©clarations n'affecte pas le rÃ©sultat final du registre.

- **RedÃ©claration sÃ»re** : Les modules peuvent redÃ©clarer leurs capacitÃ©s Ã  chaque dÃ©marrage sans effet indÃ©sirable.

**Ce que cela signifie concrÃ¨tement :**

- Un module qui redÃ©marre et redÃ©clare ses capacitÃ©s ne corrompt pas le registre
- Les dÃ©clarations concurrentes ne crÃ©ent pas d'Ã©tats incohÃ©rents
- Le registre converge vers le mÃªme Ã©tat quelle que soit la sÃ©quence de dÃ©clarations

### 2.4. ImmutabilitÃ© des identifiants (INV-MB-4)

**Principe contractuel :**

L'invariant INV-MB-4 Ã©tablit que les identifiants de capacitÃ©s sont **immuables**. Une fois qu'une capacitÃ© est dÃ©clarÃ©e avec un identifiant, cet identifiant ne change jamais.

**Traduction en logique d'implÃ©mentation :**

- **Identifiants stables** : Une fois crÃ©Ã©, un identifiant de capacitÃ© ne peut pas Ãªtre modifiÃ©.

- **Ã‰volution par crÃ©ation** : Si une capacitÃ© Ã©volue significativement, une nouvelle capacitÃ© est crÃ©Ã©e avec un nouvel identifiant.

- **RÃ©fÃ©rences durables** : Les rÃ©fÃ©rences aux capacitÃ©s (dans les permissions, les logs, les configurations) restent valides dans le temps.

**Ce que cela signifie concrÃ¨tement :**

- Pas de renommage d'identifiants de capacitÃ©s existantes
- Les identifiants sont des rÃ©fÃ©rences stables pour tout le systÃ¨me
- L'Ã©volution se fait par ajout, pas par modification

### 2.5. TraÃ§abilitÃ© complÃ¨te (INV-MB-5)

**Principe contractuel :**

L'invariant INV-MB-5 Ã©tablit que toute modification du registre de Master Butler est **tracÃ©e**. CrÃ©ations, modifications, suppressions : tout est enregistrÃ© avec le contexte (qui, quand, pourquoi).

**Traduction en logique d'implÃ©mentation :**

- **Journalisation systÃ©matique** : Chaque dÃ©claration de capacitÃ©, chaque dÃ©finition de permission, chaque modification est tracÃ©e.

- **Contexte complet** : Les traces incluent le contexte complet (acteur, timestamp, raison).

- **Historique auditable** : L'historique des capacitÃ©s et permissions est accessible pour audit.

**Ce que cela signifie concrÃ¨tement :**

- Aucune modification silencieuse n'est possible
- L'audit peut reconstituer l'Ã©volution complÃ¨te du registre
- Les traces sont immuables et accessibles

### 2.6. SÃ©paration capacitÃ©/permission (INV-MB-6)

**Principe contractuel :**

L'invariant INV-MB-6 Ã©tablit que les capacitÃ©s et les permissions sont **strictement sÃ©parÃ©es**. Une capacitÃ© existe indÃ©pendamment des permissions. Une permission rÃ©fÃ©rence des capacitÃ©s mais ne les dÃ©finit pas.

**Traduction en logique d'implÃ©mentation :**

- **ModÃ¨les distincts** : Les capacitÃ©s et les permissions sont des entitÃ©s distinctes avec leurs propres modÃ¨les.

- **Relations sans fusion** : Une permission peut rÃ©fÃ©rencer des capacitÃ©s, mais les deux restent des entitÃ©s sÃ©parÃ©es.

- **Suppression indÃ©pendante** : La suppression d'une permission n'affecte pas la capacitÃ© associÃ©e. La suppression d'une capacitÃ© invalide les permissions qui la rÃ©fÃ©rencent.

**Ce que cela signifie concrÃ¨tement :**

- Une capacitÃ© peut exister sans aucune permission associÃ©e
- Une permission DOIT rÃ©fÃ©rencer au moins une capacitÃ© existante
- La suppression d'une capacitÃ© orpheline les permissions qui la rÃ©fÃ©rencent

### 2.7. Pas de logique mÃ©tier (INV-MB-7)

**Principe contractuel :**

L'invariant INV-MB-7 Ã©tablit que Master Butler **ne contient aucune logique mÃ©tier**. Il ne connaÃ®t pas les rÃ¨gles du domaine, les contraintes applicatives, les limites fonctionnelles.

**Traduction en logique d'implÃ©mentation :**

- **Registre technique** : Master Butler sait ce qui est techniquement possible, pas ce qui est mÃ©tier-compatible.

- **Pas de validation mÃ©tier** : Master Butler ne valide jamais une action selon des critÃ¨res mÃ©tier.

- **NeutralitÃ© fonctionnelle** : Les capacitÃ©s et permissions sont des concepts techniques, pas des rÃ¨gles mÃ©tier.

**Ce que cela signifie concrÃ¨tement :**

- Si une rÃ¨gle mÃ©tier dit "un utilisateur ne peut crÃ©er que 10 contenus par jour", cette contrainte n'appartient PAS Ã  Master Butler
- Master Butler sait que la capacitÃ© "content.create" existe, mais ignore les limites mÃ©tier
- La logique mÃ©tier appartient aux modules, aux produits, et Ã  StrongFather

### 2.8. AccessibilitÃ© universelle (INV-MB-8)

**Principe contractuel :**

L'invariant INV-MB-8 Ã©tablit que Master Butler est **accessible Ã  tous les composants autorisÃ©s** du systÃ¨me. Aucun composant ne peut Ãªtre empÃªchÃ© d'interroger Master Butler (sous rÃ©serve des permissions d'accÃ¨s Ã  Master Butler lui-mÃªme).

**Traduction en logique d'implÃ©mentation :**

- **Service partagÃ©** : Master Butler est un service partagÃ©, pas un composant isolÃ©.

- **AccessibilitÃ© garantie** : Tout composant autorisÃ© peut interroger Master Butler.

- **DisponibilitÃ©** : Master Butler DOIT Ãªtre disponible pour rÃ©pondre aux interrogations des composants.

**Ce que cela signifie concrÃ¨tement :**

- StrongFather, BondingBrother, les produits peuvent interroger Master Butler
- Aucun composant n'est privilÃ©giÃ© dans l'accÃ¨s (sauf Master Butler lui-mÃªme qui peut contrÃ´ler son propre accÃ¨s)
- L'accessibilitÃ© ne signifie pas absence de contrÃ´le d'accÃ¨s

---

## 3. Comment traduire les contrats en logique sans interprÃ©tation abusive

### 3.1. Respecter les invariants comme contraintes absolues

**Principe :**

Les invariants contractuels (INV-MB-*) sont des contraintes absolues qui DOIVENT toujours Ãªtre vraies. Ils ne sont pas des suggestions ou des recommandations.

**Traduction :**

- **VÃ©rification systÃ©matique** : Chaque invariant DOIT Ãªtre vÃ©rifiÃ© et prÃ©servÃ© Ã  chaque opÃ©ration. Aucun invariant ne peut Ãªtre violÃ©, mÃªme temporairement.

- **PrÃ©servation garantie** : Toute opÃ©ration DOIT garantir que les invariants sont prÃ©servÃ©s aprÃ¨s exÃ©cution. Si une opÃ©ration violerait un invariant, elle DOIT Ãªtre rejetÃ©e.

- **Pas d'interprÃ©tation** : Les invariants ne peuvent pas Ãªtre interprÃ©tÃ©s ou adaptÃ©s. Ils sont absolus et non nÃ©gociables.

**Exemple conceptuel :**

Si l'invariant INV-MB-2 (non-dÃ©cision) exige que Master Butler ne produise jamais de verdict d'autorisation, alors aucune mÃ©thode ne peut retourner "autorisÃ©" ou "refusÃ©", mÃªme pour des raisons de "commoditÃ©" ou "simplification".

### 3.2. ImplÃ©menter la dÃ©couverte comme exposition pure

**Principe :**

La dÃ©couverte des capacitÃ©s est une exposition pure des informations du registre, sans filtrage dÃ©cisionnel, sans recommandation, sans suggestion.

**Traduction :**

- **Exposition neutre** : La dÃ©couverte expose les capacitÃ©s et permissions existantes, sans jugement.

- **Filtrage technique uniquement** : Le filtrage est autorisÃ© pour des critÃ¨res techniques (module, type de capacitÃ©), pas pour des critÃ¨res dÃ©cisionnels.

- **ExhaustivitÃ©** : La dÃ©couverte retourne toutes les informations pertinentes, sans omission.

**Exemple conceptuel :**

Une requÃªte "Quelles capacitÃ©s existent dans le module CMS ?" retourne la liste exhaustive des capacitÃ©s de ce module, sans filtrer selon "ce que l'utilisateur devrait voir" (qui appartient Ã  StrongFather).

### 3.3. Traiter les dÃ©clarations comme des enregistrements, pas des validations mÃ©tier

**Principe :**

Les dÃ©clarations de capacitÃ©s sont des enregistrements dans le registre, pas des validations mÃ©tier. Master Butler vÃ©rifie la structure de la dÃ©claration, pas sa pertinence mÃ©tier.

**Traduction :**

- **Validation structurelle** : Master Butler vÃ©rifie que la dÃ©claration est bien formÃ©e (identifiant, mÃ©tadonnÃ©es).

- **Pas de validation mÃ©tier** : Master Butler ne vÃ©rifie pas si la capacitÃ© "devrait" exister selon des critÃ¨res mÃ©tier.

- **Enregistrement fidÃ¨le** : La dÃ©claration est enregistrÃ©e fidÃ¨lement, sans interprÃ©tation.

**Exemple conceptuel :**

Si un module dÃ©clare une capacitÃ© "delete.all", Master Butler l'enregistre. Il ne juge pas si cette capacitÃ© est "dangereuse" ou "devrait Ãªtre limitÃ©e" â€” ces considÃ©rations appartiennent aux politiques de StrongFather.

### 3.4. Ne pas "optimiser" en fusionnant registre et dÃ©cision

**Principe :**

Aucune optimisation ne peut fusionner la fonction de registre de Master Butler avec la fonction de dÃ©cision de StrongFather. Les deux sont sÃ©parÃ©s par conception.

**Traduction :**

- **SÃ©paration stricte** : Le registre (Master Butler) et la dÃ©cision (StrongFather) sont des fonctions distinctes.

- **Pas de raccourci** : Aucun "raccourci" ne peut permettre de prendre une dÃ©cision directement depuis le registre.

- **Flux complet** : Toute dÃ©cision passe par StrongFather, mÃªme si Master Butler a "toutes les informations".

**Exemple conceptuel :**

MÃªme si Master Butler sait qu'un utilisateur possÃ¨de une permission, il ne peut pas retourner "autorisÃ©". StrongFather DOIT Ã©valuer cette permission selon les politiques avant de produire une dÃ©cision.

---

## 4. Ce qu'un dÃ©veloppeur ne doit jamais faire

### 4.1. Produire des dÃ©cisions d'autorisation

**Interdiction contractuelle :**

Master Butler ne dÃ©cide jamais. Il fournit des informations, StrongFather dÃ©cide.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- CrÃ©er une mÃ©thode `isAuthorized()` ou Ã©quivalent qui retourne un boolÃ©en de dÃ©cision
- Retourner "oui" ou "non" Ã  une question d'autorisation
- Filtrer les rÃ©sultats selon une logique d'autorisation
- Recommander ou suggÃ©rer une dÃ©cision

**ConsÃ©quence de la violation :**

- Violation de l'invariant INV-MB-2 (non-dÃ©cision)
- Usurpation du rÃ´le de StrongFather
- Compromission de la sÃ©paration des responsabilitÃ©s

### 4.2. ExÃ©cuter des actions fonctionnelles

**Interdiction contractuelle :**

Master Butler ne crÃ©e pas de contenu, ne modifie pas de hiÃ©rarchie, ne tÃ©lÃ©verse pas de mÃ©dia. Il recense les capacitÃ©s qui permettent ces actions, mais ne les exÃ©cute jamais.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- ImplÃ©menter des actions fonctionnelles dans Master Butler
- CrÃ©er des mÃ©thodes qui modifient des donnÃ©es mÃ©tier
- ExÃ©cuter des opÃ©rations au nom des modules ou produits
- DÃ©lÃ©guer des actions fonctionnelles depuis Master Butler

**ConsÃ©quence de la violation :**

- Violation du rÃ´le de registre de Master Butler
- Usurpation du rÃ´le des modules et produits
- Compromission de l'architecture

### 4.3. Stocker des donnÃ©es mÃ©tier

**Interdiction contractuelle :**

Master Butler ne stocke jamais de donnÃ©es mÃ©tier. Il stocke des mÃ©tadonnÃ©es : dÃ©finitions de capacitÃ©s, dÃ©finitions de permissions, associations, historiques.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- Stocker des donnÃ©es utilisateur dans Master Butler
- Stocker des contenus, des mÃ©dias, des documents
- Utiliser le registre comme base de donnÃ©es mÃ©tier
- MÃ©langer mÃ©tadonnÃ©es de capacitÃ©s et donnÃ©es mÃ©tier

**ConsÃ©quence de la violation :**

- Violation du rÃ´le de registre de Master Butler
- Confusion des responsabilitÃ©s avec KindMother
- Compromission de l'isolation des donnÃ©es

### 4.4. GÃ©rer les identitÃ©s

**Interdiction contractuelle :**

Master Butler ne gÃ¨re jamais les identitÃ©s des utilisateurs ou des systÃ¨mes. Il connaÃ®t les rÃ´les et les permissions associÃ©es, mais l'identitÃ© elle-mÃªme appartient au systÃ¨me d'authentification.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- ImplÃ©menter une gestion d'identitÃ© dans Master Butler
- Stocker des credentials ou des tokens
- Authentifier des utilisateurs
- GÃ©rer des sessions

**ConsÃ©quence de la violation :**

- Violation du pÃ©rimÃ¨tre de Master Butler
- Usurpation du rÃ´le du systÃ¨me d'authentification
- Compromission de la sÃ©curitÃ©

### 4.5. DÃ©finir des politiques de dÃ©cision

**Interdiction contractuelle :**

Master Butler ne dÃ©finit jamais de politiques de dÃ©cision. Les politiques (rÃ¨gles qui dÃ©terminent quand une permission est accordÃ©e ou refusÃ©e) appartiennent Ã  StrongFather.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- ImplÃ©menter des politiques d'autorisation dans Master Butler
- CrÃ©er des rÃ¨gles conditionnelles d'autorisation
- DÃ©finir des contextes d'autorisation
- ImplÃ©menter une logique "si X alors autorisÃ©"

**ConsÃ©quence de la violation :**

- Violation de l'invariant INV-MB-2 (non-dÃ©cision)
- Usurpation du rÃ´le de StrongFather
- Duplication des responsabilitÃ©s

### 4.6. Appliquer des contraintes mÃ©tier

**Interdiction contractuelle :**

Master Butler n'applique jamais de contraintes mÃ©tier. Les contraintes mÃ©tier appartiennent Ã  StrongFather ou aux produits.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- ImplÃ©menter des limites mÃ©tier (quotas, plafonds)
- Valider des rÃ¨gles de domaine
- Appliquer des contraintes temporelles mÃ©tier
- Filtrer selon des critÃ¨res mÃ©tier

**ConsÃ©quence de la violation :**

- Violation de l'invariant INV-MB-7 (pas de logique mÃ©tier)
- Confusion des responsabilitÃ©s
- Contamination du registre par la logique mÃ©tier

### 4.7. Persister directement

**Interdiction contractuelle :**

Master Butler ne gÃ¨re jamais directement la persistance. Si son registre doit Ãªtre persistÃ©, il utilise KindMother comme support.

**Ce qu'un dÃ©veloppeur ne doit JAMAIS faire :**

- AccÃ©der directement Ã  une base de donnÃ©es depuis Master Butler
- Manipuler directement un systÃ¨me de fichiers
- ImplÃ©menter une couche de persistance propre Ã  Master Butler
- Contourner KindMother pour la persistance

**ConsÃ©quence de la violation :**

- Violation de l'architecture de persistance
- Duplication des responsabilitÃ©s avec KindMother
- Compromission de la cohÃ©rence des donnÃ©es

---

## 5. Anti-patterns classiques

### 5.1. Anti-pattern 1 : Le registre dÃ©cideur

**Description :**

Tentative de transformer Master Butler en dÃ©cideur en ajoutant des mÃ©thodes qui retournent des verdicts d'autorisation.

**Exemple conceptuel :**

Un dÃ©veloppeur crÃ©e une mÃ©thode "canUserPerform(user, capability)" qui retourne true/false, pensant "simplifier" l'intÃ©gration avec les produits.

**ConsÃ©quence :**

- Violation de l'invariant INV-MB-2 (non-dÃ©cision)
- Usurpation du rÃ´le de StrongFather
- Confusion architecturale entre registre et dÃ©cision
- Les produits peuvent contourner StrongFather en interrogeant directement Master Butler

**Correction :**

Master Butler retourne les informations (permissions de l'utilisateur, capacitÃ©s requises), et StrongFather prend la dÃ©cision. Aucune mÃ©thode de Master Butler ne retourne un boolÃ©en de dÃ©cision.

### 5.2. Anti-pattern 2 : Le registre exÃ©cutant

**Description :**

Tentative de faire exÃ©cuter des actions par Master Butler en plus de son rÃ´le de registre.

**Exemple conceptuel :**

Un dÃ©veloppeur ajoute une mÃ©thode "executeCapability(capability, context)" dans Master Butler, pensant "centraliser" l'exÃ©cution des capacitÃ©s.

**ConsÃ©quence :**

- Violation du rÃ´le de registre de Master Butler
- Usurpation du rÃ´le des modules et produits
- Couplage fort entre registre et exÃ©cution
- Compromission de l'isolation des responsabilitÃ©s

**Correction :**

Master Butler recense les capacitÃ©s et fournit les informations. L'exÃ©cution appartient aux modules et produits qui possÃ¨dent ces capacitÃ©s. Master Butler ne touche jamais Ã  l'exÃ©cution.

### 5.3. Anti-pattern 3 : Le registre mÃ©tier

**Description :**

Tentative d'intÃ©grer des rÃ¨gles mÃ©tier dans le registre de capacitÃ©s et permissions.

**Exemple conceptuel :**

Un dÃ©veloppeur ajoute des propriÃ©tÃ©s mÃ©tier aux capacitÃ©s (quotas, limites, conditions d'utilisation mÃ©tier), pensant "enrichir" le registre.

**ConsÃ©quence :**

- Violation de l'invariant INV-MB-7 (pas de logique mÃ©tier)
- Contamination du registre par la logique mÃ©tier
- Confusion entre capacitÃ©s techniques et contraintes mÃ©tier
- Ã‰volution couplÃ©e du registre et des rÃ¨gles mÃ©tier

**Correction :**

Le registre contient des mÃ©tadonnÃ©es techniques (identifiant, nom, description, module d'origine). Les rÃ¨gles mÃ©tier (quotas, limites, conditions) appartiennent aux politiques de StrongFather ou aux modules.

### 5.4. Anti-pattern 4 : Le raccourci de dÃ©couverte

**Description :**

Tentative de crÃ©er des raccourcis qui combinent dÃ©couverte et dÃ©cision pour "simplifier" l'usage.

**Exemple conceptuel :**

Un dÃ©veloppeur crÃ©e une mÃ©thode "getAccessibleCapabilities(user)" qui filtre les capacitÃ©s selon ce que l'utilisateur "devrait voir", pensant "faciliter" l'intÃ©gration.

**ConsÃ©quence :**

- Violation de l'invariant INV-MB-2 (non-dÃ©cision)
- Fusion illÃ©gitime de dÃ©couverte et dÃ©cision
- Contournement de StrongFather pour le filtrage
- IncohÃ©rence entre les sources de dÃ©cision

**Correction :**

Master Butler expose toutes les capacitÃ©s (discovery neutre). StrongFather applique les filtres selon les politiques. La dÃ©couverte et la dÃ©cision restent sÃ©parÃ©es.

### 5.5. Anti-pattern 5 : Le registre avec mÃ©moire d'Ã©tat

**Description :**

Tentative de maintenir un Ã©tat d'utilisation ou de dÃ©cision dans le registre.

**Exemple conceptuel :**

Un dÃ©veloppeur stocke "derniÃ¨re utilisation" ou "nombre d'appels" d'une capacitÃ© dans le registre, pensant "optimiser" les dÃ©cisions futures.

**ConsÃ©quence :**

- Violation du rÃ´le de registre passif de Master Butler
- Introduction d'Ã©tat dynamique dans un registre statique
- Couplage entre registre et utilisation
- DÃ©rive vers la logique mÃ©tier

**Correction :**

Le registre est statique (capacitÃ©s dÃ©clarÃ©es, permissions dÃ©finies). L'Ã©tat d'utilisation appartient aux modules, aux produits, ou aux systÃ¨mes d'observabilitÃ©. Master Butler ne maintient pas d'Ã©tat d'utilisation.

---

## 6. Bonnes pratiques conceptuelles

### 6.1. DÃ©claration systÃ©matique et prÃ©coce

**Pratique :**

Tout module DOIT dÃ©clarer ses capacitÃ©s Ã  Master Butler lors de son initialisation, avant d'Ãªtre opÃ©rationnel.

**Justification :**

- Respecte l'invariant INV-MB-1 (exhaustivitÃ© du registre)
- Garantit que toutes les capacitÃ©s sont connues avant utilisation
- Permet la dÃ©couverte complÃ¨te dÃ¨s le dÃ©marrage

**ImplÃ©mentation conceptuelle :**

- DÃ©claration lors du bootstrap du module
- Validation de la dÃ©claration par Master Butler
- Confirmation avant passage en mode opÃ©rationnel
- RedÃ©claration possible sans effet indÃ©sirable (idempotence)

### 6.2. SÃ©paration stricte des modÃ¨les

**Pratique :**

Les capacitÃ©s et les permissions DOIVENT Ãªtre des modÃ¨les distincts, avec des cycles de vie indÃ©pendants.

**Justification :**

- Respecte l'invariant INV-MB-6 (sÃ©paration capacitÃ©/permission)
- Permet l'Ã©volution indÃ©pendante des capacitÃ©s et permissions
- Facilite la maintenance et l'audit

**ImplÃ©mentation conceptuelle :**

- Registre des capacitÃ©s sÃ©parÃ© du registre des permissions
- Relations explicites entre permissions et capacitÃ©s (rÃ©fÃ©rences)
- Gestion indÃ©pendante des cycles de vie
- Validation des rÃ©fÃ©rences lors de la crÃ©ation des permissions

### 6.3. TraÃ§abilitÃ© complÃ¨te et immuable

**Pratique :**

Toute modification du registre DOIT Ãªtre tracÃ©e de maniÃ¨re complÃ¨te et immuable.

**Justification :**

- Respecte l'invariant INV-MB-5 (traÃ§abilitÃ© complÃ¨te)
- Permet l'audit complet de l'Ã©volution du registre
- Garantit la responsabilitÃ© des modifications

**ImplÃ©mentation conceptuelle :**

- Journalisation systÃ©matique de chaque modification
- Contexte complet (acteur, timestamp, raison)
- Traces immuables (append-only)
- AccessibilitÃ© pour audit

### 6.4. RÃ©ponses informatives et neutres

**Pratique :**

Les rÃ©ponses de Master Butler DOIVENT Ãªtre informatives, complÃ¨tes, et neutres â€” sans jugement ni recommandation.

**Justification :**

- Respecte l'invariant INV-MB-2 (non-dÃ©cision)
- Permet aux consommateurs (StrongFather, produits) de prendre leurs propres dÃ©cisions
- Maintient la sÃ©paration des responsabilitÃ©s

**ImplÃ©mentation conceptuelle :**

- Retour d'informations descriptives (capacitÃ©s, permissions, associations)
- Pas de boolÃ©en de dÃ©cision
- Pas de recommandation ou suggestion
- ExhaustivitÃ© des informations retournÃ©es

### 6.5. Validation structurelle uniquement

**Pratique :**

Master Butler DOIT valider la structure des dÃ©clarations (forme, complÃ©tude), pas leur pertinence mÃ©tier.

**Justification :**

- Respecte l'invariant INV-MB-7 (pas de logique mÃ©tier)
- Maintient la neutralitÃ© du registre
- Permet aux modules de dÃ©clarer librement leurs capacitÃ©s

**ImplÃ©mentation conceptuelle :**

- Validation de la prÃ©sence des champs obligatoires
- Validation du format des identifiants
- Validation de l'existence des rÃ©fÃ©rences (capacitÃ©s pour les permissions)
- Pas de validation mÃ©tier (pertinence, limites, conditions)

### 6.6. Respect des lois d'autonomie

**Pratique :**

L'implÃ©mentation de Master Butler DOIT respecter les lois d'autonomie systÃ¨me (LOI-1, LOI-5).

**Justification :**

- LOI-1 : Aucune dÃ©pendance externe critique Ã  l'exÃ©cution
- LOI-5 : CoÃ»t proportionnel au hardware

**ImplÃ©mentation conceptuelle :**

- Registre local, interrogations locales
- Pas de dÃ©pendance Ã  des services externes pour les fonctions critiques
- Empreinte mÃ©moire proportionnelle au nombre de capacitÃ©s/permissions
- OpÃ©rations de lookup simples et efficaces

---

## 7. Check-list mentale avant toute feature

Avant d'implÃ©menter une nouvelle fonctionnalitÃ© dans Master Butler, un dÃ©veloppeur DOIT vÃ©rifier mentalement :

### 7.1. VÃ©rification du rÃ´le

- **Cette feature appartient-elle Ã  un registre ?** : VÃ©rifier que la fonctionnalitÃ© concerne le recensement, la dÃ©claration, la dÃ©couverte.

- **Cette feature n'est-elle pas une dÃ©cision ?** : S'assurer que la fonctionnalitÃ© ne produit pas de verdict d'autorisation.

- **Cette feature n'est-elle pas une exÃ©cution ?** : S'assurer que la fonctionnalitÃ© n'exÃ©cute pas d'action fonctionnelle.

### 7.2. VÃ©rification des invariants

- **L'invariant INV-MB-1 (exhaustivitÃ©) est-il prÃ©servÃ© ?** : La feature maintient-elle l'exhaustivitÃ© du registre ?

- **L'invariant INV-MB-2 (non-dÃ©cision) est-il respectÃ© ?** : La feature ne produit-elle pas de dÃ©cision ?

- **L'invariant INV-MB-3 (idempotence) est-il prÃ©servÃ© ?** : Les opÃ©rations sont-elles idempotentes ?

- **L'invariant INV-MB-4 (immutabilitÃ© des identifiants) est-il respectÃ© ?** : Les identifiants restent-ils stables ?

- **L'invariant INV-MB-5 (traÃ§abilitÃ©) est-il assurÃ© ?** : Toutes les modifications sont-elles tracÃ©es ?

- **L'invariant INV-MB-6 (sÃ©paration) est-il maintenu ?** : CapacitÃ©s et permissions restent-elles sÃ©parÃ©es ?

- **L'invariant INV-MB-7 (pas de logique mÃ©tier) est-il respectÃ© ?** : La feature est-elle exempte de logique mÃ©tier ?

- **L'invariant INV-MB-8 (accessibilitÃ©) est-il assurÃ© ?** : La feature reste-t-elle accessible aux composants autorisÃ©s ?

### 7.3. VÃ©rification des interdictions

- **La feature ne dÃ©cide-t-elle pas ?** : Aucune mÃ©thode ne retourne de dÃ©cision d'autorisation.

- **La feature n'exÃ©cute-t-elle pas ?** : Aucune action fonctionnelle n'est exÃ©cutÃ©e.

- **La feature ne stocke-t-elle pas de donnÃ©es mÃ©tier ?** : Seules des mÃ©tadonnÃ©es sont stockÃ©es.

- **La feature ne gÃ¨re-t-elle pas d'identitÃ©s ?** : Aucune gestion d'identitÃ© n'est implÃ©mentÃ©e.

- **La feature ne dÃ©finit-elle pas de politiques ?** : Aucune politique de dÃ©cision n'est crÃ©Ã©e.

- **La feature n'applique-t-elle pas de contraintes mÃ©tier ?** : Aucune rÃ¨gle mÃ©tier n'est appliquÃ©e.

### 7.4. VÃ©rification des relations

- **StrongFather reste-t-il le dÃ©cideur ?** : La feature ne court-circuite-t-elle pas StrongFather ?

- **BondingBrother peut-il interroger correctement ?** : La feature fournit-elle les informations nÃ©cessaires Ã  BondingBrother ?

- **KindMother gÃ¨re-t-elle la persistance ?** : La feature ne contourne-t-elle pas KindMother pour persister ?

---

## 8. Conclusion

Ce document fournit des lignes directrices pour implÃ©menter Master Butler de maniÃ¨re conforme aux contrats FONDATION.

**Points clÃ©s :**

- Master Butler est un **registre pur** : il recense, il expose, il ne dÃ©cide jamais
- Les **invariants INV-MB-1 Ã  INV-MB-8** sont des contraintes absolues
- Les **interdictions** (pas de dÃ©cision, pas d'exÃ©cution, pas de logique mÃ©tier) sont non nÃ©gociables
- Les **bonnes pratiques** garantissent la conformitÃ© architecturale
- La **check-list mentale** prÃ©vient les violations avant implÃ©mentation

**Nature informative :**

Ce document est purement informatif et ne crÃ©e aucune nouvelle obligation contractuelle. Il sert uniquement Ã  guider la comprÃ©hension et l'application des contrats FONDATION.

**Rappel :** Les contrats FONDATION priment toujours sur ce guide. En cas de doute, se rÃ©fÃ©rer aux contrats FONDATION.

---

**Document crÃ©Ã© le :** 2026-01-27  
**Version :** 1.0  
**Statut :** POST-FONDATION â€” Informatif, non normatif, non contractuel  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, Master Butler Documentation, Tous les contrats FONDATION  
**Type :** Guide d'implÃ©mentation informatif

---

## 9. Mini log â€” erreurs / warnings / arbitrages rencontrÃ©s

### Arbitrage A1 : Niveau de dÃ©tail technique

**Arbitrage rencontrÃ© :** Quel niveau de dÃ©tail technique inclure dans ce guide ? Le document doit rester conceptuel et ne pas prescrire de technologies.

**DÃ©cision prise :** Le document reste purement conceptuel. Aucun dÃ©tail technique (langages, structures de donnÃ©es, algorithmes) n'est inclus. Seuls les concepts et principes sont dÃ©crits.

**Justification :** Ce document est informatif et non normatif. Il guide la comprÃ©hension des contrats, pas l'implÃ©mentation technique. Les dÃ©tails techniques sont des choix d'implÃ©mentation.

**Documentation :** Toutes les sections restent conceptuelles, sans dÃ©tails techniques.

### Arbitrage A2 : ParallÃ¨le avec KindMother

**Arbitrage rencontrÃ© :** Dans quelle mesure le document doit-il suivre la structure du guide KindMother ?

**DÃ©cision prise :** La structure gÃ©nÃ©rale suit le modÃ¨le KindMother (sections, organisation), mais le contenu est entiÃ¨rement adaptÃ© au contexte spÃ©cifique de Master Butler (registre vs. gestionnaire de donnÃ©es).

**Justification :** La cohÃ©rence structurelle facilite la navigation entre les guides des diffÃ©rents Cores, tout en prÃ©servant la spÃ©cificitÃ© de chaque Core.

**Documentation :** Structure parallÃ¨le mais contenu spÃ©cifique Ã  Master Butler.

### Arbitrage A3 : Distinction registre passif vs. service actif

**Arbitrage rencontrÃ© :** Comment clarifier la nature de "registre passif" de Master Butler sans crÃ©er de confusion avec un service inactif ?

**DÃ©cision prise :** Le document clarifie que Master Butler est un registre "passif" au sens oÃ¹ il ne prend pas d'initiative (pas de dÃ©cision, pas d'exÃ©cution), mais il est "actif" dans le sens oÃ¹ il rÃ©pond aux requÃªtes et maintient son registre.

**Justification :** Cette distinction Ã©vite la confusion entre "passif" (nature) et "inactif" (comportement).

**Documentation :** Section 2.1 et 6.4 clarifient cette distinction.

### Arbitrage A4 : Traitement des anti-patterns

**Arbitrage rencontrÃ© :** Les anti-patterns doivent-ils Ãªtre spÃ©cifiques Ã  Master Butler ou gÃ©nÃ©riques ?

**DÃ©cision prise :** Les anti-patterns sont spÃ©cifiques au contexte de Master Butler (registre dÃ©cideur, registre exÃ©cutant, registre mÃ©tier), illustrant les violations typiques du rÃ´le de registre.

**Justification :** Des anti-patterns spÃ©cifiques sont plus utiles pour guider les dÃ©veloppeurs dans le contexte de Master Butler.

**Documentation :** Section 5 avec anti-patterns spÃ©cifiques Ã  Master Butler.

### Arbitrage A5 : Relations avec les autres Cores

**Arbitrage rencontrÃ© :** Comment prÃ©senter les relations avec StrongFather, KindMother, BondingBrother sans dupliquer la documentation fondatrice ?

**DÃ©cision prise :** Les relations sont mentionnÃ©es dans le contexte des vÃ©rifications (check-list) et des interdictions, sans dÃ©tailler les flux complets qui appartiennent Ã  la documentation fondatrice.

**Justification :** Ce guide est orientÃ© implÃ©mentation, pas architecture. Les relations dÃ©taillÃ©es appartiennent aux contrats d'intÃ©gration.

**Documentation :** Section 7.4 et mentions dans les interdictions.

---

*Aucune autre erreur, warning, ou arbitrage rencontrÃ© lors de la rÃ©daction de ce document.*

---

## 10. Conformite MSCM/MIP

### 10.1 Obligation de balisage MSCM

Tout code implemente pour Master Butler DOIT etre balise selon le protocole MSCM v1.

**Reference :** [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md)

**Obligations minimales :**
- Chaque bloc fonctionnel DOIT avoir un identifiant unique (`@id`)
- Le role semantique DOIT etre explicite (`@role`)
- La couche architecturale DOIT etre declaree (`@layer`)
- Une description humaine DOIT accompagner chaque bloc (`@human`)

### 10.2 Integration MIP

Apres implementation, l'index MIP DOIT etre regenere pour :
- Valider l'integrite des blocs MSCM
- Mettre a jour le graphe de dependances
- Verifier la coherence hierarchique

### 10.3 Check-list MSCM

Avant toute livraison, verifier :
- [ ] Tous les blocs critiques sont balises MSCM
- [ ] Les identifiants sont uniques globalement
- [ ] Les couches (layer) sont coherentes avec l'architecture
- [ ] L'index MIP peut etre regenere sans erreur

