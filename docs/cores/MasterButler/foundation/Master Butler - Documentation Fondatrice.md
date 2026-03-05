# Master Butler - Documentation Fondatrice

## 1. Introduction

### RÃ´le de Master Butler

Master Butler est le **Capability & Permission Core** du Miyukini Core System. Il incarne la connaissance de ce qui est possible : quelles capacitÃ©s existent dans le systÃ¨me, quelles permissions sont dÃ©finies, et quels droits peuvent Ãªtre accordÃ©s.

Master Butler rÃ©pond Ã  une question fondamentale : **"Que peut-on faire ici, et qui a le droit de le faire ?"**

Cette question est distincte de la dÃ©cision ("devrait-on le faire ?") qui appartient Ã  StrongFather, et distincte de l'exÃ©cution ("comment le faire ?") qui appartient aux produits et Ã  KindMother.

Master Butler est le registre vivant des possibilitÃ©s du systÃ¨me. Il ne dÃ©cide jamais, il n'exÃ©cute jamais, il n'autorise jamais. Il expose, il recense, il documente ce qui existe comme capacitÃ©s et ce qui est dÃ©fini comme permissions.

### Question fondamentale

Master Butler existe pour rÃ©pondre Ã  cette question unique et fondamentale :

**"Quelles sont les capacitÃ©s du systÃ¨me, et quelles permissions existent pour y accÃ©der ?"**

Cette question se dÃ©cline en sous-questions :
- Quelles actions sont techniquement possibles dans ce module ?
- Quelles permissions sont dÃ©finies pour accÃ©der Ã  ces capacitÃ©s ?
- Quels rÃ´les portent quelles permissions ?
- Quelles capacitÃ©s sont disponibles pour un contexte donnÃ© ?

Master Butler fournit ces informations de maniÃ¨re exhaustive, cohÃ©rente, et traÃ§able. Il est le cartographe des possibilitÃ©s, jamais le dÃ©cideur de leur usage.

---

## 2. Raison d'Ãªtre

### ProblÃ¨me que Master Butler rÃ©sout

Dans un systÃ¨me modulaire comme Miyukini, les capacitÃ©s sont dispersÃ©es dans les modules, les adaptateurs, et les produits. Sans centralisation de cette connaissance, plusieurs problÃ¨mes Ã©mergent :

1. **OpacitÃ© des possibilitÃ©s** : Aucun composant ne sait quelles capacitÃ©s existent ailleurs dans le systÃ¨me. Chaque module connaÃ®t ses propres capacitÃ©s, mais ignore celles des autres.

2. **DÃ©finitions de permissions dispersÃ©es** : Les permissions sont dÃ©finies localement, sans registre central. Cela conduit Ã  des duplications, des incohÃ©rences, et des zones d'ombre.

3. **ImpossibilitÃ© de rÃ©pondre "que puis-je faire ?"** : Sans registre central, un produit ou un utilisateur ne peut pas obtenir la liste des capacitÃ©s accessibles dans un contexte donnÃ©.

4. **Couplage entre connaissance et dÃ©cision** : Les composants mÃ©langent la connaissance de ce qui est possible avec la dÃ©cision de ce qui est autorisÃ©, crÃ©ant de la confusion architecturale.

5. **Absence de dÃ©couverte** : Aucun mÃ©canisme ne permet de dÃ©couvrir dynamiquement les capacitÃ©s du systÃ¨me, obligeant un codage en dur des fonctionnalitÃ©s.

### Ce que Master Butler apporte

Master Butler rÃ©sout ces problÃ¨mes en fournissant :

- **Un registre central des capacitÃ©s** : Toutes les capacitÃ©s du systÃ¨me sont recensÃ©es, documentÃ©es, et accessibles via Master Butler.

- **Un registre central des permissions** : Toutes les permissions sont dÃ©finies, nommÃ©es, et organisÃ©es de maniÃ¨re cohÃ©rente.

- **Une API de dÃ©couverte** : Les composants peuvent interroger Master Butler pour dÃ©couvrir les capacitÃ©s et permissions disponibles.

- **Une sÃ©paration claire** : Master Butler sÃ©pare la connaissance (ce qui existe) de la dÃ©cision (ce qui est autorisÃ©). Il fournit les informations, StrongFather dÃ©cide.

### NÃ©cessitÃ© architecturale

Sans Master Butler, l'Ã©cosystÃ¨me Miyukini serait incapable de rÃ©pondre aux questions fondamentales sur ses propres capacitÃ©s. Les produits devraient maintenir leurs propres registres, crÃ©ant des duplications et des incohÃ©rences. La dÃ©cision serait basÃ©e sur des informations partielles ou obsolÃ¨tes.

Master Butler est nÃ©cessaire parce que la connaissance des possibilitÃ©s doit Ãªtre centralisÃ©e, cohÃ©rente, et accessible. Cette connaissance est la base sur laquelle StrongFather prend ses dÃ©cisions, et sans laquelle aucune dÃ©cision Ã©clairÃ©e n'est possible.

---

## 3. Positionnement familial

### Relation avec StrongFather

Master Butler et StrongFather forment un couple complÃ©mentaire et indissociable :

- **Master Butler** expose ce qui est possible (capacitÃ©s et permissions)
- **StrongFather** dÃ©cide ce qui est autorisÃ© (Ã©valuation des intentions)

