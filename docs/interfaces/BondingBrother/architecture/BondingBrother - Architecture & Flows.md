# BondingBrother — Architecture & Flows

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** ARCHITECTURE — Normatif

---

## 1. Contexte

Ce document décrit l'architecture technique interne de Bonding Brother, ses composants structurels, et sa vision de haut niveau en tant que **strate de liaison gouvernée** de l'écosystème Miyukini.

Ce document fusionne et remplace les anciens documents "Architecture et Composants" et "Strate de Liaison Gouvernée" pour une vision unifiée.

**Dépendances :**
- [Documentation Fondatrice](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) — Principes fondamentaux
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

## 2. Portée / Scope

Ce document couvre :
- La définition et le rôle fondamental de Bonding Brother
- Son positionnement dans la pyramide Miyukini
- Les rôles internes de la strate (Adapter, Bridge, Gateway, Translator, Contract Enforcer)
- La structure en couches de Bonding Brother
- Les composants internes et leurs responsabilités
- Les flux de données internes
- Les invariants architecturaux

Ce document **ne couvre pas** :
- Les règles métier (voir les contrats spécifiques dans `contracts/`)
- Les protocoles d'intégration avec les autorités (voir `contracts/integration/`)
- Les guidelines d'implémentation (voir `implementation/`)

---

## 3. Définition

**Bonding Brother est la strate de liaison gouvernée de Miyukini.**

Il permet aux entités hétérogènes (cores, outils, opérateurs, COGs, interfaces) de se parler **sans jamais se comprendre implicitement**.

### 3.1 Ce qu'il n'apporte PAS

| Exclusion | Description |
|-----------|-------------|
| ❌ Aucune logique métier | BB ne connaît pas le domaine |
| ❌ Aucune décision | BB ne tranche jamais |
| ❌ Aucune autorité | BB n'a pas de pouvoir |
| ❌ Aucune persistance | BB ne stocke pas d'état métier |

### 3.2 Ce qu'il apporte

| Capacité | Description |
|----------|-------------|
| ✅ Traduction | Conversion entre vocabulaires |
| ✅ Normalisation | Format uniforme pour l'écosystème |
| ✅ Encapsulation | Isolation des implémentations |
| ✅ Isolation | Frontière stricte entre entités |
| ✅ Traçabilité | Tout échange est journalisé |

---

## 4. Positionnement dans la pyramide Miyukini

Bonding Brother **n'est pas un core de gouvernance**, mais il est au même niveau structurel qu'eux.

```
┌─────────────────────────────────────────────┐
│     Interfaces / Réseau / Terminaux         │
└─────────────────────┬───────────────────────┘
                      ▼
┌─────────────────────────────────────────────┐
│           BONDING BROTHER (Strate 5)        │ ← STRATE DE LIAISON
└─────────────────────┬───────────────────────┘
                      ▼
┌─────────────────────────────────────────────┐
│   Cores (StrongFather, KindMother, etc.)    │ ← STRATE 4
└─────────────────────┬───────────────────────┘
                      ▼
┌─────────────────────────────────────────────┐
│                  Kernel                      │
└─────────────────────────────────────────────┘
```

**Règles fondamentales :**
- Tout échange passe par lui
- Aucun échange ne le traverse sans être transformé

### Question fondamentale

> **"Comment deux entités qui n'ont pas le droit de se connaître peuvent-elles échanger ?"**

---

## 5. Rôles internes de la strate

**Bonding Brother n'est PAS un seul composant.** C'est une strate composée avec plusieurs rôles internes.

### 5.1 Adapter (Interne)

**Rôle :** Adapter une entité au langage Miyukini.

| Source | Cible |
|--------|-------|
| UI | Intent Miyukini |
| Tool | Capability Call |
| Produit | Demande gouvernée |
| API externe | Requête normalisée |

**Propriétés :**
- Sens unique ou bidirectionnel
- Stateless
- Strictement typé
- Versionné

> Un adapter ne décide jamais si c'est valide. Il rend simplement la chose auditable.

