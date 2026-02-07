# Caring Nanny - Architecture et Composants

## 1. Contexte

Ce document décrit l'architecture technique interne de Caring Nanny et ses composants structurels. Il complète la [Documentation Fondatrice](./Caring%20Nanny%20-%20Documentation%20Fondatrice.md) en détaillant **comment** Caring Nanny est construit, sans jamais remettre en question **pourquoi** il existe ou **ce qu'il fait**.

## 2. Portée / Scope

Ce document couvre :
- La structure en couches de Caring Nanny
- Les composants internes et leurs responsabilités
- Les interfaces entre composants
- Les flux de données internes

Ce document **ne couvre pas** :
- Les règles métier (voir les contrats spécifiques)
- Les protocoles d'intégration avec les autres membres de la famille (voir les contrats d'intégration)
- Les invariants comportementaux (voir Invariants et Garanties)

---

## 3. Architecture en couches

Caring Nanny est organisé en **quatre couches distinctes**, chacune avec une responsabilité unique et des interfaces claires. Cette architecture reflète la nature purement observatrice de Caring Nanny : collecter, classer, propager, historiser — sans jamais modifier ni décider.

```
┌─────────────────────────────────────────────────────────────┐
│                    COUCHE CONSULTATION                       │
│        (Interface de lecture pour les consommateurs)         │
├─────────────────────────────────────────────────────────────┤
│                    COUCHE PROPAGATION                        │
│   (Diffusion des changements d'état aux composants concernés)│
├─────────────────────────────────────────────────────────────┤
│                    COUCHE CLASSIFICATION                     │
│     (Évaluation, catégorisation et agrégation des états)     │
├─────────────────────────────────────────────────────────────┤
│                    COUCHE OBSERVATION                        │
│      (Collecte des conditions depuis les composants)         │
└─────────────────────────────────────────────────────────────┘
```

### 3.1 Couche Observation

**Responsabilité :** Collecter les conditions observables depuis les différents composants du système sans interférer avec leur fonctionnement.

**Composants :**
- **ConditionCollector** : Point de collecte unique pour toutes les conditions observées
- **ComponentProbe** : Sonde passive pour chaque type de composant (KindMother, StrongFather, modules SPM)
- **ConditionNormalizer** : Normalisation des conditions dans un format unifié
- **TimestampMarker** : Horodatage précis de chaque observation (horodatage local, conforme à **LOI-4** : pas de temps global requis, voir [Miyukini Framework - Lois Autonomie Systeme.md](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md))

**Interfaces exposées :**
- `IConditionReception` : Réception des conditions depuis les composants
- `IProbeConfiguration` : Configuration des sondes d'observation
- `IObservationMetadata` : Métadonnées d'observation (source, timestamp, contexte)

**Règle architecturale :** Cette couche est strictement passive. Aucune sonde ne peut modifier l'état du composant observé. L'observation est non bloquante et sans effet de bord.

### 3.2 Couche Classification

**Responsabilité :** Évaluer les conditions collectées, les catégoriser selon les cinq états définis (healthy, degraded, offline, syncing, error), et les agréger en état système global.

