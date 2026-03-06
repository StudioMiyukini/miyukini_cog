use axum::Router;
use axum::routing::any;

use crate::common::dav_store::DavStore;
use super::handlers;

/// Create the WebDAV router mounted at `/dav/`.
///
/// The caller must provide a `DavStore` via `.with_state(store)`.
pub fn webdav_router() -> Router<DavStore> {
    Router::new()
        .route("/dav/{*path}", any(dav_dispatch))
        .route("/dav/", any(dav_dispatch_root))
}

async fn dav_dispatch(
    state: axum::extract::State<DavStore>,
    req: axum::extract::Request,
) -> axum::response::Response {
    dispatch_by_method(state, req).await
}

async fn dav_dispatch_root(
    state: axum::extract::State<DavStore>,
    req: axum::extract::Request,
) -> axum::response::Response {
    dispatch_by_method(state, req).await
}

async fn dispatch_by_method(
    state: axum::extract::State<DavStore>,
    req: axum::extract::Request,
) -> axum::response::Response {
    use axum::response::IntoResponse;

    let method = req.method().as_str().to_uppercase();
    match method.as_str() {
        "PROPFIND" => handlers::handle_propfind(state, req)
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
        "COPY" => handlers::handle_copy(state, req)
            .await
            .unwrap_or_else(|e| e.into_response()),
        "MOVE" => handlers::handle_move(state, req)
            .await
            .unwrap_or_else(|e| e.into_response()),
        "OPTIONS" => handlers::handle_options().await,
        _ => axum::http::StatusCode::METHOD_NOT_ALLOWED.into_response(),
    }
}
