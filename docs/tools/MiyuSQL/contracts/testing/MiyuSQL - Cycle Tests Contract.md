# MiyuSQL — Cycle Tests Contract

## 1. Contexte

Ce document definit le contrat pour les **tests de cycle** du kit MiyuSQL. Les tests de cycle verifient le chemin complet des donnees (WriteIntent, validations Cores, execution MiyuSQL) et peuvent etre executes par MiyukiniAdmin pour valider MiyuSQL de facon precise.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portee / Scope

Ce document definit :
- Les types de tests de cycle applicables a MiyuSQL (performance, latence, chemin complet)
- Le **test chemin complet MiyuSQL (MiyukiniSQLtest)** : scenario E2E (WriteIntent → validations Cores → creation table → colonne → donnee → lecture → affichage → suppression)
- Le lien avec MiyukiniAdmin comme executant du test
- Les metriques et criteres de succes

Ce document **ne couvre pas** :
- Les tests unitaires (voir MiyuSQL - Unit Tests Contract)
- L'implementation technique des tests

---

## 3. Principe Fondamental

### 3.1 Environnement de Diagnostic

> **Les tests de cycle MiyuSQL sont executes dans un environnement de diagnostic controle. Le test chemin complet utilise une table dediee (MiyukiniSQLtest) ; aucune donnee metier n'est modifiee.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-CT-MSQL-1** | Aucune modification des donnees metier (tables de production) |
| **INV-CT-MSQL-2** | La table MiyukiniSQLtest est reservee au test ; isolation et nettoyage obligatoires |
| **INV-CT-MSQL-3** | Tracabilite complete de chaque etape du test |
| **INV-CT-MSQL-4** | Validation StrongFather (et Cores) avant toute ecriture |
| **INV-CT-MSQL-5** | Rapports conserves pour audit |

---

## 4. Types de Tests de Cycle MiyuSQL

### 4.1 Tests de Latence (via MiyuSQL)

| Test | Description | Cible |
|------|-------------|-------|
| **MSQL-LAT-001** | Latence tool.query.execute (SELECT) | < seuil configure (ex. 50ms) |
| **MSQL-LAT-002** | Latence tool.schema.read | < seuil configure |
| **MSQL-LAT-003** | Latence chemin complet (BondingBrother → KindMother → MiyuSQL) | < 100ms |

### 4.2 Tests de Performance

| Test | Description | Impact |
|------|-------------|--------|
| **MSQL-PERF-001** | Debit lectures (SELECT via MiyuSQL) | Sandbox ou table dediee |
| **MSQL-PERF-002** | Debit ecritures (INSERT via MiyuSQL, table MiyukiniSQLtest) | Table dediee ; nettoyage obligatoire |

### 4.3 Test Chemin Complet (MiyukiniSQLtest)

Voir section 5.

---

## 5. Test Chemin Complet MiyuSQL — MiyukiniSQLtest

### 5.1 Objectif

Ce test verifie le **chemin complet d'une donnee DB** : de l'emission d'une WriteIntent jusqu'a la suppression de la donnee, en passant par toutes les validations Cores, la creation de la table dediee `MiyukiniSQLtest`, la creation d'une colonne, l'insertion d'une donnee aleatoire, la lecture, l'affichage et la suppression. MiyukiniAdmin peut executer ce test pour valider MiyuSQL de facon precise.

### 5.2 Table Dédiée

| Element | Valeur |
|---------|--------|
| **Nom de la table** | `MiyukiniSQLtest` |
| **Usage** | Reservee a ce test ; aucune donnee metier |
| **Isolation** | La table est creee, utilisee et supprimee (ou videe) dans le cadre du test ; nettoyage obligatoire en fin de test |

### 5.3 Scenario E2E (Etapes)

Les etapes suivantes sont executees dans l'ordre. Chaque etape doit reussir pour que le test soit considere reussi.

