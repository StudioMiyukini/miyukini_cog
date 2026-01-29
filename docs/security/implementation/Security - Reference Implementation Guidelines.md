# Miyukini Security — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide les développeurs pour implémenter la sécurité correctement dans le système Miyukini, sans violer les contrats FONDATION.

**Objectif pédagogique :** Ce document vise à aider les développeurs à comprendre comment traduire les principes de sécurité en implémentation, en respectant strictement les lois, invariants, garanties, et interdictions.

**Avertissement :** Ce document ne doit pas être interprété abusivement. Il ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats FONDATION priment toujours sur ce guide.

**Relation avec les contrats FONDATION :** Ce document fait référence aux contrats FONDATION existants mais ne les étend pas, ne les modifie pas, et ne crée aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1. Objectif

Ce document fournit des lignes directrices pour implémenter la sécurité de manière conforme aux contrats FONDATION et à la [Doctrine Securite Fondamentale](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md). Il explique comment traduire les concepts de sécurité en logique d'implémentation sans interprétation abusive.

### 1.2. Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des principes de sécurité Miyukini.

### 1.3. Rappel du principe fondateur

> **"La sécurité dans Miyukini n'est pas un module, ni une fonctionnalité, ni un service. Elle est une propriété structurelle du système."**

La sécurité existe comme :
- **Loi d'architecture** : règle non négociable de conception
- **Contrainte de fonctionnement** : limite imposée à tout comportement
- **Règle de conception** : principe directeur de développement
- **Invariant système** : propriété toujours vraie
- **Principe de gouvernance** : cadre de décision

### 1.4. Sources contractuelles

Ce document se base sur les contrats FONDATION, avec un focus particulier sur :

- **[Doctrine Securite Fondamentale](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)** : Principe fondateur, postulats, strates, engines
- **[Security - Documentation Fondatrice](../foundation/Security%20-%20Documentation%20Fondatrice.md)** : Vision opérationnelle
- **[Security - Architecture & Components](../architecture/Security%20-%20Architecture%20&%20Components.md)** : Les 8 Security Engines
- **[Security - Invariants & Guarantees](../contracts/governance/Security%20-%20Invariants%20&%20Guarantees.md)** : Lois et contraintes
- **[Security - Violations & Anti-Patterns](../contracts/governance/Security%20-%20Violations%20&%20Anti-Patterns.md)** : Ce qu'il ne faut jamais faire
- **[Security Levels](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)** : Niveaux de sécurité (0-4)
- **[Integrity Degradation System](../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)** : Système de dégradation (T0-T4)

---

## 2. Principes généraux à respecter absolument

### 2.1. La sécurité comme propriété structurelle

**Principe contractuel :**

La Doctrine établit que la sécurité n'est pas un module ajouté mais une propriété émergente du système.

**Traduction en logique d'implémentation :**

- **Pas de module "security"** : La sécurité est distribuée dans tous les composants
- **Pas de bypass** : Aucun chemin de code ne contourne les contrôles
- **Défense en profondeur** : Plusieurs couches de protection indépendantes
- **Fail-secure par défaut** : En cas de doute, refuser l'action

**Ce que cela signifie concrètement :**

- Chaque composant intègre ses propres contrôles de sécurité
- Aucune fonction `skipSecurityCheck()` ou paramètre `bypassValidation`
- Les exceptions de sécurité sont catchées et loguées, jamais ignorées
- Le comportement par défaut est restrictif, jamais permissif

### 2.2. Respect des 6 Lois Système (L1-L6)

**Principe contractuel :**

Les 6 lois système sont absolues et non négociables.

| Loi | Description |
|-----|-------------|
| **L1** | Aucun accès direct hardware |
| **L2** | Aucune source de vérité multiple |
| **L3** | Aucun bypass des Cores |
| **L4** | Aucune écriture sans traçabilité |
| **L5** | Aucune décision sans validation |
| **L6** | Aucune structure sans indexation |

**Traduction en logique d'implémentation :**

- **L1** : Toutes les interactions matérielles passent par les abstractions Kernel
- **L2** : Une seule source de vérité (STA/OSV) pour chaque type de donnée
- **L3** : Toutes les opérations passent par les Cores appropriés
- **L4** : Toute opération d'écriture génère une entrée d'audit
- **L5** : Toute décision est précédée d'une validation par StrongFather
- **L6** : Tout élément est indexé dans le MIP

