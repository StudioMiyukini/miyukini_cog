# Border Guard — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter Border Guard correctement, sans violer les contrats FONDATION.

**Objectif pédagogique :** Ce document vise à aider les développeurs à comprendre comment traduire les contrats FONDATION en implémentation, en respectant strictement les invariants, garanties, et interdictions.

**Avertissement :** Ce document ne doit pas être interprété abusivement. Il ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats FONDATION priment toujours sur ce guide.

**Relation avec les contrats FONDATION :** Ce document fait référence aux contrats FONDATION existants mais ne les étend pas, ne les modifie pas, et ne crée aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1. Objectif

Ce document fournit des lignes directrices pour implémenter Border Guard de manière conforme aux contrats FONDATION. Il explique comment traduire les concepts contractuels en logique d'implémentation sans interprétation abusive.

### 1.2. Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats FONDATION.

### 1.3. Rappel de la mission de Border Guard

Border Guard est le **core de définition des frontières et des règles d'entrée/sortie** du Miyukini Core System. Il répond à la question fondamentale :

> **"Où sont les frontières du système, et quelles règles gouvernent leur franchissement ?"**

Border Guard **définit, classifie, et établit des règles**. Il **ne filtre jamais**, **ne bloque jamais**, **n'exécute jamais**, et **ne décide jamais**.

### 1.4. Sources contractuelles

Ce document se base sur les contrats FONDATION, avec un focus particulier sur :

