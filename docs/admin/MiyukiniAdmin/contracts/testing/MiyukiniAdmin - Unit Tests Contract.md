# MiyukiniAdmin â€” Unit Tests Contract

## 1. Contexte

Ce document definit le contrat pour les **tests unitaires** dans MiyukiniAdmin. Les tests unitaires sont des tests de coherence DB et de conformite contractuelle qui verifient l'integrite et la conformite du systeme.

Ces tests sont **non destructifs** et n'impactent pas les donnees.

## 2. Portee / Scope

Ce document definit :
- Les types de tests unitaires disponibles
- Les protocoles de verification
- Les rapports de conformite
- Les criteres de succes/echec

Ce document **ne couvre pas** :
- Les tests de cycle (voir Cycle Tests Contract)
- L'implementation technique des tests
- L'interface utilisateur (voir UI documentation)

---

## 3. Principe Fondamental

### 3.1 Non-Destructivite

> **Les tests unitaires sont des operations de verification en lecture seule qui ne modifient jamais les donnees ni le schema.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-UT-1** | Aucune modification des donnees |
| **INV-UT-2** | Aucune modification du schema |
| **INV-UT-3** | Tracabilite complete de chaque test |
| **INV-UT-4** | Rapports conserves pour audit |

---

## 4. Categories de Tests

### 4.1 Tests de Coherence DB

| Code | Test | Description |
|------|------|-------------|
| **COH-001** | Integrite referentielle | Verification des cles etrangeres |
| **COH-002** | Contraintes NOT NULL | Verification des champs obligatoires |
| **COH-003** | Unicite | Verification des contraintes UNIQUE |
| **COH-004** | Coherence inter-tables | Verification des relations logiques |
| **COH-005** | Orphelins | Detection des enregistrements orphelins |
| **COH-006** | Doublons | Detection des doublons suspects |
| **COH-007** | Valeurs aberrantes | Detection des valeurs hors bornes |

### 4.2 Tests de Conformite Contractuelle

| Code | Test | Description |
|------|------|-------------|
| **CONF-001** | Schema KindMother | Conformite au schema attendu |
| **CONF-002** | Types de donnees | Verification des types |
| **CONF-003** | Formats | Verification des formats (dates, UUID, etc.) |
| **CONF-004** | Invariants metier | Verification des regles invariantes |
| **CONF-005** | Conventions nommage | Verification des conventions |

### 4.3 Tests de Sante Structurelle

| Code | Test | Description |
|------|------|-------------|
| **STRUCT-001** | Index existants | Verification des index requis |
| **STRUCT-002** | Index non utilises | Detection index inutiles |
| **STRUCT-003** | Tables vides | Detection tables sans donnees |
| **STRUCT-004** | Fragmentation | Niveau de fragmentation |
| **STRUCT-005** | Statistiques | Fraicheur des statistiques |

---

## 5. Protocole de Test

### 5.1 Execution d'un Test

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 1. Chargement definition du test                            â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - ID du test                                                â”‚
â”‚ - Requete de verification                                   â”‚
â”‚ - Criteres de succes                                        â”‚
â”‚ - Seuils d'alerte                                           â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 2. Execution requete (via KindMother)                       â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Requete en lecture seule                                  â”‚
â”‚ - Timeout configurable                                      â”‚
â”‚ - Collecte resultats                                        â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 3. Evaluation resultats                                      â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Comparaison aux criteres                                  â”‚
â”‚ - Determination verdict                                     â”‚
â”‚ - Collecte details echecs                                   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 4. Generation rapport                                        â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Verdict (PASS/WARN/FAIL)                                  â”‚
â”‚ - Details des violations                                    â”‚
â”‚ - Recommandations                                           â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 5.2 Flux d'Execution

