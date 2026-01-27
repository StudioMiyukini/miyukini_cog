# Master Butler — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter Master Butler correctement, sans violer les contrats FONDATION.

**Objectif pédagogique :** Ce document vise à aider les développeurs à comprendre comment traduire les contrats FONDATION en implémentation, en respectant strictement les invariants, garanties, et interdictions.

**Avertissement :** Ce document ne doit pas être interprété abusivement. Il ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats FONDATION priment toujours sur ce guide.

**Relation avec les contrats FONDATION :** Ce document fait référence aux contrats FONDATION existants mais ne les étend pas, ne les modifie pas, et ne crée aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1. Objectif

Ce document fournit des lignes directrices pour implémenter Master Butler de manière conforme aux contrats FONDATION. Il explique comment traduire les concepts contractuels en logique d'implémentation sans interprétation abusive.

Master Butler est le **Capability & Permission Core** du Miyukini Core System : il recense les capacités, définit les permissions, fournit une API de découverte, mais **ne décide jamais** et **n'exécute jamais**.

### 1.2. Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats FONDATION.

### 1.3. Sources contractuelles

Ce document se base sur tous les contrats FONDATION de Master Butler, avec un focus particulier sur :

- **Documentation Fondatrice** : Invariants INV-MB-1 à INV-MB-8, responsabilités, interdictions
- **Capability Registry Contract** : Modèle du registre des capacités
- **Permission Registry Contract** : Modèle du registre des permissions
- **Capability API Contract** : Déclaration et interrogation des capacités
- **Permission API Contract** : Définition et gestion des permissions
- **Discovery API Contract** : API de découverte
- **Tool Governance Contract** : Gouvernance des Tools et Toolkits
- **[Miyukini Conceptual References — Lois Autonomie Système](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Les lignes directrices d'implémentation doivent respecter les lois d'autonomie, notamment **LOI-1** (aucune dépendance externe critique), **LOI-5** (coût proportionnel au hardware).

---

## 2. Principes généraux à respecter absolument

### 2.1. Registre pur et passif (INV-MB-1)

**Principe contractuel :**

L'invariant INV-MB-1 établit que le registre de Master Butler est **exhaustif**. Toute capacité existant dans le système est recensée dans Master Butler. Si une capacité n'est pas dans le registre, elle n'existe pas officiellement dans le système.

**Traduction en logique d'implémentation :**

- **Registre exhaustif** : Toutes les capacités du système DOIVENT être présentes dans le registre. Aucune capacité non déclarée ne peut exister officiellement.

- **Enregistrement obligatoire** : Aucun module ne peut exposer une capacité sans la déclarer à Master Butler. Tout contournement est interdit.

- **Source de vérité unique** : Le registre de Master Butler est la seule source de vérité pour les capacités et permissions. Aucun registre parallèle n'est autorisé.

**Ce que cela signifie concrètement :**

- Tout module DOIT déclarer ses capacités à Master Butler lors de son initialisation
- Les capacités non déclarées sont considérées comme inexistantes
- Le registre est l'unique référence pour toute interrogation sur les possibilités du système

### 2.2. Non-décision absolue (INV-MB-2)

**Principe contractuel :**

L'invariant INV-MB-2 établit que Master Butler **ne prend jamais de décision**. Il fournit des informations, répond à des questions, mais ne produit jamais de verdict "autorisé" ou "refusé". Toute décision appartient à StrongFather.

**Traduction en logique d'implémentation :**

- **Information pure** : Master Butler retourne des informations (capacités, permissions, associations), jamais des décisions.

- **Pas de booléen d'autorisation** : Aucune méthode de Master Butler ne retourne un booléen d'autorisation directe. Les retours sont des informations descriptives.

- **Neutralité absolue** : Master Butler ne recommande pas, ne suggère pas, ne juge pas. Il expose les faits.

**Ce que cela signifie concrètement :**

- Une requête "L'utilisateur X a-t-il accès ?" DOIT retourner les permissions de X, pas une décision oui/non
- StrongFather utilise ces informations pour prendre la décision
- Master Butler ne filtre jamais selon une logique de décision

### 2.3. Idempotence des déclarations (INV-MB-3)

**Principe contractuel :**

L'invariant INV-MB-3 établit que les déclarations de capacités sont **idempotentes**. Déclarer deux fois la même capacité n'a pas d'effet supplémentaire. Le registre reste cohérent quel que soit l'ordre ou le nombre de déclarations.

**Traduction en logique d'implémentation :**

- **Déduplication automatique** : Une capacité déclarée plusieurs fois ne crée pas de duplications dans le registre.

- **Ordre indépendant** : L'ordre des déclarations n'affecte pas le résultat final du registre.

- **Redéclaration sûre** : Les modules peuvent redéclarer leurs capacités à chaque démarrage sans effet indésirable.

**Ce que cela signifie concrètement :**

- Un module qui redémarre et redéclare ses capacités ne corrompt pas le registre
- Les déclarations concurrentes ne créent pas d'états incohérents
- Le registre converge vers le même état quelle que soit la séquence de déclarations

### 2.4. Immutabilité des identifiants (INV-MB-4)

**Principe contractuel :**

L'invariant INV-MB-4 établit que les identifiants de capacités sont **immuables**. Une fois qu'une capacité est déclarée avec un identifiant, cet identifiant ne change jamais.

**Traduction en logique d'implémentation :**

- **Identifiants stables** : Une fois créé, un identifiant de capacité ne peut pas être modifié.

- **Évolution par création** : Si une capacité évolue significativement, une nouvelle capacité est créée avec un nouvel identifiant.

- **Références durables** : Les références aux capacités (dans les permissions, les logs, les configurations) restent valides dans le temps.

**Ce que cela signifie concrètement :**

- Pas de renommage d'identifiants de capacités existantes
- Les identifiants sont des références stables pour tout le système
- L'évolution se fait par ajout, pas par modification

### 2.5. Traçabilité complète (INV-MB-5)

**Principe contractuel :**

L'invariant INV-MB-5 établit que toute modification du registre de Master Butler est **tracée**. Créations, modifications, suppressions : tout est enregistré avec le contexte (qui, quand, pourquoi).

**Traduction en logique d'implémentation :**

- **Journalisation systématique** : Chaque déclaration de capacité, chaque définition de permission, chaque modification est tracée.

- **Contexte complet** : Les traces incluent le contexte complet (acteur, timestamp, raison).

- **Historique auditable** : L'historique des capacités et permissions est accessible pour audit.

**Ce que cela signifie concrètement :**

- Aucune modification silencieuse n'est possible
- L'audit peut reconstituer l'évolution complète du registre
- Les traces sont immuables et accessibles

### 2.6. Séparation capacité/permission (INV-MB-6)

**Principe contractuel :**

L'invariant INV-MB-6 établit que les capacités et les permissions sont **strictement séparées**. Une capacité existe indépendamment des permissions. Une permission référence des capacités mais ne les définit pas.

**Traduction en logique d'implémentation :**

- **Modèles distincts** : Les capacités et les permissions sont des entités distinctes avec leurs propres modèles.

- **Relations sans fusion** : Une permission peut référencer des capacités, mais les deux restent des entités séparées.

- **Suppression indépendante** : La suppression d'une permission n'affecte pas la capacité associée. La suppression d'une capacité invalide les permissions qui la référencent.

**Ce que cela signifie concrètement :**

- Une capacité peut exister sans aucune permission associée
- Une permission DOIT référencer au moins une capacité existante
- La suppression d'une capacité orpheline les permissions qui la référencent

### 2.7. Pas de logique métier (INV-MB-7)

**Principe contractuel :**

L'invariant INV-MB-7 établit que Master Butler **ne contient aucune logique métier**. Il ne connaît pas les règles du domaine, les contraintes applicatives, les limites fonctionnelles.

**Traduction en logique d'implémentation :**

- **Registre technique** : Master Butler sait ce qui est techniquement possible, pas ce qui est métier-compatible.

- **Pas de validation métier** : Master Butler ne valide jamais une action selon des critères métier.

- **Neutralité fonctionnelle** : Les capacités et permissions sont des concepts techniques, pas des règles métier.

**Ce que cela signifie concrètement :**

- Si une règle métier dit "un utilisateur ne peut créer que 10 contenus par jour", cette contrainte n'appartient PAS à Master Butler
- Master Butler sait que la capacité "content.create" existe, mais ignore les limites métier
- La logique métier appartient aux modules, aux produits, et à StrongFather

### 2.8. Accessibilité universelle (INV-MB-8)

**Principe contractuel :**

L'invariant INV-MB-8 établit que Master Butler est **accessible à tous les composants autorisés** du système. Aucun composant ne peut être empêché d'interroger Master Butler (sous réserve des permissions d'accès à Master Butler lui-même).

**Traduction en logique d'implémentation :**

- **Service partagé** : Master Butler est un service partagé, pas un composant isolé.

- **Accessibilité garantie** : Tout composant autorisé peut interroger Master Butler.

- **Disponibilité** : Master Butler DOIT être disponible pour répondre aux interrogations des composants.

**Ce que cela signifie concrètement :**

- StrongFather, BondingBrother, les produits peuvent interroger Master Butler
- Aucun composant n'est privilégié dans l'accès (sauf Master Butler lui-même qui peut contrôler son propre accès)
- L'accessibilité ne signifie pas absence de contrôle d'accès

---

## 3. Comment traduire les contrats en logique sans interprétation abusive

### 3.1. Respecter les invariants comme contraintes absolues

**Principe :**

Les invariants contractuels (INV-MB-*) sont des contraintes absolues qui DOIVENT toujours être vraies. Ils ne sont pas des suggestions ou des recommandations.

**Traduction :**

- **Vérification systématique** : Chaque invariant DOIT être vérifié et préservé à chaque opération. Aucun invariant ne peut être violé, même temporairement.

- **Préservation garantie** : Toute opération DOIT garantir que les invariants sont préservés après exécution. Si une opération violerait un invariant, elle DOIT être rejetée.

- **Pas d'interprétation** : Les invariants ne peuvent pas être interprétés ou adaptés. Ils sont absolus et non négociables.

**Exemple conceptuel :**

Si l'invariant INV-MB-2 (non-décision) exige que Master Butler ne produise jamais de verdict d'autorisation, alors aucune méthode ne peut retourner "autorisé" ou "refusé", même pour des raisons de "commodité" ou "simplification".

### 3.2. Implémenter la découverte comme exposition pure

**Principe :**

La découverte des capacités est une exposition pure des informations du registre, sans filtrage décisionnel, sans recommandation, sans suggestion.

**Traduction :**

- **Exposition neutre** : La découverte expose les capacités et permissions existantes, sans jugement.

- **Filtrage technique uniquement** : Le filtrage est autorisé pour des critères techniques (module, type de capacité), pas pour des critères décisionnels.

- **Exhaustivité** : La découverte retourne toutes les informations pertinentes, sans omission.

**Exemple conceptuel :**

Une requête "Quelles capacités existent dans le module CMS ?" retourne la liste exhaustive des capacités de ce module, sans filtrer selon "ce que l'utilisateur devrait voir" (qui appartient à StrongFather).

### 3.3. Traiter les déclarations comme des enregistrements, pas des validations métier

**Principe :**

Les déclarations de capacités sont des enregistrements dans le registre, pas des validations métier. Master Butler vérifie la structure de la déclaration, pas sa pertinence métier.

**Traduction :**

- **Validation structurelle** : Master Butler vérifie que la déclaration est bien formée (identifiant, métadonnées).

- **Pas de validation métier** : Master Butler ne vérifie pas si la capacité "devrait" exister selon des critères métier.

- **Enregistrement fidèle** : La déclaration est enregistrée fidèlement, sans interprétation.

**Exemple conceptuel :**

Si un module déclare une capacité "delete.all", Master Butler l'enregistre. Il ne juge pas si cette capacité est "dangereuse" ou "devrait être limitée" — ces considérations appartiennent aux politiques de StrongFather.

### 3.4. Ne pas "optimiser" en fusionnant registre et décision

**Principe :**

Aucune optimisation ne peut fusionner la fonction de registre de Master Butler avec la fonction de décision de StrongFather. Les deux sont séparés par conception.

**Traduction :**

- **Séparation stricte** : Le registre (Master Butler) et la décision (StrongFather) sont des fonctions distinctes.

- **Pas de raccourci** : Aucun "raccourci" ne peut permettre de prendre une décision directement depuis le registre.

- **Flux complet** : Toute décision passe par StrongFather, même si Master Butler a "toutes les informations".

**Exemple conceptuel :**

Même si Master Butler sait qu'un utilisateur possède une permission, il ne peut pas retourner "autorisé". StrongFather DOIT évaluer cette permission selon les politiques avant de produire une décision.

---

## 4. Ce qu'un développeur ne doit jamais faire

### 4.1. Produire des décisions d'autorisation

**Interdiction contractuelle :**

Master Butler ne décide jamais. Il fournit des informations, StrongFather décide.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Créer une méthode `isAuthorized()` ou équivalent qui retourne un booléen de décision
- Retourner "oui" ou "non" à une question d'autorisation
- Filtrer les résultats selon une logique d'autorisation
- Recommander ou suggérer une décision

**Conséquence de la violation :**

- Violation de l'invariant INV-MB-2 (non-décision)
- Usurpation du rôle de StrongFather
- Compromission de la séparation des responsabilités

### 4.2. Exécuter des actions fonctionnelles

**Interdiction contractuelle :**

Master Butler ne crée pas de contenu, ne modifie pas de hiérarchie, ne téléverse pas de média. Il recense les capacités qui permettent ces actions, mais ne les exécute jamais.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Implémenter des actions fonctionnelles dans Master Butler
- Créer des méthodes qui modifient des données métier
- Exécuter des opérations au nom des modules ou produits
- Déléguer des actions fonctionnelles depuis Master Butler

**Conséquence de la violation :**

- Violation du rôle de registre de Master Butler
- Usurpation du rôle des modules et produits
- Compromission de l'architecture

### 4.3. Stocker des données métier

**Interdiction contractuelle :**

Master Butler ne stocke jamais de données métier. Il stocke des métadonnées : définitions de capacités, définitions de permissions, associations, historiques.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Stocker des données utilisateur dans Master Butler
- Stocker des contenus, des médias, des documents
- Utiliser le registre comme base de données métier
- Mélanger métadonnées de capacités et données métier

**Conséquence de la violation :**

- Violation du rôle de registre de Master Butler
- Confusion des responsabilités avec KindMother
- Compromission de l'isolation des données

### 4.4. Gérer les identités

**Interdiction contractuelle :**

Master Butler ne gère jamais les identités des utilisateurs ou des systèmes. Il connaît les rôles et les permissions associées, mais l'identité elle-même appartient au système d'authentification.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Implémenter une gestion d'identité dans Master Butler
- Stocker des credentials ou des tokens
- Authentifier des utilisateurs
- Gérer des sessions

**Conséquence de la violation :**

- Violation du périmètre de Master Butler
- Usurpation du rôle du système d'authentification
- Compromission de la sécurité

### 4.5. Définir des politiques de décision

**Interdiction contractuelle :**

Master Butler ne définit jamais de politiques de décision. Les politiques (règles qui déterminent quand une permission est accordée ou refusée) appartiennent à StrongFather.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Implémenter des politiques d'autorisation dans Master Butler
- Créer des règles conditionnelles d'autorisation
- Définir des contextes d'autorisation
- Implémenter une logique "si X alors autorisé"

**Conséquence de la violation :**

- Violation de l'invariant INV-MB-2 (non-décision)
- Usurpation du rôle de StrongFather
- Duplication des responsabilités

### 4.6. Appliquer des contraintes métier

**Interdiction contractuelle :**

Master Butler n'applique jamais de contraintes métier. Les contraintes métier appartiennent à StrongFather ou aux produits.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Implémenter des limites métier (quotas, plafonds)
- Valider des règles de domaine
- Appliquer des contraintes temporelles métier
- Filtrer selon des critères métier

**Conséquence de la violation :**

- Violation de l'invariant INV-MB-7 (pas de logique métier)
- Confusion des responsabilités
- Contamination du registre par la logique métier

### 4.7. Persister directement

**Interdiction contractuelle :**

Master Butler ne gère jamais directement la persistance. Si son registre doit être persisté, il utilise KindMother comme support.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Accéder directement à une base de données depuis Master Butler
- Manipuler directement un système de fichiers
- Implémenter une couche de persistance propre à Master Butler
- Contourner KindMother pour la persistance

**Conséquence de la violation :**

- Violation de l'architecture de persistance
- Duplication des responsabilités avec KindMother
- Compromission de la cohérence des données

---

## 5. Anti-patterns classiques

### 5.1. Anti-pattern 1 : Le registre décideur

**Description :**

Tentative de transformer Master Butler en décideur en ajoutant des méthodes qui retournent des verdicts d'autorisation.

**Exemple conceptuel :**

Un développeur crée une méthode "canUserPerform(user, capability)" qui retourne true/false, pensant "simplifier" l'intégration avec les produits.

**Conséquence :**

- Violation de l'invariant INV-MB-2 (non-décision)
- Usurpation du rôle de StrongFather
- Confusion architecturale entre registre et décision
- Les produits peuvent contourner StrongFather en interrogeant directement Master Butler

**Correction :**

Master Butler retourne les informations (permissions de l'utilisateur, capacités requises), et StrongFather prend la décision. Aucune méthode de Master Butler ne retourne un booléen de décision.

### 5.2. Anti-pattern 2 : Le registre exécutant

**Description :**

Tentative de faire exécuter des actions par Master Butler en plus de son rôle de registre.

**Exemple conceptuel :**

Un développeur ajoute une méthode "executeCapability(capability, context)" dans Master Butler, pensant "centraliser" l'exécution des capacités.

**Conséquence :**

- Violation du rôle de registre de Master Butler
- Usurpation du rôle des modules et produits
- Couplage fort entre registre et exécution
- Compromission de l'isolation des responsabilités

**Correction :**

Master Butler recense les capacités et fournit les informations. L'exécution appartient aux modules et produits qui possèdent ces capacités. Master Butler ne touche jamais à l'exécution.

### 5.3. Anti-pattern 3 : Le registre métier

**Description :**

Tentative d'intégrer des règles métier dans le registre de capacités et permissions.

**Exemple conceptuel :**

Un développeur ajoute des propriétés métier aux capacités (quotas, limites, conditions d'utilisation métier), pensant "enrichir" le registre.

**Conséquence :**

- Violation de l'invariant INV-MB-7 (pas de logique métier)
- Contamination du registre par la logique métier
- Confusion entre capacités techniques et contraintes métier
- Évolution couplée du registre et des règles métier

**Correction :**

Le registre contient des métadonnées techniques (identifiant, nom, description, module d'origine). Les règles métier (quotas, limites, conditions) appartiennent aux politiques de StrongFather ou aux modules.

### 5.4. Anti-pattern 4 : Le raccourci de découverte

**Description :**

Tentative de créer des raccourcis qui combinent découverte et décision pour "simplifier" l'usage.

**Exemple conceptuel :**

Un développeur crée une méthode "getAccessibleCapabilities(user)" qui filtre les capacités selon ce que l'utilisateur "devrait voir", pensant "faciliter" l'intégration.

**Conséquence :**

- Violation de l'invariant INV-MB-2 (non-décision)
- Fusion illégitime de découverte et décision
- Contournement de StrongFather pour le filtrage
- Incohérence entre les sources de décision

**Correction :**

Master Butler expose toutes les capacités (discovery neutre). StrongFather applique les filtres selon les politiques. La découverte et la décision restent séparées.

### 5.5. Anti-pattern 5 : Le registre avec mémoire d'état

**Description :**

Tentative de maintenir un état d'utilisation ou de décision dans le registre.

**Exemple conceptuel :**

Un développeur stocke "dernière utilisation" ou "nombre d'appels" d'une capacité dans le registre, pensant "optimiser" les décisions futures.

**Conséquence :**

- Violation du rôle de registre passif de Master Butler
- Introduction d'état dynamique dans un registre statique
- Couplage entre registre et utilisation
- Dérive vers la logique métier

**Correction :**

Le registre est statique (capacités déclarées, permissions définies). L'état d'utilisation appartient aux modules, aux produits, ou aux systèmes d'observabilité. Master Butler ne maintient pas d'état d'utilisation.

---

## 6. Bonnes pratiques conceptuelles

### 6.1. Déclaration systématique et précoce

**Pratique :**

Tout module DOIT déclarer ses capacités à Master Butler lors de son initialisation, avant d'être opérationnel.

**Justification :**

- Respecte l'invariant INV-MB-1 (exhaustivité du registre)
- Garantit que toutes les capacités sont connues avant utilisation
- Permet la découverte complète dès le démarrage

**Implémentation conceptuelle :**

- Déclaration lors du bootstrap du module
- Validation de la déclaration par Master Butler
- Confirmation avant passage en mode opérationnel
- Redéclaration possible sans effet indésirable (idempotence)

### 6.2. Séparation stricte des modèles

**Pratique :**

Les capacités et les permissions DOIVENT être des modèles distincts, avec des cycles de vie indépendants.

**Justification :**

- Respecte l'invariant INV-MB-6 (séparation capacité/permission)
- Permet l'évolution indépendante des capacités et permissions
- Facilite la maintenance et l'audit

**Implémentation conceptuelle :**

- Registre des capacités séparé du registre des permissions
- Relations explicites entre permissions et capacités (références)
- Gestion indépendante des cycles de vie
- Validation des références lors de la création des permissions

### 6.3. Traçabilité complète et immuable

**Pratique :**

Toute modification du registre DOIT être tracée de manière complète et immuable.

**Justification :**

- Respecte l'invariant INV-MB-5 (traçabilité complète)
- Permet l'audit complet de l'évolution du registre
- Garantit la responsabilité des modifications

**Implémentation conceptuelle :**

- Journalisation systématique de chaque modification
- Contexte complet (acteur, timestamp, raison)
- Traces immuables (append-only)
- Accessibilité pour audit

### 6.4. Réponses informatives et neutres

**Pratique :**

Les réponses de Master Butler DOIVENT être informatives, complètes, et neutres — sans jugement ni recommandation.

**Justification :**

- Respecte l'invariant INV-MB-2 (non-décision)
- Permet aux consommateurs (StrongFather, produits) de prendre leurs propres décisions
- Maintient la séparation des responsabilités

**Implémentation conceptuelle :**

- Retour d'informations descriptives (capacités, permissions, associations)
- Pas de booléen de décision
- Pas de recommandation ou suggestion
- Exhaustivité des informations retournées

### 6.5. Validation structurelle uniquement

**Pratique :**

Master Butler DOIT valider la structure des déclarations (forme, complétude), pas leur pertinence métier.

**Justification :**

- Respecte l'invariant INV-MB-7 (pas de logique métier)
- Maintient la neutralité du registre
- Permet aux modules de déclarer librement leurs capacités

**Implémentation conceptuelle :**

- Validation de la présence des champs obligatoires
- Validation du format des identifiants
- Validation de l'existence des références (capacités pour les permissions)
- Pas de validation métier (pertinence, limites, conditions)

### 6.6. Respect des lois d'autonomie

**Pratique :**

L'implémentation de Master Butler DOIT respecter les lois d'autonomie système (LOI-1, LOI-5).

**Justification :**

- LOI-1 : Aucune dépendance externe critique à l'exécution
- LOI-5 : Coût proportionnel au hardware

**Implémentation conceptuelle :**

- Registre local, interrogations locales
- Pas de dépendance à des services externes pour les fonctions critiques
- Empreinte mémoire proportionnelle au nombre de capacités/permissions
- Opérations de lookup simples et efficaces

---

## 7. Check-list mentale avant toute feature

Avant d'implémenter une nouvelle fonctionnalité dans Master Butler, un développeur DOIT vérifier mentalement :

### 7.1. Vérification du rôle

- **Cette feature appartient-elle à un registre ?** : Vérifier que la fonctionnalité concerne le recensement, la déclaration, la découverte.

- **Cette feature n'est-elle pas une décision ?** : S'assurer que la fonctionnalité ne produit pas de verdict d'autorisation.

- **Cette feature n'est-elle pas une exécution ?** : S'assurer que la fonctionnalité n'exécute pas d'action fonctionnelle.

### 7.2. Vérification des invariants

- **L'invariant INV-MB-1 (exhaustivité) est-il préservé ?** : La feature maintient-elle l'exhaustivité du registre ?

- **L'invariant INV-MB-2 (non-décision) est-il respecté ?** : La feature ne produit-elle pas de décision ?

- **L'invariant INV-MB-3 (idempotence) est-il préservé ?** : Les opérations sont-elles idempotentes ?

- **L'invariant INV-MB-4 (immutabilité des identifiants) est-il respecté ?** : Les identifiants restent-ils stables ?

- **L'invariant INV-MB-5 (traçabilité) est-il assuré ?** : Toutes les modifications sont-elles tracées ?

- **L'invariant INV-MB-6 (séparation) est-il maintenu ?** : Capacités et permissions restent-elles séparées ?

- **L'invariant INV-MB-7 (pas de logique métier) est-il respecté ?** : La feature est-elle exempte de logique métier ?

- **L'invariant INV-MB-8 (accessibilité) est-il assuré ?** : La feature reste-t-elle accessible aux composants autorisés ?

### 7.3. Vérification des interdictions

- **La feature ne décide-t-elle pas ?** : Aucune méthode ne retourne de décision d'autorisation.

- **La feature n'exécute-t-elle pas ?** : Aucune action fonctionnelle n'est exécutée.

- **La feature ne stocke-t-elle pas de données métier ?** : Seules des métadonnées sont stockées.

- **La feature ne gère-t-elle pas d'identités ?** : Aucune gestion d'identité n'est implémentée.

- **La feature ne définit-elle pas de politiques ?** : Aucune politique de décision n'est créée.

- **La feature n'applique-t-elle pas de contraintes métier ?** : Aucune règle métier n'est appliquée.

### 7.4. Vérification des relations

- **StrongFather reste-t-il le décideur ?** : La feature ne court-circuite-t-elle pas StrongFather ?

- **BondingBrother peut-il interroger correctement ?** : La feature fournit-elle les informations nécessaires à BondingBrother ?

- **KindMother gère-t-elle la persistance ?** : La feature ne contourne-t-elle pas KindMother pour persister ?

---

## 8. Conclusion

Ce document fournit des lignes directrices pour implémenter Master Butler de manière conforme aux contrats FONDATION.

**Points clés :**

- Master Butler est un **registre pur** : il recense, il expose, il ne décide jamais
- Les **invariants INV-MB-1 à INV-MB-8** sont des contraintes absolues
- Les **interdictions** (pas de décision, pas d'exécution, pas de logique métier) sont non négociables
- Les **bonnes pratiques** garantissent la conformité architecturale
- La **check-list mentale** prévient les violations avant implémentation

**Nature informative :**

Ce document est purement informatif et ne crée aucune nouvelle obligation contractuelle. Il sert uniquement à guider la compréhension et l'application des contrats FONDATION.

**Rappel :** Les contrats FONDATION priment toujours sur ce guide. En cas de doute, se référer aux contrats FONDATION.

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** POST-FONDATION — Informatif, non normatif, non contractuel  
**Référence :** Miyukini Core System v2.4, Master Butler Documentation, Tous les contrats FONDATION  
**Type :** Guide d'implémentation informatif

---

## 9. Mini log — erreurs / warnings / arbitrages rencontrés

### Arbitrage A1 : Niveau de détail technique

**Arbitrage rencontré :** Quel niveau de détail technique inclure dans ce guide ? Le document doit rester conceptuel et ne pas prescrire de technologies.

**Décision prise :** Le document reste purement conceptuel. Aucun détail technique (langages, structures de données, algorithmes) n'est inclus. Seuls les concepts et principes sont décrits.

**Justification :** Ce document est informatif et non normatif. Il guide la compréhension des contrats, pas l'implémentation technique. Les détails techniques sont des choix d'implémentation.

**Documentation :** Toutes les sections restent conceptuelles, sans détails techniques.

### Arbitrage A2 : Parallèle avec KindMother

**Arbitrage rencontré :** Dans quelle mesure le document doit-il suivre la structure du guide KindMother ?

**Décision prise :** La structure générale suit le modèle KindMother (sections, organisation), mais le contenu est entièrement adapté au contexte spécifique de Master Butler (registre vs. gestionnaire de données).

**Justification :** La cohérence structurelle facilite la navigation entre les guides des différents Cores, tout en préservant la spécificité de chaque Core.

**Documentation :** Structure parallèle mais contenu spécifique à Master Butler.

### Arbitrage A3 : Distinction registre passif vs. service actif

**Arbitrage rencontré :** Comment clarifier la nature de "registre passif" de Master Butler sans créer de confusion avec un service inactif ?

**Décision prise :** Le document clarifie que Master Butler est un registre "passif" au sens où il ne prend pas d'initiative (pas de décision, pas d'exécution), mais il est "actif" dans le sens où il répond aux requêtes et maintient son registre.

**Justification :** Cette distinction évite la confusion entre "passif" (nature) et "inactif" (comportement).

**Documentation :** Section 2.1 et 6.4 clarifient cette distinction.

### Arbitrage A4 : Traitement des anti-patterns

**Arbitrage rencontré :** Les anti-patterns doivent-ils être spécifiques à Master Butler ou génériques ?

**Décision prise :** Les anti-patterns sont spécifiques au contexte de Master Butler (registre décideur, registre exécutant, registre métier), illustrant les violations typiques du rôle de registre.

**Justification :** Des anti-patterns spécifiques sont plus utiles pour guider les développeurs dans le contexte de Master Butler.

**Documentation :** Section 5 avec anti-patterns spécifiques à Master Butler.

### Arbitrage A5 : Relations avec les autres Cores

**Arbitrage rencontré :** Comment présenter les relations avec StrongFather, KindMother, BondingBrother sans dupliquer la documentation fondatrice ?

**Décision prise :** Les relations sont mentionnées dans le contexte des vérifications (check-list) et des interdictions, sans détailler les flux complets qui appartiennent à la documentation fondatrice.

**Justification :** Ce guide est orienté implémentation, pas architecture. Les relations détaillées appartiennent aux contrats d'intégration.

**Documentation :** Section 7.4 et mentions dans les interdictions.

---

*Aucune autre erreur, warning, ou arbitrage rencontré lors de la rédaction de ce document.*