- **Documentation Fondatrice** : Invariants INV-BG-1 à INV-BG-10, responsabilités exclusives, interdictions
- **Boundary Definition Contract** : Types de frontières, propriétés, taxonomie
- **Trust Level Classification Contract** : Niveaux de confiance (trusted, verified, unknown, hostile)
- **Crossing Rules Contract** : Règles déclaratives de franchissement
- **Invariants & Guarantees** : Garanties structurelles non négociables
- **Violations & Anti-Patterns** : Ce qu'il ne faut jamais faire
- **[Miyukini Conceptual References — Lois Autonomie Système](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Les lignes directrices d'implémentation doivent respecter les 6 lois d'autonomie, notamment **LOI-1** (aucune dépendance externe critique), **LOI-6** (fédération explicite et réversible).

---

## 2. Principes généraux à respecter absolument

### 2.1. Aucune capacité d'exécution (INV-BG-1)

**Principe contractuel :**

L'invariant INV-BG-1 établit que Border Guard ne possède **jamais** la capacité d'exécuter une action : filtrage, blocage, interception, application. Il définit les règles et classifie les sources, mais toute exécution est déléguée aux autorités compétentes.

**Traduction en logique d'implémentation :**

- **Border Guard DÉFINIT** : Il établit les frontières, les règles de franchissement, les niveaux de confiance.
- **Border Guard CLASSIFIE** : Il attribue les niveaux de confiance aux sources et destinations.
- **Border Guard NE FAIT JAMAIS** : Il ne filtre pas, ne bloque pas, n'intercepte pas, n'applique pas.

**Ce que cela signifie concrètement :**

- Aucun mécanisme de filtrage ne doit être accessible à Border Guard
- Le blocage est exécuté par BondingBrother (selon décision de StrongFather)
- Border Guard fournit les règles déclaratives — jamais l'exécution

### 2.2. Aucune persistance directe (INV-BG-2)

**Principe contractuel :**

L'invariant INV-BG-2 établit que Border Guard n'accède **jamais** directement à la persistance. Toute définition de frontière ou de règle qui doit être persistée est transmise à KindMother via les canaux appropriés.

**Traduction en logique d'implémentation :**

- **Pas d'accès DB** : Border Guard ne contient pas de drivers de base de données.
- **Pas d'écriture fichier** : Border Guard n'écrit jamais sur le système de fichiers.
- **Délégation à KindMother** : Les définitions à persister sont transmises à KindMother.

**Ce que cela signifie concrètement :**

- Les définitions de frontières sont maintenues en mémoire par Border Guard
- La persistance est déléguée via événements ou canaux vers KindMother
- Aucune importation de bibliothèques de persistance dans Border Guard

### 2.3. Aucune décision autonome (INV-BG-3)

**Principe contractuel :**

L'invariant INV-BG-3 établit que Border Guard ne prend **jamais** de décision de manière autonome. Il informe, il classifie, il définit, mais la décision finale appartient toujours à StrongFather ou aux autorités appropriées.

**Traduction en logique d'implémentation :**

- **Border Guard INFORME** : Il fournit le contexte de confiance à StrongFather.
- **Border Guard CLASSIFIE** : Il attribue un niveau de confiance (trusted, verified, unknown, hostile).
- **Border Guard NE DÉCIDE JAMAIS** : La décision d'accepter ou refuser appartient à StrongFather.

**Ce que cela signifie concrètement :**

- Aucune méthode `decide()`, `allow()`, `deny()` dans Border Guard
- Les classifications sont des informations, pas des verdicts
- StrongFather consulte Border Guard, puis décide

### 2.4. Classification exhaustive (INV-BG-4)

**Principe contractuel :**

L'invariant INV-BG-4 établit que toute source, destination, ou interaction **doit** être classifiée selon un niveau de confiance. Aucune interaction ne peut exister sans classification. Par défaut, tout ce qui n'est pas explicitement classifié est considéré comme "unknown".

**Traduction en logique d'implémentation :**

- **Classification systématique** : Toute source qui traverse une frontière a un niveau de confiance.
- **Défaut = unknown** : Si pas de classification explicite, le niveau est "unknown".
- **Aucune exception** : Pas de traitement "sans classification".

**Ce que cela signifie concrètement :**

- Toute API de Border Guard retourne un niveau de confiance (jamais null)
- Le niveau "unknown" est le défaut sécuritaire
- Les règles de franchissement s'appliquent selon le niveau retourné

### 2.5. Frontières explicites (INV-BG-5)

**Principe contractuel :**

L'invariant INV-BG-5 établit que toute frontière **doit** être explicitement définie et documentée. Aucune frontière implicite n'est autorisée. Si une démarcation existe dans le système, elle doit être formalisée par Border Guard.

**Traduction en logique d'implémentation :**

- **Registre exhaustif** : Toutes les frontières sont dans le registre de Border Guard.
- **Définition formelle** : Chaque frontière a un identifiant, un type, une direction, une perméabilité.
- **Pas de frontière cachée** : Aucun contrôle de franchissement sans frontière définie.

**Ce que cela signifie concrètement :**

- Le registre des frontières est la source de vérité
- Toute demande de règles pour une frontière non définie retourne NOT_FOUND
- Les frontières sont créées explicitement, jamais inférées

### 2.6. Règles déclaratives (INV-BG-6)

**Principe contractuel :**

L'invariant INV-BG-6 établit que toutes les règles de franchissement **doivent** être déclaratives. Aucune règle procédurale ou impérative n'est autorisée. Une règle exprime ce qui est requis, pas comment le vérifier.

**Traduction en logique d'implémentation :**

- **Conditions, pas procédures** : "Authentification requise" plutôt que "Vérifier le token JWT".
- **Ce qui est requis, pas comment** : "Niveau verified minimum" plutôt que "Appeler le service auth".
- **Neutralité technique** : Les règles ne référencent pas de technologies spécifiques.

**Ce que cela signifie concrètement :**

- Les règles sont des expressions de conditions
- L'implémentation technique des vérifications appartient à BondingBrother
- Border Guard ne contient jamais de code de vérification

### 2.7. Séparation définition/application (INV-BG-7)

**Principe contractuel :**

L'invariant INV-BG-7 établit que la définition des frontières et des règles est **strictement séparée** de leur application. Border Guard définit, BondingBrother applique. Cette séparation est non négociable et ne peut être contournée.

**Traduction en logique d'implémentation :**

- **Interface claire** : Border Guard expose des APIs de consultation (GET), pas d'action.
- **Contrat d'interface** : Les règles fournies sont déclaratives, l'application est libre.
- **Indépendance** : Border Guard peut évoluer sans modifier BondingBrother, et inversement.

**Ce que cela signifie concrètement :**

- Border Guard fournit des règles via des consultations
- BondingBrother implémente la vérification technique de ces règles
- Aucune dépendance circulaire entre Border Guard et BondingBrother

### 2.8. Traçabilité complète (INV-BG-8)

**Principe contractuel :**

L'invariant INV-BG-8 établit que toute définition de frontière, toute classification de confiance, toute règle établie **doit** être traçable avec son origine, sa date, et sa justification.

**Traduction en logique d'implémentation :**

- **Métadonnées obligatoires** : `createdAt`, `createdBy`, `justification`, `version`.
- **Historique** : Les modifications sont tracées.
- **Audit possible** : Toute définition peut être auditée.

**Ce que cela signifie concrètement :**

- Chaque frontière, règle, classification a des métadonnées complètes
- L'historique des modifications est conservé
- Les consultations peuvent inclure l'origine de la définition

### 2.9. Cohérence globale (INV-BG-9)

**Principe contractuel :**

L'invariant INV-BG-9 établit que les définitions de Border Guard **doivent** être globalement cohérentes. Aucune contradiction entre frontières, niveaux de confiance, ou règles n'est autorisée.

**Traduction en logique d'implémentation :**

- **Validation à la création** : Toute nouvelle définition est validée contre l'existant.
- **Pas de contradiction** : Deux règles ne peuvent pas avoir des résultats opposés.
- **Hiérarchie respectée** : Les zones de confiance sont cohérentes entre elles.

**Ce que cela signifie concrètement :**

- Un mécanisme de validation de cohérence existe
- Les créations qui créent des contradictions sont rejetées
- La cohérence globale peut être auditée

### 2.10. Neutralité conceptuelle (INV-BG-10)

**Principe contractuel :**

L'invariant INV-BG-10 établit que Border Guard **ne fait jamais** de supposition sur la technologie d'implémentation. Les définitions sont purement conceptuelles et peuvent être implémentées par n'importe quelle technologie.

**Traduction en logique d'implémentation :**

- **Pas de référence technique** : "Authentification requise", pas "JWT RS256 requis".
- **Portabilité** : Les définitions fonctionnent indépendamment de la stack technique.
- **Abstraction** : "Données chiffrées", pas "AES-256-GCM".

**Ce que cela signifie concrètement :**

- Aucune bibliothèque technique (crypto, auth, réseau) importée dans Border Guard
- Les définitions sont des contrats conceptuels
- L'implémentation technique est du ressort des adaptateurs

---

## 3. Comment traduire les contrats en logique sans interprétation abusive

### 3.1. Respecter les invariants comme contraintes absolues

**Principe :**

Les invariants contractuels (INV-BG-*) sont des contraintes absolues qui DOIVENT toujours être vraies. Ils ne sont pas des suggestions ou des recommandations.

**Traduction :**

- **Vérification systématique** : Chaque invariant DOIT être vérifié à chaque opération.
- **Préservation garantie** : Toute opération DOIT garantir que les invariants sont préservés après exécution.
- **Pas d'interprétation** : Les invariants ne peuvent pas être interprétés ou adaptés.

**Exemple conceptuel :**

Si l'invariant INV-BG-1 (aucune capacité d'exécution) interdit le filtrage, alors aucune méthode de filtrage n'est possible dans Border Guard, même pour des raisons "pratiques" ou de "performance".

