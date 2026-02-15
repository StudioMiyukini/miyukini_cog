# MiyukiniWatch — Guide d'Implémentation Complet

## Contexte

Ce document constitue le **guide d'implémentation le plus exhaustif possible** du Service MiyukiniWatch. Il synthétise et détaille les spécifications contenues dans les huit documents fondateurs et fournit des instructions étape par étape pour réaliser une implémentation conforme.

## Portée / Scope

- **Applicable à :** Toutes les phases de développement de MiyukiniWatch (crates, Central, Miou).
- **Audience :** Développeurs, architectes, équipes produit.
- **Statut :** Guide normatif — référence d'implémentation.

---

## Table des matières

1. [Vue d'ensemble de l'implémentation](#1-vue-densemble-de-limplementation)
2. [Prérequis et dépendances](#2-prérequis-et-dépendances)
3. [Phase 1 — Infrastructure et persistance](#3-phase-1--infrastructure-et-persistance)
4. [Phase 2 — Opérateur Collector](#4-phase-2--opérateur-collector)
5. [Phase 3 — Opérateur Aggregator](#5-phase-3--opérateur-aggregator)
6. [Phase 4 — Opérateur Presenter et interface](#6-phase-4--opérateur-presenter-et-interface)
7. [Phase 5 — Intégration Central](#7-phase-5--intégration-central)
8. [Phase 6 — Intégration Miou](#8-phase-6--intégration-miou)
9. [Phase 7 — Sécurité, conformité et audit](#9-phase-7--sécurité-conformité-et-audit)
10. [Matrices de vérification et checklist](#10-matrices-de-vérification-et-checklist)

---

## 1. Vue d'ensemble de l'implémentation

### 1.1 Composants à créer

| Composant | Type | Emplacement | Rôle |
|-----------|------|-------------|------|
| **miyukiniwatch** | Crate Service (Strate 7) | `crates/miyukiniwatch/` | Logique métier, métriques, agrégats, persistance |
| **MiyukiniWatchCollector** | Opérateur | Dans `miyukiniwatch` | Collecte passive des métriques |
| **MiyukiniWatchAggregator** | Opérateur | Dans `miyukiniwatch` | Agrégation périodique |
| **MiyukiniWatchPresenter** | Opérateur | Dans `miyukiniwatch` + Central UI | Consultation, effacement, paramètres |
| **Vue MiyukiniWatch** | Service Central | `apps/central/src/services/miyukiniwatch/` | Interface utilisateur (4 écrans) |
| **Connexion DB** | Data layer | `apps/central/src/data.rs` | `MiyukiniWatchDb` dans `ServiceConnections` |

### 1.2 Ordre de dépendances

```
KindMother (existant)
       │
       ▼
miyukiniwatch (crate) — persistance, opérateurs
       │
       ├── Miyukini Central (service view, intégration)
       │
       └── Miou (consommation agrégats)
```

### 1.3 Principes d'implémentation impératifs

| # | Principe | Vérification |
|---|----------|---------------|
| P1 | **Jamais de lecture de contenus** | Aucun accès aux champs texte, messages, fichiers, DOM |
| P2 | **Données locales uniquement** | Aucune dépendance réseau ; `unsafe_code = "forbid"` |
| P3 | **Gouvernance par les Cores** | Toutes les opérations via BondingBrother → StrongFather, KindMother, Master Butler |
| P4 | **Collecte asynchrone et non bloquante** | WriteIntent en tâche de fond ; priorité basse |
| P5 | **Agrégats pour Miou, pas données brutes** | Miou ne consomme que des structures pré-calculées |

---

## 2. Prérequis et dépendances

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

### 2.2 Aucune dépendance réseau

Vérifier que **ni** `hyper`, **ni** `reqwest`, **ni** `tokio::net` ne sont importés. Conformité C-05, C-08.

### 2.3 Lints obligatoires

```toml
[lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"
```

---

## 3. Phase 1 — Infrastructure et persistance

### 3.1 Structure du crate miyukiniwatch

```
crates/miyukiniwatch/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── errors.rs
    ├── types.rs           # Métriques, agrégats, structures
    ├── schema.sql         # Schéma SQL
    ├── db.rs              # MiyukiniWatchDb (KindMother Daughter)
    ├── operator/
    │   ├── mod.rs
    │   ├── collector.rs
    │   ├── aggregator.rs
    │   └── presenter.rs
    └── aggregates.rs      # Calcul des agrégats exposés à Miou
```

### 3.2 Schéma SQL (KindMother)

Créer `crates/miyukiniwatch/src/schema.sql` :

```sql
-- Métriques brutes (rétention 7-90 j. par défaut 30)
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

-- Agrégats quotidiens (rétention 30-365 j. par défaut 90)
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

-- Agrégats hebdomadaires (rétention 90-730 j. par défaut 365)
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

-- Compteurs globaux (effaçables manuellement)
CREATE TABLE IF NOT EXISTS miyukiniwatch_globals (
    profile_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value INTEGER NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (profile_id, key)
);

-- Journal d'audit (rétention min 90 j., défaut 365)
CREATE TABLE IF NOT EXISTS miyukiniwatch_audit (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    profile_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    event_type TEXT NOT NULL,
    details TEXT,
    records_affected INTEGER
);

-- Préférences (collecte, rétention, catégories)
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

### 3.3 Identifiants de métriques (référence SF Métriques)

| ID | Nom | Table |
|----|-----|-------|
| S-01 | Début session | `metric_id = "S-01"` |
| S-02 | Fin session | S-02 |
| S-03 | Durée session | S-03 |
| S-04 | Tranche horaire | S-04 |
| S-05 | Jours depuis dernière session | S-05 (global) |
| S-06 | Compteur sessions | S-06 (global) |
| S-07 | Jours actifs consécutifs | S-07 (global) |
| SV-01 | Service ouvert | SV-01 |
| SV-02 | Service fermé | SV-02 |
| SV-03 | Temps par service | SV-03 |
| SV-07 | Onglet principal | SV-07 |
| A-01 | Ami contacté | A-01 |
| A-03 | Durée discussion ami | A-03 |
| I-01 | Clics (global) | I-01 |
| I-02 | Clics par service | I-02 |
| L-01 | Rite d'Entrée | L-01 |
| L-02 | Type d'entrée | L-02 |
| L-03 | Premier service installé | L-03 |
| L-04 | Connexion MWS | L-04 |

### 3.4 Tranches horaires

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeSlot {
    Morning,    // 06:00 – 11:59
    Afternoon,  // 12:00 – 17:59
    Evening,    // 18:00 – 22:59
    Night,      // 23:00 – 05:59
}
```

### 3.5 Implémentation MiyukiniWatchDb

- Suivre le pattern `JayXposeDb` : SQLite chiffré via `kindmother_db_key`, instance KindMother Daughter.
- Méthodes minimales : `open`, `init_schema`, `insert_metric`, `insert_audit`, `get_prefs`, `set_prefs`.
- Isolation par `profile_id` sur toutes les requêtes.

---

## 4. Phase 2 — Opérateur Collector

### 4.1 Responsabilité

Le Collector **écoute passivement** les événements du COG. Il ne crée jamais d'événement. Il ne modifie pas le comportement des autres services.

### 4.2 Événements à consommer

| Source | Événement | Métriques produites |
|--------|-----------|---------------------|
| Central — Auth | `user_logged_in` | S-01, S-04, L-02 |
| Central — Auth | `user_logged_out` | S-02, S-03 |
| Central — Navigation | `service_opened` | SV-01 |
| Central — Navigation | `service_closed` | SV-02, SV-03 |
| Central — Navigation | `tab_changed` | SV-07 |
| Central — Rite | `rite_completed` | L-01 |
| Jay1Tribu / sociaux | `conversation_opened` | A-01 |
| Jay1Tribu / sociaux | `conversation_closed` | A-03 ( durée ) |
| Central — UI | `user_click` | I-01, I-02 |
| MWS Participant | `webway_connected` | L-04 |
| Central — Services | `service_installed` | L-03 |

### 4.3 Mécanisme de bus d'événements

- **Option A (si existant) :** S'abonner au bus d'événements Central (ex. `EventBus`, `watch()`).
- **Option B (MVP) :** Le Collector reçoit des appels directs depuis Central lors des transitions (login, logout, ouverture service, etc.).

### 4.4 Règles de collecte

| Règle | Implémentation |
|-------|----------------|
| Passivité | Aucun `emit` ou `publish` vers d'autres services |
| Atomicité | Un `insert_metric` par WriteIntent ; échec isolé |
| Idempotence | Clé unique `(session_id, metric_id, timestamp)` ; `INSERT OR IGNORE` |
| Priorité basse | Exécuter en `spawn_blocking` ou tâche basse priorité |
| Aucun blocage | Jamais d'`await` bloquant sur l'UI |

### 4.5 Vérification pré-collecte

Avant d'écrire toute métrique :

1. Vérifier `collect_enabled` dans les préférences.
2. Vérifier la catégorie (sessions, services, amis, activité) si désactivée individuellement.
3. Vérifier l'état de confiance (Caring Nanny) : en T2, ne collecter que sessions + services ; en T3, ne pas collecter.

### 4.6 Limites de volumétrie

- Max 10 000 métriques brutes par session (configurable via TAMR).
- Si dépassement : arrêter la collecte pour la session, ne pas crasher.

---

## 5. Phase 3 — Opérateur Aggregator

### 5.1 Responsabilité

Agréger les métriques brutes en résumés quotidiens et hebdomadaires. Produire les agrégats exposés à Miou. Exécuter la purge automatique.

### 5.2 Déclencheurs d'agrégation

| Déclencheur | Action |
|-------------|--------|
| Fin de session | Agrégation des métriques de la session en cours |
| Première session du jour | Agrégation quotidienne de la veille ; purge métriques brutes expirées |
| Première session de la semaine | Agrégation hebdomadaire ; purge agrégats quotidiens expirés |
| Première session du mois | Purge agrégats hebdomadaires expirés |
| À la demande (Miou, UI) | Calcul des agrégats en mémoire pour la période demandée |

### 5.3 Catalogue des agrégats exposés (contrat Miou)

Implémenter les structures et calculs pour :

| Identifiant | Contenu |
|-------------|---------|
| `AGG_SESSION_SUMMARY` | `days_since_last_session`, `avg_duration_seconds`, `usual_time_slot`, `total_sessions`, `consecutive_active_days` |
| `AGG_SESSION_RETURN` | `is_returning`, `days_away` |
| `AGG_SESSION_TIME` | `current_time_slot` |
| `AGG_TOP_SERVICES` | Liste top 5 `(service_id, open_count, total_duration)` sur 7 jours |
| `AGG_NEGLECTED_SERVICES` | Services non ouverts depuis > 14 jours |
| `AGG_FAVORITE_SERVICE` | Service le plus fréquenté sur 7 jours |
| `AGG_FAVORITE_TAB` | Salon / Bibliothèque / Webway le plus utilisé |
| `AGG_FRIEND_REMINDERS` | Amis non contactés depuis > 7 jours |
| `AGG_TOP_FRIENDS` | Top 3 par durée sur 30 jours |
| `AGG_SOCIAL_ACTIVITY` | `distinct_friends_contacted_week`, `total_social_time_week` |
| `AGG_ACTIVITY_LEVEL` | `level`, `sessions_week`, `total_duration_week` |
| `AGG_CURRENT_SESSION` | `duration_current_session`, `services_opened_count` |
| `AGG_MILESTONES` | Liste de jalons (streaks, badges) |
| `AGG_NEW_MILESTONE` | Jalon atteint dans la session en cours (optionnel) |

### 5.4 Purge automatique

1. Lire les préférences de rétention (`retention_raw_days`, etc.).
2. Identifier les enregistrements expirés.
3. Soumettre un PurgeIntent (conceptuel ; en pratique, exécuter des `DELETE` via KindMother).
4. Enregistrer un événement d'audit : `event_type = "purge_automatic"`, `records_affected`, `details`.

### 5.5 Cascade d'agrégation

- **Ordre impératif :** D'abord calculer/écrire les agrégats de niveau N, puis purger le niveau N+1.
- Exemple : avant de purger les métriques brutes de plus de 30 jours, s'assurer que les agrégats quotidiens correspondants existent.

---

## 6. Phase 4 — Opérateur Presenter et interface

### 6.1 Responsabilité du Presenter

- Lire les agrégats et métriques pour affichage.
- Exécuter les DeleteIntent (effacement) sur demande utilisateur.
- Gérer les préférences (activation/désactivation, rétention).

### 6.2 Quatre écrans à implémenter

#### Écran 1 — Tableau de bord

| Zone | Composants | Données |
|------|------------|---------|
| En-tête | Titre + phrase d'explication | Statique |
| Bloc périmètre | 4 dimensions (Quand, Où, Qui, Combien) avec ✓/✗ | Depuis préférences |
| Carte Sessions | Dernière session, jours depuis visite, tranche habituelle | `AGG_SESSION_SUMMARY`, `AGG_SESSION_RETURN` |
| Carte Services | Top 3, dernier ouvert, plus long | `AGG_TOP_SERVICES`, `AGG_FAVORITE_SERVICE` |
| Carte Amis | Contactés récemment, non contacté le plus longtemps | `AGG_FRIEND_REMINDERS`, `AGG_TOP_FRIENDS` |
| Carte Activité | Clics/jour, sessions/semaine, streak | `AGG_ACTIVITY_LEVEL`, `AGG_MILESTONES` |
| Pied | Liens Détail, Paramètres, Effacer tout | Navigation |

États spéciaux : première utilisation, collecte désactivée, données effacées.

#### Écran 2 — Détail des métriques

- Onglets : Sessions, Services, Amis, Activité.
- Filtres : Aujourd'hui, Cette semaine, Ce mois, Personnalisé.
- Tableaux triables, graphiques (barres, circulaire).
- Rappel de transparence en bas de chaque onglet.

#### Écran 3 — Paramètres et vie privée

- Toggle collecte globale.
- Toggles par catégorie (Sessions, Services, Amis, Activité).
- Sliders rétention : brut (7–90 j.), quotidien (30–365 j.), hebdo (90–730 j.).
- Indicateur espace utilisé.
- Boutons effacement : dernière semaine, dernier mois, tout, par catégorie.
- Modale de confirmation pour tout effacement.

#### Écran 4 — Historique des actions (audit)

- Journal chronologique (collecte on/off, effacements, purges, modifications rétention).
- Filtrage par type, période.
- Détail au clic.

### 6.3 Navigation locale

Barre d'onglets ou sidebar : [Tableau de bord] [Détail] [Paramètres] [Audit].

### 6.4 Principes UX

- Tutoiement, bienveillance, clarté.
- Responsive : desktop 2 colonnes, mobile colonne unique.
- Accessibilité : WCAG AA, navigation clavier, ARIA sur graphiques et toggles.

---

## 7. Phase 5 — Intégration Central

### 7.1 Enregistrement du service

Dans `apps/central/src/state.rs`, ajouter à `default_services()` :

```rust
ServiceInfo {
    id: "miyukiniwatch".to_string(),
    name: "MiyukiniWatch".to_string(),
    description: "Tes habitudes et tes mesures — consulte, comprends, efface.".to_string(),
    icon: "👁".to_string(),  // ou icône thématique définie
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

Émettre (ou appeler le Collector) aux moments suivants :

| Moment | Emplacement probable | Événement / Action |
|--------|----------------------|---------------------|
| Login réussi | `Connexion` ou `auth` | `user_logged_in` |
| Logout / fermeture | Idem | `user_logged_out` |
| Ouverture service | `open_service` dans state | `service_opened` |
| Fermeture service / navigation | Navigation logic | `service_closed` |
| Changement d'onglet | `TabBar` | `tab_changed` |
| Clic utilisateur | Wrapper ou zone commune | `user_click` (agrégat uniquement) |
| Rite terminé | `RiteEntree` | `rite_completed` |
| Service installé | Logique d'installation | `service_installed` |

### 7.5 Badge « Collecte désactivée »

Si `collect_enabled == false`, afficher un indicateur discret sur la carte du service dans la grille (ex. petite pastille grise).

---

## 8. Phase 6 — Intégration Miou

### 8.1 Interface d'accès aux agrégats

Créer une façade ou module exposé à Miou :

```rust
pub trait MiyukiniWatchAggregatesProvider {
    fn get_aggregate(&self, profile_id: &str, aggregate_id: &str) -> Option<Aggregate>;
    fn get_all_aggregates(&self, profile_id: &str) -> Vec<Aggregate>;
    fn has_data(&self, profile_id: &str) -> bool;
    fn is_collecting(&self, profile_id: &str) -> bool;
}
```

### 8.2 Règles de consommation (côté Miou)

- Lecture seule : Miou ne modifie jamais MiyukiniWatch.
- Dégradation gracieuse : si agrégat absent → message générique.
- Pas de cache longue : recharger à chaque session.
- Résolution `friend_cog_id` → pseudo : via le service de contacts (Jay1Tribu), pas MiyukiniWatch.

### 8.3 Dérivées pour le Bot Miou

D'après [Bot - Intégration et Flux de Données](../MiyukiniCentral/Miou/Bot/Bot%20-%20Integration%20et%20Flux%20de%20Donnees.md) :

| Requête | Retour | Usage |
|---------|--------|-------|
| `get_session_summary(profile_id)` | SessionSummary | Jours absence, durée |
| `get_services_usage(profile_id, period)` | Vec<ServiceUsage> | Service délaissé, top |
| `get_friends_status(profile_id)` | Vec<FriendStatus> | Ami le plus délaissé |
| `get_clicks_count(profile_id, period)` | u32 | Indicateur activité |

### 8.4 Matrice agrégat → bulle

Voir [MiyukiniWatch - Intégration Miou et Agrégats](./MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md) section 4.1 pour les exemples de bulles par agrégat.

---

## 9. Phase 7 — Sécurité, conformité et audit

### 9.1 Classification des données

- Métriques sessions/services : niveau 1 (Standard).
- Métriques amis : niveau 2 (Sensitive).
- Chiffrement au repos : via KindMother / `kindmother_db_key` (déjà en place pour les DB filles).

### 9.2 Matrice d'accès

Respecter la matrice [Sécurité et Conformité](./MiyukiniWatch%20-%20Securite%20et%20Conformite.md) section 3.1 :

- Utilisateur : lecture, effacement, config — pas d'écriture directe.
- Collector : écriture uniquement.
- Aggregator : lecture + écriture agrégats + purge.
- Presenter : lecture + effacement (DeleteIntent).
- Miou : lecture agrégats uniquement.

### 9.3 États de confiance (T0–T4)

Adapter le Collector selon l'état Caring Nanny :

- T0 : collecte complète.
- T1 : collecte normale, agrégation peut être retardée.
- T2 : collecte réduite (sessions + services uniquement).
- T3 : collecte suspendue.
- T4 : MiyukiniWatch inaccessible.

### 9.4 Journal d'audit

Enregistrer tous les événements listés dans [Gouvernance Données et Rétention](./MiyukiniWatch%20-%20Gouvernance%20Donnees%20et%20Retention.md) section 6.1. Rétention du journal : min 90 j., défaut 365 j.

### 9.5 Règles de code

- `unsafe_code = "forbid"`.
- Pas de crate réseau.
- Pas d'accès fichier direct (tout via KindMother/MiyukiniWatchDb).

---

## 10. Matrices de vérification et checklist

### 10.1 Matrice des contraintes (extrait Contraintes et Invariants)

| Vérification | Critère | OK ? |
|--------------|---------|------|
| MiyukiniWatch accède-t-il au contenu des messages ? | C-01 | Non |
| MiyukiniWatch accède-t-il aux champs de saisie ? | C-02 | Non |
| MiyukiniWatch accède-t-il aux fichiers ? | C-03 | Non |
| Des données quittent-elles le COG ? | C-05, C-06, C-07 | Non |
| L'utilisateur peut-il voir toutes les données ? | C-09 | Oui |
| L'utilisateur peut-il effacer les données ? | C-10 | Oui |
| L'utilisateur peut-il désactiver la collecte ? | C-11 | Oui |
| L'écriture passe-t-elle par KindMother ? | C-13 | Oui |
| L'autorisation passe-t-elle par StrongFather ? | C-14 | Oui |
| MiyukiniWatch génère-t-il des notifications ? | INV-02 | Non |
| La collecte bloque-t-elle l'UI ? | INV-05 | Non |
| Miou reçoit-il des données brutes ? | INV-04 | Non |
| Les données ont-elles une rétention bornée ? | DAT-01 | Oui |
| Les horodatages sont-ils locaux ? | INV-07 | Oui |
| Le service fonctionne-t-il hors réseau ? | C-08 | Oui |

### 10.2 Checklist d'implémentation

#### Phase 1 — Infrastructure
- [ ] Crate `miyukiniwatch` créé avec Cargo.toml
- [ ] Schéma SQL créé et appliqué
- [ ] MiyukiniWatchDb implémenté (open, init_schema, insert_metric, etc.)
- [ ] Types de métriques et agrégats définis
- [ ] Isolation par profile_id sur toutes les requêtes

#### Phase 2 — Collector
- [ ] Collecteur implémenté et connecté aux événements
- [ ] Vérification pré-collecte (collect_enabled, catégories, état T)
- [ ] Collecte asynchrone, non bloquante
- [ ] Limites de volumétrie respectées
- [ ] Déduplication par (session_id, metric_id, timestamp)

#### Phase 3 — Aggregator
- [ ] Agrégation quotidienne et hebdomadaire
- [ ] Purge automatique en cascade
- [ ] Catalogue des agrégats Miou implémenté
- [ ] Événements d'audit pour les purges

#### Phase 4 — Presenter et UI
- [ ] 4 écrans implémentés (Tableau de bord, Détail, Paramètres, Audit)
- [ ] Modale de confirmation d'effacement
- [ ] Toggles et sliders de configuration
- [ ] Navigation locale et responsive

#### Phase 5 — Intégration Central
- [ ] Service enregistré dans default_services
- [ ] Vue MiyukiniWatch routée dans ActiveServiceView
- [ ] MiyukiniWatchDb dans ServiceConnections
- [ ] Points d'injection Collector branchés (login, logout, navigation, etc.)

#### Phase 6 — Intégration Miou
- [ ] API/provider d'agrégats exposé
- [ ] Miou consomme les agrégats (pas les bruts)
- [ ] Dégradation gracieuse si pas de données
- [ ] Résolution friend_id → pseudo externalisée

#### Phase 7 — Sécurité
- [ ] Chiffrement au repos vérifié
- [ ] Matrice d'accès respectée
- [ ] Gestion T0–T4 implémentée
- [ ] Journal d'audit opérationnel
- [ ] `unsafe_code = "forbid"` et pas de dépendance réseau

---

## Références

| Document | Rôle |
|----------|------|
| [MiyukiniWatch - Document Fondateur](./MiyukiniWatch%20-%20Document%20Fondateur.md) | Vision, principes, métriques initiales |
| [MiyukiniWatch - Architecture et Positionnement](./MiyukiniWatch%20-%20Architecture%20et%20Positionnement.md) | Pyramide, Cores, flux de gouvernance |
| [MiyukiniWatch - Spécification Métriques et Collecte](./MiyukiniWatch%20-%20Specification%20Fonctionnelle%20Metriques%20et%20Collecte.md) | Catalogue métriques, événements, structures |
| [MiyukiniWatch - Gouvernance Données et Rétention](./MiyukiniWatch%20-%20Gouvernance%20Donnees%20et%20Retention.md) | Rétention, purge, effacement, audit |
| [MiyukiniWatch - Interface Utilisateur et Écrans](./MiyukiniWatch%20-%20Interface%20Utilisateur%20et%20Ecrans.md) | Spécification des 4 écrans |
| [MiyukiniWatch - Intégration Miou et Agrégats](./MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md) | Contrat d'agrégats, exemples de bulles |
| [MiyukiniWatch - Contraintes et Invariants](./MiyukiniWatch%20-%20Contraintes%20et%20Invariants.md) | Contraintes non négociables |
| [MiyukiniWatch - Sécurité et Conformité](./MiyukiniWatch%20-%20Securite%20et%20Conformite.md) | Classification, chiffrement, états de confiance |
| [Bot - Intégration et Flux de Données](../MiyukiniCentral/Miou/Bot/Bot%20-%20Integration%20et%20Flux%20de%20Donnees.md) | Intégration Miou détaillée |
| [miyukini-rust-patterns](.cursor/skills/miyukini-rust-patterns/) | Patterns crate, admin_cell, context, errors |
| [miyukini-architecture](.cursor/skills/miyukini-architecture/) | Pyramide, Cores, Lois d'Autonomie |

---

**Document** : MiyukiniWatch — Guide d'Implémentation Complet  
**Version** : 1.0  
**Date** : 2026-02-15  
**Statut** : Guide normatif — référence d'implémentation exhaustive
