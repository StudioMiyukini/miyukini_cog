# BondingBrother - Architecture et Composants

## 1. Contexte

Ce document décrit l'architecture technique interne de Bonding Brother et ses composants structurels. Il complète la [Documentation Fondatrice](./BondingBrother%20-%20Documentation%20Fondatrice.md) en détaillant **comment** Bonding Brother est construit, sans jamais remettre en question **pourquoi** il existe ou **ce qu'il fait**.

Cette architecture respecte les [Lois d'Autonomie Système](../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md), notamment **LOI-1** (aucune dépendance externe critique), **LOI-2** (isolement comme état normal), et **LOI-5** (coût proportionnel au hardware).

## 2. Portée / Scope

Ce document couvre :
- La structure en couches de Bonding Brother
- Les composants internes et leurs responsabilités
- Les interfaces entre composants
- Les flux de données internes

Ce document **ne couvre pas** :
- Les règles métier (voir les contrats spécifiques)
- Les protocoles d'intégration avec les autorités (voir les contrats d'intégration)
- Les invariants comportementaux (voir Invariants & Guarantees)

---

## 3. Architecture en couches

Bonding Brother est organisé en **quatre couches distinctes**, chacune avec une responsabilité unique et des interfaces claires.

```
┌─────────────────────────────────────────────────────────────┐
│                    COUCHE PRODUIT                           │
│         (Interface stable vers les produits)                │
├─────────────────────────────────────────────────────────────┤
│                    COUCHE TRADUCTION                        │
│    (Transformation intention ↔ demande, réponse ↔ résultat) │
├─────────────────────────────────────────────────────────────┤
│                    COUCHE MÉDIATION                         │
│   (Orchestration, délégation, filtrage, journalisation)     │
├─────────────────────────────────────────────────────────────┤
│                    COUCHE AUTORITÉ                          │
│         (Interface vers Kind Mother et Strong Father)       │
└─────────────────────────────────────────────────────────────┘
```

### 3.1 Couche Produit

**Responsabilité :** Exposer une interface stable, prévisible et documentée aux produits de l'écosystème.

**Composants :**
- **ProductGateway** : Point d'entrée unique pour toutes les intentions des produits
- **IntentReceiver** : Réception et validation structurelle des intentions
- **ResultEmitter** : Émission des résultats filtrés vers les produits
- **NotificationDispatcher** : Distribution des notifications de l'écosystème vers les produits

**Interfaces exposées :**
- `IIntentSubmission` : Soumission d'intentions
- `IResultConsumption` : Consommation de résultats
- `INotificationSubscription` : Abonnement aux notifications

**Règle architecturale :** Cette couche est la seule que les produits peuvent voir. Toute autre couche est invisible et inaccessible aux produits.

### 3.2 Couche Traduction

**Responsabilité :** Transformer les intentions en demandes et les réponses en résultats, en préservant la sémantique tout en adaptant le format.

**Composants :**
- **IntentTranslator** : Transformation intention → demande
- **ResponseTranslator** : Transformation réponse → résultat
- **VocabularyMapper** : Correspondance entre vocabulaires (produit ↔ autorité)
- **ContextEnricher** : Enrichissement du contexte pour les autorités

**Interfaces internes :**
- `ITranslation` : Contrat de traduction bidirectionnelle
- `IVocabularyMapping` : Règles de correspondance de vocabulaire
- `IContextEnrichment` : Règles d'enrichissement contextuel

**Règle architecturale :** La traduction est pure et sans effet de bord. Elle ne modifie aucun état, ne prend aucune décision, ne stocke aucune donnée.

### 3.3 Couche Médiation

**Responsabilité :** Orchestrer le flux complet entre les produits et les autorités, en appliquant les règles de filtrage et de journalisation.

**Composants :**
- **MediationOrchestrator** : Coordination du flux complet d'une intention
- **AuthorityRouter** : Routage vers l'autorité appropriée (Kind Mother ou Strong Father)
- **FilterEngine** : Application des règles de filtrage (entrée et sortie)
- **JournalWriter** : Journalisation systématique de toutes les interactions
- **OfflineBuffer** : Gestion des intentions en mode déconnecté (conforme à **LOI-2** : isolement comme état normal)

Cette couche garantit que le système fonctionne même en mode offline, respectant **LOI-2** en acceptant l'isolement comme état normal plutôt qu'une erreur.