### 3.2. Séparer strictement définition et application

**Principe :**

La séparation entre définition (Border Guard) et application (BondingBrother) est fondamentale. C'est la règle structurante de toute l'architecture.

**Traduction :**

- **Border Guard = Quoi** : Quelles sont les règles ? Quel niveau de confiance ?
- **BondingBrother = Comment** : Comment vérifier techniquement ces règles ?
- **Aucun chevauchement** : Border Guard ne fait jamais le travail de BondingBrother.

**Exemple conceptuel :**

Border Guard définit : "La source doit être authentifiée avec un niveau verified minimum."
BondingBrother applique : "Vérifier le token JWT, valider la signature, vérifier l'expiration."

### 3.3. Traiter la classification comme un service d'information

**Principe :**

La classification de confiance est une information, pas une décision. Border Guard informe du niveau de confiance ; il ne décide pas des conséquences.

**Traduction :**

- **Information pure** : `getTrustLevel(source)` retourne un niveau (trusted, verified, unknown, hostile).
- **Pas de verdict** : Border Guard ne dit pas "bloqué" ou "autorisé".
- **StrongFather décide** : La décision basée sur le niveau appartient à StrongFather.

**Exemple conceptuel :**

Border Guard : "Cette source est classifiée 'hostile'."
StrongFather : "Je décide de bloquer cette source."
BondingBrother : "J'exécute le blocage."

### 3.4. Implémenter la traçabilité comme obligation structurelle

**Principe :**

