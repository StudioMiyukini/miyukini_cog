# MiyukiniAdmin — Organisation des pages et UX DB

## 1. Contexte

Ce document definit les **besoins utilisateur**, l'**organisation des pages** (arborescence, routes) et les **parcours utilisateur** pour la partie base de donnees de MiyukiniAdmin, en coherence avec les interfaces deja specifiees (DB Management Interface, Dashboard & Metrics Display).

## 2. Portee / Scope

Ce document definit :
- Les besoins explicites pour le Dashboard et la section Database
- L'arborescence des pages (sidebar, routes)
- Les parcours utilisateur types (ex. appliquer une migration, consulter un backup)
- Les references aux ecrans existants

Ce document **ne couvre pas** :
- L'implementation technique des composants
- Les contrats de donnees (voir contrats database)

---

## 3. Besoins explicites

### 3.1 Dashboard

- **Vue d'ensemble** : sante systeme, niveau de confiance (T0–T4), niveau de securite (0–4), metriques systeme (CPU, RAM, disque, reseau), metriques DB (requetes/s, latence, pool), statut des Operateurs, alertes recentes.
- **Actions rapides** : liens vers Database, Metriques, Tests, Securite, Logs.
- **Rafraichissement** : manuel ou auto (5s, 15s, etc.) selon niveau de securite.

**Reference :** [MiyukiniAdmin - Dashboard & Metrics Display](./MiyukiniAdmin%20-%20Dashboard%20&%20Metrics%20Display.md)

### 3.2 Database — Tables

- **Liste des tables** : nom, nombre de lignes, taille, index, derniere modification, actions (Voir, Structure, Index, Export, Stats).
- **Detail d'une table** : onglets Structure / Data / Indexes / Stats / Export.
- **Donnees** : pagination, filtres (colonnes, operateurs, valeurs), tri, export CSV/JSON.
- **Structure** : colonnes (nom, type, nullable, default), cles etrangeres, export DDL.

**Reference :** [MiyukiniAdmin - DB Management Interface](./MiyukiniAdmin%20-%20DB%20Management%20Interface.md) §3–§6

### 3.3 Database — Query

- **Console SQL** : zone de saisie, execution, historique, requetes sauvegardees.
- **Mode normal** : SELECT uniquement (lecture seule) ; validation StrongFather ; timeout, LIMIT force.
- **Resultats** : affichage tabulaire, export CSV/JSON, copie.

**Reference :** [MiyukiniAdmin - DB Management Interface](./MiyukiniAdmin%20-%20DB%20Management%20Interface.md) §7

### 3.4 Database — Migrations

- **Liste des migrations** : appliquees (date, duree, resultat) et en attente (nom, version).
- **Detail d'une migration** : contenu du script, checksum, statut.
- **Execution** : declencher l'application des migrations en attente (avec validation StrongFather, backup automatique, pre/post tests).
- **Historique** : consultation complete, filtres par date/resultat.

**Reference :** [MiyukiniAdmin - DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md) §9, [Gestion DB type Supabase](../reference/MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md)

### 3.5 Database — Backups

- **Liste des sauvegardes** : date, type (complet / incrementale), taille, statut.
- **Declenchement manuel** : bouton "Creer une sauvegarde" (si prevu), justification optionnelle selon niveau securite.
- **Restauration** : selection d'un backup, justification obligatoire, confirmation, workflow controle (StrongFather, WorrySentinel).

**Reference :** [MiyukiniAdmin - Backup Restore Contract](../contracts/database/MiyukiniAdmin%20-%20Backup%20Restore%20Contract.md)

### 3.6 Database — Maintenance

- **Validation** : lancer une validation complete (integrite, contraintes, conformite schema) ; affichage dernier run et statut.
- **Optimisation** : vacuum, reindex, mise a jour statistiques sur tables selectionnees ; approbation StrongFather, justification.
- **Reparation** : correction orphelins, doublons, contraintes ; conditions et justification (voir DB Operations Contract).

**Reference :** [MiyukiniAdmin - DB Management Interface](./MiyukiniAdmin%20-%20DB%20Management%20Interface.md) §8

### 3.7 Database — Recovery

- **Activation** : conditions cumulatives (T3/T4, protocole renforce, MFA, justification, approbation StrongFather, duree max).
- **Console Recovery** : SQL en ecriture (UPDATE, INSERT, DELETE, etc.) ; preview "rows affected", confirmation explicite ; log de session.
- **Fin de session** : bouton "Terminer Recovery" ; deblocage des Operateurs.

**Reference :** [MiyukiniAdmin - DB Management Interface](./MiyukiniAdmin%20-%20DB%20Management%20Interface.md) §9, [Emergency DB Access Contract](../contracts/database/MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md)

### 3.8 Autres sections

- **Metriques** : detail systeme, DB, Operateurs (deja evoque dans Dashboard).
- **Tests** : page dediee tests de flux (cores).
- **Securite** : panneau niveaux de securite, etats de confiance.
- **Logs** : consultation des logs d'audit et operationnels.

---

## 4. Arborescence des pages (sidebar / routes)

Alignement avec le modele Supabase (Studio) : Project Overview, Table Editor, SQL Editor, Database, Migrations, Backups, Observability, Logs, API Docs, Project Settings.