**Interfaces internes :**
- `IMediation` : Contrat d'orchestration
- `IAuthorityRouting` : Règles de routage vers les autorités
- `IFiltering` : Règles de filtrage
- `IJournaling` : Contrat de journalisation

**Règle architecturale :** La médiation ne décide jamais. Elle applique des règles définies ailleurs, délègue les décisions aux autorités, et journalise tout.

### 3.4 Couche Autorité

**Responsabilité :** Interfacer avec Kind Mother et Strong Father de manière standardisée et traçable.

**Composants :**
- **KindMotherAdapter** : Adaptateur pour les interactions avec Kind Mother
- **StrongFatherAdapter** : Adaptateur pour les interactions avec Strong Father
- **AuthorityResponseHandler** : Réception et normalisation des réponses des autorités
- **DeferredAuthorityManager** : Gestion de l'autorité différée (mode offline)

**Interfaces vers les autorités :**
- `IKindMotherInterface` : Contrat d'interface avec Kind Mother
- `IStrongFatherInterface` : Contrat d'interface avec Strong Father

**Règle architecturale :** Cette couche adapte les formats, mais ne modifie jamais le sens. Elle transmet fidèlement dans les deux sens.

---

## 4. Composants transversaux

Ces composants servent plusieurs couches et assurent des fonctions critiques non spécifiques à une couche.

### 4.1 ConfigurationStore

**Responsabilité :** Stocker et fournir la configuration de Bonding Brother.

**Caractéristiques :**
- Configuration immuable après initialisation
- Pas de configuration dynamique en production
- Traçabilité complète des valeurs de configuration

**Ce qu'il ne fait pas :**
- Ne stocke aucune donnée métier
- Ne prend aucune décision basée sur la configuration
- Ne modifie pas son état après démarrage

### 4.2 MetricsCollector

**Responsabilité :** Collecter les métriques de fonctionnement sans impacter le flux principal.

**Métriques collectées :**
- Nombre d'intentions reçues/traduites/transmises
- Temps de traitement par étape
- Taux de succès/échec par autorité
- Volume de données journalisées

**Ce qu'il ne fait pas :**
- Ne prend aucune décision basée sur les métriques
- Ne modifie pas le comportement de Bonding Brother
- Ne stocke pas de données métier

### 4.3 HealthChecker

**Responsabilité :** Vérifier l'état de santé de Bonding Brother et de ses connexions aux autorités.

**Vérifications :**
- Connectivité vers Kind Mother
- Connectivité vers Strong Father
- État des composants internes
- Capacité du buffer offline

**Ce qu'il ne fait pas :**
- Ne répare pas automatiquement
- Ne prend pas de décision de routage
- Ne masque pas les problèmes

---

## 5. Flux de données internes

### 5.1 Flux Produit → Écosystème

```
Produit
   │
   ▼
┌─────────────────┐
│ ProductGateway  │ ← Validation structurelle
└────────┬────────┘
         │ Intention validée
         ▼
┌─────────────────┐
│ IntentTranslator│ ← Traduction intention → demande
└────────┬────────┘
         │ Demande traduite
         ▼
┌─────────────────┐
│ FilterEngine    │ ← Filtrage d'entrée
└────────┬────────┘
         │ Demande filtrée
         ▼
┌─────────────────┐
│ JournalWriter   │ ← Journalisation
└────────┬────────┘
         │ Demande journalisée
         ▼
┌─────────────────┐
│ AuthorityRouter │ ← Routage vers autorité
└────────┬────────┘
         │
    ┌────┴────┐
    ▼         ▼
┌───────┐ ┌───────┐
│  KM   │ │  SF   │ ← Transmission à l'autorité
└───┬───┘ └───┬───┘
    └────┬────┘
         │ Réponse autorité
         ▼
┌─────────────────┐
│ResponseTranslator│ ← Traduction réponse → résultat
└────────┬────────┘
         │ Résultat traduit
         ▼
┌─────────────────┐
│ FilterEngine    │ ← Filtrage de sortie
└────────┬────────┘
         │ Résultat filtré
         ▼
┌─────────────────┐
│ ResultEmitter   │ ← Émission vers produit
└────────┬────────┘
         │
         ▼
      Produit
```

### 5.2 Flux Écosystème → Produit

