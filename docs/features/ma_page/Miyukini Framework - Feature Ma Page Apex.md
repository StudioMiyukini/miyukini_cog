# Miyukini Framework - Feature Ma Page Apex

## Contexte

La fonctionnalité **Ma Page** permet d’avoir un **profil enrichi** et **composable** (une “page profil”) dont le contenu varie selon :
- le **rôle** (pro / artisan / exposant / manager / bénévole / visiteur…),
- des **tags** (ex: `tatoueur`, `food`, `vip`, `jeux_video`, `speaker`, `orga`),
- et les modules installés (Agenda/Booking, Boutique, RPG, Documents, etc.).

Le back-office (ou un module) peut :
- créer des **types de profils** (templates) ;
- définir les règles d’assignation (par rôle, tag, contexte) ;
- assigner un type à un utilisateur (ou à un “profil public”) ;
- choisir les “blocs” affichés (présentation, galerie, horaires, réservation, mini-boutique, stats RPG…).

## Portée / Scope

- **Capability** : Ma Page (page builder de profil).
- **Objectifs** :
  - rendre le profil **évolutif** sans refactor UI à chaque nouveau module,
  - standardiser l’intégration modules ↔ blocs “Ma Page”,
  - garantir la sécurité (RLS) et éviter les fuites de données.
- **Hors scope** :
  - devenir un CMS complet,
  - éditeur WYSIWYG avancé (peut venir plus tard).

---

## 1) Concepts

- **Profile Type** : un template de page (ex: `profil_pro`, `profil_exposant`, `profil_benevole`).
- **Rule d’assignation** : conditions qui sélectionnent le type (rôle, tags, contexte).
- **Profile Page** : instance de page (pour un user/profil), avec des blocs ordonnés.
- **Block (Widget)** : unité de contenu (texte, galerie, horaires, booking, boutique…).
- **Block Provider** : un module qui “déclare” un type de bloc et sait le rendre/alimenter.

---

## 2) Exemples d’usages

### Profil “Pro / Artisan”

- Bloc Présentation + Galerie + Horaires + Zone Contact + Bloc Réservation (Agenda) + Bloc Boutique (Shop).

### Profil “Exposant”

- Bloc Présentation + Galerie + Tags + Bloc “Emplacement/Stand” (module salon) + Bloc Documents publics.

### Profil “Bénévole”

- Bloc Présentation + Bloc Planning (Agenda) + Bloc Missions + Bloc Badges/RPG.

### Profil “Manager / Admin”

- Bloc Présentation + Bloc Stats + Bloc Outils (liens rapides) + Bloc Audit.

---

## 3) APEX : Architecture / Processus / Expérience

### A = Architecture

#### Ports (interfaces)

- `ProfileTypePort`
  - CRUD `profile_types`
  - CRUD `profile_type_rules`
- `ProfilePagePort`
  - CRUD `profile_pages` + `profile_blocks`
  - publish/unpublish (public/private)
- `BlockRegistryPort`
  - registration des block types (par modules)
  - schema de config + validation
- `BlockDataPort`
  - resolve de données “dynamiques” (ex: stats RPG, créneaux agenda)

#### Events (domain ma-page)

- `profile.type.created`
- `profile.type.updated`
- `profile.type.assigned`
- `profile.page.updated`
- `profile.block.updated`
- `profile.page.published`
- `profile.page.unpublished`

#### Events consommés (cross-modules)

- `agenda.slot.confirmed` (afficher prochaines réservations si autorisé)
- `billing.invoice.paid` (badge “client premium”, factures, etc.)
- `rpg.profile.updated` (stats visibles)
- `documents.file.uploaded` (galerie/documents)

> Règle : un block “dépendant d’un module” ne doit afficher que des données accessibles selon les policies du module source.

### P = Processus (orchestration)

1) Admin crée des **Profile Types** (templates) + des rules d’assignation.
2) Un user obtient un type par défaut (role/tag) ou assignation explicite.
3) L’instance `profile_page` est créée (ou générée) + blocs par défaut.
4) L’utilisateur (ou admin) édite sa page (dans les limites autorisées).
5) La page est rendue :
   - blocs statiques (texte, horaires),
   - blocs dynamiques (agenda, boutique, rpg) via block providers.

### E = Expérience (UX)

Back-office recommandé :
- **Ma Page → Types de profils** (CRUD templates, preview)
- **Ma Page → Règles** (if role/tag/context → type)
- **Ma Page → Assignations** (user → type, exceptions)
- **Ma Page → Catalogue de blocs** (par module, configuration)
- **Ma Page → Modération/Publications** (public/privé, validation)

Front-office (user) :
- “Mon profil” → “Ma Page” → éditeur simple (réordonner, activer/désactiver, remplir).

---

## 4) Data contract (proposition)

### 4.1 Tables

