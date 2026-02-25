# JayManga — Guide d'implementation

## Contexte

Ce document est le **guide d'implementation technique** du service JayManga. Il definit la structure du crate `jaymanga`, les modules a developper, les patterns Rust a suivre (alignes sur les crates existants JayShop, JayFestival, JayXpose), les dependances, les interfaces (APIs, endpoints, composants UI) et les conventions de code.

Ce guide s'adresse aux developpeurs implementant JayManga. Il traduit les specifications fonctionnelles (documents deja rediges) en specifications techniques actionnables.

## Portee / Scope

- **Perimetre** : Architecture du crate `jaymanga`, structure des modules, types de donnees Rust, schemas de base de donnees, APIs REST, composants UI (Dioxus natif + Web Portal), integration MWS (manifestes, presence), systeme de gamification.
- **Hors perimetre** : Implementation ligne par ligne (ce guide donne la structure et les signatures, pas le corps des fonctions), deploiement, configuration serveur.
- **References** : [JayManga - Document Fondateur](./JayManga%20-%20Document%20Fondateur.md), [JayManga - Plan Implementation](./JayManga%20-%20Plan%20Implementation.md), skills `miyukini-rust-patterns`, `miyukini-services`, `miyukini-architecture`.

---

## 1. Architecture du crate

### 1.1 Position dans la pyramide

| Attribut | Valeur |
|----------|--------|
| **Strate** | 7 (Operateur) |
| **Type** | Service (pattern Jay*) |
| **Crate** | `crates/jaymanga/` |
| **Dependances Cores** | KindMother (persistance), StrongFather (gouvernance publication), MasterButler (permissions lecture/telechargement), WorrySentinel (securite paiement), BorderGuard (frontieres contenu) |
| **Dependances MWS** | `miyuwebway_participant` (presence, manifestes) |
| **Dependances Toolkits** | `miyuvalidate`, `miyutext`, `miyulocale`, `miyunotify` |
| **Dependances Services** | Optionnelles : `jayxpose` (catalogue vitrine), `jaykonta` (comptabilite Phase 2) |

### 1.2 Structure du crate

```
crates/jaymanga/
├── Cargo.toml
└── src/
    ├── lib.rs                          # Point d'entree, expose les modules
    │
    ├── data/                           # Couche persistance
    │   ├── mod.rs                      # Feature flags, re-exports
    │   ├── types.rs                    # Types de domaine (Work, Chapter, Page, Series, etc.)
    │   ├── types_payment.rs            # Types paiement (PurchaseLicense, PaymentTransaction, Promotion)
    │   ├── types_reader.rs             # Types lecteur (ReaderFavorite, ReaderProgression, ReaderBadge)
    │   ├── types_aggregator.rs         # Types aggregation (AggregatedCatalogEntry, IndexedSeller)
    │   ├── types_federation.rs         # Types federation API (FederationCatalogEntry, FederationInfo)
    │   ├── kindmother_db.rs            # SQLite direct (feature legacy-sqlite)
    │   ├── kindmother_client_db.rs     # Client KindMother (feature kindmother-only)
    │   └── schema.sql                  # Schema de base de donnees
    │
    ├── auth/                           # Authentification et permissions
    │   ├── mod.rs                      # sign_in, sign_up, sign_out
    │   ├── permissions.rs              # Roles (Admin/Vendor, Reader, Visitor), RLS
    │   └── license_verify.rs           # Verification des licences d'achat
    │
    ├── domain/                         # Logique metier
    │   ├── mod.rs                      # Re-exports
    │   ├── catalog.rs                  # Publication, statuts, series, metadonnees
    │   ├── reader.rs                   # Liseuse, progression, marque-pages
    │   ├── optimizer.rs                # Optimisation/compression des images
    │   ├── payment.rs                  # Panier, checkout, licences, remboursements
    │   ├── promotion.rs                # Promotions et remises
    │   ├── download.rs                 # Telechargement hors-ligne, integrite SHA-256
    │   ├── favorites.rs                # Favoris cross-COG, cache metadonnees
    │   ├── gamification.rs             # XP, niveaux, streaks, badges
    │   └── aggregator.rs              # Collecteur de catalogues, cache, synchronisation
    │
    ├── services/                       # Adaptateurs inter-services
    │   ├── mod.rs                      # Re-exports
    │   ├── mws/                        # Integration MWS
    │   │   ├── mod.rs
    │   │   ├── presence.rs             # Requetes de presence (batch, unitaire)
    │   │   ├── manifests.rs            # Publication/mise a jour manifeste JayManga
    │   │   └── discovery.rs            # Decouverte COGs JayManga via trackers
    │   ├── jayxpose/                   # Adaptateur JayXpose (optionnel)
    │   │   ├── mod.rs
    │   │   ├── adapter.rs              # Synchronisation catalogue → vitrine
    │   │   └── contract.rs             # Types de contrat
    │   └── jaykonta/                   # Adaptateur JayKonta (Phase 2)
    │       ├── mod.rs
    │       └── adapter.rs              # Export transactions vers comptabilite
    │
    ├── api/                            # Endpoints REST (surface web / Portail)
    │   ├── mod.rs                      # Router principal
    │   ├── catalog_api.rs              # GET /api/jaymanga/catalog, /works/{id}, etc.
    │   ├── reader_api.rs               # GET /api/jaymanga/reader/*, POST /api/jaymanga/reader/*
    │   ├── payment_api.rs              # POST /api/jaymanga/cart, /checkout, webhooks
    │   ├── federation_api.rs           # GET /api/jaymanga/federation/* (opt-in)
    │   ├── presence_api.rs             # GET /api/jaymanga/presence/*
    │   └── media_api.rs               # GET /api/jaymanga/media/* (pages, couvertures, variantes)
    │
    ├── export/                         # Exports
    │   ├── mod.rs
    │   ├── csv.rs                      # Export CSV des ventes
    │   └── pdf.rs                      # Rapport PDF synthetique
    │
    └── web/                            # Templates/assets Portail web
        ├── mod.rs                      # Configuration du serveur web Portail
        ├── portal.rs                   # Portail vendeur (catalogue public, liseuse web)
        ├── aggregator_portal.rs        # Portail Agrege (vue inter-COG unifiee)
        └── templates/                  # Templates HTML (SSR)
```

