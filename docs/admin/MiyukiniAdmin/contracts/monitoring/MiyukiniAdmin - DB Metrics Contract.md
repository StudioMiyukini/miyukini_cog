# MiyukiniAdmin â€” DB Metrics Contract

## 1. Contexte

Ce document definit le contrat de collecte et d'affichage des **metriques base de donnees** dans MiyukiniAdmin. Ces metriques couvrent les statistiques de requetes, la latence, l'etat du pool de connexions et la sante du SQL engine.

Les metriques DB sont collectees via CaringNanny et KindMother, en **lecture seule**.

## 2. Portee / Scope

Ce document definit :
- Les metriques DB collectees
- Les sources de donnees (CaringNanny, KindMother)
- Les frequences de collecte
- Les formats de representation
- Les seuils d'alerte specifiques DB

Ce document **ne couvre pas** :
- Les metriques systeme (voir Consumption Metrics Contract)
- Les operations de manipulation DB (voir DB Operations Contract)
- L'interface utilisateur (voir UI documentation)

---

## 3. Principe Fondamental

### 3.1 Lecture Passive

> **La collecte de metriques DB est une operation de lecture passive qui ne modifie jamais les donnees ni le schema.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-DBM-1** | Collecte en lecture seule uniquement |
| **INV-DBM-2** | Aucune modification de donnees ou schema |
| **INV-DBM-3** | Requetes de monitoring optimisees (pas de full scan) |
| **INV-DBM-4** | Toutes les metriques sont horodatees |

---

## 4. Metriques de Requetes

### 4.1 Metriques Collectees

| Metrique | Description | Unite |
|----------|-------------|-------|
| `db.queries.total` | Nombre total de requetes | Count |
| `db.queries.select` | Requetes SELECT | Count |
| `db.queries.insert` | Requetes INSERT | Count |
| `db.queries.update` | Requetes UPDATE | Count |
| `db.queries.delete` | Requetes DELETE | Count |
| `db.queries.rate` | Requetes par seconde | Queries/sec |
| `db.queries.slow` | Requetes lentes (> seuil) | Count |
| `db.queries.failed` | Requetes en echec | Count |

### 4.2 Source

- **Core source :** CaringNanny + KindMother
- **Capacite :** `admin.metrics.db`

### 4.3 Seuils d'Alerte

| Niveau | Slow Queries | Failed Queries | Action |
|--------|--------------|----------------|--------|
| **Normal** | < 1% | 0 | Aucune |
| **Warning** | 1-5% | 1-10 | Alerte visuelle |
| **Critical** | 5-10% | 10-50 | Alerte + Notification |
| **Emergency** | > 10% | > 50 | Alerte rouge + Log |

### 4.4 Format de Donnees

```json
{
  "timestamp": "2026-01-28T12:00:00Z",
  "queries": {
    "total": 15000,
    "by_type": {
      "select": 12000,
      "insert": 1500,
      "update": 1000,
      "delete": 500
    },
    "rate_per_sec": 250,
    "slow_count": 15,
    "slow_threshold_ms": 100,
    "failed_count": 2
  }
}
```

---

## 5. Metriques de Latence

### 5.1 Metriques Collectees

| Metrique | Description | Unite |
|----------|-------------|-------|
| `db.latency.avg` | Latence moyenne | Millisecondes |
| `db.latency.p50` | Percentile 50 | Millisecondes |
| `db.latency.p90` | Percentile 90 | Millisecondes |
| `db.latency.p95` | Percentile 95 | Millisecondes |
| `db.latency.p99` | Percentile 99 | Millisecondes |
| `db.latency.max` | Latence maximale | Millisecondes |
| `db.latency.min` | Latence minimale | Millisecondes |

### 5.2 Source

- **Core source :** CaringNanny + KindMother
- **Capacite :** `admin.metrics.db`

### 5.3 Seuils d'Alerte

| Niveau | P95 | P99 | Action |
|--------|-----|-----|--------|
| **Normal** | < 50ms | < 100ms | Aucune |
| **Warning** | 50-100ms | 100-200ms | Alerte visuelle |
| **Critical** | 100-200ms | 200-500ms | Alerte + Notification |
| **Emergency** | > 200ms | > 500ms | Alerte rouge + Log |

### 5.4 Format de Donnees

