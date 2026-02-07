# MiyukiniAdmin — Gestion DB type Supabase

## 1. Contexte

Ce document definit comment MiyukiniAdmin couvre des **capacites de gestion de base de donnees** comparables a celles de Supabase (migrations, scripts SQL, backups, editeur SQL, etc.), en les transposant dans le langage et l'architecture Miyukini COG. Il sert de reference pour l'implementation et complete les contrats existants (DB Operations, Emergency DB Access, KindMother).

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

## 2. Portee / Scope

Ce document definit :
- L'inventaire des capacites Supabase pertinentes pour MiyukiniAdmin
- La transposition en concepts COG (KindMother, StrongFather, BondingBrother, Ever Buddy, etc.)
- Le mapping vers les contrats MiyukiniAdmin existants
- Les principes d'implementation pour migrations, backups, SQL editor, storage (si dans le perimetre)

Ce document **ne remplace pas** les contrats ; il les complete et les relie a un referentiel externe (Supabase).

---

## 3. Inventaire des capacites Supabase

### 3.1 Tableau des domaines Supabase

| Domaine Supabase | Capacites techniques | Description courte |
|------------------|---------------------|--------------------|
| **Database** | PostgreSQL, REST/GraphQL auto, webhooks, Vault, replication | Moteur DB, APIs auto-generees, secrets, replication |
| **Migrations** | Fichiers SQL versionnes, CLI `migration up`, schema diff, seed, reset | Evolution du schema dans le temps, ordre d'application, rollback |
| **Backups** | Logical / physical, PITR, dump/restore CLI, backups quotidiens | Sauvegarde et restauration, point-in-time recovery |
| **SQL Editor** | Editeur SQL integre au Dashboard | Execution de requetes (lecture/ecriture selon contexte) |
| **Storage** | Buckets S3-compatible, CDN, transformations d'images | Fichiers, objets, stockage externe a la DB |
| **Dashboard / Studio** | Gestion projet, tables, logs, settings | Interface d'administration centralisee |

---

## 4. Transposition Miyukini COG

### 4.1 Tableau de transposition

| Domaine Supabase | Transposition Miyukini | Cores impliques | Contrat(s) MiyukiniAdmin |
|------------------|------------------------|-----------------|---------------------------|
| **Database** | **KindMother** = autorite persistance. Exposition des donnees uniquement via **BondingBrother**. MiyukiniAdmin est un **Operateur Souverain** ; aucune API auto publique (PostgREST/GraphQL) exposee par l'admin. | KindMother, BondingBrother | [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md) |
| **Migrations** | **Scripts de migration** = intentions d'evolution de schema. Execution via **KindMother** sous validation **StrongFather**. **Ever Buddy** pour compatibilité et versions (etats de vie, evolution). | KindMother, StrongFather, Ever Buddy | [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md) (MIG-001 a MIG-003) |
| **Backups** | **Sauvegarde / Restauration** = operations MAINT ou dediees. Validation **StrongFather** obligatoire. Traçabilite complete. Niveau de confiance (**Etats de confiance T0-T4**) et **Niveaux de securite (0-4)** pris en compte. | KindMother, StrongFather, WorrySentinel | DB Operations Contract ; optionnel [Backup Restore Contract](../contracts/database/MiyukiniAdmin%20-%20Backup%20Restore%20Contract.md) |
| **SQL Editor** | **Console Query** = lecture seule (SELECT) en mode normal. **Recovery** = ecriture exceptionnelle sous conditions cumulatives (voir Emergency DB Access). Validation **StrongFather** pour toute ecriture. | KindMother, StrongFather | [DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md), [Emergency DB Access Contract](../contracts/database/MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md) |
| **Storage** | Si dans le perimetre admin : gestion des **buckets** ou assets sous autorite **KindMother**. A borner : administration de la persistance uniquement, pas de logique metier (CDN, transformations = hors scope admin ou delegue a un Operateur). | KindMother | A definir selon perimetre |
| **Dashboard / Studio** | **MiyukiniAdmin** = Opérateur Souverain (Strate 9). Pas de notion "projet" Supabase ; equivalent = **Environnement** (COG). Gestion des tables, logs, parametres = vue sur l'environnement courant. | Tous (lecture via BondingBrother) | [Documentation Fondatrice](../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md), [Dashboard & Metrics Display](../ui/MiyukiniAdmin%20-%20Dashboard%20&%20Metrics%20Display.md) |

