# MiyukiniAdmin — DB Management Interface

## 1. Contexte

Ce document definit la specification de l'**interface de gestion de base de donnees** dans MiyukiniAdmin. Cette interface est inspiree de PHPMyAdmin et permet l'exploration, la maintenance et les operations sur les donnees.

## 2. Portee / Scope

Ce document definit :
- La structure de l'interface DB
- Les vues d'exploration
- Les operations de maintenance
- Le mode recovery (UI)

Ce document **ne couvre pas** :
- Les autres interfaces
- Les contrats de donnees (voir Database contracts)
- L'implementation technique

---

## 3. Structure de l'Interface DB

### 3.1 Layout Principal

```
┌─────────────────────────────────────────────────────────────────────────┐
│  MiyukiniAdmin > Database                    [Alerts: 0] [User] [L2]    │
├────────────┬────────────────────────────────────────────────────────────┤
│            │  Database Management                      [Refresh] [...]   │
│ Dashboard  │────────────────────────────────────────────────────────────│
│ Metriques  │                                                            │
│ ► Database │  ┌──────────────────────────────────────────────────────┐ │
│   Tables   │  │ DB Stats: 45 tables | 10.5 GB | Pool: 15/50          │ │
│   Query    │  └──────────────────────────────────────────────────────┘ │
│   Maint.   │                                                            │
│   Recovery │  TABLES                                        [Search: __]│
│ Tests      │  ┌──────┬─────────────┬────────┬─────────┬───────────────┐│
│ Securite   │  │ Name │ Rows        │ Size   │ Indexes │ Actions       ││
│ Logs       │  ├──────┼─────────────┼────────┼─────────┼───────────────┤│
│            │  │users │ 15,234      │ 2.5 MB │ 3       │[View][Struct] ││
│            │  │orders│ 45,892      │ 8.2 MB │ 5       │[View][Struct] ││
│            │  │prods │ 3,456       │ 1.1 MB │ 2       │[View][Struct] ││
│            │  │logs  │ 1,234,567   │ 45 MB  │ 4       │[View][Struct] ││
│            │  │...   │             │        │         │               ││
│            │  ├──────┴─────────────┴────────┴─────────┴───────────────┤│
│            │  │ Page 1 of 5 | Showing 1-10 of 45  [<] [1][2][3] [>]   ││
│            │  └──────────────────────────────────────────────────────┘ │
│            │                                                            │
│            │  QUICK ACTIONS                                             │
│            │  [Run Validation] [Optimize All] [Export Schema]           │
│            │                                                            │
├────────────┴────────────────────────────────────────────────────────────┤
│  v1.0.0 | DB: PostgreSQL 15.2 | Connected | Trust: T0                   │
└─────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Sous-Sections

| Section | Description |
|---------|-------------|
| **Tables** | Liste et exploration des tables |
| **Query** | Console de requetes (lecture seule) |
| **Maintenance** | Operations de maintenance |
| **Recovery** | Mode recovery (conditions strictes) |

---

## 4. Vue Tables

### 4.1 Liste des Tables

```
┌────────────────────────────────────────────────────────────────────────┐
│  Tables                                                    [Search: __]│
├──────────────┬────────────┬──────────┬─────────┬──────────┬───────────┤
│ Table        │ Rows       │ Size     │ Indexes │ Last Mod │ Actions   │
├──────────────┼────────────┼──────────┼─────────┼──────────┼───────────┤
│ □ users      │ 15,234     │ 2.5 MB   │ 3       │ 2h ago   │ [▼]       │
│ □ orders     │ 45,892     │ 8.2 MB   │ 5       │ 5m ago   │ [▼]       │
│ □ products   │ 3,456      │ 1.1 MB   │ 2       │ 1d ago   │ [▼]       │
│ □ audit_logs │ 1,234,567  │ 45 MB    │ 4       │ now      │ [▼]       │
│ □ sessions   │ 892        │ 128 KB   │ 2       │ 1m ago   │ [▼]       │
├──────────────┴────────────┴──────────┴─────────┴──────────┴───────────┤
│ [Select All] [Deselect]    With selected: [Optimize] [Export]         │
├────────────────────────────────────────────────────────────────────────┤
│ ◄ 1 2 3 4 5 ►   Show: [10 ▼] per page                                 │
└────────────────────────────────────────────────────────────────────────┘
```

### 4.2 Actions par Table

| Action | Icone | Description |
|--------|-------|-------------|
| View Data | 👁 | Voir les donnees |
| Structure | 🏗 | Voir le schema |
| Indexes | 📑 | Voir les index |
| Export | 📤 | Exporter CSV/JSON |
| Stats | 📊 | Statistiques detaillees |

### 4.3 Create a new table (flux inspire Supabase)

Objectif d'interface : formulaire "Create a new table under [schema]" (ex. public), inspire du Table Editor Supabase.

- **Champs :** Name (obligatoire), Description (optionnel).
- **Options table :**
  - **Enable Realtime** : optionnel ; transposition = ecoute des changements (WebSockets ou polling). Voir [Reference SQL et DB](../reference/MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md). MiyukiniAdmin etant un service hors-bord, aucune option RLS (Row Level Security) n'est proposee ; l'auth est propre au service admin.
- **Section Columns :** pour chaque colonne : Name, Type (liste types : int2, int4, int8, float4, float8, numeric, json, jsonb, text, varchar, uuid, timestamptz, bool, etc.), Default Value, Primary (checkbox). Boutons "About data types", "Import data from CSV" si dans le perimetre. Reference : [Reference SQL et DB](../reference/MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md).
- **Section Foreign keys :** definition des cles etrangeres (optionnel).
- **Actions :** Cancel, Save. La creation effective passe par validation StrongFather et execution via KindMother (migration ou mode Recovery selon contrat). Voir [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md).

---

## 5. Vue Donnees Table

**Objectifs d'interface (inspiration Supabase) :** Filter, Sort, Insert (ajout de ligne), Index Advisor, Enable Realtime, Role (ex. postgres), Import data from CSV (ou drag-and-drop). Ces elements sont relies aux contrats READ-*, MIG-*, Emergency DB Access selon le mode (normal vs Recovery). Aucune notion RLS (MiyukiniAdmin = service hors-bord, auth propre). Voir [Reference SQL et DB](../reference/MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md) et [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md).

### 5.1 Exploration des Donnees

```
┌────────────────────────────────────────────────────────────────────────┐
│  Table: users                                                          │
│  [◄ Back to Tables]                                                    │
├────────────────────────────────────────────────────────────────────────┤
│  [Structure] [Data ◄] [Indexes] [Stats] [Export]                       │
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│  Filters: [Add Filter +]                                               │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │ status = 'active' [x]                                            │ │
│  └──────────────────────────────────────────────────────────────────┘ │
│                                                                        │
│  ┌──────┬──────────────────────┬─────────────┬────────────┬─────────┐│
│  │ id   │ email                │ status      │ created_at │ [...]   ││
│  ├──────┼──────────────────────┼─────────────┼────────────┼─────────┤│
│  │ 1    │ john@example.com     │ active      │ 2026-01-15 │ [...]   ││
│  │ 2    │ jane@example.com     │ active      │ 2026-01-16 │ [...]   ││
│  │ 3    │ bob@example.com      │ active      │ 2026-01-17 │ [...]   ││
│  │ ...  │                      │             │            │         ││
│  ├──────┴──────────────────────┴─────────────┴────────────┴─────────┤│
│  │ Showing 1-50 of 15,234 (filtered: 12,456)  [<] [1][2][3] [>]    ││
│  └──────────────────────────────────────────────────────────────────┘ │
│                                                                        │
│  [Export Results] [Copy Query]                                         │
│                                                                        │
└────────────────────────────────────────────────────────────────────────┘
```

### 5.2 Filtres

```
┌────────────────────────────────────────────────────────────────────────┐
│  Add Filter                                                            │
├────────────────────────────────────────────────────────────────────────┤
│  Column: [email        ▼]                                              │
│  Operator: [contains   ▼]                                              │
│  Value: [example.com   ]                                               │
│                                                                        │
│  [Cancel]                                        [Apply Filter]        │
└────────────────────────────────────────────────────────────────────────┘
```

### 5.3 Detail Row (Click sur row)

```
┌────────────────────────────────────────────────────────────────────────┐
│  Row Details                                                     [X]   │
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│  id            │ 1                                                     │
│  email         │ john@example.com                                      │
│  status        │ active                                                │
│  created_at    │ 2026-01-15T10:30:00Z                                 │
│  updated_at    │ 2026-01-27T14:22:15Z                                 │
│  metadata      │ {"source": "web", "verified": true}                  │
│                                                                        │
│  Related:                                                              │
│  - orders (15 records)                                                 │
│  - sessions (2 records)                                                │
│                                                                        │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 6. Vue Structure Table