La traçabilité n'est pas une fonctionnalité optionnelle. C'est une obligation structurelle (INV-BG-8) qui s'applique à toute définition.

**Traduction :**

- **Métadonnées obligatoires** : Toute création ou modification a des métadonnées.
- **Pas d'exception** : Même les définitions "triviales" sont traçables.
- **Historique immuable** : Les traces ne peuvent pas être supprimées.

**Exemple conceptuel :**

Même une frontière interne "technique" entre deux modules doit être traçable avec qui l'a créée, quand, et pourquoi.

---

## 4. Ce qu'un développeur ne doit jamais faire

### 4.1. Exécuter un filtrage ou un blocage (INV-BG-1)

**Interdiction contractuelle :**

L'invariant INV-BG-1 établit que Border Guard ne possède **jamais** la capacité d'exécuter une action.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Implémenter des méthodes `filter()`, `block()`, `intercept()` dans Border Guard
- Permettre à Border Guard de rejeter directement une requête
- Créer des middlewares d'exécution dans Border Guard
- Lancer des exceptions de blocage depuis Border Guard

**Conséquence de la violation :**

- Violation de l'invariant INV-BG-1 (aucune exécution)
- Violation de la séparation définition / application
- Compromission de l'architecture fondamentale

### 4.2. Accéder directement à la persistance (INV-BG-2)

**Interdiction contractuelle :**

L'invariant INV-BG-2 établit que Border Guard n'accède **jamais** directement à la persistance.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Importer des drivers de base de données dans Border Guard
- Écrire des queries SQL ou NoSQL dans Border Guard
- Accéder au système de fichiers pour persister des définitions
- Implémenter un cache persisté dans Border Guard

**Conséquence de la violation :**

- Violation de l'invariant INV-BG-2 (aucune persistance directe)
- Violation de la souveraineté de KindMother sur les données
- Risque de désynchronisation

### 4.3. Prendre des décisions autonomes (INV-BG-3)

**Interdiction contractuelle :**

L'invariant INV-BG-3 établit que Border Guard ne prend **jamais** de décision de manière autonome.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Implémenter des méthodes `decide()`, `allow()`, `deny()` dans Border Guard
- Retourner des verdicts (accept/reject) depuis Border Guard
- Créer des logiques if/else décisionnelles dans Border Guard
- Émettre des décisions d'autorisation depuis Border Guard

**Conséquence de la violation :**

- Violation de l'invariant INV-BG-3 (aucune décision autonome)
- Usurpation du rôle de StrongFather
- Compromission de l'architecture de gouvernance

### 4.4. Implémenter des règles procédurales (INV-BG-6)

**Interdiction contractuelle :**

L'invariant INV-BG-6 établit que les règles **doivent** être déclaratives.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Écrire des règles qui décrivent "comment faire" plutôt que "ce qui est requis"
- Inclure du pseudo-code ou des séquences d'étapes dans les règles
- Référencer des technologies spécifiques dans les règles
- Créer des règles qui contiennent de la logique d'exécution

**Conséquence de la violation :**

- Violation de l'invariant INV-BG-6 (règles déclaratives)
- Couplage avec l'implémentation technique
- Impossibilité de portage vers d'autres technologies

### 4.5. Créer des frontières implicites (INV-BG-5)

**Interdiction contractuelle :**

L'invariant INV-BG-5 établit que les frontières **doivent** être explicites.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Vérifier des permissions sans frontière définie formellement
- Créer des zones de confiance implicites dans le code
- Ajouter des points de contrôle non référencés dans Border Guard
- Inférer l'existence de frontières depuis le comportement du système

**Conséquence de la violation :**

- Violation de l'invariant INV-BG-5 (frontières explicites)
- Incohérence de sécurité
- Impossibilité d'audit complet

### 4.6. Omettre la traçabilité (INV-BG-8)

**Interdiction contractuelle :**

L'invariant INV-BG-8 établit que toute définition **doit** être traçable.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Créer des définitions sans métadonnées (createdAt, createdBy, justification)
- Modifier des définitions sans tracer la modification
- Supprimer l'historique des définitions
- Omettre la justification pour des définitions "évidentes"

**Conséquence de la violation :**

- Violation de l'invariant INV-BG-8 (traçabilité complète)
- Impossibilité d'audit
- Perte de responsabilité attribuable

---

## 5. Anti-patterns classiques

