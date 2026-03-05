# MiyukiniAdmin â€” Consumption Metrics Contract

## 1. Contexte

Ce document definit le contrat de collecte et d'affichage des **metriques de consommation** dans MiyukiniAdmin. Ces metriques couvrent l'utilisation des ressources systeme : CPU, RAM, disque et reseau.

Les metriques de consommation sont **en lecture seule** et ne modifient jamais l'etat du systeme.

## 2. Portee / Scope

Ce document definit :
- Les metriques de consommation collectees
- Les sources de donnees
- Les frequences de collecte
- Les formats de representation
- Les seuils d'alerte

Ce document **ne couvre pas** :
- Les metriques DB (voir DB Metrics Contract)
- L'implementation technique de la collecte
- L'interface utilisateur (voir UI documentation)

---

## 3. Principe Fondamental

### 3.1 Lecture Passive

> **La collecte de metriques est une operation de lecture passive qui ne modifie jamais l'etat du systeme.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-CM-1** | Collecte en lecture seule uniquement |
| **INV-CM-2** | Aucun impact sur les performances du systeme surveille |
| **INV-CM-3** | Toutes les metriques sont horodatees |
| **INV-CM-4** | Les metriques passent par BondingBrother/CaringNanny |

---

## 4. Metriques CPU

### 4.1 Metriques Collectees

| Metrique | Description | Unite |
|----------|-------------|-------|
| `cpu.usage.total` | Utilisation CPU totale | Pourcentage (0-100) |
| `cpu.usage.user` | Temps utilisateur | Pourcentage |
| `cpu.usage.system` | Temps systeme | Pourcentage |
| `cpu.usage.idle` | Temps inactif | Pourcentage |
| `cpu.usage.iowait` | Attente I/O | Pourcentage |
| `cpu.cores.count` | Nombre de coeurs | Entier |
| `cpu.load.1m` | Charge moyenne 1 minute | Decimal |
| `cpu.load.5m` | Charge moyenne 5 minutes | Decimal |
| `cpu.load.15m` | Charge moyenne 15 minutes | Decimal |

### 4.2 Source

- **Core source :** CaringNanny
- **Capacite :** `admin.metrics.system`

### 4.3 Seuils d'Alerte

| Niveau | Seuil | Action |
|--------|-------|--------|
| **Normal** | < 70% | Aucune |
| **Warning** | 70-85% | Alerte visuelle |
| **Critical** | 85-95% | Alerte + Notification |
| **Emergency** | > 95% | Alerte rouge + Log |

### 4.4 Format de Donnees

```json
{
  "timestamp": "2026-01-28T12:00:00Z",
  "cpu": {
    "usage": {
      "total": 45.2,
      "user": 30.1,
      "system": 15.1,
      "idle": 54.8,
      "iowait": 2.3
    },
    "cores": 8,
    "load": {
      "1m": 2.4,
      "5m": 2.1,
      "15m": 1.8
    }
  }
}
```

---

## 5. Metriques RAM

### 5.1 Metriques Collectees

| Metrique | Description | Unite |
|----------|-------------|-------|
| `ram.total` | Memoire totale | Bytes |
| `ram.used` | Memoire utilisee | Bytes |
| `ram.free` | Memoire libre | Bytes |
| `ram.available` | Memoire disponible | Bytes |
| `ram.cached` | Memoire en cache | Bytes |
| `ram.buffers` | Buffers | Bytes |
| `ram.usage` | Pourcentage utilise | Pourcentage |
| `swap.total` | Swap total | Bytes |
| `swap.used` | Swap utilise | Bytes |
| `swap.free` | Swap libre | Bytes |

### 5.2 Source

- **Core source :** CaringNanny
- **Capacite :** `admin.metrics.system`

### 5.3 Seuils d'Alerte

| Niveau | Seuil RAM | Seuil Swap | Action |
|--------|-----------|------------|--------|
| **Normal** | < 75% | < 10% | Aucune |
| **Warning** | 75-85% | 10-30% | Alerte visuelle |
| **Critical** | 85-95% | 30-50% | Alerte + Notification |
| **Emergency** | > 95% | > 50% | Alerte rouge + Log |

### 5.4 Format de Donnees

```json
{
  "timestamp": "2026-01-28T12:00:00Z",
  "ram": {
    "total": 17179869184,
    "used": 8589934592,
    "free": 4294967296,
    "available": 8589934592,
    "cached": 2147483648,
    "buffers": 1073741824,
    "usage": 50.0
  },
  "swap": {
    "total": 8589934592,
    "used": 0,
    "free": 8589934592
  }
}
```

---

## 6. Metriques Disque

### 6.1 Metriques Collectees