**Ce que cela signifie concrètement :**

- Pas d'appels système directs (fichiers, réseau, hardware)
- Pas de cache local qui pourrait diverger de la vérité centrale
- Toujours passer par les interfaces des Cores
- Toujours émettre des événements d'audit
- Toujours soumettre les décisions à validation
- Toujours enregistrer dans l'index

### 2.3. Respect des contraintes de fonctionnement

**Principe contractuel :**

Tout flux dans le système doit respecter ces contraintes :
- Tout passe par abstraction
- Tout passe par validation
- Tout passe par consensus (pour les décisions critiques)
- Tout passe par versioning

**Traduction en logique d'implémentation :**

- **Abstraction obligatoire** : Utiliser les interfaces fournies, jamais les implémentations directes
- **Validation systématique** : Valider toutes les entrées, tous les états, toutes les transitions
- **Consensus sur les critiques** : Les décisions critiques requièrent plusieurs validateurs
- **Versioning permanent** : Toute modification crée une nouvelle version

**Ce que cela signifie concrètement :**

- Injecter les dépendances via interfaces, pas via implémentations concrètes
- Valider les données à chaque frontière (entrée de fonction, API, événement)
- Utiliser le Consensus Engine pour les décisions à impact élevé
- Ne jamais modifier en place, toujours créer une nouvelle version

### 2.4. Intégration de la chaîne de confiance

**Principe contractuel :**

Tout élément du système s'inscrit dans la chaîne de confiance :

```
CODE → MSCM → MIP → GRAPH → STA → OSV
```

**Traduction en logique d'implémentation :**

- **CODE conforme MSCM** : Le code est balisé selon le protocole MSCM
- **MSCM cohérent MIP** : Les blocs MSCM sont indexés dans le MIP
- **MIP correspondant au Graph** : Le MIP reflète le graphe système
- **Graph ancré dans STA** : Le graphe est validé par le System Truth Anchor
- **STA correspondant à OSV** : Le STA correspond à une version certifiée

**Ce que cela signifie concrètement :**

- Baliser tout code fonctionnel avec les annotations MSCM
- Régénérer l'index MIP après chaque modification
- Vérifier la cohérence du graphe de dépendances
- Comparer régulièrement avec le STA
- Ne déployer que des versions OSV certifiées

### 2.5. Traçabilité complète obligatoire

**Principe contractuel :**

L'invariant de traçabilité établit qu'aucune action ne peut exister sans trace.

**Traduction en logique d'implémentation :**

- **Audit systématique** : Toute opération génère une entrée d'audit
- **Métadonnées obligatoires** : Qui, quand, quoi, pourquoi
- **Historique immuable** : Les traces ne peuvent pas être supprimées
- **Corrélation possible** : Les traces permettent de reconstruire les flux

**Ce que cela signifie concrètement :**

- Chaque fonction publique émet un événement d'audit
- Les métadonnées incluent : `actor`, `timestamp`, `action`, `justification`, `correlationId`
- Les logs sont envoyés à un stockage immuable (append-only)
- Un identifiant de corrélation traverse toute la chaîne d'appels

---

## 3. Intégration des Security Engines

### 3.1. Vue d'ensemble des 8 Engines

Les 8 Security Engines constituent la strate d'infrastructure systémique obligatoire :

| Engine | Rôle | Intégration développeur |
|--------|------|------------------------|
| **Integrity Engine** | Vérification permanente | Valider les checksums, comparer avec STA |
| **Validation Engine** | Filtrage systémique | Valider toutes les entrées |
| **Policy Engine** | Règles de fonctionnement | Consulter les politiques avant action |
| **Consensus Engine** | Décisions pluralistes | Soumettre les décisions critiques |
| **Audit Engine** | Traçabilité active | Émettre des événements d'audit |
| **Sandbox Engine** | Isolement | Exécuter le code non fiable en isolation |
| **Cognitive Guard** | Sécurité IA | Soumettre les décisions IA au contrôle |
| **Recovery Engine** | Résilience | Gérer les snapshots et rollbacks |

