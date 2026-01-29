# Ever Buddy — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter Ever Buddy correctement, sans violer les contrats FONDATION.

**Objectif pédagogique :** Ce document vise à aider les développeurs à comprendre comment traduire les contrats FONDATION en implémentation, en respectant strictement les invariants, garanties, et interdictions.

**Avertissement :** Ce document ne doit pas être interprété abusivement. Il ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats FONDATION priment toujours sur ce guide.

**Relation avec les contrats FONDATION :** Ce document fait référence aux contrats FONDATION existants mais ne les étend pas, ne les modifie pas, et ne crée aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1. Objectif

Ce document fournit des lignes directrices pour implémenter Ever Buddy de manière conforme aux contrats FONDATION. Il explique comment traduire les concepts contractuels en logique d'implémentation sans interprétation abusive.

### 1.2. Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats FONDATION.

### 1.3. Rappel de la mission d'Ever Buddy

Ever Buddy est le **core de cycle de vie et d'évolution** (Strate 4). Il répond à la question fondamentale :

> **"Comment le système évolue-t-il sans jamais se rompre ?"**

Ever Buddy **observe, enregistre, et guide** l'évolution du système. Il **ne migre jamais**, **ne modifie jamais**, et **n'exécute jamais**.

### 1.4. Sources contractuelles

Ce document se base sur les contrats FONDATION, avec un focus particulier sur :

