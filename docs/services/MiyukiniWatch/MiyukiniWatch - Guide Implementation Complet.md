# MiyukiniWatch â€” Guide d'ImplÃ©mentation Complet

## Contexte

Ce document constitue le **guide d'implÃ©mentation le plus exhaustif possible** du Service MiyukiniWatch. Il synthÃ©tise et dÃ©taille les spÃ©cifications contenues dans les huit documents fondateurs et fournit des instructions Ã©tape par Ã©tape pour rÃ©aliser une implÃ©mentation conforme.

## PortÃ©e / Scope

- **Applicable Ã  :** Toutes les phases de dÃ©veloppement de MiyukiniWatch (crates, Central, Miou).
- **Audience :** DÃ©veloppeurs, architectes, Ã©quipes produit.
- **Statut :** Guide normatif â€” rÃ©fÃ©rence d'implÃ©mentation.

---

## Table des matiÃ¨res

1. [Vue d'ensemble de l'implÃ©mentation](#1-vue-densemble-de-limplementation)
2. [PrÃ©requis et dÃ©pendances](#2-prÃ©requis-et-dÃ©pendances)
3. [Phase 1 â€” Infrastructure et persistance](#3-phase-1--infrastructure-et-persistance)
4. [Phase 2 â€” OpÃ©rateur Collector](#4-phase-2--opÃ©rateur-collector)
5. [Phase 3 â€” OpÃ©rateur Aggregator](#5-phase-3--opÃ©rateur-aggregator)
6. [Phase 4 â€” OpÃ©rateur Presenter et interface](#6-phase-4--opÃ©rateur-presenter-et-interface)
7. [Phase 5 â€” IntÃ©gration Central](#7-phase-5--intÃ©gration-central)
8. [Phase 6 â€” IntÃ©gration Miou](#8-phase-6--intÃ©gration-miou)
9. [Phase 7 â€” SÃ©curitÃ©, conformitÃ© et audit](#9-phase-7--sÃ©curitÃ©-conformitÃ©-et-audit)
10. [Matrices de vÃ©rification et checklist](#10-matrices-de-vÃ©rification-et-checklist)

---

## 1. Vue d'ensemble de l'implÃ©mentation

### 1.1 Composants Ã  crÃ©er

| Composant | Type | Emplacement | RÃ´le |
|-----------|------|-------------|------|
| **miyukiniwatch** | Crate Service (Strate 7) | `crates/miyukiniwatch/` | Logique mÃ©tier, mÃ©triques, agrÃ©gats, persistance |
| **MiyukiniWatchCollector** | OpÃ©rateur | Dans `miyukiniwatch` | Collecte passive des mÃ©triques |
| **MiyukiniWatchAggregator** | OpÃ©rateur | Dans `miyukiniwatch` | AgrÃ©gation pÃ©riodique |
| **MiyukiniWatchPresenter** | OpÃ©rateur | Dans `miyukiniwatch` + Central UI | Consultation, effacement, paramÃ¨tres |
| **Vue MiyukiniWatch** | Service Central | `apps/central/src/services/miyukiniwatch/` | Interface utilisateur (4 Ã©crans) |
| **Connexion DB** | Data layer | `apps/central/src/data.rs` | `MiyukiniWatchDb` dans `ServiceConnections` |

### 1.2 Ordre de dÃ©pendances

```
KindMother (existant)
       â”‚
       â–¼
miyukiniwatch (crate) â€” persistance, opÃ©rateurs
       â”‚
       â”œâ”€â”€ Miyukini Central (service view, intÃ©gration)
       â”‚
       â””â”€â”€ Miou (consommation agrÃ©gats)
```

### 1.3 Principes d'implÃ©mentation impÃ©ratifs

| # | Principe | VÃ©rification |
|---|----------|---------------|
| P1 | **Jamais de lecture de contenus** | Aucun accÃ¨s aux champs texte, messages, fichiers, DOM |
| P2 | **DonnÃ©es locales uniquement** | Aucune dÃ©pendance rÃ©seau ; `unsafe_code = "forbid"` |
| P3 | **Gouvernance par les Cores** | Toutes les opÃ©rations via BondingBrother â†’ StrongFather, KindMother, Master Butler |
| P4 | **Collecte asynchrone et non bloquante** | WriteIntent en tÃ¢che de fond ; prioritÃ© basse |
| P5 | **AgrÃ©gats pour Miou, pas donnÃ©es brutes** | Miou ne consomme que des structures prÃ©-calculÃ©es |

---

## 2. PrÃ©requis et dÃ©pendances

### 2.1 Crates existants requis

```toml
# Dans crates/miyukiniwatch/Cargo.toml
[dependencies]
# Persistance
kindmother = { path = "../kindmother" }
kindmother_db_key = { path = "../kindmother-db-key" }
rusqlite = "0.32"

# Utilitaires
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
thiserror = "2"
```

### 2.2 Aucune dÃ©pendance rÃ©seau

VÃ©rifier que **ni** `hyper`, **ni** `reqwest`, **ni** `tokio::net` ne sont importÃ©s. ConformitÃ© C-05, C-08.

### 2.3 Lints obligatoires

```toml
[lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"
```

---

## 3. Phase 1 â€” Infrastructure et persistance

### 3.1 Structure du crate miyukiniwatch

```
crates/miyukiniwatch/
â”œâ”€â”€ Cargo.toml
â””â”€â”€ src/
    â”œâ”€â”€ lib.rs
    â”œâ”€â”€ errors.rs
    â”œâ”€â”€ types.rs           # MÃ©triques, agrÃ©gats, structures
    â”œâ”€â”€ schema.sql         # SchÃ©ma SQL
    â”œâ”€â”€ db.rs              # MiyukiniWatchDb (KindMother Daughter)
    â”œâ”€â”€ operator/
    â”‚   â”œâ”€â”€ mod.rs
    â”‚   â”œâ”€â”€ collector.rs
    â”‚   â”œâ”€â”€ aggregator.rs
    â”‚   â””â”€â”€ presenter.rs
    â””â”€â”€ aggregates.rs      # Calcul des agrÃ©gats exposÃ©s Ã  Miou
```

### 3.2 SchÃ©ma SQL (KindMother)

CrÃ©er `crates/miyukiniwatch/src/schema.sql` :

```sql
-- MÃ©triques brutes (rÃ©tention 7-90 j. par dÃ©faut 30)
CREATE TABLE IF NOT EXISTS miyukiniwatch_metrics (
    record_id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    metric_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    service_id TEXT,
    friend_id TEXT,
    value_int INTEGER,
    value_str TEXT,
    UNIQUE(session_id, metric_id, timestamp)
);

CREATE INDEX IF NOT EXISTS idx_mw_metrics_profile_ts
    ON miyukiniwatch_metrics(profile_id, timestamp);
CREATE INDEX IF NOT EXISTS idx_mw_metrics_metric
    ON miyukiniwatch_metrics(metric_id);

-- AgrÃ©gats quotidiens (rÃ©tention 30-365 j. par dÃ©faut 90)
CREATE TABLE IF NOT EXISTS miyukiniwatch_daily (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    profile_id TEXT NOT NULL,
    date TEXT NOT NULL,
    metric_id TEXT NOT NULL,
    service_id TEXT,
    friend_id TEXT,
    count INTEGER NOT NULL,
    total_value INTEGER NOT NULL,
    min_value INTEGER,
    max_value INTEGER,
    UNIQUE(profile_id, date, metric_id, service_id, friend_id)
);

-- AgrÃ©gats hebdomadaires (rÃ©tention 90-730 j. par dÃ©faut 365)
CREATE TABLE IF NOT EXISTS miyukiniwatch_weekly (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    profile_id TEXT NOT NULL,
    year INTEGER NOT NULL,
    week INTEGER NOT NULL,
    metric_id TEXT NOT NULL,
    service_id TEXT,
    friend_id TEXT,
    count INTEGER NOT NULL,
    total_value INTEGER NOT NULL,
    avg_value REAL,
    UNIQUE(profile_id, year, week, metric_id, service_id, friend_id)
);

-- Compteurs globaux (effaÃ§ables manuellement)
CREATE TABLE IF NOT EXISTS miyukiniwatch_globals (
    profile_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value INTEGER NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (profile_id, key)
);

-- Journal d'audit (rÃ©tention min 90 j., dÃ©faut 365)
CREATE TABLE IF NOT EXISTS miyukiniwatch_audit (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    profile_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    event_type TEXT NOT NULL,
    details TEXT,
    records_affected INTEGER
);

-- PrÃ©fÃ©rences (collecte, rÃ©tention, catÃ©gories)
CREATE TABLE IF NOT EXISTS miyukiniwatch_prefs (
    profile_id TEXT PRIMARY KEY,
    collect_enabled INTEGER NOT NULL DEFAULT 1,
    collect_sessions INTEGER NOT NULL DEFAULT 1,
    collect_services INTEGER NOT NULL DEFAULT 1,
    collect_friends INTEGER NOT NULL DEFAULT 1,
    collect_activity INTEGER NOT NULL DEFAULT 1,
    retention_raw_days INTEGER NOT NULL DEFAULT 30,
    retention_daily_days INTEGER NOT NULL DEFAULT 90,
    retention_weekly_days INTEGER NOT NULL DEFAULT 365,
    updated_at TEXT NOT NULL
);
```

### 3.3 Identifiants de mÃ©triques (rÃ©fÃ©rence SF MÃ©triques)

| ID | Nom | Table |
|----|-----|-------|
| S-01 | DÃ©but session | `metric_id = "S-01"` |
| S-02 | Fin session | S-02 |
| S-03 | DurÃ©e session | S-03 |
| S-04 | Tranche horaire | S-04 |
| S-05 | Jours depuis derniÃ¨re session | S-05 (global) |
| S-06 | Compteur sessions | S-06 (global) |
| S-07 | Jours actifs consÃ©cutifs | S-07 (global) |
| SV-01 | Service ouvert | SV-01 |
| SV-02 | Service fermÃ© | SV-02 |
| SV-03 | Temps par service | SV-03 |
| SV-07 | Onglet principal | SV-07 |
| A-01 | Ami contactÃ© | A-01 |
| A-03 | DurÃ©e discussion ami | A-03 |
| I-01 | Clics (global) | I-01 |
| I-02 | Clics par service | I-02 |
| L-01 | Rite d'EntrÃ©e | L-01 |
| L-02 | Type d'entrÃ©e | L-02 |
| L-03 | Premier service installÃ© | L-03 |
| L-04 | Connexion MWS | L-04 |

### 3.4 Tranches horaires

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeSlot {
    Morning,    // 06:00 â€“ 11:59
    Afternoon,  // 12:00 â€“ 17:59
    Evening,    // 18:00 â€“ 22:59
    Night,      // 23:00 â€“ 05:59
}
```

### 3.5 ImplÃ©mentation MiyukiniWatchDb

- Suivre le pattern `JayXposeDb` : SQLite chiffrÃ© via `kindmother_db_key`, instance KindMother Daughter.
- MÃ©thodes minimales : `open`, `init_schema`, `insert_metric`, `insert_audit`, `get_prefs`, `set_prefs`.
- Isolation par `profile_id` sur toutes les requÃªtes.

---

## 4. Phase 2 â€” OpÃ©rateur Collector

### 4.1 ResponsabilitÃ©

Le Collector **Ã©coute passivement** les Ã©vÃ©nements du COG. Il ne crÃ©e jamais d'Ã©vÃ©nement. Il ne modifie pas le comportement des autres services.

### 4.2 Ã‰vÃ©nements Ã  consommer

| Source | Ã‰vÃ©nement | MÃ©triques produites |
|--------|-----------|---------------------|
| Central â€” Auth | `user_logged_in` | S-01, S-04, L-02 |
| Central â€” Auth | `user_logged_out` | S-02, S-03 |
| Central â€” Navigation | `service_opened` | SV-01 |
| Central â€” Navigation | `service_closed` | SV-02, SV-03 |
| Central â€” Navigation | `tab_changed` | SV-07 |
| Central â€” Rite | `rite_completed` | L-01 |
| Jay1Tribu / sociaux | `conversation_opened` | A-01 |
| Jay1Tribu / sociaux | `conversation_closed` | A-03 ( durÃ©e ) |
| Central â€” UI | `user_click` | I-01, I-02 |
| MWS Participant | `webway_connected` | L-04 |
| Central â€” Services | `service_installed` | L-03 |

### 4.3 MÃ©canisme de bus d'Ã©vÃ©nements

- **Option A (si existant) :** S'abonner au bus d'Ã©vÃ©nements Central (ex. `EventBus`, `watch()`).
- **Option B (MVP) :** Le Collector reÃ§oit des appels directs depuis Central lors des transitions (login, logout, ouverture service, etc.).

### 4.4 RÃ¨gles de collecte

| RÃ¨gle | ImplÃ©mentation |
|-------|----------------|
| PassivitÃ© | Aucun `emit` ou `publish` vers d'autres services |
| AtomicitÃ© | Un `insert_metric` par WriteIntent ; Ã©chec isolÃ© |
| Idempotence | ClÃ© unique `(session_id, metric_id, timestamp)` ; `INSERT OR IGNORE` |
| PrioritÃ© basse | ExÃ©cuter en `spawn_blocking` ou tÃ¢che basse prioritÃ© |
| Aucun blocage | Jamais d'`await` bloquant sur l'UI |

### 4.5 VÃ©rification prÃ©-collecte

Avant d'Ã©crire toute mÃ©trique :

1. VÃ©rifier `collect_enabled` dans les prÃ©fÃ©rences.
2. VÃ©rifier la catÃ©gorie (sessions, services, amis, activitÃ©) si dÃ©sactivÃ©e individuellement.
3. VÃ©rifier l'Ã©tat de confiance (Caring Nanny) : en T2, ne collecter que sessions + services ; en T3, ne pas collecter.

### 4.6 Limites de volumÃ©trie

- Max 10 000 mÃ©triques brutes par session (configurable via TAMR).
- Si dÃ©passement : arrÃªter la collecte pour la session, ne pas crasher.

---

## 5. Phase 3 â€” OpÃ©rateur Aggregator

### 5.1 ResponsabilitÃ©

AgrÃ©ger les mÃ©triques brutes en rÃ©sumÃ©s quotidiens et hebdomadaires. Produire les agrÃ©gats exposÃ©s Ã  Miou. ExÃ©cuter la purge automatique.

### 5.2 DÃ©clencheurs d'agrÃ©gation

| DÃ©clencheur | Action |
|-------------|--------|
| Fin de session | AgrÃ©gation des mÃ©triques de la session en cours |
| PremiÃ¨re session du jour | AgrÃ©gation quotidienne de la veille ; purge mÃ©triques brutes expirÃ©es |
| PremiÃ¨re session de la semaine | AgrÃ©gation hebdomadaire ; purge agrÃ©gats quotidiens expirÃ©s |
| PremiÃ¨re session du mois | Purge agrÃ©gats hebdomadaires expirÃ©s |
| Ã€ la demande (Miou, UI) | Calcul des agrÃ©gats en mÃ©moire pour la pÃ©riode demandÃ©e |

### 5.3 Catalogue des agrÃ©gats exposÃ©s (contrat Miou)

ImplÃ©menter les structures et calculs pour :

| Identifiant | Contenu |
|-------------|---------|
| `AGG_SESSION_SUMMARY` | `days_since_last_session`, `avg_duration_seconds`, `usual_time_slot`, `total_sessions`, `consecutive_active_days` |
| `AGG_SESSION_RETURN` | `is_returning`, `days_away` |
| `AGG_SESSION_TIME` | `current_time_slot` |
| `AGG_TOP_SERVICES` | Liste top 5 `(service_id, open_count, total_duration)` sur 7 jours |
| `AGG_NEGLECTED_SERVICES` | Services non ouverts depuis > 14 jours |
| `AGG_FAVORITE_SERVICE` | Service le plus frÃ©quentÃ© sur 7 jours |
| `AGG_FAVORITE_TAB` | Salon / BibliothÃ¨que / Webway le plus utilisÃ© |
| `AGG_FRIEND_REMINDERS` | Amis non contactÃ©s depuis > 7 jours |
| `AGG_TOP_FRIENDS` | Top 3 par durÃ©e sur 30 jours |
| `AGG_SOCIAL_ACTIVITY` | `distinct_friends_contacted_week`, `total_social_time_week` |
| `AGG_ACTIVITY_LEVEL` | `level`, `sessions_week`, `total_duration_week` |
| `AGG_CURRENT_SESSION` | `duration_current_session`, `services_opened_count` |
| `AGG_MILESTONES` | Liste de jalons (streaks, badges) |
| `AGG_NEW_MILESTONE` | Jalon atteint dans la session en cours (optionnel) |

### 5.4 Purge automatique

1. Lire les prÃ©fÃ©rences de rÃ©tention (`retention_raw_days`, etc.).
2. Identifier les enregistrements expirÃ©s.
3. Soumettre un PurgeIntent (conceptuel ; en pratique, exÃ©cuter des `DELETE` via KindMother).
4. Enregistrer un Ã©vÃ©nement d'audit : `event_type = "purge_automatic"`, `records_affected`, `details`.

### 5.5 Cascade d'agrÃ©gation

- **Ordre impÃ©ratif :** D'abord calculer/Ã©crire les agrÃ©gats de niveau N, puis purger le niveau N+1.
- Exemple : avant de purger les mÃ©triques brutes de plus de 30 jours, s'assurer que les agrÃ©gats quotidiens correspondants existent.

---

## 6. Phase 4 â€” OpÃ©rateur Presenter et interface

### 6.1 ResponsabilitÃ© du Presenter

- Lire les agrÃ©gats et mÃ©triques pour affichage.
- ExÃ©cuter les DeleteIntent (effacement) sur demande utilisateur.
- GÃ©rer les prÃ©fÃ©rences (activation/dÃ©sactivation, rÃ©tention).

### 6.2 Quatre Ã©crans Ã  implÃ©menter

#### Ã‰cran 1 â€” Tableau de bord

| Zone | Composants | DonnÃ©es |
|------|------------|---------|
| En-tÃªte | Titre + phrase d'explication | Statique |
| Bloc pÃ©rimÃ¨tre | 4 dimensions (Quand, OÃ¹, Qui, Combien) avec âœ“/âœ— | Depuis prÃ©fÃ©rences |
| Carte Sessions | DerniÃ¨re session, jours depuis visite, tranche habituelle | `AGG_SESSION_SUMMARY`, `AGG_SESSION_RETURN` |
| Carte Services | Top 3, dernier ouvert, plus long | `AGG_TOP_SERVICES`, `AGG_FAVORITE_SERVICE` |
| Carte Amis | ContactÃ©s rÃ©cemment, non contactÃ© le plus longtemps | `AGG_FRIEND_REMINDERS`, `AGG_TOP_FRIENDS` |
| Carte ActivitÃ© | Clics/jour, sessions/semaine, streak | `AGG_ACTIVITY_LEVEL`, `AGG_MILESTONES` |
| Pied | Liens DÃ©tail, ParamÃ¨tres, Effacer tout | Navigation |

Ã‰tats spÃ©ciaux : premiÃ¨re utilisation, collecte dÃ©sactivÃ©e, donnÃ©es effacÃ©es.

#### Ã‰cran 2 â€” DÃ©tail des mÃ©triques

- Onglets : Sessions, Services, Amis, ActivitÃ©.
- Filtres : Aujourd'hui, Cette semaine, Ce mois, PersonnalisÃ©.
- Tableaux triables, graphiques (barres, circulaire).
- Rappel de transparence en bas de chaque onglet.

#### Ã‰cran 3 â€” ParamÃ¨tres et vie privÃ©e

- Toggle collecte globale.
- Toggles par catÃ©gorie (Sessions, Services, Amis, ActivitÃ©).
- Sliders rÃ©tention : brut (7â€“90 j.), quotidien (30â€“365 j.), hebdo (90â€“730 j.).
- Indicateur espace utilisÃ©.
- Boutons effacement : derniÃ¨re semaine, dernier mois, tout, par catÃ©gorie.
- Modale de confirmation pour tout effacement.

#### Ã‰cran 4 â€” Historique des actions (audit)

- Journal chronologique (collecte on/off, effacements, purges, modifications rÃ©tention).
- Filtrage par type, pÃ©riode.
- DÃ©tail au clic.

### 6.3 Navigation locale

Barre d'onglets ou sidebar : [Tableau de bord] [DÃ©tail] [ParamÃ¨tres] [Audit].

### 6.4 Principes UX

- Tutoiement, bienveillance, clartÃ©.
- Responsive : desktop 2 colonnes, mobile colonne unique.
- AccessibilitÃ© : WCAG AA, navigation clavier, ARIA sur graphiques et toggles.

---

## 7. Phase 5 â€” IntÃ©gration Central

### 7.1 Enregistrement du service

Dans `apps/central/src/state.rs`, ajouter Ã  `default_services()` :

```rust
ServiceInfo {
    id: "miyukiniwatch".to_string(),
    name: "MiyukiniWatch".to_string(),
    description: "Tes habitudes et tes mesures â€” consulte, comprends, efface.".to_string(),
    icon: "ðŸ‘".to_string(),  // ou icÃ´ne thÃ©matique dÃ©finie
    service_type: ServiceType::InterneCog,
    is_installed: true,
    is_favorite: false,
    version: "0.1.0".to_string(),
    developer: "Miyukini".to_string(),
},
```

### 7.2 Routage de la vue

Dans `apps/central/src/services/mod.rs` :

- Ajouter `mod miyukiniwatch;`
- Exporter `pub use miyukiniwatch::MiyukiniWatchView;`

Dans `ActiveServiceView`, ajouter :

```rust
Some("miyukiniwatch") => rsx! { MiyukiniWatchView {} },
```

### 7.3 Connexion DB

Dans `apps/central/src/data.rs` :

- Ajouter `pub miyukiniwatch: Arc<MiyukiniWatchDb>` dans `ServiceConnections`.
- Dans `ServiceConnections::open()`, ouvrir `MiyukiniWatchDb::open(base_path.join("miyukiniwatch.db"))`.

### 7.4 Points d'injection Collector

Ã‰mettre (ou appeler le Collector) aux moments suivants :

| Moment | Emplacement probable | Ã‰vÃ©nement / Action |
|--------|----------------------|---------------------|
| Login rÃ©ussi | `Connexion` ou `auth` | `user_logged_in` |
| Logout / fermeture | Idem | `user_logged_out` |
| Ouverture service | `open_service` dans state | `service_opened` |
| Fermeture service / navigation | Navigation logic | `service_closed` |
| Changement d'onglet | `TabBar` | `tab_changed` |
| Clic utilisateur | Wrapper ou zone commune | `user_click` (agrÃ©gat uniquement) |
| Rite terminÃ© | `RiteEntree` | `rite_completed` |
| Service installÃ© | Logique d'installation | `service_installed` |

### 7.5 Badge Â« Collecte dÃ©sactivÃ©e Â»

Si `collect_enabled == false`, afficher un indicateur discret sur la carte du service dans la grille (ex. petite pastille grise).

---

## 8. Phase 6 â€” IntÃ©gration Miou

### 8.1 Interface d'accÃ¨s aux agrÃ©gats

CrÃ©er une faÃ§ade ou module exposÃ© Ã  Miou :

```rust
pub trait MiyukiniWatchAggregatesProvider {
    fn get_aggregate(&self, profile_id: &str, aggregate_id: &str) -> Option<Aggregate>;
    fn get_all_aggregates(&self, profile_id: &str) -> Vec<Aggregate>;
    fn has_data(&self, profile_id: &str) -> bool;
    fn is_collecting(&self, profile_id: &str) -> bool;
}
```

### 8.2 RÃ¨gles de consommation (cÃ´tÃ© Miou)

- Lecture seule : Miou ne modifie jamais MiyukiniWatch.
- DÃ©gradation gracieuse : si agrÃ©gat absent â†’ message gÃ©nÃ©rique.
- Pas de cache longue : recharger Ã  chaque session.
- RÃ©solution `friend_cog_id` â†’ pseudo : via le service de contacts (Jay1Tribu), pas MiyukiniWatch.

### 8.3 DÃ©rivÃ©es pour le Bot Miou

D'aprÃ¨s [Bot - IntÃ©gration et Flux de DonnÃ©es](../MiyukiniCentral/Miou/Bot/Bot%20-%20Integration%20et%20Flux%20de%20Donnees.md) :

| RequÃªte | Retour | Usage |
|---------|--------|-------|
| `get_session_summary(profile_id)` | SessionSummary | Jours absence, durÃ©e |
| `get_services_usage(profile_id, period)` | Vec<ServiceUsage> | Service dÃ©laissÃ©, top |
| `get_friends_status(profile_id)` | Vec<FriendStatus> | Ami le plus dÃ©laissÃ© |
| `get_clicks_count(profile_id, period)` | u32 | Indicateur activitÃ© |

### 8.4 Matrice agrÃ©gat â†’ bulle

Voir [MiyukiniWatch - IntÃ©gration Miou et AgrÃ©gats](./MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md) section 4.1 pour les exemples de bulles par agrÃ©gat.

---

## 9. Phase 7 â€” SÃ©curitÃ©, conformitÃ© et audit

### 9.1 Classification des donnÃ©es

- MÃ©triques sessions/services : niveau 1 (Standard).
- MÃ©triques amis : niveau 2 (Sensitive).
- Chiffrement au repos : via KindMother / `kindmother_db_key` (dÃ©jÃ  en place pour les DB filles).

### 9.2 Matrice d'accÃ¨s

Respecter la matrice [SÃ©curitÃ© et ConformitÃ©](./MiyukiniWatch%20-%20Securite%20et%20Conformite.md) section 3.1 :

- Utilisateur : lecture, effacement, config â€” pas d'Ã©criture directe.
- Collector : Ã©criture uniquement.
- Aggregator : lecture + Ã©criture agrÃ©gats + purge.
- Presenter : lecture + effacement (DeleteIntent).
- Miou : lecture agrÃ©gats uniquement.

### 9.3 Ã‰tats de confiance (T0â€“T4)

Adapter le Collector selon l'Ã©tat Caring Nanny :

- T0 : collecte complÃ¨te.
- T1 : collecte normale, agrÃ©gation peut Ãªtre retardÃ©e.
- T2 : collecte rÃ©duite (sessions + services uniquement).
- T3 : collecte suspendue.
- T4 : MiyukiniWatch inaccessible.

### 9.4 Journal d'audit

Enregistrer tous les Ã©vÃ©nements listÃ©s dans [Gouvernance DonnÃ©es et RÃ©tention](./MiyukiniWatch%20-%20Gouvernance%20Donnees%20et%20Retention.md) section 6.1. RÃ©tention du journal : min 90 j., dÃ©faut 365 j.

### 9.5 RÃ¨gles de code

- `unsafe_code = "forbid"`.
- Pas de crate rÃ©seau.
- Pas d'accÃ¨s fichier direct (tout via KindMother/MiyukiniWatchDb).

---

## 10. Matrices de vÃ©rification et checklist

### 10.1 Matrice des contraintes (extrait Contraintes et Invariants)

| VÃ©rification | CritÃ¨re | OK ? |
|--------------|---------|------|
| MiyukiniWatch accÃ¨de-t-il au contenu des messages ? | C-01 | Non |
| MiyukiniWatch accÃ¨de-t-il aux champs de saisie ? | C-02 | Non |
| MiyukiniWatch accÃ¨de-t-il aux fichiers ? | C-03 | Non |
| Des donnÃ©es quittent-elles le COG ? | C-05, C-06, C-07 | Non |
| L'utilisateur peut-il voir toutes les donnÃ©es ? | C-09 | Oui |
| L'utilisateur peut-il effacer les donnÃ©es ? | C-10 | Oui |
| L'utilisateur peut-il dÃ©sactiver la collecte ? | C-11 | Oui |
| L'Ã©criture passe-t-elle par KindMother ? | C-13 | Oui |
| L'autorisation passe-t-elle par StrongFather ? | C-14 | Oui |
| MiyukiniWatch gÃ©nÃ¨re-t-il des notifications ? | INV-02 | Non |
| La collecte bloque-t-elle l'UI ? | INV-05 | Non |
| Miou reÃ§oit-il des donnÃ©es brutes ? | INV-04 | Non |
| Les donnÃ©es ont-elles une rÃ©tention bornÃ©e ? | DAT-01 | Oui |
| Les horodatages sont-ils locaux ? | INV-07 | Oui |
| Le service fonctionne-t-il hors rÃ©seau ? | C-08 | Oui |

### 10.2 Checklist d'implÃ©mentation

#### Phase 1 â€” Infrastructure
- [ ] Crate `miyukiniwatch` crÃ©Ã© avec Cargo.toml
- [ ] SchÃ©ma SQL crÃ©Ã© et appliquÃ©
- [ ] MiyukiniWatchDb implÃ©mentÃ© (open, init_schema, insert_metric, etc.)
- [ ] Types de mÃ©triques et agrÃ©gats dÃ©finis
- [ ] Isolation par profile_id sur toutes les requÃªtes

#### Phase 2 â€” Collector
- [ ] Collecteur implÃ©mentÃ© et connectÃ© aux Ã©vÃ©nements
- [ ] VÃ©rification prÃ©-collecte (collect_enabled, catÃ©gories, Ã©tat T)
- [ ] Collecte asynchrone, non bloquante
- [ ] Limites de volumÃ©trie respectÃ©es
- [ ] DÃ©duplication par (session_id, metric_id, timestamp)

#### Phase 3 â€” Aggregator
- [ ] AgrÃ©gation quotidienne et hebdomadaire
- [ ] Purge automatique en cascade
- [ ] Catalogue des agrÃ©gats Miou implÃ©mentÃ©
- [ ] Ã‰vÃ©nements d'audit pour les purges

#### Phase 4 â€” Presenter et UI
- [ ] 4 Ã©crans implÃ©mentÃ©s (Tableau de bord, DÃ©tail, ParamÃ¨tres, Audit)
- [ ] Modale de confirmation d'effacement
- [ ] Toggles et sliders de configuration
- [ ] Navigation locale et responsive

#### Phase 5 â€” IntÃ©gration Central
- [ ] Service enregistrÃ© dans default_services
- [ ] Vue MiyukiniWatch routÃ©e dans ActiveServiceView
- [ ] MiyukiniWatchDb dans ServiceConnections
- [ ] Points d'injection Collector branchÃ©s (login, logout, navigation, etc.)

#### Phase 6 â€” IntÃ©gration Miou
- [ ] API/provider d'agrÃ©gats exposÃ©
- [ ] Miou consomme les agrÃ©gats (pas les bruts)
- [ ] DÃ©gradation gracieuse si pas de donnÃ©es
- [ ] RÃ©solution friend_id â†’ pseudo externalisÃ©e

#### Phase 7 â€” SÃ©curitÃ©
- [ ] Chiffrement au repos vÃ©rifiÃ©
- [ ] Matrice d'accÃ¨s respectÃ©e
- [ ] Gestion T0â€“T4 implÃ©mentÃ©e
- [ ] Journal d'audit opÃ©rationnel
- [ ] `unsafe_code = "forbid"` et pas de dÃ©pendance rÃ©seau

---

## RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [MiyukiniWatch - Document Fondateur](./MiyukiniWatch%20-%20Document%20Fondateur.md) | Vision, principes, mÃ©triques initiales |
| [MiyukiniWatch - Architecture et Positionnement](./MiyukiniWatch%20-%20Architecture%20et%20Positionnement.md) | Pyramide, Cores, flux de gouvernance |
| [MiyukiniWatch - SpÃ©cification MÃ©triques et Collecte](./MiyukiniWatch%20-%20Specification%20Fonctionnelle%20Metriques%20et%20Collecte.md) | Catalogue mÃ©triques, Ã©vÃ©nements, structures |
| [MiyukiniWatch - Gouvernance DonnÃ©es et RÃ©tention](./MiyukiniWatch%20-%20Gouvernance%20Donnees%20et%20Retention.md) | RÃ©tention, purge, effacement, audit |
| [MiyukiniWatch - Interface Utilisateur et Ã‰crans](./MiyukiniWatch%20-%20Interface%20Utilisateur%20et%20Ecrans.md) | SpÃ©cification des 4 Ã©crans |
| [MiyukiniWatch - IntÃ©gration Miou et AgrÃ©gats](./MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md) | Contrat d'agrÃ©gats, exemples de bulles |
| [MiyukiniWatch - Contraintes et Invariants](./MiyukiniWatch%20-%20Contraintes%20et%20Invariants.md) | Contraintes non nÃ©gociables |
| [MiyukiniWatch - SÃ©curitÃ© et ConformitÃ©](./MiyukiniWatch%20-%20Securite%20et%20Conformite.md) | Classification, chiffrement, Ã©tats de confiance |
| [Bot - IntÃ©gration et Flux de DonnÃ©es](../MiyukiniCentral/Miou/Bot/Bot%20-%20Integration%20et%20Flux%20de%20Donnees.md) | IntÃ©gration Miou dÃ©taillÃ©e |
| [miyukini-rust-patterns](_index.md) | Patterns crate, admin_cell, context, errors |
| [miyukini-architecture](_index.md) | Pyramide, Cores, Lois d'Autonomie |

---

**Document** : MiyukiniWatch â€” Guide d'ImplÃ©mentation Complet  
**Version** : 1.0  
**Date** : 2026-02-15  
**Statut** : Guide normatif â€” rÃ©fÃ©rence d'implÃ©mentation exhaustive


