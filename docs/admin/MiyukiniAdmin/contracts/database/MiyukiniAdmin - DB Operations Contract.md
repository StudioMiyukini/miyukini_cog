# MiyukiniAdmin â€” DB Operations Contract

## 1. Contexte

Ce document definit le contrat pour les **operations de manipulation de base de donnees** dans MiyukiniAdmin. Ces operations sont effectuees **exclusivement via KindMother**, sous l'autorite de StrongFather.

Ce contrat couvre les operations normales de maintenance et d'administration DB, pas le mode recovery (voir Emergency DB Access Contract).

## 2. Portee / Scope

Ce document definit :
- Les operations DB autorisees
- Les protocoles d'execution
- Les validations requises
- Les contraintes de securite

Ce document **ne couvre pas** :
- L'acces DB direct en mode recovery (voir Emergency DB Access Contract)
- Les metriques DB (voir DB Metrics Contract)
- L'interface utilisateur (voir UI documentation)

---

## 3. Principe Fondamental

### 3.1 Mediation KindMother

> **Toutes les operations DB passent par KindMother. Aucun acces direct a la base de donnees n'est autorise en mode normal.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-DBO-1** | Toutes les operations passent par KindMother |
| **INV-DBO-2** | Validation StrongFather pour operations d'ecriture |
| **INV-DBO-3** | Tracabilite complete de chaque operation |
| **INV-DBO-4** | Jamais de logique metier applicative |
| **INV-DBO-5** | Retour en arriere possible pour operations critiques |

---

## 4. Categories d'Operations

### 4.1 Operations de Lecture

| Operation | Description | Validation SF |
|-----------|-------------|---------------|
| **READ-001** | Exploration tables | Non |
| **READ-002** | Consultation schema | Non |
| **READ-003** | Visualisation donnees | Non |
| **READ-004** | Export donnees | Non |
| **READ-005** | Statistiques tables | Non |

### 4.2 Operations de Maintenance

| Operation | Description | Validation SF |
|-----------|-------------|---------------|
| **MAINT-001** | Analyse tables | Oui |
| **MAINT-002** | Vacuum/Optimize | Oui |
| **MAINT-003** | Reindex | Oui |
| **MAINT-004** | Mise a jour statistiques | Oui |
| **MAINT-005** | Nettoyage logs | Oui |

### 4.3 Operations de Reparation

| Operation | Description | Validation SF | Conditions |
|-----------|-------------|---------------|------------|
| **REPAIR-001** | Correction orphelins | Oui | Apres test COH-005 |
| **REPAIR-002** | Correction doublons | Oui | Apres test COH-006 |
| **REPAIR-003** | Correction contraintes | Oui | Avec justification |

### 4.4 Operations de Migration

| Operation | Description | Validation SF | Conditions |
|-----------|-------------|---------------|------------|
| **MIG-001** | Migration schema | Oui | Pre-tests requis |
| **MIG-002** | Migration donnees | Oui | Backup obligatoire |
| **MIG-003** | Rollback migration | Oui | Apres echec |

---

## 5. Flux d'Operations

### 5.1 Operation de Lecture (READ)

```
MiyukiniAdmin            BondingBrother              KindMother
     â”‚                         â”‚                          â”‚
     â”‚â”€â”€ReadRequestâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚                          â”‚
     â”‚  (table: users)          â”‚                          â”‚
     â”‚  (filters: {...})        â”‚                          â”‚
     â”‚                         â”‚                          â”‚
     â”‚                         â”‚â”€â”€DataQueryâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                         â”‚                          â”‚
     â”‚                         â”‚â—€â”€DataResultâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                         â”‚                          â”‚
     â”‚â—€â”€ReadResponseâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚                          â”‚
     â”‚  (data, metadata)        â”‚                          â”‚
     â”‚                         â”‚                          â”‚
```

### 5.2 Operation de Maintenance (MAINT)

