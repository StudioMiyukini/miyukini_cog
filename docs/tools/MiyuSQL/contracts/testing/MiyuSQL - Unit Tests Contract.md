# MiyuSQL — Unit Tests Contract

## 1. Contexte

Ce document definit le contrat pour les **tests unitaires** des Tools du kit MiyuSQL. Les tests unitaires verifient le comportement de chaque Tool (requete, transaction, cache, schema) sans modifier les donnees reelles et sans logique metier.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portee / Scope

Ce document definit :
- Les types de tests unitaires sur les Tools MiyuSQL (requete, transaction, cache, schema)
- Les criteres de succes et d'echec
- La non-destructivite et l'absence de modification de donnees reelles
- Les protocoles de verification

Ce document **ne couvre pas** :
- Les tests de cycle (voir MiyuSQL - Cycle Tests Contract)
- L'implementation technique des tests
- Les tests de coherence DB applicative (voir MiyukiniAdmin - Unit Tests Contract)

---

## 3. Principe Fondamental

### 3.1 Non-Destructivite

> **Les tests unitaires MiyuSQL verifient le comportement des Tools sans modifier les donnees reelles. Les tests d'ecriture utilisent une table/sandbox dediee (ex. MiyukiniSQLtest) ou des mocks ; aucune donnee metier n'est modifiee.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-UT-MSQL-1** | Aucune modification des donnees metier (tables de production) |
| **INV-UT-MSQL-2** | Les tests d'ecriture (execute, transaction) utilisent une sandbox ou une table dediee (ex. MiyukiniSQLtest) avec nettoyage obligatoire |
| **INV-UT-MSQL-3** | Tracabilite complete de chaque test (contexte, verdict, duree) |
| **INV-UT-MSQL-4** | Rapports conserves pour audit |

---

## 4. Categories de Tests par Tool

### 4.1 Tests Requete (tool.query.execute, tool.query.prepare)

| Code | Test | Description | Donnees |
|------|------|-------------|---------|
| **MSQL-Q-001** | Execute SELECT valide | Verifie qu'une requete SELECT mandatee s'execute et retourne un resultat (structure) | Lecture seule ; pas de modification |
| **MSQL-Q-002** | Prepare requete valide | Verifie que tool.query.prepare valide une requete sans l'executer | Aucune |
| **MSQL-Q-003** | Prepare requete invalide | Verifie que tool.query.prepare rejette une requete mal formee | Aucune |
| **MSQL-Q-004** | Timeout / limite | Verifie le respect du timeout et des limites (LIMIT) | Lecture seule |

### 4.2 Tests Transaction (tool.transaction.begin, commit, rollback)

| Code | Test | Description | Donnees |
|------|------|-------------|---------|
| **MSQL-T-001** | Begin / Commit | Verifie qu'une transaction begin + commit s'execute correctement | Sandbox ou table MiyukiniSQLtest ; nettoyage obligatoire |
| **MSQL-T-002** | Begin / Rollback | Verifie qu'un rollback annule les modifications | Sandbox ou table MiyukiniSQLtest ; nettoyage obligatoire |
| **MSQL-T-003** | Isolation | Verifie que les modifications ne sont pas visibles avant commit | Sandbox ; pas de donnees metier |
| **MSQL-T-004** | Transaction non ouverte | Verifie le refus d'execution (commit/rollback sans begin) selon contrat | Aucune |

### 4.3 Tests Cache (tool.cache.get, tool.cache.set, tool.cache.invalidate)

| Code | Test | Description | Donnees |
|------|------|-------------|---------|
| **MSQL-C-001** | Set / Get | Verifie qu'une valeur enregistree est recuperable | Cache dedie test ; pas de donnees metier |
| **MSQL-C-002** | Get absent | Verifie le comportement (absent / vide) pour une cle inexistante | Aucune |
| **MSQL-C-003** | Invalidate | Verifie qu'une entree invalidee n'est plus retournee | Cache dedie test |
| **MSQL-C-004** | TTL (si applicable) | Verifie l'expiration selon TTL | Cache dedie test |

### 4.4 Tests Schema (tool.schema.read)

| Code | Test | Description | Donnees |
|------|------|-------------|---------|
| **MSQL-S-001** | Lecture schema tables | Verifie que les metadonnees (tables) sont retournees | Lecture seule |
| **MSQL-S-002** | Lecture schema colonnes | Verifie que les colonnes et types sont retournes pour une table | Lecture seule |
| **MSQL-S-003** | Table inexistante | Verifie le comportement pour une table inexistante | Aucune |

---

## 5. Criteres de Succes et d'Echec

### 5.1 Criteres de Succes

