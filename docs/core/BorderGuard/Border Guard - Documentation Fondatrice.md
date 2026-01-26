# Miyukini Core System — Border Guard Documentation Fondatrice

## 1. Introduction

### Rôle de Border Guard

Border Guard (BG) est le **core de définition des frontières et des règles d'entrée/sortie** du Miyukini Core System. Il incarne la capacité conceptuelle du système à distinguer ce qui est interne de ce qui est externe, à classifier les niveaux de confiance, et à établir les règles qui gouvernent toute interaction traversant une frontière.

Border Guard ne filtre pas lui-même, ne bloque pas lui-même, n'exécute pas lui-même. Il **définit** les frontières, **établit** les règles, et **classifie** les niveaux de confiance. L'application de ces règles est déléguée à Bonding Brother et aux autres cores opérationnels.

### Question fondamentale

Border Guard répond à une question fondamentale : **"Où sont les frontières du système, et quelles règles gouvernent leur franchissement ?"**

Cette question se décline en plusieurs sous-questions :
- Qu'est-ce qui est "interne" et qu'est-ce qui est "externe" ?
- Quel niveau de confiance accorder à une source ou une destination ?
- Quelles conditions doivent être respectées pour franchir une frontière ?
- Comment classifier les intégrations selon leur nature et leur risque ?

### Portée

Ce contrat s'applique à **toutes les définitions de frontières** dans le système Miyukini et définit de manière absolue :
- La définition formelle des frontières et de leur nature
- La classification des niveaux de confiance
- Les règles de franchissement des frontières
- Les invariants de définition de frontière
- Les garanties offertes par Border Guard
- Les distinctions entre définition conceptuelle et application technique

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

---

## 2. Raison d'être

### Problème que Border Guard résout

Dans l'architecture actuelle de MCS, les frontières entre l'interne et l'externe sont implicites, dispersées, et incohérentes. Cette absence de définition formelle présente plusieurs limitations :

1. **Frontières implicites** : Chaque composant définit ses propres frontières sans vision globale, conduisant à des définitions contradictoires

2. **Niveaux de confiance non standardisés** : Chaque intégration gère ses propres niveaux de confiance sans classification cohérente à l'échelle du système

3. **Règles de franchissement dispersées** : Les règles qui gouvernent le passage d'une frontière sont répliquées et incohérentes entre composants

4. **Absence de gouvernance des intégrations** : Aucun point central pour définir la politique d'intégration avec les systèmes externes

5. **Confusion entre définition et application** : La définition des frontières est mélangée avec leur application technique, créant un couplage fort

Border Guard résout ces problèmes en fournissant un core dédié qui :
- Définit formellement les frontières du système
- Établit une classification standardisée des niveaux de confiance
- Centralise les règles de franchissement des frontières
- Gouverne conceptuellement toutes les intégrations
- Sépare strictement la définition de l'application

### Positionnement architectural

Border Guard est un **core conceptuel** :
- Il ne possède aucune capacité d'exécution
- Il ne filtre pas, ne bloque pas, n'intercepte pas
- Il définit, classifie, et établit des règles
- Ses définitions sont consommées par Bonding Brother pour l'application

Border Guard est conçu comme une **autorité de définition** :
- Autorité exclusive sur la définition des frontières
- Autorité exclusive sur la classification des niveaux de confiance
- Aucune autorité sur l'application des règles
- Aucune autorité sur l'exécution technique

---

## 3. Positionnement familial

### Relation avec Kind Mother

Border Guard et Kind Mother sont complémentaires mais distincts :

**Kind Mother** gouverne les données et leur persistance. Elle définit ce qui est une donnée, comment elle est stockée, comment elle est synchronisée.

**Border Guard** gouverne les frontières et les niveaux de confiance. Il définit si une donnée venant de l'extérieur peut entrer, avec quel niveau de confiance, selon quelles règles.

La relation est de complémentarité : Kind Mother traite les données une fois qu'elles sont "à l'intérieur" ; Border Guard définit les conditions pour qu'elles y entrent.