Cette relation est asymÃ©trique : StrongFather dÃ©pend de Master Butler pour connaÃ®tre les possibilitÃ©s, mais Master Butler ne dÃ©pend pas de StrongFather pour exister. Master Butler recense, StrongFather dÃ©cide.

**Flux typique :**
1. Un produit exprime une intention (via BondingBrother)
2. StrongFather interroge Master Butler : "Cette capacitÃ© existe-t-elle ? Quelles permissions sont requises ?"
3. Master Butler rÃ©pond avec les informations demandÃ©es
4. StrongFather Ã©value l'intention selon les politiques et les permissions
5. StrongFather produit une dÃ©cision

Master Butler ne prend jamais part Ã  la dÃ©cision. Il fournit les informations nÃ©cessaires, sans jugement, sans interprÃ©tation, sans recommandation.

### Relation avec KindMother

Master Butler et KindMother opÃ¨rent dans des domaines distincts mais complÃ©mentaires :

- **KindMother** gÃ¨re les donnÃ©es (persistance, synchronisation, cohÃ©rence)
- **Master Butler** gÃ¨re la connaissance des capacitÃ©s et permissions (pas des donnÃ©es)

Master Butler ne stocke pas de donnÃ©es mÃ©tier. Il maintient un registre de mÃ©tadonnÃ©es : quelles capacitÃ©s existent, quelles permissions sont dÃ©finies. Ces mÃ©tadonnÃ©es peuvent Ãªtre persistÃ©es via KindMother, mais Master Butler ne gÃ¨re jamais directement la persistance.

La relation est indirecte : Master Butler utilise KindMother comme support de persistance pour son registre, mais ne connaÃ®t pas les dÃ©tails de cette persistance.

### Relation avec BondingBrother

Master Butler est interrogÃ© par BondingBrother lorsque celui-ci traduit des intentions :

- BondingBrother peut demander Ã  Master Butler : "Cette capacitÃ© existe-t-elle dans ce module ?"
- BondingBrother peut demander : "Quelles permissions sont requises pour cette action ?"

Master Butler fournit ces informations, permettant Ã  BondingBrother de traduire correctement les intentions et de prÃ©parer le contexte pour l'Ã©valuation par StrongFather.

### Relation avec les produits

Les produits enregistrent leurs capacitÃ©s auprÃ¨s de Master Butler :

- Lors de leur initialisation, les produits dÃ©clarent leurs capacitÃ©s Ã  Master Butler
- Les produits dÃ©finissent les permissions qu'ils reconnaissent
- Les produits peuvent interroger Master Butler pour dÃ©couvrir d'autres capacitÃ©s

Cette relation est bidirectionnelle : les produits alimentent Master Butler (dÃ©claration) et consomment Master Butler (dÃ©couverte).

### Position dans la famille Miyukini

Dans la famille Miyukini, Master Butler est le majordome de la maison : il connaÃ®t chaque piÃ¨ce, chaque Ã©quipement, chaque rÃ¨gle d'accÃ¨s. Il ne prend pas les dÃ©cisions (c'est le rÃ´le des parents), il n'exÃ©cute pas les tÃ¢ches (c'est le rÃ´le des enfants), mais il sait tout ce qui est possible et peut rÃ©pondre Ã  toute question sur les capacitÃ©s de la maison.

Master Butler est au service de tous, sans jamais prendre parti. Il informe, il recense, il expose, mais il ne juge jamais.

---

## 4. Concepts fondamentaux

### CapacitÃ© (Capability)

Une **capacitÃ©** est un pouvoir technique qu'un composant possÃ¨de. C'est ce qu'un module, un adaptateur, ou un produit peut faire techniquement, indÃ©pendamment des permissions.

**CaractÃ©ristiques d'une capacitÃ© :**
- Elle est intrinsÃ¨que au composant (le composant la possÃ¨de ou ne la possÃ¨de pas)
- Elle est technique (elle dÃ©crit un pouvoir fonctionnel)
- Elle est dÃ©clarative (elle est dÃ©clarÃ©e par le composant qui la possÃ¨de)
- Elle est identifiable (elle a un identifiant unique et stable)
- Elle est documentÃ©e (elle a une description et des mÃ©tadonnÃ©es)

**Exemples de capacitÃ©s :**
- `content.create` : CapacitÃ© de crÃ©er du contenu
- `hierarchy.reorder` : CapacitÃ© de rÃ©organiser une hiÃ©rarchie
- `media.upload` : CapacitÃ© de tÃ©lÃ©verser des mÃ©dias
- `search.index` : CapacitÃ© d'indexer pour la recherche

Une capacitÃ© existe indÃ©pendamment de toute permission. Un module peut possÃ©der la capacitÃ© de supprimer du contenu, mÃªme si aucune permission n'autorise cette suppression.

### Permission

Une **permission** est un droit accordÃ© pour accÃ©der Ã  une capacitÃ©. C'est l'autorisation conceptuelle d'utiliser une capacitÃ©, indÃ©pendamment de la dÃ©cision finale.

**CaractÃ©ristiques d'une permission :**
- Elle est dÃ©finie (elle est crÃ©Ã©e et nommÃ©e explicitement)
- Elle est associÃ©e Ã  une ou plusieurs capacitÃ©s
- Elle est attribuable (elle peut Ãªtre accordÃ©e Ã  des rÃ´les ou des contextes)
- Elle est rÃ©vocable (elle peut Ãªtre retirÃ©e)
- Elle est traÃ§able (son attribution est enregistrÃ©e)