| Metrique | Description | Unite |
|----------|-------------|-------|
| `disk.total` | Espace total | Bytes |
| `disk.used` | Espace utilise | Bytes |
| `disk.free` | Espace libre | Bytes |
| `disk.usage` | Pourcentage utilise | Pourcentage |
| `disk.read.bytes` | Bytes lus | Bytes/sec |
| `disk.write.bytes` | Bytes ecrits | Bytes/sec |
| `disk.read.ops` | Operations lecture | Ops/sec |
| `disk.write.ops` | Operations ecriture | Ops/sec |
| `disk.latency.read` | Latence lecture | Millisecondes |
| `disk.latency.write` | Latence ecriture | Millisecondes |

### 6.2 Source

- **Core source :** CaringNanny
- **Capacite :** `admin.metrics.system`

### 6.3 Seuils d'Alerte

| Niveau | Seuil Espace | Seuil Latence | Action |
|--------|--------------|---------------|--------|
| **Normal** | < 70% | < 10ms | Aucune |
| **Warning** | 70-85% | 10-50ms | Alerte visuelle |
| **Critical** | 85-95% | 50-100ms | Alerte + Notification |
| **Emergency** | > 95% | > 100ms | Alerte rouge + Log |

### 6.4 Format de Donnees

```json
{
  "timestamp": "2026-01-28T12:00:00Z",
  "disks": [
    {
      "mount": "/",
      "device": "/dev/sda1",
      "total": 1099511627776,
      "used": 549755813888,
      "free": 549755813888,
      "usage": 50.0,
      "io": {
        "read_bytes_sec": 1048576,
        "write_bytes_sec": 524288,
        "read_ops_sec": 100,
        "write_ops_sec": 50,
        "latency_read_ms": 5,
        "latency_write_ms": 8
      }
    }
  ]
}
```

---

## 7. Metriques Reseau

### 7.1 Metriques Collectees

| Metrique | Description | Unite |
|----------|-------------|-------|
| `network.rx.bytes` | Bytes recus | Bytes/sec |
| `network.tx.bytes` | Bytes emis | Bytes/sec |
| `network.rx.packets` | Paquets recus | Packets/sec |
| `network.tx.packets` | Paquets emis | Packets/sec |
| `network.rx.errors` | Erreurs reception | Count |
| `network.tx.errors` | Erreurs emission | Count |
| `network.rx.dropped` | Paquets perdus (rx) | Count |
| `network.tx.dropped` | Paquets perdus (tx) | Count |
| `network.connections.active` | Connexions actives | Count |
| `network.connections.established` | Connexions etablies | Count |

### 7.2 Source

- **Core source :** CaringNanny
- **Capacite :** `admin.metrics.system`

### 7.3 Seuils d'Alerte

| Niveau | Seuil Errors | Seuil Dropped | Action |
|--------|--------------|---------------|--------|
| **Normal** | 0 | < 0.1% | Aucune |
| **Warning** | 1-10 | 0.1-1% | Alerte visuelle |
| **Critical** | 10-100 | 1-5% | Alerte + Notification |
| **Emergency** | > 100 | > 5% | Alerte rouge + Log |

### 7.4 Format de Donnees

```json
{
  "timestamp": "2026-01-28T12:00:00Z",
  "network": {
    "interfaces": [
      {
        "name": "eth0",
        "rx": {
          "bytes_sec": 1048576,
          "packets_sec": 1000,
          "errors": 0,
          "dropped": 0
        },
        "tx": {
          "bytes_sec": 524288,
          "packets_sec": 500,
          "errors": 0,
          "dropped": 0
        }
      }
    ],
    "connections": {
      "active": 150,
      "established": 120,
      "time_wait": 30
    }
  }
}
```

---

## 8. Frequences de Collecte

### 8.1 Modes de Collecte

| Mode | Frequence | Usage |
|------|-----------|-------|
| **Realtime** | 1 seconde | Dashboard live |
| **Standard** | 5 secondes | Monitoring normal |
| **Economy** | 30 secondes | Monitoring reduit |
| **Snapshot** | A la demande | Export ponctuel |

### 8.2 Selection du Mode

Le mode est selectionne en fonction de :
- Etat du systeme (niveaux de confiance T0-T4)
- Niveau de securite actuel (0-4)
- Charge du systeme

| Condition | Mode suggere |
|-----------|--------------|
| Normal (T0) | Standard |
| Degrade (T1-T2) | Realtime |
| Critique (T3-T4) | Realtime |
| Haute charge CPU | Economy |

---

## 9. Agregation et Historique

### 9.1 Retention des Donnees

| Resolution | Retention | Usage |
|------------|-----------|-------|
| **1 seconde** | 1 heure | Dashboard live |
| **1 minute** | 24 heures | Analyse recente |
| **5 minutes** | 7 jours | Analyse hebdomadaire |
| **1 heure** | 30 jours | Analyse mensuelle |
| **1 jour** | 1 an | Analyse annuelle |

### 9.2 Agregation