Border Guard ne connaît pas les détails de persistance de Kind Mother. Kind Mother ne connaît pas les détails de classification de Border Guard. Chacun reste souverain dans son domaine.

### Relation avec Strong Father

Border Guard et Strong Father sont complémentaires et collaboratifs :

**Strong Father** prend les décisions stratégiques et politiques. Il évalue les intentions et produit des décisions (acceptée, refusée, ambiguë).

**Border Guard** définit le contexte de confiance dans lequel Strong Father opère. Il fournit à Strong Father l'information sur le niveau de confiance de l'origine d'une intention, la nature de la frontière franchie, les règles applicables.

La relation est de conseil : Border Guard informe Strong Father sur le contexte de frontière ; Strong Father décide en tenant compte de cette information.

Border Guard ne décide jamais. Strong Father décide en s'appuyant sur les définitions de Border Guard.

### Relation avec Bonding Brother

Border Guard et Bonding Brother ont une relation fondamentale et asymétrique :

**Border Guard définit les règles** de franchissement des frontières, les niveaux de confiance, les conditions d'entrée et de sortie.

**Bonding Brother applique ces règles** lors de la médiation entre les produits et l'écosystème. Il consulte les définitions de Border Guard et les applique concrètement.

La relation est de définition/application : Border Guard est l'autorité conceptuelle, Bonding Brother est l'exécutant opérationnel.

Cette relation est non négociable : Bonding Brother ne définit jamais de frontière, Border Guard n'applique jamais de règle. La séparation est absolue.

### Relation avec Caring Nanny

Border Guard et Caring Nanny sont complémentaires dans l'observation :

**Caring Nanny** observe l'état global du système (healthy, degraded, offline, syncing, error).

**Border Guard** définit comment l'état des frontières influence l'état global. Une frontière compromise peut signaler un état dégradé. Une intégration défaillante peut signaler un problème.

La relation est d'information : Border Guard informe Caring Nanny sur l'état des frontières ; Caring Nanny intègre cette information dans l'état global.

### La famille Miyukini

Dans la famille Miyukini, Border Guard est le **gardien des limites** : il connaît les frontières de la maison, il sait qui peut entrer par quelle porte, il définit les règles d'accueil des visiteurs.

Border Guard ne décide pas qui entre (c'est Strong Father), ne stocke pas les informations des visiteurs (c'est Kind Mother), n'accueille pas lui-même les visiteurs (c'est Bonding Brother). Il définit où sont les portes, quelles sont les règles, quel niveau de confiance accorder.

---

## 4. Concepts fondamentaux

### Frontière

Une **frontière** est une démarcation conceptuelle qui sépare deux zones de confiance différentes. Une frontière peut être :

**Frontière externe** : Sépare l'écosystème Miyukini du monde extérieur (internet, systèmes tiers, utilisateurs non authentifiés). C'est la limite entre le "dehors" et le "dedans".

**Frontière interne** : Sépare différentes zones de confiance au sein de l'écosystème (zone admin vs zone utilisateur, module sensible vs module standard, données critiques vs données publiques).

**Frontière d'intégration** : Sépare l'écosystème d'un système externe avec lequel il interagit de manière contrôlée (API partenaire, service tiers, base de données externe).

Une frontière possède :
- Une identité unique et stable
- Une direction (entrée, sortie, bidirectionnelle)
- Un niveau de perméabilité (ouvert, contrôlé, fermé)
- Des règles de franchissement associées

### Niveau de confiance

Un **niveau de confiance** est une classification qui indique le degré de fiabilité accordé à une source, une destination, ou une interaction. Border Guard définit quatre niveaux canoniques :

**Trusted (Confiance totale)** : La source ou destination fait partie du cercle de confiance absolu. Aucune vérification supplémentaire n'est requise. Réservé aux composants internes validés, aux autorités du système.

**Verified (Confiance vérifiée)** : La source ou destination a été authentifiée et validée selon des critères stricts. Des vérifications ont été effectuées. Niveau accordé aux utilisateurs authentifiés, aux intégrations certifiées.

