//! Implémentation SQLite directe (feature `legacy-sqlite`) pour JayManga.
//!
//! Utilise rusqlite avec `Mutex<Connection>` pour la thread-safety.
//! Identité KindMother : `InstanceType::Daughter` (base locale).

use std::path::Path;
use std::sync::Mutex;

use rusqlite::{params, Connection, OptionalExtension};

use kindmother::{InstanceIdentity, InstanceType};

use super::types::*;
use super::types_payment::*;
use super::types_reader::*;
use super::types_aggregator::*;

// ---------------------------------------------------------------------------
// Erreur
// ---------------------------------------------------------------------------

/// Erreur de la couche persistance JayManga.
#[derive(Debug)]
pub struct DbError(pub String);

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JayMangaDb error: {}", self.0)
    }
}

impl std::error::Error for DbError {}

impl From<rusqlite::Error> for DbError {
    fn from(e: rusqlite::Error) -> Self {
        Self(e.to_string())
    }
}

// ---------------------------------------------------------------------------
// JayMangaDb
// ---------------------------------------------------------------------------

/// Base de données JayManga (SQLite locale via KindMother).
pub struct JayMangaDb {
    conn: Mutex<Connection>,
    /// Identité KindMother : instance Fille (base locale).
    pub instance: InstanceIdentity,
}

impl JayMangaDb {
    /// Ouvre (ou crée) la base de données JayManga.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let path = path.as_ref();
        let conn = Connection::open(path)?;

        #[cfg(feature = "db-encryption")]
        {
            let data_dir = path.parent().unwrap_or(path);
            let db_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("jaymanga");
            let kd = kindmother_db_key::KeyDerivation::new(data_dir)
                .map_err(|e| DbError(e.0))?;
            let pragma_key = kd.pragma_key_hex(db_name).map_err(|e| DbError(e.0))?;
            conn.pragma_update(None, "key", &pragma_key)?;
        }

