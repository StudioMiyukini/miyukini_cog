# KindMother — Core Server

## 1. Introduction

### Objet du document

Ce document décrit l'architecture et le fonctionnement du **KindMother Core Server** — le processus isolé qui détient l'autorité exclusive sur la persistance des données dans l'écosystème Miyukini COG.

### Contexte

Le Core Server est la matérialisation technique du principe fondamental :
> **"Les Cores gouvernent, jamais n'exécutent directement."**

En isolant KindMother dans un processus séparé, nous garantissons que la gouvernance des données n'est pas une simple convention de code mais une **réalité technique incontournable**.

### Portée

Ce document couvre :
- Architecture interne du serveur
- Mécaniques d'arbitrage et de gouvernance
- Gestion des bases de données multiples
- Cycle de vie des requêtes
- Observabilité et audit

### Prérequis

- [KindMother - Documentation Fondatrice](../foundation/KindMother%20-%20Documentation%20Fondatrice.md)
- [Security - Gouvernance Cores Protection Donnees](../../../security/foundation/Security%20-%20Gouvernance%20Cores%20Protection%20Donnees.md)

### Ce document ne couvre PAS

- Détails d'implémentation Rust (voir [Systeme Persistance libSQL Migration](../implementation/KindMother%20-%20Systeme%20Persistance%20libSQL%20Migration.md))
- Configuration et déploiement (voir document Migration)
- API client (voir [KindMother - Client](./KindMother%20-%20Client.md))

---

## 2. Architecture du Core Server

### 2.1 Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        KINDMOTHER CORE SERVER                               │
│                     (Processus isolé, autorité exclusive)                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                      COUCHE TRANSPORT (IPC)                           │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                   │  │
│  │  │ gRPC Server │  │ Unix Socket │  │ Named Pipe  │                   │  │
│  │  │   (tonic)   │  │   (Linux)   │  │  (Windows)  │                   │  │
│  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘                   │  │
│  │         └────────────────┼────────────────┘                          │  │
│  └──────────────────────────┼───────────────────────────────────────────┘  │
│                             ▼                                               │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                      COUCHE ARBITRAGE                                 │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │  │
│  │  │    Auth     │  │ Permission  │  │  Validation │  │   Quota     │  │  │
│  │  │  Validator  │→ │   Engine    │→ │   Engine    │→ │  Manager    │  │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘  │  │
│  └──────────────────────────┼───────────────────────────────────────────┘  │
│                             ▼                                               │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                      COUCHE ORCHESTRATION                             │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                   │  │
│  │  │  Database   │  │  WriteIntent│  │    Sync     │                   │  │
│  │  │   Router    │  │   Handler   │  │ Coordinator │                   │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘                   │  │
│  └──────────────────────────┼───────────────────────────────────────────┘  │
│                             ▼                                               │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                      COUCHE PERSISTANCE                               │  │
│  │  ┌─────────────────────────────────────────────────────────────────┐  │  │
│  │  │                    libSQL Engine                                │  │  │
│  │  │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐    │  │  │
│  │  │  │ jayxpose  │  │ jaykonta  │  │jayfestival│  │   ...     │    │  │  │
│  │  │  │    .db    │  │    .db    │  │    .db    │  │           │    │  │  │
│  │  │  └───────────┘  └───────────┘  └───────────┘  └───────────┘    │  │  │
│  │  │                     [Chiffrement AES-256-GCM]                   │  │  │
│  │  └─────────────────────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                      COUCHE OBSERVABILITÉ                             │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                   │  │
│  │  │   Metrics   │  │   Audit     │  │    Health   │                   │  │
│  │  │  Collector  │  │    Log      │  │   Monitor   │                   │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘                   │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Principes architecturaux

| Principe | Description | Garantie |
|----------|-------------|----------|
| **Isolation totale** | Processus séparé, mémoire isolée | Aucun accès direct aux données |
| **Autorité exclusive** | Seul propriétaire des fichiers DB | Permissions fichier `600` |
| **Chiffrement souverain** | Clé dérivée localement | Fichiers illisibles sans clé |
| **Arbitrage systématique** | Chaque requête validée | Aucun contournement possible |

---

## 3. Mécaniques d'Arbitrage

