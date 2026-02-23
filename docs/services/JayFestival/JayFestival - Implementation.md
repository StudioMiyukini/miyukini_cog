# JayFestival — Documentation de l’implémentation

## Contexte

Ce document décrit l’**implémentation actuelle** du service JayFestival : architecture, structure du code, modules, flux de données et points d’entrée. Il sert de référence pour les développeurs et complète le [Plan d’implémentation](./JayFestival%20-%20Plan%20Implementation.md) (plan de tâches) et les documents [Bornage](./JayFestival%20-%20Bornage%20Implementation.md), [Connexions et synchronisation](./reference/JayFestival%20-%20Connexions%20Synchronisation%20Services%20Jay.md).

**Références** : [Document fondateur](./JayFestival%20-%20Document%20Fondateur.md), [Audit complet 2026-02](./JayFestival%20-%20Audit%20Complet%202026-02.md).

---

## 1. Architecture globale

### 1.1 Répartition des responsabilités

```
┌────────────────────────────────────────────────────────────────────────────┐
│  Miyukini Central (apps/central)                                           │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │  ServiceConnections : jayfestival (Arc<JayFestivalDb>)                 │ │
│  │  ActiveServiceView → JayFestivalView (si tab "jayfestival")            │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │  services/jayfestival/  — Vue Dioxus complète (UNC, ORG, EXP, VIS)     │ │
│  │  → Lit conns.read().jayfestival (JayFestivalDb)                        │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────────────────────┘
                                        │
                                        │ utilise
                                        ▼
┌────────────────────────────────────────────────────────────────────────────┐
│  Crate jayfestival (crates/jayfestival)                                     │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────────────────┐ │
│  │  data/      │  │  auth/      │  │  services/ (adapters)                │ │
│  │  JayFestivalDb│ │  permissions │  │  jayxpose, jaykoa, jaykonta,         │ │
│  │  types      │  │  sign_in    │  │  miyunotify, miyubooking, miyuclock   │ │
│  └─────────────┘  └─────────────┘  └─────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────────────────────┘
```

| Composant | Emplacement | Rôle |
|-----------|-------------|------|
| **Crate jayfestival** | `crates/jayfestival/` | Données (KindMother SQLite), auth, types, adapters inter-services. Pas d’UI (migration Tauri/React). |
| **Vue JayFestival** | `apps/central/src/services/jayfestival/` | UI Dioxus complète : parcours UNC, ORG, EXP, VIS. |
| **Connexions** | `apps/central/src/data.rs` | `JayFestivalDb::open(jayfestival.db)` ; `ServiceConnections.jayfestival`. |

---

## 2. Crate jayfestival — structure des modules

### 2.1 Arborescence

```
crates/jayfestival/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Point d'entrée : data, auth, services
│   ├── data/
│   │   ├── mod.rs          # Feature flags ; re-exports types, JayFestivalDb
│   │   ├── types.rs        # Profile, UserType, Edition, Organisateur, Exposant, EditionExposant, Animation, BudgetEntry
│   │   ├── kindmother_db.rs       # [legacy-sqlite] impl SQLite directe
│   │   └── kindmother_client_db.rs # [kindmother-only] via KindMother Client
│   ├── auth/
│   │   ├── mod.rs          # sign_in, sign_up, sign_out (profiles table)
│   │   └── permissions.rs # user_type_from_profile, can_access_edition
│   └── services/
│       ├── mod.rs
│       ├── jayxpose/       # Client lecture profil/répertoire ; types contrat
│       │   ├── mod.rs
│       │   ├── client.rs   # jayxpose_list_repertoire, jayxpose_fiche_by_id
│       │   └── contract.rs # JayXposeProfile, RepertoireItem, RepertoireFilters
│       ├── jaykoa/         # Adapter stub (jaykoa_publish_edition → Err)
│       │   ├── mod.rs
│       │   └── adapter.rs
│       ├── jaykonta/       # Adapter Miyuinvoice (quote, invoice)
│       │   ├── mod.rs
│       │   └── adapter.rs
│       ├── miyunotify/     # Adapter
│       ├── miyubooking/    # Adapter
│       └── miyuclock/      # Adapter
└── tests/
    ├── parcours_unc.rs     # Tests navigation UNC (cassés : app_state/screens supprimés)
    ├── parcours_org.rs
    ├── parcours_exp.rs
    ├── parcours_vis.rs
    └── global_router.rs
```

### 2.2 Module `data`

