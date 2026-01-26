# Miyukini Core System — Adaptateur Produit Documentation Conceptuelle

## 1. Introduction

### Contexte

Dans l'architecture Miyukini Core System v2.4, l'adaptateur produit est un composant architectural fondamental qui assure la médiation entre trois couches distinctes : les produits, les modules SPM CMS, et KindMother. Cette documentation définit formellement le concept d'adaptateur produit au niveau purement conceptuel et architectural, sans aucun détail d'implémentation technique.

**Pattern architectural :** Cette architecture suit le pattern **"Authoritative Core with Product Adapters"** (Cœur Autoritaire avec Adaptateurs Produits). Ce pattern établit que KindMother est le cœur autoritaire du système, responsable de la validation, de la cohérence et de l'autorité des données, tandis que les adaptateurs produits assurent la traduction et l'isolation entre les couches fonctionnelles et le cœur autoritaire.

### Objectif de cette documentation

Cette documentation établit le contrat conceptuel de l'adaptateur produit. Elle définit :
- Ce qu'est un adaptateur produit
- Ce qu'il doit faire (responsabilités)
- Ce qu'il ne doit jamais faire (interdictions)
- Son positionnement exact dans l'architecture
- Pourquoi chaque produit nécessite ses propres adaptateurs
- Les typologies d'adaptateurs selon les profils d'usage

Cette documentation est contractuelle et sert de référence pour l'architecture du système. Elle ne contient aucun détail d'implémentation, aucun code, aucune hypothèse technique.

---

## 2. Définition formelle

### Qu'est-ce qu'un adaptateur produit ?

Un **adaptateur produit** est un composant architectural qui assure la traduction bidirectionnelle entre le domaine fonctionnel des modules SPM CMS et le domaine d'autorité des données de KindMother. Il est l'unique point d'entrée autorisé pour toute interaction avec KindMother depuis un produit.

### Caractéristiques fondamentales

**Médiateur :** L'adaptateur produit est un médiateur qui isole complètement les modules SPM de KindMother. Les modules SPM ne connaissent pas KindMother et ne peuvent pas l'appeler directement.

**Traducteur :** L'adaptateur traduit les opérations fonctionnelles des modules SPM (créer un contenu, lire une hiérarchie, etc.) en opérations conceptuelles de KindMother (read, submitWriteIntent, sync, etc.).

**Implémenteur de traits :** L'adaptateur produit implémente les traits fonctionnels exposés par les modules SPM (ContentManager, HierarchyManager, MediaManager, etc.). Le produit utilise ces traits via les adaptateurs, sans connaître les détails de persistance.

**Fournisseur de contexte :** L'adaptateur produit est responsable de construire et fournir le contexte complet nécessaire à KindMother pour chaque opération (utilisateur, autorisations, instance, exécution).

**Isolateur :** L'adaptateur produit isole le produit de KindMother. Le produit n'appelle jamais directement KindMother et ne connaît pas son existence.

### Positionnement dans l'architecture

L'adaptateur produit se situe à l'intersection de trois couches :

- **Couche Produit :** Le produit utilise les modules SPM via les adaptateurs, sans connaître KindMother
- **Couche Modules SPM :** Les modules SPM exposent des traits fonctionnels que les adaptateurs implémentent
- **Couche KindMother :** KindMother est appelé uniquement par les adaptateurs, jamais par les produits ni les modules SPM

L'adaptateur produit est donc le seul composant qui connaît à la fois les contrats des modules SPM et l'interface de KindMother.

---

## 3. Responsabilités (ce qu'il DOIT faire)

Un adaptateur produit a des responsabilités strictes et non négociables. Ces responsabilités garantissent l'intégrité architecturale du système.

### 3.1. Traduction bidirectionnelle entre modules SPM et KindMother

**Responsabilité principale :** L'adaptateur produit DOIT traduire toutes les opérations entre le domaine fonctionnel des modules SPM et le domaine d'autorité des données de KindMother.

Cette traduction respecte **LOI-1** (aucune dépendance externe critique à l'exécution) : toutes les opérations fonctionnent localement sans appel externe obligatoire. En cas d'isolement, l'adaptateur continue de fonctionner avec l'état local disponible (**LOI-2** : le système accepte l'isolement comme état normal, voir [Lois d'Autonomie Système](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md)).

**Direction SPM → KindMother :**
- Recevoir une demande d'opération fonctionnelle depuis un module SPM (ex. `create_content`)
- Identifier l'opération KindMother correspondante (ex. `submitWriteIntent`)
- Traduire les types et structures du module SPM vers les formats attendus par KindMother
- Construire les paramètres de l'opération KindMother à partir des données du module SPM

**Direction KindMother → SPM :**
- Recevoir le résultat d'une opération KindMother
- Traduire le résultat vers les types et structures du module SPM
- Gérer les erreurs KindMother et les convertir en erreurs SPM appropriées
- Retourner le résultat au module SPM dans le format attendu

**Garantie :** Toute opération SPM qui nécessite de la persistance DOIT être traduite en opération KindMother. Aucune opération ne peut contourner cette traduction.

### 3.2. Fourniture du contexte complet à KindMother

**Responsabilité :** L'adaptateur produit DOIT fournir un contexte complet et cohérent à KindMother pour chaque opération.

