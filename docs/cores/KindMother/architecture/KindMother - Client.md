# KindMother — Client (Délégation)

## 1. Introduction

### Objet du document

Ce document décrit le **KindMother Client** — le composant qui permet aux Services et Opérateurs COG d'accéder aux données via le Core KindMother. Il définit le pattern de **délégation** par lequel les Opérateurs confient leurs opérations de données à KindMother.

### Contexte

Dans l'architecture Miyukini COG, les Opérateurs (JayXpose, JayKonta, JayFestival...) n'ont **aucun accès direct** aux bases de données. Toute opération de persistance est **déléguée** à KindMother via le Client.

> **"Aucun module SPM ne parle directement à une base de données."**
> — [KindMother - Documentation Fondatrice](../foundation/KindMother%20-%20Documentation%20Fondatrice.md)

### Portée

Ce document couvre :
- Architecture du Client
- Pattern de délégation
- API et opérations disponibles
- Gestion du cycle de vie
- Bonnes pratiques d'utilisation

### Ce document ne couvre PAS

- Architecture interne du Core Server (voir [KindMother - Core Server](./KindMother%20-%20Core%20Server.md))
- Implémentation technique (voir [Systeme Persistance libSQL Migration](../implementation/KindMother%20-%20Systeme%20Persistance%20libSQL%20Migration.md))

---

## 2. Architecture du Client

### 2.1 Positionnement

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        SERVICE / OPÉRATEUR                                  │
│                     (JayXpose, JayKonta, etc.)                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                      LOGIQUE MÉTIER                                   │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                   │  │
│  │  │   Screens   │  │  Services   │  │   Handlers  │                   │  │
│  │  │    (UI)     │  │  (Business) │  │    (API)    │                   │  │
│  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘                   │  │
│  │         └────────────────┼────────────────┘                          │  │
│  └──────────────────────────┼───────────────────────────────────────────┘  │
│                             │                                               │
│                             ▼                                               │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                    KINDMOTHER CLIENT                                  │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │  │
│  │  │   Storage   │  │    Auth     │  │    IPC      │  │   Error     │  │  │
│  │  │    Trait    │  │   Manager   │  │  Transport  │  │  Handler    │  │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘  │  │
│  └──────────────────────────┬───────────────────────────────────────────┘  │
│                             │                                               │
└─────────────────────────────┼───────────────────────────────────────────────┘
                              │ IPC (gRPC / Socket)
                              ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                     KINDMOTHER CORE SERVER                                  │
│                    (Processus isolé séparé)                                 │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Composants du Client

| Composant | Rôle | Description |
|-----------|------|-------------|
| **Storage Trait** | Interface | Implémente le trait `Storage` de KindMother |
| **Auth Manager** | Authentification | Génère et gère les tokens d'auth |
| **IPC Transport** | Communication | Gère la connexion gRPC/Socket |
| **Error Handler** | Erreurs | Traduit les erreurs serveur en erreurs locales |

### 2.3 Caractéristiques

| Caractéristique | Description |
|-----------------|-------------|
| **Transparent** | L'appelant ne sait pas que les données sont sur un serveur séparé |
| **Asynchrone** | Toutes les opérations sont async (tokio) |
| **Résilient** | Gère les déconnexions et reconnexions |
| **Typé** | API fortement typée, pas de SQL brut |

---

## 3. Pattern de Délégation

### 3.1 Principe

La délégation est le mécanisme par lequel un Opérateur **confie** ses opérations de données à KindMother, qui les **exécute** en son nom après **arbitrage**.