L'arbitrage est le processus par lequel le Core Server **valide, autorise et gouverne** chaque opération de données. C'est le coeur de la gouvernance KindMother.

### 3.1 Pipeline d'arbitrage

Chaque requête traverse un pipeline de validation en 4 étapes :

```
┌──────────────────────────────────────────────────────────────────────────┐
│                       PIPELINE D'ARBITRAGE                               │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Requête    ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐    │
│  entrante → │  AUTH   │ → │  PERMS  │ → │ VALIDA- │ → │  QUOTA  │ →   │
│             │         │    │         │    │  TION   │    │         │    │
│             └────┬────┘    └────┬────┘    └────┬────┘    └────┬────┘    │
│                  │              │              │              │         │
│             ┌────▼────┐    ┌────▼────┐    ┌────▼────┐    ┌────▼────┐    │
│             │ Token   │    │ Matrice │    │ Schema  │    │ Rate    │    │
│             │ Verify  │    │ Accès   │    │ Check   │    │ Limit   │    │
│             └─────────┘    └─────────┘    └─────────┘    └─────────┘    │
│                                                                          │
│  ❌ Rejet à n'importe quelle étape = Requête refusée                    │
│  ✅ Toutes les étapes passées = Exécution autorisée                     │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Étape 1 : Authentification (Auth Validator)

**Rôle** : Vérifier l'identité de l'appelant et la validité du token.

| Vérification | Description | Échec = |
|--------------|-------------|---------|
| Token présent | Le token d'auth est fourni | `UNAUTHENTICATED` |
| Signature valide | HMAC-SHA256 correct | `INVALID_TOKEN` |
| Non expiré | Timestamp dans la fenêtre | `TOKEN_EXPIRED` |
| Non rejoué | Request ID unique | `REPLAY_DETECTED` |

**Structure du token** :

```
┌────────────────────────────────────────────────────────┐
│                     AUTH TOKEN                         │
├────────────────────────────────────────────────────────┤
│  operator_id   │ "jayxpose"                            │
│  request_id    │ "uuid-unique-par-requête"             │
│  timestamp     │ 1707350400 (Unix epoch)               │
│  signature     │ HMAC-SHA256(payload, shared_secret)   │
└────────────────────────────────────────────────────────┘
```

**Fenêtre temporelle** : ±5 minutes (configurable) pour compenser les dérives d'horloge locales.

### 3.3 Étape 2 : Permissions (Permission Engine)

**Rôle** : Vérifier que l'opérateur a le droit d'effectuer l'opération demandée.

#### Matrice de permissions

La matrice définit les accès par **Opérateur** × **Base** × **Table** × **Opération** :

```
┌───────────────────────────────────────────────────────────────────────────┐
│                    MATRICE DE PERMISSIONS                                 │
├────────────────┬───────────────┬─────────────────────────────────────────┤
│    Opérateur   │     Base      │           Permissions                    │
├────────────────┼───────────────┼─────────────────────────────────────────┤
│   jayxpose     │   jayxpose    │ exposants: CRUD                         │
│                │               │ produits: CRUD                          │
│                │               │ vitrines: CRUD                          │
│                │               │ documents: CRUD                         │
│                │               │ cms_articles: CRUD                      │
├────────────────┼───────────────┼─────────────────────────────────────────┤
│   jaykonta     │   jaykonta    │ comptes: CRUD                           │
│                │               │ transactions: CR                        │
│                │               │ rapports: R                             │
├────────────────┼───────────────┼─────────────────────────────────────────┤
│   jayfestival  │  jayfestival  │ evenements: CRUD                        │
│                │               │ participants: CRUD                      │
├────────────────┼───────────────┼─────────────────────────────────────────┤
│ miyukiniadmin  │     TOUTES    │ TOUTES: CRUD + ADMIN                    │
├────────────────┼───────────────┼─────────────────────────────────────────┤
│   jayxpose     │   jaykonta    │ ❌ AUCUN ACCÈS                          │
│   jaykonta     │   jayxpose    │ ❌ AUCUN ACCÈS                          │
└────────────────┴───────────────┴─────────────────────────────────────────┘
```

#### Opérations

| Code | Signification | Description |
|------|---------------|-------------|
| **C** | Create | Insertion de nouvelles entités |
| **R** | Read | Lecture d'entités |
| **U** | Update | Modification d'entités existantes |
| **D** | Delete | Suppression d'entités |
| **ADMIN** | Administration | Opérations de maintenance (vacuum, reindex, etc.) |

#### Règles d'isolation

| Règle | Description |
|-------|-------------|
| **ISO-1** | Un Opérateur ne peut accéder qu'à ses propres bases |
| **ISO-2** | L'accès inter-bases est interdit sauf pour MiyukiniAdmin |
| **ISO-3** | Les permissions sont définies au démarrage, non modifiables à runtime |

### 3.4 Étape 3 : Validation (Validation Engine)

**Rôle** : Vérifier la cohérence et l'intégrité de la requête.

| Validation | Description | Exemple de rejet |
|------------|-------------|------------------|
| **Schema** | La requête respecte le schéma attendu | Champ requis manquant |
| **Types** | Les types de données sont corrects | String au lieu de Integer |
| **Références** | Les clés étrangères existent | `exposant_id` inexistant |
| **Contraintes** | Les contraintes métier sont respectées | Prix négatif |
| **Taille** | Les données ne dépassent pas les limites | Texte > 65535 chars |

#### Validation SQL

Pour les requêtes SQL directes (si autorisées), le Validation Engine applique :

| Contrôle | Description |
|----------|-------------|
| **Whitelist tables** | Seules les tables autorisées sont accessibles |
| **Blacklist keywords** | `DROP`, `TRUNCATE`, `ALTER` interdits |
| **Paramètres liés** | Pas de concaténation SQL (anti-injection) |
| **Limite résultats** | Maximum 10 000 lignes par requête |

### 3.5 Étape 4 : Quotas (Quota Manager)

**Rôle** : Protéger le système contre les abus et surcharges.

| Quota | Limite par défaut | Description |
|-------|-------------------|-------------|
| **Requêtes/minute** | 1000 | Par opérateur |
| **Écritures/minute** | 100 | Par opérateur |
| **Taille payload** | 10 MB | Par requête |
| **Connexions simultanées** | 50 | Par opérateur |

#### Réponses en cas de dépassement

| Situation | Réponse |
|-----------|---------|
| Limite atteinte | `RESOURCE_EXHAUSTED` + délai retry |
| Abus détecté | Blocage temporaire (1-60 min) |
| Attaque suspectée | Alerte WorrySentinel |

---

## 4. Gestion Multi-Bases

Le Core Server gère plusieurs bases de données simultanément, une par Opérateur.

### 4.1 Architecture multi-bases

```
┌────────────────────────────────────────────────────────────────────┐
│                    DATABASE ROUTER                                 │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│   Requête                   ┌──────────────────┐                   │
│   (database: "jayxpose") → │  Route Resolver  │                   │
│                             └────────┬─────────┘                   │
│                                      │                             │
│        ┌─────────────────────────────┼─────────────────────────┐   │
│        ▼                             ▼                         ▼   │
│   ┌─────────┐               ┌─────────────┐            ┌─────────┐ │
│   │jayxpose │               │  jaykonta   │            │festival │ │
│   │Connection               │ Connection  │            │Connection│ │
│   │   Pool  │               │    Pool     │            │   Pool  │ │
│   └────┬────┘               └──────┬──────┘            └────┬────┘ │
│        │                           │                        │      │
│   ┌────▼────┐               ┌──────▼──────┐            ┌────▼────┐ │
│   │jayxpose │               │  jaykonta   │            │festival │ │
│   │   .db   │               │     .db     │            │   .db   │ │
│   │(chiffré)│               │  (chiffré)  │            │(chiffré)│ │
│   └─────────┘               └─────────────┘            └─────────┘ │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