| Criteres | Description |
|----------|-------------|
| **Execution conforme** | Le Tool s'execute comme specifie (pas d'exception non contractuelle) |
| **Resultat attendu** | Pour les lectures : structure et types conformes ; pour les ecritures (sandbox) : modification appliquee puis nettoyee |
| **Pas de fuite** | Aucune modification persistante en dehors de la sandbox ; sandbox nettoyee apres test |
| **Tracabilite** | Contexte, verdict, duree enregistres |

### 5.2 Criteres d'Echec

| Criteres | Description |
|----------|-------------|
| **Exception non contractuelle** | Le Tool leve une exception non prevue par le contrat |
| **Modification hors sandbox** | Une donnee metier est modifiee |
| **Nettoyage non effectue** | La sandbox ou la table dediee n'est pas nettoyee apres un test d'ecriture |
| **Timeout depasse** | Le test depasse le timeout configure sans resultat |

### 5.3 Verdicts

| Verdict | Description |
|---------|-------------|
| **PASS** | Tous les criteres de succes sont remplis |
| **WARN** | Comportement conforme avec alertes mineures (ex. performance) |
| **FAIL** | Un ou plusieurs criteres d'echec sont remplis |
| **SKIP** | Pre-condition non remplie (ex. sandbox indisponible) |
| **ERROR** | Erreur technique pendant le test (configuration, environnement) |

---

## 6. Protocole de Test

### 6.1 Execution d'un Test Unitaire MiyuSQL

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Chargement definition du test                            │
├─────────────────────────────────────────────────────────────┤
│ - ID du test (MSQL-*)                                        │
│ - ToolId concerne                                            │
│ - Parametres (sandbox, timeout, etc.)                         │
│ - Criteres de succes                                         │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. Preparation (si sandbox)                                  │
├─────────────────────────────────────────────────────────────┤
│ - Creation sandbox / table dediee si necessaire              │
│ - Via gouvernance (WriteIntent, KindMother)                   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. Execution du Tool (via gouvernance)                       │
├─────────────────────────────────────────────────────────────┤
│ - BondingBrother → Master Butler → WorrySentinel →           │
│   Caring Nanny → StrongFather → MiyuSQL Tool                 │
│ - Collecte resultat ou exception                              │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. Nettoyage (si sandbox)                                    │
├─────────────────────────────────────────────────────────────┤
│ - Suppression donnees test / tear-down sandbox                │
│ - Via gouvernance                                            │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 5. Evaluation et rapport                                     │
├─────────────────────────────────────────────────────────────┤
│ - Verdict (PASS/WARN/FAIL/SKIP/ERROR)                        │
│ - Details, duree, tracabilite                                │
└─────────────────────────────────────────────────────────────┘
```

### 6.2 Flux d'Execution (Schema)

```
Test Runner             BondingBrother         StrongFather      KindMother      MiyuSQL Tool
     │                         │                     │                │                │
     │──UnitTestRequest────────▶│                     │                │                │
     │  (test: MSQL-Q-001)      │                     │                │                │
     │                         │──Validate───────────▶│                │                │
     │                         │◀─ALLOW──────────────│                │                │
     │                         │──DataQuery──────────────────────────▶│                │
     │                         │                     │                │──Execute───────▶│
     │                         │                     │                │◀─Result────────│
     │                         │◀─QueryResults─────────────────────────│                │
     │◀─TestResults────────────│                     │                │                │
     │  (verdict, details)      │                     │                │                │
```

---

## 7. Suites de Tests

### 7.1 Suites Predefinies

| Suite | Tests inclus | Duree estimee | Usage |
|-------|--------------|---------------|-------|
| **Quick** | MSQL-Q-001, MSQL-Q-002, MSQL-S-001 | < 1 min | Verification rapide |
| **Standard** | Tous MSQL-Q, MSQL-S, MSQL-C (sans ecriture metier) | 2–5 min | Verification quotidienne |
| **Full** | Tous MSQL-* (avec sandbox pour T-*) | 5–10 min | Verification complete |

### 7.2 Configuration Suite

Les tests d'ecriture (MSQL-T-*, eventuellement MSQL-Q avec INSERT en sandbox) doivent utiliser une table dediee (ex. `MiyukiniSQLtest`) ou une sandbox avec nettoyage obligatoire. Aucune donnee metier ne doit etre modifiee.

---

## 8. References Croisees

| Document | Lien |
|----------|------|
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../../MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| MiyuSQL - Reference Outils | [MiyuSQL - Reference Outils](../../MiyuSQL%20-%20Reference%20Outils.md) |
| MiyuSQL - Cycle Tests Contract | [MiyuSQL - Cycle Tests Contract](./MiyuSQL%20-%20Cycle%20Tests%20Contract.md) |
| MiyuSQL - KindMother Integration Contract | [MiyuSQL - KindMother Integration Contract](../integration/MiyuSQL%20-%20KindMother%20Integration%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |

---

**Date de creation :** 2026-01-29  
**Version :** 1.0  
**Statut :** Contrat de reference