### 6.1 Schema

```
┌────────────────────────────────────────────────────────────────────────┐
│  Table: users > Structure                                              │
│  [◄ Back]                                                              │
├────────────────────────────────────────────────────────────────────────┤
│  [Structure ◄] [Data] [Indexes] [Stats] [Export]                       │
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│  COLUMNS                                                               │
│  ┌────────────────┬──────────────┬──────────┬─────────┬──────────────┐│
│  │ Name           │ Type         │ Nullable │ Default │ Extra        ││
│  ├────────────────┼──────────────┼──────────┼─────────┼──────────────┤│
│  │ id             │ UUID         │ NO       │ gen()   │ PRIMARY KEY  ││
│  │ email          │ VARCHAR(255) │ NO       │ -       │ UNIQUE       ││
│  │ status         │ VARCHAR(50)  │ NO       │ pending │              ││
│  │ created_at     │ TIMESTAMP    │ NO       │ now()   │              ││
│  │ updated_at     │ TIMESTAMP    │ YES      │ -       │              ││
│  │ metadata       │ JSONB        │ YES      │ {}      │              ││
│  └────────────────┴──────────────┴──────────┴─────────┴──────────────┘│
│                                                                        │
│  FOREIGN KEYS                                                          │
│  ┌────────────────┬──────────────────────┬──────────────────────────┐ │
│  │ Column         │ References           │ On Delete                │ │
│  ├────────────────┼──────────────────────┼──────────────────────────┤ │
│  │ organization_id│ organizations(id)    │ CASCADE                  │ │
│  └────────────────┴──────────────────────┴──────────────────────────┘ │
│                                                                        │
│  [Export DDL]                                                          │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 7. Console Query (SQL Editor)

### 7.1 Interface Query (Lecture Seule)

**Objectifs d'interface (inspiration Supabase) :**
- **Onglets de sortie :** Results (affichage tabulaire des resultats), Explain (plan d'execution pour performance), Chart (visualisation optionnelle : tableau + export ou graphiques). Voir [Reference SQL et DB](../reference/MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md) et [Affichage Dynamique et Metriques](./MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md).
- **Barre d'outils :** Run (execution), Role (contexte d'acces, ex. postgres), Source / Primary Database (environnement COG). Lien vers contrats : SELECT seul en mode normal ; ecriture uniquement en Recovery ([Emergency DB Access Contract](../contracts/database/MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md)).
- **Historique / requetes sauvegardees :** si dans le perimetre, sections Private, Favorites, Shared (ou equivalent) pour retrouver et rejouer des requetes.

```
┌────────────────────────────────────────────────────────────────────────┐
│  Query Console                                        [Lecture seule]  │
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │ SELECT id, email, created_at                                     │ │
│  │ FROM users                                                        │ │
│  │ WHERE status = 'active'                                          │ │
│  │ ORDER BY created_at DESC                                         │ │
│  │ LIMIT 100;                                                        │ │
│  └──────────────────────────────────────────────────────────────────┘ │
│                                                                        │
│  [Execute]  [Clear]  [History ▼]  [Saved Queries ▼]                   │
│                                                                        │
│  ⚠ Note: Only SELECT queries are allowed. For modifications,          │
│    use Maintenance or Recovery mode.                                   │
│                                                                        │
├────────────────────────────────────────────────────────────────────────┤
│  Results (executed in 25ms, 100 rows)                                  │
│  ┌──────┬──────────────────────┬────────────────────┐                 │
│  │ id   │ email                │ created_at         │                 │
│  ├──────┼──────────────────────┼────────────────────┤                 │
│  │ 1    │ john@example.com     │ 2026-01-15         │                 │
│  │ ...  │                      │                    │                 │
│  └──────┴──────────────────────┴────────────────────┘                 │
│                                                                        │
│  [Export CSV] [Export JSON] [Copy]                                     │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 8. Interface Maintenance