### 4.2 Pool de connexions

Chaque base dispose de son propre pool de connexions :

| Paramètre | Valeur | Description |
|-----------|--------|-------------|
| **min_connections** | 2 | Connexions maintenues à froid |
| **max_connections** | 10 | Maximum simultané |
| **connection_timeout** | 30s | Attente max pour obtenir une connexion |
| **idle_timeout** | 300s | Fermeture après inactivité |

### 4.3 Isolation des bases

| Garantie | Mécanisme |
|----------|-----------|
| **Isolation fichier** | Chaque base = fichier séparé |
| **Isolation connexion** | Pool dédié par base |
| **Isolation transaction** | Pas de transaction cross-base |
| **Isolation erreur** | Erreur sur une base n'affecte pas les autres |

---

## 5. Cycle de Vie des Requêtes

### 5.1 Requête de lecture (Read)

```
┌────────────────────────────────────────────────────────────────────────────┐
│                     CYCLE DE VIE : LECTURE                                 │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  1. RÉCEPTION                                                              │
│     Client → IPC → Core Server                                             │
│     Parse request, extract auth token                                      │
│                                                                            │
│  2. ARBITRAGE                                                              │
│     Auth → Permissions → Validation → Quota                                │
│     Tout OK ? Continue : Reject                                            │
│                                                                            │
│  3. ROUTAGE                                                                │
│     Database Router → Select correct DB                                    │
│     Connection Pool → Acquire connection                                   │
│                                                                            │
│  4. EXÉCUTION                                                              │
│     libSQL → Execute query                                                 │
│     Decrypt data (transparent)                                             │
│                                                                            │
│  5. RÉPONSE                                                                │
│     Serialize result → IPC → Client                                        │
│     Audit log (async)                                                      │
│                                                                            │
│  Latence typique : 1-5 ms (local)                                          │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 5.2 Requête d'écriture (Write)

```
┌────────────────────────────────────────────────────────────────────────────┐
│                     CYCLE DE VIE : ÉCRITURE                                │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  1. RÉCEPTION                                                              │
│     Client → IPC → Core Server                                             │
│     Parse WriteIntent                                                      │
│                                                                            │
│  2. ARBITRAGE                                                              │
│     Auth → Permissions (write) → Validation → Quota                        │
│     Vérifications supplémentaires pour écritures                           │
│                                                                            │
│  3. VALIDATION AVANCÉE                                                     │
│     Schema validation                                                      │
│     Foreign key check                                                      │
│     Business constraints                                                   │
│                                                                            │
│  4. TRANSACTION                                                            │
│     BEGIN TRANSACTION                                                      │
│     Execute write(s)                                                       │
│     COMMIT (ou ROLLBACK si erreur)                                         │
│                                                                            │
│  5. CONFIRMATION                                                           │
│     Result (id, affected rows) → Client                                    │
│     Audit log (sync for writes)                                            │
│                                                                            │
│  Latence typique : 5-20 ms (local)                                         │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 5.3 États d'une requête

