# MiyukiniAdmin — Cycle Tests Contract

## 1. Contexte

Ce document definit le contrat pour les **tests de cycle** dans MiyukiniAdmin. Les tests de cycle sont des tests de performance, de latence et de montee en charge qui evaluent le comportement du systeme sous differentes conditions.

Ces tests sont executes dans un **environnement de diagnostic**, pas en production cachee.

## 2. Portee / Scope

Ce document definit :
- Les types de tests de cycle disponibles
- Les protocoles d'execution
- Les metriques collectees pendant les tests
- Les rapports generes
- Les contraintes de securite

Ce document **ne couvre pas** :
- Les tests unitaires (voir Unit Tests Contract)
- L'implementation technique des tests
- L'interface utilisateur (voir UI documentation)

---

## 3. Principe Fondamental

### 3.1 Environnement de Diagnostic

> **Les tests de cycle sont executes dans un environnement de diagnostic controle, avec tracabilite complete et sans impact sur les donnees de production.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-CT-1** | Aucune modification des donnees de production |
| **INV-CT-2** | Tracabilite complete de chaque test |
| **INV-CT-3** | Validation StrongFather avant tests impactants |
| **INV-CT-4** | Rapports conserves pour audit |
| **INV-CT-5** | Arret automatique si seuils critiques atteints |

---

## 4. Types de Tests de Cycle

### 4.1 Tests de Performance

| Test | Description | Impact |
|------|-------------|--------|
| **PERF-001** | Latence requetes simples | Faible |
| **PERF-002** | Latence requetes complexes | Moyen |
| **PERF-003** | Debit lectures | Moyen |
| **PERF-004** | Debit ecritures (sandbox) | Moyen |
| **PERF-005** | Performance cache | Faible |

### 4.2 Tests de Latence

| Test | Description | Cible |
|------|-------------|-------|
| **LAT-001** | Latence decisionnelle StrongFather | < 10ms |
| **LAT-002** | Latence persistance KindMother | < 50ms |
| **LAT-003** | Latence mediation BondingBrother | < 5ms |
| **LAT-004** | Latence bout-en-bout | < 100ms |

### 4.3 Tests de Montee en Charge

| Test | Description | Parametres |
|------|-------------|------------|
| **LOAD-001** | Charge progressive | 10 → 100 → 500 req/s |
| **LOAD-002** | Pic de charge | Burst 1000 req/s |
| **LOAD-003** | Charge soutenue | 200 req/s pendant 5 min |
| **LOAD-004** | Stress test | Jusqu'a saturation |

### 4.4 Tests de Resilience

| Test | Description | Scenario |
|------|-------------|----------|
| **RES-001** | Degradation gracieuse | Reduction ressources |
| **RES-002** | Recovery apres pic | Post-charge elevee |
| **RES-003** | Stabilite long terme | 1 heure charge moderee |

---

## 5. Protocole d'Execution

### 5.1 Pre-conditions

Avant tout test de cycle :

1. **Validation operateur** : Operateur authentifie avec role adequat
2. **Verification etat systeme** : Etat nominal (T0 ou T1)
3. **Validation StrongFather** : Pour tests impactants (LOAD, STRESS)
4. **Notification CaringNanny** : Debut phase test

### 5.2 Phases d'Execution

```
┌─────────────────────────────────────────────────────────────┐
│ Phase 1: Preparation                                         │
├─────────────────────────────────────────────────────────────┤
│ - Verification pre-conditions                               │
│ - Baseline metriques                                        │
│ - Configuration parametres                                   │
│ - Creation sandbox si necessaire                            │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Phase 2: Execution                                           │
├─────────────────────────────────────────────────────────────┤
│ - Lancement test                                            │
│ - Collecte metriques temps reel                             │
│ - Monitoring seuils critiques                               │
│ - Arret si seuils depasses                                  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Phase 3: Analyse                                             │
├─────────────────────────────────────────────────────────────┤
│ - Agregation resultats                                      │
│ - Calcul statistiques                                       │
│ - Comparaison baseline                                      │
│ - Detection anomalies                                       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Phase 4: Rapport                                             │
├─────────────────────────────────────────────────────────────┤
│ - Generation rapport                                        │
│ - Stockage pour audit                                       │
│ - Notification resultats                                    │
│ - Nettoyage sandbox                                         │
└─────────────────────────────────────────────────────────────┘
```