### 5.2 Bridge (Inter-COG / Inter-Environment)

**Rôle :** Relier deux environnements souverains sans fusion.

| Liaison | Description |
|---------|-------------|
| COG ↔ COG | Visite, migration |
| Environnement isolé ↔ Environnement connecté | Passage de frontière |
| Offline ↔ Online | Synchronisation différée |

**Propriétés :**
- Canal diplomatique
- Aucun état métier
- Transport chiffré
- Vérification d'intégrité

> Le Bridge ne connaît pas le sens de ce qu'il transporte.

### 5.3 Gateway (Exposition contrôlée)

**Rôle :** Exposer une surface vers l'extérieur.

| Surface | Description |
|---------|-------------|
| Site web public | Accès non authentifié |
| API REST / GraphQL | Intégration technique |
| WebSocket temps réel | Communication bidirectionnelle |
| App mobile | Interface native |

**Propriétés :**
- Frontière stricte
- Pas de logique métier
- Couplée à BorderGuard
- Observée par WorrySentinel

> Une gateway n'est jamais une API "libre".

### 5.4 Translator (Sémantique)

**Rôle :** Traduire sans enrichir.

| Entrée | Sortie |
|--------|--------|
| JSON | Intent Structure |
| HTTP | Demande gouvernée |
| UI Event | Action abstraite |

**Propriétés :**
- Perte contrôlée
- Aucune inférence
- Pas de raccourci

> Toute information non comprise est rejetée ou neutralisée.

### 5.5 Contract Enforcer (Structurel)

**Rôle :** Vérifier que l'échange respecte un contrat connu.

| Vérification | Description |
|--------------|-------------|
| Version de protocole | Compatibilité garantie |
| Schéma attendu | Structure valide |
| Champs interdits | Sécurité respectée |
| Sens de circulation | Direction autorisée |

> Il ne valide pas le fond, seulement la forme.

---

## 6. Architecture en couches

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
│         (Interface vers KindMother et StrongFather)         │
└─────────────────────────────────────────────────────────────┘
```

### 6.1 Couche Produit

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

### 6.2 Couche Traduction

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

### 6.3 Couche Médiation

**Responsabilité :** Orchestrer le flux complet entre les produits et les autorités, en appliquant les règles de filtrage et de journalisation.

**Composants :**
- **MediationOrchestrator** : Coordination du flux complet d'une intention
- **AuthorityRouter** : Routage vers l'autorité appropriée (KindMother ou StrongFather)
- **FilterEngine** : Application des règles de filtrage (entrée et sortie)
- **JournalWriter** : Journalisation systématique de toutes les interactions
- **OfflineBuffer** : Gestion des intentions en mode déconnecté (conforme à **LOI-2**)

Cette couche garantit que le système fonctionne même en mode offline, respectant **LOI-2** en acceptant l'isolement comme état normal plutôt qu'une erreur.

**Interfaces internes :**
- `IMediation` : Contrat d'orchestration
- `IAuthorityRouting` : Règles de routage vers les autorités
- `IFiltering` : Règles de filtrage
- `IJournaling` : Contrat de journalisation

**Règle architecturale :** La médiation ne décide jamais. Elle applique des règles définies ailleurs, délègue les décisions aux autorités, et journalise tout.

### 6.4 Couche Autorité

**Responsabilité :** Interfacer avec KindMother et StrongFather de manière standardisée et traçable.

**Composants :**
- **KindMotherAdapter** : Adaptateur pour les interactions avec KindMother
- **StrongFatherAdapter** : Adaptateur pour les interactions avec StrongFather
- **AuthorityResponseHandler** : Réception et normalisation des réponses des autorités
- **DeferredAuthorityManager** : Gestion de l'autorité différée (mode offline)

**Interfaces vers les autorités :**
- `IKindMotherInterface` : Contrat d'interface avec KindMother
- `IStrongFatherInterface` : Contrat d'interface avec StrongFather

**Règle architecturale :** Cette couche adapte les formats, mais ne modifie jamais le sens. Elle transmet fidèlement dans les deux sens.

---

## 7. Composants transversaux

Ces composants servent plusieurs couches et assurent des fonctions critiques non spécifiques à une couche.

### 7.1 ConfigurationStore

**Responsabilité :** Stocker et fournir la configuration de Bonding Brother.

**Caractéristiques :**
- Configuration immuable après initialisation
- Pas de configuration dynamique en production
- Traçabilité complète des valeurs de configuration

### 7.2 MetricsCollector

**Responsabilité :** Collecter les métriques de fonctionnement sans impacter le flux principal.

**Métriques collectées :**
- Nombre d'intentions reçues/traduites/transmises
- Temps de traitement par étape
- Taux de succès/échec par autorité
- Volume de données journalisées

### 7.3 HealthChecker

**Responsabilité :** Vérifier l'état de santé de Bonding Brother et de ses connexions aux autorités.

**Vérifications :**
- Connectivité vers KindMother
- Connectivité vers StrongFather
- État des composants internes
- Capacité du buffer offline

---

## 8. Flux de données internes

### 8.1 Flux Produit → Écosystème

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
┌──────────────────┐
│ResponseTranslator│ ← Traduction réponse → résultat
└────────┬─────────┘
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

### 8.2 Flux Écosystème → Produit

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
┌──────────────────┐
│ResponseTranslator│ ← Traduction
└────────┬─────────┘
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
┌───────────────────────┐
│ NotificationDispatcher│ ← Distribution
└────────┬──────────────┘
         │
         ▼
      Produits concernés
```