```
MiyukiniAdmin            BondingBrother              KindMother
     â”‚                         â”‚                          â”‚
     â”‚â”€â”€UnitTestRequestâ”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚                          â”‚
     â”‚  (test: COH-001)         â”‚                          â”‚
     â”‚                         â”‚                          â”‚
     â”‚                         â”‚â”€â”€VerificationQueryâ”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                         â”‚  (SELECT avec contraintes) â”‚
     â”‚                         â”‚                          â”‚
     â”‚                         â”‚â—€â”€QueryResultsâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                         â”‚                          â”‚
     â”‚â—€â”€TestResultsâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚                          â”‚
     â”‚  (verdict, details)      â”‚                          â”‚
     â”‚                         â”‚                          â”‚
```

---

## 6. Definitions de Tests

### 6.1 COH-001 : Integrite Referentielle

```json
{
  "id": "COH-001",
  "name": "Integrite Referentielle",
  "description": "Verifie que toutes les cles etrangeres pointent vers des enregistrements existants",
  "category": "coherence",
  "query_type": "foreign_key_check",
  "criteria": {
    "violations_count": 0
  },
  "severity": "HIGH",
  "output": {
    "total_fk": "Nombre de FK verifiees",
    "violations": "Liste des violations",
    "tables_affected": "Tables concernees"
  }
}
```

### 6.2 COH-005 : Detection Orphelins

```json
{
  "id": "COH-005",
  "name": "Detection Orphelins",
  "description": "Detecte les enregistrements orphelins (sans parent valide)",
  "category": "coherence",
  "criteria": {
    "orphans_count": 0
  },
  "severity": "MEDIUM",
  "output": {
    "orphans": "Liste des orphelins par table",
    "suggested_action": "Nettoyage ou correction"
  }
}
```

### 6.3 CONF-001 : Schema KindMother

```json
{
  "id": "CONF-001",
  "name": "Conformite Schema KindMother",
  "description": "Verifie que le schema DB correspond aux contrats KindMother",
  "category": "conformite",
  "criteria": {
    "missing_tables": 0,
    "missing_columns": 0,
    "type_mismatches": 0
  },
  "severity": "CRITICAL",
  "output": {
    "expected_schema": "Schema attendu",
    "actual_schema": "Schema actuel",
    "differences": "Liste des differences"
  }
}
```

### 6.4 STRUCT-001 : Index Existants

```json
{
  "id": "STRUCT-001",
  "name": "Index Requis",
  "description": "Verifie que tous les index requis existent",
  "category": "structure",
  "criteria": {
    "missing_indexes": 0
  },
  "severity": "HIGH",
  "output": {
    "required_indexes": "Liste index requis",
    "existing_indexes": "Liste index existants",
    "missing": "Index manquants"
  }
}
```

---

## 7. Verdicts et Severites

### 7.1 Verdicts Possibles

| Verdict | Description | Couleur |
|---------|-------------|---------|
| **PASS** | Test reussi, aucune violation | Vert |
| **WARN** | Test reussi avec alertes mineures | Jaune |
| **FAIL** | Test echoue, violations detectees | Rouge |
| **SKIP** | Test ignore (pre-condition non remplie) | Gris |
| **ERROR** | Erreur technique pendant le test | Orange |

### 7.2 Niveaux de Severite

| Severite | Description | Action requise |
|----------|-------------|----------------|
| **CRITICAL** | Bloque le fonctionnement | Immediate |
| **HIGH** | Impact significatif | Prioritaire |
| **MEDIUM** | Impact modere | Planifiee |
| **LOW** | Impact mineur | A surveiller |
| **INFO** | Informatif | Aucune |

---

## 8. Suites de Tests

### 8.1 Suites Predefinies

| Suite | Tests inclus | Duree | Usage |
|-------|--------------|-------|-------|
| **Quick** | COH-001, CONF-001 | < 1 min | Verification rapide |
| **Standard** | Tous COH, CONF-001-003 | 5-10 min | Verification quotidienne |
| **Full** | Tous | 15-30 min | Verification complete |
| **Pre-Migration** | CONF-*, STRUCT-* | 10 min | Avant migration |
| **Post-Migration** | Tous | 15-30 min | Apres migration |

### 8.2 Configuration Suite