| Élément | Fichier | Description |
|---------|---------|-------------|
| **JayFestivalDb** | kindmother_db.rs | Base SQLite KindMother Daughter. Tables : profiles, editions, organisateurs, exposants, editions_exposants, animations, budget_entries |
| **Feature** | Cargo.toml | `legacy-sqlite` (défaut) ou `kindmother-only` |
| **Méthodes CRUD** | kindmother_db.rs | `editions_list`, `edition_by_id`, `organisateurs_list`, `exposants_list`, `exposant_by_id`, `editions_exposants_*`, `animations_*`, `budget_*`, `profile_*`, `candidatures_*`, etc. |

### 2.3 Module `auth`

| Élément | Fichier | Description |
|---------|---------|-------------|
| **auth_sign_in** | auth/mod.rs | `profile_by_email_password` ; hash SHA256 |
| **auth_sign_up** | auth/mod.rs | `profile_create` avec user_type |
| **auth_user_type_from_profile** | permissions.rs | Mapping `user_type` → `UserType` |
| **auth_can_access_edition** | permissions.rs | Admin toujours ; Manager si is_manager |

### 2.4 Module `services` (adapters)

| Service | Fichier | État |
|---------|---------|------|
| **JayXpose** | jayxpose/client.rs | Lit `JayFestivalDb.exposants_list`, `exposant_by_id` ; mappe vers `JayXposeProfile`, `RepertoireItem` |
| **JayKoa** | jaykoa/adapter.rs | Stub : `jaykoa_publish_edition` → Err ; `jaykoa_get_conflicts` → Vec vide |
| **JayKonta** | jaykonta/adapter.rs | Appels Miyuinvoice `quote::create`, `invoice::create` |
| **Miyunotify** | miyunotify/adapter.rs | Adapter présent |
| **Miyubooking** | miyubooking/adapter.rs | Adapter présent |
| **MiyuClock** | miyuclock/adapter.rs | Adapter présent |

---

## 3. Vue JayFestival (apps/central)

### 3.1 Arborescence UI

```
apps/central/src/services/jayfestival/
├── mod.rs           # JayFestivalView, JayFestivalState, UncSection, OrgSection, ExpSection, VisSection
├── sidebar.rs       # JayFestivalSidebar — rôle + navigation
├── components.rs    # StatCard, Badge, TabButton, EmptyState
├── unc_landing.rs   # UNC-E01 Landing
├── unc_events.rs    # UncEventsList, UncEventDetail
├── unc_directory.rs # UncOrganisateursList, UncOrganisateurDetail, UncExposantsList, UncExposantDetail
├── unc_search.rs
├── unc_auth.rs      # UncConnexion, UncInscription, UncMentionsLegales
├── org_dashboard.rs
├── org_editions.rs
├── org_edition_hub.rs
├── org_exposants.rs
├── org_programme.rs
├── org_plan.rs
├── org_budget.rs
├── org_parametres.rs
├── org_documents.rs
├── org_annonces.rs
├── org_services.rs
├── org_publication.rs
├── org_compte.rs
├── org_equipe.rs
├── exp_dashboard.rs
├── exp_candidatures.rs
├── exp_participations.rs
├── exp_agenda.rs
├── exp_documents.rs
├── exp_factures.rs
├── exp_fiche_publique.rs
├── exp_compte.rs
├── exp_notifications.rs
├── vis_dashboard.rs
├── vis_catalogue.rs
├── vis_agenda.rs
├── vis_billets.rs
├── vis_reservations.rs
├── vis_activites.rs
└── vis_compte.rs
```

### 3.2 État local (JayFestivalState)

| Champ | Type | Rôle |
|-------|------|------|
| `is_connected` | bool | Utilisateur connecté ou non |
| `current_user_id` | Option<String> | ID profil |
| `unc_section` | UncSection | Section UNC active (Landing, Events, Organisateurs, etc.) |
| `role` | JayFestivalRole | Organisateur \| Exposant \| Visiteur |
| `org_section` | OrgSection | Dashboard, Editions, EditionHub, Compte, Equipe |
| `selected_edition_id` | Option<String> | Édition sélectionnée |
| `edition_tab` | OrgEditionTab | Overview, Parametres, Exposants, Programme, Budget, Plan, etc. |
| `exp_section` | ExpSection | Dashboard, Candidatures, Participations, Agenda, etc. |
| `vis_section` | VisSection | Dashboard, Catalogue, Agenda, Billets, Reservations, Activites, Compte |

### 3.3 Accès aux données

Tous les composants utilisent :

```rust
let conns = use_service_connections();
let db = &conns.read().jayfestival;
// ex: db.editions_list(), db.exposants_list(true), db.edition_by_id(&id)
```

### 3.4 Points d’entrée