**Exemples de permissions :**
- `content.create.any` : Permission de crÃ©er n'importe quel contenu
- `content.edit.own` : Permission de modifier son propre contenu
- `hierarchy.manage` : Permission de gÃ©rer les hiÃ©rarchies
- `media.delete.all` : Permission de supprimer tous les mÃ©dias

Une permission ne garantit pas l'autorisation finale. StrongFather Ã©value les permissions dans le contexte des politiques pour produire une dÃ©cision.

### Distinction fondamentale CapacitÃ© vs Permission

| Aspect | CapacitÃ© | Permission |
|--------|----------|------------|
| Nature | Pouvoir technique | Droit accordÃ© |
| Origine | IntrinsÃ¨que au composant | DÃ©finie par le systÃ¨me |
| Question | "Peut-on le faire techniquement ?" | "A-t-on le droit de le faire ?" |
| Possession | Le composant la possÃ¨de | Le contexte (rÃ´le, utilisateur) la dÃ©tient |
| Existence | IndÃ©pendante des permissions | AssociÃ©e aux capacitÃ©s |

**MÃ©taphore :** Une serrure (capacitÃ©) existe sur une porte. Une clÃ© (permission) permet d'ouvrir cette serrure. Avoir la clÃ© ne signifie pas qu'on a le droit d'entrer (dÃ©cision de StrongFather), mais sans la clÃ©, on ne peut pas entrer du tout.

### Registre des capacitÃ©s