- **Documentation Fondatrice** : Invariants INV-EB-1 à INV-EB-12, responsabilités exclusives, interdictions
- **Lifecycle States Contract** : États DRAFT, ACTIVE, DEPRECATED, RETIRED, ARCHIVED
- **Transition Rules Contract** : Matrice des transitions valides, périodes minimales
- **Compatibility Rules Contract** : Rétrocompatibilité, compatibilité amont, ruptures
- **Invariants & Guarantees** : Garanties structurelles non négociables
- **[Miyukini Conceptual References — Lois Autonomie Système](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Les lignes directrices d'implémentation doivent respecter les 6 lois d'autonomie, notamment **LOI-1** (aucune dépendance externe critique), **LOI-3** (état local souverain), **LOI-4** (pas de temps global requis).

---

## 2. Principes généraux à respecter absolument

### 2.1. Séparation gouvernance / exécution (INV-EB-1)

**Principe contractuel :**

L'invariant INV-EB-1 établit qu'Ever Buddy ne possède **jamais** la capacité d'exécuter une migration, une transformation, ou une modification de données. Il définit les règles et observe les transitions, mais toute exécution est déléguée aux autorités compétentes.

**Traduction en logique d'implémentation :**

- **Ever Buddy OBSERVE** : Il enregistre les états, les transitions, et l'historique.
- **Ever Buddy DÉFINIT** : Il établit les règles d'évolution et de compatibilité.
- **Ever Buddy NE FAIT JAMAIS** : Il n'exécute aucune migration, ne modifie aucune donnée.

**Ce que cela signifie concrètement :**

- Aucun mécanisme d'écriture de données ne doit être accessible à Ever Buddy
- Les migrations sont exécutées par KindMother (pour les données) ou par les produits (pour leur code)
- Ever Buddy fournit les règles, les calendriers, et les validations — jamais l'exécution

### 2.2. Traçabilité complète et immuable (INV-EB-2)

**Principe contractuel :**

L'invariant INV-EB-2 établit que toute transition d'état de cycle de vie est **obligatoirement** enregistrée et cet enregistrement est **immuable**. L'historique ne peut être ni modifié, ni effacé, ni falsifié.

**Traduction en logique d'implémentation :**

- **Traçabilité systématique** : Chaque transition DOIT être enregistrée avec son contexte complet.
- **Immuabilité garantie** : Les traces NE PEUVENT JAMAIS être modifiées après création.
- **Accessibilité auditée** : L'historique DOIT être accessible pour audit par les acteurs autorisés.

**Ce que cela signifie concrètement :**

- Toute transition est tracée avec : raison, impact, date, chemin de migration (si applicable)
- Les traces sont append-only — aucune modification, aucune suppression
- L'historique complet est consultable pour comprendre les décisions passées

### 2.3. Aucun état ambigu (INV-EB-3)

**Principe contractuel :**

L'invariant INV-EB-3 établit que chaque élément du système possède **exactement un** état de cycle de vie à tout moment. Il n'existe pas d'état intermédiaire, incertain, ou non défini. Les transitions sont atomiques.

**Traduction en logique d'implémentation :**

- **État unique** : Un élément est DRAFT, ACTIVE, DEPRECATED, RETIRED, ou ARCHIVED — jamais entre deux.
- **Transitions atomiques** : Le passage d'un état à un autre est instantané, sans état transitoire.
- **Pas d'ambiguïté** : À tout instant, l'état d'un élément est déterminable sans incertitude.

**Ce que cela signifie concrètement :**

- Le registre des états ne contient jamais d'états "en transition" ou "indéterminés"
- Une transition réussit ou échoue — pas d'état intermédiaire
- L'API d'interrogation retourne toujours un état clair et défini

### 2.4. Période de dépréciation obligatoire (INV-EB-4)

**Principe contractuel :**

L'invariant INV-EB-4 établit qu'aucun élément ACTIVE ne peut passer directement à RETIRED ou ARCHIVED. La transition par DEPRECATED est **obligatoire**. La période de dépréciation minimale ne peut être contournée.

**Traduction en logique d'implémentation :**

- **Passage obligatoire par DEPRECATED** : Toute transition ACTIVE → RETIRED DOIT passer par DEPRECATED.
- **Période minimale respectée** : La durée de dépréciation est définie par catégorie d'élément et non négociable.
- **Aucun raccourci** : Aucun mécanisme ne peut permettre de contourner cette règle.

**Ce que cela signifie concrètement :**

- La matrice des transitions est strictement appliquée
- Toute tentative de transition ACTIVE → RETIRED est rejetée
- Les périodes minimales sont vérifiées avant toute transition DEPRECATED → RETIRED

### 2.5. Rétrocompatibilité par défaut (INV-EB-5)

**Principe contractuel :**

L'invariant INV-EB-5 établit que toute évolution est **présumée rétrocompatible** sauf déclaration explicite contraire. Si une évolution est incompatible, elle doit être explicitement déclarée comme telle.

**Traduction en logique d'implémentation :**

- **Présomption de compatibilité** : Par défaut, une nouvelle version est considérée rétrocompatible.
- **Déclaration explicite des ruptures** : Les breaking changes DOIVENT être déclarés explicitement.
- **Justification obligatoire** : Toute rupture DOIT être justifiée et accompagnée d'un plan de transition.

**Ce que cela signifie concrètement :**

- Le système suppose qu'une évolution mineure est compatible
- Les ruptures sont des exceptions qui nécessitent une documentation explicite
- Aucune rupture silencieuse n'est autorisée

### 2.6. Vision long terme obligatoire (INV-EB-6)

**Principe contractuel :**

L'invariant INV-EB-6 établit que toute décision d'évolution doit considérer l'impact sur **au moins deux générations** de versions. Une évolution qui résout un problème immédiat mais crée un problème futur plus grave est invalide.

**Traduction en logique d'implémentation :**

- **Analyse prospective** : Chaque évolution DOIT être évaluée sur son impact à long terme.
- **Rejet des solutions court-termistes** : Une solution qui accumule la dette est invalide.
- **Pensée générationnelle** : Ever Buddy pense en générations, pas en sprints.

**Ce que cela signifie concrètement :**

- Les décisions d'évolution incluent une analyse d'impact sur N+1 et N+2 générations
- Les solutions qui créent des problèmes futurs sont rejetées même si elles résolvent un problème immédiat
- La dette structurelle est surveillée et limitée

---

## 3. Comment traduire les contrats en logique sans interprétation abusive

### 3.1. Respecter les invariants comme contraintes absolues

**Principe :**

Les invariants contractuels (INV-EB-*) sont des contraintes absolues qui DOIVENT toujours être vraies. Ils ne sont pas des suggestions ou des recommandations.

**Traduction :**

- **Vérification systématique** : Chaque invariant DOIT être vérifié à chaque opération.
- **Préservation garantie** : Toute opération DOIT garantir que les invariants sont préservés après exécution.
- **Pas d'interprétation** : Les invariants ne peuvent pas être interprétés ou adaptés.

**Exemple conceptuel :**

Si l'invariant INV-EB-4 (période de dépréciation obligatoire) exige le passage par DEPRECATED, alors aucune transition directe ACTIVE → RETIRED n'est possible, même pour des raisons "urgentes" ou "exceptionnelles".

### 3.2. Implémenter la traçabilité comme obligation, pas comme option

**Principe :**

La traçabilité complète et immuable (INV-EB-2) est une obligation structurelle, pas une fonctionnalité optionnelle.

**Traduction :**

- **Traçabilité obligatoire** : Chaque transition DOIT être tracée. Aucune exception.
- **Immuabilité structurelle** : Le mécanisme de stockage DOIT garantir l'immuabilité.
- **Accessibilité auditée** : Les traces DOIVENT être accessibles pour audit.

**Exemple conceptuel :**

Même si une transition semble "triviale" (ex: passage d'un élément interne en DEPRECATED), elle DOIT être tracée avec le même niveau de détail qu'une transition majeure.

### 3.3. Traiter la matrice des transitions comme non négociable

**Principe :**

La matrice des transitions valides est absolue. Seules les transitions marquées ✓ sont autorisées.

**Traduction :**

- **Validation stricte** : Toute transition DOIT être validée contre la matrice avant exécution.
- **Rejet immédiat** : Les transitions invalides sont rejetées immédiatement, sans exception.
- **Pas de contournement** : Aucun mécanisme ne peut permettre une transition invalide.

**Exemple conceptuel :**

Une demande de transition RETIRED → ACTIVE est structurellement impossible. Elle est rejetée sans évaluation de la raison ou de l'urgence.

### 3.4. Ne pas "optimiser" en contournant les périodes de transition

**Principe :**

Les périodes minimales de transition sont des protections pour les consommateurs, pas des inefficacités à éliminer.

**Traduction :**

- **Périodes respectées** : Les périodes minimales ne peuvent pas être raccourcies.
- **Pas d'optimisation au détriment de la protection** : La protection des consommateurs prime sur la commodité.
- **Calcul honnête** : Les périodes sont calculées selon les règles définies, sans manipulation.

**Exemple conceptuel :**

Même si tous les consommateurs connus ont migré, la période minimale de dépréciation doit être respectée. Il peut exister des consommateurs inconnus.

---

## 4. Ce qu'un développeur ne doit jamais faire

### 4.1. Exécuter une migration (INV-EB-1)

**Interdiction contractuelle :**

L'invariant INV-EB-1 établit qu'Ever Buddy ne possède **jamais** la capacité d'exécuter une migration.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Implémenter des mécanismes de migration de données dans Ever Buddy
- Permettre à Ever Buddy de modifier directement des structures ou des données
- Créer des "migrations automatiques" exécutées par Ever Buddy
- Déléguer l'exécution de migrations à Ever Buddy

**Conséquence de la violation :**

- Violation de l'invariant INV-EB-1 (aucune exécution de migration)
- Violation de la séparation gouvernance / exécution
- Compromission de l'architecture fondamentale du Miyukini Core System

### 4.2. Modifier l'historique (INV-EB-2)

**Interdiction contractuelle :**

L'invariant INV-EB-2 établit que l'historique des transitions est **immuable**.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Implémenter des mécanismes de modification de l'historique
- Permettre la suppression de traces, même "obsolètes"
- Créer des mécanismes de "correction" de l'historique
- Exposer des APIs de modification des enregistrements passés

**Conséquence de la violation :**

- Violation de l'invariant INV-EB-2 (traçabilité immuable)
- Compromission de l'auditabilité du système
- Perte de confiance dans l'historique des évolutions

### 4.3. Permettre des états ambigus (INV-EB-3)

**Interdiction contractuelle :**

L'invariant INV-EB-3 établit qu'il n'existe pas d'état intermédiaire ou incertain.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Créer des états "en transition" ou "pending"
- Permettre des transitions non atomiques
- Exposer des états incertains ou indéterminés
- Implémenter des transitions qui peuvent rester "en cours" indéfiniment

**Conséquence de la violation :**

- Violation de l'invariant INV-EB-3 (aucun état ambigu)
- Compromission de la prédictibilité du système
- Confusion sur l'état réel des éléments

### 4.4. Contourner la dépréciation (INV-EB-4)

**Interdiction contractuelle :**

L'invariant INV-EB-4 établit que le passage par DEPRECATED est **obligatoire**.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Créer des "fast paths" pour éviter la dépréciation
- Permettre des transitions directes ACTIVE → RETIRED
- Implémenter des "exceptions d'urgence" qui contournent DEPRECATED
- Réduire les périodes minimales de dépréciation

**Conséquence de la violation :**

- Violation de l'invariant INV-EB-4 (dépréciation obligatoire)
- Rupture brutale pour les consommateurs
- Perte de confiance dans les règles d'évolution

### 4.5. Prendre des décisions à la place de StrongFather

**Interdiction contractuelle :**

Ever Buddy fournit le contexte de cycle de vie, mais la décision d'autoriser ou non une action appartient à StrongFather.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Implémenter des décisions d'autorisation dans Ever Buddy
- Bloquer des actions directement sans passer par StrongFather
- Créer des mécanismes de "décision automatique" dans Ever Buddy
- Confondre "information de cycle de vie" et "décision d'autorisation"

**Conséquence de la violation :**

- Violation de la séparation des autorités entre cores
- Conflit d'autorité avec StrongFather
- Compromission de l'architecture de gouvernance

### 4.6. Modifier les données de KindMother

**Interdiction contractuelle :**

Ever Buddy **ne modifie jamais** les données gérées par KindMother.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Permettre à Ever Buddy d'écrire dans les données de KindMother
- Créer des "mises à jour de schéma" exécutées par Ever Buddy
- Implémenter des "corrections de données" dans Ever Buddy
- Accéder directement aux mécanismes de persistance de KindMother

**Conséquence de la violation :**

- Violation de l'autorité exclusive de KindMother sur les données
- Violation de l'invariant INV-EB-1 (pas d'exécution)
- Compromission de l'intégrité des données

---

## 5. Anti-patterns classiques

### 5.1. Anti-pattern 1 : Migration automatique

**Description :**

Tentative d'implémenter des migrations automatiques exécutées par Ever Buddy lors des transitions d'état.

**Exemple conceptuel :**

Un développeur crée un mécanisme où quand un schéma passe de DEPRECATED à RETIRED, Ever Buddy exécute automatiquement une migration des données vers le nouveau schéma.

**Conséquence :**

- Violation de l'invariant INV-EB-1 (aucune exécution de migration)
- Violation de l'autorité de KindMother sur les données
- Couplage dangereux entre gouvernance et exécution

**Correction :**

Ever Buddy définit les règles de migration et communique les calendriers. L'exécution de la migration est la responsabilité de KindMother (pour les données) ou des produits (pour leur code).

### 5.2. Anti-pattern 2 : Historique modifiable

**Description :**

Tentative de permettre la modification de l'historique pour "corriger des erreurs" ou "nettoyer les données obsolètes".

**Exemple conceptuel :**

Un développeur implémente une fonction "cleanHistory()" pour supprimer les anciennes traces de transition jugées "inutiles".

**Conséquence :**

- Violation de l'invariant INV-EB-2 (traçabilité immuable)
- Perte de la capacité d'audit
- Compromission de la confiance dans l'historique

**Correction :**

L'historique est strictement append-only. Les traces ne sont jamais modifiées ni supprimées. Si l'espace devient un problème, des mécanismes d'archivage (pas de suppression) peuvent être envisagés.

### 5.3. Anti-pattern 3 : États de transition

**Description :**

Tentative de créer des états intermédiaires pour gérer les transitions complexes.

**Exemple conceptuel :**

Un développeur crée un état "DEPRECATING" entre ACTIVE et DEPRECATED pour gérer la "transition en cours".

**Conséquence :**

- Violation de l'invariant INV-EB-3 (aucun état ambigu)
- Ambiguïté sur l'état réel des éléments
- Complexité inutile et risque d'états bloqués

**Correction :**

Les transitions sont atomiques. Un élément est ACTIVE, puis instantanément DEPRECATED. Il n'y a pas d'état intermédiaire. Les processus de préparation se font avant la transition, pas pendant.

### 5.4. Anti-pattern 4 : Fast path de retirement

**Description :**

Tentative de créer un chemin rapide pour retirer des éléments "urgents" sans passer par la dépréciation.

**Exemple conceptuel :**

Un développeur crée une fonction "forceRetire()" qui permet de passer directement de ACTIVE à RETIRED en cas d'"urgence sécurité".

**Conséquence :**

- Violation de l'invariant INV-EB-4 (dépréciation obligatoire)
- Rupture brutale pour les consommateurs
- Perte de confiance dans les règles d'évolution

**Correction :**

Même en cas d'urgence, le passage par DEPRECATED est obligatoire. La période de dépréciation peut être réduite au minimum défini, mais jamais contournée. Les urgences de sécurité peuvent justifier une période minimale très courte, mais pas l'absence de période.

### 5.5. Anti-pattern 5 : Décision d'autorisation intégrée

**Description :**

Tentative de faire prendre des décisions d'autorisation à Ever Buddy basées sur l'état de cycle de vie.

**Exemple conceptuel :**

Un développeur implémente dans Ever Buddy la logique "si l'élément est DEPRECATED, bloquer les nouvelles intégrations".

**Conséquence :**

- Violation de la séparation des autorités
- Conflit avec StrongFather (qui décide des autorisations)
- Couplage dangereux entre cores

**Correction :**

Ever Buddy fournit l'information de cycle de vie à StrongFather ("cet élément est DEPRECATED"). StrongFather décide si l'action est autorisée. Ever Buddy ne prend jamais de décision d'autorisation.

---

## 6. Bonnes pratiques conceptuelles

### 6.1. Registre d'états centralisé et souverain

**Pratique :**

Maintenir un registre centralisé des états de cycle de vie, accessible en lecture par tous les cores mais en écriture uniquement par Ever Buddy.

**Justification :**

- Respecte l'autorité exclusive d'Ever Buddy sur les états de vie (Section 5.1 de la Documentation Fondatrice)
- Garantit l'unicité de l'état (INV-EB-3)
- Facilite la consultation par les autres cores

**Implémentation conceptuelle :**

- Registre centralisé avec états courants
- API de lecture accessible aux autres cores
- API d'écriture réservée à Ever Buddy
- Synchronisation avec l'historique immuable

### 6.2. Historique append-only avec signatures

**Pratique :**

Implémenter l'historique comme une structure append-only avec des signatures cryptographiques pour garantir l'intégrité.

**Justification :**

- Respecte l'invariant INV-EB-2 (traçabilité immuable)
- Garantit l'impossibilité de modification
- Permet l'audit et la vérification

**Implémentation conceptuelle :**

- Structure de données append-only (log immuable)
- Signature de chaque entrée
- Chaînage des signatures pour détecter les modifications
- Pas de mécanisme de suppression ou modification

### 6.3. Validation stricte des transitions avant enregistrement

**Pratique :**

Valider toute transition contre la matrice des transitions valides AVANT de l'enregistrer.

**Justification :**

- Respecte la matrice des transitions (Section 4 de la Documentation Fondatrice)
- Garantit qu'aucune transition invalide n'est enregistrée
- Préserve la cohérence du système

**Implémentation conceptuelle :**

- Validation de la transition (état source → état cible) contre la matrice
- Vérification des périodes minimales si applicable
- Rejet immédiat si la transition est invalide
- Enregistrement atomique si la transition est valide

### 6.4. Séparation claire entre observation et action

**Pratique :**

Maintenir une séparation architecturale claire entre les fonctions d'observation/enregistrement d'Ever Buddy et les fonctions d'action des autres cores.

**Justification :**

- Respecte l'invariant INV-EB-1 (aucune exécution)
- Garantit la séparation gouvernance / exécution
- Facilite l'audit et la compréhension

**Implémentation conceptuelle :**

- Ever Buddy n'a accès à aucun mécanisme d'écriture de données métier
- Les interfaces d'Ever Buddy sont strictement de lecture et d'enregistrement de transitions
- Les actions de migration sont explicitement déléguées aux autorités compétentes

### 6.5. Communication proactive des calendriers d'évolution

**Pratique :**

Communiquer proactivement les calendriers de dépréciation et les plans de transition à tous les consommateurs concernés.

**Justification :**

- Respecte l'invariant INV-EB-12 (responsabilité de l'annonce)
- Garantit que les consommateurs ont le temps de réagir
- Préserve la confiance dans les règles d'évolution