**Contexte utilisateur :**
- Identifier l'utilisateur effectuant l'opération (fourni par le produit)
- Extraire les métadonnées utilisateur nécessaires (rôles, organisation, etc.)
- Construire le contexte utilisateur selon le format attendu par KindMother

**Contexte d'autorisation :**
- Récupérer les règles de permissions définies par le produit
- Construire le contexte d'autorisation avec les règles, rôles, et permissions
- Fournir le contexte métier nécessaire aux règles (organisation, projet, ressource parente, etc.)

**Contexte d'instance :**
- Identifier l'instance de base de données à utiliser (DB Mère ou DB Fille)
- Déterminer l'instance selon la configuration du produit et le mode d'exécution
- Fournir l'identité d'instance à KindMother

**Contexte d'exécution :**
- Détecter ou déterminer le mode d'exécution (online/offline)
- Fournir les informations nécessaires sur l'état de synchronisation si applicable

**Garantie :** Aucune opération KindMother ne peut être exécutée sans contexte complet. L'adaptateur est responsable de la complétude du contexte.

### 3.3. Implémentation des traits SPM

**Responsabilité :** L'adaptateur produit DOIT implémenter tous les traits SPM CMS utilisés par le produit.

**Traits à implémenter :**
- `ContentManager` pour le module Content
- `HierarchyManager` pour le module Hierarchy
- `TaxonomiesManager` pour le module Taxonomies
- `MediaManager` pour le module Media
- `PublicationManager` pour le module Publication
- `SearchManager` pour le module Search
- Et tout autre trait SPM utilisé par le produit

**Contrat d'implémentation :**
- Respecter strictement la signature et le comportement documenté de chaque trait
- Retourner les types et erreurs définis par le module SPM
- Respecter les invariants et contraintes documentés
- Gérer tous les cas d'erreur possibles selon le contrat du trait

**Garantie :** L'implémentation DOIT être conforme au contrat fonctionnel du trait. Aucune déviation n'est autorisée.

### 3.4. Gestion des erreurs et résultats

**Responsabilité :** L'adaptateur produit DOIT gérer correctement tous les résultats et erreurs possibles.

**Gestion des résultats :**
- Interpréter les résultats KindMother (succès, échec définitif, échec temporaire, état transitoire)
- Traduire les résultats vers les types de retour attendus par les modules SPM
- Gérer les cas de résultats vides (non-erreurs) selon le contrat SPM

**Gestion des erreurs :**
- Intercepter toutes les erreurs KindMother
- Traduire les erreurs KindMother en erreurs SPM appropriées
- Préserver les informations de contexte nécessaires pour le diagnostic
- Gérer les erreurs de permission, de cohérence, de contexte, et fonctionnelles

**Gestion des états transitoires :**
- Interpréter les états transitoires KindMother (opération en attente, synchronisation partielle, etc.)
- Traduire ces états vers des résultats SPM appropriés
- Informer le produit des états nécessitant une action ultérieure

**Garantie :** Aucune erreur KindMother ne doit être exposée directement au module SPM ou au produit. Toutes les erreurs doivent être traduites selon le contrat SPM.

### 3.5. Isolation des modules SPM vis-à-vis de KindMother

**Responsabilité :** L'adaptateur produit DOIT garantir l'isolation complète des modules SPM vis-à-vis de KindMother.

**Isolation conceptuelle :**
- Les modules SPM ne doivent jamais connaître l'existence de KindMother
- Les modules SPM ne doivent jamais recevoir de types, structures, ou erreurs provenant directement de KindMother
- Les modules SPM ne doivent jamais être exposés aux concepts de KindMother (WriteIntent, DB Mère/Fille, synchronisation, etc.)

**Isolation technique :**
- Aucune dépendance directe ou indirecte des modules SPM vers KindMother
- Aucune référence à KindMother dans les types ou structures exposés par les modules SPM
- Aucune fuite de détails d'implémentation KindMother vers les modules SPM

**Garantie :** L'isolation DOIT être totale. Aucune exception n'est autorisée. Les modules SPM doivent pouvoir fonctionner sans connaître KindMother.

---

## 4. Interdictions (ce qu'il ne doit JAMAIS faire)

Un adaptateur produit est soumis à des interdictions strictes qui garantissent l'intégrité architecturale. Ces interdictions sont absolues et aucune exception n'est autorisée.

### 4.1. Contourner KindMother pour accéder directement à la persistance

**Interdiction absolue :** L'adaptateur produit ne doit JAMAIS contourner KindMother pour accéder directement à la persistance.

**Interdictions spécifiques :**
- Accéder directement à SQLite ou à tout autre moteur de persistance
- Exécuter des requêtes SQL ou toute autre requête de persistance
- Lire ou écrire directement dans des fichiers de base de données
- Accéder aux schémas, structures, ou métadonnées de persistance
- Utiliser des bibliothèques de persistance qui contournent KindMother

**Raison :** L'abstraction complète de la persistance est un principe fondamental. Toute violation compromet l'évolution future de KindMother et crée des dépendances indésirables.

**Conséquence :** Toute tentative d'accès direct à la persistance constitue une violation architecturale majeure et compromet l'intégrité du système.

### 4.2. Exposer KindMother directement au produit

**Interdiction :** L'adaptateur produit ne doit JAMAIS exposer KindMother directement au produit.

