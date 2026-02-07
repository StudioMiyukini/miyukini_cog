# MiyukiniAdmin — KindMother Integration Contract

## 1. Contexte

Ce document definit le contrat d'integration entre MiyukiniAdmin et **KindMother**. KindMother est l'autorite absolue sur les donnees et la persistance du systeme.

## 2. Portee / Scope

Ce document definit :
- Le role de KindMother pour MiyukiniAdmin
- Les operations de donnees autorisees
- Les protocoles d'acces
- Les contraintes et exceptions (mode recovery)

Ce document **ne couvre pas** :
- L'implementation interne de KindMother
- Les autres integrations cores
- Les details du mode recovery (voir Emergency DB Access Contract)

---

## 3. Role de KindMother

### 3.1 Autorite sur les Donnees

> **KindMother est l'autorite absolue sur les donnees. MiyukiniAdmin accede aux donnees exclusivement via KindMother (sauf mode recovery).**

**Question fondamentale :**
> "Comment les donnees sont-elles persistees, synchronisees et accessibles ?"

### 3.2 Responsabilites

| Responsabilite | Description |
|----------------|-------------|
| **Persistance** | Stockage et recuperation des donnees |
| **Coherence** | Garantie d'integrite des donnees |
| **Synchronisation** | Gestion des conflits et reconciliation |
| **Validation** | Verification des contraintes |

---

## 4. Operations Autorisees

### 4.1 Operations de Lecture

| Operation | Capacite | Description |
|-----------|----------|-------------|
| Exploration tables | `admin.db.read` | Navigation dans les tables |
| Consultation schema | `admin.db.read` | Structure des tables |
| Statistiques | `admin.db.stats` | Metriques DB |
| Export | `admin.db.read` | Export de donnees |

### 4.2 Operations de Maintenance

| Operation | Capacite | Validation SF |
|-----------|----------|---------------|
| Validation coherence | `admin.db.validate` | Non |
| Migration schema | `admin.db.migrate` | Oui |
| Reparation donnees | `admin.db.repair` | Oui |
| Optimisation | `admin.db.maintain` | Oui |

### 4.3 Exception : Mode Recovery

| Operation | Capacite | Conditions |
|-----------|----------|------------|
| Acces direct DB | `admin.db.recovery` | Conditions cumulatives strictes |

---

## 5. Protocole d'Acces

### 5.1 Format de Requete Lecture

```json
{
  "request_id": "uuid-request-001",
  "timestamp": "2026-01-28T12:00:00Z",
  "operation": "READ",
  "target": {
    "table": "users",
    "columns": ["id", "email", "created_at"],
    "filters": {
      "created_at": {"gte": "2026-01-01"}
    },
    "pagination": {
      "page": 1,
      "per_page": 50
    }
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
  "status": "SUCCESS",
  "data": {
    "rows": [...],
    "total_count": 1500,
    "page": 1,
    "total_pages": 30
  },
  "metadata": {
    "query_time_ms": 25,
    "source": "KindMother"
  }
}
```

---

## 6. Flux d'Acces

### 6.1 Flux Lecture Standard

```
MiyukiniAdmin           BondingBrother              KindMother
     │                        │                          │
     │──ReadRequest───────────▶│                          │
     │  (table: users)         │                          │
     │                        │                          │
     │                        │──DataQuery───────────────▶│
     │                        │                          │
     │                        │◀─DataResult──────────────│
     │                        │                          │
     │◀─ReadResponse──────────│                          │
```

### 6.2 Flux Maintenance (avec validation SF)

```
MiyukiniAdmin           BondingBrother         StrongFather      KindMother
     │                        │                     │                │
     │──MaintenanceRequest────▶│                     │                │
     │  (op: MIGRATE)          │                     │                │
     │                        │                     │                │
     │                        │──ValidateOp─────────▶│                │
     │                        │                     │                │
     │                        │◀─APPROVED───────────│                │
     │                        │                     │                │
     │                        │──ExecuteMigration────────────────────▶│
     │                        │                     │                │
     │                        │◀─MigrationResult─────────────────────│
     │                        │                     │                │
     │◀─MaintenanceResponse───│                     │                │
```

### 6.3 Flux Validation Coherence

```
MiyukiniAdmin           BondingBrother              KindMother
     │                        │                          │
     │──ValidateRequest───────▶│                          │
     │  (type: referential)    │                          │
     │                        │                          │
     │                        │──RunValidation───────────▶│
     │                        │                          │
     │                        │◀─ValidationResult────────│
     │                        │  (violations: [...])      │
     │                        │                          │
     │◀─ValidateResponse──────│                          │
```

---

## 7. Types de Donnees Accessibles

### 7.1 Donnees Systeme