### 5.1. Anti-pattern 1 : Filtrage intégré

**Description :**

Tentative d'implémenter des mécanismes de filtrage directement dans Border Guard.

**Exemple conceptuel :**

Un développeur crée une méthode `filterIncomingRequests()` dans Border Guard qui rejette les requêtes non conformes aux règles.

**Conséquence :**

- Violation de l'invariant INV-BG-1 (aucune exécution)
- Violation de l'invariant INV-BG-7 (séparation définition/application)
- Couplage dangereux entre définition et exécution

**Correction :**

Border Guard définit les règles. BondingBrother les consulte et exécute le filtrage.

### 5.2. Anti-pattern 2 : Décision cachée

**Description :**

Tentative de prendre des décisions de manière déguisée en classification.

**Exemple conceptuel :**

Un développeur crée une méthode `isAllowed(source)` qui retourne `true` ou `false` plutôt qu'un niveau de confiance.

**Conséquence :**

- Violation de l'invariant INV-BG-3 (aucune décision autonome)
- Border Guard usurpe le rôle de StrongFather
- Décisions prises sans vision globale

**Correction :**

Border Guard retourne `getTrustLevel(source)` qui retourne un niveau de confiance. StrongFather décide si ce niveau permet l'action.

### 5.3. Anti-pattern 3 : Règles techniques

**Description :**

Tentative de définir des règles qui incluent des détails d'implémentation technique.

**Exemple conceptuel :**

Un développeur crée une règle "Le token JWT doit être signé avec RS256 et avoir un claim 'role' égal à 'admin'".

**Conséquence :**

- Violation de l'invariant INV-BG-6 (règles déclaratives)
- Violation de l'invariant INV-BG-10 (neutralité conceptuelle)
- Couplage avec une technologie spécifique

**Correction :**

La règle devient "Authentification requise avec niveau de privilège administrateur". L'implémentation technique (JWT, SAML, session...) appartient aux adaptateurs.

### 5.4. Anti-pattern 4 : Frontière à la volée

**Description :**

Tentative de créer des frontières dynamiquement au moment du besoin sans les formaliser.

**Exemple conceptuel :**

Un développeur crée une vérification de permission inline dans le code produit, sans frontière définie dans Border Guard.

**Conséquence :**

- Violation de l'invariant INV-BG-5 (frontières explicites)
- Frontières fantômes non auditables
- Incohérence de sécurité

**Correction :**

Toute démarcation de confiance est d'abord définie formellement dans Border Guard, puis utilisée.

### 5.5. Anti-pattern 5 : Persistance directe

**Description :**

Tentative de persister les définitions directement depuis Border Guard.

**Exemple conceptuel :**

Un développeur ajoute un appel `await db.boundaries.insert(boundary)` dans Border Guard.

**Conséquence :**

- Violation de l'invariant INV-BG-2 (aucune persistance directe)
- Violation de la souveraineté de KindMother
- Risque de désynchronisation

**Correction :**

Border Guard émet un événement `boundary-defined`. KindMother écoute et persiste.

### 5.6. Anti-pattern 6 : Classification sans défaut

**Description :**

Tentative de traiter des sources sans les classifier explicitement.

**Exemple conceptuel :**

Un développeur crée un chemin de code qui traite une source sans appeler `getTrustLevel()`, assumant qu'elle est de confiance.

**Conséquence :**

- Violation de l'invariant INV-BG-4 (classification exhaustive)
- Faille de sécurité potentielle
- Sources non classifiées traitées comme de confiance

**Correction :**

Toute source est classifiée. Si non classifiée explicitement, le défaut est "unknown".

---

## 6. Bonnes pratiques conceptuelles

### 6.1. Registre de frontières centralisé

**Pratique :**

Maintenir un registre centralisé des frontières, accessible en lecture par tous les cores mais modifiable uniquement par Border Guard.

**Justification :**

- Respecte l'autorité exclusive de Border Guard sur les frontières (INV-BG-5)
- Garantit l'unicité et l'exhaustivité des définitions
- Facilite la consultation par les autres cores

**Implémentation conceptuelle :**

- Registre en mémoire avec toutes les frontières définies
- API de lecture accessible aux autres cores
- API de modification réservée aux canaux autorisés
- Synchronisation de la persistance via KindMother

### 6.2. Classificateur de confiance avec défaut sécuritaire

**Pratique :**