```
┌────────────────────────────────────────────────────────────────────────────┐
│                     PATTERN DE DÉLÉGATION                                  │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│   OPÉRATEUR                    CLIENT                    CORE SERVER       │
│       │                           │                           │            │
│       │  "Je veux lire X"         │                           │            │
│       │─────────────────────────► │                           │            │
│       │                           │   Délégation + Auth       │            │
│       │                           │ ─────────────────────────►│            │
│       │                           │                           │            │
│       │                           │        Arbitrage          │            │
│       │                           │        Exécution          │            │
│       │                           │                           │            │
│       │                           │   Résultat (ou erreur)    │            │
│       │                           │ ◄─────────────────────────│            │
│       │  Données (ou erreur)      │                           │            │
│       │◄───────────────────────── │                           │            │
│       │                           │                           │            │
│                                                                            │
│   L'Opérateur ne sait pas :                                                │
│   - Où sont physiquement les données                                       │
│   - Comment elles sont chiffrées                                           │
│   - Quel arbitrage a été appliqué                                          │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Contrats de délégation

Les contrats de délégation définissent les règles de la relation Client ↔ Server :

| Code | Contrat | Description |
|------|---------|-------------|
| **KM-DELEG-01** | Délégation exclusive | Toute persistance passe par KindMother |
| **KM-DELEG-02** | Authentification obligatoire | Chaque requête porte un token valide |
| **KM-DELEG-03** | Contexte complet | Le client transmet tout le contexte nécessaire |
| **KM-DELEG-04** | Acceptation du verdict | L'opérateur accepte la décision du serveur |
| **KM-DELEG-05** | Non-contournement | Le client ne tente jamais d'accès direct |

### 3.3 Ce que le Client fait / ne fait pas

| Le Client FAIT | Le Client NE FAIT PAS |
|----------------|----------------------|
| Transmet les requêtes | Exécute les requêtes lui-même |
| Génère les tokens d'auth | Décide des permissions |
| Gère les erreurs réseau | Accède aux fichiers DB |
| Sérialise/désérialise | Connaît le schéma SQL |
| Retry sur erreurs temporaires | Contourne les refus |

---

## 4. API du Client

### 4.1 Initialisation

```
┌────────────────────────────────────────────────────────────────────────────┐
│                    INITIALISATION DU CLIENT                                │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  1. CONNEXION                                                              │
│     Client::connect(                                                       │
│       address: "http://[::1]:50051",  // Adresse du Core Server            │
│       operator_id: "jayxpose",         // Identité de l'opérateur          │
│       database: "jayxpose",            // Base de données cible            │
│     )                                                                      │
│                                                                            │
│  2. AUTHENTIFICATION                                                       │
│     Le client s'authentifie auprès du serveur                              │
│     Obtient un shared_secret pour signer les requêtes                      │
│                                                                            │
│  3. PRÊT                                                                   │
│     Le client peut maintenant effectuer des opérations                     │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 4.2 Opérations disponibles

#### Lecture d'entité

| Méthode | Description |
|---------|-------------|
| `read_entity(table, id)` | Lit une entité par son ID |
| `list_entities(table, filter)` | Liste les entités avec filtre optionnel |
| `count_entities(table, filter)` | Compte les entités |

**Paramètres de lecture :**

| Paramètre | Type | Description |
|-----------|------|-------------|
| `table` | String | Nom de la table |
| `id` | String | Identifiant de l'entité |
| `filter` | Filter | Critères de filtrage (optionnel) |
| `pagination` | Pagination | Limit/Offset (optionnel) |
| `sort` | Sort | Tri (optionnel) |

#### Écriture d'entité

| Méthode | Description |
|---------|-------------|
| `create_entity(table, data)` | Crée une nouvelle entité |
| `update_entity(table, id, data)` | Met à jour une entité existante |
| `upsert_entity(table, id, data)` | Crée ou met à jour |
| `delete_entity(table, id)` | Supprime une entité |

**Paramètres d'écriture :**

| Paramètre | Type | Description |
|-----------|------|-------------|
| `table` | String | Nom de la table |
| `id` | String | Identifiant (optionnel pour create) |
| `data` | Bytes | Données sérialisées (JSON) |

#### Opérations avancées

