# Master Butler - Documentation Fondatrice

## 1. Introduction

### Rôle de Master Butler

Master Butler est le **Capability & Permission Core** du Miyukini Core System. Il incarne la connaissance de ce qui est possible : quelles capacités existent dans le système, quelles permissions sont définies, et quels droits peuvent être accordés.

Master Butler répond à une question fondamentale : **"Que peut-on faire ici, et qui a le droit de le faire ?"**

Cette question est distincte de la décision ("devrait-on le faire ?") qui appartient à StrongFather, et distincte de l'exécution ("comment le faire ?") qui appartient aux produits et à KindMother.

Master Butler est le registre vivant des possibilités du système. Il ne décide jamais, il n'exécute jamais, il n'autorise jamais. Il expose, il recense, il documente ce qui existe comme capacités et ce qui est défini comme permissions.

### Question fondamentale

Master Butler existe pour répondre à cette question unique et fondamentale :

**"Quelles sont les capacités du système, et quelles permissions existent pour y accéder ?"**

Cette question se décline en sous-questions :
- Quelles actions sont techniquement possibles dans ce module ?
- Quelles permissions sont définies pour accéder à ces capacités ?
- Quels rôles portent quelles permissions ?
- Quelles capacités sont disponibles pour un contexte donné ?

Master Butler fournit ces informations de manière exhaustive, cohérente, et traçable. Il est le cartographe des possibilités, jamais le décideur de leur usage.

---

## 2. Raison d'être

### Problème que Master Butler résout

Dans un système modulaire comme Miyukini, les capacités sont dispersées dans les modules, les adaptateurs, et les produits. Sans centralisation de cette connaissance, plusieurs problèmes émergent :

1. **Opacité des possibilités** : Aucun composant ne sait quelles capacités existent ailleurs dans le système. Chaque module connaît ses propres capacités, mais ignore celles des autres.

2. **Définitions de permissions dispersées** : Les permissions sont définies localement, sans registre central. Cela conduit à des duplications, des incohérences, et des zones d'ombre.

3. **Impossibilité de répondre "que puis-je faire ?"** : Sans registre central, un produit ou un utilisateur ne peut pas obtenir la liste des capacités accessibles dans un contexte donné.

4. **Couplage entre connaissance et décision** : Les composants mélangent la connaissance de ce qui est possible avec la décision de ce qui est autorisé, créant de la confusion architecturale.

5. **Absence de découverte** : Aucun mécanisme ne permet de découvrir dynamiquement les capacités du système, obligeant un codage en dur des fonctionnalités.

### Ce que Master Butler apporte

Master Butler résout ces problèmes en fournissant :

- **Un registre central des capacités** : Toutes les capacités du système sont recensées, documentées, et accessibles via Master Butler.

- **Un registre central des permissions** : Toutes les permissions sont définies, nommées, et organisées de manière cohérente.

- **Une API de découverte** : Les composants peuvent interroger Master Butler pour découvrir les capacités et permissions disponibles.

- **Une séparation claire** : Master Butler sépare la connaissance (ce qui existe) de la décision (ce qui est autorisé). Il fournit les informations, StrongFather décide.

### Nécessité architecturale

Sans Master Butler, l'écosystème Miyukini serait incapable de répondre aux questions fondamentales sur ses propres capacités. Les produits devraient maintenir leurs propres registres, créant des duplications et des incohérences. La décision serait basée sur des informations partielles ou obsolètes.

Master Butler est nécessaire parce que la connaissance des possibilités doit être centralisée, cohérente, et accessible. Cette connaissance est la base sur laquelle StrongFather prend ses décisions, et sans laquelle aucune décision éclairée n'est possible.

---

## 3. Positionnement familial

### Relation avec StrongFather

Master Butler et StrongFather forment un couple complémentaire et indissociable :

- **Master Butler** expose ce qui est possible (capacités et permissions)
- **StrongFather** décide ce qui est autorisé (évaluation des intentions)