### 5.3 Flux d'Execution

```
MiyukiniAdmin            BondingBrother         StrongFather      CaringNanny
     │                         │                     │                │
     │──TestRequest────────────▶│                     │                │
     │  (type: LOAD-001)        │                     │                │
     │                         │                     │                │
     │                         │──ValidateTest───────▶│                │
     │                         │                     │                │
     │                         │◀─Approved───────────│                │
     │                         │                     │                │
     │                         │──NotifyTestStart────────────────────▶│
     │                         │                     │                │
     │◀─TestStarted────────────│                     │                │
     │                         │                     │                │
     │  ═══ EXECUTION TEST ════│                     │                │
     │                         │                     │                │
     │◀─TestProgress───────────│                     │                │
     │  (metriques temps reel) │                     │                │
     │                         │                     │                │
     │◀─TestCompleted──────────│                     │                │
     │  (rapport final)        │                     │                │
     │                         │                     │                │
```

---

## 6. Metriques Collectees

### 6.1 Metriques de Performance

| Metrique | Description | Unite |
|----------|-------------|-------|
| `test.throughput` | Debit | Requests/sec |
| `test.latency.avg` | Latence moyenne | ms |
| `test.latency.p50` | Percentile 50 | ms |
| `test.latency.p95` | Percentile 95 | ms |
| `test.latency.p99` | Percentile 99 | ms |
| `test.errors.count` | Nombre d'erreurs | Count |
| `test.errors.rate` | Taux d'erreur | Pourcentage |

### 6.2 Metriques Systeme pendant Test

| Metrique | Description |
|----------|-------------|
| `test.cpu.peak` | Pic CPU pendant test |
| `test.ram.peak` | Pic RAM pendant test |
| `test.db.connections.peak` | Pic connexions DB |
| `test.network.peak` | Pic reseau |

### 6.3 Format Resultats

```json
{
  "test_id": "uuid-test-001",
  "test_type": "LOAD-001",
  "timestamp_start": "2026-01-28T12:00:00Z",
  "timestamp_end": "2026-01-28T12:05:30Z",
  "duration_seconds": 330,
  "status": "COMPLETED",
  "parameters": {
    "target_rps": [10, 100, 500],
    "duration_per_step_seconds": 60
  },
  "results": {
    "total_requests": 125000,
    "successful_requests": 124950,
    "failed_requests": 50,
    "error_rate": 0.04,
    "throughput": {
      "avg_rps": 378.8,
      "max_rps": 512.3,
      "min_rps": 10.2
    },
    "latency": {
      "avg_ms": 25.5,
      "min_ms": 5.2,
      "max_ms": 450.8,
      "p50_ms": 22.0,
      "p95_ms": 45.0,
      "p99_ms": 120.0
    }
  },
  "system_impact": {
    "cpu_peak_percent": 75.5,
    "ram_peak_percent": 60.2,
    "db_connections_peak": 45
  },
  "verdict": "PASS",
  "notes": "Performance dans les limites acceptables"
}
```

---

## 7. Seuils et Criteres

### 7.1 Seuils d'Arret Automatique

| Condition | Seuil | Action |
|-----------|-------|--------|
| CPU | > 95% pendant 30s | Arret test |
| RAM | > 95% | Arret test |
| Error rate | > 10% | Arret test |
| Latency P99 | > 1000ms | Arret test |
| DB Pool | Saturation | Arret test |

### 7.2 Criteres de Succes

| Test Type | Critere |
|-----------|---------|
| **PERF** | Latence P95 < seuil configure |
| **LAT** | Latence moyenne < cible |
| **LOAD** | Maintien debit sans degradation > 10% |
| **RES** | Recovery complet en < 30s |

### 7.3 Verdicts Possibles

| Verdict | Description |
|---------|-------------|
| **PASS** | Tous les criteres respectes |
| **WARN** | Criteres respectes avec alertes |
| **FAIL** | Un ou plusieurs criteres non respectes |
| **ABORT** | Test interrompu (seuils critiques) |

---

## 8. Tests Predefinies

### 8.1 Suite Standard

| Suite | Tests inclus | Duree estimee |
|-------|--------------|---------------|
| **Quick Check** | PERF-001, LAT-001 | 2 minutes |
| **Standard** | PERF-001-005, LAT-001-004 | 10 minutes |
| **Full** | Tous PERF, LAT, LOAD-001-003 | 30 minutes |
| **Stress** | LOAD-004, RES-001-003 | 1+ heure |

