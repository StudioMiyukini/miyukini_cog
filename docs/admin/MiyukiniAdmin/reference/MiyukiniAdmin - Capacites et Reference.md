# MiyukiniAdmin â€” CapacitÃ©s et RÃ©fÃ©rence

## 1. Contexte

Ce document explicite les **capacitÃ©s** de MiyukiniAdmin et leur alignement avec les documents de rÃ©fÃ©rence de lâ€™Ã©cosystÃ¨me Miyukini. Il sert de pont entre la documentation core MiyukiniAdmin et le dossier `docs/reference`.

**Objectif :** permettre de comprendre quelles capacitÃ©s de lâ€™interface MiyukiniAdmin sâ€™appuient sur quelles rÃ©fÃ©rences conceptuelles et contractuelles.

---

## 2. PortÃ©e / Scope

Ce document :
- Inventorie les capacitÃ©s livrÃ©es par lâ€™interface MiyukiniAdmin.
- RÃ©fÃ©rence pour chaque capacitÃ© les documents de `docs/reference` et de `docs/core/MiyukiniAdmin` concernÃ©s.
- Indique le statut (livrÃ© / prÃ©vu / hors scope).

Ce document **ne remplace pas** les contrats ni les guides dâ€™implÃ©mentation.

---

## 3. RÃ©fÃ©rences conceptuelles (docs/reference)

Les documents suivants dÃ©finissent le cadre dans lequel MiyukiniAdmin opÃ¨re. Les capacitÃ©s de lâ€™admin sâ€™y alignent.