Cette relation est asymétrique : StrongFather dépend de Master Butler pour connaître les possibilités, mais Master Butler ne dépend pas de StrongFather pour exister. Master Butler recense, StrongFather décide.

**Flux typique :**
1. Un produit exprime une intention (via BondingBrother)
2. StrongFather interroge Master Butler : "Cette capacité existe-t-elle ? Quelles permissions sont requises ?"
3. Master Butler répond avec les informations demandées
4. StrongFather évalue l'intention selon les politiques et les permissions
5. StrongFather produit une décision

Master Butler ne prend jamais part à la décision. Il fournit les informations nécessaires, sans jugement, sans interprétation, sans recommandation.

### Relation avec KindMother

Master Butler et KindMother opèrent dans des domaines distincts mais complémentaires :

- **KindMother** gère les données (persistance, synchronisation, cohérence)
- **Master Butler** gère la connaissance des capacités et permissions (pas des données)

Master Butler ne stocke pas de données métier. Il maintient un registre de métadonnées : quelles capacités existent, quelles permissions sont définies. Ces métadonnées peuvent être persistées via KindMother, mais Master Butler ne gère jamais directement la persistance.

La relation est indirecte : Master Butler utilise KindMother comme support de persistance pour son registre, mais ne connaît pas les détails de cette persistance.

### Relation avec BondingBrother

Master Butler est interrogé par BondingBrother lorsque celui-ci traduit des intentions :

- BondingBrother peut demander à Master Butler : "Cette capacité existe-t-elle dans ce module ?"
- BondingBrother peut demander : "Quelles permissions sont requises pour cette action ?"

Master Butler fournit ces informations, permettant à BondingBrother de traduire correctement les intentions et de préparer le contexte pour l'évaluation par StrongFather.

### Relation avec les produits

Les produits enregistrent leurs capacités auprès de Master Butler :

- Lors de leur initialisation, les produits déclarent leurs capacités à Master Butler
- Les produits définissent les permissions qu'ils reconnaissent
- Les produits peuvent interroger Master Butler pour découvrir d'autres capacités

Cette relation est bidirectionnelle : les produits alimentent Master Butler (déclaration) et consomment Master Butler (découverte).

### Position dans la famille Miyukini

Dans la famille Miyukini, Master Butler est le majordome de la maison : il connaît chaque pièce, chaque équipement, chaque règle d'accès. Il ne prend pas les décisions (c'est le rôle des parents), il n'exécute pas les tâches (c'est le rôle des enfants), mais il sait tout ce qui est possible et peut répondre à toute question sur les capacités de la maison.

Master Butler est au service de tous, sans jamais prendre parti. Il informe, il recense, il expose, mais il ne juge jamais.

---

## 4. Concepts fondamentaux

### Capacité (Capability)

Une **capacité** est un pouvoir technique qu'un composant possède. C'est ce qu'un module, un adaptateur, ou un produit peut faire techniquement, indépendamment des permissions.

**Caractéristiques d'une capacité :**
- Elle est intrinsèque au composant (le composant la possède ou ne la possède pas)
- Elle est technique (elle décrit un pouvoir fonctionnel)
- Elle est déclarative (elle est déclarée par le composant qui la possède)
- Elle est identifiable (elle a un identifiant unique et stable)
- Elle est documentée (elle a une description et des métadonnées)

**Exemples de capacités :**
- `content.create` : Capacité de créer du contenu
- `hierarchy.reorder` : Capacité de réorganiser une hiérarchie
- `media.upload` : Capacité de téléverser des médias
- `search.index` : Capacité d'indexer pour la recherche

Une capacité existe indépendamment de toute permission. Un module peut posséder la capacité de supprimer du contenu, même si aucune permission n'autorise cette suppression.

### Permission

Une **permission** est un droit accordé pour accéder à une capacité. C'est l'autorisation conceptuelle d'utiliser une capacité, indépendamment de la décision finale.

**Caractéristiques d'une permission :**
- Elle est définie (elle est créée et nommée explicitement)
- Elle est associée à une ou plusieurs capacités
- Elle est attribuable (elle peut être accordée à des rôles ou des contextes)
- Elle est révocable (elle peut être retirée)
- Elle est traçable (son attribution est enregistrée)