| Metrique | Min | Max | Avg | Sum |
|----------|-----|-----|-----|-----|
| CPU usage | âœ… | âœ… | âœ… | âŒ |
| RAM usage | âœ… | âœ… | âœ… | âŒ |
| Disk usage | âœ… | âœ… | âœ… | âŒ |
| Disk I/O | âœ… | âœ… | âœ… | âœ… |
| Network I/O | âœ… | âœ… | âœ… | âœ… |

---

## 10. Integration UI

### 10.1 Widgets Dashboard

| Widget | Metriques | Type affichage |
|--------|-----------|----------------|
| **CPU Gauge** | cpu.usage.total | Jauge circulaire |
| **RAM Bar** | ram.usage | Barre de progression |
| **Disk Usage** | disk.usage | Graphique secteurs |
| **Network Flow** | rx/tx bytes | Graphique ligne |
| **System Health** | Tous | Indicateur global |

### 10.2 Couleurs d'Etat

| Etat | Couleur | Code |
|------|---------|------|
| **Normal** | Vert | #28a745 |
| **Warning** | Jaune | #ffc107 |
| **Critical** | Orange | #fd7e14 |
| **Emergency** | Rouge | #dc3545 |

---

## 11. Flux de Collecte

```
MiyukiniAdmin                 BondingBrother              CaringNanny
     â”‚                              â”‚                          â”‚
     â”‚â”€â”€MetricsRequestâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚                          â”‚
     â”‚  (type: consumption)          â”‚                          â”‚
     â”‚  (interval: 5s)               â”‚                          â”‚
     â”‚                              â”‚                          â”‚
     â”‚                              â”‚â”€â”€ReadMetricsâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                              â”‚                          â”‚
     â”‚                              â”‚â—€â”€MetricsDataâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                              â”‚  (cpu, ram, disk, net)    â”‚
     â”‚                              â”‚                          â”‚
     â”‚â—€â”€MetricsResponseâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚                          â”‚
     â”‚                              â”‚                          â”‚
     â”‚  (Update Dashboard)          â”‚                          â”‚
     â”‚                              â”‚                          â”‚
```

---

## 12. Implications Securite

### 12.1 Exposition des Metriques

Les metriques de consommation contiennent des informations sensibles sur l'infrastructure. Leur exposition doit etre strictement controlee.

| Risque | Description | Mitigation |
|--------|-------------|------------|
| **Information Disclosure** | Les metriques revelent la capacite et la charge du systeme | Acces restreint via capacites `admin.metrics.system` |
| **Reconnaissance Attack** | Un attaquant peut analyser les patterns de charge | Metriques non exposees publiquement |
| **Timing Analysis** | Correlation entre metriques et operations sensibles | Agregation temporelle minimale (1s) |

### 12.2 Controles d'Acces

| Controle | Implementation |
|----------|----------------|
| **Authentification** | Session MiyukiniAdmin validee par StrongFather |
| **Autorisation** | Capacite `admin.metrics.system` requise |
| **Audit** | Toute consultation de metriques est tracee |
| **Rate Limiting** | Maximum 60 requetes/minute par session |

### 12.3 Integrite des Metriques

> **INV-CM-SEC-1 : Les metriques ne doivent jamais etre falsifiables.**

| Garantie | Mecanisme |
|----------|-----------|
| **Source authentifiee** | Metriques provenant uniquement de CaringNanny |
| **Transit securise** | Passage par BondingBrother avec tracabilite |
| **Non-modification** | Lecture seule, aucune API de modification |

### 12.4 Adaptation par Niveau de Confiance (T0-T4)

| Niveau | Comportement Metriques |
|--------|------------------------|
| **T0** | Collecte normale, tous les modes disponibles |
| **T1** | Collecte intensifiee, mode Realtime recommande |
| **T2** | Metriques critiques uniquement visibles |
| **T3** | Metriques de diagnostic prioritaires |
| **T4** | Uniquement metriques de diagnostic |

### 12.5 Adaptation par Niveau de Securite (0-4)

| Niveau | Restrictions |
|--------|--------------|
| **0-1** | Toutes metriques accessibles |
| **2** | Metriques reseau detaillees masquees |
| **3** | Metriques agregees uniquement |
| **4** | Metriques minimales, audit renforce |

### 12.6 References Securite

- [Security - Core Integration Map](..//..//..//..//cores//WorrySentinel//_index.md)
- [Security - Documentation Fondatrice](..//..//..//..//cores//WorrySentinel//_index.md)

---

## 13. Documents Associes

- [MiyukiniAdmin - DB Metrics Contract](./MiyukiniAdmin%20-%20DB%20Metrics%20Contract.md)
- [MiyukiniAdmin - Architecture & Flows](../../architecture/MiyukiniAdmin%20-%20Architecture%20&%20Flows.md)
- [MiyukiniAdmin - Dashboard & Metrics Display](../../ui/MiyukiniAdmin%20-%20Dashboard%20&%20Metrics%20Display.md)
- [CaringNanny - Documentation Fondatrice](..//..//..//..//cores//CaringNanny//foundation//Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Contrat de reference


