//! Routes de service — GET /:service et GET /:service/:slug.
//!
//! @id: cog_web_portal_route_service @do: render_portal_service_pages
//! @role: api @layer: app
//! @human: Routes service Portal — page d'accueil service et page par slug.

use axum::Extension;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Html;

use crate::AppState;
use crate::security_headers::CspNonce;
use crate::templates::{render_error, render_service_home, render_service_page};

/// Handler GET /:service — page d'accueil du service.
pub async fn service_home(
    State(state): State<AppState>,
    Extension(CspNonce(nonce)): Extension<CspNonce>,
    Path(service_slug): Path<String>,
) -> Result<Html<String>, StatusCode> {
    let svc = state
        .services
        .iter()
        .find(|s| s.service_slug() == service_slug)
        .ok_or_else(|| StatusCode::NOT_FOUND)?;

    let pages = svc.public_pages().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(render_service_home(
        svc.service_name(),
        svc.service_slug(),
        &pages,
        &nonce,
    )))
}

/// Handler GET /:service/:slug — page spécifique du service.
pub async fn service_page(
    State(state): State<AppState>,
    Extension(CspNonce(nonce)): Extension<CspNonce>,
    Path((service_slug, page_slug)): Path<(String, String)>,
) -> Html<String> {
    let svc = state.services.iter().find(|s| s.service_slug() == service_slug);
    let Some(svc) = svc else {
        return Html(render_error(404, "Service introuvable", &nonce));
    };

    match svc.page_by_slug(&page_slug) {
        Ok(Some(page)) => Html(render_service_page(
            svc.service_name(),
            svc.service_slug(),
            &page,
            &nonce,
        )),
        Ok(None) => Html(render_error(404, "Page introuvable", &nonce)),
        Err(_) => Html(render_error(500, "Erreur interne", &nonce)),
    }
}