```json
{
  "timestamp": "2026-01-28T12:00:00Z",
  "latency": {
    "avg_ms": 15.5,
    "min_ms": 1.2,
    "max_ms": 350.8,
    "percentiles": {
      "p50": 12.0,
      "p90": 35.0,
      "p95": 45.0,
      "p99": 120.0
    }
  }
}
```

---

## 6. Metriques Pool de Connexions

### 6.1 Metriques Collectees

| Metrique | Description | Unite |
|----------|-------------|-------|
| `db.pool.size` | Taille du pool | Count |
| `db.pool.active` | Connexions actives | Count |
| `db.pool.idle` | Connexions inactives | Count |
| `db.pool.waiting` | Requetes en attente | Count |
| `db.pool.max` | Connexions max configurees | Count |
| `db.pool.usage` | Pourcentage d'utilisation | Pourcentage |
| `db.pool.wait_time` | Temps d'attente moyen | Millisecondes |
| `db.pool.timeouts` | Timeouts de connexion | Count |

### 6.2 Source

- **Core source :** KindMother
- **Capacite :** `admin.metrics.db`

### 6.3 Seuils d'Alerte

| Niveau | Usage Pool | Waiting | Timeouts | Action |
|--------|------------|---------|----------|--------|
| **Normal** | < 70% | 0 | 0 | Aucune |
| **Warning** | 70-85% | 1-5 | 1-5 | Alerte visuelle |
| **Critical** | 85-95% | 5-20 | 5-20 | Alerte + Notification |
| **Emergency** | > 95% | > 20 | > 20 | Alerte rouge + Log |

### 6.4 Format de Donnees

```json
{
  "timestamp": "2026-01-28T12:00:00Z",
  "pool": {
    "size": 20,
    "max": 50,
    "active": 15,
    "idle": 5,
    "waiting": 0,
    "usage_percent": 30.0,
    "avg_wait_time_ms": 0,
    "timeouts": 0
  }
}
```

---

## 7. Metriques Sante SQL Engine

### 7.1 Metriques Collectees

| Metrique | Description | Unite |
|----------|-------------|-------|
| `db.health.status` | Etat global | Enum |
| `db.health.uptime` | Temps de fonctionnement | Secondes |
| `db.health.version` | Version du moteur | String |
| `db.replication.lag` | Retard replication | Secondes |
| `db.replication.status` | Etat replication | Enum |
| `db.cache.hit_ratio` | Ratio cache hit | Pourcentage |
| `db.cache.size` | Taille cache | Bytes |
| `db.locks.active` | Verrous actifs | Count |
| `db.locks.waiting` | Verrous en attente | Count |
| `db.transactions.active` | Transactions actives | Count |
| `db.transactions.committed` | Transactions commitees | Count |
| `db.transactions.rollback` | Transactions annulees | Count |

### 7.2 Source

- **Core source :** KindMother
- **Capacite :** `admin.metrics.db`

### 7.3 Etats de Sante

| Etat | Description | Couleur |
|------|-------------|---------|
| **HEALTHY** | Fonctionnement normal | Vert |
| **DEGRADED** | Performance reduite | Jaune |
| **UNHEALTHY** | Problemes detectes | Orange |
| **CRITICAL** | Etat critique | Rouge |

### 7.4 Seuils d'Alerte

| Niveau | Cache Hit | Replication Lag | Locks Waiting | Action |
|--------|-----------|-----------------|---------------|--------|
| **Normal** | > 95% | < 1s | 0 | Aucune |
| **Warning** | 90-95% | 1-5s | 1-5 | Alerte visuelle |
| **Critical** | 80-90% | 5-30s | 5-20 | Alerte + Notification |
| **Emergency** | < 80% | > 30s | > 20 | Alerte rouge + Log |

### 7.5 Format de Donnees

```json
{
  "timestamp": "2026-01-28T12:00:00Z",
  "health": {
    "status": "HEALTHY",
    "uptime_seconds": 864000,
    "version": "PostgreSQL 15.2"
  },
  "replication": {
    "enabled": true,
    "status": "STREAMING",
    "lag_seconds": 0.5
  },
  "cache": {
    "hit_ratio": 98.5,
    "size_bytes": 1073741824
  },
  "locks": {
    "active": 5,
    "waiting": 0
  },
  "transactions": {
    "active": 10,
    "committed_total": 1500000,
    "rollback_total": 150
  }
}
```

