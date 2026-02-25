-- JayManga — Schéma de base de données
-- Service de lecture et vente de manga en ligne sous gouvernance COG.
--
-- Groupes de tables :
--   Catalogue  : works, chapters, pages, series, seller_config, optimization_config
--   Paiement   : purchase_licenses, payment_transactions, promotions, cart_items
--   Lecteur    : reader_favorites, reader_progression, reader_badges
--   Agrégation : aggregated_catalog_entries, indexed_sellers, aggregator_config

-- ============================================================
-- CATALOGUE (COG vendeur)
-- ============================================================

CREATE TABLE IF NOT EXISTS works (
    id                TEXT PRIMARY KEY,
    series_id         TEXT,
    title             TEXT NOT NULL,
    authors           TEXT,            -- JSON
    genres            TEXT,            -- JSON
    synopsis          TEXT,
    cover_image_path  TEXT,
    language          TEXT DEFAULT 'fr',
    volume_number     INTEGER,
    status            TEXT DEFAULT 'draft',
    pricing_model     TEXT DEFAULT 'free',
    price             INTEGER DEFAULT 0,
    currency          TEXT DEFAULT 'EUR',
    demo_pages_count  INTEGER DEFAULT 5,
    reading_format    TEXT DEFAULT 'manga',
    allow_download    INTEGER DEFAULT 0,
    total_pages       INTEGER DEFAULT 0,
    tags              TEXT,            -- JSON
    created_at        TEXT,
    updated_at        TEXT,
    FOREIGN KEY (series_id) REFERENCES series(id)
);

CREATE TABLE IF NOT EXISTS chapters (
    id              TEXT PRIMARY KEY,
    work_id         TEXT NOT NULL,
    chapter_number  INTEGER NOT NULL,
    title           TEXT,
    page_count      INTEGER DEFAULT 0,
    sort_order      INTEGER DEFAULT 0,
    created_at      TEXT,
    FOREIGN KEY (work_id) REFERENCES works(id)
);

CREATE TABLE IF NOT EXISTS pages (
    id                   TEXT PRIMARY KEY,
    chapter_id           TEXT NOT NULL,
    page_number          INTEGER NOT NULL,
    original_image_path  TEXT,
    optimized_variants   TEXT,         -- JSON Vec<ImageVariant>
    width                INTEGER,
    height               INTEGER,
    file_size            INTEGER,
    optimization_status  TEXT DEFAULT 'pending',
    sort_order           INTEGER DEFAULT 0,
    FOREIGN KEY (chapter_id) REFERENCES chapters(id)
);

CREATE TABLE IF NOT EXISTS series (
    id                TEXT PRIMARY KEY,
    title             TEXT NOT NULL,
    synopsis          TEXT,
    cover_image_path  TEXT,
    status            TEXT DEFAULT 'ongoing',
    created_at        TEXT,
    updated_at        TEXT
);

CREATE TABLE IF NOT EXISTS seller_config (
    id                          TEXT PRIMARY KEY,
    shop_name                   TEXT,
    shop_description            TEXT,
    default_demo_pages          INTEGER DEFAULT 5,
    default_allow_download      INTEGER DEFAULT 0,
    accepted_payment_methods    TEXT,       -- JSON
    currency                    TEXT DEFAULT 'EUR',
    reading_direction           TEXT DEFAULT 'rtl',
    theme                       TEXT,       -- JSON
    allow_aggregation           INTEGER DEFAULT 1,
    federation_synopsis_length  INTEGER DEFAULT 300,
    federation_include_prices   INTEGER DEFAULT 1,
    payment_gateway             TEXT,
    gateway_config              TEXT,       -- JSON chiffré (niveau 3)
    created_at                  TEXT,
    updated_at                  TEXT
);