### 4.2 Termes du Glossaire a utiliser

- **Capacite** : pouvoir technique (ex. "executer une migration")
- **Operateur** : MiyukiniAdmin est un Operateur Souverain
- **Core** : KindMother, StrongFather, BondingBrother, Ever Buddy, WorrySentinel, Caring Nanny
- **KindMother** : autorite donnees et persistance
- **StrongFather** : autorite decision (validation des operations d'ecriture / migration / backup)
- **BondingBrother** : mediation ; MiyukiniAdmin n'accede jamais directement a KindMother
- **Mandat** : pas d'emission de Mandat de Permission par MiyukiniAdmin ; il agit sous protocole souverain
- **Environnement** : COG ; equivalent conceptuel au "projet" Supabase
- **Migration** : processus d'evolution du schema (ou entre environnements, voir Glossaire)
- **Etats de confiance (T0-T4)** : gouvernes par WorrySentinel ; impactent disponibilite Recovery, backups
- **Niveaux de securite (0-4)** : gouvernes par WorrySentinel ; impactent permissions et traçabilite

---

## 5. Mapping vers les contrats existants

### 5.1 DB Operations Contract

- **Lecture** : READ-001 a READ-005 (exploration tables, schema, donnees, export, stats) — pas de validation StrongFather.
- **Maintenance** : MAINT-001 a MAINT-005 (analyse, vacuum, reindex, stats, nettoyage logs) — validation StrongFather.
- **Reparation** : REPAIR-001 a REPAIR-003 — validation StrongFather + conditions.
- **Migration** : MIG-001 (schema), MIG-002 (donnees), MIG-003 (rollback) — validation StrongFather, backup obligatoire, pre/post tests.

Voir [MiyukiniAdmin - DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md) pour les flux et formats.

### 5.2 Emergency DB Access Contract

- **SQL Editor en ecriture** : uniquement en mode Recovery. Conditions cumulatives (T3/T4, protocole renforce, MFA, justification, approbation StrongFather, fenetre limitee). Toute ecriture directe en dehors de KindMother passe par ce contrat.

Voir [MiyukiniAdmin - Emergency DB Access Contract](../contracts/database/MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md).

### 5.3 KindMother Integration Contract

- Toutes les operations DB (lecture, maintenance, migration, backup) sont executees par KindMother ou deleguees via BondingBrother. MiyukiniAdmin ne possede pas de connexion DB directe en mode normal.

Voir [MiyukiniAdmin - KindMother Integration Contract](../contracts/integration/MiyukiniAdmin%20-%20KindMother%20Integration%20Contract.md).

---

## 6. Principes d'implementation

### 6.1 Migrations (scripts de migration)

- **Format** : un fichier par migration, nommage stable (ex. `YYYYMMDDHHMMSS_description_courte.sql` ou version semver). Table d'historique en DB (appliquees, ordre, checksum).
- **Ordre** : application stricte par ordre chronologique ou version ; pas de saut.
- **Idempotence** : recommandee (IF NOT EXISTS, etc.) pour limiter les echecs en cas de rejeu.
- **Ever Buddy** : coherence des versions et compatibilité (etats de vie ACTIF / DEPRECIE) ; pas d'execution par Ever Buddy, mais consultation pour validation pre-migration.
- **Workflow** : Pre-validation → Backup → Validation StrongFather → Execution via KindMother → Post-validation / Rollback (deja decrit DB Operations Contract §9). Extension detaillee dans le meme contrat (section Scripts de migration).

### 6.2 Backups et restauration

- **Declenchement** : manuel (depuis MiyukiniAdmin) ou planifie (cron / job) ; dans les deux cas, traçabilite et validation StrongFather si ecriture ou impact systeme.
- **Stockage** : hors DB (fichier, objet) ; pas de stockage des backups dans la meme instance que la DB cible.
- **Restauration** : conditions StrongFather + WorrySentinel (niveau confiance, niveau securite) ; procedure documentee ; rollback possible si echec.
- **PITR** : si implémente, traite comme une capacite avancee ; memes principes gouvernance.

### 6.3 SQL Editor (Console Query / Recovery)

- **Mode normal** : SELECT uniquement ; parsing AST, whitelist, timeout, LIMIT force (voir [DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md) §11). Validation StrongFather pour l'intention de lecture si politique l'exige.
- **Mode Recovery** : ecriture autorisee temporairement ; voir Emergency DB Access Contract. Aucune reutilisation du code "SQL Editor" public ; interface dediee Recovery.
- **Fonctionnalites inspirees Supabase (documentation externe) :** historique des requetes executees, auto-completion (noms de tables, colonnes, fonctions), syntax highlighting pour la lisibilite, onglets de sortie **Results** (affichage tabulaire), **Explain** (plan d'execution), **Chart** (visualisation optionnelle). References : [Supabase SQL Editor](https://supabase.com/features/sql-editor), [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md).

### 6.4 Table Editor (creation table, colonnes)

- **Creation de table :** formulaire avec nom, description (optionnel), option Realtime (voir [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md)). Aucune option RLS (MiyukiniAdmin = service hors-bord, auth propre).
- **Types de donnees :** sous-ensemble Postgres pris en charge (int2, int4, int8, float4, float8, numeric, json, jsonb, text, varchar, uuid, timestamptz, bool, etc.) pour la definition des colonnes. Reference : [Supabase Tables and Data](https://supabase.com/docs/guides/database/tables).
- **Realtime :** option "Enable Realtime" sur la table ; transposition COG documentee dans [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md).

### 6.5 Storage / Buckets

- **Perimetre** : si MiyukiniAdmin gere des buckets (fichiers, assets), cela reste sous KindMother (persistance). Pas d'API CDN ou transformations cote admin ; uniquement liste, creation/suppression de buckets, politiques d'acces minimales si besoin.
- **Hors scope actuel** : transformations d'images, CDN, logique metier Storage = delegues a un Operateur metier ou hors MiyukiniAdmin.

---

## 7. Documentation Supabase utilisee

Pour traçabilite et enrichissement des sujets SQL/DB, les documents Supabase suivants sont utilises comme reference (sans dependance technique) :

- [Supabase SQL Editor](https://supabase.com/features/sql-editor) — fonctionnalites editeur SQL (syntax highlighting, auto-completion, execution history, onglets Results/Explain/Chart).
- [Tables and Data](https://supabase.com/docs/guides/database/tables) — Table Editor, creation de tables, types de donnees, Realtime.

Voir egalement [MiyukiniAdmin - Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md) et [Pages et Outils Reference Supabase](./MiyukiniAdmin%20-%20Pages%20et%20Outils%20Reference%20Supabase.md).

---

## 8. Documents associes

- [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [MiyukiniAdmin - Pages et Outils Reference Supabase](./MiyukiniAdmin%20-%20Pages%20et%20Outils%20Reference%20Supabase.md)
- [MiyukiniAdmin - Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md)
- [MiyukiniAdmin - DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md)
- [MiyukiniAdmin - Emergency DB Access Contract](../contracts/database/MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md)
- [MiyukiniAdmin - KindMother Integration Contract](../contracts/integration/MiyukiniAdmin%20-%20KindMother%20Integration%20Contract.md)
- [MiyukiniAdmin - DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md)
- [MiyukiniAdmin - Capacites et Reference](./MiyukiniAdmin%20-%20Capacites%20et%20Reference.md)

---

**Date de creation :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Document de reference (implementation)