**Implémentation conceptuelle :**

- Publication des calendriers de dépréciation
- Notifications aux consommateurs concernés
- Suivi de l'accusé de réception des annonces
- Documentation des communications pour audit

### 6.6. Surveillance active de la dette structurelle

**Pratique :**

Surveiller activement le ratio de dette structurelle et alerter quand il dépasse les seuils définis.

**Justification :**

- Respecte la responsabilité de surveillance de la dette (Section 5.4 de la Documentation Fondatrice)
- Empêche l'accumulation non contrôlée
- Permet des actions correctives avant que la dette ne devienne critique

**Implémentation conceptuelle :**

- Calcul du debt ratio : (DEPRECATED + RETIRED) / ACTIVE
- Définition de seuils d'alerte
- Émission d'alertes quand les seuils sont dépassés
- Recommandations de nettoyage

---

## 7. Check-list mentale avant toute feature

Avant d'implémenter une nouvelle fonctionnalité liée à Ever Buddy, un développeur DOIT vérifier mentalement :

### 7.1. Vérification des invariants

- **INV-EB-1 est-il préservé ?** : La fonctionnalité n'exécute-t-elle aucune migration ?
- **INV-EB-2 est-il préservé ?** : L'historique reste-t-il immuable ?
- **INV-EB-3 est-il préservé ?** : Aucun état ambigu n'est-il créé ?
- **INV-EB-4 est-il préservé ?** : Le passage par DEPRECATED est-il obligatoire ?
- **INV-EB-5 est-il préservé ?** : La rétrocompatibilité est-elle présumée par défaut ?
- **INV-EB-6 est-il préservé ?** : L'impact long terme est-il considéré ?
- **INV-EB-7 est-il préservé ?** : La documentation est-elle obligatoire ?
- **INV-EB-8 est-il préservé ?** : Les règles sont-elles universelles ?
- **INV-EB-9 est-il préservé ?** : Les règles sont-elles publiques et stables ?
- **INV-EB-10 est-il préservé ?** : Un seul successeur est-il déclaré ?
- **INV-EB-11 est-il préservé ?** : Les changements de règles ne sont-ils pas rétroactifs ?
- **INV-EB-12 est-il préservé ?** : La responsabilité d'annonce est-elle respectée ?