### 8.2 Configuration Suite Standard

```json
{
  "suite": "standard",
  "tests": [
    {"id": "PERF-001", "iterations": 1000},
    {"id": "PERF-002", "iterations": 500},
    {"id": "PERF-003", "duration_sec": 60},
    {"id": "LAT-001", "iterations": 500},
    {"id": "LAT-002", "iterations": 500},
    {"id": "LAT-003", "iterations": 500},
    {"id": "LAT-004", "iterations": 200}
  ],
  "options": {
    "stop_on_first_failure": false,
    "parallel_tests": false
  }
}
```

---

## 9. Rapports

### 9.1 Structure Rapport

| Section | Contenu |
|---------|---------|
| **Resume** | Verdict global, metriques cles |
| **Configuration** | Parametres du test |
| **Resultats detailles** | Metriques par test |
| **Graphiques** | Courbes de performance |
| **Comparaison** | vs baseline ou test precedent |
| **Recommandations** | Actions suggerees |

### 9.2 Export Formats

| Format | Usage |
|--------|-------|
| **JSON** | Integration automatisee |
| **HTML** | Consultation navigateur |
| **PDF** | Archivage officiel |
| **CSV** | Analyse externe |

---

## 10. Securite et Contraintes

### 10.1 Roles Autorises

| Test Type | Role Minimum |
|-----------|--------------|
| PERF | Operator |
| LAT | Operator |
| LOAD-001/002/003 | Admin |
| LOAD-004 (Stress) | Admin + Justification |
| RES | Admin |

### 10.2 Contraintes de Planification

| Contrainte | Description |
|------------|-------------|
| **Heures creuses** | Tests LOAD recommandes hors heures de pointe |
| **Intervalle** | Minimum 5 minutes entre tests LOAD |
| **Concurrent** | Maximum 1 test de charge a la fois |

---

## 11. Implications Securite — DoS et Stress Testing

### 11.1 Risques de Securite des Tests de Charge

Les tests de cycle (LOAD, STRESS) peuvent involontairement creer des conditions de **Denial of Service (DoS)**. Cette section definit les controles obligatoires.

| Risque | Description | Impact |
|--------|-------------|--------|
| **Auto-DoS** | Test de charge qui sature le systeme de production | Service indisponible |
| **Resource Exhaustion** | Epuisement CPU/RAM/connexions | Degradation generale |
| **Cascade Failure** | Echec d'un composant entraine les autres | Panne complete |
| **Data Corruption** | Charge excessive cause des erreurs de persistance | Perte de donnees |

### 11.2 Controles Anti-DoS Obligatoires

> **INV-CT-SEC-1 : Tout test de charge doit avoir des seuils d'arret automatiques.**

| Controle | Seuil | Action |
|----------|-------|--------|
| **CPU Guardian** | > 90% pendant 15s | Arret immediat |
| **Memory Guardian** | > 90% | Arret immediat |
| **Connection Guardian** | Pool saturation 95% | Arret immediat |
| **Error Rate Guardian** | > 5% sur 30s | Reduction charge 50% |
| **Latency Guardian** | P99 > 2000ms | Reduction charge 50% |
| **Health Guardian** | CaringNanny status != HEALTHY | Arret immediat |

### 11.3 Isolation des Tests de Charge

| Exigence | Implementation |
|----------|----------------|
| **Environnement dedie** | Tests LOAD-003/004 sur replique ou staging |
| **Ressources isolees** | Pool de connexions separe pour tests |
| **Quota de charge** | Maximum 50% de la capacite totale |
| **Notification** | Tous les operateurs informes avant LOAD test |

### 11.4 Stress Test (LOAD-004) — Controles Speciaux