```
Autorité (KM ou SF)
         │
         │ Notification/Événement
         ▼
┌─────────────────────┐
│ AuthorityResponse   │ ← Réception
│ Handler             │
└────────┬────────────┘
         │ Événement normalisé
         ▼
┌─────────────────┐
│ResponseTranslator│ ← Traduction
└────────┬────────┘
         │ Message traduit
         ▼
┌─────────────────┐
│ FilterEngine    │ ← Filtrage
└────────┬────────┘
         │ Message filtré
         ▼
┌─────────────────┐
│ JournalWriter   │ ← Journalisation
└────────┬────────┘
         │
         ▼
┌─────────────────────┐
│ NotificationDispatcher│ ← Distribution
└────────┬────────────┘
         │
         ▼
      Produits concernés
```

---

## 6. Isolation et encapsulation

### 6.1 Principe d'isolation

Chaque couche est **strictement isolée** des autres. Une couche ne peut accéder qu'à :
- Ses propres composants internes
- Les interfaces exposées par la couche adjacente

**Interdit :**
- Accès direct d'une couche à une couche non adjacente
- Partage d'état entre couches
- Dépendances circulaires entre composants

### 6.2 Principe d'encapsulation

Chaque composant **encapsule** son implémentation :
- L'interface est stable et documentée
- L'implémentation peut évoluer sans impacter les consommateurs
- Aucun détail interne n'est exposé

### 6.3 Frontières de responsabilité

| Composant | Responsable de | Non responsable de |
|-----------|----------------|-------------------|
| ProductGateway | Recevoir les intentions | Décider de leur validité métier |
| IntentTranslator | Transformer le format | Valider la permission |
| FilterEngine | Appliquer les règles | Définir les règles |
| AuthorityRouter | Choisir l'autorité | Décider à la place de l'autorité |
| JournalWriter | Enregistrer | Interpréter les enregistrements |

---

## 7. Extensibilité

### 7.1 Points d'extension

Bonding Brother peut être étendu **uniquement** aux points suivants :

| Point d'extension | Type | Contrainte |
|-------------------|------|------------|
| Nouveaux types d'intention | Addition | Doivent suivre le contrat IIntentSubmission |
| Nouveaux vocabulaires produit | Addition | Doivent avoir un mapping complet |
| Nouvelles règles de filtrage | Addition | Doivent être définies par une autorité |
| Nouveaux types de notification | Addition | Doivent suivre le contrat INotificationSubscription |

### 7.2 Points non extensibles

Ces éléments sont **figés** et non extensibles :

- Structure en 4 couches
- Flux de données (direction et ordre des étapes)
- Rôle de chaque composant
- Interfaces entre couches
- Principe de délégation aux autorités

---

## 8. Dépendances

### 8.1 Dépendances internes (entre composants)

```
ProductGateway ──────► IntentReceiver
                      ▼
              IntentTranslator
                      ▼
               FilterEngine ◄──── FilterRules (config)
                      ▼
               JournalWriter
                      ▼
              AuthorityRouter
                   ▼    ▼
    KindMotherAdapter  StrongFatherAdapter
```

### 8.2 Dépendances externes (vers l'écosystème)

| Dépendance | Type | Criticité |
|------------|------|-----------|
| Kind Mother | Autorité données | Critique |
| Strong Father | Autorité identités/permissions | Critique |
| Storage Journal | Persistance journal | Haute |
| Configuration | Paramétrage | Démarrage |

**Note sur l'autonomie :** Conforme à **LOI-1** (aucune dépendance externe critique), Bonding Brother peut fonctionner en mode offline avec buffer, même si les autorités sont temporairement indisponibles. Les autorités sont nécessaires pour l'évaluation, mais leur absence ne bloque pas le démarrage ni la réception d'intentions.

### 8.3 Absence de dépendances

Bonding Brother **ne dépend pas** :
- D'aucun produit spécifique
- D'aucune base de données métier
- D'aucun service externe autre que les autorités
- D'aucune logique métier spécifique
- D'aucune connexion réseau permanente (conforme à **LOI-1** et **LOI-2**)

---

## 9. Statut contractuel

Ce document est **contractuel, normatif, et de statut ARCHITECTURE**. Il établit la structure interne de Bonding Brother qui ne peut être modifiée sans processus formel de versionnement.

Toute implémentation de Bonding Brother doit respecter cette architecture. Toute extension doit utiliser les points d'extension définis. Toute modification structurelle nécessite une nouvelle version de ce document.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** ARCHITECTURE — Normatif  
**Dépendance :** Documentation Fondatrice v1.0