**Unknown (Confiance inconnue)** : La source ou destination n'a pas encore été classifiée. Niveau par défaut pour tout ce qui arrive de l'extérieur. Toute interaction avec ce niveau est soumise à des règles restrictives.

**Hostile (Confiance nulle)** : La source ou destination a été identifiée comme malveillante, compromise, ou violant les règles. Aucune interaction n'est autorisée. Niveau appliqué aux sources blacklistées, aux patterns d'attaque détectés.

### Règle de franchissement

Une **règle de franchissement** est une condition qui doit être satisfaite pour qu'une interaction puisse traverser une frontière. Une règle est :

**Déclarative** : Elle exprime ce qui est requis, pas comment le vérifier techniquement.

**Non ambiguë** : Elle spécifie clairement les conditions sans interprétation possible.

**Associée à une frontière** : Elle est définie pour une frontière spécifique ou un ensemble de frontières.

Une règle de franchissement peut porter sur :
- Le niveau de confiance requis
- L'authentification requise
- Les données autorisées à traverser
- Les actions autorisées
- Les conditions temporelles

### Zone de confiance

Une **zone de confiance** est un espace conceptuel délimité par des frontières, où tous les éléments partagent un même niveau de confiance. Une zone de confiance :

- Est délimitée par une ou plusieurs frontières
- Possède un niveau de confiance homogène
- Contient des composants, des données, des services
- Interagit avec d'autres zones via des frontières

### Intégration

Une **intégration** est une relation établie entre l'écosystème Miyukini et un système externe. Une intégration est classifiée par Border Guard selon :

- Son niveau de confiance initial
- Les frontières qu'elle traverse
- Les règles de franchissement applicables
- Son état (active, suspendue, révoquée)

---

## 5. Responsabilités exclusives

### Définition des frontières

Border Guard est **exclusivement responsable** de la définition formelle des frontières du système. Cette responsabilité inclut :

- Identifier et nommer chaque frontière
- Classifier la nature de chaque frontière (externe, interne, intégration)
- Définir la direction de chaque frontière (entrée, sortie, bidirectionnelle)
- Établir le niveau de perméabilité de chaque frontière
- Maintenir le registre exhaustif des frontières du système

Aucun autre core ne définit de frontière. Toute définition de frontière provient exclusivement de Border Guard.

### Classification des niveaux de confiance

Border Guard est **exclusivement responsable** de la classification des niveaux de confiance. Cette responsabilité inclut :

- Définir les critères de chaque niveau de confiance (trusted, verified, unknown, hostile)
- Classifier les sources et destinations selon ces niveaux
- Établir les règles de transition entre niveaux
- Maintenir la cohérence de la classification à travers le système

Aucun autre core ne classifie les niveaux de confiance. Toute classification provient exclusivement de Border Guard.

### Établissement des règles de franchissement

Border Guard est **exclusivement responsable** de l'établissement des règles de franchissement. Cette responsabilité inclut :

- Définir les règles associées à chaque frontière
- Spécifier les conditions de franchissement
- Établir les exceptions et cas particuliers
- Maintenir la cohérence des règles entre frontières

Aucun autre core n'établit de règle de franchissement. Toute règle provient exclusivement de Border Guard.

### Gouvernance conceptuelle des intégrations

Border Guard est **exclusivement responsable** de la gouvernance conceptuelle des intégrations. Cette responsabilité inclut :

- Classifier chaque intégration selon sa nature et son risque
- Définir le cadre d'interaction avec chaque système externe
- Établir les conditions de suspension ou révocation d'une intégration
- Maintenir le registre des intégrations et leur état

Aucun autre core ne gouverne conceptuellement les intégrations. Cette gouvernance provient exclusivement de Border Guard.

### Conseil aux autres cores

Border Guard est **responsable** de fournir les informations de frontière aux autres cores. Cette responsabilité inclut :

