# JayFestival — Référence base de données et migration Supabase → SQLite

## Contexte

**JayFestival** a pour **genèse** le projet **Catakana**, qui est **pré-COG** : il repose sur **Supabase** (PostgreSQL, Auth, Storage, Realtime) comme unique backend. Cette situation est documentée comme **exception explicite** : Supabase est le backend de la **version alpha** fonctionnelle de JayFestival. Une **migration** vers **SQLite + outils maison** (KindMother, persistance COG-native) est prévue pour la version COG-native ultérieure.

Ce document décrit : (1) l’**état actuel** des interactions avec la base Supabase (Catakana), (2) le **mapping** tables / services → concepts JayFestival et Kits, (3) la **stratégie de migration** Supabase → SQLite + outils maison, (4) le positionnement de la **version alpha** (fonctionnelle, pas MVP — Catakana fonctionne déjà).

**Références** : [Document fondateur](../JayFestival%20-%20Document%20Fondateur.md), [Bornage Implementation](../JayFestival%20-%20Bornage%20Implementation.md), [Audit documentation Catakana](../JayFestival%20-%20Audit%20Documentation%20Catakana.md) ; Catakana `.Catakana/docs/reference/database_schema.md`, `.Catakana/supabase/migrations/`, `.Catakana/src/lib/supabase/`.

## Portée / Scope

- **Périmètre** : État actuel Supabase (tables, RLS, services Catakana) ; mapping vers JayFestival ; stratégie de migration vers SQLite + outils maison ; version alpha (Supabase autorisé).
- **Hors périmètre** : Implémentation détaillée des outils maison (KindMother, schémas SQLite) — référencée dans les contrats COG.

---

## 1. Exception pré-COG : Supabase comme backend alpha

### 1.1 Principe

- **Catakana** = application fonctionnelle **pré-COG** : stack React + Supabase (PostgreSQL, Auth, Storage).
- **JayFestival** reprend les spécificités de Catakana et les porte à l’échelle COG. Pour livrer une **version alpha fonctionnelle** sans attendre la complète implémentation COG (KindMother, SQLite, Mandats côté persistance), le **backend Supabase** est **autorisé en exception** pour l’alpha.
- **Alpha** = version **fonctionnelle** (pas un MVP « minimal ») : Catakana fonctionne déjà ; l’alpha JayFestival vise une reprise fonctionnelle (egui/eframe + même périmètre métier) avec Supabase en backend, puis migration documentée vers SQLite + outils maison.

### 1.2 Règles

| Règle | Description |
|-------|-------------|
| **Alpha** | Backend = Supabase (client REST/PostgREST, Auth). Pas de violation des lois COG pour la persistance tant que l’alpha est clairement marquée « pre-COG / exception Supabase ». |
| **Authentification alpha** | Supabase Auth (email/mot de passe, lien magique) ; pas encore Miyauth natif. Les rôles (`user_type` dans `profiles`) servent de proxy pour Admin, Manager, Exposant, Bénévole, Visiteur. |
| **Persistance alpha** | Tables Supabase (PostgreSQL) ; pas encore KindMother/SQLite. Les services Catakana (`editionService`, `exposantService`, `budgetService`, etc.) restent la référence des **interactions avec la DB**. |
| **Migration obligatoire** | Pour la version COG-native (post-alpha), la persistance devra migrer vers SQLite + outils maison (KindMother, contrats de persistance). Ce document et le Bornage en fixent la stratégie. |

### 1.3 Auth JayFestival dérivée de Catakana

**JayFestival aura une Auth à lui**, dérivée de celle de **Catakana**, qui utilise l’**Auth Supabase**.

| Aspect | Description |
|--------|-------------|
| **Genèse** | Catakana repose sur Supabase Auth (email/mot de passe, lien magique) ; les rôles sont portés par `profiles.user_type` (admin, manager, exhibitor, volunteer, visitor). |
| **Auth JayFestival** | Couche d’authentification **propre à JayFestival**, dérivée de l’Auth Catakana : mêmes mécanismes (Supabase Auth en alpha), même modèle de rôles (`profiles`), mais **implémentation et contrat d’usage sous la responsabilité JayFestival** (écrans Connexion/Inscription, session, redirection selon type de compte). |
| **Alpha** | En alpha, l’Auth JayFestival s’appuie sur **Supabase Auth** (client Auth Supabase appelé depuis JayFestival) ; pas encore Miyauth natif. |
| **Post-alpha** | La bascule vers **Miyauth** (COG-native) sera documentée ; l’Auth JayFestival restera la couche d’usage côté JayFestival, en consommant Miyauth en backend. |