**Conformité LOI-2 :** Cette couche reconnaît explicitement l'état `offline` comme un état normal (isolement accepté), distinct de l'état `error` (anomalie). Cette distinction respecte **LOI-2** (le système accepte l'isolement comme état normal) définie dans [Miyukini Framework - Lois Autonomie Systeme.md](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

**Composants :**
- **StateEvaluator** : Évaluation d'une condition en état partiel
- **CategoryClassifier** : Classification selon les cinq catégories d'état
- **StateAggregator** : Agrégation des états partiels en état système global
- **TransitionDetector** : Détection des changements d'état (transitions)
- **AnomalyDetector** : Détection des conditions anormales

**Interfaces internes :**
- `IStateEvaluation` : Contrat d'évaluation condition → état
- `ICategoryClassification` : Règles de classification par catégorie
- `IStateAggregation` : Règles d'agrégation des états partiels
- `ITransitionDetection` : Détection et enregistrement des transitions

**Règle architecturale :** La classification est déterministe et reproductible. Une même condition, dans un même contexte, produit toujours le même état. La classification n'interprète pas, elle applique des règles définies.

### 3.3 Couche Propagation

**Responsabilité :** Diffuser les changements d'état aux composants concernés via BondingBrother, de manière fidèle et traçable.

**Composants :**
- **ChangeNotifier** : Détection des changements d'état à propager
- **RecipientResolver** : Identification des destinataires d'une notification
- **MessageFormatter** : Construction du message de notification
- **PropagationDispatcher** : Transmission à BondingBrother pour distribution
- **PropagationTracker** : Suivi des propagations effectuées

**Interfaces internes :**
- `IChangeNotification` : Contrat de notification de changement
- `IRecipientResolution` : Règles d'identification des destinataires
- `IMessageFormatting` : Format standard des messages de notification
- `IPropagationTracking` : Traçabilité des propagations

**Règle architecturale :** La propagation est passive et fidèle. Caring Nanny informe, elle ne commande pas. Le message transmis est exactement celui observé, sans interprétation ni filtrage.

### 3.4 Couche Consultation

**Responsabilité :** Exposer une interface de lecture pour les consommateurs (StrongFather, produits, modules) permettant d'interroger l'état actuel ou l'historique.

**Composants :**
- **StateQueryHandler** : Traitement des requêtes d'état actuel
- **HistoryQueryHandler** : Traitement des requêtes d'historique
- **ResponseBuilder** : Construction des réponses avec contexte et métadonnées
- **CacheManager** : Gestion du cache d'état pour performances

**Interfaces exposées :**
- `IStateQuery` : Interrogation de l'état actuel (global ou spécifique)
- `IHistoryQuery` : Interrogation de l'historique des observations
- `ITransitionQuery` : Interrogation de l'historique des transitions

**Règle architecturale :** Cette couche est en lecture seule. Aucune consultation ne peut modifier l'état observé. La consultation n'a aucun effet de bord sur le système.

---

## 4. Composants transversaux

Ces composants servent plusieurs couches et assurent des fonctions critiques non spécifiques à une couche.

### 4.1 HistoryStore

**Responsabilité :** Maintenir l'historique complet des observations, transitions, et propagations pour audit et diagnostic.

**Caractéristiques :**
- Enregistrement chronologique de toutes les observations
- Conservation des transitions avec leur cause
- Indexation pour recherche rapide
- Gestion de la rétention selon les politiques définies

**Ce qu'il ne fait pas :**
- Ne stocke aucune donnée métier
- Ne prend aucune décision basée sur l'historique
- Ne modifie pas les observations enregistrées

### 4.2 ConfigurationStore

**Responsabilité :** Stocker et fournir la configuration de Caring Nanny (seuils, règles de classification, politiques de rétention).

**Caractéristiques :**
- Configuration immuable après initialisation
- Pas de configuration dynamique en production
- Traçabilité complète des valeurs de configuration

**Ce qu'il ne fait pas :**
- Ne stocke aucune donnée métier
- Ne prend aucune décision basée sur la configuration
- Ne modifie pas son état après démarrage

### 4.3 ObservationMetrics

**Responsabilité :** Collecter les métriques de fonctionnement de Caring Nanny sans impacter l'observation.

**Métriques collectées :**
- Nombre de conditions observées par composant
- Nombre de transitions détectées par catégorie
- Temps de traitement par étape
- Volume d'historique et taux de rétention
- Latence de propagation

**Conformité LOI-5 :** La collecte de métriques est optimisée pour une consommation minimale de ressources, conforme à **LOI-5** (le coût doit être proportionnel au hardware) définie dans [Miyukini Framework - Lois Autonomie Systeme.md](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

**Ce qu'il ne fait pas :**
- Ne prend aucune décision basée sur les métriques
- Ne modifie pas le comportement de Caring Nanny
- Ne stocke pas de données métier

### 4.4 SelfHealthReporter

**Responsabilité :** Rapporter l'état de santé de Caring Nanny lui-même, sans créer de récursion infinie.

**Vérifications :**
- État des sondes d'observation (actives, dégradées, en erreur)
- Capacité de l'historique (espace disponible)
- Connectivité avec BondingBrother pour propagation
- Latence des opérations internes

**Ce qu'il ne fait pas :**
- Ne s'auto-observe pas de manière récursive
- Ne répare pas automatiquement
- Ne masque pas les problèmes

---

## 5. Flux de données internes

### 5.1 Flux d'observation (Composant → Caring Nanny)

```
Composant (KM, SF, Module SPM)
         │
         │ Condition observée
         ▼
┌─────────────────┐
│ ComponentProbe  │ ← Sonde passive spécifique au composant
└────────┬────────┘
         │ Condition brute
         ▼
┌─────────────────────┐
│ ConditionNormalizer │ ← Normalisation du format
└────────┬────────────┘
         │ Condition normalisée
         ▼
┌─────────────────┐
│ TimestampMarker │ ← Horodatage précis
└────────┬────────┘
         │ Condition horodatée
         ▼
┌─────────────────┐
│ConditionCollector│ ← Collecte centralisée
└────────┬────────┘
         │ Condition collectée
         ▼
┌─────────────────┐
│ StateEvaluator  │ ← Évaluation condition → état
└────────┬────────┘
         │ État partiel
         ▼
┌──────────────────┐
│CategoryClassifier│ ← Classification (healthy, degraded, ...)
└────────┬─────────┘
         │ État classifié
         ▼
┌─────────────────┐
│ StateAggregator │ ← Agrégation en état global
└────────┬────────┘
         │ État système
         ▼
┌──────────────────┐
│TransitionDetector│ ← Détection de transition
└────────┬─────────┘
         │
    ┌────┴────┐
    ▼         ▼
 Historique  Propagation
```

### 5.2 Flux de propagation (Caring Nanny → Composants)

```
Transition détectée
         │
         ▼
┌─────────────────┐
│ ChangeNotifier  │ ← Identification du changement à propager
└────────┬────────┘
         │ Changement identifié
         ▼
┌──────────────────┐
│RecipientResolver │ ← Identification des destinataires
└────────┬─────────┘
         │ Liste des destinataires
         ▼
┌──────────────────┐
│ MessageFormatter │ ← Construction du message
└────────┬─────────┘
         │ Message formaté
         ▼
┌─────────────────────┐
│PropagationDispatcher│ ← Transmission à BondingBrother
└────────┬────────────┘
         │
         ▼
┌─────────────────────┐
│ PropagationTracker  │ ← Enregistrement de la propagation
└────────┬────────────┘
         │
         ▼
    BondingBrother
         │
         ▼
   Composants concernés
```

### 5.3 Flux de consultation (Consommateur → Caring Nanny)

```
Consommateur (SF, Produit, Module)
         │
         │ Demande d'état
         ▼
┌──────────────────┐
│ StateQueryHandler │ ← Traitement de la requête
└────────┬─────────┘
         │
    ┌────┴────┐
    ▼         ▼
┌────────┐ ┌────────────────┐
│ Cache  │ │ StateAggregator│ ← Source de l'état
└───┬────┘ └───────┬────────┘
    └──────┬───────┘
           ▼
┌─────────────────┐
│ ResponseBuilder │ ← Construction de la réponse
└────────┬────────┘
         │ Réponse avec contexte
         ▼
    Consommateur
```

---

## 6. Isolation et encapsulation

### 6.1 Principe d'isolation

Chaque couche est **strictement isolée** des autres. Une couche ne peut accéder qu'à :
- Ses propres composants internes
- Les interfaces exposées par la couche adjacente

**Interdit :**
- Accès direct d'une couche à une couche non adjacente
- Partage d'état mutable entre couches
- Dépendances circulaires entre composants

### 6.2 Principe d'encapsulation

Chaque composant **encapsule** son implémentation :
- L'interface est stable et documentée
- L'implémentation peut évoluer sans impacter les consommateurs
- Aucun détail interne n'est exposé

### 6.3 Frontières de responsabilité

| Composant | Responsable de | Non responsable de |
|-----------|----------------|-------------------|
| ComponentProbe | Observer passivement | Modifier le composant observé |
| StateEvaluator | Évaluer une condition | Décider d'une action |
| CategoryClassifier | Classifier selon les règles | Définir les règles |
| StateAggregator | Agréger les états partiels | Résoudre les conflits métier |
| ChangeNotifier | Identifier les changements | Décider qui doit réagir |
| RecipientResolver | Lister les destinataires | Forcer les destinataires à agir |
| PropagationDispatcher | Transmettre fidèlement | Interpréter le message |
| HistoryStore | Enregistrer | Interpréter l'historique |

---

## 7. Extensibilité

### 7.1 Points d'extension

Caring Nanny peut être étendu **uniquement** aux points suivants :

| Point d'extension | Type | Contrainte |
|-------------------|------|------------|
| Nouvelles sondes (ComponentProbe) | Addition | Doivent être passives et sans effet de bord |
| Nouvelles règles de classification | Addition | Doivent respecter les cinq catégories définies |
| Nouveaux critères d'anomalie | Addition | Doivent être définis par le produit ou l'écosystème |
| Nouvelles requêtes d'historique | Addition | Doivent suivre le contrat IHistoryQuery |
| Nouveaux types de notification | Addition | Doivent suivre le contrat IChangeNotification |

### 7.2 Points non extensibles

Ces éléments sont **figés** et non extensibles :

- Structure en 4 couches
- Flux de données (direction et ordre des étapes)
- Rôle de chaque composant
- Interfaces entre couches
- Catégories d'état (healthy, degraded, offline, syncing, error)
- Nature purement observatrice (aucune capacité d'action)
- Principe de non-décision

---

## 8. Dépendances

### 8.1 Dépendances internes (entre composants)

```
ComponentProbe ──────► ConditionNormalizer
                              ▼
                       TimestampMarker
                              ▼
                      ConditionCollector
                              ▼
                       StateEvaluator
                              ▼
                      CategoryClassifier
                              ▼
                       StateAggregator
                              ▼
                     TransitionDetector
                         ▼      ▼
              HistoryStore    ChangeNotifier
                                   ▼
                            RecipientResolver
                                   ▼
                            MessageFormatter
                                   ▼
                         PropagationDispatcher
                                   ▼
                          PropagationTracker
```

### 8.2 Dépendances externes (vers l'écosystème)

| Dépendance | Type | Criticité |
|------------|------|-----------|
| KindMother | Source d'observation | Haute |
| StrongFather | Source d'observation + Consommateur | Haute |
| BondingBrother | Canal de propagation | Haute |
| Modules SPM | Sources d'observation | Moyenne |
| Configuration | Paramétrage | Démarrage |

### 8.3 Absence de dépendances

Caring Nanny **ne dépend pas** :
- D'aucun produit spécifique
- D'aucune base de données métier
- D'aucun service externe autre que les composants du core
- D'aucune logique métier spécifique
- D'aucune capacité d'exécution ou de décision

Cette absence de dépendances externes critiques garantit que Caring Nanny fonctionne en autonomie complète, conformément à **LOI-1** (aucune dépendance externe critique à l'exécution) définie dans [Miyukini Framework - Lois Autonomie Systeme.md](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

---

## 9. Garanties architecturales

### 9.1 Observation non intrusive

L'architecture garantit que l'observation n'a aucun effet de bord :
- Les sondes sont passives et en lecture seule
- Aucun composant observé n'est modifié par l'observation
- L'observation ne bloque jamais les opérations normales

**Conformité LOI-2 :** Cette garantie d'observation non bloquante permet au système de fonctionner normalement même en isolation, respectant **LOI-2** (le système accepte l'isolement comme état normal) définie dans [Miyukini Framework - Lois Autonomie Systeme.md](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

### 9.2 Cohérence de l'état

L'architecture garantit que l'état rapporté est toujours cohérent :
- L'agrégation est déterministe et reproductible
- Aucune contradiction n'est possible dans l'état global
- Les transitions sont atomiques et ordonnées

### 9.3 Traçabilité complète

L'architecture garantit une traçabilité complète :
- Chaque observation est horodatée et contextualisée
- Chaque transition est enregistrée avec sa cause
- Chaque propagation est suivie et archivée

### 9.4 Propagation fidèle

L'architecture garantit une propagation fidèle :
- Le message transmis est exactement celui observé
- Aucune interprétation ou filtrage n'est appliqué
- La propagation n'attend pas de confirmation d'action

---

## 10. Statut contractuel

Ce document est **contractuel, normatif, et de statut ARCHITECTURE**. Il établit la structure interne de Caring Nanny qui ne peut être modifiée sans processus formel de versionnement.

Toute implémentation de Caring Nanny doit respecter cette architecture. Toute extension doit utiliser les points d'extension définis. Toute modification structurelle nécessite une nouvelle version de ce document.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** ARCHITECTURE — Normatif  
**Dépendance :** Documentation Fondatrice v1.0