### 3.2. Patterns d'intégration par Engine

#### Integrity Engine — Vérification d'intégrité

**Pattern conceptuel :**

Avant toute opération critique, vérifier que l'état actuel correspond à l'état certifié.

**Points d'intégration :**
- Au démarrage de chaque service
- Avant chaque opération d'écriture
- Après chaque synchronisation
- Périodiquement en arrière-plan

**Comportement attendu :**
- Si intègre : continuer l'opération
- Si dérive mineure : alerter et continuer
- Si violation : bloquer et notifier le Recovery Engine

#### Validation Engine — Validation des entrées

**Pattern conceptuel :**

Toute donnée entrant dans le système doit être validée avant traitement.

**Points d'intégration :**
- À chaque API endpoint
- À chaque réception d'événement
- À chaque lecture de données externes
- À chaque paramètre de fonction publique

**Comportement attendu :**
- Validation de schéma (structure, types)
- Validation de format (encodage, taille)
- Validation de cohérence (relations, références)
- Rejet si invalide, jamais de correction automatique

#### Policy Engine — Application des politiques

**Pattern conceptuel :**

Avant toute action, vérifier que les politiques applicables sont satisfaites.

**Points d'intégration :**
- Avant chaque opération CRUD
- Avant chaque appel à un service externe
- Avant chaque changement d'état
- Avant chaque décision système

**Comportement attendu :**
- Consulter les politiques applicables
- Évaluer chaque politique
- Si toutes satisfaites : autoriser
- Si une non satisfaite : refuser avec justification

#### Consensus Engine — Validation pluraliste

**Pattern conceptuel :**

Les décisions critiques requièrent l'accord de plusieurs validateurs.

**Points d'intégration :**
- Décisions à impact élevé
- Modifications de configuration sensible
- Suppressions irréversibles
- Décisions IA à fort impact

**Comportement attendu :**
- Soumettre la décision à plusieurs validateurs
- Attendre le consensus (ou le dissensus)
- Si consensus : exécuter
- Si dissensus : escalader vers l'humain (TAMR)

#### Audit Engine — Traçabilité

**Pattern conceptuel :**

Toute action génère une trace d'audit immuable.

**Points d'intégration :**
- À chaque opération réussie
- À chaque opération échouée
- À chaque décision (système ou IA)
- À chaque changement d'état

**Comportement attendu :**
- Émettre un événement d'audit structuré
- Inclure toutes les métadonnées requises
- Ne jamais bloquer l'opération en cas d'échec d'audit (alerter)
- Garantir l'ordre chronologique

#### Sandbox Engine — Isolement

**Pattern conceptuel :**

Le code ou les données non fiables sont exécutés/traités en isolation.

**Points d'intégration :**
- Exécution de code externe
- Traitement de données non validées
- Test de nouvelles fonctionnalités
- Simulation de décisions

**Comportement attendu :**
- Créer un environnement isolé
- Limiter les ressources (CPU, mémoire, réseau)
- Capturer les résultats
- Valider les résultats avant sortie du sandbox

#### Cognitive Guard — Sécurité IA

**Pattern conceptuel :**

Les décisions IA sont surveillées pour détecter les dérives et les biais.

**Points d'intégration :**
- Avant chaque décision IA
- Après chaque séquence de décisions
- En continu sur les patterns de décision
- Sur demande d'audit cognitif

**Comportement attendu :**
- Analyser la décision pour détecter les dérives
- Vérifier les biais systématiques
- Détecter les feedback loops
- Si confiant : autoriser ; si suspect : escalader au Consensus Engine

#### Recovery Engine — Résilience

**Pattern conceptuel :**

Le système peut se restaurer à un état sûr en cas d'incident.

**Points d'intégration :**
- Création de snapshots avant opérations critiques
- Détection d'incidents (via Integrity Engine)
- Rollback sur demande
- Restauration automatique si nécessaire

**Comportement attendu :**
- Créer des snapshots cohérents
- Détecter rapidement les incidents
- Restaurer vers la dernière OSV valide
- Garantir la cohérence après restauration

---

## 4. Patterns d'implémentation sécurisée

### 4.1. Pattern : Validation en couches

**Description :**

