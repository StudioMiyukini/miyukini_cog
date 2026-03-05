//! Types de domaine principaux pour JayManga.
//!
//! Définit les entités catalogue : Work, Chapter, Page, Series,
//! SellerConfig, OptimizationConfig, et les enums associés.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Oeuvre (Work)
// ---------------------------------------------------------------------------

/// Oeuvre manga complète (volume, one-shot, etc.).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Work {
    pub id: Option<String>,
    pub series_id: Option<String>,
    pub title: Option<String>,
    pub authors: Option<String>, // JSON sérialisé Vec<Author>
    pub genres: Option<String>,  // JSON sérialisé Vec<String>
    pub synopsis: Option<String>,
    pub cover_image_path: Option<String>,
    pub language: Option<String>, // ISO 639-1
    pub volume_number: Option<i32>,
    pub status: Option<String>,        // WorkStatus as_str
    pub pricing_model: Option<String>, // PricingModel as_str
    pub price: Option<i64>,            // centimes (RM-05)
    pub currency: Option<String>,      // défaut "EUR"
    pub demo_pages_count: Option<i32>,
    pub reading_format: Option<String>, // ReadingFormat as_str
    pub allow_download: Option<bool>,
    pub total_pages: Option<i32>,
    pub tags: Option<String>, // JSON sérialisé Vec<String>
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Auteur d'une oeuvre.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub role: Option<String>, // scénariste, dessinateur, encreur, coloriste
}

/// Statut de publication d'une oeuvre.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkStatus {
    Draft,
    Published,
    Unlisted,
    Archived,
}

impl WorkStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Published => "published",
            Self::Unlisted => "unlisted",
            Self::Archived => "archived",
        }
    }

    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "published" => Self::Published,
            "unlisted" => Self::Unlisted,
            "archived" => Self::Archived,
            _ => Self::Draft,
        }
    }
}

/// Modèle de tarification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PricingModel {
    Free,
    Paid,
}

impl PricingModel {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Free => "free",
            Self::Paid => "paid",
        }
    }

    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "paid" => Self::Paid,
            _ => Self::Free,
        }
    }
}

/// Format de lecture d'une oeuvre.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadingFormat {
    Manga,
    Webtoon,
    Landscape,
    Comics,
    Free,
}

impl ReadingFormat {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manga => "manga",
            Self::Webtoon => "webtoon",
            Self::Landscape => "landscape",
            Self::Comics => "comics",
            Self::Free => "free",
        }
    }

    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "manga" => Self::Manga,
            "webtoon" => Self::Webtoon,
            "landscape" => Self::Landscape,
            "comics" => Self::Comics,
            _ => Self::Free,
        }
    }
}

// ---------------------------------------------------------------------------
// Chapitre (Chapter)
// ---------------------------------------------------------------------------

/// Chapitre d'une oeuvre.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Chapter {
    pub id: Option<String>,
    pub work_id: Option<String>,
    pub chapter_number: Option<i32>,
    pub title: Option<String>,
    pub page_count: Option<i32>,
    pub sort_order: Option<i32>,
    #[serde(default)]
    pub created_at: Option<String>,
}

// ---------------------------------------------------------------------------
// Page
// ---------------------------------------------------------------------------

/// Page d'un chapitre (image manga).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Page {
    pub id: Option<String>,
    pub chapter_id: Option<String>,
    pub page_number: Option<i32>,
    pub original_image_path: Option<String>,
    pub optimized_variants: Option<String>, // JSON sérialisé Vec<ImageVariant>
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub file_size: Option<i64>,              // octets
    pub optimization_status: Option<String>, // OptimizationStatus as_str
    pub sort_order: Option<i32>,
}

/// Variante optimisée d'une image de page.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ImageVariant {
    pub profile: String, // "hd", "sd", "mobile", "thumb"
    pub format: String,  // "webp", "avif", "jpeg"
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub file_size: i64,
}

/// Statut d'optimisation d'une page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationStatus {
    Pending,
    Optimized,
    Skipped,
}

impl OptimizationStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Optimized => "optimized",
            Self::Skipped => "skipped",
        }
    }

    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "optimized" => Self::Optimized,
            "skipped" => Self::Skipped,
            _ => Self::Pending,
        }
    }
}

// ---------------------------------------------------------------------------
// Série (Series)
// ---------------------------------------------------------------------------

/// Série regroupant plusieurs oeuvres.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Series {
    pub id: Option<String>,
    pub title: Option<String>,
    pub synopsis: Option<String>,
    pub cover_image_path: Option<String>,
    pub status: Option<String>, // SeriesStatus as_str
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Statut d'une série.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeriesStatus {
    Ongoing,
    Completed,
    Hiatus,
}

impl SeriesStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ongoing => "ongoing",
            Self::Completed => "completed",
            Self::Hiatus => "hiatus",
        }
    }

    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "completed" => Self::Completed,
            "hiatus" => Self::Hiatus,
            _ => Self::Ongoing,
        }
    }
}

// ---------------------------------------------------------------------------
// Configuration vendeur (SellerConfig)
// ---------------------------------------------------------------------------

