use axum::Router;
use axum::routing::any;

use crate::caldav::service::CalDavService;
use super::handlers;

/// Create the CalDAV router mounted at `/caldav/`.
///
/// The caller must provide a `CalDavService` via `.with_state(service)`.
pub fn caldav_router() -> Router<CalDavService> {
    Router::new()
        .route("/caldav/{*path}", any(caldav_dispatch))
        .route("/caldav/", any(caldav_dispatch_root))
}

async fn caldav_dispatch(
    state: axum::extract::State<CalDavService>,
    req: axum::extract::Request,
) -> axum::response::Response {
    dispatch_by_method(state, req).await
}

async fn caldav_dispatch_root(
    state: axum::extract::State<CalDavService>,
    req: axum::extract::Request,
) -> axum::response::Response {
    dispatch_by_method(state, req).await
}

async fn dispatch_by_method(
    state: axum::extract::State<CalDavService>,
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
        "MKCALENDAR" => handlers::handle_mkcalendar(state, req)
            .await
            .unwrap_or_else(|e| e.into_response()),
        "OPTIONS" => handlers::handle_options().await,
        _ => axum::http::StatusCode::METHOD_NOT_ALLOWED.into_response(),
    }
}