```json
{
  "suite": "standard",
  "tests": [
    "COH-001", "COH-002", "COH-003", "COH-004", 
    "COH-005", "COH-006", "COH-007",
    "CONF-001", "CONF-002", "CONF-003"
  ],
  "options": {
    "stop_on_critical_failure": true,
    "parallel_execution": true,
    "timeout_per_test_sec": 120
  }
}
```

---

## 9. Rapports

### 9.1 Structure Rapport

```json
{
  "report_id": "uuid-report-001",
  "suite": "standard",
  "timestamp": "2026-01-28T12:00:00Z",
  "duration_seconds": 245,
  "summary": {
    "total_tests": 10,
    "passed": 8,
    "warnings": 1,
    "failed": 1,
    "skipped": 0,
    "errors": 0
  },
  "overall_verdict": "FAIL",
  "tests": [
    {
      "id": "COH-001",
      "name": "Integrite Referentielle",
      "verdict": "PASS",
      "duration_ms": 1250,
      "details": {
        "fk_checked": 45,
        "violations": 0
      }
    },
    {
      "id": "COH-005",
      "name": "Detection Orphelins",
      "verdict": "FAIL",
      "severity": "MEDIUM",
      "duration_ms": 3500,
      "details": {
        "orphans_found": 12,
        "tables_affected": ["orders", "order_items"],
        "sample_violations": [
          {"table": "order_items", "id": 1234, "missing_parent": "orders.5678"}
        ]
      },
      "recommendation": "Executer script de nettoyage orphelins"
    }
  ],
  "recommendations": [
    {
      "severity": "MEDIUM",
      "issue": "12 enregistrements orphelins detectes",
      "action": "Executer admin.db.cleanup.orphans"
    }
  ]
}
```

### 9.2 Historique

| Retention | Granularite |
|-----------|-------------|
| 7 jours | Tous les rapports |
| 30 jours | Rapports FAIL/WARN |
| 1 an | Rapports CRITICAL |
| Permanent | Rapports Pre/Post-Migration |

---

## 10. Integration UI

### 10.1 Vue Tests Unitaires

| Element | Description |
|---------|-------------|
| **Liste Tests** | Catalogue de tous les tests disponibles |
| **Execution** | Bouton pour lancer un test ou une suite |
| **Progression** | Barre de progression pendant execution |
| **Resultats** | Tableau des resultats avec verdict |
| **Details** | Modal avec details des violations |
| **Historique** | Liste des executions precedentes |

### 10.2 Indicateurs Visuels

| Verdict | Icone | Couleur |
|---------|-------|---------|
| PASS | Check | #28a745 |
| WARN | Triangle | #ffc107 |
| FAIL | X | #dc3545 |
| SKIP | Minus | #6c757d |
| ERROR | Exclamation | #fd7e14 |

---

## 11. Planification Automatique

### 11.1 Schedules Recommandes

| Suite | Frequence | Heure |
|-------|-----------|-------|
| Quick | Toutes les 4 heures | - |
| Standard | Quotidien | 03:00 |
| Full | Hebdomadaire | Dimanche 02:00 |

### 11.2 Configuration Schedule

```json
{
  "schedules": [
    {
      "suite": "quick",
      "cron": "0 */4 * * *",
      "enabled": true
    },
    {
      "suite": "standard",
      "cron": "0 3 * * *",
      "enabled": true,
      "notify_on_failure": true
    },
    {
      "suite": "full",
      "cron": "0 2 * * 0",
      "enabled": true,
      "notify_always": true
    }
  ]
}
```

---

## 12. Tests de Securite Requis

### 12.1 Categorie : Tests de Securite (SEC)

En complement des tests COH, CONF et STRUCT, les tests de securite suivants sont **obligatoires** :

| Code | Test | Description | Severite |
|------|------|-------------|----------|
| **SEC-001** | Permissions non escaladees | Verifie qu'aucun utilisateur n'a des permissions superieures a son role | CRITICAL |
| **SEC-002** | Tokens expires | Detecte les tokens/sessions non expires qui devraient l'etre | HIGH |
| **SEC-003** | Audit trail integrity | Verifie l'integrite des logs d'audit | CRITICAL |
| **SEC-004** | Encryption at rest | Verifie que les donnees sensibles sont chiffrees | HIGH |
| **SEC-005** | Foreign key constraints | Verifie que les FK sont actives (pas de bypass) | CRITICAL |
| **SEC-006** | No orphan permissions | Detecte les permissions orphelines | MEDIUM |

