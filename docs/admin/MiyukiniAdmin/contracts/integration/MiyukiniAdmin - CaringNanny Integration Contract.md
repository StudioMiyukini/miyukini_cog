# MiyukiniAdmin â€” CaringNanny Integration Contract

## 1. Contexte

Ce document definit le contrat d'integration entre MiyukiniAdmin et **CaringNanny**. CaringNanny est l'observateur d'etat du systeme qui expose les metriques et la sante globale.

## 2. Portee / Scope

Ce document definit :
- Le role de CaringNanny pour MiyukiniAdmin
- Les metriques exposees
- Les protocoles d'observation
- Les notifications d'etat

Ce document **ne couvre pas** :
- L'implementation interne de CaringNanny
- Les autres integrations cores

---

## 3. Role de CaringNanny

### 3.1 Observateur d'Etat

> **CaringNanny observe l'etat du systeme sans le modifier. MiyukiniAdmin utilise CaringNanny pour le monitoring et la supervision.**

**Question fondamentale :**
> "Dans quel etat se trouve le systeme ?"

### 3.2 Responsabilites

| Responsabilite | Description |
|----------------|-------------|
| **Observation** | Collecte des metriques systeme |
| **Etat Operateurs** | Suivi des Operateurs actifs |
| **Sante globale** | Evaluation de la sante |
| **Niveaux de confiance** | Suivi T0-T4 |

---

## 4. Metriques Exposees

### 4.1 Metriques Systeme

| Metrique | Capacite | Description |
|----------|----------|-------------|
| CPU | `admin.metrics.system` | Utilisation CPU |
| RAM | `admin.metrics.system` | Utilisation memoire |
| Disque | `admin.metrics.system` | Utilisation stockage |
| Reseau | `admin.metrics.system` | I/O reseau |
| Load | `admin.metrics.system` | Charge systeme |

### 4.2 Metriques Operateurs

| Metrique | Capacite | Description |
|----------|----------|-------------|
| Liste | `admin.metrics.operators` | Operateurs actifs |
| Etat | `admin.metrics.operators` | Etat par Operateur |
| Charge | `admin.metrics.operators` | Charge par Operateur |
| Erreurs | `admin.metrics.operators` | Erreurs par Operateur |

### 4.3 Metriques Sante

| Metrique | Capacite | Description |
|----------|----------|-------------|
| Health score | `admin.metrics.health` | Score global 0-100 |
| Trust level | `admin.metrics.health` | Niveau T0-T4 |
| Alerts | `admin.metrics.health` | Alertes actives |
| Degradations | `admin.metrics.health` | Modes degradation |

### 4.4 Metriques Performance

| Metrique | Capacite | Description |
|----------|----------|-------------|
| Latence SF | `admin.metrics.latency` | Latence StrongFather |
| Latence KM | `admin.metrics.latency` | Latence KindMother |
| Latence BB | `admin.metrics.latency` | Latence BondingBrother |
| Latence E2E | `admin.metrics.latency` | Bout-en-bout |

---

## 5. Protocole d'Observation

### 5.1 Format de Requete

```json
{
  "request_id": "uuid-request-001",
  "timestamp": "2026-01-28T12:00:00Z",
  "observation_type": "SYSTEM_METRICS",
  "parameters": {
    "include": ["cpu", "ram", "disk", "network"],
    "interval": "5s"
  },
  "context": {
    "source": "miyukini_admin",
    "operator_id": "uuid-operator"
  }
}
```

### 5.2 Format de Reponse

```json
{
  "request_id": "uuid-request-001",
  "timestamp": "2026-01-28T12:00:01Z",
  "status": "SUCCESS",
  "observations": {
    "cpu": {
      "usage_percent": 45.2,
      "cores": 8,
      "load_1m": 2.4
    },
    "ram": {
      "usage_percent": 60.1,
      "total_bytes": 17179869184,
      "available_bytes": 6871947673
    },
    "disk": {
      "usage_percent": 55.0,
      "read_bytes_sec": 1048576,
      "write_bytes_sec": 524288
    },
    "network": {
      "rx_bytes_sec": 1048576,
      "tx_bytes_sec": 524288,
      "connections_active": 150
    }
  },
  "metadata": {
    "collection_time_ms": 15,
    "source": "CaringNanny"
  }
}
```