### 8.1 Operations Disponibles

```
┌────────────────────────────────────────────────────────────────────────┐
│  Database Maintenance                                                  │
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│  VALIDATION                                                            │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │ [Run Full Validation]  Last run: 2h ago  Status: ✓ Pass          │ │
│  │ Checks: Referential Integrity, Constraints, Schema Compliance    │ │
│  └──────────────────────────────────────────────────────────────────┘ │
│                                                                        │
│  OPTIMIZATION                                                          │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │ [Optimize Selected Tables]  [Reindex All]  [Update Statistics]   │ │
│  │ ⚠ Requires StrongFather approval                                 │ │
│  └──────────────────────────────────────────────────────────────────┘ │
│                                                                        │
│  REPAIR                                                                │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │ [Fix Orphans]  [Fix Duplicates]  [Fix Constraints]               │ │
│  │ ⚠ Requires StrongFather approval + Justification                 │ │
│  └──────────────────────────────────────────────────────────────────┘ │
│                                                                        │
│  MIGRATION                                                             │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │ Pending migrations: 0                                             │ │
│  │ [View Migration History]                                          │ │
│  └──────────────────────────────────────────────────────────────────┘ │
│                                                                        │
└────────────────────────────────────────────────────────────────────────┘
```

### 8.2 Dialogue Operation Maintenance

