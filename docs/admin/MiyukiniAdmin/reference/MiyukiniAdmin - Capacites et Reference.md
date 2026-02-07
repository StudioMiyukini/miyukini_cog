# MiyukiniAdmin — Capacités et Référence

## 1. Contexte

Ce document explicite les **capacités** de MiyukiniAdmin et leur alignement avec les documents de référence de l’écosystème Miyukini. Il sert de pont entre la documentation core MiyukiniAdmin et le dossier `docs/reference`.

**Objectif :** permettre de comprendre quelles capacités de l’interface MiyukiniAdmin s’appuient sur quelles références conceptuelles et contractuelles.

---

## 2. Portée / Scope

Ce document :
- Inventorie les capacités livrées par l’interface MiyukiniAdmin.
- Référence pour chaque capacité les documents de `docs/reference` et de `docs/core/MiyukiniAdmin` concernés.
- Indique le statut (livré / prévu / hors scope).

Ce document **ne remplace pas** les contrats ni les guides d’implémentation.

---

## 3. Références conceptuelles (docs/reference)

Les documents suivants définissent le cadre dans lequel MiyukiniAdmin opère. Les capacités de l’admin s’y alignent.

| Document | Lien | Usage dans MiyukiniAdmin |
|----------|------|---------------------------|
| **MiyukiniAdmin Status** | [Miyukini Conceptual References - MiyukiniAdmin Status](../../../reference/Miyukini%20Conceptual%20References%20-%20MiyukiniAdmin%20Status.md) | Statut officiel, périmètre fonctionnel, règles absolues |
| **Définition COG** | [Miyukini Conceptual References - Definition COG](../../../reference/Miyukini%20Conceptual%20References%20-%20Definition%20COG.md) | Compréhension des Cores et de la médiation |
| **Pyramide architecture** | [Miyukini Conceptual References - Pyramide Architecture Complete](../../../reference/Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) | Position Strate 9, au-dessus de la pyramide |
| **Security Levels** | [Miyukini Conceptual References - Security Levels](../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) | Niveaux de sécurité 0–4, panneau sécurité |
| **Connexion Inter-COG** | [Miyukini Conceptual References - Connexion Inter-COG](../../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md) | Accès aux cores via BondingBrother |
| **Lois Autonomie Système** | [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) | Conformité LOI-1, LOI-2, LOI-3, LOI-5, LOI-6 |
| **Catalogue capacités produit** | [Miyukini Conceptual References - Catalogue Capacites Produit](../../../reference/Miyukini%20Conceptual%20References%20-%20Catalogue%20Capacites%20Produit.md) | Contexte capacités métier (MiyukiniAdmin n’implémente pas ces capacités métier ; il administre le système qui les porte) |
| **Glossaire** | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Vocabulaire canonique Opérateur, Core, Strate |
| **Accès DB et droits agents** | [Miyukini Conceptual References - Acces DB et Droits Agents IA](../../../reference/Miyukini%20Conceptual%20References%20-%20Acces%20DB%20et%20Droits%20Agents%20IA.md) | Politique d’accès DB et droits administrateur |
| **Objectif final** | [Miyukini Conceptual References - Objectif Final](../../../reference/Miyukini%20Conceptual%20References%20-%20Objectif%20Final.md) | Vision et rôle de MiyukiniAdmin comme clé de voûte |

---

## 4. Capacités de l’interface MiyukiniAdmin

### 4.1 Tableau de synthèse