CREATE TABLE IF NOT EXISTS optimization_config (
    id                   INTEGER PRIMARY KEY DEFAULT 1,
    quality_hd           INTEGER DEFAULT 85,
    quality_sd           INTEGER DEFAULT 80,
    quality_mobile       INTEGER DEFAULT 75,
    quality_thumb        INTEGER DEFAULT 70,
    output_format        TEXT DEFAULT 'webp',
    generate_avif        INTEGER DEFAULT 0,
    active_profiles      TEXT DEFAULT '["hd","sd","mobile","thumb"]',
    max_concurrent_jobs  INTEGER DEFAULT 2,
    jpeg_fallback        INTEGER DEFAULT 0
);

-- ============================================================
-- PAIEMENT (COG vendeur)
-- ============================================================

CREATE TABLE IF NOT EXISTS purchase_licenses (
    id                TEXT PRIMARY KEY,
    buyer_cog_id      TEXT NOT NULL,
    buyer_identity    TEXT,
    work_id           TEXT,
    purchase_type     TEXT DEFAULT 'work',
    target_id         TEXT NOT NULL,
    amount_paid       INTEGER DEFAULT 0,
    currency          TEXT DEFAULT 'EUR',
    payment_method    TEXT,
    download_allowed  INTEGER DEFAULT 0,
    status            TEXT DEFAULT 'active',
    purchased_at      TEXT,
    refunded_at       TEXT,
    FOREIGN KEY (work_id) REFERENCES works(id)
);

CREATE TABLE IF NOT EXISTS payment_transactions (
    id              TEXT PRIMARY KEY,
    license_id      TEXT,
    buyer_cog_id    TEXT NOT NULL,
    amount          INTEGER DEFAULT 0,
    currency        TEXT DEFAULT 'EUR',
    method          TEXT,
    status          TEXT DEFAULT 'pending',
    external_ref    TEXT,
    created_at      TEXT,
    completed_at    TEXT,
    FOREIGN KEY (license_id) REFERENCES purchase_licenses(id)
);

CREATE TABLE IF NOT EXISTS promotions (
    id              TEXT PRIMARY KEY,
    name            TEXT,
    discount_type   TEXT DEFAULT 'percent',
    discount_value  INTEGER DEFAULT 0,
    target_scope    TEXT DEFAULT 'work',
    target_ids      TEXT,              -- JSON
    start_date      TEXT,
    end_date        TEXT,
    active          INTEGER DEFAULT 1
);

CREATE TABLE IF NOT EXISTS cart_items (
    id               TEXT PRIMARY KEY,
    buyer_cog_id     TEXT NOT NULL,
    work_id          TEXT,
    purchase_type    TEXT DEFAULT 'work',
    target_id        TEXT NOT NULL,
    unit_price       INTEGER DEFAULT 0,
    currency         TEXT DEFAULT 'EUR',
    promotion_id     TEXT,
    discounted_price INTEGER,
    added_at         TEXT,
    FOREIGN KEY (work_id) REFERENCES works(id),
    FOREIGN KEY (promotion_id) REFERENCES promotions(id)
);

-- ============================================================
-- LECTEUR (COG lecteur)
-- ============================================================

CREATE TABLE IF NOT EXISTS reader_favorites (
    id                 TEXT PRIMARY KEY,
    seller_cog_id      TEXT NOT NULL,
    work_id            TEXT NOT NULL,
    cached_title       TEXT,
    cached_cover_url   TEXT,
    cached_authors     TEXT,           -- JSON
    cached_format      TEXT,
    purchase_status    TEXT DEFAULT 'demo',
    last_read_chapter  INTEGER,
    last_read_page     INTEGER,
    reading_progress   REAL DEFAULT 0.0,
    added_at           TEXT,
    last_synced_at     TEXT
);