Implémenter le classificateur de confiance avec un défaut "unknown" systématique pour toute source non explicitement classifiée.

**Justification :**

- Respecte l'invariant INV-BG-4 (classification exhaustive)
- Garantit un comportement sécuritaire par défaut
- Empêche les failles par omission de classification

**Implémentation conceptuelle :**

- Toute requête de classification retourne un niveau
- Si pas de classification explicite, retour de "unknown"
- Le niveau "unknown" déclenche les règles restrictives par défaut

### 6.3. Règles structurées en conditions déclaratives

**Pratique :**

Structurer les règles de franchissement comme des ensembles de conditions déclaratives, sans logique procédurale.

**Justification :**

- Respecte l'invariant INV-BG-6 (règles déclaratives)
- Respecte l'invariant INV-BG-10 (neutralité conceptuelle)
- Permet l'implémentation technique libre par BondingBrother

**Implémentation conceptuelle :**

- Règle = liste de conditions à satisfaire
- Chaque condition est une expression déclarative
- Pas de verbes d'action, pas de séquences d'étapes
- BondingBrother traduit en vérifications techniques

### 6.4. Validation de cohérence à chaque modification

**Pratique :**

Valider la cohérence globale des définitions à chaque création ou modification.

**Justification :**

- Respecte l'invariant INV-BG-9 (cohérence globale)
- Empêche les contradictions entre règles
- Garantit un comportement prévisible

**Implémentation conceptuelle :**

- Avant toute création : vérification de non-contradiction
- Avant toute modification : vérification d'impact sur la cohérence
- Rejet des modifications qui créent des incohérences
- Audit périodique de la cohérence globale

### 6.5. Métadonnées de traçabilité systématiques

**Pratique :**

Inclure systématiquement les métadonnées de traçabilité sur chaque définition.

**Justification :**

- Respecte l'invariant INV-BG-8 (traçabilité complète)
- Permet l'audit et l'attribution de responsabilité
- Facilite la compréhension des décisions passées

**Implémentation conceptuelle :**

- Chaque définition inclut : `createdAt`, `createdBy`, `justification`, `version`
- L'historique des modifications est conservé
- Les consultations peuvent inclure les métadonnées

### 6.6. Séparation claire des interfaces

**Pratique :**

Exposer des interfaces distinctes pour chaque type de consommateur (StrongFather, BondingBrother, CaringNanny).

**Justification :**

- Respecte les contrats d'intégration avec chaque core
- Empêche les usages non prévus
- Facilite l'évolution indépendante

**Implémentation conceptuelle :**

- Interface StrongFather : contexte de confiance pour décision
- Interface BondingBrother : règles de franchissement pour application
- Interface CaringNanny : état des frontières pour observation

---

## 7. Check-list mentale avant toute feature

Avant d'implémenter une nouvelle fonctionnalité liée à Border Guard, un développeur DOIT vérifier mentalement :

### 7.1. Vérification des invariants d'identité

- **INV-BG-1 est-il préservé ?** : La fonctionnalité n'exécute-t-elle aucune action (filtrage, blocage, interception) ?
- **INV-BG-3 est-il préservé ?** : La fonctionnalité ne prend-elle aucune décision autonome ?

### 7.2. Vérification des invariants de comportement

- **INV-BG-2 est-il préservé ?** : La fonctionnalité n'accède-t-elle pas directement à la persistance ?
- **INV-BG-4 est-il préservé ?** : Toute source est-elle classifiée (défaut = unknown) ?
- **INV-BG-5 est-il préservé ?** : Toute frontière est-elle explicitement définie ?
- **INV-BG-6 est-il préservé ?** : Les règles sont-elles purement déclaratives ?

### 7.3. Vérification des invariants de qualité

- **INV-BG-7 est-il préservé ?** : La définition est-elle strictement séparée de l'application ?
- **INV-BG-8 est-il préservé ?** : La traçabilité est-elle complète (origine, date, justification) ?
- **INV-BG-9 est-il préservé ?** : La cohérence globale est-elle maintenue ?
- **INV-BG-10 est-il préservé ?** : Aucune supposition technique n'est-elle faite ?

### 7.4. Vérification de la séparation des responsabilités