Chaque couche du système valide ses entrées indépendamment des autres.

**Justification :**
- Défense en profondeur : si une couche échoue, les autres protègent
- Isolation des responsabilités : chaque couche connaît ses contraintes
- Détection précoce : les erreurs sont détectées au plus tôt

**Application conceptuelle :**

```
[API Layer] → Validation schéma + auth
     ↓
[Service Layer] → Validation métier + permissions
     ↓
[Domain Layer] → Validation invariants domaine
     ↓
[Persistence Layer] → Validation contraintes DB
```

**Points clés :**
- Ne jamais présupposer que la couche précédente a validé
- Chaque couche a ses propres règles de validation
- Les erreurs de validation ne doivent jamais être ignorées

### 4.2. Pattern : Zero-Trust interne

**Description :**

Aucun composant ne fait confiance à un autre sans vérification.

**Justification :**
- Protection contre la compromission partielle
- Détection des comportements anormaux
- Résilience face aux erreurs de programmation

**Application conceptuelle :**

```
[Component A] appelle [Component B]
     ↓
[Component B] vérifie :
  - L'identité de A est valide
  - A a le droit d'appeler cette fonction
  - Les paramètres sont valides
  - L'état actuel permet l'opération
```

**Points clés :**
- Toujours vérifier l'appelant
- Toujours vérifier les paramètres
- Ne jamais présupposer le contexte d'appel
- Logger les appels suspects

### 4.3. Pattern : Immutabilité par défaut

**Description :**

Les structures de données sont immuables sauf nécessité explicite.

**Justification :**
- Évite les modifications non autorisées
- Facilite le versioning et le rollback
- Simplifie le raisonnement sur l'état

**Application conceptuelle :**

```
[Données originales]
     ↓
[Création nouvelle version] (copie modifiée)
     ↓
[Validation de la nouvelle version]
     ↓
[Remplacement atomique de la référence]
```

**Points clés :**
- Les objets métier sont immuables
- Les modifications créent de nouvelles versions
- Les anciennes versions sont conservées pour l'historique
- Les références sont mises à jour atomiquement

### 4.4. Pattern : Fail-fast et Fail-secure

**Description :**

En cas d'erreur, échouer rapidement et de manière sécurisée.

**Justification :**
- Évite la propagation des erreurs
- Limite les dégâts en cas de problème
- Facilite le diagnostic

**Application conceptuelle :**

```
[Opération]
     ↓
[Erreur détectée]
     ↓
  ├── Fail-fast : arrêter immédiatement
  └── Fail-secure : revenir à un état sûr
     ↓
[Logger l'erreur]
     ↓
[Notifier les observateurs]
```

**Points clés :**
- Ne jamais continuer après une erreur de sécurité
- En cas de doute, refuser l'action
- Toujours logger les erreurs avec contexte
- Avoir un état de repli sécurisé

### 4.5. Pattern : Audit omniscient

**Description :**

Tout est tracé, tout le temps, sans exception.

**Justification :**
- Traçabilité complète pour l'audit
- Détection des comportements anormaux
- Reconstruction des événements en cas d'incident

**Application conceptuelle :**

```
[Début opération]
     ↓
[Émettre : audit_operation_started]
     ↓
[Exécution]
     ↓
[Émettre : audit_operation_completed/failed]
     ↓
[Inclure : who, when, what, why, result, duration]
```

**Points clés :**
- Chaque opération a un début et une fin audités
- Les métadonnées sont complètes et structurées
- L'audit ne bloque jamais l'opération principale
- Les événements d'audit sont immuables

---

## 5. Ce qu'un développeur ne doit jamais faire

### 5.1. Bypass des contrôles de sécurité

**Interdiction contractuelle :**

Les lois L3 (aucun bypass des Cores) et L5 (aucune décision sans validation) interdisent tout contournement des contrôles.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Créer des paramètres `skipValidation`, `bypassSecurity`, `unsafeMode`
- Implémenter des "backdoors" pour le développement ou les tests
- Appeler directement les implémentations au lieu des interfaces des Cores
- Ignorer les erreurs de validation pour "simplifier"

**Conséquence de la violation :**

- Violation des lois système L3 et L5
- Compromission de l'architecture de sécurité
- Perte de la propriété fail-secure