---

## 6. Flux d'Observation

### 6.1 Flux Metriques Temps Reel

```
MiyukiniAdmin           BondingBrother              CaringNanny
     â”‚                        â”‚                          â”‚
     â”‚â”€â”€MetricsRequestâ”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚                          â”‚
     â”‚  (type: realtime)       â”‚                          â”‚
     â”‚  (interval: 1s)         â”‚                          â”‚
     â”‚                        â”‚                          â”‚
     â”‚                        â”‚â”€â”€ObserveMetricsâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                        â”‚                          â”‚
     â”‚                        â”‚â—€â”€MetricsStreamâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                        â”‚  (continuous)             â”‚
     â”‚â—€â”€MetricsResponseâ”€â”€â”€â”€â”€â”€â”€â”‚                          â”‚
     â”‚  (stream)               â”‚                          â”‚
     â”‚                        â”‚                          â”‚
     â”‚  [... updates every 1s ...]                       â”‚
```

### 6.2 Flux Sante Globale

```
MiyukiniAdmin           BondingBrother              CaringNanny
     â”‚                        â”‚                          â”‚
     â”‚â”€â”€HealthRequestâ”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚                          â”‚
     â”‚                        â”‚                          â”‚
     â”‚                        â”‚â”€â”€GetSystemHealthâ”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                        â”‚                          â”‚
     â”‚                        â”‚â—€â”€HealthReportâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                        â”‚                          â”‚
     â”‚â—€â”€HealthResponseâ”€â”€â”€â”€â”€â”€â”€â”€â”‚                          â”‚
     â”‚  (score, level, alerts) â”‚                          â”‚
```

### 6.3 Flux Etat Operateurs

```
MiyukiniAdmin           BondingBrother              CaringNanny
     â”‚                        â”‚                          â”‚
     â”‚â”€â”€OperatorsRequestâ”€â”€â”€â”€â”€â”€â–¶â”‚                          â”‚
     â”‚                        â”‚                          â”‚
     â”‚                        â”‚â”€â”€GetOperatorsStateâ”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                        â”‚                          â”‚
     â”‚                        â”‚â—€â”€OperatorsStateâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                        â”‚                          â”‚
     â”‚â—€â”€OperatorsResponseâ”€â”€â”€â”€â”€â”‚                          â”‚
     â”‚  (list with states)     â”‚                          â”‚
```

---

## 7. Niveaux de Confiance (Trust Levels)

### 7.1 Definition

| Niveau | Nom | Description |
|--------|-----|-------------|
| **T0** | Normal | Fonctionnement optimal |
| **T1** | Attention | Anomalies mineures |
| **T2** | Degrade | Performance reduite |
| **T3** | Critique | Intervention requise |
| **T4** | Urgence | Mode survie |

### 7.2 Format

```json
{
  "trust_level": {
    "current": "T0",
    "since": "2026-01-28T10:00:00Z",
    "reason": "All systems nominal",
    "history": [
      {"level": "T1", "from": "2026-01-27T08:00:00Z", "to": "2026-01-27T10:00:00Z"}
    ]
  }
}
```

### 7.3 Impact sur MiyukiniAdmin

| Niveau | Impact |
|--------|--------|
| T0-T1 | Fonctionnement normal |
| T2 | Alertes renforcees |
| T3-T4 | Mode recovery disponible |

---

## 8. Alertes et Notifications

### 8.1 Types d'Alertes

| Type | Severite | Description |
|------|----------|-------------|
| `PERF_CPU_HIGH` | WARNING | CPU > 85% |
| `PERF_RAM_HIGH` | WARNING | RAM > 90% |
| `PERF_DISK_LOW` | CRITICAL | Disk < 10% libre |
| `HEALTH_DEGRADED` | WARNING | Score sante < 70 |
| `OPERATOR_ERROR` | WARNING | Erreurs Operateur |
| `TRUST_LEVEL_CHANGE` | INFO | Changement niveau |

### 8.2 Format Alerte