### 1.3 Cargo.toml

```toml
[package]
name = "jaymanga"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
documentation.workspace = true
keywords = ["jaymanga", "manga", "reader", "webway", "miyukini"]
categories.workspace = true
description = "Service JayManga — Lecture et vente de manga en ligne sous gouvernance COG"

[features]
default = ["legacy-sqlite"]
legacy-sqlite = ["rusqlite", "kindmother-db-key"]
kindmother-only = ["kindmother-client"]
db-encryption = ["kindmother-db-key/encryption"]

[dependencies]
# Core
miyukini-kernel = { path = "../miyukini-kernel" }
kindmother = { path = "../kindmother" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
thiserror = "2"

# Persistance (feature-gated)
rusqlite = { version = "0.32", features = ["bundled"], optional = true }
kindmother-db-key = { path = "../kindmother-db-key", optional = true }
kindmother-client = { path = "../kindmother-client", optional = true }
kindmother-db-adapter = { path = "../kindmother-db-adapter" }

# MWS
miyuwebway_participant = { path = "../miyuwebway_participant" }

# Toolkits
miyuvalidate = { path = "../miyuvalidate" }
miyutext = { path = "../miyutext" }
miyunotify = { path = "../miyunotify" }

# Image processing
image = "0.25"
webp = "0.3"
sha2 = "0.10"

# Web
axum = { version = "0.7", optional = true }
tower = { version = "0.5", optional = true }
askama = { version = "0.12", optional = true }

[dev-dependencies]
tempfile = "3"

[lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"

[lints.clippy]
all = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
```

---

## 2. Types de domaine

### 2.1 Entites principales (`data/types.rs`)

```rust
use serde::{Deserialize, Serialize};

// --- Oeuvre ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Work {
    pub id: String,                         // UUID v4
    pub series_id: Option<String>,          // FK Serie
    pub title: String,
    pub authors: Vec<Author>,
    pub genres: Vec<String>,
    pub synopsis: Option<String>,
    pub cover_image_path: Option<String>,   // KindMother
    pub language: Option<String>,           // ISO 639-1
    pub volume_number: Option<i32>,
    pub status: WorkStatus,
    pub pricing_model: PricingModel,
    pub price: i64,                         // centimes (RM-05)
    pub currency: String,                   // defaut "EUR"
    pub demo_pages_count: i32,
    pub reading_format: ReadingFormat,
    pub allow_download: bool,
    pub total_pages: i32,
    pub tags: Vec<String>,
    pub created_at: String,                 // ISO 8601
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub role: Option<String>,   // scenariste, dessinateur, encreur, coloriste
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WorkStatus {
    Draft,
    Published,
    Unlisted,
    Archived,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PricingModel {
    Free,
    Paid,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReadingFormat {
    Manga,
    Webtoon,
    Landscape,
    Comics,
    Free,
}

// --- Chapitre ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub id: String,
    pub work_id: String,
    pub chapter_number: i32,
    pub title: Option<String>,
    pub page_count: i32,
    pub sort_order: i32,
    pub created_at: String,
}

// --- Page ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub id: String,
    pub chapter_id: String,
    pub page_number: i32,
    pub original_image_path: String,        // KindMother
    pub optimized_variants: Vec<ImageVariant>,
    pub width: i32,
    pub height: i32,
    pub file_size: i64,                     // octets
    pub optimization_status: OptimizationStatus,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageVariant {
    pub profile: String,        // "hd", "sd", "mobile", "thumb"
    pub format: String,         // "webp", "avif", "jpeg"
    pub path: String,           // KindMother
    pub width: i32,
    pub height: i32,
    pub file_size: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OptimizationStatus {
    Pending,
    Optimized,
    Skipped,
}

// --- Serie ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub id: String,
    pub title: String,
    pub synopsis: Option<String>,
    pub cover_image_path: Option<String>,
    pub status: SeriesStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SeriesStatus {
    Ongoing,
    Completed,
    Hiatus,
}

// --- Configuration vendeur ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellerConfig {
    pub id: String,
    pub shop_name: String,
    pub shop_description: Option<String>,
    pub default_demo_pages: i32,
    pub default_allow_download: bool,
    pub accepted_payment_methods: Vec<String>,
    pub currency: String,
    pub reading_direction: String,          // "rtl" ou "ltr"
    pub theme: Option<serde_json::Value>,   // personnalisation visuelle
    pub allow_aggregation: bool,            // opt-in Portail Agrege (defaut true)
    pub federation_synopsis_length: i32,    // defaut 300
    pub federation_include_prices: bool,    // defaut true
    pub payment_gateway: Option<String>,    // "stripe", "paypal", "mollie", "manual"
    pub gateway_config: Option<String>,     // JSON chiffre (niveau 3)
    pub created_at: String,
    pub updated_at: String,
}

// --- Optimisation ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    pub quality_hd: i32,            // defaut 85
    pub quality_sd: i32,            // defaut 80
    pub quality_mobile: i32,        // defaut 75
    pub quality_thumb: i32,         // defaut 70
    pub output_format: String,      // defaut "webp"
    pub generate_avif: bool,        // defaut false
    pub active_profiles: Vec<String>, // defaut ["hd","sd","mobile","thumb"]
    pub max_concurrent_jobs: i32,   // defaut 2
    pub jpeg_fallback: bool,        // defaut false
}
```