| Etape | Description | Acteurs / Tools |
|-------|-------------|-----------------|
| **1. WriteIntent** | Emission d'une intention d'ecriture (creation de structure + donnee). L'Operateur (ou MiyukiniAdmin) emet une WriteIntent pour creer la table, la colonne, inserer une donnee, puis supprimer la donnee (ou plusieurs WriteIntent selon le modele). | Operateur / MiyukiniAdmin → BondingBrother |
| **2. Validations Cores** | Parcours explicite : BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather. La decision doit etre ALLOW pour chaque operation d'ecriture. | BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather |
| **3. Creation table** | Creation de la table dediee `MiyukiniSQLtest` via la gouvernance, sous autorite KindMother. Utilisation de tool.query.execute (ou equivalent) avec DDL mandate. | KindMother, MiyuSQL (tool.query.execute) |
| **4. Creation colonne** | Ajout d'une colonne dans `MiyukiniSQLtest`. Nom et type sont definis dans le contrat (ex. colonne `test_value` de type `text` ou `uuid`). | KindMother, MiyuSQL |
| **5. Creation donnee** | Insertion d'une donnee aleatoire dans la colonne. La donnee est generee de facon deterministe (seed documentee) ou aleatoire ; la valeur est enregistree pour verification en etape 6. | WriteIntent acceptee → KindMother, MiyuSQL |
| **6. Lecture** | Lecture de la donnee inseree via le flux gouverne (SELECT via MiyuSQL sous autorite KindMother). | KindMother, MiyuSQL (tool.query.execute SELECT) |
| **7. Affichage** | Verification que la donnee peut etre exposee (ex. pour l'UI MiyukiniAdmin DB Management). La valeur lue doit correspondre a la valeur inseree. | MiyukiniAdmin (affichage) / assertion dans le test |
| **8. Suppression** | Suppression de la donnee (DELETE). Nettoyage optionnel : suppression de la table ou tear-down documente. | WriteIntent acceptee → KindMother, MiyuSQL |

### 5.4 Specification de la Table et de la Colonne (Contrat)

| Element | Valeur |
|---------|--------|
| **Table** | `MiyukiniSQLtest` |
| **Colonne (ex.)** | `test_value` (type `text`) ou `id` (type `uuid`), `payload` (type `text`) — a fixer selon implementation |
| **Donnee aleatoire** | Genération deterministe (seed documentee) ou aleatoire ; valeur enregistree pour assertion (lecture = valeur inseree) |

### 5.5 Criteres de Succes du Test MiyukiniSQLtest

| Critere | Description |
|---------|-------------|
| **C1** | WriteIntent emise et acceptee pour chaque operation d'ecriture (creation table, colonne, INSERT, DELETE) |
| **C2** | Validations Cores (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather) executees et ALLOW obtenu |
| **C3** | Table `MiyukiniSQLtest` creee |
| **C4** | Colonne creee dans la table |
| **C5** | Donnee aleatoire inseree et persistee |
| **C6** | Lecture retourne la meme valeur que celle inseree |
| **C7** | Affichage (ou assertion) confirme la valeur |
| **C8** | Donnee supprimee ; nettoyage effectue (table videe ou supprimee) |

### 5.6 Verdicts

| Verdict | Description |
|---------|-------------|
| **PASS** | Toutes les etapes 1 à 8 reussies et tous les criteres C1–C8 remplis |
| **FAIL** | Une etape echoue ou un critere n'est pas rempli |
| **ERROR** | Erreur technique (environnement, configuration, gouvernance indisponible) |

### 5.7 Executant : MiyukiniAdmin

MiyukiniAdmin peut executer ce test pour verifier le chemin complet MiyuSQL. La reference croisee est etablie dans [MiyukiniAdmin - Cycle Tests Contract](../../../core/MiyukiniAdmin/contracts/testing/MiyukiniAdmin%20-%20Cycle%20Tests%20Contract.md) (section MiyuSQL Full Path Test / MiyukiniSQLtest), qui pointe vers ce contrat pour la specification du scenario et des criteres.

---

## 6. Protocole d'Execution (Test MiyukiniSQLtest)

### 6.1 Phases

```
┌─────────────────────────────────────────────────────────────┐
│ Phase 1: Preparation                                         │
├─────────────────────────────────────────────────────────────┤
│ - Verification pre-conditions (gouvernance, etat systeme)   │
│ - Verification absence de table MiyukiniSQLtest ou decision │
│   de reutilisation / nettoyage                               │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Phase 2: Execution (Etapes 1 à 8)                            │
├─────────────────────────────────────────────────────────────┤
│ - WriteIntent → Validations Cores → Creation table           │
│ - Creation colonne → Insertion donnee → Lecture             │
│ - Affichage / assertion → Suppression → Nettoyage            │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Phase 3: Rapport                                             │
├─────────────────────────────────────────────────────────────┤
│ - Verdict (PASS/FAIL/ERROR)                                  │
│ - Details par etape                                          │
│ - Stockage pour audit                                        │
└─────────────────────────────────────────────────────────────┘
```

### 6.2 Flux Simplifie

```
MiyukiniAdmin            BondingBrother         StrongFather      KindMother      MiyuSQL
     │                         │                     │                │                │
     │──MiyukiniSQLtestTest────▶│                     │                │                │
     │                         │──Validate───────────▶│                │                │
     │                         │◀─ALLOW──────────────│                │                │
     │                         │──WriteIntent (create table)──────────▶│                │
     │                         │                     │                │──Execute──────▶│
     │                         │                     │                │◀─Done──────────│
     │                         │  ... (colonne, insert, read, delete)  │                │
     │◀─TestResult─────────────│                     │                │                │
     │  (verdict, details)      │                     │                │                │
```

---

## 7. Metriques et Rapports

### 7.1 Metriques Collectees (Optionnel)

| Metrique | Description |
|----------|-------------|
| `step_duration_ms` | Duree par etape (1 à 8) |
| `total_duration_ms` | Duree totale du test |
| `validation_latency_ms` | Latence cumulee des validations Cores |

### 7.2 Structure Rapport (Resume)

```json
{
  "test_id": "MiyukiniSQLtest",
  "timestamp": "2026-01-29T12:00:00Z",
  "verdict": "PASS",
  "steps": [
    {"step": 1, "name": "WriteIntent", "status": "OK"},
    {"step": 2, "name": "Validations Cores", "status": "OK"},
    {"step": 3, "name": "Creation table", "status": "OK"},
    {"step": 4, "name": "Creation colonne", "status": "OK"},
    {"step": 5, "name": "Creation donnee", "status": "OK"},
    {"step": 6, "name": "Lecture", "status": "OK"},
    {"step": 7, "name": "Affichage", "status": "OK"},
    {"step": 8, "name": "Suppression", "status": "OK"}
  ],
  "criteria_met": true,
  "duration_ms": 450
}
```

---

## 8. References Croisees

| Document | Lien |
|----------|------|
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../../MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| MiyuSQL - Unit Tests Contract | [MiyuSQL - Unit Tests Contract](./MiyuSQL%20-%20Unit%20Tests%20Contract.md) |
| MiyuSQL - KindMother Integration Contract | [MiyuSQL - KindMother Integration Contract](../integration/MiyuSQL%20-%20KindMother%20Integration%20Contract.md) |
| MiyukiniAdmin - Cycle Tests Contract | [MiyukiniAdmin - Cycle Tests Contract](../../../core/MiyukiniAdmin/contracts/testing/MiyukiniAdmin%20-%20Cycle%20Tests%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |

---

**Date de creation :** 2026-01-29  
**Version :** 1.0  
**Statut :** Contrat de reference