```
MiyukiniAdmin            BondingBrother         StrongFather      KindMother
     â”‚                         â”‚                     â”‚                â”‚
     â”‚â”€â”€MaintenanceRequestâ”€â”€â”€â”€â”€â–¶â”‚                     â”‚                â”‚
     â”‚  (op: VACUUM)            â”‚                     â”‚                â”‚
     â”‚  (table: orders)         â”‚                     â”‚                â”‚
     â”‚                         â”‚                     â”‚                â”‚
     â”‚                         â”‚â”€â”€ValidateOpâ”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚                â”‚
     â”‚                         â”‚                     â”‚                â”‚
     â”‚                         â”‚â—€â”€Approvedâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚                â”‚
     â”‚                         â”‚                     â”‚                â”‚
     â”‚                         â”‚â”€â”€ExecuteMaintenanceâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                         â”‚                     â”‚                â”‚
     â”‚                         â”‚â—€â”€MaintenanceCompleteâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                         â”‚                     â”‚                â”‚
     â”‚â—€â”€MaintenanceResponseâ”€â”€â”€â”€â”‚                     â”‚                â”‚
     â”‚  (status, stats)         â”‚                     â”‚                â”‚
     â”‚                         â”‚                     â”‚                â”‚
```

### 5.3 Operation de Migration (MIG)

```
MiyukiniAdmin            BondingBrother         StrongFather      KindMother
     â”‚                         â”‚                     â”‚                â”‚
     â”‚â”€â”€MigrationRequestâ”€â”€â”€â”€â”€â”€â”€â–¶â”‚                     â”‚                â”‚
     â”‚  (migration: v2.1)       â”‚                     â”‚                â”‚
     â”‚                         â”‚                     â”‚                â”‚
     â”‚                         â”‚  [Pre-validation]    â”‚                â”‚
     â”‚                         â”‚â”€â”€RunPreTestsâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                         â”‚â—€â”€PreTestsOKâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                         â”‚                     â”‚                â”‚
     â”‚                         â”‚â”€â”€CreateBackupâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                         â”‚â—€â”€BackupCreatedâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                         â”‚                     â”‚                â”‚
     â”‚                         â”‚â”€â”€ValidateMigrationâ”€â”€â–¶â”‚                â”‚
     â”‚                         â”‚â—€â”€Approvedâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚                â”‚
     â”‚                         â”‚                     â”‚                â”‚
     â”‚                         â”‚â”€â”€ExecuteMigrationâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                         â”‚                     â”‚                â”‚
     â”‚                         â”‚â—€â”€MigrationCompleteâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                         â”‚                     â”‚                â”‚
     â”‚                         â”‚â”€â”€RunPostTestsâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                         â”‚â—€â”€PostTestsOKâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                         â”‚                     â”‚                â”‚
     â”‚â—€â”€MigrationResponseâ”€â”€â”€â”€â”€â”€â”‚                     â”‚                â”‚
     â”‚  (status, report)        â”‚                     â”‚                â”‚
```

---

## 6. Exploration de Tables (Style PHPMyAdmin)

### 6.1 Fonctionnalites

| Fonctionnalite | Description |
|----------------|-------------|
| **Liste tables** | Affichage de toutes les tables |
| **Structure** | Schema de la table (colonnes, types, index) |
| **Donnees** | Visualisation des enregistrements (pagine) |
| **Filtres** | Filtrage par colonnes |
| **Tri** | Tri par colonnes |
| **Export** | Export CSV, JSON |

### 6.2 Format Requete Exploration

```json
{
  "operation": "READ-001",
  "target": {
    "table": "users",
    "columns": ["id", "email", "created_at"],
    "filters": {
      "created_at": {"gte": "2026-01-01"}
    },
    "order_by": [{"column": "created_at", "direction": "DESC"}],
    "pagination": {
      "page": 1,
      "per_page": 50
    }
  }
}
```

### 6.3 Format Reponse