### 2.2 Entites paiement (`data/types_payment.rs`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseLicense {
    pub id: String,
    pub buyer_cog_id: String,
    pub buyer_identity: String,         // LSI, VID, WID
    pub work_id: String,
    pub purchase_type: PurchaseType,    // work, chapter, series
    pub target_id: String,
    pub amount_paid: i64,               // centimes
    pub currency: String,
    pub payment_method: String,
    pub download_allowed: bool,
    pub status: LicenseStatus,
    pub purchased_at: String,
    pub refunded_at: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PurchaseType { Work, Chapter, Series }

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LicenseStatus { Active, Refunded, Revoked }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTransaction {
    pub id: String,
    pub license_id: String,
    pub buyer_cog_id: String,
    pub amount: i64,
    pub currency: String,
    pub method: String,                 // "card", "transfer", "other"
    pub status: TransactionStatus,
    pub external_ref: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus { Pending, Completed, Failed, Refunded, Expired }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Promotion {
    pub id: String,
    pub name: String,
    pub discount_type: DiscountType,
    pub discount_value: i64,            // pourcentage ou centimes
    pub target_scope: String,           // "work", "chapter", "series", "catalog"
    pub target_ids: Vec<String>,
    pub start_date: String,
    pub end_date: String,
    pub active: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DiscountType { Percent, FixedAmount, Free }
```

### 2.3 Entites lecteur (`data/types_reader.rs`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderFavorite {
    pub id: String,
    pub seller_cog_id: String,
    pub work_id: String,
    pub cached_title: String,
    pub cached_cover_url: Option<String>,
    pub cached_authors: Vec<Author>,
    pub cached_format: Option<String>,
    pub purchase_status: PurchaseStatus,
    pub last_read_chapter: Option<i32>,
    pub last_read_page: Option<i32>,
    pub reading_progress: f64,          // 0.0 a 1.0
    pub added_at: String,
    pub last_synced_at: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PurchaseStatus { Demo, Purchased, Downloaded }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderProgression {
    pub id: String,
    pub total_xp: i64,
    pub current_level: i32,             // 1-8
    pub current_streak: i32,
    pub longest_streak: i32,
    pub streak_shield_available: bool,
    pub last_read_date: Option<String>,
    pub total_pages_read: i64,
    pub total_works_completed: i32,
    pub total_chapters_completed: i32,
    pub genres_explored: Vec<String>,
    pub formats_explored: Vec<String>,
    pub cogs_visited: Vec<String>,
    pub languages_read: Vec<String>,
    pub onboarding_completed: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderBadge {
    pub id: String,
    pub badge_id: String,               // ex: "first_page", "centurion"
    pub badge_name: String,
    pub badge_category: BadgeCategory,
    pub earned_at: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BadgeCategory { Reading, Regularity, Exploration }
```

### 2.4 Entites aggregation (`data/types_aggregator.rs`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedCatalogEntry {
    pub id: String,
    pub seller_cog_id: String,
    pub work_id: String,
    pub title: String,
    pub authors: Vec<Author>,
    pub genres: Vec<String>,
    pub synopsis: Option<String>,
    pub cover_thumb_path: Option<String>,
    pub reading_format: String,
    pub pricing_model: String,
    pub price: i64,
    pub currency: String,
    pub chapter_count: i32,
    pub total_pages: i32,
    pub demo_pages_count: i32,
    pub series_title: Option<String>,
    pub tags: Vec<String>,
    pub language: Option<String>,
    pub portal_url: String,
    pub published_at: String,
    pub updated_at: String,
    pub cached_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedSeller {
    pub cog_id: String,
    pub shop_name: String,
    pub shop_description: Option<String>,
    pub avatar_path: Option<String>,
    pub work_count: i32,
    pub online_status: OnlineStatus,
    pub last_synced_at: String,
    pub last_seen_online_at: Option<String>,
    pub blocked: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OnlineStatus { Online, Offline, Unknown }
```

---

## 3. Couche persistance

### 3.1 Feature flags

Le module `data/mod.rs` suit le pattern standard des services Miyukini :

```rust
pub mod types;
pub mod types_payment;
pub mod types_reader;
pub mod types_aggregator;
pub mod types_federation;

#[cfg(feature = "legacy-sqlite")]
mod kindmother_db;

#[cfg(feature = "kindmother-only")]
mod kindmother_client_db;

pub use types::*;
pub use types_payment::*;
pub use types_reader::*;
pub use types_aggregator::*;
pub use types_federation::*;

#[cfg(feature = "legacy-sqlite")]
pub use kindmother_db::{DbError, JayMangaDb};

#[cfg(feature = "kindmother-only")]
pub use kindmother_client_db::{DbError, JayMangaDb};
```

### 3.2 Tables de la base de donnees

Le schema SQL (`data/schema.sql`) cree les tables suivantes, regroupees par domaine :

| Groupe | Tables | Stockage |
|--------|--------|----------|
| **Catalogue** | `works`, `chapters`, `pages`, `series`, `seller_config`, `optimization_config` | COG vendeur |
| **Paiement** | `purchase_licenses`, `payment_transactions`, `promotions`, `cart_items` | COG vendeur |
| **Lecteur** | `reader_favorites`, `reader_progression`, `reader_badges` | COG lecteur |
| **Aggregation** | `aggregated_catalog_entries`, `indexed_sellers`, `aggregator_stats`, `aggregator_config` | COG aggregateur |

### 3.3 Methodes CRUD (`JayMangaDb`)

Convention de nommage alignee sur le pattern standard :

```rust
impl JayMangaDb {
    // --- Works ---
    pub fn work_list(&self, filters: &WorkFilters) -> Result<Vec<Work>, DbError>;
    pub fn work_by_id(&self, id: &str) -> Result<Option<Work>, DbError>;
    pub fn work_create(&self, work: &Work) -> Result<(), DbError>;
    pub fn work_update(&self, work: &Work) -> Result<(), DbError>;
    pub fn work_delete(&self, id: &str) -> Result<(), DbError>;
    pub fn work_count(&self) -> Result<i64, DbError>;

    // --- Chapters ---
    pub fn chapter_list_by_work(&self, work_id: &str) -> Result<Vec<Chapter>, DbError>;
    pub fn chapter_create(&self, chapter: &Chapter) -> Result<(), DbError>;
    pub fn chapter_update(&self, chapter: &Chapter) -> Result<(), DbError>;
    pub fn chapter_delete(&self, id: &str) -> Result<(), DbError>;
    pub fn chapter_reorder(&self, work_id: &str, order: &[(String, i32)]) -> Result<(), DbError>;

    // --- Pages ---
    pub fn page_list_by_chapter(&self, chapter_id: &str) -> Result<Vec<Page>, DbError>;
    pub fn page_create(&self, page: &Page) -> Result<(), DbError>;
    pub fn page_update_variants(&self, page_id: &str, variants: &[ImageVariant]) -> Result<(), DbError>;
    pub fn page_delete(&self, id: &str) -> Result<(), DbError>;
    pub fn page_reorder(&self, chapter_id: &str, order: &[(String, i32)]) -> Result<(), DbError>;

    // --- Series ---
    pub fn series_list(&self) -> Result<Vec<Series>, DbError>;
    pub fn series_by_id(&self, id: &str) -> Result<Option<Series>, DbError>;
    pub fn series_create(&self, series: &Series) -> Result<(), DbError>;
    pub fn series_update(&self, series: &Series) -> Result<(), DbError>;

    // --- Licenses ---
    pub fn license_create(&self, license: &PurchaseLicense) -> Result<(), DbError>;
    pub fn license_by_buyer_and_target(&self, buyer_cog_id: &str, target_id: &str) -> Result<Option<PurchaseLicense>, DbError>;
    pub fn license_list_by_buyer(&self, buyer_cog_id: &str) -> Result<Vec<PurchaseLicense>, DbError>;
    pub fn license_update_status(&self, id: &str, status: LicenseStatus) -> Result<(), DbError>;

    // --- Transactions ---
    pub fn transaction_create(&self, tx: &PaymentTransaction) -> Result<(), DbError>;
    pub fn transaction_list(&self, filters: &TransactionFilters) -> Result<Vec<PaymentTransaction>, DbError>;
    pub fn transaction_update_status(&self, id: &str, status: TransactionStatus) -> Result<(), DbError>;

    // --- Promotions ---
    pub fn promotion_list_active(&self) -> Result<Vec<Promotion>, DbError>;
    pub fn promotion_create(&self, promo: &Promotion) -> Result<(), DbError>;

    // --- Reader (sur COG lecteur) ---
    pub fn favorite_list(&self) -> Result<Vec<ReaderFavorite>, DbError>;
    pub fn favorite_create(&self, fav: &ReaderFavorite) -> Result<(), DbError>;
    pub fn favorite_delete(&self, id: &str) -> Result<(), DbError>;
    pub fn favorite_update_progress(&self, id: &str, chapter: i32, page: i32, progress: f64) -> Result<(), DbError>;
    pub fn favorite_update_cache(&self, id: &str, title: &str, cover: Option<&str>, authors: &[Author]) -> Result<(), DbError>;

    pub fn progression_get(&self) -> Result<Option<ReaderProgression>, DbError>;
    pub fn progression_upsert(&self, prog: &ReaderProgression) -> Result<(), DbError>;
    pub fn badge_list(&self) -> Result<Vec<ReaderBadge>, DbError>;
    pub fn badge_create(&self, badge: &ReaderBadge) -> Result<(), DbError>;

    // --- Aggregation (sur COG aggregateur) ---
    pub fn aggregated_entry_upsert(&self, entry: &AggregatedCatalogEntry) -> Result<(), DbError>;
    pub fn aggregated_entry_list(&self, filters: &AggregatorFilters) -> Result<Vec<AggregatedCatalogEntry>, DbError>;
    pub fn indexed_seller_upsert(&self, seller: &IndexedSeller) -> Result<(), DbError>;
    pub fn indexed_seller_list(&self) -> Result<Vec<IndexedSeller>, DbError>;

    // --- Config ---
    pub fn seller_config_get(&self) -> Result<Option<SellerConfig>, DbError>;
    pub fn seller_config_upsert(&self, config: &SellerConfig) -> Result<(), DbError>;
    pub fn optimization_config_get(&self) -> Result<OptimizationConfig, DbError>;
    pub fn optimization_config_upsert(&self, config: &OptimizationConfig) -> Result<(), DbError>;
}
```

---

## 4. Domaine metier

### 4.1 Module `domain/catalog.rs`

Logique de gestion du catalogue vendeur :

```rust
pub fn catalog_import_from_directory(db: &JayMangaDb, path: &Path, work: &Work) -> Result<ImportResult, CatalogError>;
pub fn catalog_import_from_archive(db: &JayMangaDb, archive_path: &Path, work: &Work) -> Result<ImportResult, CatalogError>;
pub fn catalog_import_incremental(db: &JayMangaDb, work_id: &str, chapter_path: &Path) -> Result<ImportResult, CatalogError>;
pub fn catalog_publish(db: &JayMangaDb, work_id: &str) -> Result<(), CatalogError>;
pub fn catalog_archive(db: &JayMangaDb, work_id: &str) -> Result<(), CatalogError>;
pub fn catalog_validate_demo_pages(total_pages: i32, demo_count: i32) -> Result<(), CatalogError>;  // RM-07
```

### 4.2 Module `domain/optimizer.rs`

Traitement d'image en arriere-plan :

```rust
pub struct OptimizationJob {
    pub page_id: String,
    pub original_path: String,
    pub config: OptimizationConfig,
}

pub fn optimizer_process_page(job: &OptimizationJob) -> Result<Vec<ImageVariant>, OptimizerError>;
pub fn optimizer_select_variant(variants: &[ImageVariant], viewport_width: i32, pixel_ratio: f64, supports_webp: bool, supports_avif: bool) -> Option<&ImageVariant>;
pub fn optimizer_queue_work(db: &JayMangaDb, work_id: &str, config: &OptimizationConfig) -> Result<usize, OptimizerError>;
```

### 4.3 Module `domain/payment.rs`

Gestion des achats et licences :

```rust
pub fn payment_create_license(db: &JayMangaDb, buyer_cog_id: &str, target_id: &str, purchase_type: PurchaseType, amount: i64) -> Result<PurchaseLicense, PaymentError>;
pub fn payment_verify_license(db: &JayMangaDb, buyer_identity: &str, content_id: &str) -> Result<bool, PaymentError>;
pub fn payment_process_webhook(db: &JayMangaDb, gateway: &str, payload: &[u8], signature: &str) -> Result<(), PaymentError>;
pub fn payment_refund(db: &JayMangaDb, license_id: &str, amount: Option<i64>) -> Result<(), PaymentError>;
pub fn payment_apply_promotion(price: i64, promotion: &Promotion) -> i64;
pub fn payment_expire_pending(db: &JayMangaDb, max_age_days: i32) -> Result<usize, PaymentError>;
```

### 4.4 Module `domain/gamification.rs`

Systeme de progression lecteur :

```rust
// Constantes XP
pub const XP_PER_PAGE: i64 = 1;
pub const XP_CHAPTER_BONUS: i64 = 10;
pub const XP_WORK_BONUS: i64 = 50;
pub const XP_DAILY_BONUS: i64 = 5;
pub const XP_NEW_GENRE: i64 = 15;
pub const XP_NEW_COG: i64 = 10;
pub const XP_NEW_FORMAT: i64 = 20;
pub const XP_STREAK_7: i64 = 25;
pub const XP_STREAK_30: i64 = 100;

// Seuils de niveaux
pub const LEVEL_THRESHOLDS: [(i32, &str, i64); 8] = [
    (1, "Curieux", 0),
    (2, "Lecteur", 100),
    (3, "Passione", 500),
    (4, "Devore", 1_500),
    (5, "Otaku", 5_000),
    (6, "Connaisseur", 15_000),
    (7, "Sage", 40_000),
    (8, "Legendaire", 100_000),
];

pub fn gamification_award_page_xp(db: &JayMangaDb, page_read_duration_secs: f64, format: ReadingFormat) -> Result<XpEvent, GamificationError>;
pub fn gamification_award_chapter_complete(db: &JayMangaDb) -> Result<XpEvent, GamificationError>;
pub fn gamification_award_work_complete(db: &JayMangaDb) -> Result<XpEvent, GamificationError>;
pub fn gamification_update_streak(db: &JayMangaDb) -> Result<StreakEvent, GamificationError>;
pub fn gamification_check_badges(db: &JayMangaDb) -> Result<Vec<ReaderBadge>, GamificationError>;
pub fn gamification_level_for_xp(xp: i64) -> (i32, &'static str);
pub fn gamification_xp_for_next_level(current_xp: i64) -> i64;
```

### 4.5 Module `domain/aggregator.rs`

Collecte et cache des catalogues inter-COG :

```rust
pub struct AggregatorConfig {
    pub enabled: bool,
    pub name: String,
    pub sync_interval_minutes: i32,
    pub presence_refresh_minutes: i32,
    pub max_indexed_cogs: i32,
    pub show_offline_works: bool,
    pub highlight_own_catalog: bool,
    pub blocked_cogs: Vec<String>,
}

pub async fn aggregator_sync_cycle(db: &JayMangaDb, mws: &MwsClient, config: &AggregatorConfig) -> Result<SyncReport, AggregatorError>;
pub async fn aggregator_refresh_presence(db: &JayMangaDb, mws: &MwsClient) -> Result<usize, AggregatorError>;
pub fn aggregator_search(db: &JayMangaDb, query: &str, filters: &AggregatorFilters) -> Result<Vec<AggregatedCatalogEntry>, AggregatorError>;
pub fn aggregator_recommend_similar(db: &JayMangaDb, work_id: &str) -> Result<Vec<AggregatedCatalogEntry>, AggregatorError>;
```

---

## 5. APIs REST (Portail web)

### 5.1 Catalogue public

| Methode | Endpoint | Description |
|---------|----------|-------------|
| GET | `/api/jaymanga/catalog` | Liste des oeuvres publiees avec filtres et pagination. |
| GET | `/api/jaymanga/catalog/works/{work_id}` | Fiche oeuvre complete (metadonnees, chapitres, prix). |
| GET | `/api/jaymanga/catalog/series/{series_id}` | Serie avec ses volumes. |
| GET | `/api/jaymanga/catalog/search?q=...` | Recherche textuelle. |

### 5.2 Media

| Methode | Endpoint | Description |
|---------|----------|-------------|
| GET | `/api/jaymanga/media/page/{page_id}` | Image de page (selection variante automatique via `Accept`, viewport). |
| GET | `/api/jaymanga/media/page/{page_id}/{profile}` | Variante specifique (hd, sd, mobile, thumb). |
| GET | `/api/jaymanga/media/cover/{work_id}` | Image de couverture. |

### 5.3 Lecteur

| Methode | Endpoint | Description |
|---------|----------|-------------|
| POST | `/api/jaymanga/reader/progress` | Sauvegarder la progression de lecture (page, chapitre). |
| GET | `/api/jaymanga/reader/progress/{work_id}` | Recuperer la progression. |
| POST | `/api/jaymanga/reader/xp` | Reporter un evenement XP (page lue, chapitre termine). |

### 5.4 Paiement

| Methode | Endpoint | Description |
|---------|----------|-------------|
| POST | `/api/jaymanga/cart/add` | Ajouter un article au panier. |
| GET | `/api/jaymanga/cart` | Contenu du panier. |
| POST | `/api/jaymanga/checkout` | Initier le checkout. |
| POST | `/api/jaymanga/webhook/{gateway}` | Callback de la passerelle de paiement. |
| GET | `/api/jaymanga/license/verify/{content_id}` | Verifier une licence pour un contenu. |

### 5.5 Federation (opt-in)

| Methode | Endpoint | Description |
|---------|----------|-------------|
| GET | `/api/jaymanga/federation/info` | Informations vendeur. |
| GET | `/api/jaymanga/federation/catalog` | Resume du catalogue public. |
| GET | `/api/jaymanga/federation/catalog/since/{timestamp}` | Delta catalogue incrementiel. |

### 5.6 Presence MWS

| Methode | Endpoint | Description |
|---------|----------|-------------|
| GET | `/api/jaymanga/presence/{cog_id}` | Statut de presence d'un COG. |
| POST | `/api/jaymanga/presence/batch` | Statut de presence de plusieurs COGs. |
| GET | `/api/jaymanga/discover` | Liste des COGs JayManga connus. |

---

## 6. Integration MWS

### 6.1 Manifeste JayManga

Le COG vendeur publie un manifeste de service aupres du Tracker MWS (voir [MWS - Manifestes de Services](../../miyukini-webway-system/protocole/MWS%20-%20Manifestes%20de%20Services.md)) :

```rust
pub fn mws_build_jaymanga_manifest(db: &JayMangaDb) -> Result<ServiceManifest, MwsError> {
    let config = db.seller_config_get()?;
    let work_count = db.work_count()?;
    // Construire le payload JayManga selon le schema defini dans MWS - Manifestes
    // Champs : shop_name, work_count, free_work_count, formats, genres,
    //          languages, allow_aggregation, federation_api_base, last_catalog_update
}

pub fn mws_update_manifest(mws: &MwsClient, manifest: &ServiceManifest) -> Result<(), MwsError>;
pub fn mws_query_presence_batch(mws: &MwsClient, cog_ids: &[String]) -> Result<Vec<(String, OnlineStatus)>, MwsError>;
pub fn mws_query_manifests_by_service(mws: &MwsClient, service_id: &str) -> Result<Vec<ServiceManifest>, MwsError>;
```

### 6.2 Declencheurs de mise a jour

Le manifeste est mis a jour automatiquement :

| Evenement | Action |
|-----------|--------|
| Publication/archivage d'une oeuvre | `mws_update_manifest()` |
| Modification `allow_aggregation` | `mws_update_manifest()` |
| Modification `seller_config` | `mws_update_manifest()` |
| Periodique (toutes les 6 heures) | `mws_update_manifest()` si changement detecte |

---

## 7. Composants UI

### 7.1 Central / Stable (Dioxus natif)

Les composants UI pour Central suivent le pattern decrit dans le skill `miyukini-dioxus-ui` :

| Composant | Fichier | Description |
|-----------|---------|-------------|
| `JayMangaServiceCard` | `services/jaymanga/card.rs` | Carte dans le Salon, compte d'oeuvres, ventes du jour. |
| `JayMangaDashboard` | `services/jaymanga/dashboard.rs` | Tableau de bord vendeur (ventes, stats, actions rapides). |
| `JayMangaCatalogAdmin` | `services/jaymanga/catalog_admin.rs` | Gestion du catalogue (liste, filtres, actions). |
| `JayMangaWorkEditor` | `services/jaymanga/work_editor.rs` | Editeur d'oeuvre multi-etape (import, metadata, format, prix). |
| `JayMangaLibrary` | `services/jaymanga/library.rs` | Bibliotheque lecteur (5 onglets). |
| `JayMangaReader` | `services/jaymanga/reader.rs` | Liseuse native (5 formats, raccourcis clavier). |
| `JayMangaProfile` | `services/jaymanga/profile.rs` | Profil lecteur, progression, badges, streaks. |
| `JayMangaSettings` | `services/jaymanga/settings.rs` | Configuration vendeur et preferences lecteur. |

### 7.2 Mobile / Terminal (Dioxus natif)

Memes composants, adaptes au contexte mobile (navigation bottom tabs, gestures tactiles, sync bidirectionnelle avec COG parent).

### 7.3 Web Portal (HTML/CSS/JS)

Templates SSR via le module `web/` avec les routes suivantes :

| Route | Template | Description |
|-------|----------|-------------|
| `/manga` | `catalog.html` | Catalogue public du vendeur |
| `/manga/{work_id}` | `work.html` | Fiche oeuvre |
| `/manga/read/{chapter_id}` | `reader.html` | Liseuse web |
| `/manga/cart` | `cart.html` | Panier |
| `/manga/aggregate` | `aggregate.html` | Portail Agrege (si active) |

---

## 8. Gestion des erreurs

Chaque module de domaine definit ses propres erreurs avec `thiserror` :

```rust
#[derive(Debug, thiserror::Error)]
pub enum JayMangaError {
    #[error("Database error: {0}")]
    Db(#[from] DbError),
    #[error("Catalog error: {0}")]
    Catalog(#[from] CatalogError),
    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),
    #[error("Optimizer error: {0}")]
    Optimizer(#[from] OptimizerError),
    #[error("MWS error: {0}")]
    Mws(#[from] MwsError),
    #[error("Gamification error: {0}")]
    Gamification(#[from] GamificationError),
    #[error("Aggregator error: {0}")]
    Aggregator(#[from] AggregatorError),
    #[error("Not authorized")]
    Unauthorized,
    #[error("Not found: {0}")]
    NotFound(String),
}
```

---

## 9. Stockage fichiers (KindMother)

### 9.1 Arborescence

```
/jaymanga/
├── config/                                 # Configuration vendeur
│   ├── seller_config.json
│   └── optimization_config.json
├── works/{work_id}/                        # Par oeuvre
│   ├── metadata.json                       # Metadonnees en cache
│   ├── cover.jpg                           # Couverture
│   └── chapters/{chapter_id}/
│       ├── originals/                      # Images originales
│       │   ├── page_001.jpg
│       │   └── ...
│       └── optimized/                      # Variantes optimisees
│           ├── page_001_hd.webp
│           ├── page_001_sd.webp
│           ├── page_001_mob.webp
│           ├── page_001_thumb.webp
│           └── ...
├── downloads/{seller_cog_id}/{work_id}/    # Telechargements (COG lecteur)
│   ├── metadata.json
│   ├── license.json
│   ├── cover.jpg
│   └── chapters/...
├── reader/                                 # Donnees lecteur
│   ├── progression.json
│   └── badges.json
└── aggregator/                             # Cache aggregateur
    ├── sellers/
    │   └── {cog_id}/
    │       ├── info.json
    │       └── catalog.json
    └── thumbnails/
        └── {cog_id}_{work_id}_thumb.webp
```

---

## 10. Securite

### 10.1 Niveaux de securite par donnee

| Donnee | Niveau | Protection |
|--------|--------|------------|
| Catalogue public, couvertures, synopsis | 0 (Public) | Aucune restriction |
| Pages de demonstration | 0 (Public) | Servies sans licence |
| Pages payantes | 2 (Sensitive) | Licence requise, verification a chaque requete |
| Licences d'achat | 2 (Sensitive) | Acces restreint (acheteur + vendeur) |
| Transactions de paiement | 3 (Critical) | Chiffrement, acces admin uniquement |
| Cles API passerelle (`gateway_config`) | 3 (Critical) | Chiffrement KindMother (db-encryption) |
| Favoris et progression | 1 (Standard) | Donnees locales COG lecteur |
| Cache aggregateur (metadonnees) | 0-1 (Public-Standard) | Metadonnees publiques en cache |

### 10.2 Anti-scraping

- Rate limiting sur les endpoints media (pages/minute par IP)
- Hotlink protection (verification `Referer` pour les images)
- CSP headers sur les pages du Portail

---

## 11. Tests

### 11.1 Tests unitaires

Chaque module de domaine est accompagne de tests unitaires :

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamification_level_for_xp() {
        assert_eq!(gamification_level_for_xp(0), (1, "Curieux"));
        assert_eq!(gamification_level_for_xp(100), (2, "Lecteur"));
        assert_eq!(gamification_level_for_xp(5000), (5, "Otaku"));
        assert_eq!(gamification_level_for_xp(200_000), (8, "Legendaire"));
    }

    #[test]
    fn test_catalog_validate_demo_pages() {
        assert!(catalog_validate_demo_pages(100, 50).is_ok());   // 50% exact = OK
        assert!(catalog_validate_demo_pages(100, 51).is_err());  // 51% > 50% = RM-07
        assert!(catalog_validate_demo_pages(10, 0).is_err());    // 0 < 1 = erreur
    }

    #[test]
    fn test_payment_apply_promotion() {
        let promo = Promotion { discount_type: DiscountType::Percent, discount_value: 20, .. };
        assert_eq!(payment_apply_promotion(1000, &promo), 800);  // -20%
    }
}
```

### 11.2 Tests d'integration

Tests d'integration pour les flux complets (publication → lecture → achat → telechargement) avec base de donnees temporaire.

---

## 12. References

| Document | Role |
|----------|------|
| [JayManga - Document Fondateur](./JayManga%20-%20Document%20Fondateur.md) | Vision, scope, principes directeurs. |
| [JayManga - Plan Implementation](./JayManga%20-%20Plan%20Implementation.md) | Phases, ordre, dependances, jalons. |
| [JayManga - Publication et Catalogue](./JayManga%20-%20Publication%20et%20Catalogue.md) | Specification import, formats, optimisation. |
| [JayManga - Lecture et Liseuse](./JayManga%20-%20Lecture%20et%20Liseuse.md) | Specification liseuse et modes de lecture. |
| [JayManga - Achat et Paiement](./JayManga%20-%20Achat%20et%20Paiement.md) | Specification paiement et licences. |
| [JayManga - Favoris et Bibliotheque](./JayManga%20-%20Favoris%20et%20Bibliotheque.md) | Specification favoris, telechargement, presence. |
| [JayManga - Portail Agrege et Decouverte](./JayManga%20-%20Portail%20Agrege%20et%20Decouverte.md) | Specification aggregation inter-COG. |
| [JayManga - Onboarding Miou et Gamification](./JayManga%20-%20Onboarding%20Miou%20et%20Gamification.md) | Specification XP, niveaux, streaks, badges, Miou. |
| [JayManga - UI Central et Stable](./JayManga%20-%20UI%20Central%20et%20Stable.md) | Specification UI Dioxus natif. |
| [JayManga - UI Mobile Terminal](./JayManga%20-%20UI%20Mobile%20Terminal.md) | Specification UI mobile. |
| [JayManga - UI Web Portal](./JayManga%20-%20UI%20Web%20Portal.md) | Specification UI web. |
| [MWS - Manifestes de Services](../../miyukini-webway-system/protocole/MWS%20-%20Manifestes%20de%20Services.md) | Protocole de manifestes (schema JayManga). |

---

**Document** : JayManga — Guide d'implementation
**Version** : 1.0
**Date** : 2026-02-24
**Statut** : Guide technique d'implementation — structure crate, types, modules, APIs, patterns.