---

## 9. Cycle d'un échange typique

**Exemple :** Utilisateur web → Service

```
1. UI produit un événement
         ↓
2. Adapter UI → Intent neutre
         ↓
3. Translator → format Miyukini
         ↓
4. Contract Enforcer → vérifie structure
         ↓
5. BorderGuard → filtre
         ↓
6. StrongFather → décide
         ↓
7. KindMother → lit
         ↓
8. Réponse repasse par Bonding Brother
         ↓
9. Adapter → UI Response
```

**À aucun moment :**
- ❌ l'UI "appelle" un core
- ❌ un core "comprend" l'UI

---

## 10. Isolation et encapsulation

### 10.1 Principe d'isolation

Chaque couche est **strictement isolée** des autres. Une couche ne peut accéder qu'à :
- Ses propres composants internes
- Les interfaces exposées par la couche adjacente

**Interdit :**
- Accès direct d'une couche à une couche non adjacente
- Partage d'état entre couches
- Dépendances circulaires entre composants

### 10.2 Principe d'encapsulation

Chaque composant **encapsule** son implémentation :
- L'interface est stable et documentée
- L'implémentation peut évoluer sans impacter les consommateurs
- Aucun détail interne n'est exposé

### 10.3 Frontières de responsabilité

| Composant | Responsable de | Non responsable de |
|-----------|----------------|-------------------|
| ProductGateway | Recevoir les intentions | Décider de leur validité métier |
| IntentTranslator | Transformer le format | Valider la permission |
| FilterEngine | Appliquer les règles | Définir les règles |
| AuthorityRouter | Choisir l'autorité | Décider à la place de l'autorité |
| JournalWriter | Enregistrer | Interpréter les enregistrements |

---

## 11. Extensibilité

### 11.1 Points d'extension

Bonding Brother peut être étendu **uniquement** aux points suivants :

| Point d'extension | Type | Contrainte |
|-------------------|------|------------|
| Nouveaux types d'intention | Addition | Doivent suivre le contrat IIntentSubmission |
| Nouveaux vocabulaires produit | Addition | Doivent avoir un mapping complet |
| Nouvelles règles de filtrage | Addition | Doivent être définies par une autorité |
| Nouveaux types de notification | Addition | Doivent suivre le contrat INotificationSubscription |

### 11.2 Points non extensibles

Ces éléments sont **figés** et non extensibles :
- Structure en 4 couches
- Flux de données (direction et ordre des étapes)
- Rôle de chaque composant
- Interfaces entre couches
- Principe de délégation aux autorités

---