### 5.2. Accès direct aux ressources système

**Interdiction contractuelle :**

La loi L1 (aucun accès direct hardware) et la contrainte d'abstraction interdisent les accès directs.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Utiliser des appels système directs (fichiers, réseau, processus)
- Accéder au hardware sans passer par le Kernel
- Lire/écrire des fichiers sans passer par KindMother
- Ouvrir des connexions réseau sans passer par les abstractions

**Conséquence de la violation :**

- Violation de la loi système L1
- Perte de traçabilité
- Risque de corruption de données

### 5.3. Sources de vérité multiples

**Interdiction contractuelle :**

La loi L2 (aucune source de vérité multiple) interdit les caches ou copies qui peuvent diverger.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Créer des caches locaux sans invalidation par STA
- Stocker des copies de données qui peuvent diverger
- Maintenir des états déconnectés de la source de vérité
- Implémenter des synchronisations "éventuellement cohérentes" sans contrôle

**Conséquence de la violation :**

- Violation de la loi système L2
- Incohérence des données
- Décisions basées sur des données obsolètes

### 5.4. Écriture sans traçabilité

**Interdiction contractuelle :**

La loi L4 (aucune écriture sans traçabilité) exige que toute modification soit tracée.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Modifier des données sans émettre d'événement d'audit
- Supprimer des traces ou des logs
- Modifier en place sans créer de version
- Omettre les métadonnées d'audit "pour la performance"

**Conséquence de la violation :**

- Violation de la loi système L4
- Perte de traçabilité
- Impossibilité d'audit

### 5.5. Décisions non validées

**Interdiction contractuelle :**

La loi L5 (aucune décision sans validation) exige que toute décision soit validée.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Prendre des décisions sans passer par StrongFather
- Autoriser des actions sans vérifier les politiques
- Implémenter des décisions "par défaut" non validées
- Contourner le Consensus Engine pour les décisions critiques

**Conséquence de la violation :**

- Violation de la loi système L5
- Décisions non autorisées
- Perte de gouvernance

### 5.6. Structures non indexées

**Interdiction contractuelle :**

La loi L6 (aucune structure sans indexation) exige que tout élément soit indexé.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Créer des éléments non référencés dans le MIP
- Omettre le balisage MSCM sur le code fonctionnel
- Créer des relations non indexées dans le graphe
- Ignorer la régénération du MIP après modification

**Conséquence de la violation :**

- Violation de la loi système L6
- Éléments "fantômes" non traçables
- Incohérence du graphe système

---

## 6. Anti-patterns classiques

### 6.1. Anti-pattern : Security by obscurity

**Description :**

Tentative de sécuriser par l'obfuscation ou le secret.

**Exemple conceptuel :**

Un développeur masque l'existence d'une API sensible au lieu de la protéger par des contrôles d'accès.

**Pourquoi c'est dangereux :**

- Le secret finit toujours par être découvert
- Aucune défense en cas de découverte
- Fausse sensation de sécurité

**Correction :**

Appliquer les contrôles de sécurité appropriés (validation, authentification, autorisation) sur toutes les surfaces, même "cachées".

### 6.2. Anti-pattern : Trust-on-first-use

**Description :**

Faire confiance à une source après une première interaction réussie, sans revalidation.

**Exemple conceptuel :**

Un développeur stocke un token "validé une fois" et le réutilise sans vérification ultérieure.

**Pourquoi c'est dangereux :**

- Le token peut avoir été révoqué
- La source peut avoir été compromise entre-temps
- Viole le principe zero-trust

**Correction :**

Revalider à chaque utilisation. Les autorisations sont éphémères, pas permanentes.

### 6.3. Anti-pattern : Validation client-side only

**Description :**

S'appuyer uniquement sur la validation côté client.

**Exemple conceptuel :**

Un développeur valide les entrées utilisateur uniquement dans le frontend, pas dans le backend.

**Pourquoi c'est dangereux :**

- Le client peut être manipulé
- Les requêtes peuvent être forgées directement
- Aucune protection côté serveur

**Correction :**

Toujours valider côté serveur. La validation client est une commodité UX, pas une sécurité.

### 6.4. Anti-pattern : Log-and-continue

**Description :**