- Informer Strong Father du contexte de confiance d'une intention
- Informer Bonding Brother des règles à appliquer
- Informer Caring Nanny de l'état des frontières

Cette responsabilité de conseil n'est pas une autorité : Border Guard informe, les autres cores décident ou agissent.

---

## 6. Ce que Border Guard ne fait PAS

### Ne filtre pas

Border Guard ne filtre **jamais** les interactions. Le filtrage est une action d'application, pas de définition. Border Guard définit les règles de filtrage ; Bonding Brother les applique.

### Ne bloque pas

Border Guard ne bloque **jamais** les accès. Le blocage est une action d'exécution. Border Guard définit les conditions qui peuvent conduire à un blocage ; Bonding Brother ou Strong Father exécute le blocage.

### N'authentifie pas

Border Guard ne gère **jamais** l'authentification technique. L'authentification (tokens, sessions, OAuth, JWT) est du ressort du produit ou d'un module auth dédié. Border Guard définit les niveaux de confiance ; l'authentification technique détermine comment atteindre ces niveaux.

### Ne persiste pas

Border Guard ne persiste **jamais** de données. La persistance est du ressort exclusif de Kind Mother. Border Guard définit des frontières et des règles ; leur stockage est délégué à Kind Mother.

### Ne décide pas

Border Guard ne prend **jamais** de décision stratégique ou politique. La décision est du ressort exclusif de Strong Father. Border Guard informe sur le contexte de confiance ; Strong Father décide.

### N'exécute pas

Border Guard n'exécute **jamais** d'action technique. L'exécution est du ressort des cores opérationnels (Bonding Brother, adaptateurs, produits). Border Guard est purement conceptuel.

### Ne modifie pas l'état

Border Guard ne modifie **jamais** l'état du système. L'observation de l'état est du ressort de Caring Nanny, la modification de l'état est du ressort des cores exécutants. Border Guard définit, il ne modifie pas.

### Ne contient pas de logique métier

Border Guard ne contient **jamais** de logique métier spécifique aux produits. Il définit des concepts généraux (frontières, confiance, règles) applicables à tous les produits. La logique métier spécifique reste dans les produits.

---

## 7. Invariants non négociables

### INV-BG-1 : Aucune capacité d'exécution

Border Guard ne possède **jamais** de capacité d'exécution. Il ne filtre pas, ne bloque pas, n'intercepte pas, n'applique pas. Toute capacité d'exécution viole cet invariant fondamental.

### INV-BG-2 : Aucune persistance directe

Border Guard n'accède **jamais** directement à la persistance. Toute définition de frontière ou de règle qui doit être persistée est transmise à Kind Mother via les canaux appropriés.

### INV-BG-3 : Aucune décision autonome

Border Guard ne prend **jamais** de décision de manière autonome. Il informe, il classifie, il définit, mais la décision finale appartient toujours à Strong Father ou aux autorités appropriées.

### INV-BG-4 : Classification exhaustive

Toute source, destination, ou interaction **doit** être classifiée selon un niveau de confiance. Aucune interaction ne peut exister sans classification. Par défaut, tout ce qui n'est pas explicitement classifié est considéré comme "unknown".

### INV-BG-5 : Frontières explicites

Toute frontière **doit** être explicitement définie et documentée. Aucune frontière implicite n'est autorisée. Si une démarcation existe dans le système, elle doit être formalisée par Border Guard.

### INV-BG-6 : Règles déclaratives

Toutes les règles de franchissement **doivent** être déclaratives. Aucune règle procédurale ou impérative n'est autorisée. Une règle exprime ce qui est requis, pas comment le vérifier.

### INV-BG-7 : Séparation définition/application

La définition des frontières et des règles est **strictement séparée** de leur application. Border Guard définit, Bonding Brother applique. Cette séparation est non négociable et ne peut être contournée.

### INV-BG-8 : Traçabilité complète

Toute définition de frontière, toute classification de confiance, toute règle établie **doit** être traçable avec son origine, sa date, et sa justification.