- **Border Guard reste-t-il conceptuel ?** : La fonctionnalité définit-elle sans exécuter ?
- **L'autorité de KindMother est-elle respectée ?** : Aucune persistance directe ?
- **L'autorité de StrongFather est-elle respectée ?** : Aucune décision d'autorisation ?
- **L'autorité de BondingBrother est-elle respectée ?** : Aucune application de règles ?

### 7.5. Vérification de la conformité aux Lois d'Autonomie

- **LOI-1 respectée ?** : Aucune dépendance externe critique pour les définitions ?
- **LOI-2 respectée ?** : Les frontières fonctionnent-elles en mode isolé ?
- **LOI-6 respectée ?** : La fédération reste-t-elle explicite, contrôlée, réversible ?

### 7.6. Vérification de la traçabilité et de la cohérence

- **Toutes les définitions sont-elles traçables ?** : Métadonnées complètes ?
- **La cohérence globale est-elle vérifiée ?** : Pas de contradiction détectable ?
- **L'audit est-il possible ?** : Toute définition peut-elle être auditée ?

---

## 8. Interactions avec les autres cores — Guide pratique

### 8.1. Interaction avec StrongFather

**Nature de l'interaction :** Border Guard **informe** StrongFather sur le contexte de confiance.

**Ce que Border Guard fournit :**

- Niveau de confiance de la source (trusted, verified, unknown, hostile)
- Frontières traversées par l'intention
- Règles de franchissement applicables
- État des intégrations concernées

**Ce que Border Guard ne fait JAMAIS :**

- Décider à la place de StrongFather
- Retourner un verdict (accept/reject)
- Bloquer une intention

**Exemple de flux :**

1. StrongFather évalue une intention
2. StrongFather demande à Border Guard : "Quel est le contexte de confiance de cette intention ?"
3. Border Guard retourne : niveau de confiance, frontières, règles
4. StrongFather utilise ces informations pour prendre sa décision

### 8.2. Interaction avec BondingBrother

**Nature de l'interaction :** Border Guard **définit** les règles que BondingBrother **applique**.

**Ce que Border Guard fournit :**

- Règles de franchissement pour chaque frontière
- Niveaux de confiance des sources
- État des intégrations
- Frontières identifiées entre source et destination

**Ce que Border Guard ne fait JAMAIS :**

- Filtrer les interactions
- Appliquer les règles
- Exécuter des vérifications techniques
- Bloquer des accès

**Exemple de flux :**

1. BondingBrother reçoit une intention à médier
2. BondingBrother demande à Border Guard : "Quelles sont les frontières et les règles ?"
3. Border Guard retourne : frontières traversées, règles déclaratives
4. BondingBrother applique les règles techniquement

### 8.3. Interaction avec CaringNanny

**Nature de l'interaction :** Border Guard **informe** CaringNanny sur l'état des frontières.

**Ce que Border Guard fournit :**

- Création/modification/suppression de frontières
- Changements d'état (intégration suspendue, frontière fermée)
- Anomalies détectées sur les frontières

**Ce que Border Guard ne fait JAMAIS :**

- Modifier l'état global du système
- Décider de l'état de santé
- Agir sur l'état observé

**Exemple de flux :**

1. Border Guard détecte un changement (intégration révoquée)
2. Border Guard notifie CaringNanny : "L'intégration X est révoquée"
3. CaringNanny intègre cette information dans l'état global

### 8.4. Interaction avec KindMother

**Nature de l'interaction :** Border Guard **délègue** la persistance à KindMother.

**Ce que Border Guard transmet :**

- Définitions de frontières à persister
- Classifications de confiance à stocker
- Historique des modifications

**Ce que Border Guard ne fait JAMAIS :**

- Accéder directement à la base de données
- Écrire des fichiers
- Gérer un cache persisté

**Exemple de flux :**

1. Border Guard crée une nouvelle frontière
2. Border Guard émet un événement : "Frontière X définie"
3. KindMother reçoit l'événement et persiste la définition

---

## 9. Conclusion

Ce document fournit des lignes directrices pour implémenter Border Guard de manière conforme aux contrats FONDATION.

**Points clés :**

- Border Guard **définit, classifie, et établit des règles** — il **n'exécute jamais**
- Les invariants INV-BG-1 à INV-BG-10 sont des **contraintes absolues**
- La **séparation définition/application** avec BondingBrother est fondamentale
- La **traçabilité est obligatoire** et la **cohérence est vérifiée**
- Les **Lois d'Autonomie** doivent être respectées

**Nature informative :**