CREATE TABLE IF NOT EXISTS reader_progression (
    id                        TEXT PRIMARY KEY,
    total_xp                  INTEGER DEFAULT 0,
    current_level             INTEGER DEFAULT 1,
    current_streak            INTEGER DEFAULT 0,
    longest_streak            INTEGER DEFAULT 0,
    streak_shield_available   INTEGER DEFAULT 1,
    last_read_date            TEXT,
    total_pages_read          INTEGER DEFAULT 0,
    total_works_completed     INTEGER DEFAULT 0,
    total_chapters_completed  INTEGER DEFAULT 0,
    genres_explored           TEXT DEFAULT '[]',
    formats_explored          TEXT DEFAULT '[]',
    cogs_visited              TEXT DEFAULT '[]',
    languages_read            TEXT DEFAULT '[]',
    onboarding_completed      INTEGER DEFAULT 0,
    created_at                TEXT,
    updated_at                TEXT
);

CREATE TABLE IF NOT EXISTS reader_badges (
    id              TEXT PRIMARY KEY,
    badge_id        TEXT NOT NULL,
    badge_name      TEXT,
    badge_category  TEXT DEFAULT 'reading',
    earned_at       TEXT
);

-- ============================================================
-- AGRÉGATION (COG agrégateur)
-- ============================================================

CREATE TABLE IF NOT EXISTS aggregated_catalog_entries (
    id                TEXT PRIMARY KEY,
    seller_cog_id     TEXT NOT NULL,
    work_id           TEXT NOT NULL,
    title             TEXT,
    authors           TEXT,
    genres            TEXT,
    synopsis          TEXT,
    cover_thumb_path  TEXT,
    reading_format    TEXT,
    pricing_model     TEXT,
    price             INTEGER,
    currency          TEXT,
    chapter_count     INTEGER,
    total_pages       INTEGER,
    demo_pages_count  INTEGER,
    series_title      TEXT,
    tags              TEXT,
    language          TEXT,
    portal_url        TEXT,
    published_at      TEXT,
    updated_at        TEXT,
    cached_at         TEXT
);

CREATE TABLE IF NOT EXISTS indexed_sellers (
    cog_id               TEXT PRIMARY KEY,
    shop_name            TEXT,
    shop_description     TEXT,
    avatar_path          TEXT,
    work_count           INTEGER DEFAULT 0,
    online_status        TEXT DEFAULT 'unknown',
    last_synced_at       TEXT,
    last_seen_online_at  TEXT,
    blocked              INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS aggregator_config (
    id                        INTEGER PRIMARY KEY DEFAULT 1,
    enabled                   INTEGER DEFAULT 0,
    name                      TEXT DEFAULT 'Portail Agrégé JayManga',
    sync_interval_minutes     INTEGER DEFAULT 30,
    presence_refresh_minutes  INTEGER DEFAULT 5,
    max_indexed_cogs          INTEGER DEFAULT 500,
    show_offline_works        INTEGER DEFAULT 1,
    highlight_own_catalog     INTEGER DEFAULT 1,
    blocked_cogs              TEXT DEFAULT '[]'
);

-- Index pour les requêtes fréquentes
CREATE INDEX IF NOT EXISTS idx_works_status ON works(status);
CREATE INDEX IF NOT EXISTS idx_works_series ON works(series_id);
CREATE INDEX IF NOT EXISTS idx_works_format ON works(reading_format);
CREATE INDEX IF NOT EXISTS idx_chapters_work ON chapters(work_id);
CREATE INDEX IF NOT EXISTS idx_pages_chapter ON pages(chapter_id);
CREATE INDEX IF NOT EXISTS idx_licenses_buyer ON purchase_licenses(buyer_cog_id);
CREATE INDEX IF NOT EXISTS idx_licenses_target ON purchase_licenses(target_id);
CREATE INDEX IF NOT EXISTS idx_transactions_license ON payment_transactions(license_id);
CREATE INDEX IF NOT EXISTS idx_favorites_seller ON reader_favorites(seller_cog_id);
CREATE INDEX IF NOT EXISTS idx_aggregated_seller ON aggregated_catalog_entries(seller_cog_id);
CREATE INDEX IF NOT EXISTS idx_aggregated_genre ON aggregated_catalog_entries(genres);