| Méthode | Description |
|---------|-------------|
| `batch_write(operations)` | Exécute plusieurs écritures en transaction |
| `query(sql, params)` | Requête SQL (si autorisé par permissions) |

### 4.3 Réponses

#### Succès

```
┌────────────────────────────────────────────────┐
│             RÉPONSE SUCCÈS                     │
├────────────────────────────────────────────────┤
│  success: true                                 │
│  data: { ... }      // Données demandées       │
│  id: "xxx"          // Pour create/upsert      │
│  affected_rows: 1   // Pour update/delete      │
└────────────────────────────────────────────────┘
```

#### Erreur

```
┌────────────────────────────────────────────────┐
│             RÉPONSE ERREUR                     │
├────────────────────────────────────────────────┤
│  success: false                                │
│  error_code: "PERMISSION_DENIED"               │
│  error_message: "No write access to table X"   │
│  retry_after: null   // Délai si quota         │
└────────────────────────────────────────────────┘
```

---

## 5. Cycle de Vie du Client

### 5.1 États du Client

```
                    ┌─────────────────┐
                    │  DISCONNECTED   │
                    └────────┬────────┘
                             │ connect()
                    ┌────────▼────────┐
                    │   CONNECTING    │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
      ┌───────▼───────┐ ┌───▼───┐  ┌───────▼───────┐
      │  CONNECTED    │ │ ERROR │  │ AUTH_FAILED   │
      │   (Ready)     │ └───────┘  └───────────────┘
      └───────┬───────┘
              │
      ┌───────▼───────┐
      │  OPERATING    │ ◄─── Requêtes en cours
      └───────┬───────┘
              │
      ┌───────▼───────┐
      │   CLOSING     │
      └───────┬───────┘
              │
      ┌───────▼───────┐
      │   CLOSED      │
      └───────────────┘
```

### 5.2 Gestion de la connexion

| Événement | Action Client |
|-----------|---------------|
| Connexion perdue | Tentative de reconnexion automatique |
| Reconnexion réussie | Reprend les opérations en attente |
| Reconnexion échouée (max retries) | Signale l'erreur, passe en DISCONNECTED |
| Fermeture explicite | Annule les opérations en cours, ferme proprement |

### 5.3 Timeout et retry

| Paramètre | Valeur par défaut | Description |
|-----------|-------------------|-------------|
| `connect_timeout` | 10s | Timeout de connexion initiale |
| `request_timeout` | 30s | Timeout par requête |
| `retry_count` | 3 | Nombre de tentatives |
| `retry_delay` | 1s, 2s, 4s | Backoff exponentiel |

---

## 6. Utilisation par les Services

### 6.1 Pattern d'intégration

```
┌────────────────────────────────────────────────────────────────────────────┐
│                  INTÉGRATION DANS UN SERVICE                               │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │                     SERVICE (ex: JayXpose)                          │  │
│   │                                                                     │  │
│   │   ┌─────────────────────────────────────────────────────────────┐   │  │
│   │   │                    AppState                                 │   │  │
│   │   │                                                             │   │  │
│   │   │   km_client: Arc<Mutex<KindMotherClient>>                   │   │  │
│   │   │   // Partagé entre tous les handlers                        │   │  │
│   │   │                                                             │   │  │
│   │   └─────────────────────────────────────────────────────────────┘   │  │
│   │                              │                                      │  │
│   │          ┌───────────────────┼───────────────────┐                  │  │
│   │          ▼                   ▼                   ▼                  │  │
│   │   ┌─────────────┐   ┌─────────────┐   ┌─────────────┐              │  │
│   │   │  Handler A  │   │  Handler B  │   │  Handler C  │              │  │
│   │   │             │   │             │   │             │              │  │
│   │   │ state.km_   │   │ state.km_   │   │ state.km_   │              │  │
│   │   │ client.     │   │ client.     │   │ client.     │              │  │
│   │   │ read(...)   │   │ write(...)  │   │ list(...)   │              │  │
│   │   └─────────────┘   └─────────────┘   └─────────────┘              │  │
│   │                                                                     │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 6.2 Exemple d'utilisation typique

**Lecture d'un exposant :**

```
Étape 1: Handler reçoit une requête GET /exposants/{id}
         │
         ▼