```
                    ┌─────────┐
                    │ PENDING │
                    └────┬────┘
                         │
              ┌──────────▼──────────┐
              │     VALIDATING      │
              └──────────┬──────────┘
                         │
         ┌───────────────┼───────────────┐
         ▼               ▼               ▼
    ┌─────────┐    ┌──────────┐    ┌──────────┐
    │REJECTED │    │ EXECUTING│    │ QUEUED   │
    └─────────┘    └────┬─────┘    └────┬─────┘
                        │               │
                        └───────┬───────┘
                                ▼
                    ┌───────────────────┐
                    │    COMPLETED      │
                    │  (success/error)  │
                    └───────────────────┘
```

---

## 6. Observabilité

### 6.1 Métriques collectées

| Catégorie | Métrique | Type | Description |
|-----------|----------|------|-------------|
| **Requêtes** | `km_requests_total` | Counter | Total requêtes par opérateur/type |
| | `km_request_duration_ms` | Histogram | Latence des requêtes |
| | `km_request_errors` | Counter | Erreurs par type |
| **Arbitrage** | `km_auth_failures` | Counter | Échecs d'authentification |
| | `km_permission_denials` | Counter | Refus de permission |
| | `km_validation_errors` | Counter | Erreurs de validation |
| **Base** | `km_db_connections` | Gauge | Connexions actives par base |
| | `km_db_queries_total` | Counter | Requêtes SQL exécutées |
| | `km_db_size_bytes` | Gauge | Taille des fichiers DB |
| **Système** | `km_memory_bytes` | Gauge | Mémoire utilisée |
| | `km_uptime_seconds` | Counter | Temps depuis démarrage |

### 6.2 Audit Log

Chaque opération est journalisée dans un log d'audit structuré :