### 7.2. Vérification de la séparation des responsabilités

- **Ever Buddy reste-t-il observateur ?** : La fonctionnalité n'exécute-t-elle rien ?
- **L'autorité de KindMother est-elle respectée ?** : Aucune modification de données ?
- **L'autorité de StrongFather est-elle respectée ?** : Aucune décision d'autorisation ?
- **Les autres cores sont-ils informés, pas contraints ?** : Ever Buddy informe, il ne commande pas.

### 7.3. Vérification de la conformité aux Lois d'Autonomie

- **LOI-1 respectée ?** : Aucune dépendance externe critique pour les états de vie ?
- **LOI-3 respectée ?** : L'état local est souverain ?
- **LOI-4 respectée ?** : Pas de temps global requis pour les transitions ?

### 7.4. Vérification de la traçabilité

- **Toutes les transitions sont-elles tracées ?** : Aucune transition silencieuse ?
- **Les traces sont-elles immuables ?** : Aucune modification possible ?
- **Les traces sont-elles accessibles ?** : Audit possible ?

---

## 8. Conclusion

Ce document fournit des lignes directrices pour implémenter Ever Buddy de manière conforme aux contrats FONDATION.

**Points clés :**

- Ever Buddy **observe, enregistre, et guide** — il **n'exécute jamais**
- Les invariants INV-EB-1 à INV-EB-12 sont des **contraintes absolues**
- La **traçabilité est immuable** et la **dépréciation est obligatoire**
- La **séparation gouvernance / exécution** est fondamentale
- Les **Lois d'Autonomie** doivent être respectées