| Entrée | Fichier | Condition |
|--------|---------|-----------|
| **Onglet JayFestival** | `services/mod.rs` | `tab.service_id == "jayfestival"` → `JayFestivalView {}` |
| **Facade UNC** | jayfestival/mod.rs | `!state.read().is_connected` → `UncFacade` |
| **Espace connecté** | jayfestival/mod.rs | `state.read().role` → OrgSection / ExpSection / VisSection |

---

## 4. Flux de données

### 4.1 Initialisation

```
AppContext::new()
  → ServiceConnections::open(base_path)
    → JayFestivalDb::open(base_path.join("jayfestival.db"))
  → JayFestivalView
    → use_service_connections()
    → conns.read().jayfestival
```

### 4.2 Connexion / inscription

| Action | Fichier | Appel |
|--------|---------|-------|
| Connexion | unc_auth.rs | `db.profile_by_email_password(email, password)` |
| Inscription | unc_auth.rs | `db.profile_create(email, password, user_type)` |

### 4.3 CRUD éditions, exposants, candidatures

| Entité | Opérations | Fichier DB |
|--------|------------|------------|
| Éditions | list, by_id, create, update | org_editions, org_edition_hub, unc_events |
| Exposants | list, by_id | unc_directory, org_exposants |
| Candidatures | pending_count, update status | org_exposants, exp_candidatures |
| Participations | editions_exposants | exp_participations |
| Programme | animations | org_programme |
| Budget | budget_entries | org_budget |
| Plan | stands (si implémenté) | org_plan |

---

## 5. Synchronisation (Central, hors crate jayfestival)

### 5.1 JayFestival → JayKoa

| Composant | Emplacement | Description |
|-----------|-------------|-------------|
| **JayFestivalSync** | `apps/central/src/services/jaykoa/sync_service.rs` | `sync_all()`, `sync_single_edition()` : lit `festival_db.editions_list()` ou une édition, crée reflets `TemporalEntry` dans JayKoa |
| **Bouton sync** | `apps/central/src/services/jaykoa/mod.rs` | `on_sync_jayfestival` — appelle `JayFestivalSync::sync_all()` (sync réelle) |
| **Bouton « Ajouter à mon agenda »** | `vis_catalogue.rs`, `unc_events.rs` (UncEventDetail) | Appelle `sync_single_edition()` pour une édition |

### 5.2 Annuaire exposants (JayXpose)

L’annuaire lit la table `exposants` de **JayFestivalDb** via `exposants_list(visible_repertoire)` et `exposant_by_id()`. Le client JayXpose du crate jayfestival utilise ces données et les mappe vers `JayXposeProfile` / `RepertoireItem`. Pas de lecture directe de JayXposeDb en alpha.

---

## 6. Tests

### 6.1 État actuel

| Fichier | Tests | État |
|---------|-------|------|
| parcours_unc.rs | 15 | **Cassés** — importe `jayfestival::app_state::AppState`, `jayfestival::screens::ScreenId` (modules supprimés) |
| parcours_org.rs | 14 | Cassés |
| parcours_exp.rs | 7 | Cassés |
| parcours_vis.rs | 5 | Cassés |
| global_router.rs | 8 | Cassé |

**Cause** : Migration Tauri/React a retiré `app_state` et `screens` du crate jayfestival. Les tests n’ont pas été adaptés.

### 6.2 Tests unitaires (modules actuels)

Aucun `#[test]` dans les modules `data`, `auth`, `services` du crate jayfestival. Les tests précédents (types, permissions) référençaient des modules supprimés.

---

## 7. Dépendances Cargo

```toml
# crates/jayfestival/Cargo.toml
[dependencies]
kindmother = { path = "../kindmother" }
kindmother-client = { path = "../kindmother-client" }
miyubooking = { path = "../miyubooking" }
miyuinvoice = { path = "../miyuinvoice" }
miyunotify = { path = "../miyunotify" }
miyuclock = { path = "../miyuclock" }
rusqlite = { version = "0.32", optional = true }
# Pas de jaykoa, jayxpose, jaykonta — orchestration dans Central
```

---

## 8. Références

- [JayFestival - Plan Implementation](./JayFestival%20-%20Plan%20Implementation.md) — plan de tâches
- [JayFestival - Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md) — périmètre
- [JayFestival - Connexions Synchronisation Services Jay](./reference/JayFestival%20-%20Connexions%20Synchronisation%20Services%20Jay.md) — liaisons, sync
- [JayFestival - Audit Complet 2026-02](./JayFestival%20-%20Audit%20Complet%202026-02.md) — métriques, pistes d’amélioration

---

**Document** : JayFestival — Documentation de l’implémentation  
**Version** : 1.0  
**Date** : 2026-02-22  
**Statut** : Document de référence (état actuel du code)