**Exemples de permissions :**
- `content.create.any` : Permission de créer n'importe quel contenu
- `content.edit.own` : Permission de modifier son propre contenu
- `hierarchy.manage` : Permission de gérer les hiérarchies
- `media.delete.all` : Permission de supprimer tous les médias

Une permission ne garantit pas l'autorisation finale. StrongFather évalue les permissions dans le contexte des politiques pour produire une décision.

### Distinction fondamentale Capacité vs Permission

| Aspect | Capacité | Permission |
|--------|----------|------------|
| Nature | Pouvoir technique | Droit accordé |
| Origine | Intrinsèque au composant | Définie par le système |
| Question | "Peut-on le faire techniquement ?" | "A-t-on le droit de le faire ?" |
| Possession | Le composant la possède | Le contexte (rôle, utilisateur) la détient |
| Existence | Indépendante des permissions | Associée aux capacités |

**Métaphore :** Une serrure (capacité) existe sur une porte. Une clé (permission) permet d'ouvrir cette serrure. Avoir la clé ne signifie pas qu'on a le droit d'entrer (décision de StrongFather), mais sans la clé, on ne peut pas entrer du tout.

### Registre des capacités

Le **registre des capacités** est la structure centrale de Master Butler. Il contient :
- L'inventaire exhaustif des capacités du système
- Les métadonnées de chaque capacité (nom, description, module d'origine)
- Les relations entre capacités (dépendances, hiérarchies)
- L'historique des capacités (ajouts, suppressions, modifications)

Le registre est dynamique : il évolue avec le système, au fur et à mesure que les modules déclarent leurs capacités.

### Registre des permissions

Le **registre des permissions** est la seconde structure centrale de Master Butler. Il contient :
- L'inventaire exhaustif des permissions définies
- Les associations entre permissions et capacités
- Les métadonnées de chaque permission (nom, description, niveau)
- L'historique des permissions (créations, modifications, révocations)

Le registre des permissions est distinct du registre des capacités, mais ils sont liés : chaque permission référence une ou plusieurs capacités.

### Contexte de capacité

Un **contexte de capacité** est l'ensemble des informations qui définissent les capacités et permissions disponibles dans une situation donnée. Le contexte inclut :
- L'identité du demandeur (utilisateur, système, produit)
- Les rôles du demandeur
- Les permissions associées à ces rôles
- Le module ou le composant ciblé
- Les capacités disponibles dans ce composant

Master Butler peut calculer le contexte de capacité pour répondre à la question : "Dans cette situation, quelles capacités sont accessibles et avec quelles permissions ?"

---

## 5. Responsabilités exclusives

### Recensement des capacités

Master Butler est **exclusivement responsable** du recensement de toutes les capacités du système. Aucun autre composant ne maintient de registre des capacités. Tout composant souhaitant connaître les capacités disponibles doit interroger Master Butler.

Cette responsabilité inclut :
- Réception des déclarations de capacités des modules et produits
- Validation de la structure des déclarations
- Stockage dans le registre des capacités
- Mise à jour lors des modifications
- Suppression lors des dépréciations

### Définition des permissions

Master Butler est **exclusivement responsable** de la définition formelle des permissions. Aucun autre composant ne définit de permissions. Toutes les permissions sont déclarées, nommées, et structurées dans Master Butler.

Cette responsabilité inclut :
- Création de nouvelles permissions
- Association des permissions aux capacités
- Structuration hiérarchique des permissions
- Gestion des métadonnées des permissions
- Historisation des modifications

### Fourniture des informations aux décideurs

Master Butler est **exclusivement responsable** de fournir les informations sur les capacités et permissions à StrongFather et aux autres composants qui en ont besoin.

Cette responsabilité inclut :
- Réponse aux requêtes de StrongFather sur les capacités
- Réponse aux requêtes de BondingBrother sur les permissions requises
- Fourniture du contexte de capacité aux composants autorisés
- Garantie de l'exactitude et de l'exhaustivité des informations

### Découverte des capacités

Master Butler est **exclusivement responsable** de permettre la découverte des capacités du système. Les produits et modules peuvent interroger Master Butler pour découvrir les capacités existantes.

Cette responsabilité inclut :
- API de découverte des capacités par module
- API de découverte des capacités par type d'action
- API de découverte des permissions par capacité
- Filtrage des capacités selon le contexte

### Traçabilité des définitions

Master Butler est **exclusivement responsable** de la traçabilité des définitions de capacités et permissions. Chaque création, modification, ou suppression est enregistrée avec son contexte.

Cette responsabilité inclut :
- Journalisation des déclarations de capacités
- Journalisation des définitions de permissions
- Historique des modifications
- Audit trail complet des évolutions

---

## 6. Ce que Master Butler ne fait PAS

### Ne décide pas

Master Butler **ne décide jamais** si une action est autorisée ou refusée. Il fournit les informations sur les capacités et permissions, mais la décision appartient à StrongFather. Master Butler répond "cette permission existe et ce rôle la possède", mais ne répond jamais "cette action est autorisée".

### Ne vérifie pas les permissions en temps réel

Master Butler **ne vérifie jamais** si un utilisateur ou un contexte possède effectivement une permission au moment d'une action. Cette vérification appartient à StrongFather lors de l'évaluation des intentions. Master Butler fournit les définitions, pas les vérifications.

### N'exécute pas

Master Butler **n'exécute jamais** d'action fonctionnelle. Il ne crée pas de contenu, ne modifie pas de hiérarchie, ne téléverse pas de média. Il recense les capacités qui permettent ces actions, mais ne les exécute jamais.

### Ne stocke pas de données métier

Master Butler **ne stocke jamais** de données métier. Il stocke des métadonnées : définitions de capacités, définitions de permissions, associations, historiques. Les données métier appartiennent aux modules et à KindMother.

### Ne gère pas les identités

Master Butler **ne gère jamais** les identités des utilisateurs ou des systèmes. Il connaît les rôles et les permissions associées, mais l'identité elle-même appartient au système d'authentification (hors-scope de Master Butler).

### Ne définit pas de politiques

Master Butler **ne définit jamais** de politiques de décision. Les politiques (règles qui déterminent quand une permission est accordée ou refusée) appartiennent à StrongFather. Master Butler définit ce qui existe, pas comment l'utiliser.

### N'applique pas de contraintes métier

Master Butler **n'applique jamais** de contraintes métier. Si une règle métier dit "un utilisateur ne peut créer que 10 contenus par jour", cette contrainte appartient à StrongFather ou au produit, pas à Master Butler. Master Butler sait que la capacité de créer du contenu existe, mais ignore les limites métier.

### Ne persiste pas directement

Master Butler **ne gère jamais** directement la persistance. Si son registre doit être persisté, il utilise KindMother comme support, mais ne manipule jamais directement une base de données ou un système de fichiers.

---

## 7. Invariants non négociables

### INV-MB-1 : Exhaustivité du registre

Le registre de Master Butler est **exhaustif**. Toute capacité existant dans le système est recensée dans Master Butler. Si une capacité n'est pas dans le registre, elle n'existe pas officiellement dans le système.

**Implication :** Aucun module ne peut exposer une capacité sans la déclarer à Master Butler. Aucun contournement n'est permis.

### INV-MB-2 : Non-décision

Master Butler **ne prend jamais de décision**. Il fournit des informations, répond à des questions, mais ne produit jamais de verdict "autorisé" ou "refusé". Toute décision appartient à StrongFather.

**Implication :** Aucune méthode de Master Butler ne retourne un booléen d'autorisation. Il retourne des informations, pas des décisions.

### INV-MB-3 : Idempotence des déclarations

Les déclarations de capacités sont **idempotentes**. Déclarer deux fois la même capacité n'a pas d'effet supplémentaire. Le registre reste cohérent quel que soit l'ordre ou le nombre de déclarations.

**Implication :** Les modules peuvent redéclarer leurs capacités à chaque démarrage sans effet indésirable.

### INV-MB-4 : Immutabilité des identifiants

Les identifiants de capacités sont **immuables**. Une fois qu'une capacité est déclarée avec un identifiant, cet identifiant ne change jamais. Si une capacité évolue significativement, une nouvelle capacité est créée avec un nouvel identifiant.

**Implication :** Les références aux capacités (dans les permissions, les logs, les configurations) restent valides dans le temps.

### INV-MB-5 : Traçabilité complète

Toute modification du registre de Master Butler est **tracée**. Créations, modifications, suppressions : tout est enregistré avec le contexte (qui, quand, pourquoi).

**Implication :** L'historique des capacités et permissions est auditable. Aucune modification silencieuse n'est possible.

### INV-MB-6 : Séparation capacité/permission

Les capacités et les permissions sont **strictement séparées**. Une capacité existe indépendamment des permissions. Une permission référence des capacités mais ne les définit pas.

**Implication :** La suppression d'une permission n'affecte pas la capacité associée. La suppression d'une capacité invalide les permissions qui la référencent.

### INV-MB-7 : Pas de logique métier

Master Butler **ne contient aucune logique métier**. Il ne connaît pas les règles du domaine, les contraintes applicatives, les limites fonctionnelles. Il sait ce qui est techniquement possible, pas ce qui est métier-compatible.

**Implication :** Master Butler ne valide jamais une action selon des critères métier. Cette validation appartient aux modules et à StrongFather.

### INV-MB-8 : Accessibilité universelle

Master Butler est **accessible à tous les composants autorisés** du système. Aucun composant ne peut être empêché d'interroger Master Butler sur les capacités et permissions (sous réserve des permissions d'accès à Master Butler lui-même).