---

## 8. Metriques Taille et Croissance

### 8.1 Metriques Collectees

| Metrique | Description | Unite |
|----------|-------------|-------|
| `db.size.total` | Taille totale DB | Bytes |
| `db.size.tables` | Taille des tables | Bytes |
| `db.size.indexes` | Taille des index | Bytes |
| `db.growth.daily` | Croissance quotidienne | Bytes |
| `db.growth.weekly` | Croissance hebdomadaire | Bytes |
| `db.tables.count` | Nombre de tables | Count |
| `db.rows.total` | Nombre total de lignes | Count |

### 8.2 Format de Donnees

```json
{
  "timestamp": "2026-01-28T12:00:00Z",
  "size": {
    "total_bytes": 10737418240,
    "tables_bytes": 8589934592,
    "indexes_bytes": 2147483648
  },
  "growth": {
    "daily_bytes": 104857600,
    "weekly_bytes": 734003200
  },
  "counts": {
    "tables": 45,
    "total_rows": 15000000
  }
}
```

---

## 9. Top Requetes

### 9.1 Metriques Collectees

| Metrique | Description |
|----------|-------------|
| `db.top.slowest` | 10 requetes les plus lentes |
| `db.top.frequent` | 10 requetes les plus frequentes |
| `db.top.consuming` | 10 requetes les plus consommatrices |

### 9.2 Format de Donnees

```json
{
  "timestamp": "2026-01-28T12:00:00Z",
  "top_queries": {
    "slowest": [
      {
        "query_hash": "abc123",
        "avg_time_ms": 250.5,
        "calls": 150,
        "table": "orders"
      }
    ],
    "most_frequent": [
      {
        "query_hash": "def456",
        "calls": 50000,
        "avg_time_ms": 5.2,
        "table": "users"
      }
    ]
  }
}
```

---

## 10. Frequences de Collecte

### 10.1 Par Type de Metrique

| Type | Frequence Standard | Frequence Realtime |
|------|-------------------|-------------------|
| Queries stats | 5 secondes | 1 seconde |
| Latency | 5 secondes | 1 seconde |
| Pool | 10 secondes | 5 secondes |
| Health | 30 secondes | 10 secondes |
| Size/Growth | 5 minutes | 1 minute |
| Top queries | 1 minute | 30 secondes |

---

## 11. Integration UI (PHPMyAdmin Style)

### 11.1 Widgets Dashboard DB

| Widget | Metriques | Type affichage |
|--------|-----------|----------------|
| **Query Rate** | queries.rate | Graphique ligne temps reel |
| **Latency Distribution** | latency.percentiles | Histogramme |
| **Pool Status** | pool.* | Jauge + indicateurs |
| **Health Status** | health.status | Indicateur couleur |
| **Slow Queries** | queries.slow | Liste avec details |
| **Size Chart** | size.* | Graphique secteurs |

### 11.2 Vue Tables (Style PHPMyAdmin)

| Colonne | Description |
|---------|-------------|
| **Table** | Nom de la table |
| **Rows** | Nombre de lignes |
| **Size** | Taille en MB |
| **Index Size** | Taille index |
| **Growth** | Croissance recente |
| **Actions** | Voir, Stats, Analyze |

---

## 12. Flux de Collecte

```
MiyukiniAdmin                 BondingBrother         CaringNanny    KindMother
     â”‚                              â”‚                     â”‚             â”‚
     â”‚â”€â”€DBMetricsRequestâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚                     â”‚             â”‚
     â”‚  (type: db_metrics)           â”‚                     â”‚             â”‚
     â”‚                              â”‚                     â”‚             â”‚
     â”‚                              â”‚â”€â”€ReadSystemMetricsâ”€â”€â–¶â”‚             â”‚
     â”‚                              â”‚                     â”‚             â”‚
     â”‚                              â”‚â—€â”€SystemDataâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚             â”‚
     â”‚                              â”‚                     â”‚             â”‚
     â”‚                              â”‚â”€â”€ReadDBMetricsâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                              â”‚                     â”‚             â”‚
     â”‚                              â”‚â—€â”€DBDataâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                              â”‚                     â”‚             â”‚
     â”‚â—€â”€DBMetricsResponseâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚                     â”‚             â”‚
     â”‚  (queries, latency, pool,    â”‚                     â”‚             â”‚
     â”‚   health, size)              â”‚                     â”‚             â”‚
     â”‚                              â”‚                     â”‚             â”‚
```