**Interdictions spécifiques :**
- Exposer l'interface KindMother au produit
- Permettre au produit d'appeler directement KindMother
- Retourner des types ou structures provenant directement de KindMother au produit
- Exposer des concepts KindMother (WriteIntent, DB Mère/Fille, synchronisation) au produit
- Créer une dépendance du produit vers KindMother

**Raison :** Le produit ne doit pas connaître KindMother. L'isolation garantit que le produit peut évoluer indépendamment de KindMother et que KindMother peut évoluer sans impact sur le produit.

**Conséquence :** Toute exposition de KindMother au produit constitue une violation architecturale et compromet l'isolation des couches.

### 4.3. Modifier les règles de permissions pour une opération

**Interdiction :** L'adaptateur produit ne doit JAMAIS modifier temporairement ou localement les règles de permissions pour une opération spécifique.

**Interdictions spécifiques :**
- Modifier les règles de permissions fournies par le produit pour une opération
- Contourner les règles de permissions en fournissant un contexte d'autorisation différent
- Forcer une opération en modifiant les règles de permissions
- Créer des règles de permissions spécifiques à l'adaptateur

**Raison :** Les règles de permissions sont définies par le produit et appliquées de manière cohérente par KindMother. Toute modification compromet la sécurité, la traçabilité, et la cohérence.

**Conséquence :** Toute modification des règles de permissions constitue une violation de sécurité et compromet l'intégrité du système.

### 4.4. Bypasser les validations de KindMother

**Interdiction :** L'adaptateur produit ne doit JAMAIS tenter de bypasser les validations de KindMother.

**Interdictions spécifiques :**
- Demander à KindMother d'exécuter une opération en mode "bypass" ou "force"
- Contourner les validations de permissions en modifiant le contexte
- Contourner les validations de cohérence en modifiant les données
- Utiliser des opérations non documentées pour contourner les validations
- Forcer une écriture sans WriteIntent

**Raison :** Les validations de KindMother (permissions, cohérence, contexte) sont essentielles à l'intégrité du système. Aucun contournement n'est possible.

**Conséquence :** Toute tentative de bypass constitue une violation architecturale et compromet l'intégrité des données.

### 4.5. Dépendre des détails d'implémentation de KindMother

**Interdiction :** L'adaptateur produit ne doit JAMAIS faire d'hypothèses sur les détails d'implémentation de KindMother.

**Interdictions spécifiques :**
- Faire des hypothèses sur la structure interne de KindMother
- Dépendre de mécanismes techniques non documentés
- Utiliser des optimisations basées sur des détails d'implémentation
- Faire des hypothèses sur SQLite ou tout autre moteur de persistance
- Dépendre de l'ordre d'exécution interne de KindMother

**Raison :** L'adaptateur doit dépendre uniquement du contrat conceptuel de KindMother (CoreDataAPI). Toute dépendance aux détails d'implémentation compromet l'évolution future de KindMother.

**Conséquence :** Toute dépendance aux détails d'implémentation crée des risques de rupture lors de l'évolution de KindMother.

### 4.6. Exposer des erreurs KindMother directement

**Interdiction :** L'adaptateur produit ne doit JAMAIS exposer des erreurs KindMother directement au module SPM ou au produit.

**Interdictions spécifiques :**
- Retourner des erreurs KindMother sans traduction
- Exposer des types d'erreur KindMother au module SPM
- Exposer des messages d'erreur contenant des détails internes de KindMother
- Créer des dépendances du module SPM vers les types d'erreur KindMother

**Raison :** Les modules SPM et le produit ne doivent pas connaître KindMother. Les erreurs doivent être traduites selon le contrat SPM.

**Conséquence :** Toute exposition directe d'erreurs KindMother compromet l'isolation des couches.

### 4.7. Prendre des décisions temporelles

**Interdiction :** L'adaptateur produit ne doit JAMAIS prendre de décision temporelle.

**Interdictions spécifiques :**
- Décider quand synchroniser les données
- Décider dans quel ordre appliquer les opérations
- Implémenter des mécanismes de retry maison
- Créer des stratégies de synchronisation "intelligente" côté produit
- Utiliser des heuristiques temporelles pour déterminer le moment des opérations
- Déterminer des délais ou des intervalles de synchronisation

**Raison :** Toute décision temporelle appartient exclusivement à KindMother. L'adaptateur produit ne doit être qu'un traducteur passif qui transmet les opérations sans influencer leur timing ou leur ordre d'exécution. Toute décision temporelle prise par l'adaptateur compromet la cohérence et l'autorité de KindMother sur la gestion des données.

**Conséquence :** Toute décision temporelle prise par l'adaptateur constitue une violation architecturale et compromet l'intégrité du système.

---

## 5. Positionnement architectural exact

### 5.1. Position entre les couches