**Nature informative :**

Ce document est purement informatif et ne crée aucune nouvelle obligation contractuelle. Il sert uniquement à guider la compréhension et l'application des contrats FONDATION.

**Rappel :** Les contrats FONDATION priment toujours sur ce guide. En cas de doute, se référer à la Documentation Fondatrice et aux contrats spécifiques.

**Phrase fondatrice à garder en mémoire :**

> **Ever Buddy est le compagnon de toujours qui observe, enregistre, et guide l'évolution du système, garantissant que chaque changement respecte la continuité, que chaque transition est traçable, et que l'avenir est préparé sans sacrifier le présent.**

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** POST-FONDATION — Informatif, non normatif, non contractuel  
**Référence :** Miyukini Core System v2.4, Ever Buddy Documentation Fondatrice, Tous les contrats FONDATION  
**Type :** Guide d'implémentation informatif

---

## 9. Conformité MSCM/MIP

### 9.1 Obligation de balisage MSCM

Tout code implémenté pour Ever Buddy DOIT être balisé selon le protocole MSCM v1.

**Référence :** [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md)

**Obligations minimales :**
- Chaque bloc fonctionnel DOIT avoir un identifiant unique (`@id`)
- Le rôle sémantique DOIT être explicite (`@role`)
- La couche architecturale DOIT être déclarée (`@layer`)
- Une description humaine DOIT accompagner chaque bloc (`@human`)