**Implication :** Master Butler est un service partagé, pas un composant isolé. Son accessibilité est garantie.

---

## 8. Interactions avec l'écosystème

### Flux de déclaration de capacités

**Acteurs :** Module SPM, Produit, Master Butler

**Séquence :**
1. Le module ou produit démarre et identifie ses capacités
2. Le module ou produit envoie une déclaration à Master Butler
3. Master Butler valide la structure de la déclaration
4. Master Butler enregistre les capacités dans le registre
5. Master Butler confirme l'enregistrement
6. Le module ou produit est opérationnel

**Règles :**
- La déclaration est obligatoire pour toute capacité exposée
- La déclaration peut être effectuée plusieurs fois (idempotence)
- La déclaration inclut les métadonnées (nom, description, module d'origine)

### Flux de définition de permissions

**Acteurs :** Produit, Master Butler

**Séquence :**
1. Le produit définit une nouvelle permission
2. Le produit associe la permission à des capacités existantes
3. Le produit envoie la définition à Master Butler
4. Master Butler valide l'existence des capacités référencées
5. Master Butler enregistre la permission dans le registre
6. Master Butler confirme l'enregistrement

**Règles :**
- Une permission doit référencer au moins une capacité existante
- Une permission ne peut pas référencer une capacité inexistante
- Les métadonnées de permission sont obligatoires

### Flux de découverte de capacités

**Acteurs :** Produit, BondingBrother, Master Butler

**Séquence :**
1. Le produit ou BondingBrother demande les capacités d'un module
2. Master Butler reçoit la requête avec le contexte
3. Master Butler filtre les capacités selon le contexte (si applicable)
4. Master Butler retourne la liste des capacités avec leurs métadonnées
5. Le demandeur utilise ces informations

**Règles :**
- La découverte ne révèle pas les capacités confidentielles aux contextes non autorisés
- La découverte retourne les métadonnées complètes des capacités

### Flux d'interrogation par StrongFather

**Acteurs :** StrongFather, Master Butler

**Séquence :**
1. StrongFather évalue une intention
2. StrongFather demande à Master Butler : "Cette capacité existe-t-elle ?"
3. Master Butler répond avec les informations de la capacité
4. StrongFather demande : "Quelles permissions sont requises ?"
5. Master Butler répond avec les permissions associées
6. StrongFather poursuit son évaluation avec ces informations

**Règles :**
- StrongFather est toujours autorisé à interroger Master Butler
- Les réponses sont exhaustives et exactes
- Master Butler ne suggère pas de décision

### Flux de calcul de contexte de capacité

**Acteurs :** BondingBrother, Master Butler

**Séquence :**
1. BondingBrother traduit une intention et a besoin du contexte de capacité
2. BondingBrother fournit le contexte (utilisateur, rôles, module cible)
3. Master Butler calcule les capacités accessibles dans ce contexte
4. Master Butler retourne le contexte de capacité
5. BondingBrother utilise ces informations pour la traduction

**Règles :**
- Le calcul de contexte ne modifie pas le registre
- Le calcul respecte les associations rôles-permissions-capacités
- Le résultat est une projection, pas une décision

---

## 9. Vocabulaire canonique

### Capacité (Capability)

Une **capacité** est un pouvoir technique intrinsèque à un composant. Elle représente ce que le composant peut faire fonctionnellement, indépendamment de toute permission ou décision. Une capacité est identifiée par un identifiant unique, possède des métadonnées descriptives, et est déclarée par le composant qui la possède.

### Permission

Une **permission** est un droit définit dans le système pour accéder à une ou plusieurs capacités. Elle représente l'autorisation conceptuelle d'utiliser des capacités, mais ne garantit pas l'autorisation finale (qui dépend de StrongFather). Une permission est nommée, associée à des capacités, et peut être attribuée à des rôles ou des contextes.

### Registre

Le **registre** est la structure de données centrale de Master Butler qui contient l'inventaire exhaustif des capacités et des permissions. Il est dynamique, traçable, et constitue la source de vérité pour les informations sur les possibilités du système.

### Déclaration

Une **déclaration** est l'acte par lequel un composant (module ou produit) informe Master Butler de ses capacités. La déclaration est obligatoire pour toute capacité exposée et doit inclure les métadonnées requises.

### Définition

Une **définition** est l'acte par lequel un produit crée une permission dans Master Butler. La définition inclut le nom de la permission, ses associations aux capacités, et ses métadonnées.

### Contexte de capacité

Le **contexte de capacité** est l'ensemble des informations qui décrivent les capacités et permissions disponibles dans une situation donnée. Il inclut l'identité du demandeur, ses rôles, le composant ciblé, et les capacités accessibles.

### Métadonnées

Les **métadonnées** sont les informations descriptives associées à une capacité ou une permission : nom, description, module d'origine, date de création, version, etc. Elles permettent la documentation et la découverte.

### Association

Une **association** est le lien entre une permission et une ou plusieurs capacités. L'association indique quelles capacités sont couvertes par une permission.

### Découverte

La **découverte** est le processus par lequel un composant interroge Master Butler pour connaître les capacités et permissions existantes. La découverte permet l'exploration dynamique des possibilités du système.

### Rôle

Un **rôle** est un ensemble nommé de permissions. Master Butler connaît les associations entre rôles et permissions, mais ne gère pas les attributions de rôles aux utilisateurs (qui appartiennent au système d'identité).

### Tool (Outil)

Un **Tool** est une capacité exécutable, sans autorité, sans décision métier, sans connaissance du produit appelant, gouvernée par les Cores.

**Caractéristiques d'un Tool :**
- Capacité exécutable atomique
- Sans autorité (ne décide jamais)
- Sans logique métier
- Gouverné par les Cores

**👉 Un Tool fait, mais ne décide jamais.**

### Toolkit

Un **Toolkit** est une composition officielle de Tools, validée et déclarée par l'environnement, optimisée pour efficience, cohérence et performance.

**Caractéristiques d'un Toolkit :**
- Agrège des Tools existants
- Ne crée pas de capacité nouvelle
- Sans logique métier
- Validé par l'environnement

**👉 Un Toolkit orchestre, mais n'ajoute pas de capacité.**

**Documentation complète :** [Miyukini Conceptual References - Tools et Toolkits](../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)

---

## 10. Responsabilité spécifique : Gouvernance des Tools et Toolkits

### Rôle de Master Butler dans la gouvernance des Tools

Master Butler est le **catalogue central** des Tools et Toolkits. Il est responsable de :

| Responsabilité | Description |
|----------------|-------------|
| **Déclarer** | Quels Tools existent dans l'environnement |
| **Lier** | Capability → Tool |
| **Définir les Toolkits** | Quels Tools composent chaque Toolkit |
| **Autoriser** | Qui peut appeler quel Tool/Toolkit |

### Ce que Master Butler fait pour les Tools

| Action | Oui/Non |
|--------|---------|
| Déclare l'existence des Tools | ✅ Oui |
| Lie les capacités aux Tools | ✅ Oui |
| Définit les permissions d'accès | ✅ Oui |
| Catalogue les Toolkits | ✅ Oui |

### Ce que Master Butler NE fait PAS pour les Tools

| Action | Oui/Non | Pourquoi |
|--------|---------|----------|
| Implémenter les Tools | ❌ Non | Master Butler catalogue, n'implémente pas |
| Exécuter les Tools | ❌ Non | L'exécution appartient aux Tools eux-mêmes |
| Décider de l'usage | ❌ Non | StrongFather décide |
| Gérer le cycle de vie | ❌ Non | Ever Buddy gère le cycle de vie |

### Question à laquelle Master Butler répond

> *"Qu'est-ce qui est possible dans cet environnement ?"*

Pour les Tools, cela se traduit par :
- Quels Tools sont disponibles ?
- Quels Toolkits sont déclarés ?
- Qui peut appeler quel Tool ?
- Quelles permissions sont requises pour un Tool ?

### Règle ABSOLUE

> **Un environnement Miyukini possède une bibliothèque d'outils finie, déclarée, gouvernée.**

| Règle | Description |
|-------|-------------|
| **Pas d'injection sauvage** | Aucun Tool ne peut être ajouté sans déclaration dans Master Butler |
| **Pas de Tool "local"** | Tout Tool doit être déclaré dans l'environnement |
| **Pas de dépendance externe cachée** | Aucune librairie externe non gouvernée |

**👉 C'est une souveraineté applicative.**

---

## 11. Conformité aux Lois d'Autonomie Système

Ce core respecte les Lois d'Autonomie Système définies dans [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md).

### LOI-1 : Aucune dépendance externe critique à l'exécution

**Conformité :** Conforme

Master Butler est un registre local des capacités et permissions. Toutes ses fonctions fondamentales opèrent sans dépendance externe :

- **Registre local** : Les capacités et permissions sont déclarées et stockées localement. Aucun service distant n'est requis pour maintenir ou consulter le registre.
- **Interrogations locales** : StrongFather, BondingBrother, et les produits interrogent Master Butler via des appels locaux. Aucune API externe n'intervient dans ces flux.
- **Déclarations locales** : Les modules et produits déclarent leurs capacités directement à Master Butler sans passer par un service externe.
- **Découverte locale** : La découverte des capacités et permissions fonctionne entièrement en local, permettant aux composants d'explorer les possibilités du système sans connexion.

**Vérification LOI-1** : *"Master Butler fonctionne-t-il si le réseau est indisponible ?"* → **Oui.** Le registre est local, les interrogations sont locales, la découverte est locale. Aucune fonction de Master Butler ne requiert de connexion externe.

### LOI-5 : Le coût doit être proportionnel au hardware

**Conformité :** Conforme

Master Butler est conçu pour une empreinte minimale sur les ressources système :

- **Registre pur** : Master Butler est un registre de métadonnées, pas un service actif. Il ne consomme des ressources qu'à la demande (lors des interrogations ou déclarations).
- **Données légères** : Les capacités et permissions sont des métadonnées descriptives (identifiants, noms, descriptions, associations). Ces données sont intrinsèquement légères.
- **Pas de workers permanents** : Master Butler ne lance aucun processus en arrière-plan. Pas de services fantômes, pas de tâches planifiées, pas de synchronisation automatique.
- **Lookups simples** : Les recherches dans le registre sont des opérations de consultation directe, optimisées pour la rapidité et la faible consommation.
- **Mémoire prévisible** : La taille du registre est proportionnelle au nombre de modules et de permissions définis, qui reste borné et prévisible.

**Vérification LOI-5** : *"Master Butler fonctionne-t-il de manière acceptable sur un Raspberry Pi 4 avec 4 Go de RAM ?"* → **Oui.** Un registre de capacités et permissions pour un système typique (quelques dizaines de modules, quelques centaines de permissions) représente quelques kilo-octets de données, avec des opérations de lookup instantanées.

### Synthèse de conformité

| Loi | Statut | Raison |
|-----|--------|--------|
| LOI-1 | ✅ Conforme | Registre local, interrogations locales, aucune dépendance externe |
| LOI-5 | ✅ Conforme | Registre pur de métadonnées légères, pas de workers, consommation à la demande |

Master Butler respecte pleinement les lois d'autonomie applicables à sa nature de registre passif. Sa conception en tant que répertoire de métadonnées consultable garantit une empreinte minimale et une indépendance totale vis-à-vis des ressources externes.

---

## 11. Conclusion et statut contractuel

### Essence de Master Butler

Master Butler est le gardien de la connaissance des possibilités dans l'écosystème Miyukini. Il recense les capacités, définit les permissions, et fournit ces informations à tous les composants qui en ont besoin, sans jamais prendre de décision, sans jamais exécuter d'action, sans jamais appliquer de règle métier.

Master Butler incarne la séparation entre la connaissance (ce qui existe) et la décision (ce qui est autorisé). Cette séparation est fondamentale pour maintenir la clarté architecturale et la cohérence du système.

### Phrase fondatrice

**Master Butler est le registre central des capacités et permissions du système Miyukini, exposant ce qui est possible sans jamais décider de ce qui est autorisé.**

Cette phrase résume l'essence de Master Butler : registre (pas décideur), central (pas dispersé), capacités et permissions (pas données métier), exposant (pas décidant).

Toute implémentation de Master Butler doit respecter cette phrase fondatrice. Toute évolution de Master Butler doit préserver cette essence. Toute spécialisation de Master Butler doit rester fidèle à cette nature.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

Toute implémentation de Master Butler doit respecter intégralement ce document. Toute évolution de Master Butler doit préserver les invariants définis ici. Toute spécialisation de Master Butler doit rester fidèle à la nature décrite ici.

### Relation contractuelle avec les autres cores

Ce document s'articule avec les documentations fondatrices des autres cores :
- **KindMother** : Master Butler peut utiliser KindMother pour persister son registre, mais ne gère pas directement la persistance
- **StrongFather** : Master Butler fournit les informations que StrongFather utilise pour ses décisions, sans jamais participer à ces décisions
- **BondingBrother** : Master Butler répond aux interrogations de BondingBrother pour la traduction des intentions

Aucune contradiction n'existe entre ces documents. Ils forment un ensemble cohérent qui définit l'architecture conceptuelle de l'écosystème Miyukini.

---

**Version :** 1.4  
**Date :** 2026-01-27  
**Statut :** FONDATION — Non négociable  
**Référence :** Miyukini Core System v2.4, [Miyukini Conceptual References - Tools et Toolkits](../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) (gouvernance des Tools et Toolkits), [Miyukini Conceptual References - External Signal Trust Reinforcement Contract](../../../reference/Miyukini%20Conceptual%20References%20-%20External%20Signal%20Trust%20Reinforcement%20Contract.md) (capacités exposées lors du bootstrap Internet), [Miyukini Conceptual References - Security Protocols](../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md) (authentification en couches RT-SEC-2, validation permission RT-SEC-3, revalidation AS-SEC-3), [Miyukini Conceptual References - Security Levels](../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) (adaptation permissions selon niveau sécurité 0-4)