## 12. Dépendances

### 12.1 Dépendances externes (vers l'écosystème)

| Dépendance | Type | Criticité |
|------------|------|-----------|
| KindMother | Autorité données | Critique |
| StrongFather | Autorité décisions | Critique |
| Storage Journal | Persistance journal | Haute |
| Configuration | Paramétrage | Démarrage |

**Note sur l'autonomie :** Conforme à **LOI-1** (aucune dépendance externe critique), Bonding Brother peut fonctionner en mode offline avec buffer, même si les autorités sont temporairement indisponibles.

### 12.2 Absence de dépendances

Bonding Brother **ne dépend pas** :
- D'aucun produit spécifique
- D'aucune base de données métier
- D'aucun service externe autre que les autorités
- D'aucune logique métier spécifique
- D'aucune connexion réseau permanente (conforme à **LOI-1** et **LOI-2**)

---

## 13. Invariants architecturaux

Ces invariants sont **gravés dans le marbre** — non négociables, non contournables.

| Code | Invariant |
|------|-----------|
| **BB-ARCH-1** | Bonding Brother ne décide jamais |
| **BB-ARCH-2** | Bonding Brother ne persiste jamais d'état métier |
| **BB-ARCH-3** | Bonding Brother ne déduit jamais |
| **BB-ARCH-4** | Tout ce qu'il transmet est traçable |
| **BB-ARCH-5** | Toute ambiguïté est rejetée |
| **BB-ARCH-6** | Il ne fait confiance à personne |
| **BB-ARCH-7** | Il ne parle jamais sans contrat |

---

## 14. Relations avec les autres cores

| Core | Relation avec Bonding Brother |
|------|------------------------------|
| **StrongFather** | Reçoit des intents normalisés |
| **KindMother** | Reçoit des requêtes de lecture traduites |
| **MasterButler** | Expose des capacités via BB |
| **BorderGuard** | Filtre AVANT BB ou AVEC BB |
| **WorrySentinel** | Observe les flux BB |
| **TAMR** | Passe par BB pour l'humain |
| **MiyukiniAdmin** | BB interne renforcé |

---

## 15. Pourquoi Bonding Brother est critique

### 15.1 Sans lui

| Problème | Conséquence |
|----------|-------------|
| Les cores seraient couplés | Fragilité architecturale |
| Les produits imposeraient leur logique | Perte de cohérence |
| Les interfaces dicteraient le modèle | Inversion de contrôle |
| La sécurité serait fragmentée | Failles multiples |
| La migration serait impossible | Dette technique |

### 15.2 Avec lui

| Bénéfice | Description |
|----------|-------------|
| Tout est remplaçable | Modularité totale |
| Tout est versionnable | Évolution contrôlée |
| Tout est observable | Debug et audit |
| Tout est gouvernable | Contrôle centralisé |

---

## 16. Analogie

> **Bonding Brother = Ministère des Affaires étrangères + Douanes + Traducteurs**

| Aspect | Description |
|--------|-------------|
| Il ne gouverne pas | Pas de pouvoir exécutif |
| Il ne légifère pas | Pas de pouvoir législatif |
| Il applique des protocoles | Exécution stricte des règles établies |

---

## 17. Phrase fondatrice architecturale

> **Bonding Brother est ce qui permet à Miyukini d'être ouvert sans jamais être permissif.**

---

## 18. Statut contractuel

Ce document est **contractuel, normatif, et de statut ARCHITECTURE**. Il établit la structure interne de Bonding Brother qui ne peut être modifiée sans processus formel de versionnement.

Toute implémentation de Bonding Brother doit respecter cette architecture. Toute extension doit utiliser les points d'extension définis. Toute modification structurelle nécessite une nouvelle version de ce document.

---

## Navigation

- [Index BondingBrother](../_index.md)
- [Documentation Fondatrice](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)
- [Core Interaction Contract](./BondingBrother%20-%20Core%20Interaction%20Contract.md)

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** ARCHITECTURE — Normatif  
**Dépendance :** Documentation Fondatrice v2.0