```
┌────────────────────────────────────────────────────────────────────────┐
│  Optimize Tables                                                       │
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│  Tables selectionnees:                                                 │
│  - orders (8.2 MB, fragmentation: 15%)                                │
│  - products (1.1 MB, fragmentation: 8%)                               │
│                                                                        │
│  Options:                                                              │
│  [x] Full vacuum                                                       │
│  [x] Update statistics                                                 │
│  [ ] Reindex                                                           │
│                                                                        │
│  Estimated duration: 2-5 minutes                                       │
│                                                                        │
│  ⚠ Cette operation necessite l'approbation de StrongFather            │
│                                                                        │
│  Justification:                                                        │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │ Optimisation de routine suite a forte activite                   │ │
│  └──────────────────────────────────────────────────────────────────┘ │
│                                                                        │
│  [Cancel]                                   [Request Approval]         │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 9. Interface Recovery Mode

### 9.1 Activation Recovery

```
┌────────────────────────────────────────────────────────────────────────┐
│ ⚠️ RECOVERY MODE                                                        │
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│  Le mode Recovery permet l'acces direct a la base de donnees.          │
│  Ce mode est reserve aux situations d'urgence.                         │
│                                                                        │
│  CONDITIONS REQUISES (toutes doivent etre remplies):                   │
│                                                                        │
│  [✓] Trust Level >= T3                    Actuel: T3 ✓                 │
│  [✓] Security Protocol = REINFORCED       Actuel: REINFORCED ✓         │
│  [✓] MFA Verified                         Actuel: Verifie ✓            │
│  [_] StrongFather Approval                En attente...                │
│                                                                        │
│  Duree maximale: [30 minutes ▼]                                        │
│                                                                        │
│  Justification (obligatoire):                                          │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │ Incident INC-2026-0128: Corruption table users necessitant       │ │
│  │ correction manuelle urgente.                                      │ │
│  └──────────────────────────────────────────────────────────────────┘ │
│                                                                        │
│  Reference incident: [INC-2026-0128]                                   │
│                                                                        │
│  [Cancel]                                    [Request Recovery Mode]   │
│                                                                        │
└────────────────────────────────────────────────────────────────────────┘
```

### 9.2 Console Recovery Active

```
┌────────────────────────────────────────────────────────────────────────┐
│ ⛔ RECOVERY MODE ACTIVE                          Time remaining: 25:30 │
│ All operators are BLOCKED during this session                          │
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│  SQL Console (FULL ACCESS)                                             │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │ UPDATE users SET status = 'active'                               │ │
│  │ WHERE id = 'uuid-corrupted-user';                                │ │
│  └──────────────────────────────────────────────────────────────────┘ │
│                                                                        │
│  [Execute]  [Clear]                                                    │
│                                                                        │
│  ⚠ WARNING: Rows affected estimate: 1                                  │
│  [Confirm Execution]                                                   │
│                                                                        │
├────────────────────────────────────────────────────────────────────────┤
│  SESSION LOG                                                           │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │ 12:05:30 - Session started by admin@miyukini                     │ │
│  │ 12:05:45 - SELECT COUNT(*) FROM users WHERE status='corrupted'   │ │
│  │            → Result: 1 row                                        │ │
│  └──────────────────────────────────────────────────────────────────┘ │
│                                                                        │
│  [End Recovery Mode]                                                   │
│                                                                        │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 10. Export

