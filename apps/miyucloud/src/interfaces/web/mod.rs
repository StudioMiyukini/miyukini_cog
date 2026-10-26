//! @id     miyucloud_interfaces_web_mod
//! @do     create_web_routes
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « interfaces web mod ».

use crate::common::config::AppConfig;
use crate::common::di::AppState;
use axum::http::header::{CACHE_CONTROL, HeaderValue};
use axum::{Router, response::Html, routing::get};
use std::sync::Arc;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;

/// Creates web routes for serving static files
pub fn create_web_routes() -> Router<Arc<AppState>> {
    // Get config to access static path
    let config = AppConfig::from_env();

    // In release builds, serve from static-dist/ (processed assets).
    // In debug builds, serve from the original static/ directory.
    let static_path = if cfg!(not(debug_assertions)) {
        let dist = config
            .static_path
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("static-dist");
        if dist.exists() {
            dist
        } else {
            config.static_path.clone()
        }
    } else {
        config.static_path.clone()
    };

    // Static assets (JS, CSS, JSON, SVG, ICO) served via ServeDir
    // with brotli + gzip compression and aggressive caching (7 days).
    // HTML pages are served via explicit routes (include_str!) and
    // do NOT pass through these layers.
    let static_service = ServeDir::new(&static_path);

    Router::new()
        // Add specific routes for clean URLs (without .html)
        .route("/login", get(serve_login_page))
        .route("/profile", get(serve_profile_page))
        .route("/admin", get(serve_admin_page))
        .route("/mobile", get(serve_mobile_page))
        .route("/device", get(serve_device_verify_page))
        // Serve static files with compression + cache headers
        .fallback_service(static_service)
        .layer(CompressionLayer::new().br(true).gzip(true))
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=604800, stale-while-revalidate=86400"),
        ))
}

/// Serve the login page
async fn serve_login_page() -> Html<&'static str> {
    Html(include_str!(concat!(env!("OUT_DIR"), "/login.html")))
}

/// Serve the profile page
async fn serve_profile_page() -> Html<&'static str> {
    Html(include_str!(concat!(env!("OUT_DIR"), "/profile.html")))
}

/// Serve the admin page
async fn serve_admin_page() -> Html<&'static str> {
    Html(include_str!(concat!(env!("OUT_DIR"), "/admin.html")))
}

/// Serve the mobile install & upload page
async fn serve_mobile_page() -> Html<&'static str> {
    Html(include_str!(concat!(env!("OUT_DIR"), "/mobile.html")))
}

/// Serve the device verification page (RFC 8628 Device Authorization Grant)
async fn serve_device_verify_page() -> Html<&'static str> {
    Html(include_str!(concat!(
        env!("OUT_DIR"),
        "/device-verify.html"
    )))
}

#[cfg(test)]
mod rendu_tests {
    /// Rendu de la page de profil : sections et formulaires des liaisons
    /// d'appareils et d'identités (I6), branchés sur les routes protégées.
    #[test]
    fn profile_page_exposes_device_and_identity_links() {
        let html = include_str!(concat!(env!("OUT_DIR"), "/profile.html"));
        for id in [
            "miyu-devices-section",
            "miyu-dev-form",
            "miyu-dev-request",
            "miyu-dev-scopes",
            "miyu-identities-section",
            "miyu-id-form",
            "miyu-id-proof",
            "/js/core/csrf.js",
        ] {
            assert!(html.contains(id), "{id}");
        }
        let js = include_str!("../../../static/js/views/profile/profile.js");
        for attendu in [
            "'/auth/miyu/appareils'",
            "'/auth/miyu/liaisons'",
            "X-Miyu-Delegations",
            "credentials: 'same-origin'",
        ] {
            assert!(js.contains(attendu), "{attendu}");
        }
        // Toute requête modifiante des liaisons porte le jeton CSRF (headers()).
        assert!(!js.contains("method: 'POST', credentials"));
        // Textes affichés : jamais d'innerHTML sur une donnée du serveur.
        assert!(!js.contains("innerHTML = d."));
        assert!(!js.contains("innerHTML = i."));
    }
}