### 1.4 Décision alpha : garder Supabase pour JayFestival

**En alpha, et vu que Catakana est déjà en production**, on **garde Supabase** pour JayFestival.

| Aspect | Description |
|--------|-------------|
| **Justification** | Catakana repose déjà sur Supabase en production ; l’alpha JayFestival réutilise la même infrastructure (Auth, PostgreSQL, Storage) pour une reprise fonctionnelle sans dupliquer ni migrer prématurément. |
| **Effet** | Backend alpha = Supabase Catakana (ou instance dédiée alignée sur le même schéma). Pas de dépendance à une autre base ni à SQLite/KindMother pour l’alpha. |
| **Périmètre** | Toutes les données métier JayFestival (éditions, exposants, candidatures, budget, programme, documents, etc.) résident sur Supabase pendant la phase alpha. |

### 1.5 Données mère et tracker (alpha)

En alpha, le **Supabase Catakana** tient lieu à la fois de **données mère** et de **référence centrale pour la traçabilité** (tracker).

| Rôle | Description |
|------|-------------|
| **Données mère** | Source de vérité persistée pour JayFestival : toutes les tables métier (profiles, exposants, editions, editions_exposants, etc.) ; équivalent fonctionnel d’une Instance Mère KindMother pour la phase alpha. |
| **Tracker** | Référence centrale où sont stockées et tracées les données : une seule base (Supabase) comme lieu de persistance et de cohérence ; pas de dispersion des données métier en alpha. |
| **Post-alpha** | Après migration, la donnée mère et la référence de traçabilité deviennent **KindMother + SQLite** ; Supabase peut être conservé en option comme backup (voir § 4.4). |

---

## 2. État actuel : schéma Supabase (Catakana)

### 2.1 Tables principales (noyau JayFestival)

Les tables ci-dessous sont celles **indispensables** pour une alpha fonctionnelle alignée sur Catakana. Les noms et types sont déduits de la doc Catakana et des migrations.

| Table | Rôle | Mapping JayFestival / Kit |
|-------|------|----------------------------|
| **profiles** | Utilisateurs (id, username, user_type, email, avatar_url, theme, etc.) | Miyauth / Miyuprofile (alpha = proxy via Supabase Auth + profiles). |
| **rpg_stats** | Stats RPG (user_id, rank, strength, magic, wisdom) | Hors scope alpha JayFestival ou optionnel. |
| **exposants** | Annuaire exposants (company_name, stand_name, contact_*, adresse, logo_url, siret, etc.) | Kit Exposants / Répertoire ; fiche exposant. |
| **editions** | Éditions / événements (name, slug, start_date, end_date, location, theme, status) | Kit Éditions. |
| **editions_exposants** | Participations exposant × édition (exposant_id, edition_id, is_accepted, is_validated, is_paid, assigned_stand, size_meters, etc.) | Kit Exposants (côté organisateur) ; Candidatures. |
| **edition_team** | Équipe par édition (edition_id, profile_id, role) | Kit Équipe & Permissions. |
| **stands** | Stands par édition (edition_id, nom, largeur_m, longueur_m, zone, position_plan JSONB, etc.) | Kit Plan de salle. |
| **emplacements** | Emplacements (edition_id, exposant_id via editions_exposants, statut) | Plan de salle ; attribution. |
| **budget_entries** | Revenus/dépenses par édition (edition_id, amount, type, category, date, receipt_url, status) | Kit Budget ; Miyucptaledger / JayKonta (alpha = table locale). |
| **invoices** | Factures (edition_id, reservation_id → editions_exposants, statut, montants) | Miyuinvoice / JayKonta (alpha = tables invoice_*). |
| **invoice_items** | Lignes de facture | Idem. |
| **invoice_settings** | Paramètres facturation (TVA, etc.) | Idem. |
| **schedule_slots** | Créneaux planning (edition_id, …) | Kit Programme ; JayKoa (données créneaux). |
| **program_events** | Événements programme (edition_id, …) | Kit Programme. |
| **events** | Événements agenda (edition_id, …) | Programme / Agenda. |
| **floor_plans** | Plans d’implantation (edition_id, plan_data JSONB, grid_*) | Kit Plan de salle (schéma Catakana `floor_plans.md`). |
| **news** | Actualités (édition ou global) | Annonces / Actualités (alpha). |
| **documents** / **edition_docs** | Documents par édition (contrats, règlements) | Kit Documents & Légal ; Miyucms/Miyumedia (alpha = tables locales). |
| **notification_settings** | Paramètres notifications | Miyunotify (alpha). |
| **email_logs** | Logs envois email | Miyunotify. |