---

## 13. Implications Securite

### 13.1 Prevention des Injections SQL

Les requetes de collecte de metriques DB sont des vecteurs potentiels d'injection SQL. Cette section definit les controles obligatoires.

| Risque | Severite | Mitigation |
|--------|----------|------------|
| **SQL Injection via filtres** | CRITIQUE | Requetes preparees uniquement, aucune concatenation |
| **SQL Injection via noms de tables** | HAUTE | Whitelist de tables autorisees |
| **SQL Injection via colonnes** | HAUTE | Whitelist de colonnes par table |
| **Second Order Injection** | MOYENNE | Validation des donnees retournees |

### 13.2 Requetes Securisees

> **INV-DBM-SEC-1 : Toute requete de metriques DB utilise des requetes preparees.**

```
INTERDIT :
  "SELECT * FROM " + tableName + " WHERE " + filter

OBLIGATOIRE :
  Requete preparee avec parametres valides par whitelist
```

| Composant | Controle |
|-----------|----------|
| **Noms de tables** | Validation contre schema KindMother |
| **Noms de colonnes** | Validation contre schema table |
| **Valeurs de filtres** | Parametres prepares, echappement automatique |
| **Operateurs** | Whitelist : `=`, `>`, `<`, `>=`, `<=`, `LIKE`, `IN` |

### 13.3 Exposition des Metriques DB

| Risque | Description | Mitigation |
|--------|-------------|------------|
| **Schema Disclosure** | Les metriques revelent la structure DB | Acces `admin.metrics.db` requis |
| **Query Pattern Disclosure** | Les slow queries revelent les requetes | Hash des requetes, pas de texte brut |
| **Performance Profiling** | Aide a identifier les points d'attaque | Metriques non publiques |

### 13.4 Controles d'Acces DB Metrics

| Controle | Implementation |
|----------|----------------|
| **Authentification** | Session MiyukiniAdmin + StrongFather |
| **Autorisation** | Capacite `admin.metrics.db` requise |
| **Segregation** | KindMother valide l'acces aux tables |
| **Audit** | Toute consultation tracee avec query hash |

### 13.5 Top Queries â€” Securite

Les "Top Queries" (section 9) presentent des risques specifiques :

| Donnee | Exposition | Protection |
|--------|------------|------------|
| **query_hash** | Expose | Hash irreversible uniquement |
| **Texte requete** | NON EXPOSE | Jamais affiche dans les metriques |
| **Tables concernees** | Expose | Si capacite `admin.metrics.db.detailed` |
| **Parametres** | NON EXPOSE | Jamais inclus dans les metriques |

### 13.6 Adaptation par Niveau de Confiance (T0-T4)

| Niveau | Comportement Metriques DB |
|--------|---------------------------|
| **T0** | Toutes metriques accessibles |
| **T1** | Idem T0, surveillance accrue des slow queries |
| **T2** | Top queries desactive, metriques agregees |
| **T3** | Metriques pool et health uniquement |
| **T4** | Uniquement health.status |

### 13.7 Adaptation par Niveau de Securite (0-4)

| Niveau | Restrictions |
|--------|--------------|
| **0-1** | Toutes metriques DB accessibles |
| **2** | Top queries avec hash uniquement |
| **3** | Metriques agregees, pas de details par table |
| **4** | Health et pool uniquement, audit complet |

### 13.8 References Securite

- [Security - Core Integration Map](..//..//..//..//cores//WorrySentinel//_index.md)
- [Security - Documentation Fondatrice](..//..//..//..//cores//WorrySentinel//_index.md)
- [KindMother - Security Contract](..//..//..//..//_index.md)

---

## 14. Documents Associes

- [MiyukiniAdmin - Consumption Metrics Contract](./MiyukiniAdmin%20-%20Consumption%20Metrics%20Contract.md)
- [MiyukiniAdmin - DB Operations Contract](../database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md)
- [MiyukiniAdmin - DB Management Interface](../../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md)
- [KindMother - Documentation Fondatrice](..//..//..//..//cores//KindMother//foundation//KindMother%20-%20Documentation%20Fondatrice.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Contrat de reference