```json
{
  "alert_id": "uuid-alert-001",
  "timestamp": "2026-01-28T12:00:00Z",
  "type": "PERF_CPU_HIGH",
  "severity": "WARNING",
  "message": "CPU usage at 87%",
  "details": {
    "current_value": 87,
    "threshold": 85,
    "duration_seconds": 120
  },
  "acknowledged": false
}
```

### 8.3 Flux Notifications

```
CaringNanny                  BondingBrother           MiyukiniAdmin
     â”‚                             â”‚                        â”‚
     â”‚  [Anomalie detectee]        â”‚                        â”‚
     â”‚                             â”‚                        â”‚
     â”‚â”€â”€AlertNotificationâ”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚                        â”‚
     â”‚                             â”‚                        â”‚
     â”‚                             â”‚â”€â”€ForwardAlertâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                             â”‚                        â”‚
     â”‚                             â”‚                        â”‚  [Afficher alerte]
```

---

## 9. Etats des Operateurs

### 9.1 Etats Possibles

| Etat | Description |
|------|-------------|
| `HEALTHY` | Fonctionnement normal |
| `DEGRADED` | Performance reduite |
| `UNHEALTHY` | Problemes detectes |
| `ISOLATED` | Isole par admin |
| `OFFLINE` | Non disponible |

### 9.2 Format Etat Operateur

```json
{
  "operator_id": "uuid-operator-001",
  "name": "MyCMS",
  "state": "HEALTHY",
  "metrics": {
    "requests_per_sec": 150,
    "error_rate": 0.1,
    "avg_latency_ms": 25
  },
  "last_health_check": "2026-01-28T12:00:00Z",
  "issues": []
}
```

---

## 10. Modes de Collecte

### 10.1 Collecte Continue (Push)

Pour le dashboard temps reel :
- Frequence : 1-5 secondes
- Format : Stream de metriques
- Usage : Affichage live

### 10.2 Collecte Ponctuelle (Pull)

Pour les rapports :
- A la demande
- Format : Snapshot complet
- Usage : Export, analyse

### 10.3 Configuration

```yaml
caring_nanny:
  metrics_collection:
    realtime:
      enabled: true
      interval_ms: 1000
    standard:
      enabled: true
      interval_ms: 5000
    alerts:
      enabled: true
      check_interval_ms: 10000
```

---

## 11. Integration UI Dashboard

### 11.1 Widgets Alimentes par CaringNanny

| Widget | Metriques | Refresh |
|--------|-----------|---------|
| CPU Gauge | cpu.usage | 1s |
| RAM Bar | ram.usage | 1s |
| Disk Chart | disk.usage | 5s |
| Network Flow | network.io | 1s |
| Health Score | health.score | 5s |
| Trust Level | trust.level | 5s |
| Operators Grid | operators.* | 5s |
| Alerts List | alerts.* | Temps reel |

### 11.2 Couleurs par Sante

| Score | Couleur |
|-------|---------|
| 80-100 | Vert |
| 60-79 | Jaune |
| 40-59 | Orange |
| 0-39 | Rouge |

---

## 12. Gestion des Erreurs

### 12.1 Erreurs Possibles

| Code | Description | Action |
|------|-------------|--------|
| `CN_001` | Metrique indisponible | Afficher N/A |
| `CN_002` | Timeout collection | Retry |
| `CN_003` | Operateur non trouve | Retirer de la liste |
| `CN_004` | Stream interrompu | Reconnecter |

### 12.2 Comportement Degraded

Si CaringNanny est partiellement indisponible :
- Afficher les metriques disponibles
- Indiquer les metriques manquantes
- Pas de blocage du dashboard

---

## 13. Documents Associes

- [MiyukiniAdmin - Core Interaction Contract](../../architecture/MiyukiniAdmin%20-%20Core%20Interaction%20Contract.md)
- [MiyukiniAdmin - Consumption Metrics Contract](../monitoring/MiyukiniAdmin%20-%20Consumption%20Metrics%20Contract.md)
- [MiyukiniAdmin - Dashboard & Metrics Display](../../ui/MiyukiniAdmin%20-%20Dashboard%20&%20Metrics%20Display.md)
- [CaringNanny - Documentation Fondatrice](..//..//..//..//cores//CaringNanny//foundation//Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Contrat de reference