```json
{
  "status": "SUCCESS",
  "data": {
    "rows": [
      {"id": "uuid-1", "email": "user@example.com", "created_at": "2026-01-15T10:00:00Z"}
    ],
    "total_count": 1500,
    "page": 1,
    "per_page": 50,
    "total_pages": 30
  },
  "metadata": {
    "table": "users",
    "query_time_ms": 25
  }
}
```

---

## 7. Operations de Maintenance

### 7.1 VACUUM / OPTIMIZE

**Description :** Recuperation d'espace et optimisation des tables.

**Pre-conditions :**
- Validation StrongFather
- Charge systeme < 70%

**Post-conditions :**
- Mise a jour statistiques automatique
- Log de l'operation

```json
{
  "operation": "MAINT-002",
  "target": {
    "table": "orders",
    "options": {
      "full": false,
      "analyze": true
    }
  },
  "justification": "Optimisation apres suppression massive"
}
```

### 7.2 REINDEX

**Description :** Reconstruction des index.

**Pre-conditions :**
- Validation StrongFather
- Periode de maintenance recommandee

```json
{
  "operation": "MAINT-003",
  "target": {
    "table": "orders",
    "index": "idx_orders_date"
  },
  "justification": "Index fragmente detecte"
}
```

---

## 8. Operations de Reparation

### 8.1 Correction Orphelins

**Description :** Suppression ou correction des enregistrements orphelins.

**Pre-conditions :**
- Test COH-005 execute avec FAIL
- Validation StrongFather
- Backup automatique avant operation

```json
{
  "operation": "REPAIR-001",
  "target": {
    "table": "order_items",
    "action": "DELETE",
    "condition": "orphan_orders"
  },
  "justification": "Nettoyage apres incident #123",
  "backup_before": true
}
```

### 8.2 Modes de Correction

| Mode | Description | Risque |
|------|-------------|--------|
| **PREVIEW** | Affiche les lignes affectees sans modifier | Aucun |
| **DELETE** | Supprime les orphelins | Moyen |
| **NULLIFY** | Met a NULL les FK invalides | Faible |
| **REASSIGN** | Reassigne a un parent par defaut | Faible |

---

## 9. Operations de Migration

### 9.1 Workflow Migration

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 1. Pre-validation                                            â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Verification etat systeme (T0/T1 requis)                  â”‚
â”‚ - Execution suite tests Pre-Migration                       â”‚
â”‚ - Verification espace disque                                â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 2. Preparation                                               â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Creation backup complet                                   â”‚
â”‚ - Notification CaringNanny (mode migration)                 â”‚
â”‚ - Blocage operations concurrentes                           â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 3. Execution                                                 â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Application des scripts de migration                      â”‚
â”‚ - Log detaille de chaque etape                              â”‚
â”‚ - Arret si erreur                                           â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 4. Post-validation                                           â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Execution suite tests Post-Migration                      â”‚
â”‚ - Verification integrite                                    â”‚
â”‚ - Si echec â†’ Rollback automatique                           â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 5. Finalisation                                              â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Notification succes/echec                                 â”‚
â”‚ - Mise a jour version DB                                    â”‚
â”‚ - Deblocage operations                                      â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 9.2 Rollback

**Declencheurs automatiques :**
- Echec pendant execution
- Echec tests Post-Migration

**Processus :**
1. Arret immediat de la migration
2. Restauration depuis backup
3. Verification integrite
4. Notification d'echec

---

## 9.3 Scripts de migration (format, ordre, historique)

### 9.3.1 Format des fichiers

| Element | Specification |
|---------|---------------|
| **Un fichier par migration** | Un script SQL par evolution de schema ou donnees |
| **Nommage** | Chronologique : `YYYYMMDDHHMMSS_description_courte.sql` ou versionnee : `V{n}_{description}.sql` (ex. `V2_add_users_roles.sql`) |
| **Encodage** | UTF-8 |
| **Contenu** | DDL et/ou DML ; pas de logique applicative |