        let instance = InstanceIdentity::new(InstanceType::Daughter);
        let db = Self {
            conn: Mutex::new(conn),
            instance,
        };
        db.init_schema()?;
        Ok(db)
    }

    /// Initialise le schéma (toutes les tables).
    fn init_schema(&self) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute_batch(include_str!("schema.sql"))?;
        Ok(())
    }

    // ====================================================================
    // WORKS
    // ====================================================================

    /// Liste les oeuvres avec filtres optionnels.
    pub fn work_list(&self, filters: &WorkFilters) -> Result<Vec<Work>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut sql = "SELECT id, series_id, title, authors, genres, synopsis, \
            cover_image_path, language, volume_number, status, pricing_model, price, \
            currency, demo_pages_count, reading_format, allow_download, total_pages, \
            tags, created_at, updated_at FROM works WHERE 1=1".to_string();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(ref status) = filters.status {
            sql.push_str(" AND status = ?");
            param_values.push(Box::new(status.clone()));
        }
        if let Some(ref fmt) = filters.reading_format {
            sql.push_str(" AND reading_format = ?");
            param_values.push(Box::new(fmt.clone()));
        }
        if let Some(ref pm) = filters.pricing_model {
            sql.push_str(" AND pricing_model = ?");
            param_values.push(Box::new(pm.clone()));
        }
        if let Some(ref sid) = filters.series_id {
            sql.push_str(" AND series_id = ?");
            param_values.push(Box::new(sid.clone()));
        }
        if let Some(ref q) = filters.search {
            sql.push_str(" AND (title LIKE ? OR tags LIKE ?)");
            let pattern = format!("%{q}%");
            param_values.push(Box::new(pattern.clone()));
            param_values.push(Box::new(pattern));
        }

        sql.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = filters.limit {
            sql.push_str(&format!(" LIMIT {limit}"));
        }
        if let Some(offset) = filters.offset {
            sql.push_str(&format!(" OFFSET {offset}"));
        }

        let params_ref: Vec<&dyn rusqlite::types::ToSql> =
            param_values.iter().map(|p| p.as_ref()).collect();
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params_ref.as_slice(), |row| {
            Ok(Work {
                id: row.get(0)?,
                series_id: row.get(1)?,
                title: row.get(2)?,
                authors: row.get(3)?,
                genres: row.get(4)?,
                synopsis: row.get(5)?,
                cover_image_path: row.get(6)?,
                language: row.get(7)?,
                volume_number: row.get(8)?,
                status: row.get(9)?,
                pricing_model: row.get(10)?,
                price: row.get(11)?,
                currency: row.get(12)?,
                demo_pages_count: row.get(13)?,
                reading_format: row.get(14)?,
                allow_download: row.get(15)?,
                total_pages: row.get(16)?,
                tags: row.get(17)?,
                created_at: row.get(18)?,
                updated_at: row.get(19)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Récupère une oeuvre par ID.
    pub fn work_by_id(&self, id: &str) -> Result<Option<Work>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let result = conn
            .query_row(
                "SELECT id, series_id, title, authors, genres, synopsis, \
                 cover_image_path, language, volume_number, status, pricing_model, price, \
                 currency, demo_pages_count, reading_format, allow_download, total_pages, \
                 tags, created_at, updated_at FROM works WHERE id = ?1",
                params![id],
                |row| {
                    Ok(Work {
                        id: row.get(0)?,
                        series_id: row.get(1)?,
                        title: row.get(2)?,
                        authors: row.get(3)?,
                        genres: row.get(4)?,
                        synopsis: row.get(5)?,
                        cover_image_path: row.get(6)?,
                        language: row.get(7)?,
                        volume_number: row.get(8)?,
                        status: row.get(9)?,
                        pricing_model: row.get(10)?,
                        price: row.get(11)?,
                        currency: row.get(12)?,
                        demo_pages_count: row.get(13)?,
                        reading_format: row.get(14)?,
                        allow_download: row.get(15)?,
                        total_pages: row.get(16)?,
                        tags: row.get(17)?,
                        created_at: row.get(18)?,
                        updated_at: row.get(19)?,
                    })
                },
            )
            .optional()?;
        Ok(result)
    }

    /// Crée une oeuvre.
    pub fn work_create(&self, work: &Work) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO works (id, series_id, title, authors, genres, synopsis, \
             cover_image_path, language, volume_number, status, pricing_model, price, \
             currency, demo_pages_count, reading_format, allow_download, total_pages, \
             tags, created_at, updated_at) \
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20)",
            params![
                work.id, work.series_id, work.title, work.authors, work.genres,
                work.synopsis, work.cover_image_path, work.language, work.volume_number,
                work.status, work.pricing_model, work.price, work.currency,
                work.demo_pages_count, work.reading_format, work.allow_download,
                work.total_pages, work.tags, work.created_at, work.updated_at,
            ],
        )?;
        Ok(())
    }

    /// Met à jour une oeuvre.
    pub fn work_update(&self, work: &Work) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "UPDATE works SET series_id=?2, title=?3, authors=?4, genres=?5, \
             synopsis=?6, cover_image_path=?7, language=?8, volume_number=?9, \
             status=?10, pricing_model=?11, price=?12, currency=?13, \
             demo_pages_count=?14, reading_format=?15, allow_download=?16, \
             total_pages=?17, tags=?18, updated_at=?19 WHERE id=?1",
            params![
                work.id, work.series_id, work.title, work.authors, work.genres,
                work.synopsis, work.cover_image_path, work.language, work.volume_number,
                work.status, work.pricing_model, work.price, work.currency,
                work.demo_pages_count, work.reading_format, work.allow_download,
                work.total_pages, work.tags, work.updated_at,
            ],
        )?;
        Ok(())
    }

    /// Supprime une oeuvre.
    pub fn work_delete(&self, id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("DELETE FROM works WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Compte le nombre total d'oeuvres publiées.
    pub fn work_count(&self) -> Result<i64, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM works WHERE status = 'published'",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    // ====================================================================
    // CHAPTERS
    // ====================================================================

    /// Liste les chapitres d'une oeuvre.
    pub fn chapter_list_by_work(&self, work_id: &str) -> Result<Vec<Chapter>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, work_id, chapter_number, title, page_count, sort_order, created_at \
             FROM chapters WHERE work_id = ?1 ORDER BY sort_order ASC",
        )?;
        let rows = stmt.query_map(params![work_id], |row| {
            Ok(Chapter {
                id: row.get(0)?,
                work_id: row.get(1)?,
                chapter_number: row.get(2)?,
                title: row.get(3)?,
                page_count: row.get(4)?,
                sort_order: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Crée un chapitre.
    pub fn chapter_create(&self, chapter: &Chapter) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO chapters (id, work_id, chapter_number, title, page_count, sort_order, created_at) \
             VALUES (?1,?2,?3,?4,?5,?6,?7)",
            params![
                chapter.id, chapter.work_id, chapter.chapter_number,
                chapter.title, chapter.page_count, chapter.sort_order, chapter.created_at,
            ],
        )?;
        Ok(())
    }

    /// Met à jour un chapitre.
    pub fn chapter_update(&self, chapter: &Chapter) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "UPDATE chapters SET chapter_number=?2, title=?3, page_count=?4, sort_order=?5 \
             WHERE id=?1",
            params![
                chapter.id, chapter.chapter_number, chapter.title,
                chapter.page_count, chapter.sort_order,
            ],
        )?;
        Ok(())
    }

    /// Supprime un chapitre.
    pub fn chapter_delete(&self, id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("DELETE FROM pages WHERE chapter_id = ?1", params![id])?;
        conn.execute("DELETE FROM chapters WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Réordonne les chapitres d'une oeuvre.
    pub fn chapter_reorder(&self, _work_id: &str, order: &[(String, i32)]) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        for (id, sort) in order {
            conn.execute(
                "UPDATE chapters SET sort_order = ?2 WHERE id = ?1",
                params![id, sort],
            )?;
        }
        Ok(())
    }

    // ====================================================================
    // PAGES
    // ====================================================================

    /// Liste les pages d'un chapitre.
    pub fn page_list_by_chapter(&self, chapter_id: &str) -> Result<Vec<Page>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, chapter_id, page_number, original_image_path, optimized_variants, \
             width, height, file_size, optimization_status, sort_order \
             FROM pages WHERE chapter_id = ?1 ORDER BY sort_order ASC",
        )?;
        let rows = stmt.query_map(params![chapter_id], |row| {
            Ok(Page {
                id: row.get(0)?,
                chapter_id: row.get(1)?,
                page_number: row.get(2)?,
                original_image_path: row.get(3)?,
                optimized_variants: row.get(4)?,
                width: row.get(5)?,
                height: row.get(6)?,
                file_size: row.get(7)?,
                optimization_status: row.get(8)?,
                sort_order: row.get(9)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Crée une page.
    pub fn page_create(&self, page: &Page) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO pages (id, chapter_id, page_number, original_image_path, \
             optimized_variants, width, height, file_size, optimization_status, sort_order) \
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            params![
                page.id, page.chapter_id, page.page_number, page.original_image_path,
                page.optimized_variants, page.width, page.height, page.file_size,
                page.optimization_status, page.sort_order,
            ],
        )?;
        Ok(())
    }

    /// Met à jour les variantes optimisées d'une page.
    pub fn page_update_variants(
        &self,
        page_id: &str,
        variants_json: &str,
        status: &str,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "UPDATE pages SET optimized_variants=?2, optimization_status=?3 WHERE id=?1",
            params![page_id, variants_json, status],
        )?;
        Ok(())
    }

    /// Supprime une page.
    pub fn page_delete(&self, id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("DELETE FROM pages WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Réordonne les pages d'un chapitre.
    pub fn page_reorder(&self, _chapter_id: &str, order: &[(String, i32)]) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        for (id, sort) in order {
            conn.execute(
                "UPDATE pages SET sort_order = ?2 WHERE id = ?1",
                params![id, sort],
            )?;
        }
        Ok(())
    }

    // ====================================================================
    // SERIES
    // ====================================================================

    /// Liste toutes les séries.
    pub fn series_list(&self) -> Result<Vec<Series>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, synopsis, cover_image_path, status, created_at, updated_at \
             FROM series ORDER BY title ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Series {
                id: row.get(0)?,
                title: row.get(1)?,
                synopsis: row.get(2)?,
                cover_image_path: row.get(3)?,
                status: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Récupère une série par ID.
    pub fn series_by_id(&self, id: &str) -> Result<Option<Series>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let result = conn
            .query_row(
                "SELECT id, title, synopsis, cover_image_path, status, created_at, updated_at \
                 FROM series WHERE id = ?1",
                params![id],
                |row| {
                    Ok(Series {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        synopsis: row.get(2)?,
                        cover_image_path: row.get(3)?,
                        status: row.get(4)?,
                        created_at: row.get(5)?,
                        updated_at: row.get(6)?,
                    })
                },
            )
            .optional()?;
        Ok(result)
    }

    /// Crée une série.
    pub fn series_create(&self, series: &Series) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO series (id, title, synopsis, cover_image_path, status, created_at, updated_at) \
             VALUES (?1,?2,?3,?4,?5,?6,?7)",
            params![
                series.id, series.title, series.synopsis, series.cover_image_path,
                series.status, series.created_at, series.updated_at,
            ],
        )?;
        Ok(())
    }

    /// Met à jour une série.
    pub fn series_update(&self, series: &Series) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "UPDATE series SET title=?2, synopsis=?3, cover_image_path=?4, status=?5, \
             updated_at=?6 WHERE id=?1",
            params![
                series.id, series.title, series.synopsis, series.cover_image_path,
                series.status, series.updated_at,
            ],
        )?;
        Ok(())
    }

    // ====================================================================
    // SELLER CONFIG
    // ====================================================================

    /// Récupère la configuration vendeur (unique).
    pub fn seller_config_get(&self) -> Result<Option<SellerConfig>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let result = conn
            .query_row(
                "SELECT id, shop_name, shop_description, default_demo_pages, \
                 default_allow_download, accepted_payment_methods, currency, \
                 reading_direction, theme, allow_aggregation, federation_synopsis_length, \
                 federation_include_prices, payment_gateway, gateway_config, \
                 created_at, updated_at FROM seller_config LIMIT 1",
                [],
                |row| {
                    Ok(SellerConfig {
                        id: row.get(0)?,
                        shop_name: row.get(1)?,
                        shop_description: row.get(2)?,
                        default_demo_pages: row.get(3)?,
                        default_allow_download: row.get(4)?,
                        accepted_payment_methods: row.get(5)?,
                        currency: row.get(6)?,
                        reading_direction: row.get(7)?,
                        theme: row.get(8)?,
                        allow_aggregation: row.get(9)?,
                        federation_synopsis_length: row.get(10)?,
                        federation_include_prices: row.get(11)?,
                        payment_gateway: row.get(12)?,
                        gateway_config: row.get(13)?,
                        created_at: row.get(14)?,
                        updated_at: row.get(15)?,
                    })
                },
            )
            .optional()?;
        Ok(result)
    }

    /// Crée ou met à jour la configuration vendeur (upsert).
    pub fn seller_config_upsert(&self, config: &SellerConfig) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO seller_config (id, shop_name, shop_description, default_demo_pages, \
             default_allow_download, accepted_payment_methods, currency, reading_direction, \
             theme, allow_aggregation, federation_synopsis_length, federation_include_prices, \
             payment_gateway, gateway_config, created_at, updated_at) \
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16) \
             ON CONFLICT(id) DO UPDATE SET \
             shop_name=excluded.shop_name, shop_description=excluded.shop_description, \
             default_demo_pages=excluded.default_demo_pages, \
             default_allow_download=excluded.default_allow_download, \
             accepted_payment_methods=excluded.accepted_payment_methods, \
             currency=excluded.currency, reading_direction=excluded.reading_direction, \
             theme=excluded.theme, allow_aggregation=excluded.allow_aggregation, \
             federation_synopsis_length=excluded.federation_synopsis_length, \
             federation_include_prices=excluded.federation_include_prices, \
             payment_gateway=excluded.payment_gateway, gateway_config=excluded.gateway_config, \
             updated_at=excluded.updated_at",
            params![
                config.id, config.shop_name, config.shop_description,
                config.default_demo_pages, config.default_allow_download,
                config.accepted_payment_methods, config.currency,
                config.reading_direction, config.theme, config.allow_aggregation,
                config.federation_synopsis_length, config.federation_include_prices,
                config.payment_gateway, config.gateway_config,
                config.created_at, config.updated_at,
            ],
        )?;
        Ok(())
    }

    // ====================================================================
    // OPTIMIZATION CONFIG
    // ====================================================================

    /// Récupère la configuration d'optimisation.
    pub fn optimization_config_get(&self) -> Result<OptimizationConfig, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let result = conn
            .query_row(
                "SELECT quality_hd, quality_sd, quality_mobile, quality_thumb, \
                 output_format, generate_avif, active_profiles, max_concurrent_jobs, \
                 jpeg_fallback FROM optimization_config WHERE id = 1",
                [],
                |row| {
                    Ok(OptimizationConfig {
                        quality_hd: row.get(0)?,
                        quality_sd: row.get(1)?,
                        quality_mobile: row.get(2)?,
                        quality_thumb: row.get(3)?,
                        output_format: row.get(4)?,
                        generate_avif: row.get(5)?,
                        active_profiles: row.get(6)?,
                        max_concurrent_jobs: row.get(7)?,
                        jpeg_fallback: row.get(8)?,
                    })
                },
            )
            .optional()?;
        Ok(result.unwrap_or_else(OptimizationConfig::defaults))
    }

    /// Crée ou met à jour la configuration d'optimisation.
    pub fn optimization_config_upsert(&self, config: &OptimizationConfig) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO optimization_config (id, quality_hd, quality_sd, quality_mobile, \
             quality_thumb, output_format, generate_avif, active_profiles, max_concurrent_jobs, \
             jpeg_fallback) VALUES (1,?1,?2,?3,?4,?5,?6,?7,?8,?9) \
             ON CONFLICT(id) DO UPDATE SET \
             quality_hd=excluded.quality_hd, quality_sd=excluded.quality_sd, \
             quality_mobile=excluded.quality_mobile, quality_thumb=excluded.quality_thumb, \
             output_format=excluded.output_format, generate_avif=excluded.generate_avif, \
             active_profiles=excluded.active_profiles, \
             max_concurrent_jobs=excluded.max_concurrent_jobs, \
             jpeg_fallback=excluded.jpeg_fallback",
            params![
                config.quality_hd, config.quality_sd, config.quality_mobile,
                config.quality_thumb, config.output_format, config.generate_avif,
                config.active_profiles, config.max_concurrent_jobs, config.jpeg_fallback,
            ],
        )?;
        Ok(())
    }

    // ====================================================================
    // LICENSES
    // ====================================================================

    /// Crée une licence d'achat.
    pub fn license_create(&self, license: &PurchaseLicense) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO purchase_licenses (id, buyer_cog_id, buyer_identity, work_id, \
             purchase_type, target_id, amount_paid, currency, payment_method, \
             download_allowed, status, purchased_at, refunded_at) \
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)",
            params![
                license.id, license.buyer_cog_id, license.buyer_identity,
                license.work_id, license.purchase_type, license.target_id,
                license.amount_paid, license.currency, license.payment_method,
                license.download_allowed, license.status,
                license.purchased_at, license.refunded_at,
            ],
        )?;
        Ok(())
    }

    /// Recherche une licence par acheteur et cible.
    pub fn license_by_buyer_and_target(
        &self,
        buyer_cog_id: &str,
        target_id: &str,
    ) -> Result<Option<PurchaseLicense>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let result = conn
            .query_row(
                "SELECT id, buyer_cog_id, buyer_identity, work_id, purchase_type, target_id, \
                 amount_paid, currency, payment_method, download_allowed, status, \
                 purchased_at, refunded_at \
                 FROM purchase_licenses WHERE buyer_cog_id=?1 AND target_id=?2 AND status='active' \
                 LIMIT 1",
                params![buyer_cog_id, target_id],
                |row| {
                    Ok(PurchaseLicense {
                        id: row.get(0)?,
                        buyer_cog_id: row.get(1)?,
                        buyer_identity: row.get(2)?,
                        work_id: row.get(3)?,
                        purchase_type: row.get(4)?,
                        target_id: row.get(5)?,
                        amount_paid: row.get(6)?,
                        currency: row.get(7)?,
                        payment_method: row.get(8)?,
                        download_allowed: row.get(9)?,
                        status: row.get(10)?,
                        purchased_at: row.get(11)?,
                        refunded_at: row.get(12)?,
                    })
                },
            )
            .optional()?;
        Ok(result)
    }

    /// Liste les licences d'un acheteur.
    pub fn license_list_by_buyer(&self, buyer_cog_id: &str) -> Result<Vec<PurchaseLicense>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, buyer_cog_id, buyer_identity, work_id, purchase_type, target_id, \
             amount_paid, currency, payment_method, download_allowed, status, \
             purchased_at, refunded_at \
             FROM purchase_licenses WHERE buyer_cog_id=?1 ORDER BY purchased_at DESC",
        )?;
        let rows = stmt.query_map(params![buyer_cog_id], |row| {
            Ok(PurchaseLicense {
                id: row.get(0)?,
                buyer_cog_id: row.get(1)?,
                buyer_identity: row.get(2)?,
                work_id: row.get(3)?,
                purchase_type: row.get(4)?,
                target_id: row.get(5)?,
                amount_paid: row.get(6)?,
                currency: row.get(7)?,
                payment_method: row.get(8)?,
                download_allowed: row.get(9)?,
                status: row.get(10)?,
                purchased_at: row.get(11)?,
                refunded_at: row.get(12)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Liste toutes les licences (vue vendeur).
    pub fn license_list_all(&self) -> Result<Vec<PurchaseLicense>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, buyer_cog_id, buyer_identity, work_id, purchase_type, target_id, \
             amount_paid, currency, payment_method, download_allowed, status, \
             purchased_at, refunded_at \
             FROM purchase_licenses ORDER BY purchased_at DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(PurchaseLicense {
                id: row.get(0)?,
                buyer_cog_id: row.get(1)?,
                buyer_identity: row.get(2)?,
                work_id: row.get(3)?,
                purchase_type: row.get(4)?,
                target_id: row.get(5)?,
                amount_paid: row.get(6)?,
                currency: row.get(7)?,
                payment_method: row.get(8)?,
                download_allowed: row.get(9)?,
                status: row.get(10)?,
                purchased_at: row.get(11)?,
                refunded_at: row.get(12)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Met à jour le statut d'une licence.
    pub fn license_update_status(&self, id: &str, status: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "UPDATE purchase_licenses SET status=?2 WHERE id=?1",
            params![id, status],
        )?;
        Ok(())
    }

    // ====================================================================
    // TRANSACTIONS
    // ====================================================================

    /// Crée une transaction de paiement.
    pub fn transaction_create(&self, tx: &PaymentTransaction) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO payment_transactions (id, license_id, buyer_cog_id, amount, \
             currency, method, status, external_ref, created_at, completed_at) \
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            params![
                tx.id, tx.license_id, tx.buyer_cog_id, tx.amount, tx.currency,
                tx.method, tx.status, tx.external_ref, tx.created_at, tx.completed_at,
            ],
        )?;
        Ok(())
    }

    /// Liste les transactions avec filtres optionnels.
    pub fn transaction_list(&self, filters: &TransactionFilters) -> Result<Vec<PaymentTransaction>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut sql = "SELECT id, license_id, buyer_cog_id, amount, currency, method, \
            status, external_ref, created_at, completed_at \
            FROM payment_transactions WHERE 1=1".to_string();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(ref status) = filters.status {
            sql.push_str(" AND status = ?");
            param_values.push(Box::new(status.clone()));
        }
        if let Some(ref method) = filters.method {
            sql.push_str(" AND method = ?");
            param_values.push(Box::new(method.clone()));
        }
        if let Some(ref from) = filters.from_date {
            sql.push_str(" AND created_at >= ?");
            param_values.push(Box::new(from.clone()));
        }
        if let Some(ref to) = filters.to_date {
            sql.push_str(" AND created_at <= ?");
            param_values.push(Box::new(to.clone()));
        }

        sql.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = filters.limit {
            sql.push_str(&format!(" LIMIT {limit}"));
        }
        if let Some(offset) = filters.offset {
            sql.push_str(&format!(" OFFSET {offset}"));
        }

        let params_ref: Vec<&dyn rusqlite::types::ToSql> =
            param_values.iter().map(|p| p.as_ref()).collect();
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params_ref.as_slice(), |row| {
            Ok(PaymentTransaction {
                id: row.get(0)?,
                license_id: row.get(1)?,
                buyer_cog_id: row.get(2)?,
                amount: row.get(3)?,
                currency: row.get(4)?,
                method: row.get(5)?,
                status: row.get(6)?,
                external_ref: row.get(7)?,
                created_at: row.get(8)?,
                completed_at: row.get(9)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Met à jour le statut d'une transaction.
    pub fn transaction_update_status(&self, id: &str, status: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE payment_transactions SET status=?2, completed_at=?3 WHERE id=?1",
            params![id, status, now],
        )?;
        Ok(())
    }

    // ====================================================================
    // PROMOTIONS
    // ====================================================================

    /// Liste les promotions actives.
    pub fn promotion_list_active(&self) -> Result<Vec<Promotion>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let now = chrono::Utc::now().to_rfc3339();
        let mut stmt = conn.prepare(
            "SELECT id, name, discount_type, discount_value, target_scope, target_ids, \
             start_date, end_date, active FROM promotions \
             WHERE active=1 AND start_date <= ?1 AND end_date >= ?1",
        )?;
        let rows = stmt.query_map(params![now], |row| {
            Ok(Promotion {
                id: row.get(0)?,
                name: row.get(1)?,
                discount_type: row.get(2)?,
                discount_value: row.get(3)?,
                target_scope: row.get(4)?,
                target_ids: row.get(5)?,
                start_date: row.get(6)?,
                end_date: row.get(7)?,
                active: row.get(8)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Crée une promotion.
    pub fn promotion_create(&self, promo: &Promotion) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO promotions (id, name, discount_type, discount_value, target_scope, \
             target_ids, start_date, end_date, active) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            params![
                promo.id, promo.name, promo.discount_type, promo.discount_value,
                promo.target_scope, promo.target_ids, promo.start_date,
                promo.end_date, promo.active,
            ],
        )?;
        Ok(())
    }

    // ====================================================================
    // READER FAVORITES (COG lecteur)
    // ====================================================================

    /// Liste les favoris du lecteur.
    pub fn favorite_list(&self) -> Result<Vec<ReaderFavorite>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, seller_cog_id, work_id, cached_title, cached_cover_url, \
             cached_authors, cached_format, purchase_status, last_read_chapter, \
             last_read_page, reading_progress, added_at, last_synced_at \
             FROM reader_favorites ORDER BY added_at DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(ReaderFavorite {
                id: row.get(0)?,
                seller_cog_id: row.get(1)?,
                work_id: row.get(2)?,
                cached_title: row.get(3)?,
                cached_cover_url: row.get(4)?,
                cached_authors: row.get(5)?,
                cached_format: row.get(6)?,
                purchase_status: row.get(7)?,
                last_read_chapter: row.get(8)?,
                last_read_page: row.get(9)?,
                reading_progress: row.get(10)?,
                added_at: row.get(11)?,
                last_synced_at: row.get(12)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Crée un favori.
    pub fn favorite_create(&self, fav: &ReaderFavorite) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO reader_favorites (id, seller_cog_id, work_id, cached_title, \
             cached_cover_url, cached_authors, cached_format, purchase_status, \
             last_read_chapter, last_read_page, reading_progress, added_at, last_synced_at) \
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)",
            params![
                fav.id, fav.seller_cog_id, fav.work_id, fav.cached_title,
                fav.cached_cover_url, fav.cached_authors, fav.cached_format,
                fav.purchase_status, fav.last_read_chapter, fav.last_read_page,
                fav.reading_progress, fav.added_at, fav.last_synced_at,
            ],
        )?;
        Ok(())
    }

    /// Supprime un favori.
    pub fn favorite_delete(&self, id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("DELETE FROM reader_favorites WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Met à jour la progression d'un favori.
    pub fn favorite_update_progress(
        &self,
        id: &str,
        chapter: i32,
        page: i32,
        progress: f64,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "UPDATE reader_favorites SET last_read_chapter=?2, last_read_page=?3, \
             reading_progress=?4 WHERE id=?1",
            params![id, chapter, page, progress],
        )?;
        Ok(())
    }

    /// Met à jour le cache d'un favori (données depuis le COG vendeur distant).
    pub fn favorite_update_cache(
        &self,
        id: &str,
        title: &str,
        cover_url: Option<&str>,
        authors: Option<&str>,
        format: Option<&str>,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE reader_favorites SET cached_title=?2, cached_cover_url=?3, \
             cached_authors=?4, cached_format=?5, last_synced_at=?6 WHERE id=?1",
            params![id, title, cover_url, authors, format, now],
        )?;
        Ok(())
    }

    // ====================================================================
    // CART ITEMS (COG vendeur)
    // ====================================================================

    /// Liste les articles du panier d'un acheteur.
    pub fn cart_list_by_buyer(&self, buyer_cog_id: &str) -> Result<Vec<CartItem>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, buyer_cog_id, work_id, purchase_type, target_id, unit_price, \
             currency, promotion_id, discounted_price, added_at \
             FROM cart_items WHERE buyer_cog_id=?1 ORDER BY added_at DESC",
        )?;
        let rows = stmt.query_map(params![buyer_cog_id], |row| {
            Ok(CartItem {
                id: row.get(0)?,
                buyer_cog_id: row.get(1)?,
                work_id: row.get(2)?,
                purchase_type: row.get(3)?,
                target_id: row.get(4)?,
                unit_price: row.get(5)?,
                currency: row.get(6)?,
                promotion_id: row.get(7)?,
                discounted_price: row.get(8)?,
                added_at: row.get(9)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Ajoute un article au panier.
    pub fn cart_add(&self, item: &CartItem) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO cart_items (id, buyer_cog_id, work_id, purchase_type, target_id, \
             unit_price, currency, promotion_id, discounted_price, added_at) \
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            params![
                item.id, item.buyer_cog_id, item.work_id, item.purchase_type,
                item.target_id, item.unit_price, item.currency, item.promotion_id,
                item.discounted_price, item.added_at,
            ],
        )?;
        Ok(())
    }

    /// Supprime un article du panier.
    pub fn cart_remove(&self, id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("DELETE FROM cart_items WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Vide le panier d'un acheteur.
    pub fn cart_clear(&self, buyer_cog_id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "DELETE FROM cart_items WHERE buyer_cog_id = ?1",
            params![buyer_cog_id],
        )?;
        Ok(())
    }

    // ====================================================================
    // READER PROGRESSION (COG lecteur)
    // ====================================================================

    /// Récupère la progression du lecteur (instance unique).
    pub fn progression_get(&self) -> Result<Option<ReaderProgression>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let result = conn
            .query_row(
                "SELECT id, total_xp, current_level, current_streak, longest_streak, \
                 streak_shield_available, last_read_date, total_pages_read, \
                 total_works_completed, total_chapters_completed, genres_explored, \
                 formats_explored, cogs_visited, languages_read, onboarding_completed, \
                 created_at, updated_at FROM reader_progression LIMIT 1",
                [],
                |row| {
                    Ok(ReaderProgression {
                        id: row.get(0)?,
                        total_xp: row.get(1)?,
                        current_level: row.get(2)?,
                        current_streak: row.get(3)?,
                        longest_streak: row.get(4)?,
                        streak_shield_available: row.get(5)?,
                        last_read_date: row.get(6)?,
                        total_pages_read: row.get(7)?,
                        total_works_completed: row.get(8)?,
                        total_chapters_completed: row.get(9)?,
                        genres_explored: row.get(10)?,
                        formats_explored: row.get(11)?,
                        cogs_visited: row.get(12)?,
                        languages_read: row.get(13)?,
                        onboarding_completed: row.get(14)?,
                        created_at: row.get(15)?,
                        updated_at: row.get(16)?,
                    })
                },
            )
            .optional()?;
        Ok(result)
    }

    /// Crée ou met à jour la progression du lecteur.
    pub fn progression_upsert(&self, prog: &ReaderProgression) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO reader_progression (id, total_xp, current_level, current_streak, \
             longest_streak, streak_shield_available, last_read_date, total_pages_read, \
             total_works_completed, total_chapters_completed, genres_explored, \
             formats_explored, cogs_visited, languages_read, onboarding_completed, \
             created_at, updated_at) \
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17) \
             ON CONFLICT(id) DO UPDATE SET \
             total_xp=excluded.total_xp, current_level=excluded.current_level, \
             current_streak=excluded.current_streak, longest_streak=excluded.longest_streak, \
             streak_shield_available=excluded.streak_shield_available, \
             last_read_date=excluded.last_read_date, total_pages_read=excluded.total_pages_read, \
             total_works_completed=excluded.total_works_completed, \
             total_chapters_completed=excluded.total_chapters_completed, \
             genres_explored=excluded.genres_explored, formats_explored=excluded.formats_explored, \
             cogs_visited=excluded.cogs_visited, languages_read=excluded.languages_read, \
             onboarding_completed=excluded.onboarding_completed, updated_at=excluded.updated_at",
            params![
                prog.id, prog.total_xp, prog.current_level, prog.current_streak,
                prog.longest_streak, prog.streak_shield_available, prog.last_read_date,
                prog.total_pages_read, prog.total_works_completed,
                prog.total_chapters_completed, prog.genres_explored,
                prog.formats_explored, prog.cogs_visited, prog.languages_read,
                prog.onboarding_completed, prog.created_at, prog.updated_at,
            ],
        )?;
        Ok(())
    }

    // ====================================================================
    // READER BADGES (COG lecteur)
    // ====================================================================

    /// Liste les badges du lecteur.
    pub fn badge_list(&self) -> Result<Vec<ReaderBadge>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, badge_id, badge_name, badge_category, earned_at \
             FROM reader_badges ORDER BY earned_at DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(ReaderBadge {
                id: row.get(0)?,
                badge_id: row.get(1)?,
                badge_name: row.get(2)?,
                badge_category: row.get(3)?,
                earned_at: row.get(4)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Crée un badge.
    pub fn badge_create(&self, badge: &ReaderBadge) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO reader_badges (id, badge_id, badge_name, badge_category, earned_at) \
             VALUES (?1,?2,?3,?4,?5)",
            params![badge.id, badge.badge_id, badge.badge_name, badge.badge_category, badge.earned_at],
        )?;
        Ok(())
    }

    // ====================================================================
    // AGGREGATION (COG agrégateur)
    // ====================================================================

    /// Upsert une entrée catalogue agrégée.
    pub fn aggregated_entry_upsert(&self, entry: &AggregatedCatalogEntry) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO aggregated_catalog_entries (id, seller_cog_id, work_id, title, \
             authors, genres, synopsis, cover_thumb_path, reading_format, pricing_model, \
             price, currency, chapter_count, total_pages, demo_pages_count, series_title, \
             tags, language, portal_url, published_at, updated_at, cached_at) \
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22) \
             ON CONFLICT(id) DO UPDATE SET \
             title=excluded.title, authors=excluded.authors, genres=excluded.genres, \
             synopsis=excluded.synopsis, cover_thumb_path=excluded.cover_thumb_path, \
             reading_format=excluded.reading_format, pricing_model=excluded.pricing_model, \
             price=excluded.price, currency=excluded.currency, \
             chapter_count=excluded.chapter_count, total_pages=excluded.total_pages, \
             demo_pages_count=excluded.demo_pages_count, series_title=excluded.series_title, \
             tags=excluded.tags, language=excluded.language, portal_url=excluded.portal_url, \
             published_at=excluded.published_at, updated_at=excluded.updated_at, \
             cached_at=excluded.cached_at",
            params![
                entry.id, entry.seller_cog_id, entry.work_id, entry.title,
                entry.authors, entry.genres, entry.synopsis, entry.cover_thumb_path,
                entry.reading_format, entry.pricing_model, entry.price, entry.currency,
                entry.chapter_count, entry.total_pages, entry.demo_pages_count,
                entry.series_title, entry.tags, entry.language, entry.portal_url,
                entry.published_at, entry.updated_at, entry.cached_at,
            ],
        )?;
        Ok(())
    }

    /// Upsert un vendeur indexé.
    pub fn indexed_seller_upsert(&self, seller: &IndexedSeller) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO indexed_sellers (cog_id, shop_name, shop_description, avatar_path, \
             work_count, online_status, last_synced_at, last_seen_online_at, blocked) \
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9) \
             ON CONFLICT(cog_id) DO UPDATE SET \
             shop_name=excluded.shop_name, shop_description=excluded.shop_description, \
             avatar_path=excluded.avatar_path, work_count=excluded.work_count, \
             online_status=excluded.online_status, last_synced_at=excluded.last_synced_at, \
             last_seen_online_at=excluded.last_seen_online_at, blocked=excluded.blocked",
            params![
                seller.cog_id, seller.shop_name, seller.shop_description,
                seller.avatar_path, seller.work_count, seller.online_status,
                seller.last_synced_at, seller.last_seen_online_at, seller.blocked,
            ],
        )?;
        Ok(())
    }

    /// Recherche un vendeur indexé par ID COG.
    pub fn indexed_seller_by_id(&self, cog_id: &str) -> Result<Option<IndexedSeller>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let result = conn
            .query_row(
                "SELECT cog_id, shop_name, shop_description, avatar_path, work_count, \
                 online_status, last_synced_at, last_seen_online_at, blocked \
                 FROM indexed_sellers WHERE cog_id=?1",
                params![cog_id],
                |row| {
                    Ok(IndexedSeller {
                        cog_id: row.get(0)?,
                        shop_name: row.get(1)?,
                        shop_description: row.get(2)?,
                        avatar_path: row.get(3)?,
                        work_count: row.get(4)?,
                        online_status: row.get(5)?,
                        last_synced_at: row.get(6)?,
                        last_seen_online_at: row.get(7)?,
                        blocked: row.get(8)?,
                    })
                },
            )
            .optional()?;
        Ok(result)
    }

    /// Liste les vendeurs indexés.
    pub fn indexed_seller_list(&self) -> Result<Vec<IndexedSeller>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT cog_id, shop_name, shop_description, avatar_path, work_count, \
             online_status, last_synced_at, last_seen_online_at, blocked \
             FROM indexed_sellers ORDER BY shop_name ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(IndexedSeller {
                cog_id: row.get(0)?,
                shop_name: row.get(1)?,
                shop_description: row.get(2)?,
                avatar_path: row.get(3)?,
                work_count: row.get(4)?,
                online_status: row.get(5)?,
                last_synced_at: row.get(6)?,
                last_seen_online_at: row.get(7)?,
                blocked: row.get(8)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Liste les entrées du catalogue agrégé avec filtres.
    pub fn aggregated_entry_list(
        &self,
        filters: &AggregatorFilters,
    ) -> Result<Vec<AggregatedCatalogEntry>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut sql = "SELECT id, seller_cog_id, work_id, title, authors, genres, synopsis, \
            cover_thumb_path, reading_format, pricing_model, price, currency, chapter_count, \
            total_pages, demo_pages_count, series_title, tags, language, portal_url, \
            published_at, updated_at, cached_at \
            FROM aggregated_catalog_entries WHERE 1=1".to_string();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(ref genre) = filters.genre {
            sql.push_str(" AND genres LIKE ?");
            param_values.push(Box::new(format!("%{genre}%")));
        }
        if let Some(ref fmt) = filters.reading_format {
            sql.push_str(" AND reading_format = ?");
            param_values.push(Box::new(fmt.clone()));
        }
        if let Some(ref pm) = filters.pricing_model {
            sql.push_str(" AND pricing_model = ?");
            param_values.push(Box::new(pm.clone()));
        }
        if let Some(ref lang) = filters.language {
            sql.push_str(" AND language = ?");
            param_values.push(Box::new(lang.clone()));
        }
        if let Some(ref seller) = filters.seller_cog_id {
            sql.push_str(" AND seller_cog_id = ?");
            param_values.push(Box::new(seller.clone()));
        }
        if let Some(ref q) = filters.search {
            sql.push_str(" AND (title LIKE ? OR tags LIKE ?)");
            let pattern = format!("%{q}%");
            param_values.push(Box::new(pattern.clone()));
            param_values.push(Box::new(pattern));
        }
        if filters.online_only == Some(true) {
            sql.push_str(
                " AND seller_cog_id IN (SELECT cog_id FROM indexed_sellers WHERE online_status='online')",
            );
        }

        sql.push_str(" ORDER BY cached_at DESC");

        if let Some(limit) = filters.limit {
            sql.push_str(&format!(" LIMIT {limit}"));
        }
        if let Some(offset) = filters.offset {
            sql.push_str(&format!(" OFFSET {offset}"));
        }

        let params_ref: Vec<&dyn rusqlite::types::ToSql> =
            param_values.iter().map(|p| p.as_ref()).collect();
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params_ref.as_slice(), |row| {
            Ok(AggregatedCatalogEntry {
                id: row.get(0)?,
                seller_cog_id: row.get(1)?,
                work_id: row.get(2)?,
                title: row.get(3)?,
                authors: row.get(4)?,
                genres: row.get(5)?,
                synopsis: row.get(6)?,
                cover_thumb_path: row.get(7)?,
                reading_format: row.get(8)?,
                pricing_model: row.get(9)?,
                price: row.get(10)?,
                currency: row.get(11)?,
                chapter_count: row.get(12)?,
                total_pages: row.get(13)?,
                demo_pages_count: row.get(14)?,
                series_title: row.get(15)?,
                tags: row.get(16)?,
                language: row.get(17)?,
                portal_url: row.get(18)?,
                published_at: row.get(19)?,
                updated_at: row.get(20)?,
                cached_at: row.get(21)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Compte le nombre total d'entrées agrégées.
    pub fn aggregated_entry_count(&self) -> Result<i64, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM aggregated_catalog_entries",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    /// Supprime les entrées agrégées d'un vendeur.
    pub fn aggregated_entry_delete_by_seller(&self, seller_cog_id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "DELETE FROM aggregated_catalog_entries WHERE seller_cog_id = ?1",
            params![seller_cog_id],
        )?;
        Ok(())
    }

    // ====================================================================
    // AGGREGATOR CONFIG
    // ====================================================================

    /// Récupère la configuration de l'agrégateur.
    pub fn aggregator_config_get(&self) -> Result<AggregatorConfig, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let result = conn
            .query_row(
                "SELECT enabled, name, sync_interval_minutes, presence_refresh_minutes, \
                 max_indexed_cogs, show_offline_works, highlight_own_catalog, blocked_cogs \
                 FROM aggregator_config WHERE id = 1",
                [],
                |row| {
                    Ok(AggregatorConfig {
                        enabled: row.get(0)?,
                        name: row.get(1)?,
                        sync_interval_minutes: row.get(2)?,
                        presence_refresh_minutes: row.get(3)?,
                        max_indexed_cogs: row.get(4)?,
                        show_offline_works: row.get(5)?,
                        highlight_own_catalog: row.get(6)?,
                        blocked_cogs: row.get(7)?,
                    })
                },
            )
            .optional()?;
        Ok(result.unwrap_or_else(AggregatorConfig::defaults))
    }

    /// Crée ou met à jour la configuration de l'agrégateur.
    pub fn aggregator_config_upsert(&self, config: &AggregatorConfig) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO aggregator_config (id, enabled, name, sync_interval_minutes, \
             presence_refresh_minutes, max_indexed_cogs, show_offline_works, \
             highlight_own_catalog, blocked_cogs) \
             VALUES (1,?1,?2,?3,?4,?5,?6,?7,?8) \
             ON CONFLICT(id) DO UPDATE SET \
             enabled=excluded.enabled, name=excluded.name, \
             sync_interval_minutes=excluded.sync_interval_minutes, \
             presence_refresh_minutes=excluded.presence_refresh_minutes, \
             max_indexed_cogs=excluded.max_indexed_cogs, \
             show_offline_works=excluded.show_offline_works, \
             highlight_own_catalog=excluded.highlight_own_catalog, \
             blocked_cogs=excluded.blocked_cogs",
            params![
                config.enabled, config.name, config.sync_interval_minutes,
                config.presence_refresh_minutes, config.max_indexed_cogs,
                config.show_offline_works, config.highlight_own_catalog,
                config.blocked_cogs,
            ],
        )?;
        Ok(())
    }
}