### 2.2 Tables complémentaires (Catakana — alpha ou phase 2)

| Table | Rôle | Alpha / Phase 2 |
|-------|------|------------------|
| **catakana_guests**, **catakana_guest_performances**, **catakana_guest_performance_schedules**, **catakana_guest_performance_bookings** | Invités, prestations, réservations | Phase 2 (services visiteur) ou alpha si reprise complète. |
| **catakana_ateliers**, **catakana_atelier_slots**, **catakana_atelier_reservations** | Ateliers, créneaux, réservations | Alpha si réservations ateliers ; sinon phase 2. |
| **catakana_animations**, **catakana_animation_slots**, **catakana_animation_reservations** | Animations, créneaux, réservations | Idem. |
| **catakana_jeux**, **catakana_jeu_slots**, **catakana_jeu_reservations** | Jeux, créneaux, réservations | Phase 2. |
| **catakana_concours*** | Concours, sessions, inscriptions, jury | Phase 2. |
| **gamification_rewards**, **gamification_reward_claims** | Récompenses, réclamations | Hors scope alpha. |
| **material_*** | Gestion matériel (catalogue, propriétaires, inventaire, listes) | Phase 2 / optionnel. |
| **communications_campaigns**, **communications_posts** | Campagnes et posts communication | Annonces (alpha = notification_settings + email_*). |
| **email_campaigns**, **email_templates**, **email_recipients**, **email_engagement** | Emailing | Miyunotify (alpha ou phase 2). |
| **catakana_intervenants**, **catakana_intervenant_*** | Intervenants, plannings, réservations, interventions | Phase 2 ou optionnel. |
| **archived_stands**, **archived_emplacements**, **archived_edition_exposants** | Archivage éditions | Alpha si clôture d’édition ; sinon phase 2. |
| **user_role_history**, **user_sanctions_history** | Historique rôles, sanctions | Optionnel alpha. |

### 2.3 Relations clés (résumé)

```
profiles 1:1 exposants (id = exposant.id)
profiles 1:1 rpg_stats (user_id)
editions 1:N editions_exposants
exposants 1:N editions_exposants
editions 1:N edition_team
editions 1:N stands
editions 1:N emplacements
editions 1:N budget_entries
editions 1:N schedule_slots, program_events, events
editions_exposants 1:N invoices (reservation_id)
editions 1:N floor_plans (si table créée)
```

### 2.4 Row Level Security (RLS) — Catakana

- **editions** : lecture publique ; écriture (INSERT/UPDATE/DELETE) pour utilisateurs authentifiés (avec politiques par rôle si besoin).
- **editions_exposants** : SELECT public ou par rôle ; ALL pour admin ; ALL pour manager des éditions (via `edition_team`) ; SELECT pour exposant sur ses propres lignes (`exposant_id = auth.uid()`).
- **profiles** : lecture/écriture selon `auth.uid()` et rôle (admin peut tout modifier).
- **budget_entries** : filtré par `edition_id` et rôle (admin/manager édition).
- **invoices** : lié à `editions_exposants` ; admin/manager édition + exposant sur ses factures.

Référence détaillée : `.Catakana/docs/reference/README_RLS_PERMISSIONS.md`.

---

## 3. Interactions avec la DB : services Catakana → Kits JayFestival

### 3.1 Mapping services Supabase → Kits / Opérateurs