*MiyukiniAdmin etant un service hors-bord, aucun test RLS (Row Level Security) type Supabase n'est retenu ; l'auth et l'autorisation sont celles du service admin, a definir.*

### 12.2 Definitions des Tests de Securite

#### SEC-001 : Permissions Non Escaladees

```json
{
  "id": "SEC-001",
  "name": "Permissions Non Escaladees",
  "description": "Verifie qu'aucun utilisateur n'a de permissions superieures a son role defini",
  "category": "security",
  "criteria": {
    "escalated_permissions_count": 0
  },
  "severity": "CRITICAL",
  "query_type": "permission_check",
  "output": {
    "users_checked": "Nombre d'utilisateurs verifies",
    "violations": "Liste des escalades detectees",
    "expected_vs_actual": "Comparaison role/permissions"
  }
}
```

#### SEC-003 : Audit Trail Integrity

```json
{
  "id": "SEC-003",
  "name": "Audit Trail Integrity",
  "description": "Verifie que les logs d'audit sont complets et non modifies",
  "category": "security",
  "criteria": {
    "gaps_detected": 0,
    "sequence_breaks": 0,
    "hash_mismatches": 0
  },
  "severity": "CRITICAL",
  "output": {
    "entries_checked": "Nombre d'entrees verifiees",
    "coverage": "Couverture temporelle",
    "integrity_status": "Statut integrite"
  }
}
```

#### SEC-006 : No Orphan Permissions

```json
{
  "id": "SEC-006",
  "name": "No Orphan Permissions",
  "description": "Detecte les permissions orphelines (sans utilisateur ou role valide)",
  "category": "security",
  "criteria": {
    "orphan_permissions_count": 0
  },
  "severity": "MEDIUM",
  "output": {
    "permissions_checked": "Nombre de permissions verifiees",
    "orphans": "Liste des permissions orphelines"
  }
}
```

### 12.3 Suites de Tests avec Securite

| Suite | Tests Securite Inclus | Usage |
|-------|----------------------|-------|
| **Quick** | SEC-001 | Verification rapide |
| **Standard** | SEC-001, SEC-003, SEC-006 | Quotidien |
| **Full** | Tous SEC-* | Verification complete |
| **Security Audit** | Tous SEC-* + verification approfondie | Audit periodique |

### 12.4 Planification Tests Securite

| Suite | Frequence | Obligatoire |
|-------|-----------|-------------|
| SEC-001 | A chaque changement de permissions | Oui |
| SEC-003 | Quotidien | Oui |
| SEC-006 | Hebdomadaire | Oui |
| Security Audit | Mensuel | Oui |

### 12.5 Invariants de Securite des Tests

| Code | Invariant |
|------|-----------|
| **INV-UT-SEC-1** | Les tests de securite ne peuvent pas etre desactives |
| **INV-UT-SEC-2** | Un echec SEC-CRITICAL bloque les deployments |
| **INV-UT-SEC-3** | Les resultats SEC sont traces dans l'audit log |
| **INV-UT-SEC-4** | Les tests SEC-* requierent validation StrongFather |

### 12.6 References Securite

- [Security - Core Integration Map](..//..//..//..//cores//WorrySentinel//_index.md)
- [Security - Invariants & Guarantees](..//..//..//..//cores//WorrySentinel//_index.md)
- [MiyukiniAdmin - Threat Model Contract](../security/MiyukiniAdmin%20-%20Threat%20Model%20Contract.md)

---

## 13. Documents Associes

- [MiyukiniAdmin - Cycle Tests Contract](./MiyukiniAdmin%20-%20Cycle%20Tests%20Contract.md)
- [MiyukiniAdmin - DB Operations Contract](../database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md)
- [KindMother - Documentation Fondatrice](..//..//..//..//cores//KindMother//foundation//KindMother%20-%20Documentation%20Fondatrice.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Contrat de reference


