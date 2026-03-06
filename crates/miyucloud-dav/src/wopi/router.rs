use axum::Router;
use axum::routing::get;

use crate::wopi::discovery::WopiState;
use super::{discovery, handlers};

/// Create the WOPI router mounted at `/wopi/`.
///
/// The caller must provide a `WopiState` via `.with_state(state)`.
pub fn wopi_router() -> Router<WopiState> {
    Router::new()
        .route("/wopi/files/{file_id}", get(discovery::handle_check_file_info))
        .route(
            "/wopi/files/{file_id}/contents",
            get(handlers::handle_get_file).post(handlers::handle_put_file),
        )
}