/// Configuration de la librairie manga du vendeur.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SellerConfig {
    pub id: Option<String>,
    pub shop_name: Option<String>,
    pub shop_description: Option<String>,
    pub default_demo_pages: Option<i32>,
    pub default_allow_download: Option<bool>,
    pub accepted_payment_methods: Option<String>, // JSON
    pub currency: Option<String>,
    pub reading_direction: Option<String>, // "rtl" ou "ltr"
    pub theme: Option<String>,             // JSON personnalisation
    pub allow_aggregation: Option<bool>,
    pub federation_synopsis_length: Option<i32>,
    pub federation_include_prices: Option<bool>,
    pub payment_gateway: Option<String>,
    pub gateway_config: Option<String>, // JSON chiffré (niveau 3)
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

// ---------------------------------------------------------------------------
// Configuration d'optimisation (OptimizationConfig)
// ---------------------------------------------------------------------------

/// Paramètres d'optimisation des images.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OptimizationConfig {
    pub quality_hd: Option<i32>,
    pub quality_sd: Option<i32>,
    pub quality_mobile: Option<i32>,
    pub quality_thumb: Option<i32>,
    pub output_format: Option<String>, // "webp", "avif", "jpeg"
    pub generate_avif: Option<bool>,
    pub active_profiles: Option<String>, // JSON Vec<String>
    pub max_concurrent_jobs: Option<i32>,
    pub jpeg_fallback: Option<bool>,
}

impl OptimizationConfig {
    /// Retourne une configuration d'optimisation par défaut.
    #[must_use]
    pub fn defaults() -> Self {
        Self {
            quality_hd: Some(85),
            quality_sd: Some(80),
            quality_mobile: Some(75),
            quality_thumb: Some(70),
            output_format: Some("webp".to_string()),
            generate_avif: Some(false),
            active_profiles: Some(r#"["hd","sd","mobile","thumb"]"#.to_string()),
            max_concurrent_jobs: Some(2),
            jpeg_fallback: Some(false),
        }
    }
}

// ---------------------------------------------------------------------------
// Filtres de requête
// ---------------------------------------------------------------------------

/// Filtres pour lister les oeuvres.
#[derive(Debug, Clone, Default)]
pub struct WorkFilters {
    pub status: Option<String>,
    pub genre: Option<String>,
    pub reading_format: Option<String>,
    pub pricing_model: Option<String>,
    pub search: Option<String>,
    pub series_id: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Filtres pour lister les transactions.
#[derive(Debug, Clone, Default)]
pub struct TransactionFilters {
    pub status: Option<String>,
    pub method: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Filtres pour le Portail Agrégé.
#[derive(Debug, Clone, Default)]
pub struct AggregatorFilters {
    pub genre: Option<String>,
    pub reading_format: Option<String>,
    pub pricing_model: Option<String>,
    pub language: Option<String>,
    pub online_only: Option<bool>,
    pub search: Option<String>,
    pub seller_cog_id: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work_status_roundtrip() {
        for status in [
            WorkStatus::Draft,
            WorkStatus::Published,
            WorkStatus::Unlisted,
            WorkStatus::Archived,
        ] {
            assert_eq!(WorkStatus::from_str(status.as_str()), status);
        }
    }

    #[test]
    fn test_pricing_model_roundtrip() {
        assert_eq!(PricingModel::from_str("free"), PricingModel::Free);
        assert_eq!(PricingModel::from_str("paid"), PricingModel::Paid);
        assert_eq!(PricingModel::from_str("unknown"), PricingModel::Free);
    }

    #[test]
    fn test_reading_format_roundtrip() {
        for fmt in [
            ReadingFormat::Manga,
            ReadingFormat::Webtoon,
            ReadingFormat::Landscape,
            ReadingFormat::Comics,
            ReadingFormat::Free,
        ] {
            assert_eq!(ReadingFormat::from_str(fmt.as_str()), fmt);
        }
    }

    #[test]
    fn test_series_status_roundtrip() {
        for status in [
            SeriesStatus::Ongoing,
            SeriesStatus::Completed,
            SeriesStatus::Hiatus,
        ] {
            assert_eq!(SeriesStatus::from_str(status.as_str()), status);
        }
    }

    #[test]
    fn test_optimization_config_defaults() {
        let cfg = OptimizationConfig::defaults();
        assert_eq!(cfg.quality_hd, Some(85));
        assert_eq!(cfg.output_format.as_deref(), Some("webp"));
        assert_eq!(cfg.max_concurrent_jobs, Some(2));
    }

    #[test]
    fn test_work_default() {
        let work = Work::default();
        assert!(work.id.is_none());
        assert!(work.price.is_none());
    }

    #[test]
    fn test_author_serde() {
        let author = Author {
            name: "Oda Eiichiro".to_string(),
            role: Some("mangaka".to_string()),
        };
        let json = serde_json::to_string(&author).unwrap();
        let parsed: Author = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, "Oda Eiichiro");
        assert_eq!(parsed.role.as_deref(), Some("mangaka"));
    }

    #[test]
    fn test_image_variant_serde() {
        let variant = ImageVariant {
            profile: "hd".to_string(),
            format: "webp".to_string(),
            path: "/jaymanga/works/abc/chapters/ch1/optimized/page_001_hd.webp".to_string(),
            width: 1920,
            height: 2880,
            file_size: 245_000,
        };
        let json = serde_json::to_string(&variant).unwrap();
        let parsed: ImageVariant = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.profile, "hd");
        assert_eq!(parsed.width, 1920);
    }
}