| Service Catakana (src/lib/supabase ou services) | Tables principales | Kit / Opérateur JayFestival |
|-------------------------------------------------|--------------------|-----------------------------|
| **editionService** | editions | Kit Éditions ; JayFestival Organisateur, JayFestival Édition. |
| **exposantService** | exposants | Kit Exposants (fiche) ; Répertoire Exposants. |
| **editionExhibitorService** / **editionCandidatureService** | editions_exposants, exposants | Kit Exposants (côté organisateur) ; Candidatures. |
| **budgetService** | budget_entries | Kit Budget ; JayKonta (alpha = table locale). |
| **invoiceService** | invoices, invoice_items, invoice_settings | Miyuinvoice / JayKonta. |
| **floorPlanService** | stands, emplacements, floor_plans (si existant) | Kit Plan de salle. |
| **agendaService** / **programService** / **eventsService** | schedule_slots, program_events, events | Kit Programme ; JayKoa (entrées agenda). |
| **editionDocsService** / **documentsService** | documents, edition_docs | Kit Documents & Légal. |
| **profileService** | profiles | Miyauth / Miyuprofile (alpha). |
| **notificationSettingsService** / **notificationService** | notification_settings, email_logs | Miyunotify. |
| **newsService** | news | Annonces / Actualités (alpha). |
| **reglementsService** | documents (is_public, type) | Documents & Légal (public). |
| **publicExhibitorService** | exposants (flags _public) | Répertoire Exposants (catalogue). |
| **editionTeamService** | edition_team | Kit Équipe & Permissions. |

### 3.2 Client Supabase (Catakana)

- **Singleton** : `getSupabaseClient()` dans `src/lib/supabase/client.ts`.
- **Auth** : `supabase.auth` (signIn, signUp, signOut, onAuthStateChange) ; création auto de profil dans `profiles` à la première connexion.
- **Accès données** : `supabase.from('table').select().eq().insert().update().delete()` ; types TypeScript via `@/types/supabase` (générés ou manuels).

Pour l’**alpha JayFestival** (egui/eframe) : soit appel HTTP direct à l’API Supabase (REST) depuis Rust, soit client existant si une couche web reste en place. La documentation des **endpoints et filtres** utilisés par chaque service Catakana sert de **spécification** pour l’adapter côté Rust ou pour reproduire la logique en SQLite.

---

## 4. Stratégie de migration : Supabase → SQLite + outils maison

### 4.1 Objectif

- **Post-alpha** : disposer d’une persistance **COG-native** : **SQLite** (ou autre moteur local gouverné) + **outils maison** (KindMother, contrats de persistance), sans dépendance critique à Supabase (alignement LOI-1, LOI-2, LOI-3).
- **Migration** = processus **formel** : schémas SQLite alignés sur le métier JayFestival, scripts d’export Supabase → import SQLite, puis bascule des clients (UI, services) vers la couche KindMother/SQLite.

### 4.2 Étapes (orientations)

| Étape | Description |
|-------|-------------|
| **1. Schéma SQLite cible** | Définir les tables SQLite équivalentes aux tables Supabase du noyau (profiles, exposants, editions, editions_exposants, edition_team, stands, emplacements, budget_entries, invoices, invoice_items, schedule_slots, program_events, events, documents, notification_settings, news). Contraintes, index, types (UUID → BLOB ou TEXT). |
| **2. Contrats KindMother** | Définir les contrats de persistance (WriteIntent, lectures) par entité (édition, exposant, candidature, budget, facture, programme, plan, document) pour que l’alpha « Supabase » soit remplaçable par une implémentation « KindMother + SQLite » sans changer les cas d’usage métier. |
| **3. Export Supabase** | Scripts ou outils d’export (par table ou par schéma) depuis Supabase vers fichiers (CSV, JSON ou SQLite) avec gestion des clés étrangères et de l’ordre d’import. |
| **4. Import SQLite** | Scripts d’import des données exportées dans le schéma SQLite cible ; vérification d’intégrité (comptages, contraintes). |
| **5. Couche d’abstraction** | Dans l’application (ou les services), une couche « persistance » qui pointe soit vers Supabase (alpha), soit vers KindMother/SQLite (post-alpha), selon configuration ou feature flag. |
| **6. Bascule** | Passage en production (ou en livrable) sur SQLite + outils maison ; Supabase en lecture seule, décommissionné, ou conservé comme serveur zéro (backup) — voir § 4.4. |

### 4.3 Outils maison (cible)

- **KindMother** : autorité de persistance (écritures via WriteIntent, lectures via contrats).
- **SQLite** : stockage local (fichier .db ou intégré) ; pas de serveur externe (LOI-1, LOI-2).
- **Outils annexes** : génération de schéma à partir des contrats ; migrations de schéma SQLite (versions) ; sauvegardes et restauration.

Les spécifications détaillées des **contrats KindMother** et du **schéma SQLite** relèvent des documents COG (KindMother, contrats d’intégration) et peuvent être créées en parallèle ou après la stabilisation de l’alpha.

### 4.4 Option : Supabase conservé comme serveur zéro (backup)