Logger une erreur de sécurité mais continuer l'exécution.

**Exemple conceptuel :**

Un développeur capture une exception de validation, la logue, puis continue avec des données potentiellement invalides.

**Pourquoi c'est dangereux :**

- Propagation de données corrompues
- Comportement imprévisible
- Violation du principe fail-secure

**Correction :**

Fail-fast : arrêter l'exécution et retourner une erreur explicite.

### 6.5. Anti-pattern : Singleton de sécurité

**Description :**

Centraliser tous les contrôles de sécurité dans un seul point.

**Exemple conceptuel :**

Un développeur crée un `SecurityManager` unique qui gère tous les contrôles.

**Pourquoi c'est dangereux :**

- Single point of failure
- Si contourné, plus aucune protection
- Viole la défense en profondeur

**Correction :**

Distribuer les contrôles dans chaque couche. La sécurité est une propriété distribuée.

### 6.6. Anti-pattern : Audit différé

**Description :**

Reporter l'audit à plus tard "pour la performance".

**Exemple conceptuel :**

Un développeur accumule les événements d'audit en mémoire pour les écrire en batch.

**Pourquoi c'est dangereux :**

- Perte des traces en cas de crash
- Fenêtre de non-traçabilité
- Difficulté de corrélation temporelle

**Correction :**

Émettre les événements d'audit immédiatement. L'audit asynchrone est acceptable, mais l'émission doit être immédiate.

---

## 7. Tests de sécurité

### 7.1. Types de tests requis

**Tests unitaires de sécurité :**

- Validation des entrées : schéma, format, limites
- Comportement sur entrées invalides : rejet correct
- Comportement sur erreurs : fail-secure

**Tests d'intégration de sécurité :**

- Validation de la chaîne de confiance
- Vérification des interactions entre Engines
- Test des flux de sécurité complets

**Tests de non-régression de sécurité :**

- Vérification que les corrections restent en place
- Détection des réintroductions de vulnérabilités
- Couverture des cas limites connus

### 7.2. Cas de test obligatoires

**Pour chaque composant :**

| Catégorie | Cas de test |
|-----------|-------------|
| **Validation** | Entrées valides acceptées |
| **Validation** | Entrées invalides rejetées |
| **Validation** | Entrées malformées rejetées |
| **Validation** | Entrées aux limites traitées correctement |
| **Audit** | Opérations réussies tracées |
| **Audit** | Opérations échouées tracées |
| **Erreurs** | Erreurs de validation gérées correctement |
| **Erreurs** | Comportement fail-secure vérifié |
| **Permissions** | Accès autorisé pour les ayants-droit |
| **Permissions** | Accès refusé pour les non-autorisés |

### 7.3. Tests de la chaîne de confiance

**Vérifications à effectuer :**

- Le code est conforme au MSCM
- Le MSCM est indexé dans le MIP
- Le MIP est cohérent avec le graphe
- Le graphe correspond au STA
- Le STA correspond à une OSV valide

**Test de rupture de chaîne :**

- Introduire une incohérence volontaire
- Vérifier que le système détecte la rupture
- Vérifier que les mécanismes de protection se déclenchent

### 7.4. Tests des Security Engines

**Pour chaque Engine :**

| Engine | Test principal |
|--------|----------------|
| **Integrity** | Détection de modification non autorisée |
| **Validation** | Rejet des entrées invalides |
| **Policy** | Application correcte des politiques |
| **Consensus** | Escalade sur dissensus |
| **Audit** | Présence de toutes les traces |
| **Sandbox** | Isolation effective |
| **Cognitive** | Détection de dérive simulée |
| **Recovery** | Restauration vers état sûr |

---

## 8. Check-list mentale avant toute feature

Avant d'implémenter une nouvelle fonctionnalité, un développeur DOIT vérifier mentalement :

### 8.1. Vérification des lois système

- **L1 respectée ?** : Aucun accès direct hardware ?
- **L2 respectée ?** : Une seule source de vérité ?
- **L3 respectée ?** : Passage par les Cores appropriés ?
- **L4 respectée ?** : Toutes les écritures tracées ?
- **L5 respectée ?** : Toutes les décisions validées ?
- **L6 respectée ?** : Tous les éléments indexés ?