| Capacité | Description | Référence contractuelle | Statut |
|----------|-------------|-------------------------|--------|
| **Dashboard** | Page d’accueil, état système, liens vers sections | [Dashboard & Metrics Display](../ui/MiyukiniAdmin%20-%20Dashboard%20&%20Metrics%20Display.md) | Livré |
| **Santé / API** | Endpoints `/health`, `/api/status` | [Architecture & Flows](../architecture/MiyukiniAdmin%20-%20Architecture%20&%20Flows.md) | Livré |
| **Serveur HTTP/HTTPS** | Affichage sécurisé, option HTTPS | [MiyukiniAdmin - Serveur HTTP HTTPS](../operations/MiyukiniAdmin%20-%20Serveur%20HTTP%20HTTPS.md) | Livré / documenté |
| **CRUD tables** | Liste tables, exploration, création/lecture/mise à jour/suppression de données | [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md), [DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md) | Livré |
| **Manipulation données** | Filtres, tri, pagination, export (lecture seule en mode normal ; écriture via KindMother/StrongFather ou mode recovery) | [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md) | Livré |
| **Page Tests / flux** | Tests de flux pour vérifier le comportement des cores | [Cycle Tests Contract](../contracts/testing/MiyukiniAdmin%20-%20Cycle%20Tests%20Contract.md), [Unit Tests Contract](../contracts/testing/MiyukiniAdmin%20-%20Unit%20Tests%20Contract.md) | Livré |
| **Auth & Permissions** | Login, MFA, session, rôles (Admin, Recovery, Audit), capacités, RBAC | [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md), [Authentication Contract](../contracts/security/MiyukiniAdmin%20-%20Authentication%20Contract.md), [Permission Contract](../contracts/security/MiyukiniAdmin%20-%20Permission%20Contract.md) | Documenté |
| **Sécurité** | Niveaux 0–4, panneau sécurité | [Security Level Management Contract](../contracts/security/MiyukiniAdmin%20-%20Security%20Level%20Management%20Contract.md) | Prévu |
| **Monitoring** | Métriques consommation, DB | [Consumption Metrics Contract](../contracts/monitoring/MiyukiniAdmin%20-%20Consumption%20Metrics%20Contract.md), [DB Metrics Contract](../contracts/monitoring/MiyukiniAdmin%20-%20DB%20Metrics%20Contract.md) | Prévu |
| **Recovery** | Accès DB direct (conditions cumulatives) | [Emergency DB Access Contract](../contracts/database/MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md) | Prévu |
| **Gestion des migrations** | Liste, détail, exécution et historique des scripts de migration (schema/données, rollback) | [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md), [Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md) | Prévu |
| **Backups / Restauration** | Sauvegarde et restauration de la base (déclenchement, stockage, traçabilité) | [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md), [Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md) | Prévu |
| **SQL Editor (Query)** | Console SQL : saisie, exécution (SELECT en mode normal), onglets Results / Explain / Chart, historique | [DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md), [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md), [Emergency DB Access Contract](../contracts/database/MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md) | Prévu |
| **Table Editor (création table, schema)** | Création de table (nom, description, Realtime, colonnes, types, clés étrangères) ; extension du CRUD existant ; pas de RLS (service hors-bord) | [DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md), [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md) | Prévu |
| **Observability / Métriques dynamiques** | Page Métriques avec rafraîchissement des données (requêtes/s, latence, pool DB, système) ; polling ou SSE | [DB Metrics Contract](../contracts/monitoring/MiyukiniAdmin%20-%20DB%20Metrics%20Contract.md), [Affichage Dynamique et Metriques](../ui/MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md) | Prévu |
| **Logs** | Consultation des logs d’audit et opérationnels ; flux continu (SSE) ou pagination | [Affichage Dynamique et Metriques](../ui/MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md) | Prévu |
| **API Docs** | Page Documentation API : liste des endpoints, paramètres ; statique ou enrichie par `/api/status` | [Pages et Outils Reference Supabase](./MiyukiniAdmin%20-%20Pages%20et%20Outils%20Reference%20Supabase.md) | Prévu |
| **Storage / Buckets** | Gestion des buckets ou assets sous autorité KindMother (si dans le périmètre) | [Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md) | À définir |

### 4.2 Détail des capacités livrées

#### Dashboard
- Affichage de l’état du système (en ligne / version).
- Liste des cores système (Kernel, StrongFather, KindMother, etc.).
- Liens vers Database, Tests, Sécurité (selon implémentation).

**Référence :** [Miyukini Conceptual References - MiyukiniAdmin Status](../../../reference/Miyukini%20Conceptual%20References%20-%20MiyukiniAdmin%20Status.md) — périmètre « Monitoring & Métriques ».