#### `profile_types`
- `id` UUID
- `code` TEXT UNIQUE (ex: `profil_pro_v1`)
- `name` TEXT
- `description` TEXT
- `default_visibility` ENUM (`private`, `public`)
- `allowed_roles` TEXT[] (ex: `['pro','artisan']`) (optionnel)
- `default_blocks` JSONB (liste ordonnée de blocs + config par défaut)
- `created_by`, `created_at`, `updated_at`

#### `profile_type_rules`
- `id` UUID
- `profile_type_id` UUID
- `priority` INT (plus petit = plus prioritaire)
- `match_role` TEXT (optionnel)
- `match_tags_all` TEXT[] (optionnel)
- `match_tags_any` TEXT[] (optionnel)
- `match_context_type` TEXT (optionnel) (ex: `edition`)
- `match_context_id` TEXT/UUID (optionnel)
- `is_active` BOOLEAN

#### `profile_pages`
- `id` UUID
- `user_id` UUID (référence profiles)
- `profile_type_id` UUID
- `slug` TEXT UNIQUE (public)
- `visibility` ENUM (`private`, `public`, `unlisted`)
- `title` TEXT
- `summary` TEXT
- `tags` TEXT[]
- `meta` JSONB
- `published_at` TIMESTAMPTZ
- `created_at`, `updated_at`

#### `profile_blocks`
- `id` UUID
- `page_id` UUID
- `block_type` TEXT (ex: `rich_text`, `gallery`, `opening_hours`, `agenda_booking`, `mini_shop`, `module_rpg_stats`)
- `position` INT
- `is_enabled` BOOLEAN
- `config` JSONB (schema dépendant du type)
- `data_ref` JSONB (optionnel) (ex: `{ module:'agenda', agendaId:'...' }`)
- `created_at`, `updated_at`

#### `profile_tags` (optionnel)
- `id` UUID
- `name` TEXT UNIQUE
- `category` TEXT (optionnel)
- `created_at`

### 4.2 Contrats “bloc” (schema)

Chaque `block_type` doit publier un schema minimal :
- `configSchema` (JSON schema / zod) pour valider `profile_blocks.config`
- `dataNeeds` (optionnel) : quels ids/permissions sont nécessaires
- `visibilityRules` : que peut-on afficher publiquement

---

## 5) Catalogue de blocs (minimum)

Blocs “core” (capability Ma Page) :
- `rich_text` (présentation)
- `gallery` (photos/vidéos)
- `opening_hours` (horaires, exceptions)
- `contact_card` (tel/email/links)
- `tags_cloud`
- `links_list`

Blocs “connectés” (fournis par modules) :
- `agenda_booking` (réservation / créneaux / prochain RDV)
- `mini_shop` (catalogue léger, CTA)
- `module_rpg_stats` (stats, badges)
- `module_documents_public` (docs publics)
- `module_events` (participations, programme)

---

## 6) Policies / RLS (modèle recommandé)

### Principes

1) **Le propriétaire** peut gérer sa page selon son rôle et les règles (certains blocs peuvent être “admin-only”).
2) **Public** : lecture uniquement des pages publiées et des blocs autorisés en public.
3) **Admin/SuperAdmin** : gestion globale (modération, assignation, templates).

### Règles types

- `profile_pages`
  - `SELECT` : public si `visibility='public'` et `published_at is not null` ; sinon propriétaire ; sinon admin.
  - `UPDATE` : propriétaire (sur certains champs) ; admin/super_admin (tout).
- `profile_blocks`
  - `SELECT` : même logique que page + `is_enabled=true` + filtrage des blocs non-publics.
  - `UPDATE/INSERT/DELETE` : propriétaire si autorisé par type de profil ; sinon admin/super_admin.
- `profile_types` / `profile_type_rules`
  - gestion : admin/super_admin uniquement.

> Important : les blocs “connectés” doivent respecter la RLS du module source.  
> Exemple : `agenda_booking` n’affiche rien si l’utilisateur n’a pas accès aux données agenda liées.

---

## 7) Standards de couplage module → Ma Page

### 7.1 Déclaration (manifest)

Un module peut déclarer :
- un ou plusieurs `block_type`,
- des presets (blocs par défaut) pour certains `profile_types`,
- des handlers d’events pour rafraîchir des données.

### 7.2 Mécanique

- **Ma Page** fournit le cadre : stockage + rendu d’un arbre de blocs.
- **Le module** fournit : le bloc (UI + data resolver) et les events.
- L’assemblage est “léger” : `manifest + block registry + events`.

---

## Prochaines étapes

1) Définir une nomenclature officielle des rôles et tags (liste de référence).
2) Valider le data model (tables + policies) et les visibilités public/privé.
3) Implémenter un “Block Registry” minimal + 4 blocs core (texte, galerie, horaires, tags).
4) Connecter un premier bloc module : `agenda_booking`.