Ce document est purement informatif et ne crée aucune nouvelle obligation contractuelle. Il sert uniquement à guider la compréhension et l'application des contrats FONDATION.

**Rappel :** Les contrats FONDATION priment toujours sur ce guide. En cas de doute, se référer à la Documentation Fondatrice et aux contrats spécifiques.

**Phrase fondatrice à garder en mémoire :**

> **Border Guard est l'autorité de définition des frontières et des niveaux de confiance qui établit les règles de franchissement sans jamais les appliquer lui-même, séparant strictement la définition conceptuelle de l'exécution technique.**

---

**Document créé le :** 2026-01-28  
**Version :** 1.0  
**Statut :** POST-FONDATION — Informatif, non normatif, non contractuel  
**Référence :** Miyukini Core System v2.4, Border Guard Documentation Fondatrice, Tous les contrats FONDATION  
**Type :** Guide d'implémentation informatif

---

## 10. Conformité MSCM/MIP

### 10.1 Obligation de balisage MSCM

Tout code implémenté pour Border Guard DOIT être balisé selon le protocole MSCM v1.

**Référence :** [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md)

**Obligations minimales :**
- Chaque bloc fonctionnel DOIT avoir un identifiant unique (`@id`)
- Le rôle sémantique DOIT être explicite (`@role`)
- La couche architecturale DOIT être déclarée (`@layer`)
- Une description humaine DOIT accompagner chaque bloc (`@human`)

### 10.2 Intégration MIP

Après implémentation, l'index MIP DOIT être régénéré pour :
- Valider l'intégrité des blocs MSCM
- Mettre à jour le graphe de dépendances
- Vérifier la cohérence hiérarchique

### 10.3 Check-list MSCM

Avant toute livraison, vérifier :
- [ ] Tous les blocs critiques sont balisés MSCM
- [ ] Les identifiants sont uniques globalement
- [ ] Les couches (layer) sont cohérentes avec l'architecture
- [ ] L'index MIP peut être régénéré sans erreur

---

## 11. Mini log — erreurs / warnings / arbitrages rencontrés

### Arbitrage A1 : Niveau de détail des exemples

**Arbitrage rencontré :** Quel niveau de détail donner aux exemples sans prescrire d'implémentation technique ?

**Décision prise :** Les exemples restent purement conceptuels et narratifs. Aucun code, aucune structure de données spécifique.

**Justification :** Ce document est informatif et non normatif. Les choix techniques appartiennent aux équipes d'implémentation.

**Documentation :** Sections 5 (anti-patterns) et 6 (bonnes pratiques) avec exemples conceptuels uniquement.

### Arbitrage A2 : Références aux contrats d'intégration

**Arbitrage rencontré :** Comment référencer les interactions avec les autres cores sans dupliquer les contrats d'intégration ?

**Décision prise :** Section 8 fournit un guide pratique des interactions, avec renvoi vers les contrats d'intégration pour les détails.

**Justification :** Permet une compréhension rapide sans créer de redondance avec les contrats existants.

**Documentation :** Section 8 avec références vers les contrats d'intégration.

### Arbitrage A3 : Check-list exhaustive vs utilisable

**Arbitrage rencontré :** La check-list des 10 invariants + vérifications additionnelles est-elle trop longue ?

**Décision prise :** Conserver la liste complète car chaque invariant est non négociable. Organisation en sous-sections pour faciliter la lecture.

**Justification :** Omettre des invariants de la check-list risquerait de les faire oublier. La vérification systématique est préférable à une simplification dangereuse.

**Documentation :** Section 7 avec les invariants organisés par catégorie (identité, comportement, qualité).

### Arbitrage A4 : Anti-patterns spécifiques vs génériques

**Arbitrage rencontré :** Fournir des anti-patterns très spécifiques (qui pourraient devenir obsolètes) ou génériques (qui pourraient être trop abstraits) ?

**Décision prise :** Anti-patterns génériques mais illustrés par des exemples conceptuels spécifiques, en évitant le code technique.

**Justification :** Les anti-patterns génériques restent valides dans le temps. Les exemples conceptuels aident à la compréhension sans prescrire d'implémentation.

**Documentation :** Section 5 avec 6 anti-patterns et corrections conceptuelles.

---

*Aucune autre erreur, warning, ou arbitrage rencontré lors de la rédaction de ce document.*