### INV-BG-9 : Cohérence globale

Les définitions de Border Guard **doivent** être globalement cohérentes. Aucune contradiction entre frontières, niveaux de confiance, ou règles n'est autorisée.

### INV-BG-10 : Neutralité conceptuelle

Border Guard **ne fait jamais** de supposition sur la technologie d'implémentation. Les définitions sont purement conceptuelles et peuvent être implémentées par n'importe quelle technologie.

---

## 8. Interactions avec l'écosystème

### Flux d'information vers Strong Father

Quand Strong Father évalue une intention, il peut consulter Border Guard pour obtenir le contexte de confiance :

1. **Strong Father** reçoit une intention à évaluer
2. **Strong Father** demande à Border Guard le contexte de frontière (quelle frontière est traversée, quel niveau de confiance de la source)
3. **Border Guard** retourne les informations de classification et les règles applicables
4. **Strong Father** utilise ces informations pour prendre sa décision

Ce flux est purement informatif : Border Guard ne participe pas à la décision, il fournit le contexte.

### Flux de règles vers Bonding Brother

Quand Bonding Brother doit médier une interaction traversant une frontière, il consulte Border Guard :

1. **Bonding Brother** reçoit une intention de médiation
2. **Bonding Brother** identifie qu'une frontière est traversée
3. **Bonding Brother** demande à Border Guard les règles de franchissement applicables
4. **Border Guard** retourne les règles déclaratives
5. **Bonding Brother** applique ces règles concrètement

Ce flux est de définition/application : Border Guard fournit les règles, Bonding Brother les exécute.

### Flux d'état vers Caring Nanny

Quand l'état d'une frontière change (intégration défaillante, frontière compromise), Border Guard informe Caring Nanny :

1. **Border Guard** détecte un changement d'état d'une frontière ou d'une intégration
2. **Border Guard** notifie Caring Nanny de ce changement
3. **Caring Nanny** intègre cette information dans l'état global du système

Ce flux est d'observation : Border Guard signale, Caring Nanny observe et agrège.

### Flux de classification

Quand une nouvelle source ou intégration doit être classifiée :

1. **Le produit** ou **Bonding Brother** soumet une demande de classification
2. **Border Guard** évalue selon ses critères et définitions
3. **Border Guard** attribue un niveau de confiance
4. **Border Guard** établit les règles de franchissement applicables
5. **Border Guard** notifie les cores concernés de cette nouvelle classification

Ce flux est de classification : Border Guard est l'autorité qui attribue les niveaux de confiance.

### Diagramme des interactions

```
┌─────────────────────────────────────────────────────────────┐
│                     ÉCOSYSTÈME MIYUKINI                      │
│                                                              │
│    ┌─────────────┐                    ┌─────────────┐       │
│    │   Strong    │◄───── contexte ────│   Border    │       │
│    │   Father    │      de confiance  │   Guard     │       │
│    │  (Décision) │                    │ (Définition)│       │
│    └─────────────┘                    └─────────────┘       │
│                                            │ │              │
│                                   règles ──┘ └── état       │
│                                            │ │              │
│    ┌─────────────┐                         ▼ │              │
│    │  Bonding    │◄────── règles ──────────┘ │              │
│    │  Brother    │        de franchissement   │              │
│    │(Application)│                            │              │
│    └─────────────┘                            ▼              │
│                                        ┌─────────────┐       │
│                                        │   Caring    │       │
│                                        │   Nanny     │       │
│                                        │   (État)    │       │
│                                        └─────────────┘       │
│                                                              │
└─────────────────────────────────────────────────────────────┘
                          │
                          │ Frontière externe
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                     MONDE EXTÉRIEUR                          │
│   (Systèmes tiers, utilisateurs, intégrations)               │
└─────────────────────────────────────────────────────────────┘
```

---

## 9. Vocabulaire canonique

Le vocabulaire de Border Guard est précis, stable, non ambigu. Chaque terme a une définition canonique, non négociable.

### Frontière