### 10.1 Options Export

```
┌────────────────────────────────────────────────────────────────────────┐
│  Export Data                                                           │
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│  Source: Table 'users' (filtered: 12,456 rows)                        │
│                                                                        │
│  Format:                                                               │
│  ( ) CSV                                                               │
│  (•) JSON                                                              │
│  ( ) SQL (INSERT statements)                                           │
│                                                                        │
│  Options:                                                              │
│  [x] Include headers                                                   │
│  [ ] Export schema only                                                │
│  [x] Apply current filters                                             │
│                                                                        │
│  Row limit: [All ▼]                                                    │
│                                                                        │
│  [Cancel]                                    [Export]                  │
│                                                                        │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 11. Implications Securite — Prevention Injection SQL

### 11.1 Risques d'Injection SQL dans l'Interface DB

L'interface de gestion DB est la surface d'attaque la plus sensible de MiyukiniAdmin. Cette section definit les controles obligatoires.

| Composant | Risque | Severite |
|-----------|--------|----------|
| **Console Query (Section 7)** | Injection via requete libre | CRITIQUE |
| **Filtres (Section 5.2)** | Injection via valeurs de filtre | HAUTE |
| **Export (Section 10)** | Injection via parametres export | MOYENNE |
| **Recovery Console (Section 9)** | Injection avec privileges eleves | CRITIQUE |

### 11.2 Console Query — Controles de Securite

> **INV-DB-SEC-1 : La Console Query en mode normal n'accepte que les SELECT.**

| Controle | Implementation |
|----------|----------------|
| **Parsing AST** | Analyse syntaxique de la requete avant execution |
| **Whitelist operations** | SELECT uniquement en mode standard |
| **Blacklist keywords** | `DROP`, `DELETE`, `UPDATE`, `INSERT`, `ALTER`, `TRUNCATE`, `GRANT`, `REVOKE` |
| **Sous-requetes** | Limitees a 2 niveaux de profondeur |
| **Fonctions** | Whitelist de fonctions autorisees |
| **Timeout** | 30 secondes maximum |
| **LIMIT** | Force si non present (max 10000 rows) |

### 11.3 Validation des Requetes

```
┌────────────────────────────────────────────────────────────────────────┐
│  Query Validation Pipeline                                             │
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│  1. LEXER → Tokenisation de la requete                                │
│     └─ Rejet si caracteres interdits                                   │
│                                                                        │
│  2. PARSER → Analyse syntaxique AST                                    │
│     └─ Rejet si syntaxe invalide ou operation interdite                │
│                                                                        │
│  3. SEMANTIC → Verification semantique                                 │
│     └─ Verification tables/colonnes existent                           │
│     └─ Verification permissions sur tables                             │
│                                                                        │
│  4. STRONGFATHER → Validation intention                                │
│     └─ Decision finale                                                 │
│                                                                        │
│  5. KINDMOTHER → Execution via pool securise                           │
│     └─ Requete preparee, parametres echappes                           │
│                                                                        │
└────────────────────────────────────────────────────────────────────────┘
```

### 11.4 Filtres — Securisation

Les filtres (section 5.2) sont des vecteurs d'injection :

| Element | Controle |
|---------|----------|
| **Nom colonne** | Validation contre schema (whitelist) |
| **Operateur** | Whitelist : `=`, `!=`, `>`, `<`, `>=`, `<=`, `LIKE`, `IN`, `IS NULL`, `IS NOT NULL` |
| **Valeur** | Parametre prepare, jamais concatene |
| **Type** | Validation type selon colonne (string, number, date, uuid) |

```
INTERDIT :
  WHERE {column} {operator} {value}  -- Concatenation

OBLIGATOIRE :
  WHERE "validated_column" = $1  -- Parametre prepare