L'adaptateur produit se situe à l'intersection de trois couches architecturales. Cette position garantit que le système respecte les [Lois d'Autonomie Système](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md) : l'adaptateur fonctionne localement (**LOI-1**), accepte l'isolement comme état normal (**LOI-2**), et considère l'état local comme souverain (**LOI-3**).

```
┌─────────────────────────────────────────┐
│           PRODUIT                        │
│  (logique métier, API, UI)               │
└─────────────────────────────────────────┘
           │
           │ utilise
           ▼
┌─────────────────────────────────────────┐
│      MODULES SPM CMS                      │
│  (traits fonctionnels purs)              │
│  - ContentManager                        │
│  - HierarchyManager                      │
│  - MediaManager                          │
│  - etc.                                  │
└─────────────────────────────────────────┘
           │
           │ implémenté par
           ▼
┌─────────────────────────────────────────┐
│      ADAPTATEUR PRODUIT                  │
│  (traduction SPM ↔ KindMother)          │
│  - Implémente les traits SPM            │
│  - Appelle KindMother                    │
│  - Fournit le contexte                   │
└─────────────────────────────────────────┘
           │
           │ appelle (uniquement)
           ▼
┌─────────────────────────────────────────┐
│         KINDMOTHER                        │
│  (moteur de données interne)             │
│  - CoreDataAPI                           │
│  - Persistance (SQLite interne)        │
│  - Synchronisation                       │
└─────────────────────────────────────────┘
           │
           │ utilise
           ▼
┌─────────────────────────────────────────┐
│           KERNEL                         │
│  (Id, Clock, Logger)                    │
└─────────────────────────────────────────┘
```

### 5.2. Flux de données et d'appels

**Flux de lecture :**

```
1. Produit → Module SPM : demande de lecture (ex. get_content)
2. Module SPM → Adaptateur : appel du trait implémenté
3. Adaptateur → KindMother : traduction en opération read avec contexte
4. KindMother → Adaptateur : résultat (données ou erreur)
5. Adaptateur → Module SPM : traduction du résultat en types SPM
6. Module SPM → Produit : retour du résultat
```

**Flux d'écriture :**

```
1. Produit → Module SPM : demande d'écriture (ex. create_content)
2. Module SPM → Adaptateur : appel du trait implémenté
3. Adaptateur → KindMother : traduction en submitWriteIntent avec contexte
4. KindMother → Adaptateur : résultat (succès ou erreur)
5. Adaptateur → Module SPM : traduction du résultat en types SPM
6. Module SPM → Produit : retour du résultat
```

**Flux de synchronisation :**

```
1. Produit → Adaptateur : demande de synchronisation (optionnel, selon produit)
2. Adaptateur → KindMother : traduction en opération sync ou requestSync
3. KindMother → Adaptateur : rapport de synchronisation
4. Adaptateur → Produit : traduction du rapport (si nécessaire)
```

### 5.3. Isolation des couches

**Isolation Produit ↔ KindMother :**
- Le produit ne connaît pas KindMother
- Le produit n'appelle jamais directement KindMother
- Aucune dépendance du produit vers KindMother
- L'isolation est garantie par l'adaptateur produit

**Isolation Modules SPM ↔ KindMother :**
- Les modules SPM ne connaissent pas KindMother
- Les modules SPM n'appellent jamais directement KindMother
- Aucune dépendance des modules SPM vers KindMother
- L'isolation est garantie par l'adaptateur produit

**Isolation Adaptateur ↔ Produit :**
- L'adaptateur n'expose pas KindMother au produit
- L'adaptateur traduit toutes les interactions
- Le produit utilise uniquement les traits SPM

### 5.4. Respect des dépendances unidirectionnelles

**Règle fondamentale :** Les dépendances sont strictement unidirectionnelles.

**Direction autorisée :**
- Produit → Modules SPM (utilisation des traits)
- Modules SPM → Adaptateur (appel des traits implémentés)
- Adaptateur → KindMother (appel de la CoreDataAPI)
- KindMother → Kernel (utilisation des capacités)

**Direction interdite :**
- KindMother → Adaptateur (pas de callback, pas de dépendance inverse)
- Adaptateur → Modules SPM (les modules SPM ne dépendent pas de l'adaptateur)
- Modules SPM → Produit (les modules SPM ne dépendent pas du produit)
- Kernel → KindMother (le kernel ne dépend pas de KindMother)

**Garantie :** L'adaptateur produit respecte strictement ces dépendances unidirectionnelles. Aucune dépendance inverse n'est autorisée.

---

## 6. Justification : pourquoi chaque produit a ses propres adaptateurs

Chaque produit nécessite ses propres adaptateurs, même s'il utilise les mêmes modules SPM et le même KindMother. Cette nécessité découle de plusieurs facteurs fondamentaux.

### 6.1. Spécificités métier du produit

**Contexte :** Chaque produit a des besoins métier spécifiques qui influencent la traduction entre les types SPM et les structures de données KindMother.

**Exemples :**
- Un CMS de blog peut avoir des types de contenu spécifiques (article, page, commentaire) qui nécessitent une traduction particulière
- Un jeu peut avoir des entités spécifiques (niveau, objet, quête) qui nécessitent une traduction différente
- Un SaaS multi-tenant peut avoir des besoins de séparation des données qui influencent la traduction

**Impact sur l'adaptateur :** L'adaptateur doit traduire les types métier du produit vers les structures génériques de KindMother. Cette traduction est spécifique à chaque produit.

### 6.2. Règles de permissions spécifiques

**Contexte :** Chaque produit définit ses propres règles de permissions conceptuelles selon ses besoins métier.

**Exemples :**
- Un CMS peut avoir des règles basées sur les rôles (éditeur, administrateur, visiteur)
- Un jeu peut avoir des règles basées sur la propriété (propriétaire du personnage, membre de la guilde)
- Un SaaS peut avoir des règles basées sur le tenant (isolation des données par client)

**Impact sur l'adaptateur :** L'adaptateur doit construire le contexte d'autorisation selon les règles spécifiques du produit. Ces règles sont différentes pour chaque produit.

### 6.3. Contexte d'autorisation unique

**Contexte :** Chaque produit a un système d'authentification et d'autorisation unique qui détermine comment extraire et construire le contexte d'autorisation.

**Exemples :**
- Un produit peut utiliser OAuth avec des tokens JWT
- Un autre produit peut utiliser des sessions avec des cookies
- Un autre produit peut utiliser une authentification par clé API

**Impact sur l'adaptateur :** L'adaptateur doit extraire le contexte utilisateur et d'autorisation depuis le système d'authentification spécifique du produit. Cette extraction est unique à chaque produit.

### 6.4. Configuration d'instance particulière

**Contexte :** Chaque produit peut avoir une configuration d'instance différente (DB Mère unique, plusieurs DB Filles, mode offline-first, etc.).

**Exemples :**
- Un CMS web utilise uniquement la DB Mère
- Une application mobile utilise une DB Fille avec synchronisation périodique
- Un jeu solo utilise une DB Fille sans synchronisation

**Impact sur l'adaptateur :** L'adaptateur doit déterminer quelle instance utiliser selon la configuration spécifique du produit. Cette détermination est unique à chaque produit.

### 6.5. Traduction des types produits vers types KindMother

**Contexte :** Chaque produit peut avoir des types de données spécifiques qui nécessitent une traduction particulière vers les structures génériques de KindMother.

**Exemples :**
- Un produit peut avoir des métadonnées structurées (JSON) qui doivent être sérialisées différemment
- Un autre produit peut avoir des relations complexes qui nécessitent une traduction spécifique
- Un autre produit peut avoir des types de contenu avec des champs spécifiques

**Impact sur l'adaptateur :** L'adaptateur doit traduire les types spécifiques du produit vers les formats attendus par KindMother. Cette traduction est spécifique à chaque produit.

### 6.6. Gestion des erreurs spécifique au produit

**Contexte :** Chaque produit peut avoir des besoins spécifiques pour la gestion des erreurs et leur présentation à l'utilisateur.

**Exemples :**
- Un produit peut vouloir masquer certains détails d'erreur à l'utilisateur
- Un autre produit peut vouloir enrichir les erreurs avec des informations métier
- Un autre produit peut vouloir logger les erreurs différemment

**Impact sur l'adaptateur :** L'adaptateur doit gérer les erreurs selon les besoins spécifiques du produit. Cette gestion est unique à chaque produit.

### 6.7. Conclusion

Chaque produit nécessite ses propres adaptateurs car :
- Les besoins métier sont spécifiques
- Les règles de permissions sont spécifiques
- Le contexte d'autorisation est unique
- La configuration d'instance est particulière
- La traduction des types est spécifique
- La gestion des erreurs est personnalisée

L'adaptateur produit est donc un composant spécifique au produit, même s'il utilise les mêmes modules SPM et le même KindMother. Cette spécificité garantit que chaque produit peut évoluer indépendamment tout en bénéficiant des capacités communes des modules SPM et de KindMother.

---

## 7. Typologie d'adaptateurs produits

Selon le profil d'usage du produit, les adaptateurs produits peuvent avoir des caractéristiques différentes. Cette section propose une typologie conceptuelle des adaptateurs selon les profils d'usage identifiés.

### 7.1. Adaptateur CMS (site web, blog)

**Contexte d'usage :** Site web ou CMS qui fonctionne principalement en ligne, avec accès via KindMother en mode DB Mère.

**Caractéristiques :**
- Utilise uniquement la DB Mère (pas de DB Fille)
- Mode online uniquement (pas de mode offline)
- Synchronisation en temps réel (pas de délai)
- Autorité finale pour toutes les écritures
- Règles de permissions basées sur les rôles (éditeur, administrateur, visiteur)
- Types de contenu spécifiques au CMS (article, page, commentaire, etc.)

**Responsabilités spécifiques :**
- Construire le contexte d'instance pour la DB Mère uniquement
- Gérer les règles de permissions basées sur les rôles
- Traduire les types de contenu CMS vers les structures KindMother
- Gérer les erreurs avec des messages adaptés au contexte web

**Exemples de produits :** CMS web classique, site e-commerce, blog, site de documentation.

### 7.2. Adaptateur Jeu Solo

**Contexte d'usage :** Jeu vidéo solo qui fonctionne entièrement en local, sans synchronisation avec un serveur.

**Caractéristiques :**
- Utilise une DB Fille locale uniquement
- Pas de synchronisation avec une DB Mère
- Fonctionne entièrement offline
- Pas de partage de données entre instances
- Règles de permissions simplifiées (propriétaire local)
- Types d'entités spécifiques au jeu (niveau, objet, quête, personnage, etc.)

**Responsabilités spécifiques :**
- Construire le contexte d'instance pour la DB Fille locale uniquement
- Gérer les règles de permissions simplifiées (propriétaire local)
- Traduire les types d'entités jeu vers les structures KindMother
- Gérer les erreurs avec des messages adaptés au contexte jeu

**Exemples de produits :** Jeu solo avec sauvegarde locale, simulateur local, application de création solo.

### 7.3. Adaptateur Jeu Multijoueur Asynchrone

**Contexte d'usage :** Jeu multijoueur asynchrone où les joueurs interagissent de manière décalée dans le temps.

**Caractéristiques :**
- Utilise une DB Fille par joueur avec synchronisation périodique
- Synchronisation avec la DB Mère (état global du jeu)
- Fonctionne en mode offline-first (chaque joueur a sa copie locale)
- Résolution de conflits lors de la synchronisation
- Règles de permissions basées sur la propriété (propriétaire du personnage, membre de la guilde)
- Types d'entités spécifiques au jeu (personnage, guilde, ressource, etc.)

**Responsabilités spécifiques :**
- Construire le contexte d'instance pour la DB Fille du joueur
- Gérer les règles de permissions basées sur la propriété
- Traduire les types d'entités jeu vers les structures KindMother
- Gérer les conflits de synchronisation avec des règles spécifiques au jeu
- Gérer les erreurs avec des messages adaptés au contexte jeu multijoueur

**Exemples de produits :** Jeu de stratégie asynchrone, jeu de gestion multijoueur, application collaborative avec sync périodique.

### 7.4. Adaptateur Application Locale (offline-first)

**Contexte d'usage :** Application desktop ou mobile qui fonctionne principalement en local, avec synchronisation occasionnelle.

**Caractéristiques :**
- Utilise une DB Fille locale avec synchronisation périodique ou manuelle
- Fonctionne en mode offline-first (toutes les opérations sont locales)
- Synchronisation avec la DB Mère selon la stratégie du produit
- Règles de permissions basées sur l'utilisateur local
- Types d'entités spécifiques à l'application (document, note, tâche, etc.)

**Responsabilités spécifiques :**
- Construire le contexte d'instance pour la DB Fille locale
- Gérer les règles de permissions basées sur l'utilisateur local
- Traduire les types d'entités application vers les structures KindMother
- Gérer la synchronisation avec des stratégies spécifiques à l'application
- Gérer les erreurs avec des messages adaptés au contexte application locale

**Exemples de produits :** Application de prise de notes, gestionnaire de tâches local, éditeur de documents offline.

### 7.5. Adaptateur SaaS (multi-tenant)

**Contexte d'usage :** Application SaaS qui gère plusieurs clients (tenants) avec isolation des données.

**Caractéristiques :**
- Utilise la DB Mère avec isolation des données par tenant
- Mode online uniquement (pas de mode offline)
- Règles de permissions basées sur le tenant (isolation des données)
- Types d'entités spécifiques au SaaS (client, projet, utilisateur, etc.)
- Gestion de l'isolation des données entre tenants

**Responsabilités spécifiques :**
- Construire le contexte d'instance pour la DB Mère avec identification du tenant
- Gérer les règles de permissions basées sur le tenant (isolation stricte)
- Traduire les types d'entités SaaS vers les structures KindMother
- Garantir l'isolation des données entre tenants dans toutes les opérations
- Gérer les erreurs avec des messages adaptés au contexte SaaS

**Exemples de produits :** Plateforme SaaS multi-tenant, application de gestion d'entreprise, service cloud.

### 7.6. Caractéristiques communes

Tous les adaptateurs produits, quelle que soit leur typologie, partagent des caractéristiques communes :

- **Traduction bidirectionnelle :** Tous traduisent entre modules SPM et KindMother
- **Fourniture de contexte :** Tous fournissent le contexte complet à KindMother
- **Implémentation de traits :** Tous implémentent les traits SPM utilisés
- **Gestion des erreurs :** Tous gèrent les erreurs et les traduisent
- **Isolation :** Tous isolent les modules SPM de KindMother

Les différences entre typologies concernent principalement :
- La configuration d'instance (DB Mère, DB Fille, ou les deux)
- Les règles de permissions spécifiques
- Les types d'entités à traduire
- Les stratégies de synchronisation
- La gestion des erreurs selon le contexte

---

## 8. Schéma conceptuel ASCII

### 8.1. Vue d'ensemble de l'architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         PRODUIT                                  │
│  (logique métier, API HTTP, interface utilisateur)              │
│                                                                   │
│  Exemple :                                                        │
│  - Handler API : POST /articles                                   │
│  - Service métier : validation, enrichissement                    │
│  - Logique spécifique au produit                                 │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │ utilise les traits SPM
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    MODULES SPM CMS                               │
│  (traits fonctionnels purs, sans persistance)                   │
│                                                                   │
│  Traits exposés :                                                 │
│  - ContentManager : create_content, get_content, etc.            │
│  - HierarchyManager : create_node, get_children, etc.            │
│  - MediaManager : upload_media, get_media, etc.                   │
│  - TaxonomiesManager : create_term, get_terms, etc.               │
│  - PublicationManager : publish, unpublish, etc.                  │
│  - SearchManager : search, index, etc.                            │
│                                                                   │
│  Règle : Aucune référence à KindMother, aucune notion de DB      │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │ implémenté par
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                   ADAPTATEUR PRODUIT                             │
│  (traduction SPM ↔ KindMother, fourniture de contexte)          │
│                                                                   │
│  Implémentations :                                                │
│  - ContentAdapter : implémente ContentManager                    │
│  - HierarchyAdapter : implémente HierarchyManager                 │
│  - MediaAdapter : implémente MediaManager                        │
│  - TaxonomiesAdapter : implémente TaxonomiesManager              │
│  - PublicationAdapter : implémente PublicationManager             │
│  - SearchAdapter : implémente SearchManager                      │
│                                                                   │
│  Responsabilités :                                                │
│  1. Recevoir les appels des traits SPM                            │
│  2. Traduire en opérations KindMother                            │
│  3. Construire le contexte (utilisateur, autorisations, instance) │
│  4. Appeler KindMother via CoreDataAPI                           │
│  5. Traduire les résultats KindMother en types SPM               │
│  6. Retourner les résultats aux modules SPM                      │
│                                                                   │
│  Règle : Seul composant autorisé à appeler KindMother            │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │ appelle (uniquement)
                            │ CoreDataAPI
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                      KINDMOTHER                                   │
│  (moteur de données interne, abstraction complète)               │
│                                                                   │
│  Interface : CoreDataAPI (10 opérations conceptuelles)         │
│  - read : lecture d'entité                                        │
│  - list : liste d'entités avec filtres                           │
│  - query : requête complexe                                       │
│  - submitWriteIntent : écriture unique                           │
│  - submitBatchWriteIntent : écriture atomique multiple          │
│  - sync : synchronisation immédiate                              │
│  - requestSync : synchronisation différée                        │
│  - getStatus : état de l'instance                                │
│  - getSyncState : état de synchronisation                        │
│  - getPendingWriteIntents : WriteIntent en attente              │
│                                                                   │
│  Persistance : SQLite interne (jamais exposé)                     │
│  Synchronisation : DB Mère ↔ DB Filles                            │
│  Permissions : règles conceptuelles (fournies par le produit)    │
│                                                                   │
│  Règle : Aucun accès direct à la persistance depuis l'extérieur │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │ utilise
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                          KERNEL                                   │
│  (capacités fondamentales)                                        │
│                                                                   │
│  - Id / IdGenerator : génération d'identifiants uniques         │
│  - Clock : horodatage, gestion du temps                          │
│  - Logger : logging structuré                                    │
└─────────────────────────────────────────────────────────────────┘
```

### 8.2. Flux de lecture détaillé

```
┌──────────┐
│ PRODUIT  │
└────┬─────┘
     │ 1. get_content(id)
     ▼
┌─────────────────────┐
│ MODULE SPM CONTENT  │
│ ContentManager      │
└────┬────────────────┘
     │ 2. get_content(id)
     ▼
┌─────────────────────┐
│ ADAPTATEUR PRODUIT  │
│ ContentAdapter      │
│                     │
│ Actions :           │
│ - Extraire contexte│
│   utilisateur       │
│ - Construire contexte│
│   d'autorisation    │
│ - Déterminer instance│
│ - Traduire id SPM → │
│   format KindMother │
└────┬────────────────┘
     │ 3. read(entity_id, context)
     ▼
┌─────────────────────┐
│    KINDMOTHER       │
│                     │
│ Actions :           │
│ - Vérifier permissions│
│ - Résoudre instance │
│ - Lire depuis       │
│   persistance       │
│ - Retourner données │
└────┬────────────────┘
     │ 4. résultat (données ou erreur)
     ▼
┌─────────────────────┐
│ ADAPTATEUR PRODUIT  │
│ ContentAdapter      │
│                     │
│ Actions :           │
│ - Traduire données  │
│   KindMother → SPM  │
│ - Traduire erreurs  │
│   si nécessaire     │
└────┬────────────────┘
     │ 5. résultat (Content ou ContentError)
     ▼
┌─────────────────────┐
│ MODULE SPM CONTENT   │
│ ContentManager       │
└────┬────────────────┘
     │ 6. résultat
     ▼
┌──────────┐
│ PRODUIT  │
└──────────┘
```

### 8.3. Flux d'écriture détaillé

```
┌──────────┐
│ PRODUIT  │
└────┬─────┘
     │ 1. create_content(input)
     ▼
┌─────────────────────┐
│ MODULE SPM CONTENT  │
│ ContentManager      │
└────┬────────────────┘
     │ 2. create_content(input)
     ▼
┌─────────────────────┐
│ ADAPTATEUR PRODUIT  │
│ ContentAdapter      │
│                     │
│ Actions :           │
│ - Extraire contexte│
│   utilisateur       │
│ - Construire contexte│
│   d'autorisation    │
│ - Déterminer instance│
│ - Traduire input SPM│
│   → WriteIntent     │
│ - Construire        │
│   WriteIntent       │
└────┬────────────────┘
     │ 3. submitWriteIntent(write_intent, context)
     ▼
┌─────────────────────┐
│    KINDMOTHER       │
│                     │
│ Actions :           │
│ - Valider permissions│
│ - Valider cohérence │
│ - Appliquer         │
│   WriteIntent       │
│ - Marquer pour sync │
│   (si DB Fille)     │
│ - Retourner résultat│
└────┬────────────────┘
     │ 4. résultat (succès ou erreur)
     ▼
┌─────────────────────┐
│ ADAPTATEUR PRODUIT  │
│ ContentAdapter      │
│                     │
│ Actions :           │
│ - Traduire résultat │
│   KindMother → SPM  │
│ - Traduire erreurs  │
│   si nécessaire     │
└────┬────────────────┘
     │ 5. résultat (Id ou ContentError)
     ▼
┌─────────────────────┐
│ MODULE SPM CONTENT   │
│ ContentManager       │
└────┬────────────────┘
     │ 6. résultat
     ▼
┌──────────┐
│ PRODUIT  │
└──────────┘
```

### 8.4. Isolation des couches

```
┌─────────────────────────────────────────────────────────────┐
│                    COUCHE PRODUIT                            │
│                                                              │
│  Connaît :                                                   │
│  ✓ Modules SPM (traits)                                     │
│  ✗ KindMother (jamais)                                      │
│  ✗ Détails de persistance (jamais)                          │
│                                                              │
│  Utilise :                                                   │
│  → Traits SPM via adaptateurs                              │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ isolation garantie
                            │ par l'adaptateur
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                  COUCHE MODULES SPM                          │
│                                                              │
│  Connaît :                                                   │
│  ✓ Kernel (Id, Clock, Logger)                              │
│  ✗ KindMother (jamais)                                      │
│  ✗ Persistance (jamais)                                     │
│  ✗ Adaptateurs (jamais directement)                          │
│                                                              │
│  Expose :                                                    │
│  → Traits fonctionnels purs                                 │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ implémenté par
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              COUCHE ADAPTATEUR PRODUIT                       │
│                                                              │
│  Connaît :                                                   │
│  ✓ Modules SPM (traits à implémenter)                      │
│  ✓ KindMother (CoreDataAPI)                                  │
│  ✓ Produit (contexte, règles de permissions)                 │
│  ✗ Détails d'implémentation KindMother (jamais)              │
│  ✗ SQLite ou persistance (jamais)                           │
│                                                              │
│  Responsabilités :                                           │
│  → Traduction bidirectionnelle                              │
│  → Fourniture de contexte                                   │
│  → Isolation des couches                                     │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ appelle uniquement
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                  COUCHE KINDMOTHER                           │
│                                                              │
│  Connaît :                                                   │
│  ✓ Kernel (Id, Clock, Logger)                              │
│  ✓ CoreDataAPI (interface contractuelle)                    │
│  ✗ Modules SPM (jamais)                                     │
│  ✗ Produit (jamais)                                          │
│                                                              │
│  Expose :                                                    │
│  → CoreDataAPI (10 opérations)                              │
│  → Abstraction complète de la persistance                    │
└─────────────────────────────────────────────────────────────┘
```

### 8.5. Flux de synchronisation (DB Fille → DB Mère)

```
┌─────────────────────┐
│   DB FILLE LOCALE   │
│  (instance joueur)  │
└────┬────────────────┘
     │ WriteIntent appliqués localement
     │ (en attente de synchronisation)
     ▼
┌─────────────────────┐
│ ADAPTATEUR PRODUIT  │
│                     │
│ Actions :           │
│ - Détecter besoin   │
│   de synchronisation│
│ - Construire contexte│
│   (instance source, │
│   instance cible)   │
└────┬────────────────┘
     │ requestSync(source, target, context)
     ▼
┌─────────────────────┐
│    KINDMOTHER       │
│                     │
│ Actions :           │
│ - Calculer deltas   │
│ - Valider deltas    │
│ - Détecter conflits │
│ - Résoudre conflits │
│ - Appliquer deltas  │
│   à DB Mère         │
│ - Mettre à jour     │
│   point de sync     │
└────┬────────────────┘
     │ résultat (rapport de synchronisation)
     ▼
┌─────────────────────┐
│ ADAPTATEUR PRODUIT  │
│                     │
│ Actions :           │
│ - Interpréter       │
│   rapport           │
│ - Informer produit  │
│   si nécessaire     │
└────┬────────────────┘
     │
     ▼
┌─────────────────────┐
│     DB MÈRE         │
│  (source de vérité)  │
└─────────────────────┘
```

---

## 9. Conclusion

L'adaptateur produit est un composant architectural fondamental dans Miyukini Core System v2.4. Il assure la médiation entre les produits, les modules SPM CMS, et KindMother, garantissant l'isolation des couches et le respect des principes architecturaux.

### Points clés

- **Définition :** L'adaptateur produit est un médiateur qui traduit entre le domaine fonctionnel des modules SPM et le domaine d'autorité des données de KindMother.

- **Responsabilités :** Traduction bidirectionnelle, fourniture de contexte, implémentation des traits SPM, gestion des erreurs, isolation des modules SPM.

- **Interdictions :** Contourner KindMother, exposer KindMother au produit, modifier les règles de permissions, bypasser les validations, dépendre des détails d'implémentation.

- **Positionnement :** L'adaptateur se situe à l'intersection de trois couches (Produit, Modules SPM, KindMother) et garantit leur isolation.

- **Spécificité :** Chaque produit nécessite ses propres adaptateurs car les besoins métier, règles de permissions, contexte d'autorisation, configuration d'instance, traduction des types, et gestion des erreurs sont spécifiques.

- **Typologie :** Les adaptateurs peuvent être classés selon les profils d'usage (CMS, jeu solo, jeu multijoueur, application locale, SaaS) avec des caractéristiques communes et des différences spécifiques.

Cette documentation établit le contrat conceptuel de l'adaptateur produit et sert de référence pour l'architecture du système. Elle garantit que chaque adaptateur respecte les principes fondamentaux tout en permettant l'adaptation aux besoins spécifiques de chaque produit.

---

**Document créé le :** 2026-01-24  
**Version :** 1.0  
**Statut :** Documentation conceptuelle validée  
**Référence :** Complète la documentation KindMother et définit le concept d'adaptateur produit dans Miyukini Core System v2.4