Une **frontière** est une démarcation conceptuelle entre deux zones de confiance différentes. Elle possède une identité, une direction, un niveau de perméabilité, et des règles de franchissement associées. Une frontière est toujours explicitement définie par Border Guard.

### Zone de confiance

Une **zone de confiance** est un espace conceptuel délimité par des frontières où tous les éléments partagent un niveau de confiance homogène. Les zones de confiance sont organisées hiérarchiquement, de la plus sécurisée (zone interne) à la moins sécurisée (zone externe).

### Niveau de confiance

Un **niveau de confiance** est une classification attribuée à une source, une destination, ou une interaction. Les niveaux canoniques sont : trusted (confiance totale), verified (confiance vérifiée), unknown (confiance inconnue), hostile (confiance nulle).

### Franchissement

Un **franchissement** est l'acte de traverser une frontière. Chaque franchissement est soumis aux règles définies pour la frontière concernée. Un franchissement peut être autorisé, conditionnel, ou interdit selon les règles.

### Règle de franchissement

Une **règle de franchissement** est une condition déclarative qui spécifie ce qui est requis pour qu'un franchissement soit autorisé. Une règle est associée à une frontière et s'applique à toutes les interactions traversant cette frontière.

### Intégration

Une **intégration** est une relation établie entre l'écosystème Miyukini et un système externe. Une intégration est classifiée par Border Guard et possède un niveau de confiance, des frontières associées, et des règles spécifiques.

### Perméabilité

La **perméabilité** est la caractéristique d'une frontière qui indique sa propension à autoriser le franchissement. Une frontière peut être ouverte (franchissement libre sous conditions minimales), contrôlée (franchissement soumis à vérification), ou fermée (franchissement interdit).

### Classification

La **classification** est l'acte d'attribuer un niveau de confiance à une source, une destination, ou une interaction. Seul Border Guard a l'autorité de classifier. Toute interaction non explicitement classifiée est considérée comme "unknown".

### Gouvernance d'intégration

La **gouvernance d'intégration** est l'ensemble des règles et processus qui encadrent la relation avec les systèmes externes. Cette gouvernance définit les conditions d'établissement, de maintien, et de révocation des intégrations.

### Contexte de frontière

Le **contexte de frontière** est l'ensemble des informations relatives aux frontières traversées par une interaction : quelles frontières, quel niveau de confiance de la source, quelles règles applicables. Ce contexte est fourni par Border Guard aux autres cores.

---

## 10. Conformité aux Lois d'Autonomie Système

Ce core respecte les **Lois d'Autonomie Système** définies dans [Miyukini Framework - Lois Autonomie Systeme.md](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md). Border Guard devient **critique pour l'autonomie** en contrôlant toutes les frontières du système.

### LOI-1 : Aucune dépendance externe critique à l'exécution

**Conformité :** ✅ **Conforme — Rôle critique**

Border Guard respecte intégralement LOI-1 et joue un rôle critique :
- **Contrôle tout ce qui entre et sort du système** via la définition des frontières
- Les règles de franchissement sont **locales** et chargées au démarrage
- Aucune définition de frontière ne nécessite un appel externe
- L'absence de connexion ne bloque jamais la définition des frontières

**Architecture :** Border Guard définit les frontières de manière locale et autonome. C'est le gardien qui garantit qu'aucune dépendance externe critique ne peut entrer dans le système.

### LOI-6 : L'autonomie n'empêche pas la fédération

**Conformité :** ✅ **Conforme — Rôle critique**

Border Guard joue un rôle critique pour LOI-6 :
- **Validation explicite des échanges fédérés** : Toute communication inter-nœuds doit passer par Border Guard pour classification
- **Rien d'implicite** : Les frontières sont explicitement définies, pas supposées
- **Contrôle des règles de partage** : Border Guard définit ce qui peut être partagé dans une fédération
- **Fédération réversible** : Les frontières peuvent être modifiées pour quitter une fédération

**Architecture :** Border Guard définit les règles de fédération, garantissant que la fédération reste explicite, contrôlée, observable, et réversible.