```
Sidebar / Routes
├── Project Overview (ou Dashboard)   /  ou  /dashboard
├── Table Editor (Database > Tables) /database  ou  /database/tables
│   └── [table]                       /database/tables/:tableId
│       └── Structure|Data|Indexes|Stats|Export  (sous-onglets)
├── SQL Editor (Database > Query)    /database/query
├── Database (vue generale)          /database  (schemas, roles, connexion)
├── Migrations                       /database/migrations
│   └── [migration]                  /database/migrations/:migrationId  (detail)
├── Backups                          /database/backups
├── Maintenance                      /database/maintenance
├── Recovery                         /database/recovery
├── Observability (ou Metriques)     /metriques
├── Logs                             /logs
├── API Docs                         /api-docs
├── Project Settings (ou Parametres) (a definir)
├── Tests                            /tests
└── Securite                         /securite
```

**Remarques :**
- La section Database peut etre un groupe pliable dans la sidebar avec sous-entrees (Tables, Query, Migrations, Backups, Maintenance, Recovery).
- Breadcrumb recommande : ex. `Dashboard > Database > Tables > users > Structure`.
- **Reference :** [MiyukiniAdmin - Pages et Outils Reference Supabase](../reference/MiyukiniAdmin%20-%20Pages%20et%20Outils%20Reference%20Supabase.md) pour la correspondance Supabase / MiyukiniAdmin.

### 4.1 Pages pour affichage dynamique

Les pages suivantes permettent de voir des modifications de facon dynamique (metriques, infos DB, logs) :

| Page | Donnees dynamiques | Methode (Rust-first) |
|------|--------------------|----------------------|
| **Dashboard / Project Overview** | Statut systeme, cartes, PROJECT API | Polling (ex. 30 s) ou statique |
| **Metriques / Observability** | Metriques DB (requetes/s, latence, pool), systeme | SSE ou polling (Rust) |
| **Logs** | Flux logs audit / operationnels | SSE pour flux continu (Rust) |
| **Table Editor** | Liste tables, schema, recent items | Polling ou bouton "Refresh" |
| **Table Editor — Donnees table** | Lignes, pagination | Polling ou refresh a la demande |

**Reference :** [MiyukiniAdmin - Affichage Dynamique et Metriques](./MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md).

---

## 5. Parcours utilisateur

### 5.1 Appliquer une migration

1. Aller dans **Database > Migrations**.
2. Consulter la liste des migrations en attente.
3. Optionnel : ouvrir le detail d'une migration (contenu, checksum).
4. Cliquer sur "Appliquer les migrations en attente" (ou equivalent).
5. Confirmer (backup automatique, pre-tests, execution, post-tests).
6. Affichage du resultat (succes / echec, rollback si echec).
7. Historique mis a jour.

### 5.2 Consulter un backup

1. Aller dans **Database > Backups**.
2. Consulter la liste des sauvegardes (date, type, taille, statut).
3. Optionnel : filtrer par date ou type.
4. Cliquer sur une ligne pour voir le detail (chemin, checksum, etc.) si disponible.

### 5.3 Restaurer depuis un backup

1. Aller dans **Database > Backups**.
2. Selectionner un backup cible.
3. Cliquer sur "Restaurer".
4. Saisir la justification (obligatoire).
5. Confirmer (avertissement perte de donnees recentes).
6. Validation StrongFather (workflow backend).
7. Affichage du resultat ; traçabilite.

### 5.4 Lancer une requete SQL (lecture seule)

1. Aller dans **Database > Query**.
2. Saisir une requete SELECT dans la console.
3. Cliquer sur "Executer".
4. Consulter les resultats (tableau, export si besoin).
5. Optionnel : sauvegarder la requete ou consulter l'historique.

### 5.5 Explorer une table

1. Aller dans **Database > Tables**.
2. Cliquer sur une table (ex. `users`).
3. Par defaut : onglet **Data** (pagination, filtres).
4. Changer d'onglet : **Structure**, **Indexes**, **Stats**, **Export** selon besoin.

---

## 6. References aux ecrans existants

| Besoin / Page | Document de specification |
|---------------|----------------------------|
| Dashboard | [MiyukiniAdmin - Dashboard & Metrics Display](./MiyukiniAdmin%20-%20Dashboard%20&%20Metrics%20Display.md) |
| Database — layout, Tables, Query, Maintenance, Recovery | [MiyukiniAdmin - DB Management Interface](./MiyukiniAdmin%20-%20DB%20Management%20Interface.md) |
| Securite (niveaux, panneau) | [MiyukiniAdmin - Security Control Panel](./MiyukiniAdmin%20-%20Security%20Control%20Panel.md) |
| Philosophie UI (couleurs, typo, etats) | [MiyukiniAdmin - UI Design Philosophy](./MiyukiniAdmin%20-%20UI%20Design%20Philosophy.md) |

Les pages **Migrations** et **Backups** etendent le perimetre decrit dans DB Management Interface ; le present document en fixe l'organisation et les parcours.

---

## 7. Documents associes

- [MiyukiniAdmin - Pages et Outils Reference Supabase](../reference/MiyukiniAdmin%20-%20Pages%20et%20Outils%20Reference%20Supabase.md)
- [MiyukiniAdmin - Affichage Dynamique et Metriques](./MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md)
- [MiyukiniAdmin - DB Management Interface](./MiyukiniAdmin%20-%20DB%20Management%20Interface.md)
- [MiyukiniAdmin - Dashboard & Metrics Display](./MiyukiniAdmin%20-%20Dashboard%20&%20Metrics%20Display.md)
- [MiyukiniAdmin - Gestion DB type Supabase](../reference/MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md)
- [MiyukiniAdmin - DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md)
- [MiyukiniAdmin - Backup Restore Contract](../contracts/database/MiyukiniAdmin%20-%20Backup%20Restore%20Contract.md)

---

**Date de creation :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Document de reference (UX / organisation pages)
