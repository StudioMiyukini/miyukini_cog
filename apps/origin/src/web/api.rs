//! API REST pour le site web Origin.

use super::content::{ContentManager, DownloadCategory};
use crate::tracker::{catalog::Catalog, pool::PoolManager, CatalogVisitTracker};
use serde_json::json;

/// Génère la réponse JSON pour /api/blog.
pub async fn api_blog(content_mgr: &ContentManager) -> String {
    let posts = content_mgr.get_blog_posts().await;
    serde_json::to_string_pretty(&posts).unwrap_or_else(|_| "[]".to_string())
}

/// Génère la réponse JSON pour /api/blog/{id}.
pub async fn api_blog_post(content_mgr: &ContentManager, id: &str) -> Option<String> {
    let post = content_mgr.get_blog_post(id).await?;
    Some(serde_json::to_string_pretty(&post).unwrap_or_else(|_| "{}".to_string()))
}

/// Génère la réponse JSON pour /api/announcements.
pub async fn api_announcements(content_mgr: &ContentManager) -> String {
    let announcements = content_mgr.get_announcements().await;
    serde_json::to_string_pretty(&announcements).unwrap_or_else(|_| "[]".to_string())
}

/// Génère la réponse JSON pour /api/downloads.
pub async fn api_downloads(content_mgr: &ContentManager) -> String {
    let downloads = content_mgr.get_downloads().await;
    serde_json::to_string_pretty(&downloads).unwrap_or_else(|_| "[]".to_string())
}

/// Génère la réponse JSON pour /api/downloads/{category}.
pub async fn api_downloads_by_category(
    content_mgr: &ContentManager,
    category: &str,
) -> Option<String> {
    let cat = match category {
        "cog" => DownloadCategory::Cog,
        "cores" => DownloadCategory::Cores,
        "toolkit" => DownloadCategory::Toolkit,
        "service" => DownloadCategory::Service,
        "devtools" => DownloadCategory::DevTools,
        _ => return None,
    };
    let downloads = content_mgr.get_downloads_by_category(cat).await;
    Some(serde_json::to_string_pretty(&downloads).unwrap_or_else(|_| "[]".to_string()))
}

/// Génère la réponse JSON pour /api/docs.
pub async fn api_docs(content_mgr: &ContentManager) -> String {
    let sections = content_mgr.get_doc_sections().await;
    serde_json::to_string_pretty(&sections).unwrap_or_else(|_| "[]".to_string())
}

/// Génère la réponse JSON pour /api/docs/{section}.
pub async fn api_doc_section(content_mgr: &ContentManager, section_id: &str) -> Option<String> {
    let section = content_mgr.get_doc_section(section_id).await?;
    Some(serde_json::to_string_pretty(&section).unwrap_or_else(|_| "{}".to_string()))
}

/// Génère la réponse JSON pour /api/docs/{section}/{article}.
pub async fn api_doc_article(
    content_mgr: &ContentManager,
    section_id: &str,
    article_id: &str,
) -> Option<String> {
    let article = content_mgr.get_doc_article(section_id, article_id).await?;
    Some(serde_json::to_string_pretty(&article).unwrap_or_else(|_| "{}".to_string()))
}

/// Génère la réponse JSON pour /api/catalog.
pub async fn api_catalog(
    pool_mgr: &PoolManager,
    visit_tracker: Option<&CatalogVisitTracker>,
) -> String {
    let catalog = Catalog::from_pools(pool_mgr).await;
    let json_value = catalog.to_json(visit_tracker).await;
    serde_json::to_string_pretty(&json_value).unwrap_or_else(|_| "{}".to_string())
}

/// Génère la réponse JSON pour /api/status.
pub async fn api_status(pool_mgr: &PoolManager) -> String {
    let total_cogs = pool_mgr.total_cog_count().await;
    let pools = pool_mgr.list_pools().await;

    let status = json!({
        "status": "operational",
        "version": env!("CARGO_PKG_VERSION"),
        "uptime_seconds": 0, // TODO: Implémenter
        "stats": {
            "total_cogs": total_cogs,
            "pools": pools.len(),
            "verified_cogs": total_cogs, // TODO: Séparer vérifié/non-vérifié
        }
    });

    serde_json::to_string_pretty(&status).unwrap_or_else(|_| "{}".to_string())
}

/// Génère la réponse JSON pour /api/health (health check).
pub fn api_health() -> String {
    json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })
    .to_string()
}