### Rôle renforcé dans l'autonomie

Border Guard devient **critique pour l'autonomie** car :
- **Contrôle des entrées/sorties** : Aucune communication externe ne peut contourner Border Guard
- **Validation explicite** : Tous les échanges fédérés sont validés selon les règles définies par Border Guard
- **Protection de l'autonomie** : Les frontières définies par Border Guard protègent l'autonomie du système

**Relation avec Bonding Brother :** Border Guard définit les règles, Bonding Brother les applique. Cette séparation garantit que les frontières sont définies localement (LOI-1) et que la fédération est contrôlée (LOI-6).

### Autres lois

- **LOI-2 (Isolement comme état normal)** : Les frontières définies par Border Guard permettent de reconnaître l'isolement comme un état normal (pas d'erreur si une frontière est fermée).
- **LOI-3 (État local souverain)** : Les définitions de frontières sont locales et souveraines.
- **LOI-5 (Coût hardware)** : Border Guard est un core conceptuel léger, sans exécution, optimisé pour les ressources limitées.

---

## 11. Conclusion et statut contractuel

### Phrase fondatrice

**Border Guard est l'autorité de définition des frontières et des niveaux de confiance qui établit les règles de franchissement sans jamais les appliquer lui-même, séparant strictement la définition conceptuelle de l'exécution technique.**

Cette phrase résume l'essence de Border Guard : autorité (mais non décisionnel), définition (mais non exécution), règles (mais non filtrage), conceptuel (mais non technique).

### Garanties offertes

Border Guard garantit :

1. **Exhaustivité** : Toute frontière du système est explicitement définie
2. **Classification complète** : Toute source et interaction est classifiée
3. **Cohérence** : Les définitions sont globalement cohérentes et non contradictoires
4. **Traçabilité** : Toute définition est traçable avec son origine et sa justification
5. **Neutralité technique** : Les définitions sont indépendantes de l'implémentation
6. **Séparation stricte** : La définition est strictement séparée de l'application

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :

- **Kind Mother — Documentation Fondatrice** : Border Guard ne persiste pas et ne stocke pas
- **Strong Father — Documentation Fondatrice** : Border Guard informe mais ne décide pas
- **Bonding Brother — Documentation Fondatrice** : Border Guard définit, Bonding Brother applique

Il n'introduit aucune contradiction et constitue la définition formelle de ce que signifie une frontière, un niveau de confiance, et une règle de franchissement dans le système Miyukini.

### Statut final

Ce document est de statut **FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Toute implémentation touchant aux frontières, aux niveaux de confiance, ou aux règles de franchissement doit respecter intégralement ce document.

Les invariants définis ici sont non négociables. Toute violation de ces invariants constitue une faute architecturale qui doit être corrigée.

---

**Version :** 1.5  
**Date :** 2026-01-26  
**Statut :** FONDATION — Non négociable  
**Référence :** Miyukini Core System v2.4, Kind Mother Documentation Fondatrice, Strong Father Documentation Fondatrice, Bonding Brother Documentation Fondatrice, Miyukini Framework - Lois Autonomie Systeme, [Miyukini Framework - Integrity & Degradation System](../../reference/Miyukini%20Framework%20-%20Integrity%20Degradation%20System.md), [Miyukini Framework - External Signal & Trust Reinforcement Contract](../../reference/Miyukini%20Framework%20-%20External%20Signal%20Trust%20Reinforcement%20Contract.md), [Miyukini Framework - Mobile & WebApp Strategy](../../reference/Miyukini%20Framework%20-%20Mobile%20WebApp%20Strategy.md) (protection injection mobile/web), [Miyukini Framework - Security Protocols](../../reference/Miyukini%20Framework%20-%20Security%20Protocols.md) (classification sources, protection injection), [Miyukini Framework - Security Levels](../../reference/Miyukini%20Framework%20-%20Security%20Levels.md) (adaptation frontières selon niveau sécurité 0-4)
