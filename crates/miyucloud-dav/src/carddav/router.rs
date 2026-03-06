use axum::Router;
use axum::routing::any;

use crate::carddav::service::CardDavService;
use super::handlers;

/// Create the CardDAV router mounted at `/carddav/`.
///
/// The caller must provide a `CardDavService` via `.with_state(service)`.
pub fn carddav_router() -> Router<CardDavService> {
    Router::new()
        .route("/carddav/{*path}", any(carddav_dispatch))
        .route("/carddav/", any(carddav_dispatch_root))
}

async fn carddav_dispatch(
    state: axum::extract::State<CardDavService>,
    req: axum::extract::Request,
) -> axum::response::Response {
    dispatch_by_method(state, req).await
}

async fn carddav_dispatch_root(
    state: axum::extract::State<CardDavService>,
    req: axum::extract::Request,
) -> axum::response::Response {
    dispatch_by_method(state, req).await
}

async fn dispatch_by_method(
    state: axum::extract::State<CardDavService>,
    req: axum::extract::Request,
) -> axum::response::Response {
    use axum::response::IntoResponse;

    let method = req.method().as_str().to_uppercase();
    match method.as_str() {
        "PROPFIND" => handlers::handle_propfind(state, req)
            .await
            .unwrap_or_else(|e| e.into_response()),
        "REPORT" => handlers::handle_report(state, req)
            .await
            .unwrap_or_else(|e| e.into_response()),
        "GET" | "HEAD" => handlers::handle_get(state, req)
            .await
            .unwrap_or_else(|e| e.into_response()),
        "PUT" => handlers::handle_put(state, req)
            .await
            .unwrap_or_else(|e| e.into_response()),
        "DELETE" => handlers::handle_delete(state, req)
            .await
            .unwrap_or_else(|e| e.into_response()),
        "MKCOL" => handlers::handle_mkcol(state, req)
            .await
            .unwrap_or_else(|e| e.into_response()),
        "OPTIONS" => handlers::handle_options().await,
        _ => axum::http::StatusCode::METHOD_NOT_ALLOWED.into_response(),
    }
}