### 9.3.2 Ordre d'application

- Les migrations sont appliquees **strictement dans l'ordre** (chronologique ou version).
- Aucun saut de version autorise sans procedure formelle (rollback puis re-application cible).
- En cas de conflit (meme timestamp ou version), le systeme rejette et signale.

### 9.3.3 Table d'historique

Une table d'historique (gouvernÃ©e par KindMother) enregistre :

| Colonne | Description |
|---------|-------------|
| `migration_id` | Identifiant unique (nom fichier ou hash) |
| `applied_at` | Horodatage d'application |
| `checksum` | Hash du contenu du script (integrite) |
| `duration_ms` | Duree d'execution |
| `result` | SUCCESS / FAIL |
| `operator_id` | Operateur humain ayant declenche (MiyukiniAdmin) |

Les scripts deja appliques ne sont pas re-executes (consultation de l'historique avant execution).

### 9.3.4 Idempotence recommandee

- Utiliser des constructions idempotentes lorsque possible : `CREATE TABLE IF NOT EXISTS`, `ADD COLUMN IF NOT EXISTS` (PostgreSQL), etc.
- En cas d'echec partiel, le rollback (MIG-003) restaure depuis le backup ; les scripts ne sont pas rejoues automatiquement sans nouvelle demande.

### 9.3.5 Lien avec Ever Buddy

- **Ever Buddy** (Core de cycle de vie) ne execute pas les migrations.
- Consultation possible pour **compatibilitÃ©** et **etats de vie** (ACTIF, DEPRECIE) des schemas ou objets avant/apres migration.
- La version du schema ou de l'environnement peut etre mise a jour apres migration reussie ; coherence avec la strate Cores (evolution par environnement, voir Glossaire).

---

## 10. Securite et Contraintes

### 10.1 Roles et Permissions

| Operation | Role Minimum |
|-----------|--------------|
| READ-* | Viewer |
| MAINT-* | Operator |
| REPAIR-* | Admin |
| MIG-* | Admin |

### 10.2 Contraintes Temporelles

| Operation | Timeout | Remarque |
|-----------|---------|----------|
| READ | 60s | Pagination recommandee |
| MAINT | 30 min | Selon taille table |
| REPAIR | 30 min | Avec backup |
| MIG | 2h | Configurable |

### 10.3 Verrous

| Situation | Comportement |
|-----------|--------------|
| Migration en cours | Blocage toutes autres ops |
| Maintenance en cours | Blocage ops sur meme table |
| Reparation en cours | Blocage ops sur meme table |

---

## 11. Audit et Tracabilite

### 11.1 Donnees Tracees

| Champ | Description |
|-------|-------------|
| `operation_id` | Identifiant unique |
| `timestamp` | Horodatage |
| `operator_id` | Operateur humain |
| `operation_type` | Type d'operation |
| `target` | Table/objet cible |
| `parameters` | Parametres de l'operation |
| `justification` | Justification (si requise) |
| `result` | Resultat (SUCCESS/FAIL) |
| `duration_ms` | Duree d'execution |
| `rows_affected` | Lignes affectees |

### 11.2 Retention Logs

| Type | Retention |
|------|-----------|
| READ | 7 jours |
| MAINT | 90 jours |
| REPAIR | 1 an |
| MIG | Permanent |

---

## 12. Documents Associes

- [MiyukiniAdmin - Gestion DB type Supabase](..//..//reference//MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md)
- [MiyukiniAdmin - Emergency DB Access Contract](./MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md)
- [MiyukiniAdmin - DB Metrics Contract](../monitoring/MiyukiniAdmin%20-%20DB%20Metrics%20Contract.md)
- [MiyukiniAdmin - Unit Tests Contract](../testing/MiyukiniAdmin%20-%20Unit%20Tests%20Contract.md)
- [KindMother - Documentation Fondatrice](..//..//..//..//cores//KindMother//foundation//KindMother%20-%20Documentation%20Fondatrice.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Contrat de reference