Le **registre des capacitÃ©s** est la structure centrale de Master Butler. Il contient :
- L'inventaire exhaustif des capacitÃ©s du systÃ¨me
- Les mÃ©tadonnÃ©es de chaque capacitÃ© (nom, description, module d'origine)
- Les relations entre capacitÃ©s (dÃ©pendances, hiÃ©rarchies)
- L'historique des capacitÃ©s (ajouts, suppressions, modifications)

Le registre est dynamique : il Ã©volue avec le systÃ¨me, au fur et Ã  mesure que les modules dÃ©clarent leurs capacitÃ©s.

### Registre des permissions

Le **registre des permissions** est la seconde structure centrale de Master Butler. Il contient :
- L'inventaire exhaustif des permissions dÃ©finies
- Les associations entre permissions et capacitÃ©s
- Les mÃ©tadonnÃ©es de chaque permission (nom, description, niveau)
- L'historique des permissions (crÃ©ations, modifications, rÃ©vocations)

Le registre des permissions est distinct du registre des capacitÃ©s, mais ils sont liÃ©s : chaque permission rÃ©fÃ©rence une ou plusieurs capacitÃ©s.

### Contexte de capacitÃ©

Un **contexte de capacitÃ©** est l'ensemble des informations qui dÃ©finissent les capacitÃ©s et permissions disponibles dans une situation donnÃ©e. Le contexte inclut :
- L'identitÃ© du demandeur (utilisateur, systÃ¨me, produit)
- Les rÃ´les du demandeur
- Les permissions associÃ©es Ã  ces rÃ´les
- Le module ou le composant ciblÃ©
- Les capacitÃ©s disponibles dans ce composant

Master Butler peut calculer le contexte de capacitÃ© pour rÃ©pondre Ã  la question : "Dans cette situation, quelles capacitÃ©s sont accessibles et avec quelles permissions ?"

---

## 5. ResponsabilitÃ©s exclusives

### Recensement des capacitÃ©s

Master Butler est **exclusivement responsable** du recensement de toutes les capacitÃ©s du systÃ¨me. Aucun autre composant ne maintient de registre des capacitÃ©s. Tout composant souhaitant connaÃ®tre les capacitÃ©s disponibles doit interroger Master Butler.

Cette responsabilitÃ© inclut :
- RÃ©ception des dÃ©clarations de capacitÃ©s des modules et produits
- Validation de la structure des dÃ©clarations
- Stockage dans le registre des capacitÃ©s
- Mise Ã  jour lors des modifications
- Suppression lors des dÃ©prÃ©ciations

### DÃ©finition des permissions

Master Butler est **exclusivement responsable** de la dÃ©finition formelle des permissions. Aucun autre composant ne dÃ©finit de permissions. Toutes les permissions sont dÃ©clarÃ©es, nommÃ©es, et structurÃ©es dans Master Butler.

Cette responsabilitÃ© inclut :
- CrÃ©ation de nouvelles permissions
- Association des permissions aux capacitÃ©s
- Structuration hiÃ©rarchique des permissions
- Gestion des mÃ©tadonnÃ©es des permissions
- Historisation des modifications

### Fourniture des informations aux dÃ©cideurs

Master Butler est **exclusivement responsable** de fournir les informations sur les capacitÃ©s et permissions Ã  StrongFather et aux autres composants qui en ont besoin.

Cette responsabilitÃ© inclut :
- RÃ©ponse aux requÃªtes de StrongFather sur les capacitÃ©s
- RÃ©ponse aux requÃªtes de BondingBrother sur les permissions requises
- Fourniture du contexte de capacitÃ© aux composants autorisÃ©s
- Garantie de l'exactitude et de l'exhaustivitÃ© des informations

### DÃ©couverte des capacitÃ©s

Master Butler est **exclusivement responsable** de permettre la dÃ©couverte des capacitÃ©s du systÃ¨me. Les produits et modules peuvent interroger Master Butler pour dÃ©couvrir les capacitÃ©s existantes.

Cette responsabilitÃ© inclut :
- API de dÃ©couverte des capacitÃ©s par module
- API de dÃ©couverte des capacitÃ©s par type d'action
- API de dÃ©couverte des permissions par capacitÃ©
- Filtrage des capacitÃ©s selon le contexte

### TraÃ§abilitÃ© des dÃ©finitions

Master Butler est **exclusivement responsable** de la traÃ§abilitÃ© des dÃ©finitions de capacitÃ©s et permissions. Chaque crÃ©ation, modification, ou suppression est enregistrÃ©e avec son contexte.

Cette responsabilitÃ© inclut :
- Journalisation des dÃ©clarations de capacitÃ©s
- Journalisation des dÃ©finitions de permissions
- Historique des modifications
- Audit trail complet des Ã©volutions

---

## 6. Ce que Master Butler ne fait PAS

### Ne dÃ©cide pas

Master Butler **ne dÃ©cide jamais** si une action est autorisÃ©e ou refusÃ©e. Il fournit les informations sur les capacitÃ©s et permissions, mais la dÃ©cision appartient Ã  StrongFather. Master Butler rÃ©pond "cette permission existe et ce rÃ´le la possÃ¨de", mais ne rÃ©pond jamais "cette action est autorisÃ©e".

### Ne vÃ©rifie pas les permissions en temps rÃ©el

Master Butler **ne vÃ©rifie jamais** si un utilisateur ou un contexte possÃ¨de effectivement une permission au moment d'une action. Cette vÃ©rification appartient Ã  StrongFather lors de l'Ã©valuation des intentions. Master Butler fournit les dÃ©finitions, pas les vÃ©rifications.

### N'exÃ©cute pas

Master Butler **n'exÃ©cute jamais** d'action fonctionnelle. Il ne crÃ©e pas de contenu, ne modifie pas de hiÃ©rarchie, ne tÃ©lÃ©verse pas de mÃ©dia. Il recense les capacitÃ©s qui permettent ces actions, mais ne les exÃ©cute jamais.

### Ne stocke pas de donnÃ©es mÃ©tier

Master Butler **ne stocke jamais** de donnÃ©es mÃ©tier. Il stocke des mÃ©tadonnÃ©es : dÃ©finitions de capacitÃ©s, dÃ©finitions de permissions, associations, historiques. Les donnÃ©es mÃ©tier appartiennent aux modules et Ã  KindMother.

### Ne gÃ¨re pas les identitÃ©s

Master Butler **ne gÃ¨re jamais** les identitÃ©s des utilisateurs ou des systÃ¨mes. Il connaÃ®t les rÃ´les et les permissions associÃ©es, mais l'identitÃ© elle-mÃªme appartient au systÃ¨me d'authentification (hors-scope de Master Butler).

### Ne dÃ©finit pas de politiques

Master Butler **ne dÃ©finit jamais** de politiques de dÃ©cision. Les politiques (rÃ¨gles qui dÃ©terminent quand une permission est accordÃ©e ou refusÃ©e) appartiennent Ã  StrongFather. Master Butler dÃ©finit ce qui existe, pas comment l'utiliser.

### N'applique pas de contraintes mÃ©tier

Master Butler **n'applique jamais** de contraintes mÃ©tier. Si une rÃ¨gle mÃ©tier dit "un utilisateur ne peut crÃ©er que 10 contenus par jour", cette contrainte appartient Ã  StrongFather ou au produit, pas Ã  Master Butler. Master Butler sait que la capacitÃ© de crÃ©er du contenu existe, mais ignore les limites mÃ©tier.

### Ne persiste pas directement

Master Butler **ne gÃ¨re jamais** directement la persistance. Si son registre doit Ãªtre persistÃ©, il utilise KindMother comme support, mais ne manipule jamais directement une base de donnÃ©es ou un systÃ¨me de fichiers.

---

## 7. Invariants non nÃ©gociables

### INV-MB-1 : ExhaustivitÃ© du registre

Le registre de Master Butler est **exhaustif**. Toute capacitÃ© existant dans le systÃ¨me est recensÃ©e dans Master Butler. Si une capacitÃ© n'est pas dans le registre, elle n'existe pas officiellement dans le systÃ¨me.

**Implication :** Aucun module ne peut exposer une capacitÃ© sans la dÃ©clarer Ã  Master Butler. Aucun contournement n'est permis.

### INV-MB-2 : Non-dÃ©cision

Master Butler **ne prend jamais de dÃ©cision**. Il fournit des informations, rÃ©pond Ã  des questions, mais ne produit jamais de verdict "autorisÃ©" ou "refusÃ©". Toute dÃ©cision appartient Ã  StrongFather.

**Implication :** Aucune mÃ©thode de Master Butler ne retourne un boolÃ©en d'autorisation. Il retourne des informations, pas des dÃ©cisions.

### INV-MB-3 : Idempotence des dÃ©clarations

Les dÃ©clarations de capacitÃ©s sont **idempotentes**. DÃ©clarer deux fois la mÃªme capacitÃ© n'a pas d'effet supplÃ©mentaire. Le registre reste cohÃ©rent quel que soit l'ordre ou le nombre de dÃ©clarations.

**Implication :** Les modules peuvent redÃ©clarer leurs capacitÃ©s Ã  chaque dÃ©marrage sans effet indÃ©sirable.

### INV-MB-4 : ImmutabilitÃ© des identifiants

Les identifiants de capacitÃ©s sont **immuables**. Une fois qu'une capacitÃ© est dÃ©clarÃ©e avec un identifiant, cet identifiant ne change jamais. Si une capacitÃ© Ã©volue significativement, une nouvelle capacitÃ© est crÃ©Ã©e avec un nouvel identifiant.

**Implication :** Les rÃ©fÃ©rences aux capacitÃ©s (dans les permissions, les logs, les configurations) restent valides dans le temps.

### INV-MB-5 : TraÃ§abilitÃ© complÃ¨te

Toute modification du registre de Master Butler est **tracÃ©e**. CrÃ©ations, modifications, suppressions : tout est enregistrÃ© avec le contexte (qui, quand, pourquoi).

**Implication :** L'historique des capacitÃ©s et permissions est auditable. Aucune modification silencieuse n'est possible.

### INV-MB-6 : SÃ©paration capacitÃ©/permission

Les capacitÃ©s et les permissions sont **strictement sÃ©parÃ©es**. Une capacitÃ© existe indÃ©pendamment des permissions. Une permission rÃ©fÃ©rence des capacitÃ©s mais ne les dÃ©finit pas.

**Implication :** La suppression d'une permission n'affecte pas la capacitÃ© associÃ©e. La suppression d'une capacitÃ© invalide les permissions qui la rÃ©fÃ©rencent.

### INV-MB-7 : Pas de logique mÃ©tier

Master Butler **ne contient aucune logique mÃ©tier**. Il ne connaÃ®t pas les rÃ¨gles du domaine, les contraintes applicatives, les limites fonctionnelles. Il sait ce qui est techniquement possible, pas ce qui est mÃ©tier-compatible.

**Implication :** Master Butler ne valide jamais une action selon des critÃ¨res mÃ©tier. Cette validation appartient aux modules et Ã  StrongFather.

### INV-MB-8 : AccessibilitÃ© universelle

Master Butler est **accessible Ã  tous les composants autorisÃ©s** du systÃ¨me. Aucun composant ne peut Ãªtre empÃªchÃ© d'interroger Master Butler sur les capacitÃ©s et permissions (sous rÃ©serve des permissions d'accÃ¨s Ã  Master Butler lui-mÃªme).