Le Stress Test (jusqu'a saturation) presente des risques majeurs :

| Controle | Obligation |
|----------|------------|
| **Double validation** | StrongFather + TAMR requis |
| **Justification ecrite** | Motif et plan de test documentes |
| **Fenetre de maintenance** | Execution hors heures de production |
| **Monitoring renforce** | CaringNanny en mode surveillance maximale |
| **Kill switch** | Bouton d'arret d'urgence actif |
| **Rollback prepare** | Plan de restauration pre-valide |

### 11.5 Detection d'Attaque pendant Tests

Les tests de charge peuvent masquer des attaques reelles :

| Indicateur | Detection | Action |
|------------|-----------|--------|
| **Patterns anormaux** | Trafic non conforme au test planifie | Alerte + Investigation |
| **Sources inattendues** | Requetes hors du perimetre test | Blocage automatique |
| **Escalade privileges** | Tentative d'elevation pendant charge | Arret test + Alerte TAMR |
| **Exfiltration** | Transfert donnees anormal | Arret test + Isolation |

### 11.6 Rapport de Securite Post-Test

Tout test de charge genere un rapport de securite :

```json
{
  "test_id": "uuid-test-001",
  "security_report": {
    "guardians_triggered": [],
    "anomalies_detected": [],
    "max_resource_usage": {
      "cpu_peak": 75.5,
      "ram_peak": 60.2,
      "pool_peak": 90
    },
    "security_events_during_test": 0,
    "containment_actions": [],
    "clearance": "SAFE"
  }
}
```

### 11.7 Adaptation par Niveau de Confiance (T0-T4)

| Niveau | Tests de Charge Autorises |
|--------|---------------------------|
| **T0** | Tous tests autorises |
| **T1** | PERF et LAT uniquement, LOAD avec approbation |
| **T2** | PERF-001/002 uniquement |
| **T3** | Aucun test de charge |
| **T4** | Aucun test de charge |

### 11.8 Adaptation par Niveau de Securite (0-4)

| Niveau | Restrictions |
|--------|--------------|
| **0-1** | Tous tests autorises |
| **2** | LOAD-004 interdit sans TAMR |
| **3** | LOAD-003/004 interdits, LOAD-001/002 avec monitoring renforce |
| **4** | Uniquement PERF/LAT, aucun LOAD |

### 11.9 References Securite

- [Security - Core Integration Map](../../../../security/architecture/Security%20-%20Core%20Integration%20Map.md)
- [Security - Documentation Fondatrice](../../../../security/foundation/Security%20-%20Documentation%20Fondatrice.md)
- [CaringNanny - Documentation Fondatrice](../../../CaringNanny/foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

## 12. MiyuSQL Full Path Test (MiyukiniSQLtest)

### 12.1 Objectif

MiyukiniAdmin peut executer un **test de cycle dedie a MiyuSQL** qui verifie le **chemin complet d'une donnee DB** : WriteIntent, validations Cores (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather), creation de la table dediee `MiyukiniSQLtest`, creation d'une colonne, insertion d'une donnee aleatoire, lecture, affichage et suppression. Ce test valide MiyuSQL de facon precise.

### 12.2 Specification du Test

La specification complete du scenario (etapes, criteres de succes, verdicts, table dediee) est definie dans le contrat MiyuSQL :

- **[MiyuSQL - Cycle Tests Contract](../../../tools/MiyuSQL/contracts/testing/MiyuSQL%20-%20Cycle%20Tests%20Contract.md)** — section « Test chemin complet MiyuSQL — MiyukiniSQLtest ».

### 12.3 Role de MiyukiniAdmin

MiyukiniAdmin est l'**executant** du test : il emet les WriteIntent, declenche le flux gouverné, et verifie les resultats (lecture, affichage). L'environnement de diagnostic et la table `MiyukiniSQLtest` sont isoles ; aucun impact sur les donnees metier.

### 12.4 Reference Croisee

| Document | Lien |
|----------|------|
| Specification du test MiyukiniSQLtest | [MiyuSQL - Cycle Tests Contract](../../../tools/MiyuSQL/contracts/testing/MiyuSQL%20-%20Cycle%20Tests%20Contract.md) |

---

## 13. Documents Associes

- [MiyukiniAdmin - Unit Tests Contract](./MiyukiniAdmin%20-%20Unit%20Tests%20Contract.md)
- [MiyukiniAdmin - Architecture & Flows](../../architecture/MiyukiniAdmin%20-%20Architecture%20&%20Flows.md)
- [StrongFather - Documentation Fondatrice](../../../StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md)
- [MiyuSQL - Cycle Tests Contract](../../../tools/MiyuSQL/contracts/testing/MiyuSQL%20-%20Cycle%20Tests%20Contract.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Contrat de reference