Après la bascule vers **SQLite + KindMother**, le **Supabase Catakana** peut être **conservé** comme **serveur zéro** : cible de backup et source de restauration, **sans être une dépendance critique à l’exécution** (conformité LOI-1).

| Aspect | Description |
|--------|-------------|
| **Rôle** | **Backup** : réplication périodique (SQLite → Supabase ou export vers Supabase) ; **Restauration** : en cas d’incident sur le SQLite local, restauration depuis un export Supabase vers SQLite. |
| **Contrainte** | À l’exécution, le système **ne dépend pas** de Supabase : persistance normale = KindMother + SQLite ; Supabase n’est sollicité que pour les flux backup/restauration. |
| **Protocole** | PostgreSQL (Supabase) et SQLite partagent le langage SQL mais avec des différences de dialecte (types, fonctions) ; les scripts d’export/import de la migration servent aussi pour ce flux backup/restore. |
| **Bénéfice** | Réutilisation d’une infrastructure robuste (Supabase) comme copie de secours, sans remettre en cause l’autonomie du système en production. |

---

## 5. Version alpha : positionnement et critères

### 5.1 Alpha = fonctionnelle (pas MVP minimal)

- **Catakana** fonctionne déjà en production (React + Supabase). L’**alpha JayFestival** vise une **reprise fonctionnelle** du même périmètre métier (catalogue, organisateurs, exposants, visiteurs, éditions, plan, programme, budget, documents, notifications, facturation) avec :
  - **UI** : stack egui/eframe (voir [Reference UI Transcription Catakana](../JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md)).
  - **Backend** : **Supabase** en exception pré-COG (Auth + PostgreSQL + Storage si besoin).
- **Pas un MVP « minimal »** : l’alpha est **fonctionnelle** (parcours principaux utilisables, données cohérentes avec Catakana).

### 5.2 Critères de livraison alpha

| Critère | Description |
|---------|-------------|
| **CF-ALPHA-1** | Catalogue (annuaire événements, répertoires organisateurs/exposants) accessible en lecture ; données fournies par Supabase. |
| **CF-ALPHA-2** | Organisateur : connexion (Supabase Auth), liste des éditions, dashboard par édition, exposants (candidatures, validation, fiches, devis/facture), plan de salle, programme, budget, documents, annonces. |
| **CF-ALPHA-3** | Exposant : dashboard (candidatures, participations, documents, factures) ; données depuis Supabase. |
| **CF-ALPHA-4** | Visiteur : espace dédié (agenda, billets, réservations, pass) selon tables Supabase existantes. |
| **CF-ALPHA-5** | Rôles et accès : cohérents avec `profiles.user_type` et RLS Supabase (admin, manager, exhibitor, volunteer, visitor). |
| **CF-ALPHA-6** | UI : thème et écrans principaux selon [Reference UI Transcription Catakana](../JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md). |
| **CF-ALPHA-7** | Documentation : Document fondateur, Bornage (alpha + migration), Reference Base de Donnees et Migration, Reference UI, Interpolarité à jour. |

### 5.3 Ce que l’alpha n’exige pas

- **KindMother / SQLite** : pas encore en place ; Supabase suffit pour l’alpha.
- **Miyauth / Master Butler natifs** : pas encore ; Supabase Auth + `profiles.user_type` + RLS suffisent.
- **Mandats de Permission** : modélisation conceptuelle documentée ; implémentation côté persistance en post-alpha.

---

## 6. Références

| Document | Rôle |
|----------|------|
| [JayFestival - Document Fondateur](../JayFestival%20-%20Document%20Fondateur.md) | Vision, macro, distribution. |
| [JayFestival - Bornage Implementation](../JayFestival%20-%20Bornage%20Implementation.md) | Périmètre alpha, phase 2, migration, critères. |
| [JayFestival - Audit Documentation Catakana](../JayFestival%20-%20Audit%20Documentation%20Catakana.md) | Métriques, manques. |
| [JayFestival - Reference UI Transcription Catakana](../JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) | UI egui/eframe. |
| Catakana `docs/reference/database_schema.md` | Schéma résumé Catakana. |
| Catakana `docs/reference/README_RLS_PERMISSIONS.md` | RLS Supabase. |
| Catakana `supabase/migrations/` | Migrations SQL Supabase. |
| Catakana `src/lib/supabase/*.ts` | Services et client Supabase. |

---

**Document** : JayFestival — Référence base de données et migration Supabase → SQLite  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Document de référence (DB alpha, migration)