#### Serveur HTTP/HTTPS
- Serveur HTTP sur `MIYUKINIADMIN_HOST:MIYUKINIADMIN_PORT`.
- Option HTTPS pour affichage sécurisé (certificat et clé configurables).
- Détails : [MiyukiniAdmin - Serveur HTTP HTTPS](../operations/MiyukiniAdmin%20-%20Serveur%20HTTP%20HTTPS.md).

#### CRUD tables et manipulation des données
- **Liste des tables** : exploration des tables disponibles (via KindMother en production).
- **Lecture** : consultation des lignes (pagination, filtres, tri).
- **Création / mise à jour / suppression** : selon contrat DB (validation StrongFather, médiation KindMother) ou mode recovery.
- **Export** : CSV/JSON (lecture seule).

**Références :**
- [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md) (READ-001 à MIG-003).
- [DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md).

#### Page Tests / flux
- Page dédiée (`/tests`) avec plusieurs scénarios de test de flux.
- Vérification du comportement des cores (disponibilité, statut, réponses attendues).
- Référence : [Cycle Tests Contract](../contracts/testing/MiyukiniAdmin%20-%20Cycle%20Tests%20Contract.md), [Unit Tests Contract](../contracts/testing/MiyukiniAdmin%20-%20Unit%20Tests%20Contract.md).

#### Capacités prévues (affichage dynamique, SQL/DB)
- **SQL Editor (Query)** : console SQL avec exécution SELECT, onglets Results / Explain / Chart ; mode Recovery pour écriture (voir [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md), [Pages et Outils Reference Supabase](./MiyukiniAdmin%20-%20Pages%20et%20Outils%20Reference%20Supabase.md)).
- **Table Editor** : création de table (nom, description, Realtime, colonnes et types) ; voir [Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md). Pas de RLS (MiyukiniAdmin = service hors-bord).
- **Observability / Métriques dynamiques** : page dédiée avec métriques DB et système rafraîchies (polling ou SSE, Rust-first). Voir [Affichage Dynamique et Metriques](../ui/MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md).
- **Logs** : flux des logs d’audit et opérationnels (SSE ou pagination).
- **API Docs** : page listant les endpoints et paramètres (inspiration Supabase).

---

## 5. Alignement avec les documents de référence

| Thème référence | Document(s) | Impact sur MiyukiniAdmin |
|----------------|-------------|--------------------------|
| Statut Opérateur Souverain | MiyukiniAdmin Status | Aucune API publique ; accès exclusif via BondingBrother ; console root |
| Sécurité | Security Levels, Security Protocols | Niveaux 0–4, protocole renforcé pour recovery |
| Données | Accès DB et droits agents | Toutes les opérations DB via KindMother ; écriture directe uniquement en recovery |
| Cores | Definition COG, Connexion Inter-COG | Liste des cores affichée ; tests de flux pour valider les interactions |
| Autonomie | Lois Autonomie Système | Offline possible pour monitoring ; pas de dépendance cachée |

---

## 6. Documents associés

- [MiyukiniAdmin - Index de Navigation](../_index.md)
- [MiyukiniAdmin - Pages et Outils Reference Supabase](./MiyukiniAdmin%20-%20Pages%20et%20Outils%20Reference%20Supabase.md)
- [MiyukiniAdmin - Reference SQL et DB](./MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md)
- [MiyukiniAdmin - Affichage Dynamique et Metriques](../ui/MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md)
- [MiyukiniAdmin - Documentation Fondatrice](../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- [MiyukiniAdmin - Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md)
- [MiyukiniAdmin - Serveur HTTP HTTPS](../operations/MiyukiniAdmin%20-%20Serveur%20HTTP%20HTTPS.md)
- [Miyukini Conceptual References - MiyukiniAdmin Status](../../../reference/Miyukini%20Conceptual%20References%20-%20MiyukiniAdmin%20Status.md)

---

**Date de création :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Document de référence
