//! Route GET / — page d'accueil du COG Web Portal.
//!
//! @id: cog_web_portal_route_home @do: render_portal_home_page
//! @role: api @layer: app
//! @human: GET / — liste les services enregistrés dans le Portal.

use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;

use crate::AppState;
use crate::security_headers::CspNonce;
use crate::templates::{ServiceInfo, render_home};

/// Handler GET / — liste les services disponibles.
pub async fn home(
    State(state): State<AppState>,
    Extension(CspNonce(nonce)): Extension<CspNonce>,
) -> Result<Html<String>, StatusCode> {
    let services: Vec<ServiceInfo> = state
        .services
        .iter()
        .map(|svc| {
            let page_count = svc
                .public_pages()
                .map(|pages| pages.len())
                .unwrap_or(0);
            ServiceInfo {
                slug: svc.service_slug().to_string(),
                name: svc.service_name().to_string(),
                page_count,
            }
        })
        .collect();

    Ok(Html(render_home(&services, &nonce)))
}