| Type | Description | Acces |
|------|-------------|-------|
| Configuration | Parametres systeme | Lecture |
| Metriques historiques | Stats stockees | Lecture |
| Logs audit | Journal operations | Lecture |
| Sessions | Sessions actives | Lecture |

### 7.2 Donnees Operateurs

| Type | Description | Acces |
|------|-------------|-------|
| Schema | Structure tables | Lecture |
| Donnees | Contenu tables | Lecture (avec pagination) |
| Index | Definition index | Lecture |
| Contraintes | FK, UNIQUE, etc. | Lecture |

### 7.3 Donnees Interdites

| Type | Raison |
|------|--------|
| Credentials chiffres | Securite |
| Cles privees | Securite |
| Tokens actifs | Securite |

---

## 8. Validations et Coherence

### 8.1 Types de Validation

| Type | Description | Capacite |
|------|-------------|----------|
| Referentielle | Integrite FK | `admin.db.validate` |
| Contraintes | NOT NULL, UNIQUE | `admin.db.validate` |
| Schema | Conformite attendue | `admin.db.validate` |
| Coherence metier | Regles logiques | `admin.db.validate` |

### 8.2 Format Resultat Validation

```json
{
  "validation_id": "uuid-valid-001",
  "timestamp": "2026-01-28T12:00:00Z",
  "status": "COMPLETED",
  "results": {
    "referential_integrity": {
      "status": "PASS",
      "checked": 45,
      "violations": 0
    },
    "constraints": {
      "status": "WARN",
      "checked": 120,
      "violations": 3,
      "details": [
        {"table": "orders", "column": "status", "issue": "NULL value"}
      ]
    }
  }
}
```

---

## 9. Operations de Maintenance

### 9.1 Migration

**Pre-conditions :**
- Validation StrongFather
- Backup automatique
- Tests pre-migration passes

**Processus :**
1. Creation backup
2. Verification pre-conditions
3. Application migration
4. Tests post-migration
5. Si echec → rollback automatique

### 9.2 Reparation

**Operations possibles :**
- Correction orphelins
- Correction doublons
- Mise a jour contraintes

**Toujours avec :**
- Backup prealable
- Log de toutes modifications
- Verification post-reparation

---

## 10. Relation Mode Normal vs Recovery

### 10.1 Mode Normal

```
MiyukiniAdmin ──▶ BondingBrother ──▶ KindMother ──▶ Database
```

**Caracteristiques :**
- Toutes les validations actives
- Contraintes respectees
- Tracabilite complete
- Pas d'acces direct DB

### 10.2 Mode Recovery (Exception)

```
MiyukiniAdmin ────────────────────────────────────▶ Database
                                                      │
                                              [Acces direct]
```

**Caracteristiques :**
- Conditions cumulatives strictes (voir Emergency DB Access Contract)
- KindMother notifie (blocage Operateurs)
- Tracabilite renforcee
- Revalidation obligatoire apres

---

## 11. Metriques et Statistiques

### 11.1 Metriques Exposees

| Metrique | Description | Source |
|----------|-------------|--------|
| Query count | Nombre requetes | KindMother |
| Query latency | Latence moyenne | KindMother |
| Pool status | Etat connexions | KindMother |
| Cache hit ratio | Efficacite cache | KindMother |
| Table sizes | Tailles tables | KindMother |

### 11.2 Format Statistiques

```json
{
  "timestamp": "2026-01-28T12:00:00Z",
  "statistics": {
    "queries": {
      "total": 150000,
      "per_second": 250,
      "avg_latency_ms": 15
    },
    "pool": {
      "size": 20,
      "active": 15,
      "idle": 5
    },
    "cache": {
      "hit_ratio": 98.5
    },
    "storage": {
      "total_size_bytes": 10737418240,
      "tables_count": 45
    }
  }
}
```

---

## 12. Gestion des Erreurs

### 12.1 Erreurs Possibles

| Code | Description | Action |
|------|-------------|--------|
| `KM_001` | Table non trouvee | Verifier nom |
| `KM_002` | Permission refusee | Verifier role |
| `KM_003` | Timeout | Retry ou paginer |
| `KM_004` | Violation contrainte | Log details |
| `KM_005` | Migration echec | Rollback auto |

### 12.2 Comportement en Erreur

- Toutes les erreurs sont tracees
- Pas de donnees partielles retournees
- Transactions rollback en cas d'echec

---

## 13. Documents Associes

- [MiyukiniAdmin - Core Interaction Contract](../../architecture/MiyukiniAdmin%20-%20Core%20Interaction%20Contract.md)
- [MiyukiniAdmin - DB Operations Contract](../database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md)
- [MiyukiniAdmin - Emergency DB Access Contract](../database/MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md)
- [KindMother - Documentation Fondatrice](../../../KindMother/foundation/KindMother%20-%20Documentation%20Fondatrice.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Contrat de reference