| Document | Lien | Usage dans MiyukiniAdmin |
|----------|------|---------------------------|
| **MiyukiniAdmin Status** | [Miyukini Conceptual References - MiyukiniAdmin Status](..//..//..//miyukini-webway-system//reference//_index.md) | Statut officiel, pÃ©rimÃ¨tre fonctionnel, rÃ¨gles absolues |
| **DÃ©finition COG** | [Miyukini Conceptual References - Definition COG](..//..//..//miyukini-webway-system//reference//_index.md) | ComprÃ©hension des Cores et de la mÃ©diation |
| **Pyramide architecture** | [Miyukini Conceptual References - Pyramide Architecture Complete](..//..//..//miyukini-webway-system//reference//_index.md) | Position Strate 9, au-dessus de la pyramide |
| **Security Levels** | [Miyukini Conceptual References - Security Levels](..//..//..//miyukini-webway-system//reference//_index.md) | Niveaux de sÃ©curitÃ© 0â€“4, panneau sÃ©curitÃ© |
| **Connexion Inter-COG** | [Miyukini Conceptual References - Connexion Inter-COG](..//..//..//miyukini-webway-system//reference//_index.md) | AccÃ¨s aux cores via BondingBrother |
| **Lois Autonomie SystÃ¨me** | [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//miyukini-webway-system//reference//_index.md) | ConformitÃ© LOI-1, LOI-2, LOI-3, LOI-5, LOI-6 |
| **Catalogue capacitÃ©s produit** | [Miyukini Conceptual References - Catalogue Capacites Produit](..//..//..//miyukini-webway-system//reference//_index.md) | Contexte capacitÃ©s mÃ©tier (MiyukiniAdmin nâ€™implÃ©mente pas ces capacitÃ©s mÃ©tier ; il administre le systÃ¨me qui les porte) |
| **Glossaire** | [Miyukini Conceptual References - Glossaire](..//..//..//miyukini-webway-system//reference//_index.md) | Vocabulaire canonique OpÃ©rateur, Core, Strate |
| **AccÃ¨s DB et droits agents** | [Miyukini Conceptual References - Acces DB et Droits Agents IA](..//..//..//miyukini-webway-system//reference//_index.md) | Politique dâ€™accÃ¨s DB et droits administrateur |
| **Objectif final** | [Miyukini Conceptual References - Objectif Final](..//..//..//miyukini-webway-system//reference//_index.md) | Vision et rÃ´le de MiyukiniAdmin comme clÃ© de voÃ»te |

---

## 4. CapacitÃ©s de lâ€™interface MiyukiniAdmin

### 4.1 Tableau de synthÃ¨se

| CapacitÃ© | Description | RÃ©fÃ©rence contractuelle | Statut |
|----------|-------------|-------------------------|--------|
| **Dashboard** | Page dâ€™accueil, Ã©tat systÃ¨me, liens vers sections | [Dashboard & Metrics Display](../ui/MiyukiniAdmin%20-%20Dashboard%20&%20Metrics%20Display.md) | LivrÃ© |
| **SantÃ© / API** | Endpoints `/health`, `/api/status` | [Architecture & Flows](../architecture/MiyukiniAdmin%20-%20Architecture%20&%20Flows.md) | LivrÃ© |
| **Serveur HTTP/HTTPS** | Affichage sÃ©curisÃ©, option HTTPS | [MiyukiniAdmin - Serveur HTTP HTTPS](../operations/MiyukiniAdmin%20-%20Serveur%20HTTP%20HTTPS.md) | LivrÃ© / documentÃ© |
| **CRUD tables** | Liste tables, exploration, crÃ©ation/lecture/mise Ã  jour/suppression de donnÃ©es | [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md), [DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md) | LivrÃ© |
| **Manipulation donnÃ©es** | Filtres, tri, pagination, export (lecture seule en mode normal ; Ã©criture via KindMother/StrongFather ou mode recovery) | [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md) | LivrÃ© |
| **Page Tests / flux** | Tests de flux pour vÃ©rifier le comportement des cores | [Cycle Tests Contract](../contracts/testing/MiyukiniAdmin%20-%20Cycle%20Tests%20Contract.md), [Unit Tests Contract](../contracts/testing/MiyukiniAdmin%20-%20Unit%20Tests%20Contract.md) | LivrÃ© |
| **Auth & Permissions** | Login, MFA, session, rÃ´les (Admin, Recovery, Audit), capacitÃ©s, RBAC | [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md), [Authentication Contract](../contracts/security/MiyukiniAdmin%20-%20Authentication%20Contract.md), [Permission Contract](../contracts/security/MiyukiniAdmin%20-%20Permission%20Contract.md) | DocumentÃ© |
| **SÃ©curitÃ©** | Niveaux 0â€“4, panneau sÃ©curitÃ© | [Security Level Management Contract](../contracts/security/MiyukiniAdmin%20-%20Security%20Level%20Management%20Contract.md) | PrÃ©vu |
| **Monitoring** | MÃ©triques consommation, DB | [Consumption Metrics Contract](../contracts/monitoring/MiyukiniAdmin%20-%20Consumption%20Metrics%20Contract.md), [DB Metrics Contract](../contracts/monitoring/MiyukiniAdmin%20-%20DB%20Metrics%20Contract.md) | PrÃ©vu |
| **Recovery** | AccÃ¨s DB direct (conditions cumulatives) | [Emergency DB Access Contract](../contracts/database/MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md) | PrÃ©vu |
| **Gestion des migrations** | Liste, dÃ©tail, exÃ©cution et historique des scripts de migration (schema/donnÃ©es, rollback) | [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md), [Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md) | PrÃ©vu |
| **Backups / Restauration** | Sauvegarde et restauration de la base (dÃ©clenchement, stockage, traÃ§abilitÃ©) | [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md), [Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md) | PrÃ©vu |
| **SQL Editor (Query)** | Console SQL : saisie, exÃ©cution (SELECT en mode normal), onglets Results / Explain / Chart, historique | [DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md), [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md), [Emergency DB Access Contract](../contracts/database/MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md) | PrÃ©vu |
| **Table Editor (crÃ©ation table, schema)** | CrÃ©ation de table (nom, description, Realtime, colonnes, types, clÃ©s Ã©trangÃ¨res) ; extension du CRUD existant ; pas de RLS (service hors-bord) | [DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md), [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md) | PrÃ©vu |
| **Observability / MÃ©triques dynamiques** | Page MÃ©triques avec rafraÃ®chissement des donnÃ©es (requÃªtes/s, latence, pool DB, systÃ¨me) ; polling ou SSE | [DB Metrics Contract](../contracts/monitoring/MiyukiniAdmin%20-%20DB%20Metrics%20Contract.md), [Affichage Dynamique et Metriques](../ui/MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md) | PrÃ©vu |
| **Logs** | Consultation des logs dâ€™audit et opÃ©rationnels ; flux continu (SSE) ou pagination | [Affichage Dynamique et Metriques](../ui/MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md) | PrÃ©vu |
| **API Docs** | Page Documentation API : liste des endpoints, paramÃ¨tres ; statique ou enrichie par `/api/status` | [Pages et Outils Reference Supabase](./MiyukiniAdmin%20-%20Pages%20et%20Outils%20Reference%20Supabase.md) | PrÃ©vu |
| **Storage / Buckets** | Gestion des buckets ou assets sous autoritÃ© KindMother (si dans le pÃ©rimÃ¨tre) | [Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md) | Ã€ dÃ©finir |

### 4.2 DÃ©tail des capacitÃ©s livrÃ©es

#### Dashboard
- Affichage de lâ€™Ã©tat du systÃ¨me (en ligne / version).
- Liste des cores systÃ¨me (Kernel, StrongFather, KindMother, etc.).
- Liens vers Database, Tests, SÃ©curitÃ© (selon implÃ©mentation).

**RÃ©fÃ©rence :** [Miyukini Conceptual References - MiyukiniAdmin Status](..//..//..//miyukini-webway-system//reference//_index.md) â€” pÃ©rimÃ¨tre Â« Monitoring & MÃ©triques Â».

#### Serveur HTTP/HTTPS
- Serveur HTTP sur `MIYUKINIADMIN_HOST:MIYUKINIADMIN_PORT`.
- Option HTTPS pour affichage sÃ©curisÃ© (certificat et clÃ© configurables).
- DÃ©tails : [MiyukiniAdmin - Serveur HTTP HTTPS](../operations/MiyukiniAdmin%20-%20Serveur%20HTTP%20HTTPS.md).

#### CRUD tables et manipulation des donnÃ©es
- **Liste des tables** : exploration des tables disponibles (via KindMother en production).
- **Lecture** : consultation des lignes (pagination, filtres, tri).
- **CrÃ©ation / mise Ã  jour / suppression** : selon contrat DB (validation StrongFather, mÃ©diation KindMother) ou mode recovery.
- **Export** : CSV/JSON (lecture seule).

**RÃ©fÃ©rences :**
- [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md) (READ-001 Ã  MIG-003).
- [DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md).

#### Page Tests / flux
- Page dÃ©diÃ©e (`/tests`) avec plusieurs scÃ©narios de test de flux.
- VÃ©rification du comportement des cores (disponibilitÃ©, statut, rÃ©ponses attendues).
- RÃ©fÃ©rence : [Cycle Tests Contract](../contracts/testing/MiyukiniAdmin%20-%20Cycle%20Tests%20Contract.md), [Unit Tests Contract](../contracts/testing/MiyukiniAdmin%20-%20Unit%20Tests%20Contract.md).

#### CapacitÃ©s prÃ©vues (affichage dynamique, SQL/DB)
- **SQL Editor (Query)** : console SQL avec exÃ©cution SELECT, onglets Results / Explain / Chart ; mode Recovery pour Ã©criture (voir [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md), [Pages et Outils Reference Supabase](./MiyukiniAdmin%20-%20Pages%20et%20Outils%20Reference%20Supabase.md)).
- **Table Editor** : crÃ©ation de table (nom, description, Realtime, colonnes et types) ; voir [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md). Pas de RLS (MiyukiniAdmin = service hors-bord).
- **Observability / MÃ©triques dynamiques** : page dÃ©diÃ©e avec mÃ©triques DB et systÃ¨me rafraÃ®chies (polling ou SSE, Rust-first). Voir [Affichage Dynamique et Metriques](../ui/MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md).
- **Logs** : flux des logs dâ€™audit et opÃ©rationnels (SSE ou pagination).
- **API Docs** : page listant les endpoints et paramÃ¨tres (inspiration Supabase).

---

## 5. Alignement avec les documents de rÃ©fÃ©rence

| ThÃ¨me rÃ©fÃ©rence | Document(s) | Impact sur MiyukiniAdmin |
|----------------|-------------|--------------------------|
| Statut OpÃ©rateur Souverain | MiyukiniAdmin Status | Aucune API publique ; accÃ¨s exclusif via BondingBrother ; console root |
| SÃ©curitÃ© | Security Levels, Security Protocols | Niveaux 0â€“4, protocole renforcÃ© pour recovery |
| DonnÃ©es | AccÃ¨s DB et droits agents | Toutes les opÃ©rations DB via KindMother ; Ã©criture directe uniquement en recovery |
| Cores | Definition COG, Connexion Inter-COG | Liste des cores affichÃ©e ; tests de flux pour valider les interactions |
| Autonomie | Lois Autonomie SystÃ¨me | Offline possible pour monitoring ; pas de dÃ©pendance cachÃ©e |

---

## 6. Documents associÃ©s

- [MiyukiniAdmin - Index de Navigation](../_index.md)
- [MiyukiniAdmin - Pages et Outils Reference Supabase](./MiyukiniAdmin%20-%20Pages%20et%20Outils%20Reference%20Supabase.md)
- [MiyukiniAdmin - Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md)
- [MiyukiniAdmin - Affichage Dynamique et Metriques](../ui/MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md)
- [MiyukiniAdmin - Documentation Fondatrice](../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- [MiyukiniAdmin - Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md)
- [MiyukiniAdmin - Serveur HTTP HTTPS](../operations/MiyukiniAdmin%20-%20Serveur%20HTTP%20HTTPS.md)
- [Miyukini Conceptual References - MiyukiniAdmin Status](..//..//..//miyukini-webway-system//reference//_index.md)

---

**Date de crÃ©ation :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Document de rÃ©fÃ©rence