**Implication :** Master Butler est un service partagÃ©, pas un composant isolÃ©. Son accessibilitÃ© est garantie.

---

## 8. Interactions avec l'Ã©cosystÃ¨me

### Flux de dÃ©claration de capacitÃ©s

**Acteurs :** Module SPM, Produit, Master Butler

**SÃ©quence :**
1. Le module ou produit dÃ©marre et identifie ses capacitÃ©s
2. Le module ou produit envoie une dÃ©claration Ã  Master Butler
3. Master Butler valide la structure de la dÃ©claration
4. Master Butler enregistre les capacitÃ©s dans le registre
5. Master Butler confirme l'enregistrement
6. Le module ou produit est opÃ©rationnel

**RÃ¨gles :**
- La dÃ©claration est obligatoire pour toute capacitÃ© exposÃ©e
- La dÃ©claration peut Ãªtre effectuÃ©e plusieurs fois (idempotence)
- La dÃ©claration inclut les mÃ©tadonnÃ©es (nom, description, module d'origine)

### Flux de dÃ©finition de permissions

**Acteurs :** Produit, Master Butler

**SÃ©quence :**
1. Le produit dÃ©finit une nouvelle permission
2. Le produit associe la permission Ã  des capacitÃ©s existantes
3. Le produit envoie la dÃ©finition Ã  Master Butler
4. Master Butler valide l'existence des capacitÃ©s rÃ©fÃ©rencÃ©es
5. Master Butler enregistre la permission dans le registre
6. Master Butler confirme l'enregistrement

**RÃ¨gles :**
- Une permission doit rÃ©fÃ©rencer au moins une capacitÃ© existante
- Une permission ne peut pas rÃ©fÃ©rencer une capacitÃ© inexistante
- Les mÃ©tadonnÃ©es de permission sont obligatoires

### Flux de dÃ©couverte de capacitÃ©s

**Acteurs :** Produit, BondingBrother, Master Butler

**SÃ©quence :**
1. Le produit ou BondingBrother demande les capacitÃ©s d'un module
2. Master Butler reÃ§oit la requÃªte avec le contexte
3. Master Butler filtre les capacitÃ©s selon le contexte (si applicable)
4. Master Butler retourne la liste des capacitÃ©s avec leurs mÃ©tadonnÃ©es
5. Le demandeur utilise ces informations

**RÃ¨gles :**
- La dÃ©couverte ne rÃ©vÃ¨le pas les capacitÃ©s confidentielles aux contextes non autorisÃ©s
- La dÃ©couverte retourne les mÃ©tadonnÃ©es complÃ¨tes des capacitÃ©s

### Flux d'interrogation par StrongFather

**Acteurs :** StrongFather, Master Butler

**SÃ©quence :**
1. StrongFather Ã©value une intention
2. StrongFather demande Ã  Master Butler : "Cette capacitÃ© existe-t-elle ?"
3. Master Butler rÃ©pond avec les informations de la capacitÃ©
4. StrongFather demande : "Quelles permissions sont requises ?"
5. Master Butler rÃ©pond avec les permissions associÃ©es
6. StrongFather poursuit son Ã©valuation avec ces informations

**RÃ¨gles :**
- StrongFather est toujours autorisÃ© Ã  interroger Master Butler
- Les rÃ©ponses sont exhaustives et exactes
- Master Butler ne suggÃ¨re pas de dÃ©cision

### Flux de calcul de contexte de capacitÃ©

**Acteurs :** BondingBrother, Master Butler

**SÃ©quence :**
1. BondingBrother traduit une intention et a besoin du contexte de capacitÃ©
2. BondingBrother fournit le contexte (utilisateur, rÃ´les, module cible)
3. Master Butler calcule les capacitÃ©s accessibles dans ce contexte
4. Master Butler retourne le contexte de capacitÃ©
5. BondingBrother utilise ces informations pour la traduction

**RÃ¨gles :**
- Le calcul de contexte ne modifie pas le registre
- Le calcul respecte les associations rÃ´les-permissions-capacitÃ©s
- Le rÃ©sultat est une projection, pas une dÃ©cision

---

## 9. Vocabulaire canonique

### CapacitÃ© (Capability)

Une **capacitÃ©** est un pouvoir technique intrinsÃ¨que Ã  un composant. Elle reprÃ©sente ce que le composant peut faire fonctionnellement, indÃ©pendamment de toute permission ou dÃ©cision. Une capacitÃ© est identifiÃ©e par un identifiant unique, possÃ¨de des mÃ©tadonnÃ©es descriptives, et est dÃ©clarÃ©e par le composant qui la possÃ¨de.

### Permission

Une **permission** est un droit dÃ©finit dans le systÃ¨me pour accÃ©der Ã  une ou plusieurs capacitÃ©s. Elle reprÃ©sente l'autorisation conceptuelle d'utiliser des capacitÃ©s, mais ne garantit pas l'autorisation finale (qui dÃ©pend de StrongFather). Une permission est nommÃ©e, associÃ©e Ã  des capacitÃ©s, et peut Ãªtre attribuÃ©e Ã  des rÃ´les ou des contextes.

### Registre

Le **registre** est la structure de donnÃ©es centrale de Master Butler qui contient l'inventaire exhaustif des capacitÃ©s et des permissions. Il est dynamique, traÃ§able, et constitue la source de vÃ©ritÃ© pour les informations sur les possibilitÃ©s du systÃ¨me.

### DÃ©claration

Une **dÃ©claration** est l'acte par lequel un composant (module ou produit) informe Master Butler de ses capacitÃ©s. La dÃ©claration est obligatoire pour toute capacitÃ© exposÃ©e et doit inclure les mÃ©tadonnÃ©es requises.

### DÃ©finition

Une **dÃ©finition** est l'acte par lequel un produit crÃ©e une permission dans Master Butler. La dÃ©finition inclut le nom de la permission, ses associations aux capacitÃ©s, et ses mÃ©tadonnÃ©es.

### Contexte de capacitÃ©

Le **contexte de capacitÃ©** est l'ensemble des informations qui dÃ©crivent les capacitÃ©s et permissions disponibles dans une situation donnÃ©e. Il inclut l'identitÃ© du demandeur, ses rÃ´les, le composant ciblÃ©, et les capacitÃ©s accessibles.

### MÃ©tadonnÃ©es

Les **mÃ©tadonnÃ©es** sont les informations descriptives associÃ©es Ã  une capacitÃ© ou une permission : nom, description, module d'origine, date de crÃ©ation, version, etc. Elles permettent la documentation et la dÃ©couverte.

### Association

Une **association** est le lien entre une permission et une ou plusieurs capacitÃ©s. L'association indique quelles capacitÃ©s sont couvertes par une permission.

### DÃ©couverte

La **dÃ©couverte** est le processus par lequel un composant interroge Master Butler pour connaÃ®tre les capacitÃ©s et permissions existantes. La dÃ©couverte permet l'exploration dynamique des possibilitÃ©s du systÃ¨me.

### RÃ´le

Un **rÃ´le** est un ensemble nommÃ© de permissions. Master Butler connaÃ®t les associations entre rÃ´les et permissions, mais ne gÃ¨re pas les attributions de rÃ´les aux utilisateurs (qui appartiennent au systÃ¨me d'identitÃ©).

### Tool (Outil)

Un **Tool** est une capacitÃ© exÃ©cutable, sans autoritÃ©, sans dÃ©cision mÃ©tier, sans connaissance du produit appelant, gouvernÃ©e par les Cores.

**CaractÃ©ristiques d'un Tool :**
- CapacitÃ© exÃ©cutable atomique
- Sans autoritÃ© (ne dÃ©cide jamais)
- Sans logique mÃ©tier
- GouvernÃ© par les Cores

**ðŸ‘‰ Un Tool fait, mais ne dÃ©cide jamais.**

### Toolkit

Un **Toolkit** est une composition officielle de Tools, validÃ©e et dÃ©clarÃ©e par l'environnement, optimisÃ©e pour efficience, cohÃ©rence et performance.

**CaractÃ©ristiques d'un Toolkit :**
- AgrÃ¨ge des Tools existants
- Ne crÃ©e pas de capacitÃ© nouvelle
- Sans logique mÃ©tier
- ValidÃ© par l'environnement

**ðŸ‘‰ Un Toolkit orchestre, mais n'ajoute pas de capacitÃ©.**

**Documentation complÃ¨te :** [Miyukini Conceptual References - Tools et Toolkits](..//..//..//miyukini-webway-system//reference//_index.md)

---

## 10. ResponsabilitÃ© spÃ©cifique : Gouvernance des Tools et Toolkits

### RÃ´le de Master Butler dans la gouvernance des Tools

Master Butler est le **catalogue central** des Tools et Toolkits. Il est responsable de :

| ResponsabilitÃ© | Description |
|----------------|-------------|
| **DÃ©clarer** | Quels Tools existent dans l'environnement |
| **Lier** | Capability â†’ Tool |
| **DÃ©finir les Toolkits** | Quels Tools composent chaque Toolkit |
| **Autoriser** | Qui peut appeler quel Tool/Toolkit |

### Ce que Master Butler fait pour les Tools

| Action | Oui/Non |
|--------|---------|
| DÃ©clare l'existence des Tools | âœ… Oui |
| Lie les capacitÃ©s aux Tools | âœ… Oui |
| DÃ©finit les permissions d'accÃ¨s | âœ… Oui |
| Catalogue les Toolkits | âœ… Oui |

### Ce que Master Butler NE fait PAS pour les Tools

| Action | Oui/Non | Pourquoi |
|--------|---------|----------|
| ImplÃ©menter les Tools | âŒ Non | Master Butler catalogue, n'implÃ©mente pas |
| ExÃ©cuter les Tools | âŒ Non | L'exÃ©cution appartient aux Tools eux-mÃªmes |
| DÃ©cider de l'usage | âŒ Non | StrongFather dÃ©cide |
| GÃ©rer le cycle de vie | âŒ Non | Ever Buddy gÃ¨re le cycle de vie |

### Question Ã  laquelle Master Butler rÃ©pond

> *"Qu'est-ce qui est possible dans cet environnement ?"*

Pour les Tools, cela se traduit par :
- Quels Tools sont disponibles ?
- Quels Toolkits sont dÃ©clarÃ©s ?
- Qui peut appeler quel Tool ?
- Quelles permissions sont requises pour un Tool ?

### RÃ¨gle ABSOLUE

> **Un environnement Miyukini possÃ¨de une bibliothÃ¨que d'outils finie, dÃ©clarÃ©e, gouvernÃ©e.**

| RÃ¨gle | Description |
|-------|-------------|
| **Pas d'injection sauvage** | Aucun Tool ne peut Ãªtre ajoutÃ© sans dÃ©claration dans Master Butler |
| **Pas de Tool "local"** | Tout Tool doit Ãªtre dÃ©clarÃ© dans l'environnement |
| **Pas de dÃ©pendance externe cachÃ©e** | Aucune librairie externe non gouvernÃ©e |

**ðŸ‘‰ C'est une souverainetÃ© applicative.**

---

## 11. ConformitÃ© aux Lois d'Autonomie SystÃ¨me

Ce core respecte les Lois d'Autonomie SystÃ¨me dÃ©finies dans [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//miyukini-webway-system//reference//_index.md).

### LOI-1 : Aucune dÃ©pendance externe critique Ã  l'exÃ©cution

**ConformitÃ© :** Conforme

Master Butler est un registre local des capacitÃ©s et permissions. Toutes ses fonctions fondamentales opÃ¨rent sans dÃ©pendance externe :

- **Registre local** : Les capacitÃ©s et permissions sont dÃ©clarÃ©es et stockÃ©es localement. Aucun service distant n'est requis pour maintenir ou consulter le registre.
- **Interrogations locales** : StrongFather, BondingBrother, et les produits interrogent Master Butler via des appels locaux. Aucune API externe n'intervient dans ces flux.
- **DÃ©clarations locales** : Les modules et produits dÃ©clarent leurs capacitÃ©s directement Ã  Master Butler sans passer par un service externe.
- **DÃ©couverte locale** : La dÃ©couverte des capacitÃ©s et permissions fonctionne entiÃ¨rement en local, permettant aux composants d'explorer les possibilitÃ©s du systÃ¨me sans connexion.

**VÃ©rification LOI-1** : *"Master Butler fonctionne-t-il si le rÃ©seau est indisponible ?"* â†’ **Oui.** Le registre est local, les interrogations sont locales, la dÃ©couverte est locale. Aucune fonction de Master Butler ne requiert de connexion externe.

### LOI-5 : Le coÃ»t doit Ãªtre proportionnel au hardware

**ConformitÃ© :** Conforme

Master Butler est conÃ§u pour une empreinte minimale sur les ressources systÃ¨me :

- **Registre pur** : Master Butler est un registre de mÃ©tadonnÃ©es, pas un service actif. Il ne consomme des ressources qu'Ã  la demande (lors des interrogations ou dÃ©clarations).
- **DonnÃ©es lÃ©gÃ¨res** : Les capacitÃ©s et permissions sont des mÃ©tadonnÃ©es descriptives (identifiants, noms, descriptions, associations). Ces donnÃ©es sont intrinsÃ¨quement lÃ©gÃ¨res.
- **Pas de workers permanents** : Master Butler ne lance aucun processus en arriÃ¨re-plan. Pas de services fantÃ´mes, pas de tÃ¢ches planifiÃ©es, pas de synchronisation automatique.
- **Lookups simples** : Les recherches dans le registre sont des opÃ©rations de consultation directe, optimisÃ©es pour la rapiditÃ© et la faible consommation.
- **MÃ©moire prÃ©visible** : La taille du registre est proportionnelle au nombre de modules et de permissions dÃ©finis, qui reste bornÃ© et prÃ©visible.

**VÃ©rification LOI-5** : *"Master Butler fonctionne-t-il de maniÃ¨re acceptable sur un Raspberry Pi 4 avec 4 Go de RAM ?"* â†’ **Oui.** Un registre de capacitÃ©s et permissions pour un systÃ¨me typique (quelques dizaines de modules, quelques centaines de permissions) reprÃ©sente quelques kilo-octets de donnÃ©es, avec des opÃ©rations de lookup instantanÃ©es.

### SynthÃ¨se de conformitÃ©

| Loi | Statut | Raison |
|-----|--------|--------|
| LOI-1 | âœ… Conforme | Registre local, interrogations locales, aucune dÃ©pendance externe |
| LOI-5 | âœ… Conforme | Registre pur de mÃ©tadonnÃ©es lÃ©gÃ¨res, pas de workers, consommation Ã  la demande |

Master Butler respecte pleinement les lois d'autonomie applicables Ã  sa nature de registre passif. Sa conception en tant que rÃ©pertoire de mÃ©tadonnÃ©es consultable garantit une empreinte minimale et une indÃ©pendance totale vis-Ã -vis des ressources externes.

---

## 11. Conclusion et statut contractuel

### Essence de Master Butler

Master Butler est le gardien de la connaissance des possibilitÃ©s dans l'Ã©cosystÃ¨me Miyukini. Il recense les capacitÃ©s, dÃ©finit les permissions, et fournit ces informations Ã  tous les composants qui en ont besoin, sans jamais prendre de dÃ©cision, sans jamais exÃ©cuter d'action, sans jamais appliquer de rÃ¨gle mÃ©tier.

Master Butler incarne la sÃ©paration entre la connaissance (ce qui existe) et la dÃ©cision (ce qui est autorisÃ©). Cette sÃ©paration est fondamentale pour maintenir la clartÃ© architecturale et la cohÃ©rence du systÃ¨me.

### Phrase fondatrice

**Master Butler est le registre central des capacitÃ©s et permissions du systÃ¨me Miyukini, exposant ce qui est possible sans jamais dÃ©cider de ce qui est autorisÃ©.**

Cette phrase rÃ©sume l'essence de Master Butler : registre (pas dÃ©cideur), central (pas dispersÃ©), capacitÃ©s et permissions (pas donnÃ©es mÃ©tier), exposant (pas dÃ©cidant).

Toute implÃ©mentation de Master Butler doit respecter cette phrase fondatrice. Toute Ã©volution de Master Butler doit prÃ©server cette essence. Toute spÃ©cialisation de Master Butler doit rester fidÃ¨le Ã  cette nature.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues qui ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

Toute implÃ©mentation de Master Butler doit respecter intÃ©gralement ce document. Toute Ã©volution de Master Butler doit prÃ©server les invariants dÃ©finis ici. Toute spÃ©cialisation de Master Butler doit rester fidÃ¨le Ã  la nature dÃ©crite ici.

### Relation contractuelle avec les autres cores

Ce document s'articule avec les documentations fondatrices des autres cores :
- **KindMother** : Master Butler peut utiliser KindMother pour persister son registre, mais ne gÃ¨re pas directement la persistance
- **StrongFather** : Master Butler fournit les informations que StrongFather utilise pour ses dÃ©cisions, sans jamais participer Ã  ces dÃ©cisions
- **BondingBrother** : Master Butler rÃ©pond aux interrogations de BondingBrother pour la traduction des intentions

Aucune contradiction n'existe entre ces documents. Ils forment un ensemble cohÃ©rent qui dÃ©finit l'architecture conceptuelle de l'Ã©cosystÃ¨me Miyukini.

---

**Version :** 1.4  
**Date :** 2026-01-27  
**Statut :** FONDATION â€” Non nÃ©gociable  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, [Miyukini Conceptual References - Tools et Toolkits](..//..//..//miyukini-webway-system//reference//_index.md) (gouvernance des Tools et Toolkits), [Miyukini Conceptual References - External Signal Trust Reinforcement Contract](..//..//..//miyukini-webway-system//reference//_index.md) (capacitÃ©s exposÃ©es lors du bootstrap Internet), [Miyukini Conceptual References - Security Protocols](..//..//..//miyukini-webway-system//reference//_index.md) (authentification en couches RT-SEC-2, validation permission RT-SEC-3, revalidation AS-SEC-3), [Miyukini Conceptual References - Security Levels](..//..//..//miyukini-webway-system//reference//_index.md) (adaptation permissions selon niveau sÃ©curitÃ© 0-4)