Étape 2: Handler appelle km_client.read_entity("exposants", id)
         │
         ▼
Étape 3: Client génère le token d'auth
         │
         ▼
Étape 4: Client envoie la requête IPC au Core Server
         │
         ▼
Étape 5: Core Server arbitre (auth, perms, validation)
         │
         ▼
Étape 6: Core Server exécute la requête sur libSQL
         │
         ▼
Étape 7: Core Server retourne les données chiffrées → déchiffrées
         │
         ▼
Étape 8: Client reçoit et désérialise les données
         │
         ▼
Étape 9: Handler reçoit l'ExposantProfile
         │
         ▼
Étape 10: Handler retourne la réponse HTTP
```

**Création d'un produit :**

```
Étape 1: Handler reçoit une requête POST /produits
         │
         ▼
Étape 2: Handler valide le payload (validation métier)
         │
         ▼
Étape 3: Handler sérialise en JSON
         │
         ▼
Étape 4: Handler appelle km_client.create_entity("produits", data)
         │
         ▼
Étape 5: Client génère le token d'auth
         │
         ▼
Étape 6: Client envoie le WriteIntent au Core Server
         │
         ▼
Étape 7: Core Server arbitre (auth, perms WRITE, validation schema)
         │
         ▼
Étape 8: Core Server exécute INSERT dans libSQL (transaction)
         │
         ▼
Étape 9: Core Server retourne l'ID créé
         │
         ▼
Étape 10: Handler retourne 201 Created avec l'ID
```

---

## 7. Gestion des Erreurs Côté Client

### 7.1 Types d'erreurs

| Catégorie | Exemples | Action recommandée |
|-----------|----------|-------------------|
| **Réseau** | Timeout, connexion refusée | Retry automatique |
| **Auth** | Token invalide, expiré | Ré-authentification |
| **Permission** | Accès refusé | Log + erreur utilisateur |
| **Validation** | Données invalides | Erreur utilisateur avec détails |
| **Not Found** | Entité inexistante | Erreur 404 |
| **Quota** | Rate limit | Retry après délai |
| **Serveur** | Erreur interne | Log + retry limité |

### 7.2 Traduction des erreurs

Le Client traduit les erreurs serveur en erreurs applicatives :

```
┌────────────────────────────────────────────────────────────────────────────┐
│                    TRADUCTION DES ERREURS                                  │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│   Erreur Serveur              │        Erreur Client                       │
│   ─────────────────────────────┼──────────────────────────────────────────  │
│   UNAUTHENTICATED             │  ClientError::AuthFailed                   │
│   PERMISSION_DENIED           │  ClientError::PermissionDenied             │
│   INVALID_ARGUMENT            │  ClientError::ValidationError(details)     │
│   NOT_FOUND                   │  ClientError::NotFound                     │
│   ALREADY_EXISTS              │  ClientError::AlreadyExists                │
│   RESOURCE_EXHAUSTED          │  ClientError::QuotaExceeded(retry_after)   │
│   INTERNAL                    │  ClientError::ServerError                  │
│   UNAVAILABLE                 │  ClientError::ServiceUnavailable           │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 7.3 Stratégie de retry

```
                    Requête échoue
                          │
                          ▼
                ┌─────────────────┐
                │ Erreur réseau ? │──Oui──► Retry (backoff)
                └────────┬────────┘              │
                         │Non                    │
                         ▼                       │
                ┌─────────────────┐              │
                │    INTERNAL ?   │──Oui──► Retry (max 3)
                └────────┬────────┘              │
                         │Non                    │
                         ▼                       │
                ┌─────────────────┐              │
                │RESOURCE_EXHAUST?│──Oui──► Retry après délai
                └────────┬────────┘              │
                         │Non                    │
                         ▼                       │
                ┌─────────────────┐              │
                │  Propager       │◄─────────────┘
                │  l'erreur       │
                └─────────────────┘
```