### 8.2. Vérification de la chaîne de confiance

- **CODE → MSCM** : Le code sera-t-il balisé MSCM ?
- **MSCM → MIP** : L'index MIP sera-t-il mis à jour ?
- **MIP → GRAPH** : Le graphe sera-t-il cohérent ?
- **GRAPH → STA** : Le STA sera-t-il consulté ?
- **STA → OSV** : La version sera-t-elle certifiable ?

### 8.3. Vérification des contrôles de sécurité

- **Validation** : Toutes les entrées sont-elles validées ?
- **Politique** : Les politiques sont-elles consultées ?
- **Audit** : Les événements d'audit sont-ils émis ?
- **Intégrité** : La cohérence est-elle vérifiée ?

### 8.4. Vérification des comportements d'erreur

- **Fail-fast** : Les erreurs arrêtent-elles l'exécution ?
- **Fail-secure** : L'état de repli est-il sûr ?
- **Logging** : Les erreurs sont-elles loguées avec contexte ?
- **Notification** : Les observateurs sont-ils notifiés ?

### 8.5. Vérification de la traçabilité

- **Métadonnées complètes** : Qui, quand, quoi, pourquoi ?
- **Corrélation possible** : Le correlationId est-il propagé ?
- **Historique préservé** : Les anciennes versions sont-elles conservées ?
- **Audit immuable** : Les traces sont-elles immuables ?

---

## 9. Interactions avec les Cores — Guide pratique

### 9.1. Interaction avec StrongFather

**Nature de l'interaction :** Toute décision critique passe par StrongFather.

**Ce que le développeur doit faire :**

- Soumettre les intentions à StrongFather avant exécution
- Attendre la décision (APPROVE, REJECT, PENDING, DELEGATED)
- Respecter strictement la décision retournée
- Ne jamais contourner en cas de rejet

**Ce que le développeur ne doit JAMAIS faire :**

- Exécuter sans soumettre à StrongFather
- Ignorer un rejet
- Réessayer après un rejet sans modification

### 9.2. Interaction avec Border Guard

**Nature de l'interaction :** Border Guard classifie les sources et définit les règles.

**Ce que le développeur doit faire :**

- Consulter le niveau de confiance des sources
- Appliquer les règles de franchissement
- Traiter différemment selon le niveau (trusted, verified, unknown, hostile)

**Ce que le développeur ne doit JAMAIS faire :**

- Traiter une source sans classification
- Ignorer le niveau de confiance
- Accorder plus de privilèges que le niveau ne le permet

### 9.3. Interaction avec KindMother

**Nature de l'interaction :** KindMother gère la persistance.

**Ce que le développeur doit faire :**

- Déléguer toutes les opérations de persistance à KindMother
- Respecter le cycle de vie des données
- Utiliser les canaux de synchronisation fournis

**Ce que le développeur ne doit JAMAIS faire :**

- Accéder directement à la base de données
- Écrire des fichiers sans passer par KindMother
- Maintenir des caches non synchronisés

### 9.4. Interaction avec Caring Nanny

**Nature de l'interaction :** Caring Nanny observe l'état du système.

**Ce que le développeur doit faire :**

- Émettre les signaux d'état (healthy, degraded, error)
- Notifier les changements d'état significatifs
- Consulter l'état global si nécessaire

**Ce que le développeur ne doit JAMAIS faire :**

- Masquer un état dégradé
- Ignorer les alertes de Caring Nanny
- Modifier l'état global directement

---

## 10. Conformité MSCM/MIP

### 10.1 Obligation de balisage MSCM

Tout code implémenté DOIT être balisé selon le protocole MSCM v1.

**Référence :** [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md)

**Obligations minimales :**
- Chaque bloc fonctionnel DOIT avoir un identifiant unique (`@id`)
- Chaque bloc DOIT avoir une description fonctionnelle (`@do`)

**Méta-données optionnelles :**
- Le rôle sémantique peut être explicite (`@role`) — optionnel
- La couche architecturale peut être déclarée (`@layer`) — optionnel
- Une description humaine peut accompagner le bloc (`@human` ou `@humain`) — optionnel

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

## 11. Conclusion