### 9.2 Intégration MIP

Après implémentation, l'index MIP DOIT être régénéré pour :
- Valider l'intégrité des blocs MSCM
- Mettre à jour le graphe de dépendances
- Vérifier la cohérence hiérarchique

### 9.3 Check-list MSCM

Avant toute livraison, vérifier :
- [ ] Tous les blocs critiques sont balisés MSCM
- [ ] Les identifiants sont uniques globalement
- [ ] Les couches (layer) sont cohérentes avec l'architecture
- [ ] L'index MIP peut être régénéré sans erreur

---

## 10. Mini log — erreurs / warnings / arbitrages rencontrés

### Arbitrage A1 : Niveau de détail des exemples

**Arbitrage rencontré :** Quel niveau de détail donner aux exemples sans prescrire d'implémentation technique ?

**Décision prise :** Les exemples restent purement conceptuels et narratifs. Aucun code, aucune structure de données spécifique.

**Justification :** Ce document est informatif et non normatif. Les choix techniques appartiennent aux équipes d'implémentation.

**Documentation :** Sections 4 (anti-patterns) et 5 (bonnes pratiques) avec exemples conceptuels uniquement.

### Arbitrage A2 : Références aux autres cores

**Arbitrage rencontré :** Comment référencer les interactions avec les autres cores sans créer de dépendances documentaires ?