```

### 11.5 Recovery Console — Controles Speciaux

La Recovery Console (section 9) permet l'ecriture. Controles renforces :

| Controle | Implementation |
|----------|----------------|
| **Double validation** | StrongFather + TAMR obligatoires |
| **Logging complet** | Chaque caractere tape est journalise |
| **Preview obligatoire** | Affichage `Rows affected estimate` avant execution |
| **Confirmation explicite** | Saisie du nombre de rows attendu |
| **Timeout court** | 10 secondes par requete |
| **Transaction explicite** | BEGIN/COMMIT/ROLLBACK manuels |
| **Audit immediat** | Journalisation synchrone avant execution |

### 11.6 Prevention Second Order Injection

Les donnees affichees peuvent contenir du SQL malveillant :

| Donnee | Traitement |
|--------|------------|
| **Valeurs de cellules** | Echappement HTML strict |
| **Noms de tables** | Validation alphanumerique + underscore |
| **Noms de colonnes** | Validation alphanumerique + underscore |
| **Metadata JSON** | Parsing JSON strict, pas d'eval |

### 11.7 Export — Securisation

L'export (section 10) peut etre exploite :

| Risque | Mitigation |
|--------|------------|
| **Formula Injection (CSV)** | Prefixe `'` pour cellules commencant par `=`, `+`, `-`, `@` |
| **JSON Injection** | Echappement strict des valeurs |
| **SQL Export** | Valeurs en tant que parametres, pas de concatenation |
| **Filename Injection** | Validation strict du nom de fichier |

### 11.8 Adaptation par Niveau de Securite (0-4)

| Fonctionnalite | Niveau 0-1 | Niveau 2 | Niveau 3 | Niveau 4 |
|----------------|------------|----------|----------|----------|
| **Console Query** | Full SELECT | SELECT avec restrictions | SELECT tables whitelist | Desactive |
| **Filtres** | Tous operateurs | Operateurs de base | `=` uniquement | Desactive |
| **Export** | Tous formats | CSV/JSON | CSV | Desactive |
| **Recovery** | Disponible | Double validation | Triple validation | Desactive |
| **Timeout query** | 30s | 15s | 10s | 5s |
| **Row limit** | 10000 | 5000 | 1000 | 100 |

### 11.9 Adaptation par Niveau de Confiance (T0-T4)

| Fonctionnalite | T0 | T1 | T2 | T3 | T4 |
|----------------|----|----|----|----|-----|
| **Console Query** | Active | Active + logging | Restreinte | Desactivee | Desactivee |
| **Filtres** | Actifs | Actifs | Simplifies | Desactives | Desactives |
| **Export** | Actif | Actif | CSV seul | Desactive | Desactive |
| **Recovery** | Disponible | Disponible | Restreint | TAMR requis | Desactive |
| **Maintenance** | Active | Active | Validation | Critique seul | Desactivee |

### 11.10 Indicateurs de Securite DB Interface

L'interface affiche toujours les indicateurs de securite :

```
┌────────────────────────────────────────────────────────────────────────┐
│  MiyukiniAdmin > Database           [Mode: READ-ONLY] [Trust: T0]     │
├────────────────────────────────────────────────────────────────────────┤
│  Query Console                                        [Lecture seule]  │
│                                                                        │
│  ⚠ Mode lecture seule actif. Pour modifications, utilisez Recovery.   │
│  🔒 Requetes validees par StrongFather                                │
└────────────────────────────────────────────────────────────────────────┘
```

### 11.11 References Securite

- [Security - Core Integration Map](../../../security/architecture/Security%20-%20Core%20Integration%20Map.md)
- [Security - Documentation Fondatrice](../../../security/foundation/Security%20-%20Documentation%20Fondatrice.md)
- [MiyukiniAdmin - Threat Model Contract](../contracts/security/MiyukiniAdmin%20-%20Threat%20Model%20Contract.md)
- [KindMother - Security Contract](../../KindMother/contracts/security/KindMother%20-%20Security%20Contract.md)

---

## 12. Documents Associes

- [MiyukiniAdmin - UI Design Philosophy](./MiyukiniAdmin%20-%20UI%20Design%20Philosophy.md)
- [MiyukiniAdmin - DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md)
- [MiyukiniAdmin - Emergency DB Access Contract](../contracts/database/MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md)
- [MiyukiniAdmin - KindMother Integration Contract](../contracts/integration/MiyukiniAdmin%20-%20KindMother%20Integration%20Contract.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Document de reference
