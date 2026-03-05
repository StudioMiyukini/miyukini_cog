# MiyukiniAdmin â€” Pages et Outils Reference Supabase

## 1. Contexte

Ce document etablit la **correspondance** entre les pages et outils du Dashboard Supabase (Studio) et les pages / outils MiyukiniAdmin. Il sert de reference unique pour savoir quelle page MiyukiniAdmin couvre quelle capacite Supabase et quels sujets SQL/DB documenter.

**Sources Supabase utilisees :**
- [Supabase SQL Editor](https://supabase.com/features/sql-editor) â€” fonctionnalites editeur SQL (syntax highlighting, auto-completion, execution history, onglets Results/Explain/Chart).
- [Tables and Data](https://supabase.com/docs/guides/database/tables) â€” Table Editor, creation de tables, types de donnees, Realtime. MiyukiniAdmin est un service hors-bord : il ne retient pas RLS ni Auth Supabase ; auth propre a definir.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. Portee / Scope

Ce document definit :
- Le tableau de correspondance Supabase -> MiyukiniAdmin pour chaque page/outil visible dans les captures Studio Supabase.
- Les sujets SQL et DB a documenter en coherence avec Supabase (schemas, tables, colonnes, types, requetes, Realtime, roles) ; sans RLS (service hors-bord).
- Les liens vers les documents MiyukiniAdmin existants (contrats, UI, reference).

Ce document **ne remplace pas** les contrats ni les specifications d'interface ; il les complete en les reliant au referentiel Supabase.

---

## 3. Tableau de correspondance Supabase -> MiyukiniAdmin

| Page / outil Supabase | Page / outil MiyukiniAdmin | Route(s) | Document(s) MiyukiniAdmin |
|-----------------------|----------------------------|----------|----------------------------|
| **Project Overview** | Dashboard / Project Overview | `/` ou `/dashboard` | [Dashboard & Metrics Display](../ui/MiyukiniAdmin%20-%20Dashboard%20&%20Metrics%20Display.md), [Organisation Pages et UX DB](../ui/MiyukiniAdmin%20-%20Organisation%20Pages%20et%20UX%20DB.md) |
| | Cartes fonctionnalites (Auth, Storage, Edge Functions, Realtime) | | Adaptees COG : KindMother, BondingBrother, etc. avec liens Explore / About |
| | Section PROJECT API (URL, cles) | | Zone affichant URL console et infos connexion/securite environnement ; pas d'API publique (INV-MA-3) |
| **Table Editor** | Database > Tables | `/database`, `/database/tables` | [DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md), [Organisation Pages et UX DB](../ui/MiyukiniAdmin%20-%20Organisation%20Pages%20et%20UX%20DB.md) |
| | Schema selector (ex. public) | | Liste schemas ; equivalent = environnement COG |
| | New table, Search tables | | Bouton creation table, recherche tables |
| | Liste tables / vues, Recent items | | Liste tables avec nom, lignes, taille, index ; elements recents |
| | Create a new table (formulaire) | `/database/tables/new` ou modal | Name, Description, Realtime, Columns (nom, type, default, primary), Foreign keys |
| | Vue donnees table (Filter, Sort, Insert, Import CSV) | `/database/tables/:tableId` | Onglets Data, Structure, Indexes, Stats, Export ; voir DB Management Interface Â§4â€“Â§5 |
| **SQL Editor** | Database > Query | `/database/query` | [DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md) Â§7, [Emergency DB Access Contract](../contracts/database/MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md) |
| | Zone saisie, Run, historique | | Console SQL ; execution SELECT uniquement en mode normal |
| | Onglets Results, Explain, Chart | | Resultats tabulaires ; plan d'execution ; visualisation (optionnel) |
| | Role, Source, Primary Database | | Equivalent = role/connexion via BondingBrother, environnement COG |
| | Search queries, Private / Favorites / Shared | | Historique et requetes sauvegardees (si dans le perimetre) |
| **Database** | Database (vue generale) | `/database` | [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md), [Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md) |
| | Schemas, roles, connexion | | Vue d'ensemble ; acces via KindMother/BondingBrother |
| **Migrations** | Database > Migrations | `/database/migrations` | [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md) Â§9, [Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md) |
| **Backups** | Database > Backups | `/database/backups` | [Backup Restore Contract](../contracts/database/MiyukiniAdmin%20-%20Backup%20Restore%20Contract.md) |
| **Observability** | Metriques / Observability | `/metriques` | [DB Metrics Contract](../contracts/monitoring/MiyukiniAdmin%20-%20DB%20Metrics%20Contract.md), [Consumption Metrics Contract](../contracts/monitoring/MiyukiniAdmin%20-%20Consumption%20Metrics%20Contract.md), [Affichage Dynamique et Metriques](../ui/MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md) |
| **Logs** | Logs | `/logs` | Affichage dynamique (SSE ou polling) ; logs audit et operationnels |
| **API Docs** | Documentation API | `/api-docs` | Liste endpoints, parametres ; statique ou enrichi par `GET /api/status` |
| **Project Settings** | Parametres | (a definir) | [Serveur HTTP HTTPS](../operations/MiyukiniAdmin%20-%20Serveur%20HTTP%20HTTPS.md), operations |

---

## 4. Sujets SQL et DB a documenter (inspiration Supabase)

Les sujets suivants sont couverts par Supabase et doivent etre documentes ou enrichis dans MiyukiniAdmin :

| Sujet | Contenu a documenter | Document cible MiyukiniAdmin |
|-------|----------------------|------------------------------|
| Schemas et tables | Notion de schema (ex. `public`), liste tables/vues, creation table (nom, description) | [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md) |
| Colonnes et types | Types (int2, int4, int8, float4, float8, numeric, json, jsonb, text, varchar, uuid, timestamptz, bool), default, primary key, nullability | [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md) |
| Requetes | SELECT uniquement en mode normal ; timeout, LIMIT force ; validation StrongFather | [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md), [Emergency DB Access Contract](../contracts/database/MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md) |
| Resultats et analyse | Affichage tabulaire, export CSV/JSON ; Explain (plan d'execution) ; Chart (optionnel) | [DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md), [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md) |
| Realtime | Transposition = ecoute des changements (WebSockets ou polling) ; BondingBrother / KindMother | [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md), [Affichage Dynamique et Metriques](../ui/MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md) |
| Roles et connexion | Role (ex. postgres), Primary Database, Source ; equivalent = environnement COG, acces BondingBrother | [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md), [KindMother Integration Contract](../contracts/integration/MiyukiniAdmin%20-%20KindMother%20Integration%20Contract.md) |
| SQL Editor (fonctionnalites) | Syntax highlighting, auto-completion, execution history, onglets Results/Explain/Chart | [Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md), [DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md) |

---

## 5. Documents associes

- [MiyukiniAdmin - Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md)
- [MiyukiniAdmin - Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md)
- [MiyukiniAdmin - Organisation Pages et UX DB](../ui/MiyukiniAdmin%20-%20Organisation%20Pages%20et%20UX%20DB.md)
- [MiyukiniAdmin - DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md)
- [MiyukiniAdmin - Dashboard & Metrics Display](../ui/MiyukiniAdmin%20-%20Dashboard%20&%20Metrics%20Display.md)
- [MiyukiniAdmin - Affichage Dynamique et Metriques](../ui/MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md)

---

**Date de creation :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Document de reference (mapping Supabase / MiyukiniAdmin)