```json
{
  "timestamp": "2026-02-08T14:30:00Z",
  "request_id": "uuid-xxx",
  "operator": "jayxpose",
  "operation": "write",
  "database": "jayxpose",
  "table": "exposants",
  "entity_id": "exp-123",
  "result": "success",
  "duration_ms": 12,
  "arbitrage": {
    "auth": "pass",
    "permission": "pass",
    "validation": "pass",
    "quota": "pass"
  }
}
```

### 6.3 Health Check

Le Core Server expose un endpoint de santé :

| Check | Description | Sain si |
|-------|-------------|---------|
| **alive** | Processus en cours | Toujours vrai si répond |
| **ready** | Prêt à servir | Toutes les bases ouvertes |
| **db_health** | Chaque base accessible | Requête `SELECT 1` réussit |

---

## 7. Gestion des Erreurs

### 7.1 Codes d'erreur

| Code | Signification | Action client |
|------|---------------|---------------|
| `OK` | Succès | - |
| `UNAUTHENTICATED` | Token manquant/invalide | Ré-authentifier |
| `PERMISSION_DENIED` | Pas le droit | Vérifier permissions |
| `INVALID_ARGUMENT` | Données invalides | Corriger payload |
| `NOT_FOUND` | Entité inexistante | - |
| `ALREADY_EXISTS` | Duplication | - |
| `RESOURCE_EXHAUSTED` | Quota dépassé | Retry après délai |
| `INTERNAL` | Erreur serveur | Reporter le bug |
| `UNAVAILABLE` | Service temporairement indisponible | Retry |

### 7.2 Retry Policy

| Erreur | Retry ? | Stratégie |
|--------|---------|-----------|
| `UNAUTHENTICATED` | Non | Ré-authentification nécessaire |
| `PERMISSION_DENIED` | Non | Configuration nécessaire |
| `INVALID_ARGUMENT` | Non | Correction payload nécessaire |
| `NOT_FOUND` | Non | - |
| `RESOURCE_EXHAUSTED` | Oui | Backoff exponentiel |
| `INTERNAL` | Oui | Max 3 tentatives |
| `UNAVAILABLE` | Oui | Backoff exponentiel |

---

## 8. Intégration avec les Autres Cores

### 8.1 StrongFather

| Interaction | Direction | Description |
|-------------|-----------|-------------|
| Validation intention | SF → KM | StrongFather valide l'intention avant écriture |
| Résultat persistance | KM → SF | KindMother confirme la persistance |

### 8.2 WorrySentinel

| Interaction | Direction | Description |
|-------------|-----------|-------------|
| Alerte sécurité | KM → WS | Détection de patterns suspects |
| Révocation mandat | WS → KM | Blocage d'un opérateur compromis |

### 8.3 Caring Nanny

| Interaction | Direction | Description |
|-------------|-----------|-------------|
| Métriques santé | KM → CN | État du Core Server |
| Détection anomalie | CN → KM | Alerte sur patterns anormaux |

---

## 9. Invariants du Core Server

Ces invariants sont **non négociables** et garantis par l'architecture :

| ID | Invariant | Violation = |
|----|-----------|-------------|
| **INV-SRV-1** | Toute requête passe par le pipeline d'arbitrage | Faille de gouvernance |
| **INV-SRV-2** | Aucun accès direct aux fichiers DB | Contournement Core |
| **INV-SRV-3** | La clé de chiffrement reste en mémoire | Fuite de secret |
| **INV-SRV-4** | Chaque opération est auditée | Perte de traçabilité |
| **INV-SRV-5** | Les erreurs ne révèlent pas les données | Fuite d'information |
| **INV-SRV-6** | Un opérateur n'accède qu'à ses bases | Violation isolation |

---

## 10. Références

- [KindMother - Documentation Fondatrice](../foundation/KindMother%20-%20Documentation%20Fondatrice.md)
- [KindMother - Client](./KindMother%20-%20Client.md)
- [Systeme Persistance libSQL Migration](../implementation/KindMother%20-%20Systeme%20Persistance%20libSQL%20Migration.md)
- [Security - Gouvernance Cores Protection Donnees](../../../security/foundation/Security%20-%20Gouvernance%20Cores%20Protection%20Donnees.md)

---

**Date de création :** 2026-02-08  
**Version :** 1.0  
**Statut :** ARCHITECTURE — Document de référence  
**Auteur :** Architecture Miyukini