**Décision prise :** Références génériques aux responsabilités des autres cores (KindMother pour les données, StrongFather pour les décisions) sans lier à des documents spécifiques de ces cores.

**Justification :** Permet l'évolution indépendante des documentations tout en préservant la cohérence conceptuelle.

**Documentation :** Sections 2, 4, 6 avec références génériques.

### Arbitrage A3 : Check-list exhaustive vs utilisable

**Arbitrage rencontré :** La check-list des 12 invariants est-elle trop longue pour être utilisable ?

**Décision prise :** Conserver la liste complète car chaque invariant est non négociable. La longueur reflète la complexité réelle des contraintes.

**Justification :** Omettre des invariants de la check-list risquerait de les faire oublier. La vérification systématique est préférable à une simplification dangereuse.

**Documentation :** Section 7 avec les 12 invariants listés.

### Arbitrage A4 : Anti-patterns spécifiques vs génériques

**Arbitrage rencontré :** Fournir des anti-patterns très spécifiques (qui pourraient devenir obsolètes) ou génériques (qui pourraient être trop abstraits) ?

**Décision prise :** Anti-patterns génériques mais illustrés par des exemples conceptuels spécifiques.

**Justification :** Les anti-patterns génériques restent valides dans le temps. Les exemples concrets aident à la compréhension sans prescrire d'implémentation.

**Documentation :** Section 5 avec anti-patterns génériques et exemples conceptuels.

---

*Aucune autre erreur, warning, ou arbitrage rencontré lors de la rédaction de ce document.*