---

## 8. Bonnes Pratiques

### 8.1 Configuration

| Pratique | Description |
|----------|-------------|
| **Client singleton** | Un seul client par service, partagé via `Arc<Mutex>` |
| **Connexion au démarrage** | Se connecter au boot, pas à la première requête |
| **Health check** | Vérifier la connexion périodiquement |
| **Graceful shutdown** | Fermer proprement le client à l'arrêt |

### 8.2 Performance

| Pratique | Description |
|----------|-------------|
| **Batch writes** | Grouper les écritures multiples en une transaction |
| **Pagination** | Toujours paginer les listes volumineuses |
| **Projection** | Ne demander que les champs nécessaires |
| **Cache applicatif** | Cacher les lectures fréquentes côté service |

### 8.3 Sécurité

| Pratique | Description |
|----------|-------------|
| **Ne jamais logger les tokens** | Les tokens sont des secrets |
| **Valider avant d'envoyer** | Validation métier côté service |
| **Ne pas exposer les erreurs internes** | Traduire pour l'utilisateur |

### 8.4 Résilience

| Pratique | Description |
|----------|-------------|
| **Timeout configuré** | Adapter aux SLA du service |
| **Circuit breaker** | Éviter de surcharger un serveur défaillant |
| **Mode dégradé** | Comportement défini si KindMother indisponible |

---

## 9. Diagnostics

### 9.1 Logs du Client

Le Client émet des logs structurés pour le diagnostic :

| Niveau | Événement | Exemple |
|--------|-----------|---------|
| **DEBUG** | Requête envoyée | `Sending read_entity to jayxpose.exposants` |
| **DEBUG** | Réponse reçue | `Received response in 5ms` |
| **INFO** | Connexion établie | `Connected to KindMother at [::1]:50051` |
| **WARN** | Retry en cours | `Retrying request (attempt 2/3)` |
| **ERROR** | Erreur finale | `Request failed: PERMISSION_DENIED` |

### 9.2 Métriques Client

| Métrique | Type | Description |
|----------|------|-------------|
| `km_client_requests_total` | Counter | Requêtes envoyées |
| `km_client_request_duration_ms` | Histogram | Latence vue par le client |
| `km_client_errors_total` | Counter | Erreurs par type |
| `km_client_retries_total` | Counter | Tentatives de retry |
| `km_client_connected` | Gauge | État de connexion (0/1) |

---

## 10. Invariants du Client

| ID | Invariant | Description |
|----|-----------|-------------|
| **INV-CLI-1** | Pas d'accès direct | Le client ne tente jamais d'accès direct aux fichiers |
| **INV-CLI-2** | Auth systématique | Chaque requête porte un token valide |
| **INV-CLI-3** | Contexte complet | Le client transmet toutes les informations nécessaires |
| **INV-CLI-4** | Erreurs typées | Les erreurs sont traduites, jamais propagées brutes |
| **INV-CLI-5** | Logs sans secrets | Aucun token ou donnée sensible dans les logs |

---

## 11. Références

- [KindMother - Core Server](./KindMother%20-%20Core%20Server.md)
- [KindMother - Documentation Fondatrice](../foundation/KindMother%20-%20Documentation%20Fondatrice.md)
- [Systeme Persistance libSQL Migration](../implementation/KindMother%20-%20Systeme%20Persistance%20libSQL%20Migration.md)
- [BondingBrother - Délégation](../../BondingBrother/)

---

**Date de création :** 2026-02-08  
**Version :** 1.0  
**Statut :** ARCHITECTURE — Document de référence  
**Auteur :** Architecture Miyukini