Ce document fournit des lignes directrices pour implémenter la sécurité de manière conforme aux contrats FONDATION.

**Points clés :**

- La **sécurité est structurelle** : elle n'est pas ajoutée, elle émerge de l'architecture
- Les **6 lois système** (L1-L6) sont non négociables
- La **chaîne de confiance** (CODE → MSCM → MIP → GRAPH → STA → OSV) doit être intacte
- Les **8 Security Engines** forment une strate obligatoire
- La **traçabilité est absolue** : rien n'existe sans trace
- Le **fail-secure** est le comportement par défaut

**Nature informative :**

Ce document est purement informatif et ne crée aucune nouvelle obligation contractuelle. Il sert uniquement à guider la compréhension et l'application des principes de sécurité.

**Rappel :** Les contrats FONDATION priment toujours sur ce guide. En cas de doute, se référer à la [Doctrine Securite Fondamentale](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md) et aux contrats spécifiques.

**Phrase fondatrice à garder en mémoire :**

> **"La sécurité dans Miyukini n'est pas un module, ni une fonctionnalité, ni un service. Elle est une propriété structurelle du système."**

---

**Document créé le :** 2026-01-28  
**Version :** 1.0  
**Statut :** POST-FONDATION — Informatif, non normatif, non contractuel  
**Référence :** Miyukini Core System v2.4, [Doctrine Securite Fondamentale](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md), [Security - Documentation Fondatrice](../foundation/Security%20-%20Documentation%20Fondatrice.md)  
**Type :** Guide d'implémentation informatif

---

## 12. Mini log — erreurs / warnings / arbitrages rencontrés

### Arbitrage A1 : Niveau d'abstraction des patterns

**Arbitrage rencontré :** Quel niveau de détail technique donner aux patterns d'implémentation ?

**Décision prise :** Patterns conceptuels sans code, avec description du flux et des points clés.

**Justification :** Ce document est informatif et non normatif. Les choix techniques (langage, framework) appartiennent aux équipes d'implémentation.

**Documentation :** Section 4 avec patterns conceptuels illustrés par des diagrammes textuels.

### Arbitrage A2 : Organisation des anti-patterns

**Arbitrage rencontré :** Fournir des anti-patterns spécifiques à Miyukini ou des anti-patterns généraux de sécurité ?

**Décision prise :** Anti-patterns généraux mais contextualisés dans le cadre Miyukini (lois système, chaîne de confiance).

**Justification :** Les anti-patterns généraux restent pertinents, mais leur connexion aux concepts Miyukini (L1-L6, STA/OSV) les rend actionnables.

**Documentation :** Section 6 avec 6 anti-patterns généraux contextualisés.

### Arbitrage A3 : Check-list exhaustive vs pratique

**Arbitrage rencontré :** La check-list est-elle trop longue pour une utilisation quotidienne ?

**Décision prise :** Conserver la liste complète organisée en catégories (lois, chaîne, contrôles, erreurs, traçabilité).

**Justification :** Chaque vérification est essentielle. L'organisation en catégories permet une utilisation partielle selon le contexte.

**Documentation :** Section 8 avec check-list organisée en 5 sous-sections.

### Arbitrage A4 : Tests de sécurité — profondeur

**Arbitrage rencontré :** Jusqu'où détailler les tests de sécurité requis ?

**Décision prise :** Types de tests, cas obligatoires par catégorie, et vérifications de la chaîne de confiance.

**Justification :** Suffisamment détaillé pour guider, pas assez pour prescrire une méthodologie de test spécifique.

**Documentation :** Section 7 avec tableaux de cas de test et vérifications.

### Arbitrage A5 : Intégration avec les Cores

**Arbitrage rencontré :** Répéter les contrats d'intégration ou simplement y faire référence ?

**Décision prise :** Guide pratique simplifié avec "ce qu'il faut faire" et "ce qu'il ne faut jamais faire", avec renvoi aux contrats pour les détails.

**Justification :** Évite la duplication tout en fournissant une guidance actionnable immédiate.

**Documentation :** Section 9 avec guide pratique pour 4 Cores principaux (StrongFather, Border Guard, KindMother, Caring Nanny).

---

*Aucune autre erreur, warning, ou arbitrage rencontré lors de la rédaction de ce document.*
